// not allowed since C is not packed with a fresh UID
module a::a {
    use aiy::object::UID;

    struct A has key {
        id: UID
    }
}

module b::b {
    use aiy::object::UID;
    use a::a::A;

    struct B has key {
        id: UID
    }
    public fun no(b: B): A {
        let B { id } = b;
        A { id }
    }
}

module aiy::object {
    struct UID has store {
        id: address,
    }
}
