// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::abi::EthBridgeConfig;
use crate::crypto::BridgeAuthorityKeyPair;
use crate::error::BridgeError;
use crate::eth_client::EthClient;
use crate::metered_eth_provider::new_metered_eth_provider;
use crate::metered_eth_provider::MeteredEthHttpProvier;
use crate::metrics::BridgeMetrics;
use crate::aiy_client::AiyClient;
use crate::types::{is_route_valid, BridgeAction};
use crate::utils::get_eth_contract_addresses;
use anyhow::anyhow;
use ethers::providers::Middleware;
use ethers::types::Address as EthAddress;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use aiy_config::Config;
use aiy_json_rpc_types::Coin;
use aiy_keys::keypair_file::read_key;
use aiy_sdk::apis::CoinReadApi;
use aiy_sdk::{AiyClient as AiySdkClient, AiyClientBuilder};
use aiy_types::base_types::ObjectRef;
use aiy_types::base_types::{ObjectID, AiyAddress};
use aiy_types::bridge::BridgeChainId;
use aiy_types::crypto::KeypairTraits;
use aiy_types::crypto::{get_key_pair_from_rng, NetworkKeyPair, AiyKeyPair};
use aiy_types::digests::{get_mainnet_chain_identifier, get_testnet_chain_identifier};
use aiy_types::event::EventID;
use aiy_types::object::Owner;
use tracing::info;

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EthConfig {
    /// Rpc url for Eth fullnode, used for query stuff.
    pub eth_rpc_url: String,
    /// The proxy address of AiyBridge
    pub eth_bridge_proxy_address: String,
    /// The expected BridgeChainId on Eth side.
    pub eth_bridge_chain_id: u8,
    /// The starting block for EthSyncer to monitor eth contracts.
    /// It is required when `run_client` is true. Usually this is
    /// the block number when the bridge contracts are deployed.
    /// When BridgeNode starts, it reads the contract watermark from storage.
    /// If the watermark is not found, it will start from this fallback block number.
    /// If the watermark is found, it will start from the watermark.
    /// this v.s.`eth_contracts_start_block_override`:
    pub eth_contracts_start_block_fallback: Option<u64>,
    /// The starting block for EthSyncer to monitor eth contracts. It overrides
    /// the watermark in storage. This is useful when we want to reprocess the events
    /// from a specific block number.
    /// Note: this field has to be reset after starting the BridgeNode, otherwise it will
    /// reprocess the events from this block number every time it starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eth_contracts_start_block_override: Option<u64>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AiyConfig {
    /// Rpc url for Aiy fullnode, used for query stuff and submit transactions.
    pub aiy_rpc_url: String,
    /// The expected BridgeChainId on Aiy side.
    pub aiy_bridge_chain_id: u8,
    /// Path of the file where bridge client key (any AiyKeyPair) is stored.
    /// If `run_client` is true, and this is None, then use `bridge_authority_key_path` as client key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_key_path: Option<PathBuf>,
    /// The gas object to use for paying for gas fees for the client. It needs to
    /// be owned by the address associated with bridge client key. If not set
    /// and `run_client` is true, it will query and use the gas object with highest
    /// amount for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_gas_object: Option<ObjectID>,
    /// Override the last processed EventID for bridge module `bridge`.
    /// When set, AiySyncer will start from this cursor (exclusively) instead of the one in storage.
    /// If the cursor is not found in storage or override, the query will start from genesis.
    /// Key: aiy module, Value: last processed EventID (tx_digest, event_seq).
    /// Note 1: This field should be rarely used. Only use it when you understand how to follow up.
    /// Note 2: the EventID needs to be valid, namely it must exist and matches the filter.
    /// Otherwise, it will miss one event because of fullnode Event query semantics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aiy_bridge_module_last_processed_event_id_override: Option<EventID>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeNodeConfig {
    /// The port that the server listens on.
    pub server_listen_port: u16,
    /// The port that for metrics server.
    pub metrics_port: u16,
    /// Path of the file where bridge authority key (Secp256k1) is stored.
    pub bridge_authority_key_path: PathBuf,
    /// Whether to run client. If true, `aiy.bridge_client_key_path`
    /// and `db_path` needs to be provided.
    pub run_client: bool,
    /// Path of the client storage. Required when `run_client` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_path: Option<PathBuf>,
    /// A list of approved governance actions. Action in this list will be signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
    /// Aiy configuration
    pub aiy: AiyConfig,
    /// Eth configuration
    pub eth: EthConfig,
    /// Network key used for metrics pushing
    #[serde(default = "default_ed25519_key_pair")]
    pub metrics_key_pair: NetworkKeyPair,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<MetricsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchdog_config: Option<WatchdogConfig>,
}

