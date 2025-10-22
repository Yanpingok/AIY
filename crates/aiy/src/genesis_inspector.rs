// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use inquire::Select;
use std::collections::BTreeMap;
use aiy_config::genesis::UnsignedGenesis;
use aiy_types::aiy_system_state::AiyValidatorGenesis;
use aiy_types::{
    base_types::ObjectID,
    coin::CoinMetadata,
    gas_coin::{GasCoin, MIST_PER_AIY, TOTAL_SUPPLY_MIST},
    governance::StakedAiy,
    move_package::MovePackage,
    object::{MoveObject, Owner},
};

const STR_ALL: &str = "All";
const STR_EXIT: &str = "Exit";
const STR_AIY: &str = "Aiy";
const STR_STAKED_AIY: &str = "StakedAiy";
const STR_PACKAGE: &str = "Package";
const STR_COIN_METADATA: &str = "CoinMetadata";
const STR_OTHER: &str = "Other";
const STR_AIY_DISTRIBUTION: &str = "Aiy Distribution";
const STR_OBJECTS: &str = "Objects";
const STR_VALIDATORS: &str = "Validators";

#[allow(clippy::or_fun_call)]
pub(crate) fn examine_genesis_checkpoint(genesis: UnsignedGenesis) {
    let system_object = genesis
        .aiy_system_object()
        .into_genesis_version_for_tooling();

    // Prepare Validator info
    let validator_set = &system_object.validators.active_validators;
    let validator_map = validator_set
        .iter()
        .map(|v| (v.verified_metadata().name.as_str(), v))
        .collect::<BTreeMap<_, _>>();
    let validator_pool_id_map = validator_set
        .iter()
        .map(|v| (v.staking_pool.id, v))
        .collect::<BTreeMap<_, _>>();

    let mut validator_options: Vec<_> = validator_map.keys().copied().collect();
    validator_options.extend_from_slice(&[STR_ALL, STR_EXIT]);
    println!("Total Number of Validators: {}", validator_set.len());

    // Prepare Aiy distribution info
    let mut aiy_distribution = BTreeMap::new();
    let entry = aiy_distribution
        .entry("Aiy System".to_string())
        .or_insert(BTreeMap::new());
    entry.insert(
        "Storage Fund".to_string(),
        (
            STR_AIY,
            system_object.storage_fund.non_refundable_balance.value(),
        ),
    );
    entry.insert(
        "Stake Subsidy".to_string(),
        (STR_AIY, system_object.stake_subsidy.balance.value()),
    );

    // Prepare Object Info
    let mut owner_map = BTreeMap::new();
    let mut package_map = BTreeMap::new();
    let mut aiy_map = BTreeMap::new();
    let mut staked_aiy_map = BTreeMap::new();
    let mut coin_metadata_map = BTreeMap::new();
    let mut other_object_map = BTreeMap::new();

    for object in genesis.objects() {
        let object_id = object.id();
        let object_id_str = object_id.to_string();
        assert_eq!(object.storage_rebate, 0);
        owner_map.insert(object.id(), object.owner.clone());

        match &object.data {
            aiy_types::object::Data::Move(move_object) => {
                if let Ok(gas) = GasCoin::try_from(object) {
                    let entry = aiy_distribution
                        .entry(object.owner.to_string())
                        .or_default();
                    entry.insert(object_id_str.clone(), (STR_AIY, gas.value()));
                    aiy_map.insert(object.id(), gas);
                } else if let Ok(coin_metadata) = CoinMetadata::try_from(object) {
                    coin_metadata_map.insert(object.id(), coin_metadata);
                } else if let Ok(staked_aiy) = StakedAiy::try_from(object) {
                    let entry = aiy_distribution
                        .entry(object.owner.to_string())
                        .or_default();
                    entry.insert(object_id_str, (STR_STAKED_AIY, staked_aiy.principal()));
                    // Assert pool id is associated with a knonw validator.
                    let validator = validator_pool_id_map.get(&staked_aiy.pool_id()).unwrap();
                    assert_eq!(validator.staking_pool.id, staked_aiy.pool_id());

                    staked_aiy_map.insert(object.id(), staked_aiy);
                } else {
                    other_object_map.insert(object.id(), move_object);
                }
            }
            aiy_types::object::Data::Package(p) => {
                package_map.insert(object.id(), p);
            }
        }
    }
    println!(
        "Total Number of Objects/Pacakges: {}",
        genesis.objects().len()
    );

    // Always check the Total Supply
    examine_total_supply(&aiy_distribution, false);

    // Main loop for inspection
    let main_options: Vec<&str> = vec![STR_AIY_DISTRIBUTION, STR_VALIDATORS, STR_OBJECTS, STR_EXIT];
    loop {
        let ans = Select::new(
            "Select one main category to examine ('Exit' to exit the program):",
            main_options.clone(),
        )
        .prompt();
        match ans {
            Ok(name) if name == STR_AIY_DISTRIBUTION => {
                examine_total_supply(&aiy_distribution, true)
            }
            Ok(name) if name == STR_VALIDATORS => {
                examine_validators(&validator_options, &validator_map);
            }
            Ok(name) if name == STR_OBJECTS => {
                println!("Examine Objects (total: {})", genesis.objects().len());
                examine_object(
                    &owner_map,
                    &validator_pool_id_map,
                    &package_map,
                    &aiy_map,
                    &staked_aiy_map,
                    &coin_metadata_map,
                    &other_object_map,
                );
            }
            Ok(name) if name == STR_EXIT => break,
            Ok(_) => (),
            Err(err) => {
                println!("Error: {err}");
                break;
            }
        }
    }
}

