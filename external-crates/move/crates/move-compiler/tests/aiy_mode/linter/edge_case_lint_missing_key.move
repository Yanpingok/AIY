module a::edge_cases {
    struct UID {}
    // Test case with a different UID type
    struct DifferentUID {
        id: aiy::another::UID,
    }

    struct NotAnObject {
        id: UID,
    }

}

module aiy::object {
    struct UID has store {
        id: address,
    }
}

module aiy::another {
    struct UID has store {
        id: address,
    }
}