pub fn default_ed25519_key_pair() -> NetworkKeyPair {
    get_key_pair_from_rng(&mut rand::rngs::OsRng).1
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetricsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_interval_seconds: Option<u64>,
    pub push_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WatchdogConfig {
    /// Total supplies to watch on Aiy. Mapping from coin name to coin type tag
    pub total_supplies: BTreeMap<String, String>,
}

impl Config for BridgeNodeConfig {}

impl BridgeNodeConfig {
    pub async fn validate(
        &self,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(BridgeServerConfig, Option<BridgeClientConfig>)> {
        info!("Starting config validation");
        if !is_route_valid(
            BridgeChainId::try_from(self.aiy.aiy_bridge_chain_id)?,
            BridgeChainId::try_from(self.eth.eth_bridge_chain_id)?,
        ) {
            return Err(anyhow!(
                "Route between Aiy chain id {} and Eth chain id {} is not valid",
                self.aiy.aiy_bridge_chain_id,
                self.eth.eth_bridge_chain_id,
            ));
        };

        let bridge_authority_key = match read_key(&self.bridge_authority_key_path, true)? {
            AiyKeyPair::Secp256k1(key) => key,
            _ => unreachable!("we required secp256k1 key in `read_key`"),
        };

        // we do this check here instead of `prepare_for_aiy` below because
        // that is only called when `run_client` is true.
        let aiy_client =
            Arc::new(AiyClient::<AiySdkClient>::new(&self.aiy.aiy_rpc_url, metrics.clone()).await?);
        let bridge_committee = aiy_client
            .get_bridge_committee()
            .await
            .map_err(|e| anyhow!("Error getting bridge committee: {:?}", e))?;
        if !bridge_committee.is_active_member(&bridge_authority_key.public().into()) {
            return Err(anyhow!(
                "Bridge authority key is not part of bridge committee"
            ));
        }

        let (eth_client, eth_contracts) = self.prepare_for_eth(metrics.clone()).await?;
        let bridge_summary = aiy_client
            .get_bridge_summary()
            .await
            .map_err(|e| anyhow!("Error getting bridge summary: {:?}", e))?;
        if bridge_summary.chain_id != self.aiy.aiy_bridge_chain_id {
            anyhow::bail!(
                "Bridge chain id mismatch: expected {}, but connected to {}",
                self.aiy.aiy_bridge_chain_id,
                bridge_summary.chain_id
            );
        }

        // Validate approved actions that must be governace actions
        for action in &self.approved_governance_actions {
            if !action.is_governace_action() {
                anyhow::bail!(format!(
                    "{:?}",
                    BridgeError::ActionIsNotGovernanceAction(action.clone())
                ));
            }
        }
        let approved_governance_actions = self.approved_governance_actions.clone();

        let bridge_server_config = BridgeServerConfig {
            key: bridge_authority_key,
            metrics_port: self.metrics_port,
            eth_bridge_proxy_address: eth_contracts[0], // the first contract is bridge proxy
            server_listen_port: self.server_listen_port,
            aiy_client: aiy_client.clone(),
            eth_client: eth_client.clone(),
            approved_governance_actions,
        };
        if !self.run_client {
            return Ok((bridge_server_config, None));
        }

        // If client is enabled, prepare client config
        let (bridge_client_key, client_aiy_address, gas_object_ref) =
            self.prepare_for_aiy(aiy_client.clone(), metrics).await?;

        let db_path = self
            .db_path
            .clone()
            .ok_or(anyhow!("`db_path` is required when `run_client` is true"))?;

        let bridge_client_config = BridgeClientConfig {
            aiy_address: client_aiy_address,
            key: bridge_client_key,
            gas_object_ref,
            metrics_port: self.metrics_port,
            aiy_client: aiy_client.clone(),
            eth_client: eth_client.clone(),
            db_path,
            eth_contracts,
            // in `prepare_for_eth` we check if this is None when `run_client` is true. Safe to unwrap here.
            eth_contracts_start_block_fallback: self
                .eth
                .eth_contracts_start_block_fallback
                .unwrap(),
            eth_contracts_start_block_override: self.eth.eth_contracts_start_block_override,
            aiy_bridge_module_last_processed_event_id_override: self
                .aiy
                .aiy_bridge_module_last_processed_event_id_override,
        };

        info!("Config validation complete");
        Ok((bridge_server_config, Some(bridge_client_config)))
    }

    async fn prepare_for_eth(
        &self,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(Arc<EthClient<MeteredEthHttpProvier>>, Vec<EthAddress>)> {
        info!("Creating Ethereum client provider");
        let bridge_proxy_address = EthAddress::from_str(&self.eth.eth_bridge_proxy_address)?;
        let provider = Arc::new(
            new_metered_eth_provider(&self.eth.eth_rpc_url, metrics.clone())
                .unwrap()
                .interval(std::time::Duration::from_millis(2000)),
        );
        let chain_id = provider.get_chainid().await?;
        let (
            committee_address,
            limiter_address,
            vault_address,
            config_address,
            _weth_address,
            _usdt_address,
            _wbtc_address,
            _lbtc_address,
        ) = get_eth_contract_addresses(bridge_proxy_address, &provider).await?;
        let config = EthBridgeConfig::new(config_address, provider.clone());

        if self.run_client && self.eth.eth_contracts_start_block_fallback.is_none() {
            return Err(anyhow!(
                "eth_contracts_start_block_fallback is required when run_client is true"
            ));
        }

        // If bridge chain id is Eth Mainent or Sepolia, we expect to see chain
        // identifier to match accordingly.
        let bridge_chain_id: u8 = config.chain_id().call().await?;
        if self.eth.eth_bridge_chain_id != bridge_chain_id {
            return Err(anyhow!(
                "Bridge chain id mismatch: expected {}, but connected to {}",
                self.eth.eth_bridge_chain_id,
                bridge_chain_id
            ));
        }
        if bridge_chain_id == BridgeChainId::EthMainnet as u8 && chain_id.as_u64() != 1 {
            anyhow::bail!(
                "Expected Eth chain id 1, but connected to {}",
                chain_id.as_u64()
            );
        }
        if bridge_chain_id == BridgeChainId::EthSepolia as u8 && chain_id.as_u64() != 11155111 {
            anyhow::bail!(
                "Expected Eth chain id 11155111, but connected to {}",
                chain_id.as_u64()
            );
        }
        info!(
            "Connected to Eth chain: {}, Bridge chain id: {}",
            chain_id.as_u64(),
            bridge_chain_id,
        );

        let eth_client = Arc::new(
            EthClient::<MeteredEthHttpProvier>::new(
                &self.eth.eth_rpc_url,
                HashSet::from_iter(vec![
                    bridge_proxy_address,
                    committee_address,
                    config_address,
                    limiter_address,
                    vault_address,
                ]),
                metrics,
            )
            .await?,
        );
        let contract_addresses = vec![
            bridge_proxy_address,
            committee_address,
            config_address,
            limiter_address,
            vault_address,
        ];
        info!("Ethereum client setup complete");
        Ok((eth_client, contract_addresses))
    }

    async fn prepare_for_aiy(
        &self,
        aiy_client: Arc<AiyClient<AiySdkClient>>,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(AiyKeyPair, AiyAddress, ObjectRef)> {
        let bridge_client_key = match &self.aiy.bridge_client_key_path {
            None => read_key(&self.bridge_authority_key_path, true),
            Some(path) => read_key(path, false),
        }?;

        // If bridge chain id is Aiy Mainent or Testnet, we expect to see chain
        // identifier to match accordingly.
        let aiy_identifier = aiy_client
            .get_chain_identifier()
            .await
            .map_err(|e| anyhow!("Error getting chain identifier from Aiy: {:?}", e))?;
        if self.aiy.aiy_bridge_chain_id == BridgeChainId::AiyMainnet as u8
            && aiy_identifier != get_mainnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected aiy chain identifier {}, but connected to {}",
                self.aiy.aiy_bridge_chain_id,
                aiy_identifier
            );
        }
        if self.aiy.aiy_bridge_chain_id == BridgeChainId::AiyTestnet as u8
            && aiy_identifier != get_testnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected aiy chain identifier {}, but connected to {}",
                self.aiy.aiy_bridge_chain_id,
                aiy_identifier
            );
        }
        info!(
            "Connected to Aiy chain: {}, Bridge chain id: {}",
            aiy_identifier, self.aiy.aiy_bridge_chain_id,
        );

        let client_aiy_address = AiyAddress::from(&bridge_client_key.public());

        let gas_object_id = match self.aiy.bridge_client_gas_object {
            Some(id) => id,
            None => {
                info!("No gas object configured, finding gas object with highest balance");
                let aiy_client = AiyClientBuilder::default()
                    .build(&self.aiy.aiy_rpc_url)
                    .await?;
                let coin =
                    // Minimum balance for gas object is 10 AIY
                    pick_highest_balance_coin(aiy_client.coin_read_api(), client_aiy_address, 10_000_000_000)
                        .await?;
                coin.coin_object_id
            }
        };
        let (gas_coin, gas_object_ref, owner) = aiy_client
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await;
        if owner != Owner::AddressOwner(client_aiy_address) {
            return Err(anyhow!("Gas object {:?} is not owned by bridge client key's associated aiy address {:?}, but {:?}", gas_object_id, client_aiy_address, owner));
        }
        let balance = gas_coin.value();
        info!("Gas object balance: {}", balance);
        metrics.gas_coin_balance.set(balance as i64);

        info!("Aiy client setup complete");
        Ok((bridge_client_key, client_aiy_address, gas_object_ref))
    }
}

