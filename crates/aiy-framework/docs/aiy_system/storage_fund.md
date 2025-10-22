---
title: Module `aiy_system::storage_fund`
---



-  [Struct `StorageFund`](#aiy_system_storage_fund_StorageFund)
-  [Function `new`](#aiy_system_storage_fund_new)
-  [Function `advance_epoch`](#aiy_system_storage_fund_advance_epoch)
-  [Function `total_object_storage_rebates`](#aiy_system_storage_fund_total_object_storage_rebates)
-  [Function `total_balance`](#aiy_system_storage_fund_total_balance)


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
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="aiy_system_storage_fund_StorageFund"></a>

## Struct `StorageFund`

Struct representing the storage fund, containing two <code>Balance</code>s:
- <code><a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a></code> has the invariant that it's the sum of <code>storage_rebate</code> of
all objects currently stored on-chain. To maintain this invariant, the only inflow of this
balance is storage charges collected from transactions, and the only outflow is storage rebates
of transactions, including both the portion refunded to the transaction senders as well as
the non-refundable portion taken out and put into <code>non_refundable_balance</code>.
- <code>non_refundable_balance</code> contains any remaining inflow of the storage fund that should not
be taken out of the fund.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_StorageFund">StorageFund</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>non_refundable_balance: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="aiy_system_storage_fund_new"></a>

## Function `new`

Called by <code><a href="../aiy_system/aiy_system.md#aiy_system_aiy_system">aiy_system</a></code> at genesis time.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_new">new</a>(initial_fund: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;): <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_StorageFund">aiy_system::storage_fund::StorageFund</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_new">new</a>(initial_fund: Balance&lt;AIY&gt;): <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_StorageFund">StorageFund</a> {
    <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_StorageFund">StorageFund</a> {
        // At the beginning there's no object in the storage yet
        <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>: balance::zero(),
        non_refundable_balance: initial_fund,
    }
}
</code></pre>



</details>

<a name="aiy_system_storage_fund_advance_epoch"></a>

## Function `advance_epoch`

Called by <code><a href="../aiy_system/aiy_system.md#aiy_system_aiy_system">aiy_system</a></code> at epoch change times to process the inflows and outflows of storage fund.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_StorageFund">aiy_system::storage_fund::StorageFund</a>, storage_charges: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;, storage_fund_reinvestment: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;, leftover_staking_rewards: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;, storage_rebate_amount: u64, non_refundable_storage_fee_amount: u64): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_StorageFund">StorageFund</a>,
    storage_charges: Balance&lt;AIY&gt;,
    storage_fund_reinvestment: Balance&lt;AIY&gt;,
    leftover_staking_rewards: Balance&lt;AIY&gt;,
    storage_rebate_amount: u64,
    non_refundable_storage_fee_amount: u64,
): Balance&lt;AIY&gt; {
    // Both the reinvestment and leftover rewards are not to be refunded so they go to the non-refundable balance.
    self.non_refundable_balance.join(storage_fund_reinvestment);
    self.non_refundable_balance.join(leftover_staking_rewards);
    // The storage charges <b>for</b> the epoch come from the storage rebate of the <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_new">new</a> objects created
    // and the <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_new">new</a> storage rebates of the objects modified during the epoch so we put the charges
    // into `<a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>`.
    self.<a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.join(storage_charges);
    // Split out the non-refundable portion of the storage rebate and put it into the non-refundable balance.
    <b>let</b> non_refundable_storage_fee = self
        .<a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>
        .split(non_refundable_storage_fee_amount);
    self.non_refundable_balance.join(non_refundable_storage_fee);
    // `storage_rebates` include the already refunded rebates of deleted objects and old rebates of modified objects and
    // should be taken out of the `<a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>`.
    <b>let</b> storage_rebate = self.<a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.split(storage_rebate_amount);
    // The storage rebate <b>has</b> already been returned to individual transaction senders' gas coins
    // so we <b>return</b> the balance to be burnt at the very end of epoch change.
    storage_rebate
}
</code></pre>



</details>

<a name="aiy_system_storage_fund_total_object_storage_rebates"></a>

## Function `total_object_storage_rebates`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>(self: &<a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_StorageFund">aiy_system::storage_fund::StorageFund</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>(self: &<a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_StorageFund">StorageFund</a>): u64 {
    self.<a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.value()
}
</code></pre>



</details>

<a name="aiy_system_storage_fund_total_balance"></a>

## Function `total_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_balance">total_balance</a>(self: &<a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_StorageFund">aiy_system::storage_fund::StorageFund</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_balance">total_balance</a>(self: &<a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_StorageFund">StorageFund</a>): u64 {
    self.<a href="../aiy_system/storage_fund.md#aiy_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.value() + self.non_refundable_balance.value()
}
</code></pre>



</details>
