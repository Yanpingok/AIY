// valid Random by immutable reference

module a::m {
    public entry fun yes_random_ref(_: &aiy::random::Random) {
        abort 0
    }
}

module aiy::random {
    struct Random has key {
        id: aiy::object::UID,
    }
}

module aiy::object {
    struct UID has store {
        id: address,
    }
}
