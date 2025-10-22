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
    use aiy_system::aiy_system_state_inner::AiySystemStateInner;
    use aiy_system::aiy_system_state_inner;

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
        _new_epoch: u64,
        _next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64,
        _reward_slashing_rate: u64,
        _epoch_start_timestamp_ms: u64,
        ctx: &mut TxContext,
    ) : Balance<AIY> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x1, 0); // aborts here
        aiy_system_state_inner::advance_epoch(
            self,
            storage_reward,
            computation_reward,
            storage_rebate,
        )
    }

    public fun active_validator_addresses(wrapper: &mut AiySystemState): vector<address> {
        vector::empty()
    }

    fun load_system_state_mut(self: &mut AiySystemState): &mut AiySystemStateInner {
        let version = self.version;
        dynamic_field::borrow_mut(&mut self.id, version)
    }
}
