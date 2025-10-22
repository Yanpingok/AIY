module aiy::object {
    public struct ID()
    public struct UID()
}
module aiy::transfer {}
module aiy::tx_context {
    public struct TxContext()
}

module a::m {
    use aiy::object::{Self, ID, UID};
    use aiy::transfer;
    use aiy::tx_context::{Self, TxContext};
}
