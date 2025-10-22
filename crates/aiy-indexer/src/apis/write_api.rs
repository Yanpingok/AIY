// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;

use aiy_json_rpc::AiyRpcModule;
use aiy_json_rpc_api::{WriteApiClient, WriteApiServer};
use aiy_json_rpc_types::{
    DevInspectArgs, DevInspectResults, DryRunTransactionBlockResponse, AiyTransactionBlockResponse,
    AiyTransactionBlockResponseOptions,
};
use aiy_open_rpc::Module;
use aiy_types::base_types::AiyAddress;
use aiy_types::quorum_driver_types::ExecuteTransactionRequestType;
use aiy_types::aiy_serde::BigInt;

use crate::types::AiyTransactionBlockResponseWithOptions;

pub(crate) struct WriteApi {
    fullnode: HttpClient,
}

impl WriteApi {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self {
            fullnode: fullnode_client,
        }
    }
}

#[async_trait]
impl WriteApiServer for WriteApi {
    async fn execute_transaction_block(
        &self,
        tx_bytes: Base64,
        signatures: Vec<Base64>,
        options: Option<AiyTransactionBlockResponseOptions>,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> RpcResult<AiyTransactionBlockResponse> {
        let aiy_transaction_response = self
            .fullnode
            .execute_transaction_block(tx_bytes, signatures, options.clone(), request_type)
            .await
            .map_err(crate::errors::client_error_to_error_object)?;
        Ok(AiyTransactionBlockResponseWithOptions {
            response: aiy_transaction_response,
            options: options.unwrap_or_default(),
        }
        .into())
    }

    async fn dev_inspect_transaction_block(
        &self,
        sender_address: AiyAddress,
        tx_bytes: Base64,
        gas_price: Option<BigInt<u64>>,
        epoch: Option<BigInt<u64>>,
        additional_args: Option<DevInspectArgs>,
    ) -> RpcResult<DevInspectResults> {
        self.fullnode
            .dev_inspect_transaction_block(
                sender_address,
                tx_bytes,
                gas_price,
                epoch,
                additional_args,
            )
            .await
            .map_err(crate::errors::client_error_to_error_object)
    }

    async fn dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> RpcResult<DryRunTransactionBlockResponse> {
        self.fullnode
            .dry_run_transaction_block(tx_bytes)
            .await
            .map_err(crate::errors::client_error_to_error_object)
    }
}

impl AiyRpcModule for WriteApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        aiy_json_rpc_api::WriteApiOpenRpc::module_doc()
    }
}
