module a::m {
    struct Obj has key { id: aiy::object::UID }
}

module aiy::object {
    struct UID has store { value: address }
    public fun borrow_address(id: &UID): &address { &id.value }
}
