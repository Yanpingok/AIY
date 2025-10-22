// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module contains the public APIs supported by the bytecode verifier.

use move_binary_format::file_format::CompiledModule;
use aiy_protocol_config::ProtocolConfig;
use aiy_types::{error::ExecutionError, move_package::FnInfoMap};

use crate::{
    entry_points_verifier, global_storage_access_verifier, id_leak_verifier,
    one_time_witness_verifier, private_generics, struct_with_key_verifier,
};
use move_bytecode_verifier_meter::dummy::DummyMeter;
use move_bytecode_verifier_meter::Meter;

/// Helper for a "canonical" verification of a module.
pub fn aiy_verify_module_metered(
    config: &ProtocolConfig,
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
    meter: &mut (impl Meter + ?Sized),
) -> Result<(), ExecutionError> {
    struct_with_key_verifier::verify_module(module)?;
    global_storage_access_verifier::verify_module(module)?;
    id_leak_verifier::verify_module(module, meter)?;
    private_generics::verify_module(module)?;
    entry_points_verifier::verify_module(config, module, fn_info_map)?;
    one_time_witness_verifier::verify_module(module, fn_info_map)
}

/// Runs the Aiy verifier and checks if the error counts as a Aiy verifier timeout
/// NOTE: this function only check if the verifier error is a timeout
/// All other errors are ignored
pub fn aiy_verify_module_metered_check_timeout_only(
    config: &ProtocolConfig,
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
    meter: &mut (impl Meter + ?Sized),
) -> Result<(), ExecutionError> {
    // Checks if the error counts as a Aiy verifier timeout
    if let Err(error) = aiy_verify_module_metered(config, module, fn_info_map, meter) {
        if matches!(
            error.kind(),
            aiy_types::execution_status::ExecutionFailureStatus::AiyMoveVerificationTimedout
        ) {
            return Err(error);
        }
    }
    // Any other scenario, including a non-timeout error counts as Ok
    Ok(())
}

pub fn aiy_verify_module_unmetered(
    config: &ProtocolConfig,
    module: &CompiledModule,
    fn_info_map: &FnInfoMap,
) -> Result<(), ExecutionError> {
    aiy_verify_module_metered(config, module, fn_info_map, &mut DummyMeter).inspect_err(|err| {
        // We must never see timeout error in execution
        debug_assert!(
            !matches!(
                err.kind(),
                aiy_types::execution_status::ExecutionFailureStatus::AiyMoveVerificationTimedout
            ),
            "Unexpected timeout error in execution"
        );
    })
}
