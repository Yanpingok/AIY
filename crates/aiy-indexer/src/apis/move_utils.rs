// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use move_binary_format::normalized;

use aiy_json_rpc::error::AiyRpcInputError;
use aiy_json_rpc::AiyRpcModule;
use aiy_json_rpc_api::MoveUtilsServer;
use aiy_json_rpc_types::ObjectValueKind;
use aiy_json_rpc_types::AiyMoveNormalizedType;
use aiy_json_rpc_types::{
    MoveFunctionArgType, AiyMoveNormalizedFunction, AiyMoveNormalizedModule,
    AiyMoveNormalizedStruct,
};
use aiy_open_rpc::Module;
use aiy_types::base_types::ObjectID;

use crate::indexer_reader::IndexerReader;

pub struct MoveUtilsApi {
    inner: IndexerReader,
}

impl MoveUtilsApi {
    pub fn new(inner: IndexerReader) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl MoveUtilsServer for MoveUtilsApi {
    async fn get_normalized_move_modules_by_package(
        &self,
        package_id: ObjectID,
    ) -> RpcResult<BTreeMap<String, AiyMoveNormalizedModule>> {
        let resolver_modules = self.inner.get_package(package_id).await?.modules().clone();
        let pool = &mut normalized::RcPool::new();
        let aiy_normalized_modules = resolver_modules
            .into_iter()
            .map(|(k, v)| {
                let m = &normalized::Module::new(pool, v.bytecode(), /* include code */ false);
                (k, m.into())
            })
            .collect::<BTreeMap<String, AiyMoveNormalizedModule>>();
        Ok(aiy_normalized_modules)
    }

    async fn get_normalized_move_module(
        &self,
        package: ObjectID,
        module_name: String,
    ) -> RpcResult<AiyMoveNormalizedModule> {
        let mut modules = self.get_normalized_move_modules_by_package(package).await?;
        let module = modules.remove(&module_name).ok_or_else(|| {
            AiyRpcInputError::GenericNotFound(format!(
                "No module was found with name {module_name}",
            ))
        })?;
        Ok(module)
    }

    async fn get_normalized_move_struct(
        &self,
        package: ObjectID,
        module_name: String,
        struct_name: String,
    ) -> RpcResult<AiyMoveNormalizedStruct> {
        let mut module = self
            .get_normalized_move_module(package, module_name)
            .await?;
        module
            .structs
            .remove(&struct_name)
            .ok_or_else(|| {
                AiyRpcInputError::GenericNotFound(format!(
                    "No struct was found with struct name {struct_name}"
                ))
            })
            .map_err(Into::into)
    }

    async fn get_normalized_move_function(
        &self,
        package: ObjectID,
        module_name: String,
        function_name: String,
    ) -> RpcResult<AiyMoveNormalizedFunction> {
        let mut module = self
            .get_normalized_move_module(package, module_name)
            .await?;
        module
            .exposed_functions
            .remove(&function_name)
            .ok_or_else(|| {
                AiyRpcInputError::GenericNotFound(format!(
                    "No function was found with function name {function_name}",
                ))
            })
            .map_err(Into::into)
    }

    async fn get_move_function_arg_types(
        &self,
        package: ObjectID,
        module: String,
        function: String,
    ) -> RpcResult<Vec<MoveFunctionArgType>> {
        let function = self
            .get_normalized_move_function(package, module, function)
            .await?;
        let args = function
            .parameters
            .iter()
            .map(|p| match p {
                AiyMoveNormalizedType::Struct { .. } => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByValue)
                }
                AiyMoveNormalizedType::Vector(_) => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByValue)
                }
                AiyMoveNormalizedType::Reference(_) => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByImmutableReference)
                }
                AiyMoveNormalizedType::MutableReference(_) => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByMutableReference)
                }
                _ => MoveFunctionArgType::Pure,
            })
            .collect::<Vec<MoveFunctionArgType>>();
        Ok(args)
    }
}

impl AiyRpcModule for MoveUtilsApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        aiy_json_rpc_api::MoveUtilsOpenRpc::module_doc()
    }
}
