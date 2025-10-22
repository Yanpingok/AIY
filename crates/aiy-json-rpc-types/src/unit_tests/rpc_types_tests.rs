// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use anyhow::anyhow;
use move_core_types::annotated_value::{MoveStruct, MoveValue};
use move_core_types::ident_str;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use serde_json::json;

use aiy_types::base_types::{ObjectDigest, SequenceNumber};
use aiy_types::base_types::{ObjectID, AiyAddress};
use aiy_types::gas_coin::GasCoin;
use aiy_types::object::{MoveObject, Owner};
use aiy_types::{parse_aiy_struct_tag, MOVE_STDLIB_ADDRESS, AIY_FRAMEWORK_ADDRESS};

use crate::{ObjectChange, AiyMoveStruct, AiyMoveValue};

#[test]
fn test_move_value_to_aiy_coin() {
    let id = ObjectID::random();
    let value = 10000;
    let coin = GasCoin::new(id, value);

    let move_object = MoveObject::new_gas_coin(SequenceNumber::new(), id, value);
    let layout = GasCoin::layout();

    let move_struct = move_object.to_move_struct(&layout).unwrap();
    let aiy_struct = AiyMoveStruct::from(move_struct);
    let gas_coin = GasCoin::try_from(&aiy_struct).unwrap();
    assert_eq!(coin.value(), gas_coin.value());
    assert_eq!(coin.id(), gas_coin.id());
}

#[test]
fn test_move_value_to_string() {
    let test_string = "Some test string";
    let bytes = test_string.as_bytes();
    let values = bytes
        .iter()
        .map(|u8| MoveValue::U8(*u8))
        .collect::<Vec<_>>();

    let move_value = MoveValue::Struct(MoveStruct {
        type_: StructTag {
            address: MOVE_STDLIB_ADDRESS,
            module: ident_str!("string").to_owned(),
            name: ident_str!("String").to_owned(),
            type_params: vec![],
        },
        fields: vec![(ident_str!("bytes").to_owned(), MoveValue::Vector(values))],
    });

    let aiy_value = AiyMoveValue::from(move_value);

    assert!(matches!(aiy_value, AiyMoveValue::String(s) if s == test_string));
}

#[test]
fn test_option() {
    // bugfix for https://github.com/MystenLabs/sui/issues/4995
    let option = MoveValue::Struct(MoveStruct {
        type_: StructTag {
            address: MOVE_STDLIB_ADDRESS,
            module: Identifier::from_str("option").unwrap(),
            name: Identifier::from_str("Option").unwrap(),
            type_params: vec![TypeTag::U8],
        },
        fields: vec![(
            Identifier::from_str("vec").unwrap(),
            MoveValue::Vector(vec![MoveValue::U8(5)]),
        )],
    });
    let aiy_value = AiyMoveValue::from(option);
    assert!(matches!(
        aiy_value,
        AiyMoveValue::Option(value) if *value == Some(AiyMoveValue::Number(5))
    ));
}

#[test]
fn test_move_value_to_url() {
    let test_url = "http://testing.com";
    let bytes = test_url.as_bytes();
    let values = bytes
        .iter()
        .map(|u8| MoveValue::U8(*u8))
        .collect::<Vec<_>>();

    let string_move_value = MoveValue::Struct(MoveStruct {
        type_: StructTag {
            address: MOVE_STDLIB_ADDRESS,
            module: ident_str!("string").to_owned(),
            name: ident_str!("String").to_owned(),
            type_params: vec![],
        },
        fields: vec![(ident_str!("bytes").to_owned(), MoveValue::Vector(values))],
    });

    let url_move_value = MoveValue::Struct(MoveStruct {
        type_: StructTag {
            address: AIY_FRAMEWORK_ADDRESS,
            module: ident_str!("url").to_owned(),
            name: ident_str!("Url").to_owned(),
            type_params: vec![],
        },
        fields: vec![(ident_str!("url").to_owned(), string_move_value)],
    });

    let aiy_value = AiyMoveValue::from(url_move_value);

    assert!(matches!(aiy_value, AiyMoveValue::String(s) if s == test_url));
}

#[test]
fn test_serde() {
    let test_values = [
        AiyMoveValue::Number(u32::MAX),
        AiyMoveValue::UID {
            id: ObjectID::random(),
        },
        AiyMoveValue::String("some test string".to_string()),
        AiyMoveValue::Address(AiyAddress::random_for_testing_only()),
        AiyMoveValue::Bool(true),
        AiyMoveValue::Option(Box::new(None)),
        AiyMoveValue::Vector(vec![
            AiyMoveValue::Number(1000000),
            AiyMoveValue::Number(2000000),
            AiyMoveValue::Number(3000000),
        ]),
    ];

    for value in test_values {
        let json = serde_json::to_string(&value).unwrap();
        let serde_value: AiyMoveValue = serde_json::from_str(&json)
            .map_err(|e| anyhow!("Serde failed for [{:?}], Error msg : {}", value, e))
            .unwrap();
        assert_eq!(
            value, serde_value,
            "Error converting {:?} [{json}], got {:?}",
            value, serde_value
        )
    }
}

