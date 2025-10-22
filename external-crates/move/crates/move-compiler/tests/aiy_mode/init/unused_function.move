// init is unused but does not error because we are in Aiy mode
module a::m {
    fun init(_: &mut aiy::tx_context::TxContext) {}
}

module aiy::tx_context {
    struct TxContext has drop {}
}
