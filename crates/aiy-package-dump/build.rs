// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

fn main() {
    cynic_codegen::register_schema("aiy")
        .from_sdl_file("../aiy-graphql-rpc/schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
