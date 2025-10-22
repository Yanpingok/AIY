// valid, ID is allowed

module a::m {
    use aiy::object;

    public entry fun yes<T>(
        _: object::ID,
        _: vector<object::ID>,
        _: vector<vector<object::ID>>,
    ) {
        abort 0
    }

}

module aiy::object {
    struct ID has store {
        id: address,
    }
}
