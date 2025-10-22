#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

echo "Start Rosetta online server"
aiy-rosetta start-online-server --data-path ./data &

echo "Start Rosetta offline server"
aiy-rosetta start-offline-server &
