// invalid, Clock by mutable reference

module a::m {
    public entry fun no_clock_mut(_: &mut aiy::clock::Clock) {
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
