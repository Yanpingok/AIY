// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use aiy_enum_compat_util::*;

use crate::{AiyMoveStruct, AiyMoveValue};

#[test]
fn enforce_order_test() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "aiy_move_struct.yaml"]);
    check_enum_compat_order::<AiyMoveStruct>(path);

    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "aiy_move_value.yaml"]);
    check_enum_compat_order::<AiyMoveValue>(path);
}
