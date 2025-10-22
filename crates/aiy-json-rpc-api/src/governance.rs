// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use aiy_json_rpc_types::{DelegatedStake, AiyCommittee, ValidatorApys};
use aiy_open_rpc_macros::open_rpc;
use aiy_types::base_types::{ObjectID, AiyAddress};
use aiy_types::aiy_serde::BigInt;
use aiy_types::aiy_system_state::aiy_system_state_summary::AiySystemStateSummary;

#[open_rpc(namespace = "aiyx", tag = "Governance Read API")]
#[rpc(server, client, namespace = "aiyx")]
pub trait GovernanceReadApi {
    /// Return one or more [DelegatedStake]. If a Stake was withdrawn its status will be Unstaked.
    #[method(name = "getStakesByIds")]
    async fn get_stakes_by_ids(
        &self,
        staked_aiy_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all [DelegatedStake].
    #[method(name = "getStakes")]
    async fn get_stakes(&self, owner: AiyAddress) -> RpcResult<Vec<DelegatedStake>>;

    /// Return the committee information for the asked `epoch`.
    #[method(name = "getCommitteeInfo")]
    async fn get_committee_info(
        &self,
        /// The epoch of interest. If None, default to the latest epoch
        epoch: Option<BigInt<u64>>,
    ) -> RpcResult<AiyCommittee>;

    /// Return the latest AIY system state object on-chain.
    #[method(name = "getLatestAiySystemState")]
    async fn get_latest_aiy_system_state(&self) -> RpcResult<AiySystemStateSummary>;

    /// Return the reference gas price for the network
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;

    /// Return the validator APY
    #[method(name = "getValidatorsApy")]
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys>;
}
