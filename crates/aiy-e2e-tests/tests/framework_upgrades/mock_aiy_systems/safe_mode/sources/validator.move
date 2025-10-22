// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module aiy_system::validator {
    use std::ascii;

    use aiy::tx_context::TxContext;
    use std::string::{Self, String};
    use aiy::bag::{Self, Bag};
    use aiy::balance::{Self, Balance};
    use aiy::aiy::AIY;

    public struct ValidatorMetadata has store {
        aiy_address: address,
        protocol_pubkey_bytes: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
        net_address: String,
        p2p_address: String,
        primary_address: String,
        worker_address: String,
        extra_fields: Bag,
    }

    public struct Validator has store {
        metadata: ValidatorMetadata,
        voting_power: u64,
        stake: Balance<AIY>,
        extra_fields: Bag,
    }

    public(package) fun new(
        aiy_address: address,
        protocol_pubkey_bytes: vector<u8>,
        network_pubkey_bytes: vector<u8>,
        worker_pubkey_bytes: vector<u8>,
        net_address: vector<u8>,
        p2p_address: vector<u8>,
        primary_address: vector<u8>,
        worker_address: vector<u8>,
        init_stake: Balance<AIY>,
        ctx: &mut TxContext
    ): Validator {
        let metadata = ValidatorMetadata {
            aiy_address,
            protocol_pubkey_bytes,
            network_pubkey_bytes,
            worker_pubkey_bytes,
            net_address: string::from_ascii(ascii::string(net_address)),
            p2p_address: string::from_ascii(ascii::string(p2p_address)),
            primary_address: string::from_ascii(ascii::string(primary_address)),
            worker_address: string::from_ascii(ascii::string(worker_address)),
            extra_fields: bag::new(ctx),
        };

        Validator {
            metadata,
            voting_power: balance::value(&init_stake),
            stake: init_stake,
            extra_fields: bag::new(ctx),
        }
    }
}
