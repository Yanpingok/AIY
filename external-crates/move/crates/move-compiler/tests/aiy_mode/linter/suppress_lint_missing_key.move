module a::trigger_lint_cases {
    use aiy::object::UID;

    // 4. Suppress warning
    #[allow(lint(missing_key))]
    struct SuppressWarning {
       id: UID,
    }
}

module aiy::object {
    struct UID has store {
        id: address,
    }
}
