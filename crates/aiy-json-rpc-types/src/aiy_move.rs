// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use colored::Colorize;
use itertools::Itertools;
use move_binary_format::file_format::{Ability, AbilitySet, DatatypeTyParameter, Visibility};
use move_binary_format::normalized::{
    self, Enum as NormalizedEnum, Field as NormalizedField, Function as NormalizedFunction,
    Module as NormalizedModule, Struct as NormalizedStruct, Type as NormalizedType,
};
use move_command_line_common::error_bitset::ErrorBitset;
use move_core_types::annotated_value::{MoveStruct, MoveValue, MoveVariant};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::hash::Hash;
use aiy_macros::EnumVariantOrder;
use tracing::warn;

use aiy_types::base_types::{ObjectID, AiyAddress};
use aiy_types::execution_status::MoveLocation;
use aiy_types::aiy_serde::AiyStructTag;

pub type AiyMoveTypeParameterIndex = u16;

#[cfg(test)]
#[path = "unit_tests/aiy_move_tests.rs"]
mod aiy_move_tests;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum AiyMoveAbility {
    Copy,
    Drop,
    Store,
    Key,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct AiyMoveAbilitySet {
    pub abilities: Vec<AiyMoveAbility>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum AiyMoveVisibility {
    Private,
    Public,
    Friend,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AiyMoveStructTypeParameter {
    pub constraints: AiyMoveAbilitySet,
    pub is_phantom: bool,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct AiyMoveNormalizedField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: AiyMoveNormalizedType,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AiyMoveNormalizedStruct {
    pub abilities: AiyMoveAbilitySet,
    pub type_parameters: Vec<AiyMoveStructTypeParameter>,
    pub fields: Vec<AiyMoveNormalizedField>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AiyMoveNormalizedEnum {
    pub abilities: AiyMoveAbilitySet,
    pub type_parameters: Vec<AiyMoveStructTypeParameter>,
    pub variants: BTreeMap<String, Vec<AiyMoveNormalizedField>>,
    #[serde(default)]
    pub variant_declaration_order: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum AiyMoveNormalizedType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Address,
    Signer,
    Struct {
        #[serde(flatten)]
        inner: Box<AiyMoveNormalizedStructType>,
    },
    Vector(Box<AiyMoveNormalizedType>),
    TypeParameter(AiyMoveTypeParameterIndex),
    Reference(Box<AiyMoveNormalizedType>),
    MutableReference(Box<AiyMoveNormalizedType>),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AiyMoveNormalizedStructType {
    pub address: String,
    pub module: String,
    pub name: String,
    pub type_arguments: Vec<AiyMoveNormalizedType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AiyMoveNormalizedFunction {
    pub visibility: AiyMoveVisibility,
    pub is_entry: bool,
    pub type_parameters: Vec<AiyMoveAbilitySet>,
    pub parameters: Vec<AiyMoveNormalizedType>,
    pub return_: Vec<AiyMoveNormalizedType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct AiyMoveModuleId {
    address: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AiyMoveNormalizedModule {
    pub file_format_version: u32,
    pub address: String,
    pub name: String,
    pub friends: Vec<AiyMoveModuleId>,
    pub structs: BTreeMap<String, AiyMoveNormalizedStruct>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub enums: BTreeMap<String, AiyMoveNormalizedEnum>,
    pub exposed_functions: BTreeMap<String, AiyMoveNormalizedFunction>,
}

impl PartialEq for AiyMoveNormalizedModule {
    fn eq(&self, other: &Self) -> bool {
        self.file_format_version == other.file_format_version
            && self.address == other.address
            && self.name == other.name
    }
}

impl<S: std::hash::Hash + Eq + ToString> From<&NormalizedModule<S>> for AiyMoveNormalizedModule {
    fn from(module: &NormalizedModule<S>) -> Self {
        Self {
            file_format_version: module.file_format_version,
            address: module.address().to_hex_literal(),
            name: module.name().to_string(),
            friends: module
                .friends
                .iter()
                .map(|module_id| AiyMoveModuleId {
                    address: module_id.address.to_hex_literal(),
                    name: module_id.name.to_string(),
                })
                .collect::<Vec<AiyMoveModuleId>>(),
            structs: module
                .structs
                .iter()
                .map(|(name, struct_)| {
                    (name.to_string(), AiyMoveNormalizedStruct::from(&**struct_))
                })
                .collect::<BTreeMap<String, AiyMoveNormalizedStruct>>(),
            enums: module
                .enums
                .iter()
                .map(|(name, enum_)| (name.to_string(), AiyMoveNormalizedEnum::from(&**enum_)))
                .collect(),
            exposed_functions: module
                .functions
                .iter()
                .filter(|(_name, function)| {
                    function.is_entry || function.visibility != Visibility::Private
                })
                .map(|(name, function)| {
                    // TODO: Do we want to expose the private functions as well?

                    (
                        name.to_string(),
                        AiyMoveNormalizedFunction::from(&**function),
                    )
                })
                .collect::<BTreeMap<String, AiyMoveNormalizedFunction>>(),
        }
    }
}

impl<S: Hash + Eq + ToString> From<&NormalizedFunction<S>> for AiyMoveNormalizedFunction {
    fn from(function: &NormalizedFunction<S>) -> Self {
        Self {
            visibility: match function.visibility {
                Visibility::Private => AiyMoveVisibility::Private,
                Visibility::Public => AiyMoveVisibility::Public,
                Visibility::Friend => AiyMoveVisibility::Friend,
            },
            is_entry: function.is_entry,
            type_parameters: function
                .type_parameters
                .iter()
                .copied()
                .map(|a| a.into())
                .collect::<Vec<AiyMoveAbilitySet>>(),
            parameters: function
                .parameters
                .iter()
                .map(|t| AiyMoveNormalizedType::from(&**t))
                .collect::<Vec<AiyMoveNormalizedType>>(),
            return_: function
                .return_
                .iter()
                .map(|t| AiyMoveNormalizedType::from(&**t))
                .collect::<Vec<AiyMoveNormalizedType>>(),
        }
    }
}

impl<S: Hash + Eq + ToString> From<&NormalizedStruct<S>> for AiyMoveNormalizedStruct {
    fn from(struct_: &NormalizedStruct<S>) -> Self {
        Self {
            abilities: struct_.abilities.into(),
            type_parameters: struct_
                .type_parameters
                .iter()
                .copied()
                .map(AiyMoveStructTypeParameter::from)
                .collect::<Vec<AiyMoveStructTypeParameter>>(),
            fields: struct_
                .fields
                .0
                .values()
                .map(|f| AiyMoveNormalizedField::from(&**f))
                .collect::<Vec<AiyMoveNormalizedField>>(),
        }
    }
}

impl<S: Hash + Eq + ToString> From<&NormalizedEnum<S>> for AiyMoveNormalizedEnum {
    fn from(value: &NormalizedEnum<S>) -> Self {
        let variants = value
            .variants
            .values()
            .map(|variant| {
                (
                    variant.name.to_string(),
                    variant
                        .fields
                        .0
                        .values()
                        .map(|f| AiyMoveNormalizedField::from(&**f))
                        .collect::<Vec<AiyMoveNormalizedField>>(),
                )
            })
            .collect::<Vec<(String, Vec<AiyMoveNormalizedField>)>>();
        let variant_declaration_order = variants
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<String>>();
        let variants = variants.into_iter().collect();
        Self {
            abilities: value.abilities.into(),
            type_parameters: value
                .type_parameters
                .iter()
                .copied()
                .map(AiyMoveStructTypeParameter::from)
                .collect::<Vec<AiyMoveStructTypeParameter>>(),
            variants,
            variant_declaration_order: Some(variant_declaration_order),
        }
    }
}

impl From<DatatypeTyParameter> for AiyMoveStructTypeParameter {
    fn from(type_parameter: DatatypeTyParameter) -> Self {
        Self {
            constraints: type_parameter.constraints.into(),
            is_phantom: type_parameter.is_phantom,
        }
    }
}

impl<S: ToString> From<&NormalizedField<S>> for AiyMoveNormalizedField {
    fn from(normalized_field: &NormalizedField<S>) -> Self {
        Self {
            name: normalized_field.name.to_string(),
            type_: AiyMoveNormalizedType::from(&normalized_field.type_),
        }
    }
}

impl<S: ToString> From<&NormalizedType<S>> for AiyMoveNormalizedType {
    fn from(type_: &NormalizedType<S>) -> Self {
        match type_ {
            NormalizedType::Bool => AiyMoveNormalizedType::Bool,
            NormalizedType::U8 => AiyMoveNormalizedType::U8,
            NormalizedType::U16 => AiyMoveNormalizedType::U16,
            NormalizedType::U32 => AiyMoveNormalizedType::U32,
            NormalizedType::U64 => AiyMoveNormalizedType::U64,
            NormalizedType::U128 => AiyMoveNormalizedType::U128,
            NormalizedType::U256 => AiyMoveNormalizedType::U256,
            NormalizedType::Address => AiyMoveNormalizedType::Address,
            NormalizedType::Signer => AiyMoveNormalizedType::Signer,
            NormalizedType::Datatype(dt) => {
                let normalized::Datatype {
                    module,
                    name,
                    type_arguments,
                } = &**dt;
                AiyMoveNormalizedType::new_struct(
                    module.address.to_hex_literal(),
                    module.name.to_string(),
                    name.to_string(),
                    type_arguments
                        .iter()
                        .map(AiyMoveNormalizedType::from)
                        .collect::<Vec<AiyMoveNormalizedType>>(),
                )
            }
            NormalizedType::Vector(v) => {
                AiyMoveNormalizedType::Vector(Box::new(AiyMoveNormalizedType::from(&**v)))
            }
            NormalizedType::TypeParameter(t) => AiyMoveNormalizedType::TypeParameter(*t),
            NormalizedType::Reference(false, r) => {
                AiyMoveNormalizedType::Reference(Box::new(AiyMoveNormalizedType::from(&**r)))
            }
            NormalizedType::Reference(true, mr) => AiyMoveNormalizedType::MutableReference(
                Box::new(AiyMoveNormalizedType::from(&**mr)),
            ),
        }
    }
}

impl From<AbilitySet> for AiyMoveAbilitySet {
    fn from(set: AbilitySet) -> AiyMoveAbilitySet {
        Self {
            abilities: set
                .into_iter()
                .map(|a| match a {
                    Ability::Copy => AiyMoveAbility::Copy,
                    Ability::Drop => AiyMoveAbility::Drop,
                    Ability::Key => AiyMoveAbility::Key,
                    Ability::Store => AiyMoveAbility::Store,
                })
                .collect::<Vec<AiyMoveAbility>>(),
        }
    }
}

impl AiyMoveNormalizedType {
    pub fn new_struct(
        address: String,
        module: String,
        name: String,
        type_arguments: Vec<AiyMoveNormalizedType>,
    ) -> Self {
        AiyMoveNormalizedType::Struct {
            inner: Box::new(AiyMoveNormalizedStructType {
                address,
                module,
                name,
                type_arguments,
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum ObjectValueKind {
    ByImmutableReference,
    ByMutableReference,
    ByValue,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum MoveFunctionArgType {
    Pure,
    Object(ObjectValueKind),
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveValue")]
pub enum AiyMoveValue {
    // u64 and u128 are converted to String to avoid overflow
    Number(u32),
    Bool(bool),
    Address(AiyAddress),
    Vector(Vec<AiyMoveValue>),
    String(String),
    UID { id: ObjectID },
    Struct(AiyMoveStruct),
    Option(Box<Option<AiyMoveValue>>),
    Variant(AiyMoveVariant),
}

impl AiyMoveValue {
    /// Extract values from MoveValue without type information in json format
    pub fn to_json_value(self) -> Value {
        match self {
            AiyMoveValue::Struct(move_struct) => move_struct.to_json_value(),
            AiyMoveValue::Vector(values) => AiyMoveStruct::Runtime(values).to_json_value(),
            AiyMoveValue::Number(v) => json!(v),
            AiyMoveValue::Bool(v) => json!(v),
            AiyMoveValue::Address(v) => json!(v),
            AiyMoveValue::String(v) => json!(v),
            AiyMoveValue::UID { id } => json!({ "id": id }),
            AiyMoveValue::Option(v) => json!(v),
            AiyMoveValue::Variant(v) => v.to_json_value(),
        }
    }
}

impl Display for AiyMoveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            AiyMoveValue::Number(value) => write!(writer, "{}", value)?,
            AiyMoveValue::Bool(value) => write!(writer, "{}", value)?,
            AiyMoveValue::Address(value) => write!(writer, "{}", value)?,
            AiyMoveValue::String(value) => write!(writer, "{}", value)?,
            AiyMoveValue::UID { id } => write!(writer, "{id}")?,
            AiyMoveValue::Struct(value) => write!(writer, "{}", value)?,
            AiyMoveValue::Option(value) => write!(writer, "{:?}", value)?,
            AiyMoveValue::Vector(vec) => {
                write!(
                    writer,
                    "{}",
                    vec.iter().map(|value| format!("{value}")).join(",\n")
                )?;
            }
            AiyMoveValue::Variant(value) => write!(writer, "{}", value)?,
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl From<MoveValue> for AiyMoveValue {
    fn from(value: MoveValue) -> Self {
        match value {
            MoveValue::U8(value) => AiyMoveValue::Number(value.into()),
            MoveValue::U16(value) => AiyMoveValue::Number(value.into()),
            MoveValue::U32(value) => AiyMoveValue::Number(value),
            MoveValue::U64(value) => AiyMoveValue::String(format!("{value}")),
            MoveValue::U128(value) => AiyMoveValue::String(format!("{value}")),
            MoveValue::U256(value) => AiyMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => AiyMoveValue::Bool(value),
            MoveValue::Vector(values) => {
                AiyMoveValue::Vector(values.into_iter().map(|value| value.into()).collect())
            }
            MoveValue::Struct(value) => {
                // Best effort Aiy core type conversion
                let MoveStruct { type_, fields } = &value;
                if let Some(value) = try_convert_type(type_, fields) {
                    return value;
                }
                AiyMoveValue::Struct(value.into())
            }
            MoveValue::Signer(value) | MoveValue::Address(value) => {
                AiyMoveValue::Address(AiyAddress::from(ObjectID::from(value)))
            }
            MoveValue::Variant(MoveVariant {
                type_,
                variant_name,
                tag: _,
                fields,
            }) => AiyMoveValue::Variant(AiyMoveVariant {
                type_: type_.clone(),
                variant: variant_name.to_string(),
                fields: fields
                    .into_iter()
                    .map(|(id, value)| (id.into_string(), value.into()))
                    .collect::<BTreeMap<_, _>>(),
            }),
        }
    }
}

fn to_bytearray(value: &[MoveValue]) -> Option<Vec<u8>> {
    if value.iter().all(|value| matches!(value, MoveValue::U8(_))) {
        let bytearray = value
            .iter()
            .flat_map(|value| {
                if let MoveValue::U8(u8) = value {
                    Some(*u8)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Some(bytearray)
    } else {
        None
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(rename = "MoveVariant")]
pub struct AiyMoveVariant {
    #[schemars(with = "String")]
    #[serde(rename = "type")]
    #[serde_as(as = "AiyStructTag")]
    pub type_: StructTag,
    pub variant: String,
    pub fields: BTreeMap<String, AiyMoveValue>,
}

impl AiyMoveVariant {
    pub fn to_json_value(self) -> Value {
        // We only care about values here, assuming type information is known at the client side.
        let fields = self
            .fields
            .into_iter()
            .map(|(key, value)| (key, value.to_json_value()))
            .collect::<BTreeMap<_, _>>();
        json!({
            "variant": self.variant,
            "fields": fields,
        })
    }
}

impl Display for AiyMoveVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        let AiyMoveVariant {
            type_,
            variant,
            fields,
        } = self;
        writeln!(writer)?;
        writeln!(writer, "  {}: {type_}", "type".bold().bright_black())?;
        writeln!(writer, "  {}: {variant}", "variant".bold().bright_black())?;
        for (name, value) in fields {
            let value = format!("{}", value);
            let value = if value.starts_with('\n') {
                indent(&value, 2)
            } else {
                value
            };
            writeln!(writer, "  {}: {value}", name.bold().bright_black())?;
        }

        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, Eq, PartialEq, EnumVariantOrder)]
#[serde(untagged, rename = "MoveStruct")]
pub enum AiyMoveStruct {
    Runtime(Vec<AiyMoveValue>),
    WithTypes {
        #[schemars(with = "String")]
        #[serde(rename = "type")]
        #[serde_as(as = "AiyStructTag")]
        type_: StructTag,
        fields: BTreeMap<String, AiyMoveValue>,
    },
    WithFields(BTreeMap<String, AiyMoveValue>),
}

impl AiyMoveStruct {
    /// Extract values from MoveStruct without type information in json format
    pub fn to_json_value(self) -> Value {
        // Unwrap MoveStructs
        match self {
            AiyMoveStruct::Runtime(values) => {
                let values = values
                    .into_iter()
                    .map(|value| value.to_json_value())
                    .collect::<Vec<_>>();
                json!(values)
            }
            // We only care about values here, assuming struct type information is known at the client side.
            AiyMoveStruct::WithTypes { type_: _, fields } | AiyMoveStruct::WithFields(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| (key, value.to_json_value()))
                    .collect::<BTreeMap<_, _>>();
                json!(fields)
            }
        }
    }

    pub fn field_value(&self, field_name: &str) -> Option<AiyMoveValue> {
        match self {
            AiyMoveStruct::WithFields(fields) => fields.get(field_name).cloned(),
            AiyMoveStruct::WithTypes { type_: _, fields } => fields.get(field_name).cloned(),
            _ => None,
        }
    }
}

impl Display for AiyMoveStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            AiyMoveStruct::Runtime(_) => {}
            AiyMoveStruct::WithFields(fields) => {
                for (name, value) in fields {
                    writeln!(writer, "{}: {value}", name.bold().bright_black())?;
                }
            }
            AiyMoveStruct::WithTypes { type_, fields } => {
                writeln!(writer)?;
                writeln!(writer, "  {}: {type_}", "type".bold().bright_black())?;
                for (name, value) in fields {
                    let value = format!("{}", value);
                    let value = if value.starts_with('\n') {
                        indent(&value, 2)
                    } else {
                        value
                    };
                    writeln!(writer, "  {}: {value}", name.bold().bright_black())?;
                }
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AiyMoveAbort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<u64>,
}

impl AiyMoveAbort {
    pub fn new(move_location: MoveLocation, code: u64) -> Self {
        let module = move_location.module.to_canonical_string(true);
        let (error_code, line) = match ErrorBitset::from_u64(code) {
            Some(c) => (c.error_code().map(|c| c as u64), c.line_number()),
            None => (Some(code), None),
        };
        Self {
            module_id: Some(module),
            function: move_location.function_name.clone(),
            line,
            error_code,
        }
    }
}

impl Display for AiyMoveAbort {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        if let Some(module_id) = &self.module_id {
            writeln!(writer, "Module ID: {module_id}")?;
        }
        if let Some(function) = &self.function {
            writeln!(writer, "Function: {function}")?;
        }
        if let Some(line) = &self.line {
            writeln!(writer, "Line: {line}")?;
        }
        if let Some(error_code) = &self.error_code {
            writeln!(writer, "Error code: {error_code}")?;
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

fn indent<T: Display>(d: &T, indent: usize) -> String {
    d.to_string()
        .lines()
        .map(|line| format!("{:indent$}{}", "", line))
        .join("\n")
}

fn try_convert_type(type_: &StructTag, fields: &[(Identifier, MoveValue)]) -> Option<AiyMoveValue> {
    let struct_name = format!(
        "0x{}::{}::{}",
        type_.address.short_str_lossless(),
        type_.module,
        type_.name
    );
    let mut values = fields
        .iter()
        .map(|(id, value)| (id.to_string(), value))
        .collect::<BTreeMap<_, _>>();
    match struct_name.as_str() {
        "0x1::string::String" | "0x1::ascii::String" => {
            if let Some(MoveValue::Vector(bytes)) = values.remove("bytes") {
                return to_bytearray(bytes)
                    .and_then(|bytes| String::from_utf8(bytes).ok())
                    .map(AiyMoveValue::String);
            }
        }
        "0x2::url::Url" => {
            return values.remove("url").cloned().map(AiyMoveValue::from);
        }
        "0x2::object::ID" => {
            return values.remove("bytes").cloned().map(AiyMoveValue::from);
        }
        "0x2::object::UID" => {
            let id = values.remove("id").cloned().map(AiyMoveValue::from);
            if let Some(AiyMoveValue::Address(address)) = id {
                return Some(AiyMoveValue::UID {
                    id: ObjectID::from(address),
                });
            }
        }
        "0x2::balance::Balance" => {
            return values.remove("value").cloned().map(AiyMoveValue::from);
        }
        "0x1::option::Option" => {
            if let Some(MoveValue::Vector(values)) = values.remove("vec") {
                return Some(AiyMoveValue::Option(Box::new(
                    // in Move option is modeled as vec of 1 element
                    values.first().cloned().map(AiyMoveValue::from),
                )));
            }
        }
        _ => return None,
    }
    warn!(
        fields =? fields,
        "Failed to convert {struct_name} to AiyMoveValue"
    );
    None
}

impl From<MoveStruct> for AiyMoveStruct {
    fn from(move_struct: MoveStruct) -> Self {
        AiyMoveStruct::WithTypes {
            type_: move_struct.type_,
            fields: move_struct
                .fields
                .into_iter()
                .map(|(id, value)| (id.into_string(), value.into()))
                .collect(),
        }
    }
}

#[test]
fn enum_size() {
    assert_eq!(std::mem::size_of::<AiyMoveNormalizedType>(), 16);
}
