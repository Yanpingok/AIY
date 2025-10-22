// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! The AiyBridgeStatus observable monitors whether the Aiy Bridge is paused.

use crate::Observable;
use async_trait::async_trait;
use prometheus::IntGauge;
use std::sync::Arc;
use aiy_bridge::aiy_client::AiyBridgeClient;

use tokio::time::Duration;
use tracing::{error, info};

pub struct AiyBridgeStatus {
    aiy_client: Arc<AiyBridgeClient>,
    metric: IntGauge,
}

impl AiyBridgeStatus {
    pub fn new(aiy_client: Arc<AiyBridgeClient>, metric: IntGauge) -> Self {
        Self { aiy_client, metric }
    }
}

#[async_trait]
impl Observable for AiyBridgeStatus {
    fn name(&self) -> &str {
        "AiyBridgeStatus"
    }

    async fn observe_and_report(&self) {
        let status = self.aiy_client.is_bridge_paused().await;
        match status {
            Ok(status) => {
                self.metric.set(status as i64);
                info!("Aiy Bridge Status: {:?}", status);
            }
            Err(e) => {
                error!("Error getting aiy bridge status: {:?}", e);
            }
        }
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(2)
    }
}
