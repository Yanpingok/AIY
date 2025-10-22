// valid, Clock by immutable reference

module a::m {
    public entry fun yes_clock_ref(_: &aiy::clock::Clock) {
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
