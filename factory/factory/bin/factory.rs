#![no_main]
#![no_std]

extern crate alloc;

use alloc::prelude::v1::Box;
use alloc::{collections::BTreeSet, format, vec, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use factory::{self, FACTORY};

#[derive(Default)]
struct Factory(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Factory {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl FACTORY<OnChainContractStorage> for Factory {}

impl Factory {
    fn constructor(
        &mut self,
        fee_to_setter: Key,
        all_pairs: Vec<Key>,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        FACTORY::init(
            self,
            fee_to_setter,
            all_pairs,
            Key::from(contract_hash),
            package_hash,
        );
    }
}

#[no_mangle]
fn constructor() {
    let fee_to_setter: Key = runtime::get_named_arg("fee_to_setter");
    let all_pairs: Vec<Key> = runtime::get_named_arg("all_pairs");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Factory::default().constructor(fee_to_setter, all_pairs, contract_hash, package_hash);
}

/// This function is to return the fee to's hash
///

#[no_mangle]
fn fee_to() {
    let ret: Key = Factory::default().get_fee_to();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the fee to setter's hash
///

#[no_mangle]
fn fee_to_setter() {
    let ret: Key = Factory::default().get_fee_to_setter();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the all Pairs
///

#[no_mangle]
fn all_pairs() {
    let ret: Vec<Key> = Factory::default().get_all_pairs();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the total length of Pairs
///

#[no_mangle]
fn all_pairs_length() {
    let ret: Vec<Key> = Factory::default().get_all_pairs();
    runtime::ret(CLValue::from_t(U256::from(ret.len())).unwrap_or_revert());
}

/// This function is to set the fee to address which is only possible if the caller matched with fee to setter's hash
///
/// # Parameters
///
/// * `fee_to` - A Key that holds the Account Hash of fee_to_setter
///

#[no_mangle]
fn set_fee_to() {
    let fee_to: Key = runtime::get_named_arg("fee_to");
    Factory::default().set_fee_to(fee_to);
}

/// This function is to set the fee to setter address who is the only one to set fee to.
///
/// # Parameters
///
/// * `fee_to_setter` - A Key that holds the Account Hash of fee_to_setter
///

#[no_mangle]
fn set_fee_to_setter() {
    let fee_to_setter: Key = runtime::get_named_arg("fee_to_setter");
    Factory::default().set_fee_to_setter(fee_to_setter);
}

/// This function is to create pair of tokens provided by user agains the pair hash provided by user
///
/// # Parameters
///
/// * `token_a` - A Key that holds the Hash of token_a
///
/// * `token_b` - A Key that holds the Hash of token1_b
///
/// * `pair_hash` - A Key that holds the Hash of Pair Contract
///

#[no_mangle]
fn create_pair() {
    let token_a: Key = runtime::get_named_arg("token_a");
    let token_b: Key = runtime::get_named_arg("token_b");
    let pair_hash: Key = runtime::get_named_arg("pair_hash");
    Factory::default().create_pair(token_a, token_b, pair_hash);
}

/// This function is to return the the pair against tokens provided by user. If pair not found it will return hash-0000000000000000000000000000000000000000000000000000000000000000
///
/// # Parameters
///
/// * `token0` - A Key that holds the Hash of token0
///
/// * `token1` - A Key that holds the Hash of token1
///

#[no_mangle]
fn get_pair() {
    let token0: Key = runtime::get_named_arg("token0");
    let token1: Key = runtime::get_named_arg("token1");
    let ret: Key = Factory::default().get_pair(token0, token1);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to set the white list addresses which is only possible if the caller matched with owners's hash
///
/// # Parameters
///
/// * `white_list` - A Key that holds the Account Hash of fee_to_setter
///

#[no_mangle]
fn set_white_list() {
    let white_list: Key = runtime::get_named_arg("white_list");
    Factory::default().set_white_list(white_list, white_list);
}

/// This function is to fetch a Contract Package Hash
///

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = Factory::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("fee_to_setter", Key::cl_type()),
            Parameter::new("all_pairs", CLType::List(Box::new(Key::cl_type()))),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "create_pair",
        vec![
            Parameter::new("token_a", Key::cl_type()),
            Parameter::new("token_b", Key::cl_type()),
            Parameter::new("pair_hash", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "get_pair",
        vec![
            Parameter::new("token0", Key::cl_type()),
            Parameter::new("token1", Key::cl_type()),
        ],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "fee_to",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "fee_to_setter",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "all_pairs",
        vec![],
        CLType::List(Box::new(Key::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "all_pairs_length",
        vec![],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_fee_to",
        vec![Parameter::new("fee_to", Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_fee_to_setter",
        vec![Parameter::new("fee_to_setter", Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_white_list",
        vec![Parameter::new("white_list", Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "package_hash",
        vec![],
        ContractPackageHash::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {

    // Contract name must be same for all new versions of the contracts
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
    
    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {

        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        let fee_to_setter: Key = runtime::get_named_arg("fee_to_setter");
        let all_pairs: Vec<Key> = Vec::new();

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "fee_to_setter" => fee_to_setter,
            "all_pairs" => all_pairs,
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash
        };

        // Add the constructor group to the package hash with a single URef.
        let constructor_access: URef =
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert();

        // Call the constructor entry point
        let _: () =
            runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

        // Remove all URefs from the constructor group, so no one can call it for the second time.
        let mut urefs = BTreeSet::new();
        urefs.insert(constructor_access);
        storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
            .unwrap_or_revert();

        // Store contract in the account's named keys.
        runtime::put_key(
            &format!("{}_package_hash", contract_name),
            package_hash.into(),
        );
        runtime::put_key(
            &format!("{}_package_hash_wrapped", contract_name),
            storage::new_uref(package_hash).into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
        runtime::put_key(
            &format!("{}_package_access_token", contract_name),
            access_token.into(),
        );
    }
    else {          // this is a contract upgrade

        let package_hash: ContractPackageHash = runtime::get_key(&format!("{}_package_hash", contract_name))
                                                            .unwrap_or_revert()
                                                            .into_hash()
                                                            .unwrap()
                                                            .into();

        let (contract_hash, _): (ContractHash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}
