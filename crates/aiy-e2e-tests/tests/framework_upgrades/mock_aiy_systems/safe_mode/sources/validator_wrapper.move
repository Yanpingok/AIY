// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module aiy_system::validator_wrapper {
    use aiy::versioned::Versioned;

    public struct ValidatorWrapper has store {
        inner: Versioned
    }
}
