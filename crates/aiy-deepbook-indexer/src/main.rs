// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::*;
use mysten_metrics::start_prometheus_server;
use std::env;
use std::net::IpAddr;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use aiy_config::Config;
use aiy_data_ingestion_core::DataIngestionMetrics;
use aiy_deepbook_indexer::config::IndexerConfig;
use aiy_deepbook_indexer::metrics::DeepBookIndexerMetrics;
use aiy_deepbook_indexer::postgres_manager::get_connection_pool;
use aiy_deepbook_indexer::server::run_server;
use aiy_deepbook_indexer::aiy_deepbook_indexer::PgDeepbookPersistent;
use aiy_deepbook_indexer::aiy_deepbook_indexer::AiyDeepBookDataMapper;
use aiy_indexer_builder::indexer_builder::IndexerBuilder;
use aiy_indexer_builder::progress::{OutOfOrderSaveAfterDurationPolicy, ProgressSavingPolicy};
use aiy_indexer_builder::aiy_datasource::AiyCheckpointDatasource;
use aiy_sdk::AiyClientBuilder;
use aiy_types::base_types::ObjectID;
use tracing::info;

#[derive(Parser, Clone, Debug)]
struct Args {
    /// Path to a yaml config
    #[clap(long, short)]
    config_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let args = Args::parse();

    // load config
    let config_path = if let Some(path) = args.config_path {
        path
    } else {
        env::current_dir()
            .expect("Couldn't get current directory")
            .join("config.yaml")
    };
    let config = IndexerConfig::load(&config_path)?;

    // Init metrics server
    let metrics_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.metric_port);
    let registry_service = start_prometheus_server(metrics_address);
    let registry = registry_service.default_registry();
    mysten_metrics::init_metrics(&registry);
    info!("Metrics server started at port {}", config.metric_port);

    let indexer_meterics = DeepBookIndexerMetrics::new(&registry);
    let ingestion_metrics = DataIngestionMetrics::new(&registry);

    let db_url = config.db_url.clone();
    let datastore = PgDeepbookPersistent::new(
        get_connection_pool(db_url.clone()).await,
        ProgressSavingPolicy::OutOfOrderSaveAfterDuration(OutOfOrderSaveAfterDurationPolicy::new(
            tokio::time::Duration::from_secs(30),
        )),
    );

    let aiy_client = Arc::new(
        AiyClientBuilder::default()
            .build(config.aiy_rpc_url.clone())
            .await?,
    );
    let aiy_checkpoint_datasource = AiyCheckpointDatasource::new(
        config.remote_store_url,
        aiy_client,
        config.concurrency as usize,
        config
            .checkpoints_path
            .map(|p| p.into())
            .unwrap_or(tempfile::tempdir()?.keep()),
        config.deepbook_genesis_checkpoint,
        ingestion_metrics.clone(),
        Box::new(indexer_meterics.clone()),
    );

    let service_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.service_port);
    run_server(service_address, datastore.clone());

    let indexer = IndexerBuilder::new(
        "AiyDeepBookIndexer",
        aiy_checkpoint_datasource,
        AiyDeepBookDataMapper {
            metrics: indexer_meterics.clone(),
            package_id: ObjectID::from_hex_literal(&config.deepbook_package_id.clone())
                .unwrap_or_else(|err| panic!("Failed to parse deepbook package ID: {}", err)),
        },
        datastore,
    )
    .build();
    indexer.start().await?;

    Ok(())
}
