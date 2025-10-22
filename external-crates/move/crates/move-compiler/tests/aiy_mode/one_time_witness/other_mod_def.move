// invalid, one-time witness type candidate used in a different module

module a::n {
    use aiy::aiy;
    use aiy::tx_context;

    fun init(_otw: aiy::AIY, _ctx: &mut tx_context::TxContext) {
    }

}


module aiy::tx_context {
    struct TxContext has drop {}
}

module aiy::aiy {
    struct AIY has drop {}
}
