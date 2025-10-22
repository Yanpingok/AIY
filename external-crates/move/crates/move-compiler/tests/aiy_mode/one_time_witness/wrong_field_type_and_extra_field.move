module a::beep {
    struct BEEP has drop {
        f0: u64,
        f1: bool,
    }
    fun init(_ctx: &mut aiy::tx_context::TxContext) {
    }
}

module aiy::tx_context {
    struct TxContext has drop {}
}
