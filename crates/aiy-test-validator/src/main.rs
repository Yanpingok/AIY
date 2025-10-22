// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

fn main() {
    println!("aiy-test-validator binary has been deprecated in favor of aiy start, which is a more powerful command that allows you to start the local network with more options.

How to install/build the aiy binary IF:
    A: you only need the basic functionality, so just faucet and no persistence (no indexer, no GraphQL service), build from source as usual (cargo build --bin aiy) or download latest archive from release archives (starting from testnet v1.28.1 or devnet v1.29) and use aiy binary.
    B: you need to also start an indexer (--with-indexer ), or a GraphQL service (--with-graphql), you either:
    - download latest archive from release archives (starting from testnet v1.28.1 or devnet v1.29) and use aiy-pg binary (note that with v1.34.0 aiy-pg no longer exists in the release. Use `aiy` binary instead).
  OR
    - build from source. This requires to have libpq/postgresql dependencies installed (just as when using aiy-test-validator):
        - cargo build --bin aiy
        - cargo run --bin aiy -- start --with-faucet --force-regenesis --with-indexer --with-graphql

Running the local network:
 - (Preferred) In the simplest form, you can replace aiy-test-validator with aiy start --with-faucet --force-regenesis. This will create a network from a new genesis and start a faucet (127.0.0.1:9123). This will not persist state.
 - Use the drop-in replacement script: aiy/scripts/aiy-test-validator.sh and pass in all the flags/options as you used to.

Use aiy start --help to see all the flags and options, such as:
  * --with-indexer --> to start the indexer on the default host and port. Note that this requires \
a Postgres database to be running locally, or you need to set the different options to connect to a \
remote indexer database.
  * --with-graphql --> to start the GraphQL server on the default host and port");
}
