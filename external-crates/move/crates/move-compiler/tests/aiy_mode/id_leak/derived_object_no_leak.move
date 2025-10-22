// valid because we can use `derived_object::claim` without triggering id leak
module a::m {
  use aiy::derived_object;
  use aiy::object;

  struct A has key {
    id: object::UID,
  }

  public fun no_leak(ctx: &mut aiy::tx_context::TxContext): A {
    A {
      id: derived_object::claim(object::new(ctx), 0),
    }
  }
}

module aiy::object {
  struct UID has store {
    id: address,
  }

  public fun new(_: &mut aiy::tx_context::TxContext): UID {
    abort 0
  }
}

module aiy::tx_context {
  struct TxContext has drop {}
}

module aiy::derived_object {
  use aiy::object::UID;

  public fun claim<T: copy + store + drop>(_: UID, _: T): UID {
    abort 0
  }
}
