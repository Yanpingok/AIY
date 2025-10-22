// invalid Random by mutable reference

module a::m {
    public entry fun no_random_mut(_: &mut aiy::random::Random) {
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
