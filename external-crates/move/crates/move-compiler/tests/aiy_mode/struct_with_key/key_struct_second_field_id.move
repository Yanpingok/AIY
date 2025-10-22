// invalid, object must have UID as first field not some other field

module a::m {
    use aiy::object;
    struct S has key {
        flag: bool,
        id: object::UID,
    }

    struct R has key {
        flag: bool,
        id: address,
    }
}

module aiy::object {
    struct UID has store {
        id: address,
    }
}