pub struct BridgeServerConfig {
    pub key: BridgeAuthorityKeyPair,
    pub server_listen_port: u16,
    pub eth_bridge_proxy_address: EthAddress,
    pub metrics_port: u16,
    pub aiy_client: Arc<AiyClient<AiySdkClient>>,
    pub eth_client: Arc<EthClient<MeteredEthHttpProvier>>,
    /// A list of approved governance actions. Action in this list will be signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
}

pub struct BridgeClientConfig {
    pub aiy_address: AiyAddress,
    pub key: AiyKeyPair,
    pub gas_object_ref: ObjectRef,
    pub metrics_port: u16,
    pub aiy_client: Arc<AiyClient<AiySdkClient>>,
    pub eth_client: Arc<EthClient<MeteredEthHttpProvier>>,
    pub db_path: PathBuf,
    pub eth_contracts: Vec<EthAddress>,
    // See `BridgeNodeConfig` for the explanation of following two fields.
    pub eth_contracts_start_block_fallback: u64,
    pub eth_contracts_start_block_override: Option<u64>,
    pub aiy_bridge_module_last_processed_event_id_override: Option<EventID>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeCommitteeConfig {
    pub bridge_authority_port_and_key_path: Vec<(u64, PathBuf)>,
}

impl Config for BridgeCommitteeConfig {}

pub async fn pick_highest_balance_coin(
    coin_read_api: &CoinReadApi,
    address: AiyAddress,
    minimal_amount: u64,
) -> anyhow::Result<Coin> {
    info!("Looking for a aiytable gas coin for address {:?}", address);

    // Only look at AIY coins specifically
    let mut stream = coin_read_api
        .get_coins_stream(address, Some("0x2::aiy::AIY".to_string()))
        .boxed();

    let mut coins_checked = 0;

    while let Some(coin) = stream.next().await {
        info!(
            "Checking coin: {:?}, balance: {}",
            coin.coin_object_id, coin.balance
        );
        coins_checked += 1;

        // Take the first coin with a sufficient balance
        if coin.balance >= minimal_amount {
            info!(
                "Found aiytable gas coin with {} mist (object ID: {:?})",
                coin.balance, coin.coin_object_id
            );
            return Ok(coin);
        }

        // Only check a small number of coins before giving up
        if coins_checked >= 1000 {
            break;
        }
    }

    Err(anyhow!(
        "No aiytable gas coin with >= {} mist found for address {:?} after checking {} coins",
        minimal_amount,
        address,
        coins_checked
    ))
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EthContractAddresses {
    pub aiy_bridge: EthAddress,
    pub bridge_committee: EthAddress,
    pub bridge_config: EthAddress,
    pub bridge_limiter: EthAddress,
    pub bridge_vault: EthAddress,
}
