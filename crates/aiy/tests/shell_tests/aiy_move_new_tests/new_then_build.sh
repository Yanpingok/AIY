# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that aiy move new followed by aiy move build succeeds

aiy move new example
cd example && aiy move build
