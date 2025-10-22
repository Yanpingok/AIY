---
title: Module `aiy::pay`
---

This module provides handy functionality for wallets and <code>aiy::Coin</code> management.


-  [Constants](#@Constants_0)
-  [Function `keep`](#aiy_pay_keep)
-  [Function `split`](#aiy_pay_split)
-  [Function `split_vec`](#aiy_pay_split_vec)
-  [Function `split_and_transfer`](#aiy_pay_split_and_transfer)
-  [Function `divide_and_keep`](#aiy_pay_divide_and_keep)
-  [Function `join`](#aiy_pay_join)
-  [Function `join_vec`](#aiy_pay_join_vec)
-  [Function `join_vec_and_transfer`](#aiy_pay_join_vec_and_transfer)


<pre><code><b>use</b> <a href="../aiy/accumulator.md#aiy_accumulator">aiy::accumulator</a>;
<b>use</b> <a href="../aiy/accumulator_metadata.md#aiy_accumulator_metadata">aiy::accumulator_metadata</a>;
<b>use</b> <a href="../aiy/accumulator_settlement.md#aiy_accumulator_settlement">aiy::accumulator_settlement</a>;
<b>use</b> <a href="../aiy/address.md#aiy_address">aiy::address</a>;
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



<a name="@Constants_0"></a>

## Constants


<a name="aiy_pay_ENoCoins"></a>

For when empty vector is supplied into join function.


<pre><code><b>const</b> <a href="../aiy/pay.md#aiy_pay_ENoCoins">ENoCoins</a>: u64 = 0;
</code></pre>



<a name="aiy_pay_keep"></a>

## Function `keep`

Transfer <code>c</code> to the sender of the current transaction


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_keep">keep</a>&lt;T&gt;(c: <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;, ctx: &<a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_keep">keep</a>&lt;T&gt;(c: Coin&lt;T&gt;, ctx: &TxContext) {
    <a href="../aiy/transfer.md#aiy_transfer_public_transfer">transfer::public_transfer</a>(c, ctx.sender())
}
</code></pre>



</details>

<a name="aiy_pay_split"></a>

## Function `split`

Split <code><a href="../aiy/coin.md#aiy_coin">coin</a></code> to two coins, one with balance <code>split_amount</code>,
and the remaining balance is left in <code><a href="../aiy/coin.md#aiy_coin">coin</a></code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_split">split</a>&lt;T&gt;(<a href="../aiy/coin.md#aiy_coin">coin</a>: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_split">split</a>&lt;T&gt;(<a href="../aiy/coin.md#aiy_coin">coin</a>: &<b>mut</b> Coin&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> TxContext) {
    <a href="../aiy/pay.md#aiy_pay_keep">keep</a>(<a href="../aiy/coin.md#aiy_coin">coin</a>.<a href="../aiy/pay.md#aiy_pay_split">split</a>(split_amount, ctx), ctx)
}
</code></pre>



</details>

<a name="aiy_pay_split_vec"></a>

## Function `split_vec`

Split coin <code>self</code> into multiple coins, each with balance specified
in <code>split_amounts</code>. Remaining balance is left in <code>self</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_split_vec">split_vec</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;, split_amounts: vector&lt;u64&gt;, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_split_vec">split_vec</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, split_amounts: vector&lt;u64&gt;, ctx: &<b>mut</b> TxContext) {
    split_amounts.do!(|amount| <a href="../aiy/pay.md#aiy_pay_split">split</a>(self, amount, ctx));
}
</code></pre>



</details>

<a name="aiy_pay_split_and_transfer"></a>

## Function `split_and_transfer`

Send <code>amount</code> units of <code>c</code> to <code>recipient</code>
Aborts with <code><a href="../aiy/balance.md#aiy_balance_ENotEnough">aiy::balance::ENotEnough</a></code> if <code>amount</code> is greater than the balance in <code>c</code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_split_and_transfer">split_and_transfer</a>&lt;T&gt;(c: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;, amount: u64, recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_split_and_transfer">split_and_transfer</a>&lt;T&gt;(
    c: &<b>mut</b> Coin&lt;T&gt;,
    amount: u64,
    recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../aiy/transfer.md#aiy_transfer_public_transfer">transfer::public_transfer</a>(c.<a href="../aiy/pay.md#aiy_pay_split">split</a>(amount, ctx), recipient)
}
</code></pre>



</details>

<a name="aiy_pay_divide_and_keep"></a>

## Function `divide_and_keep`

Divide coin <code>self</code> into <code>n - 1</code> coins with equal balances. If the balance is
not evenly divisible by <code>n</code>, the remainder is left in <code>self</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_divide_and_keep">divide_and_keep</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;, n: u64, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_divide_and_keep">divide_and_keep</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, n: u64, ctx: &<b>mut</b> TxContext) {
    self.divide_into_n(n, ctx).destroy!(|<a href="../aiy/coin.md#aiy_coin">coin</a>| <a href="../aiy/transfer.md#aiy_transfer_public_transfer">transfer::public_transfer</a>(<a href="../aiy/coin.md#aiy_coin">coin</a>, ctx.sender()));
}
</code></pre>



</details>

<a name="aiy_pay_join"></a>

## Function `join`

Join <code><a href="../aiy/coin.md#aiy_coin">coin</a></code> into <code>self</code>. Re-exports <code><a href="../aiy/coin.md#aiy_coin_join">coin::join</a></code> function.
Deprecated: you should call <code><a href="../aiy/coin.md#aiy_coin">coin</a>.<a href="../aiy/pay.md#aiy_pay_join">join</a>(other)</code> directly.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin">coin</a>: <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_join">join</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin">coin</a>: Coin&lt;T&gt;) {
    self.<a href="../aiy/pay.md#aiy_pay_join">join</a>(<a href="../aiy/coin.md#aiy_coin">coin</a>)
}
</code></pre>



</details>

<a name="aiy_pay_join_vec"></a>

## Function `join_vec`

Join everything in <code>coins</code> with <code>self</code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_join_vec">join_vec</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;, coins: vector&lt;<a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_join_vec">join_vec</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, coins: vector&lt;Coin&lt;T&gt;&gt;) {
    coins.destroy!(|<a href="../aiy/coin.md#aiy_coin">coin</a>| self.<a href="../aiy/pay.md#aiy_pay_join">join</a>(<a href="../aiy/coin.md#aiy_coin">coin</a>));
}
</code></pre>



</details>

<a name="aiy_pay_join_vec_and_transfer"></a>

## Function `join_vec_and_transfer`

Join a vector of <code>Coin</code> into a single object and transfer it to <code>receiver</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_join_vec_and_transfer">join_vec_and_transfer</a>&lt;T&gt;(coins: vector&lt;<a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;&gt;, receiver: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/pay.md#aiy_pay_join_vec_and_transfer">join_vec_and_transfer</a>&lt;T&gt;(<b>mut</b> coins: vector&lt;Coin&lt;T&gt;&gt;, receiver: <b>address</b>) {
    <b>assert</b>!(coins.length() &gt; 0, <a href="../aiy/pay.md#aiy_pay_ENoCoins">ENoCoins</a>);
    <b>let</b> <b>mut</b> self = coins.pop_back();
    <a href="../aiy/pay.md#aiy_pay_join_vec">join_vec</a>(&<b>mut</b> self, coins);
    <a href="../aiy/transfer.md#aiy_transfer_public_transfer">transfer::public_transfer</a>(self, receiver)
}
</code></pre>



</details>