#[test]
fn test_move_type_serde() {
    use crate::aiy_move as SM;
    use crate::aiy_move::AiyMoveNormalizedType as SNT;
    let test_types = vec![
        SNT::Bool,
        SNT::U8,
        SNT::U16,
        SNT::U32,
        SNT::U64,
        SNT::U128,
        SNT::U256,
        SNT::Address,
        SNT::Signer,
        SNT::Vector(Box::new(SNT::U8)),
        SNT::Struct {
            inner: Box::new(SM::AiyMoveNormalizedStructType {
                address: AIY_FRAMEWORK_ADDRESS.to_string(),
                module: "coin".to_owned(),
                name: "Coin".to_owned(),
                type_arguments: vec![SNT::Address],
            }),
        },
        SNT::Vector(Box::new(SNT::U16)),
        SNT::Vector(Box::new(SNT::Vector(Box::new(SNT::U8)))),
        SNT::TypeParameter(0),
        SNT::Reference(Box::new(SNT::U8)),
        SNT::MutableReference(Box::new(SNT::Struct {
            inner: Box::new(SM::AiyMoveNormalizedStructType {
                address: AIY_FRAMEWORK_ADDRESS.to_string(),
                module: "coin".to_owned(),
                name: "Coin".to_owned(),
                type_arguments: vec![SNT::Address],
            }),
        })),
    ];

    let mut acc = vec![];

    for value in test_types {
        let json = serde_json::to_string(&value).unwrap();
        acc.push(json);
    }

    let s = SM::AiyMoveNormalizedStruct {
        abilities: SM::AiyMoveAbilitySet {
            abilities: vec![SM::AiyMoveAbility::Copy],
        },
        type_parameters: vec![SM::AiyMoveStructTypeParameter {
            constraints: SM::AiyMoveAbilitySet {
                abilities: vec![SM::AiyMoveAbility::Drop],
            },
            is_phantom: false,
        }],
        fields: vec![
            SM::AiyMoveNormalizedField {
                name: "field1".to_string(),
                type_: SNT::U8,
            },
            SM::AiyMoveNormalizedField {
                name: "field2".to_string(),
                type_: SNT::U16,
            },
        ],
    };

    let json = serde_json::to_string(&s).unwrap();
    acc.push(json);

    // NB: variants declaration and lexicographic ordering are different here
    let variants = vec![
        ("b", vec![SNT::U16]),
        ("a", vec![]),
        (
            "c",
            vec![
                SNT::U32,
                SNT::Struct {
                    inner: Box::new(SM::AiyMoveNormalizedStructType {
                        address: AIY_FRAMEWORK_ADDRESS.to_string(),
                        module: "coin".to_owned(),
                        name: "Coin".to_owned(),
                        type_arguments: vec![SNT::Address],
                    }),
                },
            ],
        ),
    ];
    let variant_declaration_order = variants
        .iter()
        .map(|(name, _)| name.to_string())
        .collect::<Vec<_>>();
    let variants = variants
        .into_iter()
        .map(|(name, type_)| {
            (
                name.to_string(),
                type_
                    .into_iter()
                    .enumerate()
                    .map(|(i, t)| SM::AiyMoveNormalizedField {
                        name: format!("field{}", i),
                        type_: t,
                    })
                    .collect(),
            )
        })
        .collect();

    let e = SM::AiyMoveNormalizedEnum {
        abilities: SM::AiyMoveAbilitySet {
            abilities: vec![SM::AiyMoveAbility::Copy],
        },
        type_parameters: vec![],
        variants,
        variant_declaration_order: Some(variant_declaration_order),
    };

    acc.push(serde_json::to_string(&e).unwrap());

    insta::assert_snapshot!(acc.join("\n"));
}

#[test]
fn test_serde_bytearray() {
    // ensure that we serialize byte arrays as number array
    let test_values = MoveValue::Vector(vec![MoveValue::U8(1), MoveValue::U8(2), MoveValue::U8(3)]);
    let aiy_move_value = AiyMoveValue::from(test_values);
    let json = serde_json::to_value(&aiy_move_value).unwrap();
    assert_eq!(json, json!([1, 2, 3]));
}

#[test]
fn test_serde_number() {
    // ensure that we serialize byte arrays as number array
    let test_values = MoveValue::U8(1);
    let aiy_move_value = AiyMoveValue::from(test_values);
    let json = serde_json::to_value(&aiy_move_value).unwrap();
    assert_eq!(json, json!(1));
    let test_values = MoveValue::U16(1);
    let aiy_move_value = AiyMoveValue::from(test_values);
    let json = serde_json::to_value(&aiy_move_value).unwrap();
    assert_eq!(json, json!(1));
    let test_values = MoveValue::U32(1);
    let aiy_move_value = AiyMoveValue::from(test_values);
    let json = serde_json::to_value(&aiy_move_value).unwrap();
    assert_eq!(json, json!(1));
}

#[test]
fn test_type_tag_struct_tag_devnet_inc_222() {
    let offending_tags = [
        "0x1::address::MyType",
        "0x1::vector::MyType",
        "0x1::address::MyType<0x1::address::OtherType>",
        "0x1::address::MyType<0x1::address::OtherType, 0x1::vector::VecTyper>",
        "0x1::address::address<0x1::vector::address, 0x1::vector::vector>",
    ];

    for tag in offending_tags {
        let oc = ObjectChange::Created {
            sender: Default::default(),
            owner: Owner::Immutable,
            object_type: parse_aiy_struct_tag(tag).unwrap(),
            object_id: ObjectID::random(),
            version: Default::default(),
            digest: ObjectDigest::random(),
        };

        let serde_json = serde_json::to_string(&oc).unwrap();
        let deser: ObjectChange = serde_json::from_str(&serde_json).unwrap();
        assert_eq!(oc, deser);
    }
}
