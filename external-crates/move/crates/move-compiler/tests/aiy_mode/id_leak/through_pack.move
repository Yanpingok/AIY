// not allowed since C is not packed with a fresh UID
module b::test {
    use aiy::object::UID;
    use aiy::transfer::transfer;

    struct A has key {
        id: UID
    }

    struct C has key {
        id: UID
    }

    struct B {
        id: UID
    }

    public entry fun test(x: A) {
        let A { id } = x;
        let b = B { id };
        let B { id } = b;
        let c = C { id };
        transfer(c, @1);
    }
}

// allowed since Bar does not have key
module a::m {
    use aiy::object::UID;

    struct Foo has key {
        id: UID,
    }

    struct Bar {
        id: UID,
        v: u64,
    }

    public fun foo(f: Foo) {
        let Foo { id } = f;
        let _b = Bar { id, v: 0 };
        abort 0
    }

}

module aiy::object {
    struct UID has store {
        id: address,
    }
}

module aiy::tx_context {
    struct TxContext has drop {}
    public fun sender(_: &TxContext): address {
        @0
    }
}

module aiy::transfer {
    public fun transfer<T: key>(_: T, _: address) {
        abort 0
    }
}
