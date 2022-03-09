use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, ContractPackageHash, Key, RuntimeArgs, U128, U256,
};
use test_env::{Sender, TestContract, TestEnv};

pub struct PAIRInstance(TestContract);

impl PAIRInstance {
    pub fn instance(pair: TestContract) -> PAIRInstance {
        PAIRInstance(pair)
    }

    pub fn proxy(env: &TestEnv, pair: Key, sender: Sender) -> TestContract {
        TestContract::new(
            env,
            "pair-test.wasm",
            "proxy_test",
            sender,
            runtime_args! {
                "pair" => pair
            },
        )
    }
    pub fn proxy2(env: &TestEnv, pair: Key, sender: Sender) -> TestContract {
        TestContract::new(
            env,
            "pair-test2.wasm",
            "proxy_test2",
            sender,
            runtime_args! {
                "pair" => pair
            },
        )
    }

    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: Sender,
        name: &str,
        symbol: &str,
        decimals: u8,
        supply: U256,
        callee_contract_hash: Key,
        factory_hash: Key,
    ) -> TestContract {
        TestContract::new(
            env,
            "pair-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "initial_supply" => supply,
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals,
                "callee_contract_hash" => callee_contract_hash,
                "factory_hash" => factory_hash
            },
        )
    }

    pub fn constructor(
        &self,
        sender: Sender,
        name: &str,
        symbol: &str,
        decimals: u8,
        initial_supply: U256,
        callee_contract_hash: Key,
        factory_hash: Key,
    ) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "initial_supply" => initial_supply,
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals,
                "callee_contract_hash" => callee_contract_hash,
                "factory_hash" => factory_hash
            },
        );
    }

    pub fn balance_of<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("balances", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn nonce<T: Into<Key>>(&self, account: T) -> U256 {
        self.0
            .query_dictionary("nonces", key_to_str(&account.into()))
            .unwrap_or_default()
    }

    pub fn allowance<T: Into<Key>>(&self, owner: T, spender: T) -> U256 {
        let owner: Key = owner.into();
        let spender: Key = spender.into();
        self.0
            .query_dictionary("allowances", keys_to_str(&owner, &spender))
            .unwrap_or_default()
    }

    pub fn transfer<T: Into<Key>>(&self, sender: Sender, recipient: T, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient.into(),
                "amount" => amount
            },
        );
    }

    pub fn transfer_from(&self, sender: Sender, owner: Key, recipient: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }

    pub fn allowance_fn(&self, sender: Sender, owner: Key, spender: Key) {
        self.0.call_contract(
            sender,
            "allowance",
            runtime_args! {
                "owner" => owner,
                "spender" => spender,
            },
        );
    }
    // Factory Method
    pub fn set_fee_to<T: Into<Key>>(&self, sender: Sender, fee_to: T, factory_hash: Key) {
        self.0.call_contract(
            sender,
            "set_fee_to",
            runtime_args! {
                "fee_to" => fee_to.into(),
                "factory_hash" => factory_hash
            },
        );
    }
    pub fn mint_with_caller<T: Into<Key>>(&self, sender: Sender, caller: T, to: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "mint_with_caller",
            runtime_args! {
                "caller" => caller.into(),
                "to" => to,
                "amount" => amount
            },
        );
    }
    pub fn balance_with_caller<T: Into<Key>>(&self, sender: Sender, caller: T, owner: Key) {
        self.0.call_contract(
            sender,
            "balance_with_caller",
            runtime_args! {
                "caller" => caller.into(),
                "owner" => owner,
            },
        );
    }

    pub fn approve<T: Into<Key>>(&self, sender: Sender, spender: T, amount: U256) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender.into(),
                "amount" => amount
            },
        );
    }

    pub fn increase_allowance<T: Into<Key>>(&self, sender: Sender, spender: T, amount: U256) {
        self.0.call_contract(
            sender,
            "increase_allowance",
            runtime_args! {
                "spender" => spender.into(),
                "amount" => amount
            },
        );
    }

    pub fn decrease_allowance<T: Into<Key>>(&self, sender: Sender, spender: T, amount: U256) {
        self.0.call_contract(
            sender,
            "decrease_allowance",
            runtime_args! {
                "spender" => spender.into(),
                "amount" => amount
            },
        );
    }

    pub fn initialize<T: Into<Key>>(&self, sender: Sender, token0: T, token1: T, factory_hash: T) {
        self.0.call_contract(
            sender,
            "initialize",
            runtime_args! {
                "token0" => token0.into(),
                "token1" => token1.into(),
                "factory_hash" => factory_hash.into()
            },
        );
    }

    pub fn set_treasury_fee_percent(&self, sender: Sender, treasury_fee: U256) {
        self.0.call_contract(
            sender,
            "set_treasury_fee_percent",
            runtime_args! {
                "treasury_fee" => treasury_fee,
            },
        );
    }

    pub fn erc20_mint<T: Into<Key>>(&self, sender: Sender, to: T, amount: U256) {
        self.0.call_contract(
            sender,
            "erc20_mint",
            runtime_args! {
                "to" => to.into(),
                "amount" => amount
            },
        );
    }

    pub fn sync(&self, sender: Sender) {
        self.0.call_contract(sender, "sync", runtime_args! {});
    }

    pub fn skim<T: Into<Key>>(&self, sender: Sender, to: T) {
        self.0.call_contract(
            sender,
            "skim",
            runtime_args! {

                "to" => to.into(),
            },
        );
    }

    pub fn mint_no_ret<T: Into<Key>>(&self, sender: Sender, to: T) {
        self.0.call_contract(
            sender,
            "mint_no_ret",
            runtime_args! {
                "to" => to.into(),
            },
        );
    }

    pub fn burn_no_ret<T: Into<Key>>(&self, sender: Sender, to: T) {
        self.0.call_contract(
            sender,
            "burn_no_ret",
            runtime_args! {
                "to" => to.into(),
            },
        );
    }

    pub fn swap<T: Into<Key>>(
        &self,
        sender: Sender,
        amount0: U256,
        amount1: U256,
        to: T,
        data: &str,
    ) {
        self.0.call_contract(
            sender,
            "swap",
            runtime_args! {
                "amount0_out" => amount0,
                "amount1_out" => amount1,
                "to" => to.into(),
                "data" => data
            },
        );
    }

    pub fn name(&self) -> String {
        self.0.query_named_key(String::from("name"))
    }

    pub fn symbol(&self) -> String {
        self.0.query_named_key(String::from("symbol"))
    }

    pub fn decimals(&self) -> u8 {
        self.0.query_named_key(String::from("decimals"))
    }

    pub fn total_supply(&self) -> U256 {
        self.0.query_named_key(String::from("total_supply"))
    }

    pub fn token0(&self) -> Key {
        self.0.query_named_key(String::from("token0"))
    }

    pub fn token1(&self) -> Key {
        self.0.query_named_key(String::from("token1"))
    }
    pub fn balance(&self) -> U256 {
        self.0.query_named_key(String::from("balance"))
    }

    pub fn reserve0(&self) -> U128 {
        self.0.query_named_key(String::from("reserve0"))
    }

    pub fn reserve1(&self) -> U128 {
        self.0.query_named_key(String::from("reserve1"))
    }

    pub fn block_timestamp_last(&self) -> u64 {
        self.0.query_named_key(String::from("block_timestamp_last"))
    }

    pub fn price0_cumulative_last(&self) -> U256 {
        self.0
            .query_named_key(String::from("price0_cumulative_last"))
    }

    pub fn price1_cumulative_last(&self) -> U256 {
        self.0
            .query_named_key(String::from("price1_cumulative_last"))
    }

    pub fn k_last(&self) -> U256 {
        self.0.query_named_key(String::from("k_last"))
    }

    pub fn treasury_fee(&self) -> U256 {
        self.0.query_named_key(String::from("treasury_fee"))
    }

    pub fn minimum_liquidity(&self) -> U256 {
        self.0.query_named_key(String::from("minimum_liquidity"))
    }

    pub fn liquidity(&self) -> U256 {
        self.0.query_named_key(String::from("liquidity"))
    }

    pub fn amount0(&self) -> U256 {
        self.0.query_named_key(String::from("amount0"))
    }

    pub fn amount1(&self) -> U256 {
        self.0.query_named_key(String::from("amount1"))
    }

    pub fn callee_contract_hash(&self) -> Key {
        self.0.query_named_key(String::from("callee_contract_hash"))
    }

    pub fn factory_hash(&self) -> Key {
        self.0.query_named_key(String::from("factory_hash"))
    }

    pub fn get_fee_to(&self) -> Key {
        self.0.query_named_key(String::from("fee_to"))
    }

    pub fn self_contract_hash(&self) -> Key {
        self.0.query_named_key(String::from("self_contract_hash"))
    }
    pub fn self_package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key(String::from("self_package_hash"))
    }

    // Result methods
    pub fn transfer_result(&self) -> Result<(), u32> {
        self.0.query_named_key("transfer_result".to_string())
    }

    pub fn package_hash_result(&self) -> ContractPackageHash {
        self.0.query_named_key("package_hash".to_string())
    }

    pub fn transfer_from_result(&self) -> Result<(), u32> {
        self.0.query_named_key("transfer_from_result".to_string())
    }
    pub fn allowance_res(&self) -> U256 {
        self.0.query_named_key("allowance".to_string())
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn keys_to_str(key_a: &Key, key_b: &Key) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key_a.to_bytes().unwrap());
    hasher.update(key_b.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}
