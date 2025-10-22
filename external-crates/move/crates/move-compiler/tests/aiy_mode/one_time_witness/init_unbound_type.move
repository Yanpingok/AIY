module a::m {
    fun init(_ctx: who::TxContext) {}
}

module a::beep {
    struct BEEP has drop {}
    fun init(_: Who, _ctx: &mut aiy::tx_context::TxContext) {}
}

module aiy::tx_context {
    struct TxContext {}
}
