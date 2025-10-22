// invalid, a reference to vector of objects

module a::m {
    use aiy::object;

    struct S has key { id: object::UID }

    public entry fun no(_: &vector<S>) {
        abort 0
    }

}

module aiy::object {
    struct UID has store {
        id: address,
    }
}
