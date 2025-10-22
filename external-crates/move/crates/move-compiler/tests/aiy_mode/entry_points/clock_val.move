// invalid, Clock by value

module a::m {
    public entry fun no_clock_val(_: aiy::clock::Clock) {
        abort 0
    }
}

module aiy::clock {
    struct Clock has key {
        id: aiy::object::UID,
    }
}

module aiy::object {
    struct UID has store {
        id: address,
    }
}
