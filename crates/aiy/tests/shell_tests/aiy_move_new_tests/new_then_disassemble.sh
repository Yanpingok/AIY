# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that aiy move new followed by aiy move disassemble succeeds


aiy move new example
cat > example/sources/example.move <<EOF
module example::example;

public fun foo(_ctx: &mut TxContext) {}
EOF
cd example

echo "=== Build ===" | tee /dev/stderr
aiy move build

echo "=== Disassemble ===" | tee /dev/stderr
aiy move disassemble build/example/bytecode_modules/example.mv
