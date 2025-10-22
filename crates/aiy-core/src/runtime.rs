// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use aiy_config::NodeConfig;
use tokio::runtime::Runtime;

pub struct AiyRuntimes {
    // Order in this struct is the order in which runtimes are stopped
    pub aiy_node: Runtime,
    pub metrics: Runtime,
}

impl AiyRuntimes {
    pub fn new(_confg: &NodeConfig) -> Self {
        let aiy_node = tokio::runtime::Builder::new_multi_thread()
            .thread_name("aiy-node-runtime")
            .enable_all()
            .build()
            .unwrap();
        let metrics = tokio::runtime::Builder::new_multi_thread()
            .thread_name("metrics-runtime")
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap();

        Self { aiy_node, metrics }
    }
}
