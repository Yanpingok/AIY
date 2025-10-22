// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod utils;
use crate::utils::request_tokens_from_faucet;
use anyhow::anyhow;
use fastcrypto::encoding::Encoding;
use fastcrypto::hash::HashFunction;
use fastcrypto::{
    ed25519::Ed25519KeyPair,
    encoding::Base64,
    secp256k1::Secp256k1KeyPair,
    secp256r1::Secp256r1KeyPair,
    traits::{EncodeDecodeBase64, KeyPair},
};
use rand::{rngs::StdRng, SeedableRng};
use shared_crypto::intent::{Intent, IntentMessage};
use aiy_sdk::{
    rpc_types::AiyTransactionBlockResponseOptions,
    types::{
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        transaction::TransactionData,
    },
    AiyClientBuilder,
};
use aiy_types::crypto::Signer;
use aiy_types::crypto::AiySignature;
use aiy_types::crypto::ToFromBytes;
use aiy_types::signature::GenericSignature;
use aiy_types::{
    base_types::AiyAddress,
    crypto::{get_key_pair_from_rng, AiyKeyPair},
};

/// This example walks through the Rust SDK use case described in
/// https://github.com/MystenLabs/sui/blob/main/docs/content/guides/developer/sui-101/sign-and-send-txn.mdx
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // set up aiy client for the desired network.
    let aiy_client = AiyClientBuilder::default().build_testnet().await?;

    // deterministically generate a keypair, testing only, do not use for mainnet,
    // use the next section to randomly generate a keypair instead.
    let skp_determ_0 =
        AiyKeyPair::Ed25519(Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32])));
    let _skp_determ_1 =
        AiyKeyPair::Secp256k1(Secp256k1KeyPair::generate(&mut StdRng::from_seed([0; 32])));
    let _skp_determ_2 =
        AiyKeyPair::Secp256r1(Secp256r1KeyPair::generate(&mut StdRng::from_seed([0; 32])));

    // randomly generate a keypair.
    let _skp_rand_0 = AiyKeyPair::Ed25519(get_key_pair_from_rng(&mut rand::rngs::OsRng).1);
    let _skp_rand_1 = AiyKeyPair::Secp256k1(get_key_pair_from_rng(&mut rand::rngs::OsRng).1);
    let _skp_rand_2 = AiyKeyPair::Secp256r1(get_key_pair_from_rng(&mut rand::rngs::OsRng).1);

    // import a keypair from a base64 encoded 32-byte `private key` assuming scheme is Ed25519.
    let _skp_import_no_flag_0 = AiyKeyPair::Ed25519(Ed25519KeyPair::from_bytes(
        &Base64::decode("1GPhHHkVlF6GrCty2IuBkM+tj/e0jn64ksJ1pc8KPoI=")
            .map_err(|_| anyhow!("Invalid base64"))?,
    )?);
    let _skp_import_no_flag_1 = AiyKeyPair::Ed25519(Ed25519KeyPair::from_bytes(
        &Base64::decode("1GPhHHkVlF6GrCty2IuBkM+tj/e0jn64ksJ1pc8KPoI=")
            .map_err(|_| anyhow!("Invalid base64"))?,
    )?);
    let _skp_import_no_flag_2 = AiyKeyPair::Ed25519(Ed25519KeyPair::from_bytes(
        &Base64::decode("1GPhHHkVlF6GrCty2IuBkM+tj/e0jn64ksJ1pc8KPoI=")
            .map_err(|_| anyhow!("Invalid base64"))?,
    )?);

    // import a keypair from a base64 encoded 33-byte `flag || private key`.
    // The signature scheme is determined by the flag.
    let _skp_import_with_flag_0 =
        AiyKeyPair::decode_base64("ANRj4Rx5FZRehqwrctiLgZDPrY/3tI5+uJLCdaXPCj6C")
            .map_err(|_| anyhow!("Invalid base64"))?;
    let _skp_import_with_flag_1 =
        AiyKeyPair::decode_base64("AdRj4Rx5FZRehqwrctiLgZDPrY/3tI5+uJLCdaXPCj6C")
            .map_err(|_| anyhow!("Invalid base64"))?;
    let _skp_import_with_flag_2 =
        AiyKeyPair::decode_base64("AtRj4Rx5FZRehqwrctiLgZDPrY/3tI5+uJLCdaXPCj6C")
            .map_err(|_| anyhow!("Invalid base64"))?;

    // import a keypair from a Bech32 encoded 33-byte `flag || private key`.
    // this is the format of a private key exported from Aiy Wallet or aiy.keystore.
    let _skp_import_with_flag_0 = AiyKeyPair::decode(
        "aiyprivkey1qzdlfxn2qa2lj5uprl8pyhexs02sg2wrhdy7qaq50cqgnffw4c2477kg9h3",
    )
    .map_err(|_| anyhow!("Invalid Bech32"))?;
    let _skp_import_with_flag_1 = AiyKeyPair::decode(
        "aiyprivkey1qqesr6xhua2dkt840v9yefely578q5ad90znnpmhhgpekfvwtxke6ef2xyg",
    )
    .map_err(|_| anyhow!("Invalid Bech32"))?;
    let _skp_import_with_flag_2 = AiyKeyPair::decode(
        "aiyprivkey1qprzkcs823gcrk7n4hy8pzhntdxakpqk32qwjg9f2wyc3myj78egvtw3ecr",
    )
    .map_err(|_| anyhow!("Invalid Bech32"))?;

    // replace `skp_determ_0` with the variable names above
    let pk = skp_determ_0.public();
    let sender = AiyAddress::from(&pk);
    println!("Sender: {:?}", sender);

    // make sure the sender has a gas coin as an example.
    request_tokens_from_faucet(sender, &aiy_client).await?;
    let gas_coin = aiy_client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?
        .data
        .into_iter()
        .next()
        .ok_or(anyhow!("No coins found for sender"))?;

    // construct an example programmable transaction.
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder.pay_aiy(vec![sender], vec![1])?;
        builder.finish()
    };

    let gas_budget = 5_000_000;
    let gas_price = aiy_client.read_api().get_reference_gas_price().await?;

    // create the transaction data that will be sent to the network.
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![gas_coin.object_ref()],
        pt,
        gas_budget,
        gas_price,
    );

    // derive the digest that the keypair should sign on,
    // i.e. the blake2b hash of `intent || tx_data`.
    let intent_msg = IntentMessage::new(Intent::aiy_transaction(), tx_data);
    let raw_tx = bcs::to_bytes(&intent_msg).expect("bcs should not fail");
    let mut hasher = aiy_types::crypto::DefaultHash::default();
    hasher.update(raw_tx.clone());
    let digest = hasher.finalize().digest;

    // use AiyKeyPair to sign the digest.
    let aiy_sig = skp_determ_0.sign(&digest);

    // if you would like to verify the signature locally before submission, use this function.
    // if it fails to verify locally, the transaction will fail to execute in Aiy.
    let res = aiy_sig.verify_secure(
        &intent_msg,
        sender,
        aiy_types::crypto::SignatureScheme::ED25519,
    );
    assert!(res.is_ok());

    // execute the transaction.
    let transaction_response = aiy_client
        .quorum_driver_api()
        .execute_transaction_block(
            aiy_types::transaction::Transaction::from_generic_sig_data(
                intent_msg.value,
                vec![GenericSignature::Signature(aiy_sig)],
            ),
            AiyTransactionBlockResponseOptions::default(),
            None,
        )
        .await?;

    println!(
        "Transaction executed. Transaction digest: {}",
        transaction_response.digest.base58_encode()
    );
    println!("{transaction_response}");
    Ok(())
}
