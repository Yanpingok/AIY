---
title: Module `aiy_system::validator_wrapper`
---



-  [Struct `ValidatorWrapper`](#aiy_system_validator_wrapper_ValidatorWrapper)
-  [Constants](#@Constants_0)
-  [Function `create_v1`](#aiy_system_validator_wrapper_create_v1)
-  [Function `load_validator_maybe_upgrade`](#aiy_system_validator_wrapper_load_validator_maybe_upgrade)
-  [Function `destroy`](#aiy_system_validator_wrapper_destroy)
-  [Function `upgrade_to_latest`](#aiy_system_validator_wrapper_upgrade_to_latest)
-  [Function `version`](#aiy_system_validator_wrapper_version)


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
<b>use</b> <a href="../aiy/versioned.md#aiy_versioned">aiy::versioned</a>;
<b>use</b> <a href="../aiy_system/staking_pool.md#aiy_system_staking_pool">aiy_system::staking_pool</a>;
<b>use</b> <a href="../aiy_system/validator.md#aiy_system_validator">aiy_system::validator</a>;
<b>use</b> <a href="../aiy_system/validator_cap.md#aiy_system_validator_cap">aiy_system::validator_cap</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="aiy_system_validator_wrapper_ValidatorWrapper"></a>

## Struct `ValidatorWrapper`



<pre><code><b>public</b> <b>struct</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inner: <a href="../aiy/versioned.md#aiy_versioned_Versioned">aiy::versioned::Versioned</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="aiy_system_validator_wrapper_EInvalidVersion"></a>



<pre><code><b>const</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_EInvalidVersion">EInvalidVersion</a>: u64 = 0;
</code></pre>



<a name="aiy_system_validator_wrapper_create_v1"></a>

## Function `create_v1`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_create_v1">create_v1</a>(<a href="../aiy_system/validator.md#aiy_system_validator">validator</a>: <a href="../aiy_system/validator.md#aiy_system_validator_Validator">aiy_system::validator::Validator</a>, ctx: &<b>mut</b> <a href="../aiy/tx_context.md#aiy_tx_context_TxContext">aiy::tx_context::TxContext</a>): <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">aiy_system::validator_wrapper::ValidatorWrapper</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_create_v1">create_v1</a>(<a href="../aiy_system/validator.md#aiy_system_validator">validator</a>: Validator, ctx: &<b>mut</b> TxContext): <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> {
    <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> {
        inner: versioned::create(1, <a href="../aiy_system/validator.md#aiy_system_validator">validator</a>, ctx),
    }
}
</code></pre>



</details>

<a name="aiy_system_validator_wrapper_load_validator_maybe_upgrade"></a>

## Function `load_validator_maybe_upgrade`

This function should always return the latest supported version.
If the inner version is old, we upgrade it lazily in-place.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">aiy_system::validator_wrapper::ValidatorWrapper</a>): &<b>mut</b> <a href="../aiy_system/validator.md#aiy_system_validator_Validator">aiy_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): &<b>mut</b> Validator {
    self.<a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>();
    self.inner.load_value_mut()
}
</code></pre>



</details>

<a name="aiy_system_validator_wrapper_destroy"></a>

## Function `destroy`

Destroy the wrapper and retrieve the inner validator object.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_destroy">destroy</a>(self: <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">aiy_system::validator_wrapper::ValidatorWrapper</a>): <a href="../aiy_system/validator.md#aiy_system_validator_Validator">aiy_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_destroy">destroy</a>(self: <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): Validator {
    <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(&self);
    <b>let</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> { inner } = self;
    inner.<a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_destroy">destroy</a>()
}
</code></pre>



</details>

<a name="aiy_system_validator_wrapper_upgrade_to_latest"></a>

## Function `upgrade_to_latest`



<pre><code><b>fun</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(self: &<a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">aiy_system::validator_wrapper::ValidatorWrapper</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(self: &<a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>) {
    <b>let</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_version">version</a> = self.<a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_version">version</a>();
    // TODO: When new versions are added, we need to explicitly upgrade here.
    <b>assert</b>!(<a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_version">version</a> == 1, <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_EInvalidVersion">EInvalidVersion</a>);
}
</code></pre>



</details>

<a name="aiy_system_validator_wrapper_version"></a>

## Function `version`



<pre><code><b>fun</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_version">version</a>(self: &<a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">aiy_system::validator_wrapper::ValidatorWrapper</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_version">version</a>(self: &<a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): u64 {
    self.inner.<a href="../aiy_system/validator_wrapper.md#aiy_system_validator_wrapper_version">version</a>()
}
</code></pre>



</details>
