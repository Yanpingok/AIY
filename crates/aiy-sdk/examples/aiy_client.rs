// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use aiy_sdk::AiyClientBuilder;

// This example shows the few basic ways to connect to a Aiy network.
// There are several in-built methods for connecting to the
// Aiy devnet, tesnet, and localnet (running locally),
// as well as a custom way for connecting to custom URLs.
// The example prints out the API versions of the different networks,
// and finally, it prints the list of available RPC methods
// and the list of subscriptions.
// Note that running this code will fail if there is no Aiy network
// running locally on the default address: 127.0.0.1:9000

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let aiy = AiyClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("Aiy local network version: {}", aiy.api_version());

    // local Aiy network, like the above one but using the dedicated function
    let aiy_local = AiyClientBuilder::default().build_localnet().await?;
    println!("Aiy local network version: {}", aiy_local.api_version());

    // Aiy devnet -- https://fullnode.devnet.sui.io:443
    let aiy_devnet = AiyClientBuilder::default().build_devnet().await?;
    println!("Aiy devnet version: {}", aiy_devnet.api_version());

    // Aiy testnet -- https://fullnode.testnet.sui.io:443
    let aiy_testnet = AiyClientBuilder::default().build_testnet().await?;
    println!("Aiy testnet version: {}", aiy_testnet.api_version());

    // Aiy mainnet -- https://fullnode.mainnet.sui.io:443
    let aiy_mainnet = AiyClientBuilder::default().build_mainnet().await?;
    println!("Aiy mainnet version: {}", aiy_mainnet.api_version());

    println!("rpc methods: {:?}", aiy_testnet.available_rpc_methods());
    println!(
        "available subscriptions: {:?}",
        aiy_testnet.available_subscriptions()
    );

    Ok(())
}
