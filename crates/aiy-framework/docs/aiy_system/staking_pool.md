---
title: Module `aiy_system::staking_pool`
---



-  [Struct `StakingPool`](#aiy_system_staking_pool_StakingPool)
-  [Struct `PoolTokenExchangeRate`](#aiy_system_staking_pool_PoolTokenExchangeRate)
-  [Struct `StakedAiy`](#aiy_system_staking_pool_StakedAiy)
-  [Struct `FungibleStakedAiy`](#aiy_system_staking_pool_FungibleStakedAiy)
-  [Struct `FungibleStakedAiyData`](#aiy_system_staking_pool_FungibleStakedAiyData)
-  [Struct `FungibleStakedAiyDataKey`](#aiy_system_staking_pool_FungibleStakedAiyDataKey)
-  [Struct `UnderflowAiyBalance`](#aiy_system_staking_pool_UnderflowAiyBalance)
-  [Constants](#@Constants_0)
-  [Function `new`](#aiy_system_staking_pool_new)
-  [Function `request_add_stake`](#aiy_system_staking_pool_request_add_stake)
-  [Function `request_withdraw_stake`](#aiy_system_staking_pool_request_withdraw_stake)
-  [Function `redeem_fungible_staked_aiy`](#aiy_system_staking_pool_redeem_fungible_staked_aiy)
-  [Function `calculate_fungible_staked_aiy_withdraw_amount`](#aiy_system_staking_pool_calculate_fungible_staked_aiy_withdraw_amount)
-  [Function `convert_to_fungible_staked_aiy`](#aiy_system_staking_pool_convert_to_fungible_staked_aiy)
-  [Function `withdraw_from_principal`](#aiy_system_staking_pool_withdraw_from_principal)
-  [Function `unwrap_staked_aiy`](#aiy_system_staking_pool_unwrap_staked_aiy)
-  [Function `deposit_rewards`](#aiy_system_staking_pool_deposit_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#aiy_system_staking_pool_process_pending_stakes_and_withdraws)
-  [Function `process_pending_stake_withdraw`](#aiy_system_staking_pool_process_pending_stake_withdraw)
-  [Function `process_pending_stake`](#aiy_system_staking_pool_process_pending_stake)
-  [Function `withdraw_rewards`](#aiy_system_staking_pool_withdraw_rewards)
-  [Function `activate_staking_pool`](#aiy_system_staking_pool_activate_staking_pool)
-  [Function `deactivate_staking_pool`](#aiy_system_staking_pool_deactivate_staking_pool)
-  [Function `aiy_balance`](#aiy_system_staking_pool_aiy_balance)
-  [Function `pool_id`](#aiy_system_staking_pool_pool_id)
-  [Function `fungible_staked_aiy_pool_id`](#aiy_system_staking_pool_fungible_staked_aiy_pool_id)
-  [Function `staked_aiy_amount`](#aiy_system_staking_pool_staked_aiy_amount)
-  [Function `stake_activation_epoch`](#aiy_system_staking_pool_stake_activation_epoch)
-  [Function `is_preactive`](#aiy_system_staking_pool_is_preactive)
-  [Function `activation_epoch`](#aiy_system_staking_pool_activation_epoch)
-  [Function `is_inactive`](#aiy_system_staking_pool_is_inactive)
-  [Function `fungible_staked_aiy_value`](#aiy_system_staking_pool_fungible_staked_aiy_value)
-  [Function `split_fungible_staked_aiy`](#aiy_system_staking_pool_split_fungible_staked_aiy)
-  [Function `join_fungible_staked_aiy`](#aiy_system_staking_pool_join_fungible_staked_aiy)
-  [Function `split`](#aiy_system_staking_pool_split)
-  [Function `split_staked_aiy`](#aiy_system_staking_pool_split_staked_aiy)
-  [Function `join_staked_aiy`](#aiy_system_staking_pool_join_staked_aiy)
-  [Function `is_equal_staking_metadata`](#aiy_system_staking_pool_is_equal_staking_metadata)
-  [Function `pool_token_exchange_rate_at_epoch`](#aiy_system_staking_pool_pool_token_exchange_rate_at_epoch)
-  [Function `pending_stake_amount`](#aiy_system_staking_pool_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#aiy_system_staking_pool_pending_stake_withdraw_amount)
-  [Function `exchange_rates`](#aiy_system_staking_pool_exchange_rates)
-  [Function `aiy_amount`](#aiy_system_staking_pool_aiy_amount)
-  [Function `pool_token_amount`](#aiy_system_staking_pool_pool_token_amount)
-  [Function `is_preactive_at_epoch`](#aiy_system_staking_pool_is_preactive_at_epoch)
-  [Function `get_aiy_amount`](#aiy_system_staking_pool_get_aiy_amount)
-  [Function `get_token_amount`](#aiy_system_staking_pool_get_token_amount)
-  [Function `initial_exchange_rate`](#aiy_system_staking_pool_initial_exchange_rate)
-  [Function `check_balance_invariants`](#aiy_system_staking_pool_check_balance_invariants)
-  [Macro function `mul_div`](#aiy_system_staking_pool_mul_div)
-  [Function `calculate_rewards`](#aiy_system_staking_pool_calculate_rewards)


<pre><code><b>use</b> <a href="../aiy/accumulator.md#aiy_accumulator">aiy::accumulator</a>;
<b>use</b> <a href="../aiy/accumulator_metadata.md#aiy_accumulator_metadata">aiy::accumulator_metadata</a>;
<b>use</b> <a href="../aiy/accumulator_settlement.md#aiy_accumulator_settlement">aiy::accumulator_settlement</a>;
<b>use</b> <a href="../aiy/address.md#aiy_address">aiy::address</a>;
<b>use</b> <a href="../aiy/aiy.md#aiy_aiy">aiy::aiy</a>;
<b>use</b> <a href="../aiy/bag.md#aiy_bag">aiy::bag</a>;
<b>use</b> <a href="../aiy/balance.md#aiy_balance">aiy::balance</a>;
<b>use</b> <a href="../aiy/bcs.md#aiy_bcs">aiy::bcs</a>;
<b>use</b> <a href="../aiy/coin.md#aiy_coin">aiy::coin</a>;
<b>use</b> <a href="../aiy/config.md#aiy_config">aiy::config</a>;
<b>use</b> <a href="../aiy/deny_list.md#aiy_deny_list">aiy::deny_list</a>;
<b>use</b> <a href="../aiy/dynamic_field.md#aiy_dynamic_field">aiy::dynamic_field</a>;
<b>use</b> <a href="../aiy/dynamic_object_field.md#aiy_dynamic_object_field">aiy::dynamic_object_field</a>;
<b>use</b> <a href="../aiy/event.md#aiy_event">aiy::event</a>;
<b>use</b> <a href="../aiy/funds_accumulator.md#aiy_funds_accumulator">aiy::funds_accumulator</a>;
<b>use</b> <a href="../aiy/hash.md#aiy_hash">aiy::hash</a>;
<b>use</b> <a href="../aiy/hex.md#aiy_hex">aiy::hex</a>;
<b>use</b> <a href="../aiy/object.md#aiy_object">aiy::object</a>;
<b>use</b> <a href="../aiy/party.md#aiy_party">aiy::party</a>;
<b>use</b> <a href="../aiy/table.md#aiy_table">aiy::table</a>;
<b>use</b> <a href="../aiy/transfer.md#aiy_transfer">aiy::transfer</a>;
<b>use</b> <a href="../aiy/tx_context.md#aiy_tx_context">aiy::tx_context</a>;
<b>use</b> <a href="../aiy/types.md#aiy_types">aiy::types</a>;
<b>use</b> <a href="../aiy/url.md#aiy_url">aiy::url</a>;
<b>use</b> <a href="../aiy/vec_map.md#aiy_vec_map">aiy::vec_map</a>;
<b>use</b> <a href="../aiy/vec_set.md#aiy_vec_set">aiy::vec_set</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="aiy_system_staking_pool_StakingPool"></a>

## Struct `StakingPool`

A staking pool embedded in each validator struct in the system state object.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../aiy/object.md#aiy_object_UID">aiy::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch at which this pool became active.
 The value is <code>None</code> if the pool is pre-active and <code>Some(&lt;epoch_number&gt;)</code> if active or inactive.
</dd>
<dt>
<code>deactivation_epoch: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch at which this staking pool ceased to be active. <code>None</code> = {pre-active, active},
 <code>Some(&lt;epoch_number&gt;)</code> if in-active, and it was de-activated at epoch <code>&lt;epoch_number&gt;</code>.
</dd>
<dt>
<code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a>: u64</code>
</dt>
<dd>
 The total number of AIY tokens in this pool, including the AIY in the rewards_pool, as well as in all the principal
 in the <code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a></code> object, updated at epoch boundaries.
</dd>
<dt>
<code>rewards_pool: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;</code>
</dt>
<dd>
 The epoch stake rewards will be added here at the end of each epoch.
</dd>
<dt>
<code>pool_token_balance: u64</code>
</dt>
<dd>
 Total number of pool tokens issued by the pool.
</dd>
<dt>
<code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_exchange_rates">exchange_rates</a>: <a href="../aiy/table.md#aiy_table_Table">aiy::table::Table</a>&lt;u64, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">aiy_system::staking_pool::PoolTokenExchangeRate</a>&gt;</code>
</dt>
<dd>
 Exchange rate history of previous epochs. Key is the epoch number.
 The entries start from the <code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a></code> of this pool and contains exchange rates at the beginning of each epoch,
 i.e., right after the rewards for the previous epoch have been deposited into the pool.
</dd>
<dt>
<code>pending_stake: u64</code>
</dt>
<dd>
 Pending stake amount for this epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>pending_total_aiy_withdraw: u64</code>
</dt>
<dd>
 Pending stake withdrawn during the current epoch, emptied at epoch boundaries.
 This includes both the principal and rewards AIY withdrawn.
</dd>
<dt>
<code>pending_pool_token_withdraw: u64</code>
</dt>
<dd>
 Pending pool token withdrawn during the current epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>extra_fields: <a href="../aiy/bag.md#aiy_bag_Bag">aiy::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="aiy_system_staking_pool_PoolTokenExchangeRate"></a>

## Struct `PoolTokenExchangeRate`

Struct representing the exchange rate of the stake pool token to AIY.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="aiy_system_staking_pool_StakedAiy"></a>

## Struct `StakedAiy`

A self-custodial object holding the staked AIY tokens.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../aiy/object.md#aiy_object_UID">aiy::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>: <a href="../aiy/object.md#aiy_object_ID">aiy::object::ID</a></code>
</dt>
<dd>
 ID of the staking pool we are staking with.
</dd>
<dt>
<code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: u64</code>
</dt>
<dd>
 The epoch at which the stake becomes active.
</dd>
<dt>
<code>principal: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;</code>
</dt>
<dd>
 The staked AIY tokens.
</dd>
</dl>


</details>

<a name="aiy_system_staking_pool_FungibleStakedAiy"></a>

## Struct `FungibleStakedAiy`

An alternative to <code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a></code> that holds the pool token amount instead of the AIY balance.
StakedAiy objects can be converted to FungibleStakedAiys after the initial warmup period.
The advantage of this is that you can now merge multiple StakedAiy objects from different
activation epochs into a single FungibleStakedAiy object.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../aiy/object.md#aiy_object_UID">aiy::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>: <a href="../aiy/object.md#aiy_object_ID">aiy::object::ID</a></code>
</dt>
<dd>
 ID of the staking pool we are staking with.
</dd>
<dt>
<code>value: u64</code>
</dt>
<dd>
 The pool token amount.
</dd>
</dl>


</details>

<a name="aiy_system_staking_pool_FungibleStakedAiyData"></a>

## Struct `FungibleStakedAiyData`

Holds useful information


<pre><code><b>public</b> <b>struct</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiyData">FungibleStakedAiyData</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../aiy/object.md#aiy_object_UID">aiy::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>total_supply: u64</code>
</dt>
<dd>
 fungible_staked_aiy supply
</dd>
<dt>
<code>principal: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;</code>
</dt>
<dd>
 principal balance. Rewards are withdrawn from the reward pool
</dd>
</dl>


</details>

<a name="aiy_system_staking_pool_FungibleStakedAiyDataKey"></a>

## Struct `FungibleStakedAiyDataKey`



<pre><code><b>public</b> <b>struct</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiyDataKey">FungibleStakedAiyDataKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="aiy_system_staking_pool_UnderflowAiyBalance"></a>

## Struct `UnderflowAiyBalance`

Holds the amount of AIY that was underflowed when withdrawing from the pool
post safe mode. Cleaned up in the same transaction.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_UnderflowAiyBalance">UnderflowAiyBalance</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="aiy_system_staking_pool_MIN_STAKING_THRESHOLD"></a>

StakedAiy objects cannot be split to below this amount.


<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: u64 = 1000000000;
</code></pre>



<a name="aiy_system_staking_pool_EInsufficientPoolTokenBalance"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>: u64 = 0;
</code></pre>



<a name="aiy_system_staking_pool_EWrongPool"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EWrongPool">EWrongPool</a>: u64 = 1;
</code></pre>



<a name="aiy_system_staking_pool_EWithdrawAmountCannotBeZero"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EWithdrawAmountCannotBeZero">EWithdrawAmountCannotBeZero</a>: u64 = 2;
</code></pre>



<a name="aiy_system_staking_pool_EInsufficientAiyTokenBalance"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EInsufficientAiyTokenBalance">EInsufficientAiyTokenBalance</a>: u64 = 3;
</code></pre>



<a name="aiy_system_staking_pool_EInsufficientRewardsPoolBalance"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EInsufficientRewardsPoolBalance">EInsufficientRewardsPoolBalance</a>: u64 = 4;
</code></pre>



<a name="aiy_system_staking_pool_EDestroyNonzeroBalance"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EDestroyNonzeroBalance">EDestroyNonzeroBalance</a>: u64 = 5;
</code></pre>



<a name="aiy_system_staking_pool_ETokenTimeLockIsSome"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_ETokenTimeLockIsSome">ETokenTimeLockIsSome</a>: u64 = 6;
</code></pre>



<a name="aiy_system_staking_pool_EWrongDelegation"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EWrongDelegation">EWrongDelegation</a>: u64 = 7;
</code></pre>



<a name="aiy_system_staking_pool_EPendingDelegationDoesNotExist"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EPendingDelegationDoesNotExist">EPendingDelegationDoesNotExist</a>: u64 = 8;
</code></pre>



<a name="aiy_system_staking_pool_ETokenBalancesDoNotMatchExchangeRate"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>: u64 = 9;
</code></pre>



<a name="aiy_system_staking_pool_EDelegationToInactivePool"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>: u64 = 10;
</code></pre>



<a name="aiy_system_staking_pool_EDeactivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>: u64 = 11;
</code></pre>



<a name="aiy_system_staking_pool_EIncompatibleStakedAiy"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EIncompatibleStakedAiy">EIncompatibleStakedAiy</a>: u64 = 12;
</code></pre>



<a name="aiy_system_staking_pool_EWithdrawalInSameEpoch"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EWithdrawalInSameEpoch">EWithdrawalInSameEpoch</a>: u64 = 13;
</code></pre>



<a name="aiy_system_staking_pool_EPoolAlreadyActive"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>: u64 = 14;
</code></pre>



<a name="aiy_system_staking_pool_EPoolPreactiveOrInactive"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EPoolPreactiveOrInactive">EPoolPreactiveOrInactive</a>: u64 = 15;
</code></pre>



<a name="aiy_system_staking_pool_EActivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>: u64 = 16;
</code></pre>



<a name="aiy_system_staking_pool_EDelegationOfZeroAiy"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EDelegationOfZeroAiy">EDelegationOfZeroAiy</a>: u64 = 17;
</code></pre>



<a name="aiy_system_staking_pool_EStakedAiyBelowThreshold"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EStakedAiyBelowThreshold">EStakedAiyBelowThreshold</a>: u64 = 18;
</code></pre>



<a name="aiy_system_staking_pool_ECannotMintFungibleStakedAiyYet"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_ECannotMintFungibleStakedAiyYet">ECannotMintFungibleStakedAiyYet</a>: u64 = 19;
</code></pre>



<a name="aiy_system_staking_pool_EInvariantFailure"></a>



<pre><code><b>const</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EInvariantFailure">EInvariantFailure</a>: u64 = 20;
</code></pre>



<a name="aiy_system_staking_pool_new"></a>

## Function `new`

Create a new, empty staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_new">new</a>(ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a> {
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a> {
        id: object::new(ctx),
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>: option::none(),
        deactivation_epoch: option::none(),
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a>: 0,
        rewards_pool: balance::zero(),
        pool_token_balance: 0,
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_exchange_rates">exchange_rates</a>: table::new(ctx),
        pending_stake: 0,
        pending_total_aiy_withdraw: 0,
        pending_pool_token_withdraw: 0,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_request_add_stake"></a>

## Function `request_add_stake`

Request to stake to a staking pool. The stake starts counting at the beginning of the next epoch,


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_request_add_stake">request_add_stake</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, stake: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: u64, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_request_add_stake">request_add_stake</a>(
    pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>,
    stake: Balance&lt;AIY&gt;,
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a> {
    <b>let</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a> = stake.value();
    <b>assert</b>!(!pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_inactive">is_inactive</a>(), <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>);
    <b>assert</b>!(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a> &gt; 0, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EDelegationOfZeroAiy">EDelegationOfZeroAiy</a>);
    pool.pending_stake = pool.pending_stake + <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>;
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a> {
        id: object::new(ctx),
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>: object::id(pool),
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>,
        principal: stake,
    }
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw the given stake plus rewards from a staking pool.
Both the principal and corresponding rewards in AIY are withdrawn.
A proportional amount of pool token withdraw is recorded and processed at epoch change time.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, staked_aiy: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>, ctx: &<a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(
    pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>,
    staked_aiy: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>,
    ctx: &TxContext,
): Balance&lt;AIY&gt; {
    // stake is inactive and the pool is not preactive - allow direct withdraw
    // the reason why we exclude preactive pools is to avoid potential underflow
    // on subtraction, and we need to enforce `pending_stake_withdraw` call.
    <b>if</b> (staked_aiy.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a> &gt; ctx.epoch() && !pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_preactive">is_preactive</a>()) {
        <b>let</b> principal = staked_aiy.into_balance();
        pool.pending_stake = pool.pending_stake - principal.value();
        <b>return</b> principal
    };
    <b>let</b> (pool_token_withdraw_amount, <b>mut</b> principal_withdraw) = pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(
        staked_aiy,
    );
    <b>let</b> principal_withdraw_amount = principal_withdraw.value();
    <b>let</b> rewards_withdraw = pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_withdraw_rewards">withdraw_rewards</a>(
        principal_withdraw_amount,
        pool_token_withdraw_amount,
        ctx.epoch(),
    );
    <b>let</b> total_aiy_withdraw_amount = principal_withdraw_amount + rewards_withdraw.value();
    pool.pending_total_aiy_withdraw = pool.pending_total_aiy_withdraw + total_aiy_withdraw_amount;
    pool.pending_pool_token_withdraw =
        pool.pending_pool_token_withdraw + pool_token_withdraw_amount;
    // If the pool is inactive or preactive, we immediately process the withdrawal.
    <b>if</b> (pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_inactive">is_inactive</a>() || pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_preactive">is_preactive</a>()) pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>();
    // TODO: implement withdraw bonding period here.
    principal_withdraw.join(rewards_withdraw);
    principal_withdraw
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_redeem_fungible_staked_aiy"></a>

## Function `redeem_fungible_staked_aiy`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_redeem_fungible_staked_aiy">redeem_fungible_staked_aiy</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, fungible_staked_aiy: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">aiy_system::staking_pool::FungibleStakedAiy</a>, ctx: &<a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_redeem_fungible_staked_aiy">redeem_fungible_staked_aiy</a>(
    pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>,
    fungible_staked_aiy: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a>,
    ctx: &TxContext,
): Balance&lt;AIY&gt; {
    <b>let</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a> { id, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>, value } = fungible_staked_aiy;
    <b>assert</b>!(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a> == object::id(pool), <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EWrongPool">EWrongPool</a>);
    id.delete();
    <b>let</b> latest_exchange_rate = pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(ctx.epoch());
    <b>let</b> fungible_staked_aiy_data: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiyData">FungibleStakedAiyData</a> =
        &<b>mut</b> pool.extra_fields[<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiyDataKey">FungibleStakedAiyDataKey</a> {}];
    <b>let</b> (
        principal_amount,
        rewards_amount,
    ) = latest_exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_calculate_fungible_staked_aiy_withdraw_amount">calculate_fungible_staked_aiy_withdraw_amount</a>(
        value,
        fungible_staked_aiy_data.principal.value(),
        fungible_staked_aiy_data.total_supply,
    );
    fungible_staked_aiy_data.total_supply = fungible_staked_aiy_data.total_supply - value;
    <b>let</b> <b>mut</b> aiy_out = fungible_staked_aiy_data.principal.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_split">split</a>(principal_amount);
    aiy_out.join(pool.rewards_pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_split">split</a>(rewards_amount));
    pool.pending_total_aiy_withdraw = pool.pending_total_aiy_withdraw + aiy_out.value();
    pool.pending_pool_token_withdraw = pool.pending_pool_token_withdraw + value;
    aiy_out
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_calculate_fungible_staked_aiy_withdraw_amount"></a>

## Function `calculate_fungible_staked_aiy_withdraw_amount`

written in separate function so i can test with random values
returns (principal_withdraw_amount, rewards_withdraw_amount)


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_calculate_fungible_staked_aiy_withdraw_amount">calculate_fungible_staked_aiy_withdraw_amount</a>(latest_exchange_rate: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">aiy_system::staking_pool::PoolTokenExchangeRate</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_fungible_staked_aiy_value">fungible_staked_aiy_value</a>: u64, fungible_staked_aiy_data_principal_amount: u64, fungible_staked_aiy_data_total_supply: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_calculate_fungible_staked_aiy_withdraw_amount">calculate_fungible_staked_aiy_withdraw_amount</a>(
    latest_exchange_rate: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>,
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_fungible_staked_aiy_value">fungible_staked_aiy_value</a>: u64,
    fungible_staked_aiy_data_principal_amount: u64, // fungible_staked_aiy_data.principal.value()
    fungible_staked_aiy_data_total_supply: u64, // fungible_staked_aiy_data.total_supply
): (u64, u64) {
    // 1. <b>if</b> the entire <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiyData">FungibleStakedAiyData</a> supply is redeemed, how much aiy should we receive?
    <b>let</b> total_aiy_amount = latest_exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_aiy_amount">get_aiy_amount</a>(
        fungible_staked_aiy_data_total_supply,
    );
    // min with total_aiy_amount to prevent underflow
    <b>let</b> fungible_staked_aiy_data_principal_amount = fungible_staked_aiy_data_principal_amount.min(
        total_aiy_amount,
    );
    // 2. how much do we need to withdraw from the rewards pool?
    <b>let</b> total_rewards = total_aiy_amount - fungible_staked_aiy_data_principal_amount;
    // 3. proportionally withdraw from both wrt the <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_fungible_staked_aiy_value">fungible_staked_aiy_value</a>.
    <b>let</b> principal_withdraw_amount = <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_mul_div">mul_div</a>!(
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_fungible_staked_aiy_value">fungible_staked_aiy_value</a>,
        fungible_staked_aiy_data_principal_amount,
        fungible_staked_aiy_data_total_supply,
    );
    <b>let</b> rewards_withdraw_amount = <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_mul_div">mul_div</a>!(
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_fungible_staked_aiy_value">fungible_staked_aiy_value</a>,
        total_rewards,
        fungible_staked_aiy_data_total_supply,
    );
    // <b>invariant</b> check, just in case
    <b>let</b> expected_aiy_amount = latest_exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_aiy_amount">get_aiy_amount</a>(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_fungible_staked_aiy_value">fungible_staked_aiy_value</a>);
    <b>assert</b>!(
        principal_withdraw_amount + rewards_withdraw_amount &lt;= expected_aiy_amount,
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EInvariantFailure">EInvariantFailure</a>,
    );
    (principal_withdraw_amount, rewards_withdraw_amount)
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_convert_to_fungible_staked_aiy"></a>

## Function `convert_to_fungible_staked_aiy`

Convert the given staked AIY to an FungibleStakedAiy object


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_convert_to_fungible_staked_aiy">convert_to_fungible_staked_aiy</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, staked_aiy: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">aiy_system::staking_pool::FungibleStakedAiy</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_convert_to_fungible_staked_aiy">convert_to_fungible_staked_aiy</a>(
    pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>,
    staked_aiy: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a> {
    <b>let</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a> { id, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>, principal } = staked_aiy;
    <b>assert</b>!(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a> == object::id(pool), <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EWrongPool">EWrongPool</a>);
    <b>assert</b>!(ctx.epoch() &gt;= <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_ECannotMintFungibleStakedAiyYet">ECannotMintFungibleStakedAiyYet</a>);
    <b>assert</b>!(!pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_preactive">is_preactive</a>() && !pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_inactive">is_inactive</a>(), <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EPoolPreactiveOrInactive">EPoolPreactiveOrInactive</a>);
    id.delete();
    <b>let</b> exchange_rate_at_staking_epoch = pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>,
    );
    <b>let</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a> = exchange_rate_at_staking_epoch.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_token_amount">get_token_amount</a>(principal.value());
    <b>let</b> key = <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiyDataKey">FungibleStakedAiyDataKey</a> {};
    <b>if</b> (!pool.extra_fields.contains(key)) {
        pool
            .extra_fields
            .add(
                key,
                <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiyData">FungibleStakedAiyData</a> {
                    id: object::new(ctx),
                    total_supply: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>,
                    principal,
                },
            );
    } <b>else</b> {
        <b>let</b> fungible_staked_aiy_data: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiyData">FungibleStakedAiyData</a> = &<b>mut</b> pool.extra_fields[key];
        fungible_staked_aiy_data.total_supply =
            fungible_staked_aiy_data.total_supply + <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>;
        fungible_staked_aiy_data.principal.join(principal);
    };
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a> {
        id: object::new(ctx),
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>,
        value: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>,
    }
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_withdraw_from_principal"></a>

## Function `withdraw_from_principal`

Withdraw the principal AIY stored in the StakedAiy object, and calculate the corresponding amount of pool
tokens using exchange rate at staking epoch.
Returns values are amount of pool tokens withdrawn and withdrawn principal portion of AIY.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, staked_aiy: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>): (u64, <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(
    pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>,
    staked_aiy: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>,
): (u64, Balance&lt;AIY&gt;) {
    // Check that the stake information matches the pool.
    <b>assert</b>!(staked_aiy.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a> == object::id(pool), <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EWrongPool">EWrongPool</a>);
    <b>let</b> exchange_rate_at_staking_epoch = pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(staked_aiy.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>);
    <b>let</b> principal_withdraw = staked_aiy.into_balance();
    <b>let</b> pool_token_withdraw_amount = exchange_rate_at_staking_epoch.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_token_amount">get_token_amount</a>(principal_withdraw.value());
    (pool_token_withdraw_amount, principal_withdraw)
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_unwrap_staked_aiy"></a>

## Function `unwrap_staked_aiy`



<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_unwrap_staked_aiy">unwrap_staked_aiy</a>(staked_aiy: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_unwrap_staked_aiy">unwrap_staked_aiy</a>(staked_aiy: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>): Balance&lt;AIY&gt; {
    <b>let</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a> { id, principal, .. } = staked_aiy;
    id.delete();
    principal
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_deposit_rewards"></a>

## Function `deposit_rewards`

Called at epoch advancement times to add rewards (in AIY) to the staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, rewards: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>, rewards: Balance&lt;AIY&gt;) {
    pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a> = pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a> + rewards.value();
    pool.rewards_pool.join(rewards);
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, ctx: &<a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>, ctx: &TxContext) {
    <b>let</b> new_epoch = ctx.epoch() + 1;
    pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>();
    pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_process_pending_stake">process_pending_stake</a>();
    pool
        .<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_exchange_rates">exchange_rates</a>
        .add(
            new_epoch,
            <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
                <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>: pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a>,
                <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>: pool.pool_token_balance,
            },
        );
    pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_check_balance_invariants">check_balance_invariants</a>(new_epoch);
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_process_pending_stake_withdraw"></a>

## Function `process_pending_stake_withdraw`

Called at epoch boundaries to process pending stake withdraws requested during the epoch.
Also called immediately upon withdrawal if the pool is inactive.


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>) {
    pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a> = <b>if</b> (pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a> &gt;= pool.pending_total_aiy_withdraw) {
        pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a> - pool.pending_total_aiy_withdraw
    } <b>else</b> {
        // the diff will be applied in the `<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_process_pending_stake">process_pending_stake</a>` function.
        <b>let</b> diff = pool.pending_total_aiy_withdraw - pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a>;
        pool.extra_fields.add(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_UnderflowAiyBalance">UnderflowAiyBalance</a> {}, diff);
        0
    };
    pool.pool_token_balance = <b>if</b> (pool.pool_token_balance &gt;= pool.pending_pool_token_withdraw) {
        pool.pool_token_balance - pool.pending_pool_token_withdraw
    } <b>else</b> {
        0
    };
    pool.pending_total_aiy_withdraw = 0;
    pool.pending_pool_token_withdraw = 0;
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_process_pending_stake"></a>

## Function `process_pending_stake`

Called at epoch boundaries to process the pending stake.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>) {
    // Use the most up to date exchange rate with the rewards deposited and withdraws effectuated.
    <b>let</b> latest_exchange_rate = <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>: pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a>,
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>: pool.pool_token_balance,
    };
    // This key is only present <b>if</b> the `<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a>` underflowed, hence, the current value of `<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a>`
    // is `0`. Pool token balance will be recalculated automatically <b>for</b> `0` value.
    <b>let</b> aiy_diff = {
        <b>let</b> key = <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_UnderflowAiyBalance">UnderflowAiyBalance</a> {};
        <b>if</b> (pool.extra_fields.contains(key)) pool.extra_fields.remove(key) <b>else</b> 0
    };
    pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a> = pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a> + pool.pending_stake - aiy_diff;
    pool.pool_token_balance = latest_exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_token_amount">get_token_amount</a>(pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a>);
    pool.pending_stake = 0;
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_withdraw_rewards"></a>

## Function `withdraw_rewards`

This function does the following:
1. Calculates the total amount of AIY (including principal and rewards) that the provided pool tokens represent
at the current exchange rate.
2. Using the above number and the given <code>principal_withdraw_amount</code>, calculates the rewards portion of the
stake we should withdraw.
3. Withdraws the rewards portion from the rewards pool at the current exchange rate. We only withdraw the rewards
portion because the principal portion was already taken out of the staker's self custodied StakedAiy.


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_withdraw_rewards">withdraw_rewards</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, principal_withdraw_amount: u64, pool_token_withdraw_amount: u64, epoch: u64): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_withdraw_rewards">withdraw_rewards</a>(
    pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>,
    principal_withdraw_amount: u64,
    pool_token_withdraw_amount: u64,
    epoch: u64,
): Balance&lt;AIY&gt; {
    <b>let</b> exchange_rate = pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(epoch);
    <b>let</b> total_aiy_withdraw_amount = exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_aiy_amount">get_aiy_amount</a>(pool_token_withdraw_amount);
    <b>let</b> <b>mut</b> reward_withdraw_amount = <b>if</b> (total_aiy_withdraw_amount &gt;= principal_withdraw_amount) {
        total_aiy_withdraw_amount - principal_withdraw_amount
    } <b>else</b> 0;
    // This may happen when we are withdrawing everything from the pool and
    // the rewards pool balance may be less than reward_withdraw_amount.
    // TODO: FIGURE OUT EXACTLY WHY THIS CAN HAPPEN.
    reward_withdraw_amount = reward_withdraw_amount.min(pool.rewards_pool.value());
    pool.rewards_pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_split">split</a>(reward_withdraw_amount)
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_activate_staking_pool"></a>

## Function `activate_staking_pool`

Called by <code><a href="../aiy_system/validator.md#aiy_system_validator">validator</a></code> module to activate a staking pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>: u64) {
    // Add the initial exchange rate to the table.
    pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_exchange_rates">exchange_rates</a>.add(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>());
    // Check that the pool is preactive and not inactive.
    <b>assert</b>!(pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_preactive">is_preactive</a>(), <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>);
    <b>assert</b>!(!pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_inactive">is_inactive</a>(), <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>);
    // Fill in the active epoch.
    pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>.fill(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>);
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_deactivate_staking_pool"></a>

## Function `deactivate_staking_pool`

Deactivate a staking pool by setting the <code>deactivation_epoch</code>. After
this pool deactivation, the pool stops earning rewards. Only stake
withdraws can be made to the pool.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>, deactivation_epoch: u64) {
    // We can't deactivate an already deactivated pool.
    <b>assert</b>!(!pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_inactive">is_inactive</a>(), <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>);
    pool.deactivation_epoch = option::some(deactivation_epoch);
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_aiy_balance"></a>

## Function `aiy_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>): u64 { pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a> }
</code></pre>



</details>

<a name="aiy_system_staking_pool_pool_id"></a>

## Function `pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>(staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>): <a href="../aiy/object.md#aiy_object_ID">aiy::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>(staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>): ID { staked_aiy.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a> }
</code></pre>



</details>

<a name="aiy_system_staking_pool_fungible_staked_aiy_pool_id"></a>

## Function `fungible_staked_aiy_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_fungible_staked_aiy_pool_id">fungible_staked_aiy_pool_id</a>(fungible_staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">aiy_system::staking_pool::FungibleStakedAiy</a>): <a href="../aiy/object.md#aiy_object_ID">aiy::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_fungible_staked_aiy_pool_id">fungible_staked_aiy_pool_id</a>(fungible_staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a>): ID {
    fungible_staked_aiy.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_staked_aiy_amount"></a>

## Function `staked_aiy_amount`

Returns the principal amount of <code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_staked_aiy_amount">staked_aiy_amount</a>(staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_staked_aiy_amount">staked_aiy_amount</a>(staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>): u64 { staked_aiy.principal.value() }
</code></pre>



</details>

<a name="aiy_system_staking_pool_stake_activation_epoch"></a>

## Function `stake_activation_epoch`

Returns the activation epoch of <code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>(staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>(staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>): u64 {
    staked_aiy.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_is_preactive"></a>

## Function `is_preactive`

Returns true if the input staking pool is preactive.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_preactive">is_preactive</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_preactive">is_preactive</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>): bool {
    pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>.is_none()
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_activation_epoch"></a>

## Function `activation_epoch`

Returns the activation epoch of the <code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a></code>. For validator candidates,
or pending validators, the value returned is <code>None</code>. For active validators,
the value is the epoch before the validator was activated.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>): Option&lt;u64&gt; {
    pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_is_inactive"></a>

## Function `is_inactive`

Returns true if the input staking pool is inactive.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_inactive">is_inactive</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_inactive">is_inactive</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>): bool {
    pool.deactivation_epoch.is_some()
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_fungible_staked_aiy_value"></a>

## Function `fungible_staked_aiy_value`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_fungible_staked_aiy_value">fungible_staked_aiy_value</a>(fungible_staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">aiy_system::staking_pool::FungibleStakedAiy</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_fungible_staked_aiy_value">fungible_staked_aiy_value</a>(fungible_staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a>): u64 {
    fungible_staked_aiy.value
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_split_fungible_staked_aiy"></a>

## Function `split_fungible_staked_aiy`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_split_fungible_staked_aiy">split_fungible_staked_aiy</a>(fungible_staked_aiy: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">aiy_system::staking_pool::FungibleStakedAiy</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">aiy_system::staking_pool::FungibleStakedAiy</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_split_fungible_staked_aiy">split_fungible_staked_aiy</a>(
    fungible_staked_aiy: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a>,
    split_amount: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a> {
    <b>assert</b>!(split_amount &lt;= fungible_staked_aiy.value, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>);
    fungible_staked_aiy.value = fungible_staked_aiy.value - split_amount;
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a> {
        id: object::new(ctx),
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>: fungible_staked_aiy.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>,
        value: split_amount,
    }
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_join_fungible_staked_aiy"></a>

## Function `join_fungible_staked_aiy`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_join_fungible_staked_aiy">join_fungible_staked_aiy</a>(self: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">aiy_system::staking_pool::FungibleStakedAiy</a>, other: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">aiy_system::staking_pool::FungibleStakedAiy</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_join_fungible_staked_aiy">join_fungible_staked_aiy</a>(self: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a>, other: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a>) {
    <b>let</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_FungibleStakedAiy">FungibleStakedAiy</a> { id, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>, value } = other;
    <b>assert</b>!(self.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a> == <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EWrongPool">EWrongPool</a>);
    id.delete();
    self.value = self.value + value;
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_split"></a>

## Function `split`

Split StakedAiy <code>self</code> to two parts, one with principal <code>split_amount</code>,
and the remaining principal is left in <code>self</code>.
All the other parameters of the StakedAiy like <code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a></code> or <code><a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a></code> remain the same.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_split">split</a>(self: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_split">split</a>(self: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>, split_amount: u64, ctx: &<b>mut</b> TxContext): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a> {
    <b>let</b> original_amount = self.principal.value();
    <b>assert</b>!(split_amount &lt;= original_amount, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EInsufficientAiyTokenBalance">EInsufficientAiyTokenBalance</a>);
    <b>let</b> remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>.
    <b>assert</b>!(remaining_amount &gt;= <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EStakedAiyBelowThreshold">EStakedAiyBelowThreshold</a>);
    <b>assert</b>!(split_amount &gt;= <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EStakedAiyBelowThreshold">EStakedAiyBelowThreshold</a>);
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a> {
        id: object::new(ctx),
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>: self.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>,
        <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>: self.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>,
        principal: self.principal.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_split">split</a>(split_amount),
    }
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_split_staked_aiy"></a>

## Function `split_staked_aiy`

Split the given StakedAiy to the two parts, one with principal <code>split_amount</code>,
transfer the newly split part to the sender address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_split_staked_aiy">split_staked_aiy</a>(stake: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_split_staked_aiy">split_staked_aiy</a>(stake: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>, split_amount: u64, ctx: &<b>mut</b> TxContext) {
    transfer::transfer(stake.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_split">split</a>(split_amount, ctx), ctx.sender());
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_join_staked_aiy"></a>

## Function `join_staked_aiy`

Consume the staked aiy <code>other</code> and add its value to <code>self</code>.
Aborts if some of the staking parameters are incompatible (pool id, stake activation epoch, etc.)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_join_staked_aiy">join_staked_aiy</a>(self: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>, other: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_join_staked_aiy">join_staked_aiy</a>(self: &<b>mut</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>, other: <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>) {
    <b>assert</b>!(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self, &other), <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_EIncompatibleStakedAiy">EIncompatibleStakedAiy</a>);
    <b>let</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a> { id, principal, .. } = other;
    id.delete();
    self.principal.join(principal);
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_is_equal_staking_metadata"></a>

## Function `is_equal_staking_metadata`

Returns true if all the staking parameters of the staked aiy except the principal are identical


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>, other: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>, other: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>): bool {
    (self.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a> == other.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_id">pool_id</a>) &&
    (self.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a> == other.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>)
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, epoch: u64): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">aiy_system::staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(
    pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>,
    epoch: u64,
): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    // If the pool is preactive then the exchange rate is always 1:1.
    <b>if</b> (pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(epoch)) {
        <b>return</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
    };
    <b>let</b> clamped_epoch = pool.deactivation_epoch.get_with_default(epoch);
    <b>let</b> <b>mut</b> epoch = clamped_epoch.min(epoch);
    <b>let</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a> = *pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>.borrow();
    // Find the latest epoch that's earlier than the given epoch with an <b>entry</b> in the table
    <b>while</b> (epoch &gt;= <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>) {
        <b>if</b> (pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_exchange_rates">exchange_rates</a>.contains(epoch)) {
            <b>return</b> pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_exchange_rates">exchange_rates</a>[epoch]
        };
        epoch = epoch - 1;
    };
    // This line really should be unreachable. Do we want an <b>assert</b> <b>false</b> here?
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_pending_stake_amount"></a>

## Function `pending_stake_amount`

Returns the total value of the pending staking requests for this staking pool.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool">staking_pool</a>: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool">staking_pool</a>: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>): u64 {
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool">staking_pool</a>.pending_stake
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`

Returns the total withdrawal from the staking pool this epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool">staking_pool</a>: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool">staking_pool</a>: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>): u64 {
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool">staking_pool</a>.pending_total_aiy_withdraw
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_exchange_rates"></a>

## Function `exchange_rates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>): &<a href="../aiy/table.md#aiy_table_Table">aiy::table::Table</a>&lt;u64, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">aiy_system::staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>): &Table&lt;u64, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>&gt; {
    &pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_exchange_rates">exchange_rates</a>
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_aiy_amount"></a>

## Function `aiy_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>(exchange_rate: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">aiy_system::staking_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>(exchange_rate: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_pool_token_amount"></a>

## Function `pool_token_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">aiy_system::staking_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_is_preactive_at_epoch"></a>

## Function `is_preactive_at_epoch`

Returns true if the provided staking pool is preactive at the provided epoch.


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>, epoch: u64): bool {
    // Either the pool is currently preactive or the pool's starting epoch is later than the provided epoch.
    pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_is_preactive">is_preactive</a>() || (*pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_activation_epoch">activation_epoch</a>.borrow() &gt; epoch)
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_get_aiy_amount"></a>

## Function `get_aiy_amount`



<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_aiy_amount">get_aiy_amount</a>(exchange_rate: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">aiy_system::staking_pool::PoolTokenExchangeRate</a>, token_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_aiy_amount">get_aiy_amount</a>(exchange_rate: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, token_amount: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a> == 0 || exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a> == 0) {
        <b>return</b> token_amount
    };
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_mul_div">mul_div</a>!(exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>, token_amount, exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>)
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_get_token_amount"></a>

## Function `get_token_amount`



<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">aiy_system::staking_pool::PoolTokenExchangeRate</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>: u64): u64 {
    // When either amount is 0, that means we have no stakes with this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a> == 0 || exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a> == 0) {
        <b>return</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>
    };
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_mul_div">mul_div</a>!(exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>, exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>)
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_initial_exchange_rate"></a>

## Function `initial_exchange_rate`



<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">aiy_system::staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_amount">aiy_amount</a>: 0, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_amount">pool_token_amount</a>: 0 }
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_check_balance_invariants"></a>

## Function `check_balance_invariants`



<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>, epoch: u64) {
    <b>let</b> exchange_rate = pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(epoch);
    // check that the pool token balance and aiy balance ratio matches the exchange rate stored.
    <b>let</b> expected = exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_token_amount">get_token_amount</a>(pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_aiy_balance">aiy_balance</a>);
    <b>let</b> actual = pool.pool_token_balance;
    <b>assert</b>!(expected == actual, <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>)
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_mul_div"></a>

## Macro function `mul_div`



<pre><code><b>macro</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_mul_div">mul_div</a>($a: u64, $b: u64, $c: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>macro</b> <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_mul_div">mul_div</a>($a: u64, $b: u64, $c: u64): u64 {
    (($a <b>as</b> u128) * ($b <b>as</b> u128) / ($c <b>as</b> u128)) <b>as</b> u64
}
</code></pre>



</details>

<a name="aiy_system_staking_pool_calculate_rewards"></a>

## Function `calculate_rewards`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_calculate_rewards">calculate_rewards</a>(pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">aiy_system::staking_pool::StakingPool</a>, staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">aiy_system::staking_pool::StakedAiy</a>, current_epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_calculate_rewards">calculate_rewards</a>(
    pool: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakingPool">StakingPool</a>,
    staked_aiy: &<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_StakedAiy">StakedAiy</a>,
    current_epoch: u64,
): u64 {
    <b>let</b> staked_amount = staked_aiy.amount();
    <b>let</b> pool_token_withdraw_amount = {
        <b>let</b> exchange_rate_at_staking_epoch = pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(staked_aiy.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_stake_activation_epoch">stake_activation_epoch</a>);
        exchange_rate_at_staking_epoch.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_token_amount">get_token_amount</a>(staked_amount)
    };
    <b>let</b> new_epoch_exchange_rate = pool.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(current_epoch);
    <b>let</b> total_aiy_withdraw_amount = new_epoch_exchange_rate.<a href="../aiy_system/staking_pool.md#aiy_system_staking_pool_get_aiy_amount">get_aiy_amount</a>(
        pool_token_withdraw_amount,
    );
    <b>let</b> <b>mut</b> reward_withdraw_amount = <b>if</b> (total_aiy_withdraw_amount &gt;= staked_amount) {
        total_aiy_withdraw_amount - staked_amount
    } <b>else</b> 0;
    reward_withdraw_amount = reward_withdraw_amount.min(pool.rewards_pool.value());
    reward_withdraw_amount
}
</code></pre>



</details>
