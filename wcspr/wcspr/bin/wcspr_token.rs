#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
    U512,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use wcspr::{self, WCSPR};

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl WCSPR<OnChainContractStorage> for Token {}

impl Token {
    fn constructor(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
        purse: URef,
    ) {
        WCSPR::init(
            self,
            name,
            symbol,
            decimals,
            Key::from(contract_hash),
            package_hash,
            purse,
        );
    }
}

#[no_mangle]
fn constructor() {
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let decimals: u8 = runtime::get_named_arg("decimals");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let purse: URef = runtime::get_named_arg("purse");
    Token::default().constructor(name, symbol, decimals, contract_hash, package_hash, purse);
}

/// This function is to transfer tokens against the address that user provided
///
/// # Parameters
///
/// * `recipient` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for transfer
///

#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret = Token::default().transfer(recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to transfer tokens against the address that has been approved before by owner
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the user
///
/// * `recipient` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for transfer
///
/// **Recommendation:**
///
/// The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`.
///
/// Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
///
/// should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
///
/// **Note:**  Teams who decide to wait for such a standard should make these
///
/// recommendations to app developers who work with their token contract.

#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret = Token::default().transfer_from(owner, recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to approve tokens against the address that user provided
///
/// # Parameters
///
/// * `spender` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for approve
///
/// **Recommendation:**
///
/// The exploit is mitigated through use of functions that increase/decrease the allowance relative to its current value, such as `increaseAllowance()` and `decreaseAllowance()`.
///
/// Pending community agreement on an ERC standard that would protect against this exploit, we recommend that developers of applications dependent on approve() / transferFrom()
///
/// should keep in mind that they have to set allowance to 0 first and verify if it was used before setting the new value.
///
/// **Note:**  Teams who decide to wait for such a standard should make these
///
/// recommendations to app developers who work with their token contract.

#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default().approve(spender, amount);
}

/// This function is to deposit token against the address that user provided
///
/// # Parameters
///
/// * `to` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for deposit
///

#[no_mangle]
fn deposit() {
    let amount: U512 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");
    let ret = Token::default().deposit(amount, purse);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to deposit token against the address that user provided (This function can be called from command line)
///
/// # Parameters
///
/// * `to` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for deposit
///
/// 
#[no_mangle]
fn deposit_no_return() {
    let amount: U512 = runtime::get_named_arg("amount");
    let purse: URef = runtime::get_named_arg("purse");
    let _ret = Token::default().deposit(amount, purse);
}

/// This function is to withdraw token against the address that user provided
///
/// # Parameters
///
/// * `from` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for withdraw
///

#[no_mangle]
fn withdraw() {
    let to: URef = runtime::get_named_arg("to_purse");
    let amount: U512 = runtime::get_named_arg("amount");
    let ret = Token::default().withdraw(to, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Balance of owner against the address that user provided
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the user against which user wants to get balance
///

#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Name of contract
///

#[no_mangle]
fn name() {
    let ret: String = Token::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Symbol of contract
///

#[no_mangle]
fn symbol() {
    let ret: String = Token::default().symbol();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Package Hash of contract
///

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = Token::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Purse of contract
///

#[no_mangle]
fn purse() {
    let ret: URef = Token::default().purse();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to return the Allowance of owner and spender that user provided
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the user
///
/// * `spender` - A Key that holds the account address of the user
///

#[no_mangle]
fn allowance() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let ret: U256 = Token::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to increase the amount of tokens approved for a spender by an owner
///
/// # Parameters
///
/// * `amount` - Number of tokens to increment approval of tokens by for spender
///
/// * `spender` - A Key that holds the account address of the user
///
#[no_mangle]
fn increase_allowance() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    let ret: Result<(), u32> = Token::default().increase_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// This function is to decrease the amount of tokens approved for a spender by an owner
///
/// # Parameters
///
/// * `amount` - Number of tokens to decrement approval of tokens by for spender
///
/// * `spender` - A Key that holds the account address of the user
///
#[no_mangle]
fn decrease_allowance() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    let ret: Result<(), u32> = Token::default().decrease_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("decimals", u8::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("purse", URef::cl_type()),
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
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
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
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
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
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "allowance",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deposit",
        vec![
            Parameter::new("amount", U512::cl_type()),
            Parameter::new("purse", URef::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deposit_no_return",
        vec![
            Parameter::new("amount", U512::cl_type()),
            Parameter::new("purse", URef::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![
            Parameter::new("to_purse", URef::cl_type()),
            Parameter::new("amount", U512::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
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
        "increase_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decrease_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
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

        let name: String = runtime::get_named_arg("name");
        let symbol: String = runtime::get_named_arg("symbol");
        let decimals: u8 = runtime::get_named_arg("decimals");
        let purse: URef = system::create_purse();

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
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
        runtime::put_key(&format!("{}_contract_purse", contract_name), purse.into());
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
