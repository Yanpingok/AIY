// invalid Random by value

module a::m {
    public entry fun no_random_val(_: aiy::random::Random) {
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
