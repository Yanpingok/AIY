// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use std::env;
use std::sync::Arc;
use aiy_config::aiy_config_dir;
use aiy_faucet::{create_wallet_context, start_faucet, AppState};
use aiy_faucet::{FaucetConfig, LocalFaucet};

// Define the `GIT_REVISION` and `VERSION` consts
bin_version::bin_version!();

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config: FaucetConfig = FaucetConfig::parse();
    let FaucetConfig {
        wallet_client_timeout_secs,
        ..
    } = config;

    let context = create_wallet_context(wallet_client_timeout_secs, aiy_config_dir()?)?;

    let app_state = Arc::new(AppState {
        faucet: LocalFaucet::new(context, config.clone()).await.unwrap(),
        config,
    });

    start_faucet(app_state).await
}
