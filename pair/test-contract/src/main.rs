#![no_main]
#![no_std]

extern crate alloc;
use alloc::{collections::BTreeSet, format, vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{ContractHash, ContractPackageHash},
    runtime_args, ApiError, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Group, Key, Parameter, RuntimeArgs, URef, U256,
};

pub mod mappings;

#[no_mangle]
fn constructor() {
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let pair: Key = runtime::get_named_arg("pair");

    mappings::set_key(&mappings::self_hash_key(), contract_hash);
    mappings::set_key(&mappings::self_package_key(), package_hash);
    mappings::set_key(
        &mappings::pair_key(),
        ContractHash::from(pair.into_hash().unwrap_or_default()),
    );
}

#[no_mangle]
fn transfer() {
    let pair_address: ContractHash = mappings::get_key(&mappings::pair_key());

    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    let args: RuntimeArgs = runtime_args! {
        "recipient" => recipient,
        "amount" => amount,
    };

    let ret: Result<(), u32> = runtime::call_contract(pair_address, "transfer", args);
    mappings::set_key(&mappings::transfer_key(), ret);
}

#[no_mangle]
fn transfer_from() {
    let pair_address: ContractHash = mappings::get_key(&mappings::pair_key());

    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");

    let args: RuntimeArgs = runtime_args! {
        "owner" => owner,
        "recipient" => recipient,
        "amount" => amount,
    };

    let ret: Result<(), u32> = runtime::call_contract(pair_address, "transfer_from", args);
    mappings::set_key(&mappings::transfer_from_key(), ret);
}

#[no_mangle]
fn mint_with_caller() {
    let caller: Key = runtime::get_named_arg("caller");
    let to: Key = runtime::get_named_arg("to");
    let amount: U256 = runtime::get_named_arg("amount");
    let caller_hash_add_array = match caller {
        Key::Hash(package) => package,
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };

    let caller_hash_add = ContractHash::new(caller_hash_add_array);

    let _ret: () = runtime::call_contract(
        caller_hash_add,
        "mint",
        runtime_args! {"to" => to, "amount" => amount},
    );
}


#[no_mangle]
fn balance_with_caller() {
    let caller: Key = runtime::get_named_arg("caller");
    let owner: Key = runtime::get_named_arg("owner");
    let caller_hash_add_array = match caller {
        Key::Hash(package) => package,
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };

    let caller_hash_add = ContractHash::new(caller_hash_add_array);

    let balance: U256 = runtime::call_contract(
        caller_hash_add,
        "balance_of",
        runtime_args! {"owner" => owner},
    );
    mappings::set_key("balance",balance);
}

#[no_mangle]
fn set_fee_to() {
    let fee_to: Key = runtime::get_named_arg("fee_to");
    let factory_hash: Key = runtime::get_named_arg("factory_hash");
    // Test::default().set_fee_to(fee_to, factory_hash);
    let factory_hash_add_array = match factory_hash {
        Key::Hash(package) => package,
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    };
    let factory_hash_add = ContractHash::new(factory_hash_add_array);
    let _fee_to: () = runtime::call_contract(
        factory_hash_add,
        "set_fee_to",
        runtime_args! {"fee_to" => fee_to},
    );
}

#[no_mangle]
fn allowance() {
    let pair_address: ContractHash = mappings::get_key(&mappings::pair_key());
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let args: RuntimeArgs = runtime_args! {
        "owner" => owner,
        "spender" => spender,
    };

    let ret: U256 = runtime::call_contract(pair_address, "allowance", args);
    mappings::set_key(&mappings::allowance(), ret);
}

#[no_mangle]
fn increase_allowance() {
    let pair_address: ContractHash = mappings::get_key(&mappings::pair_key());

    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let _ret: Result<(), u32> = runtime::call_contract(pair_address, "increase_allowance", args);
}

#[no_mangle]
fn decrease_allowance() {
    let pair_address: ContractHash = mappings::get_key(&mappings::pair_key());

    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let _ret: Result<(), u32> = runtime::call_contract(pair_address, "decrease_allowance", args);
}

#[no_mangle]
fn approve() {
    let pair_address: ContractHash = mappings::get_key(&mappings::pair_key());
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let args: RuntimeArgs = runtime_args! {
        "spender" => spender,
        "amount" => amount,
    };

    let _ret: () = runtime::call_contract(pair_address, "approve", args);
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("pair", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_fee_to",
        vec![Parameter::new("fee_to", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint_with_caller",
        vec![
            Parameter::new("caller", Key::cl_type()),
            Parameter::new("to", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_with_caller",
        vec![
            Parameter::new("caller", Key::cl_type()),
            Parameter::new("owner", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    
    entry_points.add_entry_point(EntryPoint::new(
        "allowance",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decrease_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}

#[no_mangle]
fn call() {
    // Build new package with initial a first version of the contract.
    let (package_hash, access_token) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(package_hash, get_entry_points(), Default::default());
    let pair: Key = runtime::get_named_arg("pair");

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "contract_hash" => contract_hash,
        "package_hash" => package_hash,
        "pair" => pair
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
    let contract_name: alloc::string::String = runtime::get_named_arg("contract_name");
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
