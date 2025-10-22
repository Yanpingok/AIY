// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module locked_stake::locked_stake;

use locked_stake::epoch_time_lock::{Self, EpochTimeLock};
use aiy::balance::{Self, Balance};
use aiy::coin;
use aiy::aiy::AIY;
use aiy::vec_map::{Self, VecMap};
use aiy_system::staking_pool::StakedAiy;
use aiy_system::aiy_system::{Self, AiySystemState};

const EInsufficientBalance: u64 = 0;
const EStakeObjectNonExistent: u64 = 1;

/// An object that locks AIY tokens and stake objects until a given epoch, and allows
/// staking and unstaking operations when locked.
public struct LockedStake has key {
    id: UID,
    staked_aiy: VecMap<ID, StakedAiy>,
    aiy: Balance<AIY>,
    locked_until_epoch: EpochTimeLock,
}

// ============================= basic operations =============================

/// Create a new LockedStake object with empty staked_aiy and aiy balance given a lock time.
/// Aborts if the given epoch has already passed.
public fun new(locked_until_epoch: u64, ctx: &mut TxContext): LockedStake {
    LockedStake {
        id: object::new(ctx),
        staked_aiy: vec_map::empty(),
        aiy: balance::zero(),
        locked_until_epoch: epoch_time_lock::new(locked_until_epoch, ctx),
    }
}

/// Unlocks and returns all the assets stored inside this LockedStake object.
/// Aborts if the unlock epoch is in the future.
public fun unlock(ls: LockedStake, ctx: &TxContext): (VecMap<ID, StakedAiy>, Balance<AIY>) {
    let LockedStake { id, staked_aiy, aiy, locked_until_epoch } = ls;
    epoch_time_lock::destroy(locked_until_epoch, ctx);
    object::delete(id);
    (staked_aiy, aiy)
}

/// Deposit a new stake object to the LockedStake object.
public fun deposit_staked_aiy(ls: &mut LockedStake, staked_aiy: StakedAiy) {
    let id = object::id(&staked_aiy);
    // This insertion can't abort since each object has a unique id.
    vec_map::insert(&mut ls.staked_aiy, id, staked_aiy);
}

/// Deposit aiy balance to the LockedStake object.
public fun deposit_aiy(ls: &mut LockedStake, aiy: Balance<AIY>) {
    balance::join(&mut ls.aiy, aiy);
}

/// Take `amount` of AIY from the aiy balance, stakes it, and puts the stake object
/// back into the staked aiy vec map.
public fun stake(
    ls: &mut LockedStake,
    aiy_system: &mut AiySystemState,
    amount: u64,
    validator_address: address,
    ctx: &mut TxContext,
) {
    assert!(balance::value(&ls.aiy) >= amount, EInsufficientBalance);
    let stake = aiy_system::request_add_stake_non_entry(
        aiy_system,
        coin::from_balance(balance::split(&mut ls.aiy, amount), ctx),
        validator_address,
        ctx,
    );
    deposit_staked_aiy(ls, stake);
}

/// Unstake the stake object with `staked_aiy_id` and puts the resulting principal
/// and rewards back into the locked aiy balance.
/// Returns the amount of AIY unstaked, including both principal and rewards.
/// Aborts if no stake exists with the given id.
public fun unstake(
    ls: &mut LockedStake,
    aiy_system: &mut AiySystemState,
    staked_aiy_id: ID,
    ctx: &mut TxContext,
): u64 {
    assert!(vec_map::contains(&ls.staked_aiy, &staked_aiy_id), EStakeObjectNonExistent);
    let (_, stake) = vec_map::remove(&mut ls.staked_aiy, &staked_aiy_id);
    let aiy_balance = aiy_system::request_withdraw_stake_non_entry(aiy_system, stake, ctx);
    let amount = balance::value(&aiy_balance);
    deposit_aiy(ls, aiy_balance);
    amount
}

// ============================= getters =============================

public fun staked_aiy(ls: &LockedStake): &VecMap<ID, StakedAiy> {
    &ls.staked_aiy
}

public fun aiy_balance(ls: &LockedStake): u64 {
    balance::value(&ls.aiy)
}

public fun locked_until_epoch(ls: &LockedStake): u64 {
    epoch_time_lock::epoch(&ls.locked_until_epoch)
}

// TODO: possibly add some scenarios like switching stake, creating a new LockedStake and transferring
// it to the sender, etc. But these can also be done as PTBs.
