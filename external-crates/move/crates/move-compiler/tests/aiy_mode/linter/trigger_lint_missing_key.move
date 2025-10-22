module a::trigger_lint_cases {
    use aiy::object::UID;

    // This should trigger the linter warning (true positive)
    struct MissingKeyAbility {
        id: UID,
    }

}

module aiy::object {
    struct UID has store {
        id: address,
    }
}
