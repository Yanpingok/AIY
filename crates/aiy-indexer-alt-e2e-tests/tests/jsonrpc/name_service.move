// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 70 --simulator

// Testing various input errors for the AiyNS name resolution:
// 1. Not enough labels (need at least two)
// 2. Too long
// 3. Bad (inconsistent) use of separators
// 4. Indvidual label too short
// 5. Individual label too long
// 6, 7, 8, 9. Bad characters (non-alphanumeric, or leading/trailing hyphen)

//# run-jsonrpc
{
  "method": "aiyx_resolveNameServiceAddress",
  "params": ["foo"]
}

//# run-jsonrpc
{
  "method": "aiyx_resolveNameServiceAddress",
  "params": ["toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.toolong.aiy"]
}

//# run-jsonrpc
{
  "method": "aiyx_resolveNameServiceAddress",
  "params": ["foo*bar.aiy"]
}

//# run-jsonrpc
{
  "method": "aiyx_resolveNameServiceAddress",
  "params": ["foo..aiy"]
}

//# run-jsonrpc
{
  "method": "aiyx_resolveNameServiceAddress",
  "params": ["toolongtoolongtoolongtoolongtoolongtoolongtoolongtoolongtoolongtoolong.aiy"]
}

//# run-jsonrpc
{
  "method": "aiyx_resolveNameServiceAddress",
  "params": ["-foo.aiy"]
}

//# run-jsonrpc
{
  "method": "aiyx_resolveNameServiceAddress",
  "params": ["foo-.aiy"]
}

//# run-jsonrpc
{
  "method": "aiyx_resolveNameServiceAddress",
  "params": ["foo_bar.aiy"]
}

//# run-jsonrpc
{
  "method": "aiyx_resolveNameServiceAddress",
  "params": ["🫠.aiy"]
}
