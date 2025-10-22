#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

echo "Install binaries"
cargo install --locked --bin aiy --path crates/aiy
cargo install --locked --bin aiy-rosetta --path crates/aiy-rosetta

echo "run Aiy genesis"
aiy genesis

echo "generate rosetta configuration"
aiy-rosetta generate-rosetta-cli-config --online-url http://127.0.0.1:9002 --offline-url http://127.0.0.1:9003

echo "install rosetta-cli"
curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s