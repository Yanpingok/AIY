---
title: Module `aiy::coin`
---

Defines the <code><a href="../aiy/coin.md#aiy_coin_Coin">Coin</a></code> type - platform wide representation of fungible
tokens and coins. <code><a href="../aiy/coin.md#aiy_coin_Coin">Coin</a></code> can be described as a secure wrapper around
<code>Balance</code> type.


-  [Struct `Coin`](#aiy_coin_Coin)
-  [Struct `CoinMetadata`](#aiy_coin_CoinMetadata)
-  [Struct `RegulatedCoinMetadata`](#aiy_coin_RegulatedCoinMetadata)
-  [Struct `TreasuryCap`](#aiy_coin_TreasuryCap)
-  [Struct `DenyCapV2`](#aiy_coin_DenyCapV2)
-  [Struct `CurrencyCreated`](#aiy_coin_CurrencyCreated)
-  [Struct `DenyCap`](#aiy_coin_DenyCap)
-  [Constants](#@Constants_0)
-  [Function `total_supply`](#aiy_coin_total_supply)
-  [Function `treasury_into_supply`](#aiy_coin_treasury_into_supply)
-  [Function `supply_immut`](#aiy_coin_supply_immut)
-  [Function `supply_mut`](#aiy_coin_supply_mut)
-  [Function `value`](#aiy_coin_value)
-  [Function `balance`](#aiy_coin_balance)
-  [Function `balance_mut`](#aiy_coin_balance_mut)
-  [Function `from_balance`](#aiy_coin_from_balance)
-  [Function `into_balance`](#aiy_coin_into_balance)
-  [Function `take`](#aiy_coin_take)
-  [Function `put`](#aiy_coin_put)
-  [Function `join`](#aiy_coin_join)
-  [Function `split`](#aiy_coin_split)
-  [Function `divide_into_n`](#aiy_coin_divide_into_n)
-  [Function `zero`](#aiy_coin_zero)
-  [Function `destroy_zero`](#aiy_coin_destroy_zero)
-  [Function `create_currency`](#aiy_coin_create_currency)
-  [Function `create_regulated_currency_v2`](#aiy_coin_create_regulated_currency_v2)
-  [Function `migrate_regulated_currency_to_v2`](#aiy_coin_migrate_regulated_currency_to_v2)
-  [Function `mint`](#aiy_coin_mint)
-  [Function `mint_balance`](#aiy_coin_mint_balance)
-  [Function `burn`](#aiy_coin_burn)
-  [Function `deny_list_v2_add`](#aiy_coin_deny_list_v2_add)
-  [Function `deny_list_v2_remove`](#aiy_coin_deny_list_v2_remove)
-  [Function `deny_list_v2_contains_current_epoch`](#aiy_coin_deny_list_v2_contains_current_epoch)
-  [Function `deny_list_v2_contains_next_epoch`](#aiy_coin_deny_list_v2_contains_next_epoch)
-  [Function `deny_list_v2_enable_global_pause`](#aiy_coin_deny_list_v2_enable_global_pause)
-  [Function `deny_list_v2_disable_global_pause`](#aiy_coin_deny_list_v2_disable_global_pause)
-  [Function `deny_list_v2_is_global_pause_enabled_current_epoch`](#aiy_coin_deny_list_v2_is_global_pause_enabled_current_epoch)
-  [Function `deny_list_v2_is_global_pause_enabled_next_epoch`](#aiy_coin_deny_list_v2_is_global_pause_enabled_next_epoch)
-  [Function `mint_and_transfer`](#aiy_coin_mint_and_transfer)
-  [Function `update_name`](#aiy_coin_update_name)
-  [Function `update_symbol`](#aiy_coin_update_symbol)
-  [Function `update_description`](#aiy_coin_update_description)
-  [Function `update_icon_url`](#aiy_coin_update_icon_url)
-  [Function `get_decimals`](#aiy_coin_get_decimals)
-  [Function `get_name`](#aiy_coin_get_name)
-  [Function `get_symbol`](#aiy_coin_get_symbol)
-  [Function `get_description`](#aiy_coin_get_description)
-  [Function `get_icon_url`](#aiy_coin_get_icon_url)
-  [Function `destroy_metadata`](#aiy_coin_destroy_metadata)
-  [Function `deny_cap_id`](#aiy_coin_deny_cap_id)
-  [Function `new_deny_cap_v2`](#aiy_coin_new_deny_cap_v2)
-  [Function `new_treasury_cap`](#aiy_coin_new_treasury_cap)
-  [Function `allow_global_pause`](#aiy_coin_allow_global_pause)
-  [Function `supply`](#aiy_coin_supply)
-  [Function `create_regulated_currency`](#aiy_coin_create_regulated_currency)
-  [Function `deny_list_add`](#aiy_coin_deny_list_add)
-  [Function `deny_list_remove`](#aiy_coin_deny_list_remove)
-  [Function `deny_list_contains`](#aiy_coin_deny_list_contains)


<pre><code><b>use</b> <a href="../aiy/accumulator.md#aiy_accumulator">aiy::accumulator</a>;
<b>use</b> <a href="../aiy/accumulator_metadata.md#aiy_accumulator_metadata">aiy::accumulator_metadata</a>;
<b>use</b> <a href="../aiy/accumulator_settlement.md#aiy_accumulator_settlement">aiy::accumulator_settlement</a>;
<b>use</b> <a href="../aiy/address.md#aiy_address">aiy::address</a>;
<b>use</b> <a href="../aiy/bag.md#aiy_bag">aiy::bag</a>;
<b>use</b> <a href="../aiy/balance.md#aiy_balance">aiy::balance</a>;
<b>use</b> <a href="../aiy/bcs.md#aiy_bcs">aiy::bcs</a>;
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



<a name="aiy_coin_Coin"></a>

## Struct `Coin`

A coin of type <code>T</code> worth <code><a href="../aiy/coin.md#aiy_coin_value">value</a></code>. Transferable and storable


<pre><code><b>public</b> <b>struct</b> <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
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
<code><a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="aiy_coin_CoinMetadata"></a>

## Struct `CoinMetadata`

Each Coin type T created through <code><a href="../aiy/coin.md#aiy_coin_create_currency">create_currency</a></code> function will have a
unique instance of CoinMetadata<T> that stores the metadata for this coin type.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
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
<code>decimals: u8</code>
</dt>
<dd>
 Number of decimal places the coin uses.
 A coin with <code><a href="../aiy/coin.md#aiy_coin_value">value</a> </code> N and <code>decimals</code> D should be shown as N / 10^D
 E.g., a coin with <code><a href="../aiy/coin.md#aiy_coin_value">value</a></code> 7002 and decimals 3 should be displayed as 7.002
 This is metadata for display usage only.
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Name for the token
</dd>
<dt>
<code>symbol: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
 Symbol for the token
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Description of the token
</dd>
<dt>
<code>icon_url: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../aiy/url.md#aiy_url_Url">aiy::url::Url</a>&gt;</code>
</dt>
<dd>
 URL for the token logo
</dd>
</dl>


</details>

<a name="aiy_coin_RegulatedCoinMetadata"></a>

## Struct `RegulatedCoinMetadata`

Similar to CoinMetadata, but created only for regulated coins that use the DenyList.
This object is always immutable.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy/coin.md#aiy_coin_RegulatedCoinMetadata">RegulatedCoinMetadata</a>&lt;<b>phantom</b> T&gt; <b>has</b> key
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
<code>coin_metadata_object: <a href="../aiy/object.md#aiy_object_ID">aiy::object::ID</a></code>
</dt>
<dd>
 The ID of the coin's CoinMetadata object.
</dd>
<dt>
<code>deny_cap_object: <a href="../aiy/object.md#aiy_object_ID">aiy::object::ID</a></code>
</dt>
<dd>
 The ID of the coin's DenyCap object.
</dd>
</dl>


</details>

<a name="aiy_coin_TreasuryCap"></a>

## Struct `TreasuryCap`

Capability allowing the bearer to mint and burn
coins of type <code>T</code>. Transferable


<pre><code><b>public</b> <b>struct</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
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
<code><a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>: <a href="../aiy/balance.md#aiy_balance_Supply">aiy::balance::Supply</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="aiy_coin_DenyCapV2"></a>

## Struct `DenyCapV2`

Capability allowing the bearer to deny addresses from using the currency's coins--
immediately preventing those addresses from interacting with the coin as an input to a
transaction and at the start of the next preventing them from receiving the coin.
If <code><a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a></code> is true, the bearer can enable a global pause that behaves as if
all addresses were added to the deny list.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
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
<code><a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="aiy_coin_CurrencyCreated"></a>

## Struct `CurrencyCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../aiy/coin.md#aiy_coin_CurrencyCreated">CurrencyCreated</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>decimals: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="aiy_coin_DenyCap"></a>

## Struct `DenyCap`

Capability allowing the bearer to freeze addresses, preventing those addresses from
interacting with the coin as an input to a transaction.


<pre><code><b>public</b> <b>struct</b> <a href="../aiy/coin.md#aiy_coin_DenyCap">DenyCap</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../aiy/object.md#aiy_object_UID">aiy::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="aiy_coin_EBadWitness"></a>

A type passed to create_supply is not a one-time witness.


<pre><code><b>const</b> <a href="../aiy/coin.md#aiy_coin_EBadWitness">EBadWitness</a>: u64 = 0;
</code></pre>



<a name="aiy_coin_EInvalidArg"></a>

Invalid arguments are passed to a function.


<pre><code><b>const</b> <a href="../aiy/coin.md#aiy_coin_EInvalidArg">EInvalidArg</a>: u64 = 1;
</code></pre>



<a name="aiy_coin_ENotEnough"></a>

Trying to split a coin more times than its balance allows.


<pre><code><b>const</b> <a href="../aiy/coin.md#aiy_coin_ENotEnough">ENotEnough</a>: u64 = 2;
</code></pre>



<a name="aiy_coin_EGlobalPauseNotAllowed"></a>



<pre><code><b>const</b> <a href="../aiy/coin.md#aiy_coin_EGlobalPauseNotAllowed">EGlobalPauseNotAllowed</a>: u64 = 3;
</code></pre>



<a name="aiy_coin_DENY_LIST_COIN_INDEX"></a>

The index into the deny list vector for the <code><a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a></code> type.


<pre><code><b>const</b> <a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>: u64 = 0;
</code></pre>



<a name="aiy_coin_total_supply"></a>

## Function `total_supply`

Return the total number of <code>T</code>'s in circulation.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>&lt;T&gt;(cap: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>&lt;T&gt;(cap: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;): u64 {
    <a href="../aiy/balance.md#aiy_balance_supply_value">balance::supply_value</a>(&cap.<a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>)
}
</code></pre>



</details>

<a name="aiy_coin_treasury_into_supply"></a>

## Function `treasury_into_supply`

Unwrap <code><a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a></code> getting the <code>Supply</code>.

Operation is irreversible. Supply cannot be converted into a <code><a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a></code> due
to different security guarantees (TreasuryCap can be created only once for a type)


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_treasury_into_supply">treasury_into_supply</a>&lt;T&gt;(treasury: <a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;): <a href="../aiy/balance.md#aiy_balance_Supply">aiy::balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_treasury_into_supply">treasury_into_supply</a>&lt;T&gt;(treasury: <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;): Supply&lt;T&gt; {
    <b>let</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a> { id, <a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a> } = treasury;
    id.delete();
    <a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>
}
</code></pre>



</details>

<a name="aiy_coin_supply_immut"></a>

## Function `supply_immut`

Get immutable reference to the treasury's <code>Supply</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_supply_immut">supply_immut</a>&lt;T&gt;(treasury: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;): &<a href="../aiy/balance.md#aiy_balance_Supply">aiy::balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_supply_immut">supply_immut</a>&lt;T&gt;(treasury: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;): &Supply&lt;T&gt; {
    &treasury.<a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>
}
</code></pre>



</details>

<a name="aiy_coin_supply_mut"></a>

## Function `supply_mut`

Get mutable reference to the treasury's <code>Supply</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_supply_mut">supply_mut</a>&lt;T&gt;(treasury: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;): &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Supply">aiy::balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_supply_mut">supply_mut</a>&lt;T&gt;(treasury: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;): &<b>mut</b> Supply&lt;T&gt; {
    &<b>mut</b> treasury.<a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>
}
</code></pre>



</details>

<a name="aiy_coin_value"></a>

## Function `value`

Public getter for the coin's value


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_value">value</a>&lt;T&gt;(self: &<a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_value">value</a>&lt;T&gt;(self: &<a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;): u64 {
    self.<a href="../aiy/balance.md#aiy_balance">balance</a>.<a href="../aiy/coin.md#aiy_coin_value">value</a>()
}
</code></pre>



</details>

<a name="aiy_coin_balance"></a>

## Function `balance`

Get immutable reference to the balance of a coin.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance">balance</a>&lt;T&gt;(<a href="../aiy/coin.md#aiy_coin">coin</a>: &<a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;): &<a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/balance.md#aiy_balance">balance</a>&lt;T&gt;(<a href="../aiy/coin.md#aiy_coin">coin</a>: &<a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;): &Balance&lt;T&gt; {
    &<a href="../aiy/coin.md#aiy_coin">coin</a>.<a href="../aiy/balance.md#aiy_balance">balance</a>
}
</code></pre>



</details>

<a name="aiy_coin_balance_mut"></a>

## Function `balance_mut`

Get a mutable reference to the balance of a coin.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_balance_mut">balance_mut</a>&lt;T&gt;(<a href="../aiy/coin.md#aiy_coin">coin</a>: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;): &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_balance_mut">balance_mut</a>&lt;T&gt;(<a href="../aiy/coin.md#aiy_coin">coin</a>: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;): &<b>mut</b> Balance&lt;T&gt; {
    &<b>mut</b> <a href="../aiy/coin.md#aiy_coin">coin</a>.<a href="../aiy/balance.md#aiy_balance">balance</a>
}
</code></pre>



</details>

<a name="aiy_coin_from_balance"></a>

## Function `from_balance`

Wrap a balance into a Coin to make it transferable.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_from_balance">from_balance</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_from_balance">from_balance</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: Balance&lt;T&gt;, ctx: &<b>mut</b> TxContext): <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt; {
    <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a> { id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx), <a href="../aiy/balance.md#aiy_balance">balance</a> }
}
</code></pre>



</details>

<a name="aiy_coin_into_balance"></a>

## Function `into_balance`

Destruct a Coin wrapper and keep the balance.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_into_balance">into_balance</a>&lt;T&gt;(<a href="../aiy/coin.md#aiy_coin">coin</a>: <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_into_balance">into_balance</a>&lt;T&gt;(<a href="../aiy/coin.md#aiy_coin">coin</a>: <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;): Balance&lt;T&gt; {
    <b>let</b> <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a> { id, <a href="../aiy/balance.md#aiy_balance">balance</a> } = <a href="../aiy/coin.md#aiy_coin">coin</a>;
    id.delete();
    <a href="../aiy/balance.md#aiy_balance">balance</a>
}
</code></pre>



</details>

<a name="aiy_coin_take"></a>

## Function `take`

Take a <code><a href="../aiy/coin.md#aiy_coin_Coin">Coin</a></code> worth of <code><a href="../aiy/coin.md#aiy_coin_value">value</a></code> from <code>Balance</code>.
Aborts if <code><a href="../aiy/coin.md#aiy_coin_value">value</a> &gt; <a href="../aiy/balance.md#aiy_balance">balance</a>.<a href="../aiy/coin.md#aiy_coin_value">value</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_take">take</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_value">value</a>: u64, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_take">take</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: &<b>mut</b> Balance&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_value">value</a>: u64, ctx: &<b>mut</b> TxContext): <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt; {
    <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a> {
        id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx),
        <a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance">balance</a>.<a href="../aiy/coin.md#aiy_coin_split">split</a>(<a href="../aiy/coin.md#aiy_coin_value">value</a>),
    }
}
</code></pre>



</details>

<a name="aiy_coin_put"></a>

## Function `put`

Put a <code><a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;</code> to the <code>Balance&lt;T&gt;</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_put">put</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: &<b>mut</b> <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin">coin</a>: <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_put">put</a>&lt;T&gt;(<a href="../aiy/balance.md#aiy_balance">balance</a>: &<b>mut</b> Balance&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin">coin</a>: <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;) {
    <a href="../aiy/balance.md#aiy_balance">balance</a>.<a href="../aiy/coin.md#aiy_coin_join">join</a>(<a href="../aiy/coin.md#aiy_coin_into_balance">into_balance</a>(<a href="../aiy/coin.md#aiy_coin">coin</a>));
}
</code></pre>



</details>

<a name="aiy_coin_join"></a>

## Function `join`

Consume the coin <code>c</code> and add its value to <code>self</code>.
Aborts if <code>c.<a href="../aiy/coin.md#aiy_coin_value">value</a> + self.<a href="../aiy/coin.md#aiy_coin_value">value</a> &gt; U64_MAX</code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;, c: <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;, c: <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;) {
    <b>let</b> <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a> { id, <a href="../aiy/balance.md#aiy_balance">balance</a> } = c;
    id.delete();
    self.<a href="../aiy/balance.md#aiy_balance">balance</a>.<a href="../aiy/coin.md#aiy_coin_join">join</a>(<a href="../aiy/balance.md#aiy_balance">balance</a>);
}
</code></pre>



</details>

<a name="aiy_coin_split"></a>

## Function `split`

Split coin <code>self</code> to two coins, one with balance <code>split_amount</code>,
and the remaining balance is left is <code>self</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_split">split</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_split">split</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> TxContext): <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt; {
    <a href="../aiy/coin.md#aiy_coin_take">take</a>(&<b>mut</b> self.<a href="../aiy/balance.md#aiy_balance">balance</a>, split_amount, ctx)
}
</code></pre>



</details>

<a name="aiy_coin_divide_into_n"></a>

## Function `divide_into_n`

Split coin <code>self</code> into <code>n - 1</code> coins with equal balances. The remainder is left in
<code>self</code>. Return newly created coins.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_divide_into_n">divide_into_n</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;, n: u64, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): vector&lt;<a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_divide_into_n">divide_into_n</a>&lt;T&gt;(self: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;, n: u64, ctx: &<b>mut</b> TxContext): vector&lt;<a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;&gt; {
    <b>assert</b>!(n &gt; 0, <a href="../aiy/coin.md#aiy_coin_EInvalidArg">EInvalidArg</a>);
    <b>assert</b>!(n &lt;= self.<a href="../aiy/coin.md#aiy_coin_value">value</a>(), <a href="../aiy/coin.md#aiy_coin_ENotEnough">ENotEnough</a>);
    <b>let</b> split_amount = self.<a href="../aiy/coin.md#aiy_coin_value">value</a>() / n;
    vector::tabulate!(n - 1, |_| self.<a href="../aiy/coin.md#aiy_coin_split">split</a>(split_amount, ctx))
}
</code></pre>



</details>

<a name="aiy_coin_zero"></a>

## Function `zero`

Make any Coin with a zero value. Useful for placeholding
bids/payments or preemptively making empty balances.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_zero">zero</a>&lt;T&gt;(ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_zero">zero</a>&lt;T&gt;(ctx: &<b>mut</b> TxContext): <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt; {
    <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a> { id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx), <a href="../aiy/balance.md#aiy_balance">balance</a>: <a href="../aiy/balance.md#aiy_balance_zero">balance::zero</a>() }
}
</code></pre>



</details>

<a name="aiy_coin_destroy_zero"></a>

## Function `destroy_zero`

Destroy a coin with value zero


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_destroy_zero">destroy_zero</a>&lt;T&gt;(c: <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_destroy_zero">destroy_zero</a>&lt;T&gt;(c: <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;) {
    <b>let</b> <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a> { id, <a href="../aiy/balance.md#aiy_balance">balance</a> } = c;
    id.delete();
    <a href="../aiy/balance.md#aiy_balance">balance</a>.<a href="../aiy/coin.md#aiy_coin_destroy_zero">destroy_zero</a>()
}
</code></pre>



</details>

<a name="aiy_coin_create_currency"></a>

## Function `create_currency`

Create a new currency type <code>T</code> as and return the <code><a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a></code> for
<code>T</code> to the caller. Can only be called with a <code>one-time-witness</code>
type, ensuring that there's only one <code><a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a></code> per <code>T</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_create_currency">create_currency</a>&lt;T: drop&gt;(witness: T, decimals: u8, symbol: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, icon_url: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../aiy/url.md#aiy_url_Url">aiy::url::Url</a>&gt;, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): (<a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_create_currency">create_currency</a>&lt;T: drop&gt;(
    witness: T,
    decimals: u8,
    symbol: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    icon_url: Option&lt;Url&gt;,
    ctx: &<b>mut</b> TxContext,
): (<a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;) {
    // Make sure there's only one instance of the type T
    <b>assert</b>!(<a href="../aiy/types.md#aiy_types_is_one_time_witness">aiy::types::is_one_time_witness</a>(&witness), <a href="../aiy/coin.md#aiy_coin_EBadWitness">EBadWitness</a>);
    (
        <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a> {
            id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx),
            <a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>: <a href="../aiy/balance.md#aiy_balance_create_supply">balance::create_supply</a>(witness),
        },
        <a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a> {
            id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx),
            decimals,
            name: name.to_string(),
            symbol: symbol.to_ascii_string(),
            description: description.to_string(),
            icon_url,
        },
    )
}
</code></pre>



</details>

<a name="aiy_coin_create_regulated_currency_v2"></a>

## Function `create_regulated_currency_v2`

This creates a new currency, via <code><a href="../aiy/coin.md#aiy_coin_create_currency">create_currency</a></code>, but with an extra capability that
allows for specific addresses to have their coins frozen. When an address is added to the
deny list, it is immediately unable to interact with the currency's coin as input objects.
Additionally at the start of the next epoch, they will be unable to receive the currency's
coin.
The <code><a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a></code> flag enables an additional API that will cause all addresses to
be denied. Note however, that this doesn't affect per-address entries of the deny list and
will not change the result of the "contains" APIs.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_create_regulated_currency_v2">create_regulated_currency_v2</a>&lt;T: drop&gt;(witness: T, decimals: u8, symbol: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, icon_url: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../aiy/url.md#aiy_url_Url">aiy::url::Url</a>&gt;, <a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>: bool, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): (<a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_DenyCapV2">aiy::coin::DenyCapV2</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_create_regulated_currency_v2">create_regulated_currency_v2</a>&lt;T: drop&gt;(
    witness: T,
    decimals: u8,
    symbol: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    icon_url: Option&lt;Url&gt;,
    <a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>: bool,
    ctx: &<b>mut</b> TxContext,
): (<a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;) {
    <b>let</b> (treasury_cap, metadata) = <a href="../aiy/coin.md#aiy_coin_create_currency">create_currency</a>(
        witness,
        decimals,
        symbol,
        name,
        description,
        icon_url,
        ctx,
    );
    <b>let</b> deny_cap = <a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a> {
        id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx),
        <a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>,
    };
    <a href="../aiy/transfer.md#aiy_transfer_freeze_object">transfer::freeze_object</a>(<a href="../aiy/coin.md#aiy_coin_RegulatedCoinMetadata">RegulatedCoinMetadata</a>&lt;T&gt; {
        id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx),
        coin_metadata_object: <a href="../aiy/object.md#aiy_object_id">object::id</a>(&metadata),
        deny_cap_object: <a href="../aiy/object.md#aiy_object_id">object::id</a>(&deny_cap),
    });
    (treasury_cap, deny_cap, metadata)
}
</code></pre>



</details>

<a name="aiy_coin_migrate_regulated_currency_to_v2"></a>

## Function `migrate_regulated_currency_to_v2`

Given the <code><a href="../aiy/coin.md#aiy_coin_DenyCap">DenyCap</a></code> for a regulated currency, migrate it to the new <code><a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a></code> type.
All entries in the deny list will be migrated to the new format.
See <code><a href="../aiy/coin.md#aiy_coin_create_regulated_currency_v2">create_regulated_currency_v2</a></code> for details on the new v2 of the deny list.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_migrate_regulated_currency_to_v2">migrate_regulated_currency_to_v2</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> <a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>, cap: <a href="../aiy/coin.md#aiy_coin_DenyCap">aiy::coin::DenyCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>: bool, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/coin.md#aiy_coin_DenyCapV2">aiy::coin::DenyCapV2</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_migrate_regulated_currency_to_v2">migrate_regulated_currency_to_v2</a>&lt;T&gt;(
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    cap: <a href="../aiy/coin.md#aiy_coin_DenyCap">DenyCap</a>&lt;T&gt;,
    <a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>: bool,
    ctx: &<b>mut</b> TxContext,
): <a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt; {
    <b>let</b> <a href="../aiy/coin.md#aiy_coin_DenyCap">DenyCap</a> { id } = cap;
    id.delete();
    <b>let</b> ty = type_name::with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.migrate_v1_to_v2(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, ctx);
    <a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a> {
        id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx),
        <a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>,
    }
}
</code></pre>



</details>

<a name="aiy_coin_mint"></a>

## Function `mint`

Create a coin worth <code><a href="../aiy/coin.md#aiy_coin_value">value</a></code> and increase the total supply
in <code>cap</code> accordingly.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_mint">mint</a>&lt;T&gt;(cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_value">value</a>: u64, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_mint">mint</a>&lt;T&gt;(cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_value">value</a>: u64, ctx: &<b>mut</b> TxContext): <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt; {
    <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a> {
        id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx),
        <a href="../aiy/balance.md#aiy_balance">balance</a>: cap.<a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>.increase_supply(<a href="../aiy/coin.md#aiy_coin_value">value</a>),
    }
}
</code></pre>



</details>

<a name="aiy_coin_mint_balance"></a>

## Function `mint_balance`

Mint some amount of T as a <code>Balance</code> and increase the total
supply in <code>cap</code> accordingly.
Aborts if <code><a href="../aiy/coin.md#aiy_coin_value">value</a></code> + <code>cap.<a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a></code> >= U64_MAX


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_mint_balance">mint_balance</a>&lt;T&gt;(cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_value">value</a>: u64): <a href="../aiy/balance.md#aiy_balance_Balance">aiy::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_mint_balance">mint_balance</a>&lt;T&gt;(cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_value">value</a>: u64): Balance&lt;T&gt; {
    cap.<a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>.increase_supply(<a href="../aiy/coin.md#aiy_coin_value">value</a>)
}
</code></pre>



</details>

<a name="aiy_coin_burn"></a>

## Function `burn`

Destroy the coin <code>c</code> and decrease the total supply in <code>cap</code>
accordingly.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_burn">burn</a>&lt;T&gt;(cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;, c: <a href="../aiy/coin.md#aiy_coin_Coin">aiy::coin::Coin</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_burn">burn</a>&lt;T&gt;(cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, c: <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a>&lt;T&gt;): u64 {
    <b>let</b> <a href="../aiy/coin.md#aiy_coin_Coin">Coin</a> { id, <a href="../aiy/balance.md#aiy_balance">balance</a> } = c;
    id.delete();
    cap.<a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>.decrease_supply(<a href="../aiy/balance.md#aiy_balance">balance</a>)
}
</code></pre>



</details>

<a name="aiy_coin_deny_list_v2_add"></a>

## Function `deny_list_v2_add`

Adds the given address to the deny list, preventing it from interacting with the specified
coin type as an input to a transaction. Additionally at the start of the next epoch, the
address will be unable to receive objects of this coin type.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_add">deny_list_v2_add</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> <a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>, _deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCapV2">aiy::coin::DenyCapV2</a>&lt;T&gt;, addr: <b>address</b>, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_add">deny_list_v2_add</a>&lt;T&gt;(
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    _deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt;,
    addr: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> ty = type_name::with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.v2_add(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, addr, ctx)
}
</code></pre>



</details>

<a name="aiy_coin_deny_list_v2_remove"></a>

## Function `deny_list_v2_remove`

Removes an address from the deny list. Similar to <code><a href="../aiy/coin.md#aiy_coin_deny_list_v2_add">deny_list_v2_add</a></code>, the effect for input
objects will be immediate, but the effect for receiving objects will be delayed until the
next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_remove">deny_list_v2_remove</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> <a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>, _deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCapV2">aiy::coin::DenyCapV2</a>&lt;T&gt;, addr: <b>address</b>, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_remove">deny_list_v2_remove</a>&lt;T&gt;(
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    _deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt;,
    addr: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> ty = type_name::with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.v2_remove(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, addr, ctx)
}
</code></pre>



</details>

<a name="aiy_coin_deny_list_v2_contains_current_epoch"></a>

## Function `deny_list_v2_contains_current_epoch`

Check if the deny list contains the given address for the current epoch. Denied addresses
in the current epoch will be unable to receive objects of this coin type.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_contains_current_epoch">deny_list_v2_contains_current_epoch</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>, addr: <b>address</b>, ctx: &<a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_contains_current_epoch">deny_list_v2_contains_current_epoch</a>&lt;T&gt;(
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &DenyList,
    addr: <b>address</b>,
    ctx: &TxContext,
): bool {
    <b>let</b> ty = type_name::with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.v2_contains_current_epoch(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, addr, ctx)
}
</code></pre>



</details>

<a name="aiy_coin_deny_list_v2_contains_next_epoch"></a>

## Function `deny_list_v2_contains_next_epoch`

Check if the deny list contains the given address for the next epoch. Denied addresses in
the next epoch will immediately be unable to use objects of this coin type as inputs. At the
start of the next epoch, the address will be unable to receive objects of this coin type.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_contains_next_epoch">deny_list_v2_contains_next_epoch</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_contains_next_epoch">deny_list_v2_contains_next_epoch</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &DenyList, addr: <b>address</b>): bool {
    <b>let</b> ty = type_name::with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.v2_contains_next_epoch(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, addr)
}
</code></pre>



</details>

<a name="aiy_coin_deny_list_v2_enable_global_pause"></a>

## Function `deny_list_v2_enable_global_pause`

Enable the global pause for the given coin type. This will immediately prevent all addresses
from using objects of this coin type as inputs. At the start of the next epoch, all
addresses will be unable to receive objects of this coin type.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_enable_global_pause">deny_list_v2_enable_global_pause</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> <a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>, deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCapV2">aiy::coin::DenyCapV2</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_enable_global_pause">deny_list_v2_enable_global_pause</a>&lt;T&gt;(
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(deny_cap.<a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>, <a href="../aiy/coin.md#aiy_coin_EGlobalPauseNotAllowed">EGlobalPauseNotAllowed</a>);
    <b>let</b> ty = type_name::with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.v2_enable_global_pause(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, ctx)
}
</code></pre>



</details>

<a name="aiy_coin_deny_list_v2_disable_global_pause"></a>

## Function `deny_list_v2_disable_global_pause`

Disable the global pause for the given coin type. This will immediately allow all addresses
to resume using objects of this coin type as inputs. However, receiving objects of this coin
type will still be paused until the start of the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_disable_global_pause">deny_list_v2_disable_global_pause</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> <a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>, deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCapV2">aiy::coin::DenyCapV2</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_disable_global_pause">deny_list_v2_disable_global_pause</a>&lt;T&gt;(
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(deny_cap.<a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>, <a href="../aiy/coin.md#aiy_coin_EGlobalPauseNotAllowed">EGlobalPauseNotAllowed</a>);
    <b>let</b> ty = type_name::with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.v2_disable_global_pause(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, ctx)
}
</code></pre>



</details>

<a name="aiy_coin_deny_list_v2_is_global_pause_enabled_current_epoch"></a>

## Function `deny_list_v2_is_global_pause_enabled_current_epoch`

Check if the global pause is enabled for the given coin type in the current epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_is_global_pause_enabled_current_epoch">deny_list_v2_is_global_pause_enabled_current_epoch</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>, ctx: &<a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_is_global_pause_enabled_current_epoch">deny_list_v2_is_global_pause_enabled_current_epoch</a>&lt;T&gt;(
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &DenyList,
    ctx: &TxContext,
): bool {
    <b>let</b> ty = type_name::with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.v2_is_global_pause_enabled_current_epoch(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, ctx)
}
</code></pre>



</details>

<a name="aiy_coin_deny_list_v2_is_global_pause_enabled_next_epoch"></a>

## Function `deny_list_v2_is_global_pause_enabled_next_epoch`

Check if the global pause is enabled for the given coin type in the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_is_global_pause_enabled_next_epoch">deny_list_v2_is_global_pause_enabled_next_epoch</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_v2_is_global_pause_enabled_next_epoch">deny_list_v2_is_global_pause_enabled_next_epoch</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &DenyList): bool {
    <b>let</b> ty = type_name::with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.v2_is_global_pause_enabled_next_epoch(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty)
}
</code></pre>



</details>

<a name="aiy_coin_mint_and_transfer"></a>

## Function `mint_and_transfer`

Mint <code>amount</code> of <code><a href="../aiy/coin.md#aiy_coin_Coin">Coin</a></code> and send it to <code>recipient</code>. Invokes <code><a href="../aiy/coin.md#aiy_coin_mint">mint</a>()</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_mint_and_transfer">mint_and_transfer</a>&lt;T&gt;(c: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;, amount: u64, recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_mint_and_transfer">mint_and_transfer</a>&lt;T&gt;(
    c: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;,
    amount: u64,
    recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../aiy/transfer.md#aiy_transfer_public_transfer">transfer::public_transfer</a>(c.<a href="../aiy/coin.md#aiy_coin_mint">mint</a>(amount, ctx), recipient)
}
</code></pre>



</details>

<a name="aiy_coin_update_name"></a>

## Function `update_name`

Update name of the coin in <code><a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a></code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_update_name">update_name</a>&lt;T&gt;(_treasury: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;, name: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_update_name">update_name</a>&lt;T&gt;(
    _treasury: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;,
    metadata: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;,
    name: string::String,
) {
    metadata.name = name;
}
</code></pre>



</details>

<a name="aiy_coin_update_symbol"></a>

## Function `update_symbol`

Update the symbol of the coin in <code><a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a></code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_update_symbol">update_symbol</a>&lt;T&gt;(_treasury: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;, symbol: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_update_symbol">update_symbol</a>&lt;T&gt;(
    _treasury: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;,
    metadata: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;,
    symbol: ascii::String,
) {
    metadata.symbol = symbol;
}
</code></pre>



</details>

<a name="aiy_coin_update_description"></a>

## Function `update_description`

Update the description of the coin in <code><a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a></code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_update_description">update_description</a>&lt;T&gt;(_treasury: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;, description: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_update_description">update_description</a>&lt;T&gt;(
    _treasury: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;,
    metadata: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;,
    description: string::String,
) {
    metadata.description = description;
}
</code></pre>



</details>

<a name="aiy_coin_update_icon_url"></a>

## Function `update_icon_url`

Update the url of the coin in <code><a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a></code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_update_icon_url">update_icon_url</a>&lt;T&gt;(_treasury: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;, <a href="../aiy/url.md#aiy_url">url</a>: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_update_icon_url">update_icon_url</a>&lt;T&gt;(
    _treasury: &<a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;,
    metadata: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;,
    <a href="../aiy/url.md#aiy_url">url</a>: ascii::String,
) {
    metadata.icon_url = option::some(<a href="../aiy/url.md#aiy_url_new_unsafe">url::new_unsafe</a>(<a href="../aiy/url.md#aiy_url">url</a>));
}
</code></pre>



</details>

<a name="aiy_coin_get_decimals"></a>

## Function `get_decimals`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_get_decimals">get_decimals</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_get_decimals">get_decimals</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;): u8 {
    metadata.decimals
}
</code></pre>



</details>

<a name="aiy_coin_get_name"></a>

## Function `get_name`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_get_name">get_name</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_get_name">get_name</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;): string::String {
    metadata.name
}
</code></pre>



</details>

<a name="aiy_coin_get_symbol"></a>

## Function `get_symbol`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_get_symbol">get_symbol</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;): <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_get_symbol">get_symbol</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;): ascii::String {
    metadata.symbol
}
</code></pre>



</details>

<a name="aiy_coin_get_description"></a>

## Function `get_description`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_get_description">get_description</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_get_description">get_description</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;): string::String {
    metadata.description
}
</code></pre>



</details>

<a name="aiy_coin_get_icon_url"></a>

## Function `get_icon_url`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_get_icon_url">get_icon_url</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../aiy/url.md#aiy_url_Url">aiy::url::Url</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_get_icon_url">get_icon_url</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;): Option&lt;Url&gt; {
    metadata.icon_url
}
</code></pre>



</details>

<a name="aiy_coin_destroy_metadata"></a>

## Function `destroy_metadata`

Destroy legacy <code><a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a></code> object


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/coin.md#aiy_coin_destroy_metadata">destroy_metadata</a>&lt;T&gt;(metadata: <a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/coin.md#aiy_coin_destroy_metadata">destroy_metadata</a>&lt;T&gt;(metadata: <a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;) {
    <b>let</b> <a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a> { id, .. } = metadata;
    id.delete()
}
</code></pre>



</details>

<a name="aiy_coin_deny_cap_id"></a>

## Function `deny_cap_id`



<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_cap_id">deny_cap_id</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_RegulatedCoinMetadata">aiy::coin::RegulatedCoinMetadata</a>&lt;T&gt;): <a href="../aiy/object.md#aiy_object_ID">aiy::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_cap_id">deny_cap_id</a>&lt;T&gt;(metadata: &<a href="../aiy/coin.md#aiy_coin_RegulatedCoinMetadata">RegulatedCoinMetadata</a>&lt;T&gt;): ID {
    metadata.deny_cap_object
}
</code></pre>



</details>

<a name="aiy_coin_new_deny_cap_v2"></a>

## Function `new_deny_cap_v2`



<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/coin.md#aiy_coin_new_deny_cap_v2">new_deny_cap_v2</a>&lt;T&gt;(<a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>: bool, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/coin.md#aiy_coin_DenyCapV2">aiy::coin::DenyCapV2</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/coin.md#aiy_coin_new_deny_cap_v2">new_deny_cap_v2</a>&lt;T&gt;(
    <a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>: bool,
    ctx: &<b>mut</b> TxContext,
): <a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt; {
    <a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a> {
        id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx),
        <a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>,
    }
}
</code></pre>



</details>

<a name="aiy_coin_new_treasury_cap"></a>

## Function `new_treasury_cap`



<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/coin.md#aiy_coin_new_treasury_cap">new_treasury_cap</a>&lt;T&gt;(ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/coin.md#aiy_coin_new_treasury_cap">new_treasury_cap</a>&lt;T&gt;(ctx: &<b>mut</b> TxContext): <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt; {
    <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a> {
        id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx),
        <a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>: <a href="../aiy/balance.md#aiy_balance_create_supply_internal">balance::create_supply_internal</a>(),
    }
}
</code></pre>



</details>

<a name="aiy_coin_allow_global_pause"></a>

## Function `allow_global_pause`



<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>&lt;T&gt;(cap: &<a href="../aiy/coin.md#aiy_coin_DenyCapV2">aiy::coin::DenyCapV2</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../aiy/package.md#aiy_package">package</a>) <b>fun</b> <a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>&lt;T&gt;(cap: &<a href="../aiy/coin.md#aiy_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt;): bool {
    cap.<a href="../aiy/coin.md#aiy_coin_allow_global_pause">allow_global_pause</a>
}
</code></pre>



</details>

<a name="aiy_coin_supply"></a>

## Function `supply`



<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_supply">supply</a>&lt;T&gt;(treasury: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;): &<a href="../aiy/balance.md#aiy_balance_Supply">aiy::balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_supply">supply</a>&lt;T&gt;(treasury: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;): &Supply&lt;T&gt; {
    &treasury.<a href="../aiy/coin.md#aiy_coin_total_supply">total_supply</a>
}
</code></pre>



</details>

<a name="aiy_coin_create_regulated_currency"></a>

## Function `create_regulated_currency`

This creates a new currency, via <code><a href="../aiy/coin.md#aiy_coin_create_currency">create_currency</a></code>, but with an extra capability that
allows for specific addresses to have their coins frozen. Those addresses cannot interact
with the coin as input objects.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_create_regulated_currency">create_regulated_currency</a>&lt;T: drop&gt;(witness: T, decimals: u8, symbol: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, icon_url: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../aiy/url.md#aiy_url_Url">aiy::url::Url</a>&gt;, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): (<a href="../aiy/coin.md#aiy_coin_TreasuryCap">aiy::coin::TreasuryCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_DenyCap">aiy::coin::DenyCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_CoinMetadata">aiy::coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_create_regulated_currency">create_regulated_currency</a>&lt;T: drop&gt;(
    witness: T,
    decimals: u8,
    symbol: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    icon_url: Option&lt;Url&gt;,
    ctx: &<b>mut</b> TxContext,
): (<a href="../aiy/coin.md#aiy_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_DenyCap">DenyCap</a>&lt;T&gt;, <a href="../aiy/coin.md#aiy_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;) {
    <b>let</b> (treasury_cap, metadata) = <a href="../aiy/coin.md#aiy_coin_create_currency">create_currency</a>(
        witness,
        decimals,
        symbol,
        name,
        description,
        icon_url,
        ctx,
    );
    <b>let</b> deny_cap = <a href="../aiy/coin.md#aiy_coin_DenyCap">DenyCap</a> {
        id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx),
    };
    <a href="../aiy/transfer.md#aiy_transfer_freeze_object">transfer::freeze_object</a>(<a href="../aiy/coin.md#aiy_coin_RegulatedCoinMetadata">RegulatedCoinMetadata</a>&lt;T&gt; {
        id: <a href="../aiy/object.md#aiy_object_new">object::new</a>(ctx),
        coin_metadata_object: <a href="../aiy/object.md#aiy_object_id">object::id</a>(&metadata),
        deny_cap_object: <a href="../aiy/object.md#aiy_object_id">object::id</a>(&deny_cap),
    });
    (treasury_cap, deny_cap, metadata)
}
</code></pre>



</details>

<a name="aiy_coin_deny_list_add"></a>

## Function `deny_list_add`

Adds the given address to the deny list, preventing it
from interacting with the specified coin type as an input to a transaction.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_add">deny_list_add</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> <a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>, _deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCap">aiy::coin::DenyCap</a>&lt;T&gt;, addr: <b>address</b>, _ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_add">deny_list_add</a>&lt;T&gt;(
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    _deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCap">DenyCap</a>&lt;T&gt;,
    addr: <b>address</b>,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> `type` = type_name::into_string(type_name::get_with_original_ids&lt;T&gt;()).into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.v1_add(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, `type`, addr)
}
</code></pre>



</details>

<a name="aiy_coin_deny_list_remove"></a>

## Function `deny_list_remove`

Removes an address from the deny list.
Aborts with <code>ENotFrozen</code> if the address is not already in the list.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_remove">deny_list_remove</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> <a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>, _deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCap">aiy::coin::DenyCap</a>&lt;T&gt;, addr: <b>address</b>, _ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_remove">deny_list_remove</a>&lt;T&gt;(
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    _deny_cap: &<b>mut</b> <a href="../aiy/coin.md#aiy_coin_DenyCap">DenyCap</a>&lt;T&gt;,
    addr: <b>address</b>,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> `type` = type_name::into_string(type_name::get_with_original_ids&lt;T&gt;()).into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.v1_remove(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, `type`, addr)
}
</code></pre>



</details>

<a name="aiy_coin_deny_list_contains"></a>

## Function `deny_list_contains`

Returns true iff the given address is denied for the given coin type. It will
return false if given a non-coin type.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_contains">deny_list_contains</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &<a href="../aiy/deny_list.md#aiy_deny_list_DenyList">aiy::deny_list::DenyList</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/coin.md#aiy_coin_deny_list_contains">deny_list_contains</a>&lt;T&gt;(<a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>: &DenyList, addr: <b>address</b>): bool {
    <b>let</b> name = type_name::get_with_original_ids&lt;T&gt;();
    <b>if</b> (type_name::is_primitive(&name)) <b>return</b> <b>false</b>;
    <b>let</b> `type` = type_name::into_string(name).into_bytes();
    <a href="../aiy/deny_list.md#aiy_deny_list">deny_list</a>.v1_contains(<a href="../aiy/coin.md#aiy_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, `type`, addr)
}
</code></pre>



</details>
