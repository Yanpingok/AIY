// invalid, objects need UID not ID
module a::m {
    use aiy::object;
    struct S has key {
        id: object::ID
    }
}

module aiy::object {
    struct ID has store {
        id: address,
    }
}
