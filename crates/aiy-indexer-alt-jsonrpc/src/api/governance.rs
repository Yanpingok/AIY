// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use diesel::{ExpressionMethods, QueryDsl};

use jsonrpsee::{core::RpcResult, http_client::HttpClient, proc_macros::rpc};
use aiy_indexer_alt_schema::schema::kv_epoch_starts;
use aiy_json_rpc_api::GovernanceReadApiClient;
use aiy_json_rpc_types::{DelegatedStake, ValidatorApys};
use aiy_open_rpc::Module;
use aiy_open_rpc_macros::open_rpc;
use aiy_types::{
    base_types::{ObjectID, AiyAddress},
    dynamic_field::{derive_dynamic_field_id, Field},
    aiy_serde::BigInt,
    aiy_system_state::{
        aiy_system_state_inner_v1::AiySystemStateInnerV1,
        aiy_system_state_inner_v2::AiySystemStateInnerV2,
        aiy_system_state_summary::AiySystemStateSummary, AiySystemStateTrait,
        AiySystemStateWrapper,
    },
    TypeTag, AIY_SYSTEM_STATE_OBJECT_ID,
};

use crate::{
    config::NodeConfig,
    context::Context,
    data::load_live_deserialized,
    error::{client_error_to_error_object, rpc_bail, RpcError},
};

use super::rpc_module::RpcModule;

#[open_rpc(namespace = "aiyx", tag = "Governance API")]
#[rpc(server, namespace = "aiyx")]
trait GovernanceApi {
    /// Return the reference gas price for the network as of the latest epoch.
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;

    /// Return a summary of the latest version of the Aiy System State object (0x5), on-chain.
    #[method(name = "getLatestAiySystemState")]
    async fn get_latest_aiy_system_state(&self) -> RpcResult<AiySystemStateSummary>;
}

#[open_rpc(namespace = "aiyx", tag = "Delegation Governance API")]
#[rpc(server, namespace = "aiyx")]
trait DelegationGovernanceApi {
    /// Return one or more [DelegatedStake]. If a Stake was withdrawn its status will be Unstaked.
    #[method(name = "getStakesByIds")]
    async fn get_stakes_by_ids(
        &self,
        staked_aiy_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all [DelegatedStake].
    #[method(name = "getStakes")]
    async fn get_stakes(&self, owner: AiyAddress) -> RpcResult<Vec<DelegatedStake>>;

    /// Return the validator APY
    #[method(name = "getValidatorsApy")]
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys>;
}

pub(crate) struct Governance(pub Context);
pub(crate) struct DelegationGovernance(HttpClient);

impl DelegationGovernance {
    pub fn new(fullnode_rpc_url: url::Url, config: NodeConfig) -> anyhow::Result<Self> {
        let client = config.client(fullnode_rpc_url)?;
        Ok(Self(client))
    }
}

#[async_trait::async_trait]
impl GovernanceApiServer for Governance {
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        Ok(rgp_response(&self.0).await?)
    }

    async fn get_latest_aiy_system_state(&self) -> RpcResult<AiySystemStateSummary> {
        Ok(latest_aiy_system_state_response(&self.0).await?)
    }
}

#[async_trait::async_trait]
impl DelegationGovernanceApiServer for DelegationGovernance {
    async fn get_stakes_by_ids(
        &self,
        staked_aiy_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>> {
        let Self(client) = self;

        client
            .get_stakes_by_ids(staked_aiy_ids)
            .await
            .map_err(client_error_to_error_object)
    }

    async fn get_stakes(&self, owner: AiyAddress) -> RpcResult<Vec<DelegatedStake>> {
        let Self(client) = self;

        client
            .get_stakes(owner)
            .await
            .map_err(client_error_to_error_object)
    }

    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys> {
        let Self(client) = self;

        client
            .get_validators_apy()
            .await
            .map_err(client_error_to_error_object)
    }
}

impl RpcModule for Governance {
    fn schema(&self) -> Module {
        GovernanceApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}

impl RpcModule for DelegationGovernance {
    fn schema(&self) -> Module {
        DelegationGovernanceApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}

/// Load data and generate response for `getReferenceGasPrice`.
async fn rgp_response(ctx: &Context) -> Result<BigInt<u64>, RpcError> {
    use kv_epoch_starts::dsl as e;

    let mut conn = ctx
        .pg_reader()
        .connect()
        .await
        .context("Failed to connect to the database")?;

    let rgp: i64 = conn
        .first(
            e::kv_epoch_starts
                .select(e::reference_gas_price)
                .order(e::epoch.desc()),
        )
        .await
        .context("Failed to fetch the reference gas price")?;

    Ok((rgp as u64).into())
}

/// Load data and generate response for `getLatestAiySystemState`.
async fn latest_aiy_system_state_response(
    ctx: &Context,
) -> Result<AiySystemStateSummary, RpcError> {
    let wrapper: AiySystemStateWrapper = load_live_deserialized(ctx, AIY_SYSTEM_STATE_OBJECT_ID)
        .await
        .context("Failed to fetch system state wrapper object")?;

    let inner_id = derive_dynamic_field_id(
        AIY_SYSTEM_STATE_OBJECT_ID,
        &TypeTag::U64,
        &bcs::to_bytes(&wrapper.version).context("Failed to serialize system state version")?,
    )
    .context("Failed to derive inner system state field ID")?;

    Ok(match wrapper.version {
        1 => load_live_deserialized::<Field<u64, AiySystemStateInnerV1>>(ctx, inner_id)
            .await
            .context("Failed to fetch inner system state object")?
            .value
            .into_aiy_system_state_summary(),
        2 => load_live_deserialized::<Field<u64, AiySystemStateInnerV2>>(ctx, inner_id)
            .await
            .context("Failed to fetch inner system state object")?
            .value
            .into_aiy_system_state_summary(),
        v => rpc_bail!("Unexpected inner system state version: {v}"),
    })
}