#[allow(clippy::ptr_arg)]
fn examine_validators(
    validator_options: &Vec<&str>,
    validator_map: &BTreeMap<&str, &AiyValidatorGenesis>,
) {
    loop {
        let ans = Select::new("Select one validator to examine ('All' to display all Validators, 'Exit' to return to Main):", validator_options.clone()).prompt();
        match ans {
            Ok(name) if name == STR_ALL => {
                for validator in validator_map.values() {
                    display_validator(validator);
                }
            }
            Ok(name) if name == STR_EXIT => break,
            Ok(name) => {
                let validator = validator_map.get(name).unwrap();
                display_validator(validator);
            }
            Err(err) => {
                println!("Error: {err}");
                break;
            }
        }
    }
    print_divider("Validator");
}

fn examine_object(
    owner_map: &BTreeMap<ObjectID, Owner>,
    validator_pool_id_map: &BTreeMap<ObjectID, &AiyValidatorGenesis>,
    package_map: &BTreeMap<ObjectID, &MovePackage>,
    aiy_map: &BTreeMap<ObjectID, GasCoin>,
    staked_aiy_map: &BTreeMap<ObjectID, StakedAiy>,
    coin_metadata_map: &BTreeMap<ObjectID, CoinMetadata>,
    other_object_map: &BTreeMap<ObjectID, &MoveObject>,
) {
    let object_options: Vec<&str> = vec![
        STR_AIY,
        STR_STAKED_AIY,
        STR_COIN_METADATA,
        STR_PACKAGE,
        STR_OTHER,
        STR_EXIT,
    ];
    loop {
        let ans = Select::new(
            "Select one object category to examine ('Exit' to return to Main):",
            object_options.clone(),
        )
        .prompt();
        match ans {
            Ok(name) if name == STR_EXIT => break,
            Ok(name) if name == STR_AIY => {
                for gas_coin in aiy_map.values() {
                    display_aiy(gas_coin, owner_map);
                }
                print_divider("Aiy");
            }
            Ok(name) if name == STR_STAKED_AIY => {
                for staked_aiy_coin in staked_aiy_map.values() {
                    display_staked_aiy(staked_aiy_coin, validator_pool_id_map, owner_map);
                }
                print_divider(STR_STAKED_AIY);
            }
            Ok(name) if name == STR_PACKAGE => {
                for package in package_map.values() {
                    println!("Package ID: {}", package.id());
                    println!("Version: {}", package.version());
                    println!("Modules: {:?}\n", package.serialized_module_map().keys());
                }
                print_divider("Package");
            }
            Ok(name) if name == STR_OTHER => {
                for other_obj in other_object_map.values() {
                    println!("{:#?}", other_obj.type_());
                    println!("{:?}", other_obj.version());
                    println!("Has Public Transfer: {}\n", other_obj.has_public_transfer());
                }
                print_divider("Other");
            }
            Ok(name) if name == STR_COIN_METADATA => {
                for coin_metadata in coin_metadata_map.values() {
                    println!("{:#?}\n", coin_metadata);
                }
                print_divider("CoinMetadata");
            }
            Ok(_) => (),
            Err(err) => {
                println!("Error: {err}");
                break;
            }
        }
    }
    print_divider("Object");
}

