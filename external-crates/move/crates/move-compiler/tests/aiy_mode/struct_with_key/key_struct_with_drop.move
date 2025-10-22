// invalid, object cannot have drop since UID does not have drop

module a::m {
    use aiy::object;
    struct S has key, drop {
        id: object::UID,
        flag: bool
    }
}

module aiy::object {
    struct UID has store {
        id: address,
    }
}
