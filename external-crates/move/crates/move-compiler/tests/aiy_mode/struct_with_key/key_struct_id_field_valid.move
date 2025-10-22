// valid
module a::m {
    use aiy::object;
    struct S has key {
        id: object::UID
    }
}

module aiy::object {
    struct UID has store {
        id: address,
    }
}
