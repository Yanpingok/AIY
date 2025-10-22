# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# check that aiy move new followed by aiy move test succeeds
aiy move new example
cd example && aiy move test
