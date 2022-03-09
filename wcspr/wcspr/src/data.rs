use crate::alloc::string::ToString;
use alloc::string::String;
use casper_contract::contract_api::runtime;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ApiError, ContractPackageHash, Key, URef, U256, U512};
use contract_utils::{get_key, set_key, Dict};
//use casper_contract::{value::account::PurseId ,contract_api::{runtime,system}, unwrap_or_revert::UnwrapOrRevert};




// Events

pub enum WcsprEvents {
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },

    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },

    Deposit {
        src_purse: URef,
        amount: U512
    },

    Withdraw {
        recipient_purse: URef,
        amount: U512
    }
}

impl WcsprEvents {
    pub fn type_name(&self) -> String {
        match self {
            WcsprEvents::Approval{
                owner: _,
                spender: _,
                value: _
            } => "approve",

            WcsprEvents::Transfer {
                from: _,
                to: _,
                value: _,
            } => "erc20_transfer",

            WcsprEvents::Deposit {
                src_purse: _,
                amount: _
            } => "deposit",
            
            WcsprEvents::Withdraw {
                recipient_purse: _,
                amount: _
            } => "withdraw"
        }.to_string()
    }
}




pub const BALANCES_DICT: &str = "balances";
pub const ALLOWANCES_DICT: &str = "allowances";
pub const NAME: &str = "name";
pub const SYMBOL: &str = "symbol";
pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PURSE: &str = "self_purse";
pub const DECIMALS: &str = "decimals";
pub const CONTRACT_PACKAGE_HASH: &str = "contract_package_hash";
pub const TOTAL_SUPPLY: &str = "total_supply";


#[repr(u16)]
pub enum ErrorCodes {
    Abort = 35,
}

pub struct Balances {
    dict: Dict,
}

impl Balances {
    pub fn instance() -> Balances {
        Balances {
            dict: Dict::instance(BALANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(BALANCES_DICT)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value);
    }
}

pub struct Allowances {
    dict: Dict,
}

impl Allowances {
    pub fn instance() -> Allowances {
        Allowances {
            dict: Dict::instance(ALLOWANCES_DICT),
        }
    }

    pub fn init() {
        Dict::init(ALLOWANCES_DICT)
    }

    pub fn get(&self, owner: &Key, spender: &Key) -> U256 {
        self.dict.get_by_keys((owner, spender)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, spender: &Key, value: U256) {
        self.dict.set_by_keys((owner, spender), value);
    }
}

pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn symbol() -> String {
    get_key(SYMBOL).unwrap_or_revert()
}

pub fn set_symbol(symbol: String) {
    set_key(SYMBOL, symbol);
}

pub fn decimals() -> u8 {
    get_key(DECIMALS).unwrap_or_revert()
}

pub fn set_decimals(decimals: u8) {
    set_key(DECIMALS, decimals);
}

pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_self_purse(purse: URef) {
    runtime::put_key(&SELF_PURSE, purse.into());
}

pub fn get_self_purse() -> URef {
    let destination_purse_key = runtime::get_key(&SELF_PURSE).unwrap_or_revert();

    match destination_purse_key.as_uref() {
        Some(uref) => *uref,
        None => runtime::revert(ApiError::User(ErrorCodes::Abort as u16)),
    }
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(CONTRACT_PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(CONTRACT_PACKAGE_HASH).unwrap_or_revert()
}

pub fn set_totalsupply(value: U256) {
    set_key(TOTAL_SUPPLY, value);
}

pub fn get_totalsupply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_revert()
}