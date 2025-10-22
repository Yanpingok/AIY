---
title: Module `aiy::balance`
---

A storable handler for Balances in general. Is used in the <code>Coin</code>
module to allow balance operations and can be used to implement
custom coins with <code><a href="../aiy/balance.md#aiy_balance_Supply">Supply</a></code> and <code><a href="../aiy/balance.md#aiy_balance_Balance">Balance</a></code>s.


-  [Struct `Supply`](#aiy_balance_Supply)
-  [Struct `Balance`](#aiy_balance_Balance)
-  [Constants](#@Constants_0)
-  [Function `value`](#aiy_balance_value)
-  [Function `supply_value`](#aiy_balance_supply_value)
-  [Function `create_supply`](#aiy_balance_create_supply)
-  [Function `increase_supply`](#aiy_balance_increase_supply)
-  [Function `decrease_supply`](#aiy_balance_decrease_supply)
-  [Function `zero`](#aiy_balance_zero)
-  [Function `join`](#aiy_balance_join)
-  [Function `split`](#aiy_balance_split)
-  [Function `withdraw_all`](#aiy_balance_withdraw_all)
-  [Function `destroy_zero`](#aiy_balance_destroy_zero)
-  [Function `send_funds`](#aiy_balance_send_funds)
-  [Function `redeem_funds`](#aiy_balance_redeem_funds)
-  [Function `withdraw_funds_from_object`](#aiy_balance_withdraw_funds_from_object)
-  [Function `create_supply_internal`](#aiy_balance_create_supply_internal)
-  [Function `create_staking_rewards`](#aiy_balance_create_staking_rewards)
-  [Function `destroy_storage_rebates`](#aiy_balance_destroy_storage_rebates)
-  [Function `destroy_supply`](#aiy_balance_destroy_supply)
-  [Function `send_to_account`](#aiy_balance_send_to_account)
-  [Function `withdraw_from_account`](#aiy_balance_withdraw_from_account)


<pre><code><b>use</b> <a href="../aiy/accumulator.md#aiy_accumulator">aiy::accumulator</a>;
<b>use</b> <a href="../aiy/address.md#aiy_address">aiy::address</a>;
<b>use</b> <a href="../aiy/dynamic_field.md#aiy_dynamic_field">aiy::dynamic_field</a>;
<b>use</b> <a href="../aiy/funds_accumulator.md#aiy_funds_accumulator">aiy::funds_accumulator</a>;
<b>use</b> <a href="../aiy/hex.md#aiy_hex">aiy::hex</a>;
<b>use</b> <a href="../aiy/object.md#aiy_object">aiy::object</a>;
<b>use</b> <a href="../aiy/party.md#aiy_party">aiy::party</a>;
<b>use</b> <a href="../aiy/transfer.md#aiy_transfer">aiy::transfer</a>;
<b>use</b> <a href="../aiy/tx_context.md#aiy_tx_context">aiy::tx_context</a>;
<b>use</b> <a href="../aiy/vec_map.md#aiy_vec_map">aiy::vec_map</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="aiy_balance_Supply"></a>

## Struct `Supply`

A Supply of T. Used for minting and burning.
Wrapped into a <code>TreasuryCap</code> in the <code>Coin</code> module.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy/balance.md#aiy_balance_Supply">Supply</a>&lt;<b>phantom</b> T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../aiy/balance.md#aiy_balance_value">value</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="aiy_balance_Balance"></a>

## Struct `Balance`

Storable balance - an inner struct of a Coin type.
Can be used to store coins which don't need the key ability.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;<b>phantom</b> T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../aiy/balance.md#aiy_balance_value">value</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="aiy_balance_ENonZero"></a>

For when trying to destroy a non-zero balance.


<pre><code><b>const</b> <a href="../aiy/balance.md#aiy_balance_ENonZero">ENonZero</a>: u64 = 0;
</code></pre>



<a name="aiy_balance_EOverflow"></a>

For when an overflow is happening on Supply operations.


<pre><code><b>const</b> <a href="../aiy/balance.md#aiy_balance_EOverflow">EOverflow</a>: u64 = 1;
</code></pre>



<a name="aiy_balance_ENotEnough"></a>

For when trying to withdraw more than there is.


<pre><code><b>const</b> <a href="../aiy/balance.md#aiy_balance_ENotEnough">ENotEnough</a>: u64 = 2;
</code></pre>



<a name="aiy_balance_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../aiy/balance.md#aiy_balance_ENotSystemAddress">ENotSystemAddress</a>: u64 = 3;
</code></pre>



<a name="aiy_balance_ENotAIY"></a>

System operation performed for a coin other than AIY


<pre><code><b>const</b> <a href="../aiy/balance.md#aiy_balance_ENotAIY">ENotAIY</a>: u64 = 4;
</code></pre>



<a name="aiy_balance_AIY_TYPE_NAME"></a>



<pre><code><b>const</b> <a href="../aiy/balance.md#aiy_balance_AIY_TYPE_NAME">AIY_TYPE_NAME</a>: vector&lt;u8&gt; = vector[48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 50, 58, 58, 97, 105, 121, 58, 58, 65, 73, 89];
</code></pre>



<a name="aiy_balance_value"></a>

## Function `value`

Get the amount stored in a <code><a href="../aiy/balance.md#aiy_balance_Balance">Balance</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_value">value</a>&lt;T&gt;(self: &<a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_value">value</a>&lt;T&gt;(self: &<a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;): u64 {
    self.<a href="../aiy/balance.md#aiy_balance_value">value</a>
}
</code></pre>



</details>

<a name="aiy_balance_supply_value"></a>

## Function `supply_value`

Get the <code><a href="../aiy/balance.md#aiy_balance_Supply">Supply</a></code> value.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_supply_value">supply_value</a>&lt;T&gt;(supply: &<a href="../aiy/balance.md#aiy_balance_Supply">aiy::balance::Supply</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_supply_value">supply_value</a>&lt;T&gt;(supply: &<a href="../aiy/balance.md#aiy_balance_Supply">Supply</a>&lt;T&gt;): u64 {
    supply.<a href="../aiy/balance.md#aiy_balance_value">value</a>
}
</code></pre>



</details>

<a name="aiy_balance_create_supply"></a>

## Function `create_supply`

Create a new supply for type T.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_create_supply">create_supply</a>&lt;T: drop&gt;(_: T): <a href="../aiy/balance.md#aiy_balance_Supply">aiy::balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_create_supply">create_supply</a>&lt;T: drop&gt;(_: T): <a href="../aiy/balance.md#aiy_balance_Supply">Supply</a>&lt;T&gt; {
    <a href="../aiy/balance.md#aiy_balance_Supply">Supply</a> { <a href="../aiy/balance.md#aiy_balance_value">value</a>: 0 }
}
</code></pre>



</details>

<a name="aiy_balance_increase_supply"></a>

## Function `increase_supply`

Increase supply by <code><a href="../aiy/balance.md#aiy_balance_value">value</a></code> and create a new <code><a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;</code> with this value.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_increase_supply">increase_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Supply">aiy::balance::Supply</a>&lt;T&gt;, <a href="../aiy/balance.md#aiy_balance_value">value</a>: u64): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_increase_supply">increase_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Supply">Supply</a>&lt;T&gt;, <a href="../aiy/balance.md#aiy_balance_value">value</a>: u64): <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt; {
    <b>assert</b>!(<a href="../aiy/balance.md#aiy_balance_value">value</a> &lt; (18446744073709551615u64 - self.<a href="../aiy/balance.md#aiy_balance_value">value</a>), <a href="../aiy/balance.md#aiy_balance_EOverflow">EOverflow</a>);
    self.<a href="../aiy/balance.md#aiy_balance_value">value</a> = self.<a href="../aiy/balance.md#aiy_balance_value">value</a> + <a href="../aiy/balance.md#aiy_balance_value">value</a>;
    <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a> { <a href="../aiy/balance.md#aiy_balance_value">value</a> }
}
</code></pre>



</details>

<a name="aiy_balance_decrease_supply"></a>

## Function `decrease_supply`

Burn a Balance<T> and decrease Supply<T>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_decrease_supply">decrease_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Supply">aiy::balance::Supply</a>&lt;T&gt;, <a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_decrease_supply">decrease_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Supply">Supply</a>&lt;T&gt;, <a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;): u64 {
    <b>let</b> <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a> { <a href="../aiy/balance.md#aiy_balance_value">value</a> } = <a href="../aiy/balance.md#aiy_balance">balance</a>;
    <b>assert</b>!(self.<a href="../aiy/balance.md#aiy_balance_value">value</a> &gt;= <a href="../aiy/balance.md#aiy_balance_value">value</a>, <a href="../aiy/balance.md#aiy_balance_EOverflow">EOverflow</a>);
    self.<a href="../aiy/balance.md#aiy_balance_value">value</a> = self.<a href="../aiy/balance.md#aiy_balance_value">value</a> - <a href="../aiy/balance.md#aiy_balance_value">value</a>;
    <a href="../aiy/balance.md#aiy_balance_value">value</a>
}
</code></pre>



</details>

<a name="aiy_balance_zero"></a>

## Function `zero`

Create a zero <code><a href="../aiy/balance.md#aiy_balance_Balance">Balance</a></code> for type <code>T</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_zero">zero</a>&lt;T&gt;(): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_zero">zero</a>&lt;T&gt;(): <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt; {
    <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a> { <a href="../aiy/balance.md#aiy_balance_value">value</a>: 0 }
}
</code></pre>



</details>

<a name="aiy_balance_join"></a>

## Function `join`

Join two balances together.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;, <a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;, <a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;): u64 {
    <b>let</b> <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a> { <a href="../aiy/balance.md#aiy_balance_value">value</a> } = <a href="../aiy/balance.md#aiy_balance">balance</a>;
    self.<a href="../aiy/balance.md#aiy_balance_value">value</a> = self.<a href="../aiy/balance.md#aiy_balance_value">value</a> + <a href="../aiy/balance.md#aiy_balance_value">value</a>;
    self.<a href="../aiy/balance.md#aiy_balance_value">value</a>
}
</code></pre>



</details>

<a name="aiy_balance_split"></a>

## Function `split`

Split a <code><a href="../aiy/balance.md#aiy_balance_Balance">Balance</a></code> and take a sub balance from it.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_split">split</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;, <a href="../aiy/balance.md#aiy_balance_value">value</a>: u64): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_split">split</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;, <a href="../aiy/balance.md#aiy_balance_value">value</a>: u64): <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt; {
    <b>assert</b>!(self.<a href="../aiy/balance.md#aiy_balance_value">value</a> &gt;= <a href="../aiy/balance.md#aiy_balance_value">value</a>, <a href="../aiy/balance.md#aiy_balance_ENotEnough">ENotEnough</a>);
    self.<a href="../aiy/balance.md#aiy_balance_value">value</a> = self.<a href="../aiy/balance.md#aiy_balance_value">value</a> - <a href="../aiy/balance.md#aiy_balance_value">value</a>;
    <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a> { <a href="../aiy/balance.md#aiy_balance_value">value</a> }
}
</code></pre>



</details>

<a name="aiy_balance_withdraw_all"></a>

## Function `withdraw_all`

Withdraw all balance. After this the remaining balance must be 0.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_withdraw_all">withdraw_all</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_withdraw_all">withdraw_all</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;): <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt; {
    <b>let</b> <a href="../aiy/balance.md#aiy_balance_value">value</a> = self.<a href="../aiy/balance.md#aiy_balance_value">value</a>;
    <a href="../aiy/balance.md#aiy_balance_split">split</a>(self, <a href="../aiy/balance.md#aiy_balance_value">value</a>)
}
</code></pre>



</details>

<a name="aiy_balance_destroy_zero"></a>

## Function `destroy_zero`

Destroy a zero <code><a href="../aiy/balance.md#aiy_balance_Balance">Balance</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_destroy_zero">destroy_zero</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance_destroy_zero">destroy_zero</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;) {
    <b>assert</b>!(<a href="../aiy/balance.md#aiy_balance">balance</a>.<a href="../aiy/balance.md#aiy_balance_value">value</a> == 0, <a href="../aiy/balance.md#aiy_balance_ENonZero">ENonZero</a>);
    <b>let</b> <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a> { <a href="../aiy/balance.md#aiy_balance_value">value</a>: _ } = <a href="../aiy/balance.md#aiy_balance">balance</a>;
}
</code></pre>



</details>

<a name="aiy_balance_send_funds"></a>

## Function `send_funds`

Send a <code><a href="../aiy/balance.md#aiy_balance_Balance">Balance</a></code> to an address's funds accumulator.


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/balance.md#aiy_balance_send_funds">send_funds</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/balance.md#aiy_balance_send_funds">send_funds</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;, recipient: <b>address</b>) {
    <a href="../aiy/funds_accumulator.md#aiy_funds_accumulator_add_impl">aiy::funds_accumulator::add_impl</a>(<a href="../aiy/balance.md#aiy_balance">balance</a>, recipient);
}
</code></pre>



</details>

<a name="aiy_balance_redeem_funds"></a>

## Function `redeem_funds`

Redeem a <code>Withdrawal&lt;<a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;&gt;</code> to get the underlying <code><a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;</code> from an address's funds
accumulator.


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/balance.md#aiy_balance_redeem_funds">redeem_funds</a>&lt;T&gt;(withdrawal: <a href="../aiy/funds_accumulator.md#aiy_funds_accumulator_Withdrawal">aiy::funds_accumulator::Withdrawal</a>&lt;<a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;&gt;): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/balance.md#aiy_balance_redeem_funds">redeem_funds</a>&lt;T&gt;(
    withdrawal: <a href="../aiy/funds_accumulator.md#aiy_funds_accumulator_Withdrawal">aiy::funds_accumulator::Withdrawal</a>&lt;<a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;&gt;,
): <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt; {
    withdrawal.redeem()
}
</code></pre>



</details>

<a name="aiy_balance_withdraw_funds_from_object"></a>

## Function `withdraw_funds_from_object`

Create a <code>Withdrawal&lt;<a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;&gt;</code> from an object to withdraw funds from it.


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/balance.md#aiy_balance_withdraw_funds_from_object">withdraw_funds_from_object</a>&lt;T&gt;(obj: &<b>mut</b> <a href="../aiy/object.md#aiy_object_UID">aiy::object::UID</a>, <a href="../aiy/balance.md#aiy_balance_value">value</a>: u64): <a href="../aiy/funds_accumulator.md#aiy_funds_accumulator_Withdrawal">aiy::funds_accumulator::Withdrawal</a>&lt;<a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/balance.md#aiy_balance_withdraw_funds_from_object">withdraw_funds_from_object</a>&lt;T&gt;(
    obj: &<b>mut</b> UID,
    <a href="../aiy/balance.md#aiy_balance_value">value</a>: u64,
): Withdrawal&lt;<a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;&gt; {
    <a href="../aiy/funds_accumulator.md#aiy_funds_accumulator_withdraw_from_object">aiy::funds_accumulator::withdraw_from_object</a>(obj, <a href="../aiy/balance.md#aiy_balance_value">value</a> <b>as</b> u256)
}
</code></pre>



</details>

<a name="aiy_balance_create_supply_internal"></a>

## Function `create_supply_internal`



<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/balance.md#aiy_balance_create_supply_internal">create_supply_internal</a>&lt;T&gt;(): <a href="../aiy/balance.md#aiy_balance_Supply">aiy::balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/balance.md#aiy_balance_create_supply_internal">create_supply_internal</a>&lt;T&gt;(): <a href="../aiy/balance.md#aiy_balance_Supply">Supply</a>&lt;T&gt; {
    <a href="../aiy/balance.md#aiy_balance_Supply">Supply</a> { <a href="../aiy/balance.md#aiy_balance_value">value</a>: 0 }
}
</code></pre>



</details>

<a name="aiy_balance_create_staking_rewards"></a>

## Function `create_staking_rewards`

CAUTION: this function creates a <code><a href="../aiy/balance.md#aiy_balance_Balance">Balance</a></code> without increasing the supply.
It should only be called by the epoch change system txn to create staking rewards,
and nowhere else.


<pre><code><b>fun</b> <a href="../aiy/balance.md#aiy_balance_create_staking_rewards">create_staking_rewards</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance_value">value</a>: u64, ctx: &<a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy/balance.md#aiy_balance_create_staking_rewards">create_staking_rewards</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance_value">value</a>: u64, ctx: &TxContext): <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt; {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../aiy/balance.md#aiy_balance_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(
        <a href="../std/type_name.md#std_type_name_with_defining_ids">std::type_name::with_defining_ids</a>&lt;T&gt;().into_string().into_bytes() == <a href="../aiy/balance.md#aiy_balance_AIY_TYPE_NAME">AIY_TYPE_NAME</a>,
        <a href="../aiy/balance.md#aiy_balance_ENotAIY">ENotAIY</a>,
    );
    <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a> { <a href="../aiy/balance.md#aiy_balance_value">value</a> }
}
</code></pre>



</details>

<a name="aiy_balance_destroy_storage_rebates"></a>

## Function `destroy_storage_rebates`

CAUTION: this function destroys a <code><a href="../aiy/balance.md#aiy_balance_Balance">Balance</a></code> without decreasing the supply.
It should only be called by the epoch change system txn to destroy storage rebates,
and nowhere else.


<pre><code><b>fun</b> <a href="../aiy/balance.md#aiy_balance_destroy_storage_rebates">destroy_storage_rebates</a>&lt;T&gt;(self: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;, ctx: &<a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy/balance.md#aiy_balance_destroy_storage_rebates">destroy_storage_rebates</a>&lt;T&gt;(self: <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;, ctx: &TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../aiy/balance.md#aiy_balance_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(
        <a href="../std/type_name.md#std_type_name_with_defining_ids">std::type_name::with_defining_ids</a>&lt;T&gt;().into_string().into_bytes() == <a href="../aiy/balance.md#aiy_balance_AIY_TYPE_NAME">AIY_TYPE_NAME</a>,
        <a href="../aiy/balance.md#aiy_balance_ENotAIY">ENotAIY</a>,
    );
    <b>let</b> <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a> { <a href="../aiy/balance.md#aiy_balance_value">value</a>: _ } = self;
}
</code></pre>



</details>

<a name="aiy_balance_destroy_supply"></a>

## Function `destroy_supply`

Destroy a <code><a href="../aiy/balance.md#aiy_balance_Supply">Supply</a></code> preventing any further minting and burning.


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/balance.md#aiy_balance_destroy_supply">destroy_supply</a>&lt;T&gt;(self: <a href="../aiy/balance.md#aiy_balance_Supply">aiy::balance::Supply</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/balance.md#aiy_balance_destroy_supply">destroy_supply</a>&lt;T&gt;(self: <a href="../aiy/balance.md#aiy_balance_Supply">Supply</a>&lt;T&gt;): u64 {
    <b>let</b> <a href="../aiy/balance.md#aiy_balance_Supply">Supply</a> { <a href="../aiy/balance.md#aiy_balance_value">value</a> } = self;
    <a href="../aiy/balance.md#aiy_balance_value">value</a>
}
</code></pre>



</details>

<a name="aiy_balance_send_to_account"></a>

## Function `send_to_account`



<pre><code><b>fun</b> <a href="../aiy/balance.md#aiy_balance_send_to_account">send_to_account</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy/balance.md#aiy_balance_send_to_account">send_to_account</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;, recipient: <b>address</b>) {
    <a href="../aiy/balance.md#aiy_balance">balance</a>.<a href="../aiy/balance.md#aiy_balance_send_funds">send_funds</a>(recipient)
}
</code></pre>



</details>

<a name="aiy_balance_withdraw_from_account"></a>

## Function `withdraw_from_account`



<pre><code><b>fun</b> <a href="../aiy/balance.md#aiy_balance_withdraw_from_account">withdraw_from_account</a>&lt;T&gt;(amount: u64, ctx: &<a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy/balance.md#aiy_balance_withdraw_from_account">withdraw_from_account</a>&lt;T&gt;(amount: u64, ctx: &TxContext): <a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt; {
    <b>let</b> owner = ctx.sender();
    <b>let</b> withdrawal = <a href="../aiy/funds_accumulator.md#aiy_funds_accumulator_create_withdrawal">aiy::funds_accumulator::create_withdrawal</a>&lt;<a href="../aiy/balance.md#aiy_balance_Balance">Balance</a>&lt;T&gt;&gt;(owner, amount <b>as</b> u256);
    withdrawal.redeem()
}
</code></pre>



</details>
