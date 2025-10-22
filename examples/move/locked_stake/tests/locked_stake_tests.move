// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(deprecated_usage)] // TODO: update tests to not use deprecated governance
module locked_stake::locked_stake_tests;

use locked_stake::epoch_time_lock;
use locked_stake::locked_stake as ls;
use std::unit_test::assert_eq;
use aiy::balance;
use aiy::coin;
use aiy::test_scenario;
use aiy::test_utils::destroy;
use aiy::vec_map;
use aiy_system::governance_test_utils::{advance_epoch, set_up_aiy_system_state};
use aiy_system::aiy_system::{Self, AiySystemState};

const MIST_PER_AIY: u64 = 1_000_000_000;

#[test]
#[expected_failure(abort_code = epoch_time_lock::EEpochAlreadyPassed)]
fun test_incorrect_creation() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_aiy_system_state(vector[@0x1, @0x2, @0x3]);

    // Advance epoch twice so we are now at epoch 2.
    advance_epoch(scenario);
    advance_epoch(scenario);
    let ctx = test_scenario::ctx(scenario);
    assert_eq!(tx_context::epoch(ctx), 2);

    // Create a locked stake with epoch 1. Should fail here.
    let ls = ls::new(1, ctx);

    destroy(ls);
    test_scenario::end(scenario_val);
}

#[test]
fun test_deposit_stake_unstake() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_aiy_system_state(vector[@0x1, @0x2, @0x3]);

    let mut ls = ls::new(10, test_scenario::ctx(scenario));

    // Deposit 100 AIY.
    ls::deposit_aiy(&mut ls, balance::create_for_testing(100 * MIST_PER_AIY));

    assert_eq!(ls::aiy_balance(&ls), 100 * MIST_PER_AIY);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<AiySystemState>(scenario);

    // Stake 10 of the 100 AIY.
    ls::stake(&mut ls, &mut system_state, 10 * MIST_PER_AIY, @0x1, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);

    assert_eq!(ls::aiy_balance(&ls), 90 * MIST_PER_AIY);
    assert_eq!(vec_map::length(ls::staked_aiy(&ls)), 1);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<AiySystemState>(scenario);
    let ctx = test_scenario::ctx(scenario);

    // Create a StakedAiy object and add it to the LockedStake object.
    let staked_aiy = aiy_system::request_add_stake_non_entry(
        &mut system_state,
        coin::mint_for_testing(20 * MIST_PER_AIY, ctx),
        @0x2,
        ctx,
    );
    test_scenario::return_shared(system_state);

    ls::deposit_staked_aiy(&mut ls, staked_aiy);
    assert_eq!(ls::aiy_balance(&ls), 90 * MIST_PER_AIY);
    assert_eq!(vec_map::length(ls::staked_aiy(&ls)), 2);
    advance_epoch(scenario);

    test_scenario::next_tx(scenario, @0x1);
    let (staked_aiy_id, _) = vec_map::get_entry_by_idx(ls::staked_aiy(&ls), 0);
    let mut system_state = test_scenario::take_shared<AiySystemState>(scenario);

    // Unstake both stake objects
    ls::unstake(&mut ls, &mut system_state, *staked_aiy_id, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);
    assert_eq!(ls::aiy_balance(&ls), 100 * MIST_PER_AIY);
    assert_eq!(vec_map::length(ls::staked_aiy(&ls)), 1);

    test_scenario::next_tx(scenario, @0x1);
    let (staked_aiy_id, _) = vec_map::get_entry_by_idx(ls::staked_aiy(&ls), 0);
    let mut system_state = test_scenario::take_shared<AiySystemState>(scenario);
    ls::unstake(&mut ls, &mut system_state, *staked_aiy_id, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);
    assert_eq!(ls::aiy_balance(&ls), 120 * MIST_PER_AIY);
    assert_eq!(vec_map::length(ls::staked_aiy(&ls)), 0);

    destroy(ls);
    test_scenario::end(scenario_val);
}

#[test]
fun test_unlock_correct_epoch() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_aiy_system_state(vector[@0x1, @0x2, @0x3]);

    let mut ls = ls::new(2, test_scenario::ctx(scenario));

    ls::deposit_aiy(&mut ls, balance::create_for_testing(100 * MIST_PER_AIY));

    assert_eq!(ls::aiy_balance(&ls), 100 * MIST_PER_AIY);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<AiySystemState>(scenario);
    ls::stake(&mut ls, &mut system_state, 10 * MIST_PER_AIY, @0x1, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);

    advance_epoch(scenario);
    advance_epoch(scenario);
    advance_epoch(scenario);
    advance_epoch(scenario);

    let (staked_aiy, aiy_balance) = ls::unlock(ls, test_scenario::ctx(scenario));
    assert_eq!(balance::value(&aiy_balance), 90 * MIST_PER_AIY);
    assert_eq!(vec_map::length(&staked_aiy), 1);

    destroy(staked_aiy);
    destroy(aiy_balance);
    test_scenario::end(scenario_val);
}

#[test]
#[expected_failure(abort_code = epoch_time_lock::EEpochNotYetEnded)]
fun test_unlock_incorrect_epoch() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_aiy_system_state(vector[@0x1, @0x2, @0x3]);

    let ls = ls::new(2, test_scenario::ctx(scenario));
    let (staked_aiy, aiy_balance) = ls::unlock(ls, test_scenario::ctx(scenario));
    destroy(staked_aiy);
    destroy(aiy_balance);
    test_scenario::end(scenario_val);
}
