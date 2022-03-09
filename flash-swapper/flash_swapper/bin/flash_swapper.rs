#![no_main]
#![no_std]

extern crate alloc;

use alloc::{collections::BTreeSet, format, string::String, vec};

use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use flash_swapper::{self, FLASHSWAPPER};

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl FLASHSWAPPER<OnChainContractStorage> for Token {}

impl Token {
    fn constructor(
        &mut self,
        wcspr: Key,
        dai: Key,
        uniswap_v2_factory: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
        purse: URef,
    ) {
        FLASHSWAPPER::init(
            self,
            wcspr,
            dai,
            uniswap_v2_factory,
            Key::from(contract_hash),
            package_hash,
            purse,
        );
    }
}

#[no_mangle]
fn constructor() {
    let wcspr: Key = runtime::get_named_arg("wcspr");
    let dai: Key = runtime::get_named_arg("dai");
    let uniswap_v2_factory: Key = runtime::get_named_arg("uniswap_v2_factory");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");

    let purse: URef = runtime::get_named_arg("purse");
    Token::default().constructor(
        wcspr,
        dai,
        uniswap_v2_factory,
        contract_hash,
        package_hash,
        purse,
    );
}

/// @notice Flash-borrows amount of token_borrow from a Uniswap V2 pair and repays using token_pay
/// @param token_borrow The address of the token you want to flash-borrow, use 0x0 for ETH
/// @param amount The amount of token_borrow you will borrow
/// @param token_pay The address of the token you want to use to payback the flash-borrow, use 0x0 for ETH
/// @param user_data Data that will be passed to the `execute` function for the user
/// @dev Depending on your use case, you may want to add access controls to this function

#[no_mangle]
fn start_swap() {
    let token_borrow: Key = runtime::get_named_arg("token_borrow");
    let amount: U256 = runtime::get_named_arg("amount");
    let token_pay: Key = runtime::get_named_arg("token_pay");
    let user_data: String = runtime::get_named_arg("user_data");
    Token::default().start_swap(token_borrow, amount, token_pay, user_data);
}

/// @notice Function is called by the Uniswap V2 pair's `swap` function

#[no_mangle]
fn uniswap_v2_call() {
    let sender: Key = runtime::get_named_arg("sender");
    let amount0: U256 = runtime::get_named_arg("amount0");
    let amount1: U256 = runtime::get_named_arg("amount1");
    let data = runtime::get_named_arg("data");
    Token::default().uniswap_v2_call(sender, amount0, amount1, data);
}

/// This function is to return the Purse of contract
///

#[no_mangle]
fn purse() {
    let ret: URef = Token::default().purse();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to fetch a Contract Package Hash
///

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = Token::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("wcspr", Key::cl_type()),
            Parameter::new("dai", Key::cl_type()),
            Parameter::new("uniswap_v2_factory", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("purse", URef::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "start_swap",
        vec![
            Parameter::new("token_borrow", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
            Parameter::new("token_pay", Key::cl_type()),
            Parameter::new("user_data", String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "uniswap_v2_call",
        vec![
            Parameter::new("sender", Key::cl_type()),
            Parameter::new("amount0", U256::cl_type()),
            Parameter::new("amount1", U256::cl_type()),
            Parameter::new("data", String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "purse",
        vec![],
        URef::cl_type(),
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

        let uniswap_v2_factory: Key = runtime::get_named_arg("uniswap_v2_factory");
        let wcspr: Key = runtime::get_named_arg("wcspr");
        let dai: Key = runtime::get_named_arg("dai");
        let purse: URef = system::create_purse();
        // Prepare constructor args
        let constructor_args = runtime_args! {
            "wcspr" => wcspr,
            "dai" => dai,
            "uniswap_v2_factory" => uniswap_v2_factory,
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash,
            "purse" => purse
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
