// options:
// printWidth: 40
// useModuleLabel: true
// autoGroupImports: module

module prettier::group_imports;

use a::b as c;
use a::b::e as f;
use a::{b::g as f, b::h as i};

use aiy::balance::{Self, Balance};
use aiy::coin::{Self, Coin};
use aiy::dynamic_field as df;
use aiy::dynamic_object_field as dof;
use aiy::event;
use aiy::aiy::AIY;
use aiy::transfer_policy::{Self, TransferPolicy, TransferRequest};

use std::{
    string::String,
    ascii::String as ASCII,
    vector as vec,
    option::{Self as opt, Option},
    type_name::get as type_name_get,
};

public fun do_something() {}
