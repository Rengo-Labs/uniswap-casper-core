#![no_main]
#![no_std]

extern crate alloc;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String, vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use contract_utils::{ContractContext, OnChainContractStorage};
use erc20::{self, ERC20};

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl ERC20<OnChainContractStorage> for Token {}

impl Token {
    fn constructor(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: U256,
        domain_separator: String,
        permit_type_hash: String,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        ERC20::init(
            self,
            name,
            symbol,
            decimals,
            domain_separator,
            permit_type_hash,
            Key::from(contract_hash),
            package_hash,
        );
        let _ret = ERC20::mint(self, self.get_caller(), initial_supply);
    }
}

#[no_mangle]
fn constructor() {
    let name: String = runtime::get_named_arg("name");
    let symbol: String = runtime::get_named_arg("symbol");
    let decimals: u8 = runtime::get_named_arg("decimals");
    let initial_supply: U256 = runtime::get_named_arg("initial_supply");
    let domain_separator: String = runtime::get_named_arg("domain_separator");
    let permit_type_hash: String = runtime::get_named_arg("permit_type_hash");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    Token::default().constructor(
        name,
        symbol,
        decimals,
        initial_supply,
        domain_separator,
        permit_type_hash,
        contract_hash,
        package_hash,
    );
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

/// This function is to get meta transaction signer and verify if it is equal
/// to the signer public key or not then call approve.
///
/// # Parameters
///
/// * `public_key` - A string slice that holds the public key of the meta transaction signer,  Subscriber have to get it from running cryptoxide project externally.
///
/// * `signature` - A string slice that holds the signature of the meta transaction,  Subscriber have to get it from running cryptoxide project externally.
///
/// * `owner` - A Key that holds the account address of the owner
///
/// * `spender` - A Key that holds the account address of the spender
///  
/// * `value` - A U256 that holds the value
///  
/// * `deadeline` - A u64 that holds the deadline limit
///

#[no_mangle]
fn permit() {
    let public_key: String = runtime::get_named_arg("public");
    let signature: String = runtime::get_named_arg("signature");
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let value: U256 = runtime::get_named_arg("value");
    let deadline: u64 = runtime::get_named_arg("deadline");
    Token::default().permit(public_key, signature, owner, spender, value, deadline);
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

/// This function is to mint token against the address that user provided
///
/// # Parameters
///
/// * `to` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for mint
///

#[no_mangle]
fn mint() {
    let to: Key = runtime::get_named_arg("to");
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default().mint(to, amount);
}

/// This function is to burn token against the address that user provided
///
/// # Parameters
///
/// * `from` - A Key that holds the account address of the user
///
/// * `amount` - A U256 that holds the amount for burn
///

#[no_mangle]
fn burn() {
    let from: Key = runtime::get_named_arg("from");
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default().burn(from, amount);
}

/// This function is to return the Balance  of owner against the address that user provided
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

/// This function is to return the Nonce of owner against the address that user provided
///
/// # Parameters
///
/// * `owner` - A Key that holds the account address of the user against which user wants to get nonce
///

#[no_mangle]
fn nonce() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().nonce(owner);
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

/// This function is to return the Total Supply of the contract
///

#[no_mangle]
fn total_supply() {
    let ret: U256 = Token::default().total_supply();
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
            Parameter::new("name", String::cl_type()),
            Parameter::new("symbol", String::cl_type()),
            Parameter::new("decimals", u8::cl_type()),
            Parameter::new("initial_supply", U256::cl_type()),
            Parameter::new("domain_separator", String::cl_type()),
            Parameter::new("permit_type_hash", String::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
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
        "permit",
        vec![
            Parameter::new("public", String::cl_type()),
            Parameter::new("signature", String::cl_type()),
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("value", U256::cl_type()),
            Parameter::new("deadline", u64::cl_type()),
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

    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "nonce",
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
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "mint",
        vec![
            Parameter::new("to", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "burn",
        vec![
            Parameter::new("from", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
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
        "package_hash",
        vec![],
        ContractPackageHash::cl_type(),
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
        let initial_supply: U256 = runtime::get_named_arg("initial_supply");

        let (domain_separator, permit_type_hash) =
            Token::default().get_permit_type_and_domain_separator(&name, contract_hash);

        // Prepare constructor args
        let constructor_args = runtime_args! {
            "name" => name,
            "symbol" => symbol,
            "decimals" => decimals,
            "initial_supply" => initial_supply,
            "domain_separator" => domain_separator,
            "permit_type_hash" => permit_type_hash,
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
