// valid init function
module a::m {
    use aiy::tx_context;
    fun init(_: &mut tx_context::TxContext) {
    }
}

module aiy::tx_context {
    struct TxContext has drop {}
}
