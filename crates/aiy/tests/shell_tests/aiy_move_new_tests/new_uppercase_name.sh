# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# test that `aiy move new` works as expected with `<NAME>` containing uppercase letter(s)
aiy move new _Example_A
echo ==== files in project ====
ls -A _Example_A
echo ==== files in sources ====
ls -A _Example_A/sources
echo ==== files in tests =====
ls -A _Example_A/tests
