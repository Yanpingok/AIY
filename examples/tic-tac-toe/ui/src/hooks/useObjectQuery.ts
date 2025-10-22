// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useAiyClientContext, useAiyClientQuery, UseAiyClientQueryOptions } from "@mysten/dapp-kit";
import { GetObjectParams, AiyObjectResponse } from "@mysten/aiy/client";
import { useQueryClient, UseQueryResult } from "@tanstack/react-query";

export type UseObjectQueryOptions = UseAiyClientQueryOptions<"getObject", AiyObjectResponse>;
export type UseObjectQueryResponse = UseQueryResult<AiyObjectResponse, Error>;
export type InvalidateUseObjectQuery = () => void;

/**
 * Fetches an object, returning the response from RPC and a callback
 * to invalidate it.
 */
export function useObjectQuery(
    params: GetObjectParams,
    options?: UseObjectQueryOptions,
): [UseObjectQueryResponse, InvalidateUseObjectQuery] {
    const ctx = useAiyClientContext();
    const client = useQueryClient();
    const response = useAiyClientQuery("getObject", params, options);

    const invalidate = async () => {
        await client.invalidateQueries({
            queryKey: [ctx.network, "getObject", params],
        });
    };

    return [response, invalidate];
}
