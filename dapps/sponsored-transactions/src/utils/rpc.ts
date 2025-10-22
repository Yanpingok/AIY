// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getFullnodeUrl, AiyClient } from '@mysten/aiy/client';

export const client = new AiyClient({ url: getFullnodeUrl('testnet') });