fn examine_total_supply(
    aiy_distribution: &BTreeMap<String, BTreeMap<String, (&str, u64)>>,
    print: bool,
) {
    let mut total_aiy = 0;
    let mut total_staked_aiy = 0;
    for (owner, coins) in aiy_distribution {
        let mut amount_sum = 0;
        for (owner, value) in coins.values() {
            amount_sum += value;
            if *owner == STR_STAKED_AIY {
                total_staked_aiy += value;
            }
        }
        total_aiy += amount_sum;
        if print {
            println!("Owner {:?}", owner);
            println!(
                "Total Amount of Aiy/StakedAiy Owned: {amount_sum} MIST or {} AIY:",
                amount_sum / MIST_PER_AIY
            );
            println!("{:#?}\n", coins);
        }
    }
    assert_eq!(total_aiy, TOTAL_SUPPLY_MIST);
    // Always print this.
    println!(
        "Total Supply of Aiy: {total_aiy} MIST or {} AIY",
        total_aiy / MIST_PER_AIY
    );
    println!(
        "Total Amount of StakedAiy: {total_staked_aiy} MIST or {} AIY\n",
        total_staked_aiy / MIST_PER_AIY
    );
    if print {
        print_divider("Aiy Distribution");
    }
}

fn display_validator(validator: &AiyValidatorGenesis) {
    let metadata = validator.verified_metadata();
    println!("Validator name: {}", metadata.name);
    println!("{:#?}", metadata);
    println!("Voting Power: {}", validator.voting_power);
    println!("Gas Price: {}", validator.gas_price);
    println!("Next Epoch Gas Price: {}", validator.next_epoch_gas_price);
    println!("Commission Rate: {}", validator.commission_rate);
    println!(
        "Next Epoch Commission Rate: {}",
        validator.next_epoch_commission_rate
    );
    println!("Next Epoch Stake: {}", validator.next_epoch_stake);
    println!("Staking Pool ID: {}", validator.staking_pool.id);
    println!(
        "Staking Pool Activation Epoch: {:?}",
        validator.staking_pool.activation_epoch
    );
    println!(
        "Staking Pool Deactivation Epoch: {:?}",
        validator.staking_pool.deactivation_epoch
    );
    println!(
        "Staking Pool Aiy Balance: {:?}",
        validator.staking_pool.aiy_balance
    );
    println!(
        "Rewards Pool: {}",
        validator.staking_pool.rewards_pool.value()
    );
    println!(
        "Pool Token Balance: {}",
        validator.staking_pool.pool_token_balance
    );
    println!(
        "Pending Delegation: {}",
        validator.staking_pool.pending_stake
    );
    println!(
        "Pending Total Aiy Withdraw: {}",
        validator.staking_pool.pending_total_aiy_withdraw
    );
    println!(
        "Pendign Pool Token Withdraw: {}",
        validator.staking_pool.pending_pool_token_withdraw
    );
    println!(
        "Exchange Rates ID: {}",
        validator.staking_pool.exchange_rates.id
    );
    println!(
        "Exchange Rates Size: {}",
        validator.staking_pool.exchange_rates.size
    );
    print_divider(&metadata.name);
}

fn display_aiy(gas_coin: &GasCoin, owner_map: &BTreeMap<ObjectID, Owner>) {
    println!("ID: {}", gas_coin.id());
    println!("Balance: {}", gas_coin.value());
    println!("Owner: {}\n", owner_map.get(gas_coin.id()).unwrap());
}

fn display_staked_aiy(
    staked_aiy: &StakedAiy,
    validator_pool_id_map: &BTreeMap<ObjectID, &AiyValidatorGenesis>,
    owner_map: &BTreeMap<ObjectID, Owner>,
) {
    let validator = validator_pool_id_map.get(&staked_aiy.pool_id()).unwrap();
    println!("{:#?}", staked_aiy);
    println!(
        "Staked to Validator: {}",
        validator.verified_metadata().name
    );
    println!("Owner: {}\n", owner_map.get(&staked_aiy.id()).unwrap());
}

fn print_divider(title: &str) {
    let title = format!("End of {title}");
    let divider_length = 80;
    let left_divider_length = 10;
    assert!(title.len() <= divider_length - left_divider_length * 2);
    let divider_op = "-";
    let divider = divider_op.repeat(divider_length);
    let left_divider = divider_op.repeat(left_divider_length);
    let margin_length = (divider_length - left_divider_length * 2 - title.len()) / 2;
    let margin = " ".repeat(margin_length);
    println!();
    println!("{divider}");
    println!("{left_divider}{margin}{title}{margin}{left_divider}");
    println!("{divider}");
    println!();
}
