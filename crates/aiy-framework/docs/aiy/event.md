---
title: Module `aiy::event`
---

Events module. Defines the <code><a href="../aiy/event.md#aiy_event_emit">aiy::event::emit</a></code> function which
creates and sends a custom MoveEvent as a part of the effects
certificate of the transaction.

Every MoveEvent has the following properties:
- sender
- type signature (<code>T</code>)
- event data (the value of <code>T</code>)
- timestamp (local to a node)
- transaction digest

Example:
```
module my::marketplace {
use aiy::event;
/* ... */
struct ItemPurchased has copy, drop {
item_id: ID, buyer: address
}
entry fun buy(/* .... */) {
/* ... */
event::emit(ItemPurchased { item_id: ..., buyer: .... })
}
}
```


-  [Function `emit`](#aiy_event_emit)
-  [Function `emit_authenticated`](#aiy_event_emit_authenticated)
-  [Function `emit_authenticated_impl`](#aiy_event_emit_authenticated_impl)


<pre><code><b>use</b> <a href="../aiy/accumulator.md#aiy_accumulator">aiy::accumulator</a>;
<b>use</b> <a href="../aiy/accumulator_metadata.md#aiy_accumulator_metadata">aiy::accumulator_metadata</a>;
<b>use</b> <a href="../aiy/accumulator_settlement.md#aiy_accumulator_settlement">aiy::accumulator_settlement</a>;
<b>use</b> <a href="../aiy/address.md#aiy_address">aiy::address</a>;
<b>use</b> <a href="../aiy/bag.md#aiy_bag">aiy::bag</a>;
<b>use</b> <a href="../aiy/bcs.md#aiy_bcs">aiy::bcs</a>;
<b>use</b> <a href="../aiy/dynamic_field.md#aiy_dynamic_field">aiy::dynamic_field</a>;
<b>use</b> <a href="../aiy/hash.md#aiy_hash">aiy::hash</a>;
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



<a name="aiy_event_emit"></a>

## Function `emit`

Emit a custom Move event, sending the data offchain.

Used for creating custom indexes and tracking onchain
activity in a way that aiyts a specific application the most.

The type <code>T</code> is the main way to index the event, and can contain
phantom parameters, eg <code><a href="../aiy/event.md#aiy_event_emit">emit</a>(MyEvent&lt;<b>phantom</b> T&gt;)</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/event.md#aiy_event_emit">emit</a>&lt;T: <b>copy</b>, drop&gt;(<a href="../aiy/event.md#aiy_event">event</a>: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="../aiy/event.md#aiy_event_emit">emit</a>&lt;T: <b>copy</b> + drop&gt;(<a href="../aiy/event.md#aiy_event">event</a>: T);
</code></pre>



</details>

<a name="aiy_event_emit_authenticated"></a>

## Function `emit_authenticated`

Emits a custom Move event which can be authenticated by a light client.

This method emits the authenticated event to the event stream for the Move package that
defines the event type <code>T</code>.
Only the package that defines the type <code>T</code> can emit authenticated events to this stream.


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/event.md#aiy_event_emit_authenticated">emit_authenticated</a>&lt;T: <b>copy</b>, drop&gt;(<a href="../aiy/event.md#aiy_event">event</a>: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../aiy/event.md#aiy_event_emit_authenticated">emit_authenticated</a>&lt;T: <b>copy</b> + drop&gt;(<a href="../aiy/event.md#aiy_event">event</a>: T) {
    <b>let</b> stream_id = type_name::original_id&lt;T&gt;();
    <b>let</b> accumulator_addr = <a href="../aiy/accumulator.md#aiy_accumulator_accumulator_address">accumulator::accumulator_address</a>&lt;EventStreamHead&gt;(stream_id);
    <a href="../aiy/event.md#aiy_event_emit_authenticated_impl">emit_authenticated_impl</a>&lt;EventStreamHead, T&gt;(accumulator_addr, stream_id, <a href="../aiy/event.md#aiy_event">event</a>);
}
</code></pre>



</details>

<a name="aiy_event_emit_authenticated_impl"></a>

## Function `emit_authenticated_impl`



<pre><code><b>fun</b> <a href="../aiy/event.md#aiy_event_emit_authenticated_impl">emit_authenticated_impl</a>&lt;StreamHeadT, T: <b>copy</b>, drop&gt;(accumulator_id: <b>address</b>, stream: <b>address</b>, <a href="../aiy/event.md#aiy_event">event</a>: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../aiy/event.md#aiy_event_emit_authenticated_impl">emit_authenticated_impl</a>&lt;StreamHeadT, T: <b>copy</b> + drop&gt;(
    accumulator_id: <b>address</b>,
    stream: <b>address</b>,
    <a href="../aiy/event.md#aiy_event">event</a>: T,
);
</code></pre>



</details>
