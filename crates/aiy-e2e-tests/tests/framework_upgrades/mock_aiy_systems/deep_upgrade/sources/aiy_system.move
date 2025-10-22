// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module aiy_system::aiy_system {
    use std::vector;

    use aiy::balance::Balance;
    use aiy::object::UID;
    use aiy::aiy::AIY;
    use aiy::transfer;
    use aiy::tx_context::{Self, TxContext};
    use aiy::dynamic_field;

    use aiy_system::validator::Validator;
    use aiy_system::aiy_system_state_inner::{Self, AiySystemStateInnerV2, AiySystemStateInner};

    public struct AiySystemState has key {
        id: UID,
        version: u64,
    }

    public(package) fun create(
        id: UID,
        validators: vector<Validator>,
        storage_fund: Balance<AIY>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = aiy_system_state_inner::create(
            validators,
            storage_fund,
            protocol_version,
            epoch_start_timestamp_ms,
            epoch_duration_ms,
            ctx,
        );
        let version = aiy_system_state_inner::genesis_system_state_version();
        let mut self = AiySystemState {
            id,
            version,
        };
        dynamic_field::add(&mut self.id, version, system_state);
        transfer::share_object(self);
    }

    fun advance_epoch(
        storage_reward: Balance<AIY>,
        computation_reward: Balance<AIY>,
        wrapper: &mut AiySystemState,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
                                         // into storage fund, in basis point.
        _reward_slashing_rate: u64, // how much rewards are slashed to punish a validator, in bps.
        epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
        ctx: &mut TxContext,
    ) : Balance<AIY> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x0, 0);
        let storage_rebate = aiy_system_state_inner::advance_epoch(
            self,
            new_epoch,
            next_protocol_version,
            storage_reward,
            computation_reward,
            storage_rebate,
            epoch_start_timestamp_ms,
        );

        storage_rebate
    }

    public fun active_validator_addresses(wrapper: &mut AiySystemState): vector<address> {
        vector::empty()
    }

    fun load_system_state_mut(self: &mut AiySystemState): &mut AiySystemStateInnerV2 {
        load_inner_maybe_upgrade(self)
    }

    fun load_inner_maybe_upgrade(self: &mut AiySystemState): &mut AiySystemStateInnerV2 {
        let mut version = self.version;
        if (version == aiy_system_state_inner::genesis_system_state_version()) {
            let inner: AiySystemStateInner = dynamic_field::remove(&mut self.id, version);
            let new_inner = aiy_system_state_inner::v1_to_v2(inner);
            version = aiy_system_state_inner::system_state_version(&new_inner);
            dynamic_field::add(&mut self.id, version, new_inner);
            self.version = version;
        };

        let inner: &mut AiySystemStateInnerV2 = dynamic_field::borrow_mut(&mut self.id, version);
        assert!(aiy_system_state_inner::system_state_version(inner) == version, 0);
        inner
    }

    fun store_execution_time_estimates(wrapper: &mut AiySystemState, estimates_bytes: vector<u8>) {
        let self = load_system_state_mut(wrapper);
        aiy_system_state_inner::store_execution_time_estimates(self, estimates_bytes)
    }
}
