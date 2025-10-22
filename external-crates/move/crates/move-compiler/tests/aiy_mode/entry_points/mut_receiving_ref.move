// valid, Receiving type by mut ref with object type param

module a::m {
    use aiy::object;
    use aiy::transfer::Receiving;

    struct S has key { id: object::UID }

    public entry fun yes(_: &mut Receiving<S>) { }
}

module aiy::object {
    struct UID has store {
        id: address,
    }
}

module aiy::transfer {
    struct Receiving<phantom T: key> has drop {
        id: address
    }
}
