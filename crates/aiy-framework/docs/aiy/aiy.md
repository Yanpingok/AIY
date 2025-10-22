---
title: Module `aiy::aiy`
---

Coin<AIY> is the token used to pay for gas in Aiy.
It has 9 decimals, and the smallest unit (10^-9) is called "mist".


-  [Struct `AIY`](#aiy_aiy_AIY)
-  [Constants](#@Constants_0)
-  [Function `new`](#aiy_aiy_new)
-  [Function `transfer`](#aiy_aiy_transfer)


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



<a name="aiy_aiy_AIY"></a>

## Struct `AIY`

Name of the coin


<pre><code><b>public</b> <b>struct</b> <a href="../aiy/aiy.md#aiy_aiy_AIY">AIY</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="aiy_aiy_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="../aiy/aiy.md#aiy_aiy_EAlreadyMinted">EAlreadyMinted</a>: u64 = 0;
</code></pre>



<a name="aiy_aiy_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../aiy/aiy.md#aiy_aiy_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="aiy_aiy_MIST_PER_AIY"></a>

The amount of Mist per Aiy token based on the fact that mist is
10^-9 of a Aiy token


<pre><code><b>const</b> <a href="../aiy/aiy.md#aiy_aiy_MIST_PER_AIY">MIST_PER_AIY</a>: u64 = 1000000000;
</code></pre>



<a name="aiy_aiy_TOTAL_SUPPLY_AIY"></a>

The total supply of Aiy denominated in whole Aiy tokens (10 Billion)


<pre><code><b>const</b> <a href="../aiy/aiy.md#aiy_aiy_TOTAL_SUPPLY_AIY">TOTAL_SUPPLY_AIY</a>: u64 = 10000000000;
</code></pre>



<a name="aiy_aiy_TOTAL_SUPPLY_MIST"></a>

The total supply of Aiy denominated in Mist (10 Billion * 10^9)


<pre><code><b>const</b> <a href="../aiy/aiy.md#aiy_aiy_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>: u64 = 10000000000000000000;
</code></pre>



<a name="aiy_aiy_new"></a>

## Function `new`

Register the <code><a href="../aiy/aiy.md#aiy_aiy_AIY">AIY</a></code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>fun</b> <a href="../aiy/aiy.md#aiy_aiy_new">new</a>(ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy/aiy.md#aiy_aiy_new">new</a>(ctx: &<b>mut</b> TxContext): Balance&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">AIY</a>&gt; {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../aiy/aiy.md#aiy_aiy_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(ctx.epoch() == 0, <a href="../aiy/aiy.md#aiy_aiy_EAlreadyMinted">EAlreadyMinted</a>);
    <b>let</b> (treasury, metadata) = <a href="../aiy/coin.md#aiy_coin_create_currency">coin::create_currency</a>(
        <a href="../aiy/aiy.md#aiy_aiy_AIY">AIY</a> {},
        9,
        b"<a href="../aiy/aiy.md#aiy_aiy_AIY">AIY</a>",
        b"Aiy",
        // TODO: add appropriate description and logo <a href="../aiy/url.md#aiy_url">url</a>
        b"",
        option::none(),
        ctx,
    );
    <a href="../aiy/transfer.md#aiy_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <b>let</b> <b>mut</b> supply = treasury.treasury_into_supply();
    <b>let</b> total_aiy = supply.increase_supply(<a href="../aiy/aiy.md#aiy_aiy_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>);
    supply.destroy_supply();
    total_aiy
}
</code></pre>



</details>

<a name="aiy_aiy_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/transfer.md#aiy_transfer">transfer</a>(c: <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">aiy::aiy::AIY</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/transfer.md#aiy_transfer">transfer</a>(c: <a href="../aiy/coin.md#aiy_coin_Coin">coin::Coin</a>&lt;<a href="../aiy/aiy.md#aiy_aiy_AIY">AIY</a>&gt;, recipient: <b>address</b>) {
    <a href="../aiy/transfer.md#aiy_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>
