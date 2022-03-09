use alloc::{format, string::String, vec::Vec};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;

use crate::data::{self, Allowances, Balances, Nonces};

use casper_contract::contract_api::runtime;
use casper_contract::contract_api::storage;

use crate::alloc::string::ToString;
use alloc::collections::BTreeMap;
use casper_types::system::mint::Error as MintError;
use casper_types::{
    runtime_args, ApiError, BlockTime, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef,
    U128, U256,
};
use contract_utils::{set_key, ContractContext, ContractStorage};
use cryptoxide::ed25519;
use renvm_sig::hash_message;
use renvm_sig::keccak256;

pub enum PAIREvent {
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
    Transfer {
        from: Key,
        to: Key,
        value: U256,
        pair: Key,
    },
    Mint {
        sender: Key,
        amount0: U256,
        amount1: U256,
        pair: Key,
    },
    Burn {
        sender: Key,
        amount0: U256,
        amount1: U256,
        to: Key,
        pair: Key,
    },
    Swap {
        sender: Key,
        amount0_in: U256,
        amount1_in: U256,
        amount0_out: U256,
        amount1_out: U256,
        to: Key,
        from: Key,
        pair: Key,
    },
    Sync {
        reserve0: U128,
        reserve1: U128,
        pair: Key,
    },
}

impl PAIREvent {
    pub fn type_name(&self) -> String {
        match self {
            PAIREvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            PAIREvent::Transfer {
                from: _,
                to: _,
                value: _,
                pair: _,
            } => "transfer",
            PAIREvent::Mint {
                sender: _,
                amount0: _,
                amount1: _,
                pair: _,
            } => "mint",
            PAIREvent::Burn {
                sender: _,
                amount0: _,
                amount1: _,
                to: _,
                pair: _,
            } => "burn",
            PAIREvent::Swap {
                sender: _,
                amount0_in: _,
                amount1_in: _,
                amount0_out: _,
                amount1_out: _,
                to: _,
                from: _,
                pair: _,
            } => "swap",
            PAIREvent::Sync {
                reserve0: _,
                reserve1: _,
                pair: _,
            } => "sync",
        }
        .to_string()
    }
}

/// Enum for FailureCode, It represents codes for different smart contract errors.
#[repr(u16)]
pub enum FailureCode {
    /// 65,536 for (UniswapV2: EXPIRED)
    Twelve = 12,
    /// 65,537 for (UniswapV2: FORBIDDEN)
    Thirteen,
    /// 65,538 for (signature verification failed)
    Fourteen,
    /// 65,539 for (UniswapV2: OVERFLOW)
    Fifteen,
    /// 65,540 for (UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT)
    Sixteen,
    /// 65,541 for (UniswapV2: INSUFFICIENT_LIQUIDITY)
    Seventeen,
    /// 65,542 for (UniswapV2: INVALID_TO)
    Eighteen,
    /// 65,543 for (UniswapV2: INSUFFICIENT_INPUT_AMOUNT)
    Ninteen,
    /// 65,544 for (UniswapV2: K)
    Twenty,
    /// 65,545 for (UniswapV2: INSUFFICIENT_LIQUIDITY_MINTED)
    TwentyOne,
    /// 65,546 for (UniswapV2: INSUFFICIENT_LIQUIDITY_BURNED)
    TwentyTwo,
    /// 65,547 for (UniswapV2: OVERFLOW)
    TwentyThree,
    /// 65,548 for (UniswapV2: UNDERFLOW)
    TwentyFour,
    /// 65,549 for (UniswapV2: DENOMINATOR IS ZERO)
    TwentyFive,
    /// 65,550 for (UniswapV2: LOCKED)
    TwentySix,
    /// 65,551 for (UniswapV2: UNDERFLOW)
    TwentySeven,
    /// 65,552 for (UniswapV2: OVERFLOW)
    TwentyEight,
}

pub trait PAIR<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        domain_separator: String,
        permit_type_hash: String,
        contract_hash: Key,
        factory_hash: Key,
        package_hash: ContractPackageHash,
        reserve0: U128,
        reserve1: U128,
        block_timestamp_last: u64,
        price0_cumulative_last: U256,
        price1_cumulative_last: U256,
        k_last: U256,
        treasury_fee: U256,
        minimum_liquidity: U256,
        callee_contract_hash: Key,
        lock: u64,
    ) {
        data::set_name(name);
        data::set_symbol(symbol);
        data::set_decimals(decimals);
        data::set_domain_separator(domain_separator);
        data::set_permit_type_hash(permit_type_hash);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_factory_hash(factory_hash);
        data::set_reserve0(reserve0);
        data::set_reserve1(reserve1);
        data::set_block_timestamp_last(block_timestamp_last);
        data::set_price0_cumulative_last(price0_cumulative_last);
        data::set_price1_cumulative_last(price1_cumulative_last);
        data::set_k_last(k_last);
        data::set_treasury_fee(treasury_fee);
        data::set_minimum_liquidity(minimum_liquidity);
        data::set_callee_contract_hash(callee_contract_hash);
        data::set_lock(lock);
        Nonces::init();
        let nonces = Nonces::instance();
        nonces.set(&Key::from(self.get_caller()), U256::from(0));
        Balances::init();
        Allowances::init();
    }

    fn balance_of(&mut self, owner: Key) -> U256 {
        Balances::instance().get(&owner)
    }

    fn nonce(&mut self, owner: Key) -> U256 {
        Nonces::instance().get(&owner)
    }

    fn transfer(&mut self, recipient: Key, amount: U256) -> Result<(), u32> {
        self.make_transfer(self.get_caller(), recipient, amount)
    }

    fn approve(&mut self, spender: Key, amount: U256) {
        self._approve(self.get_caller(), spender, amount);
    }

    fn _approve(&mut self, owner: Key, spender: Key, amount: U256) {
        Allowances::instance().set(&owner, &spender, amount);
        self.emit(&PAIREvent::Approval {
            owner: owner,
            spender: spender,
            value: amount,
        });
    }

    fn increase_allowance(&mut self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowances = Allowances::instance();
        let balances = Balances::instance();

        let owner: Key = self.get_caller();

        let spender_allowance: U256 = allowances.get(&owner, &spender);
        let owner_balance: U256 = balances.get(&owner);

        let new_allowance: U256 = spender_allowance
            .checked_add(amount)
            .ok_or(ApiError::User(FailureCode::Twelve as u16))
            .unwrap_or_revert();

        if new_allowance <= owner_balance && owner != spender {
            self._approve(owner, spender, new_allowance);
            return Ok(());
        } else {
            return Err(4);
        }
    }

    fn decrease_allowance(&mut self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowances = Allowances::instance();

        let owner: Key = self.get_caller();

        let spender_allowance: U256 = allowances.get(&owner, &spender);

        let new_allowance: U256 = spender_allowance
            .checked_sub(amount)
            .ok_or(ApiError::User(FailureCode::Thirteen as u16))
            .unwrap_or_revert();

        if new_allowance >= 0.into() && new_allowance < spender_allowance && owner != spender {
            self._approve(owner, spender, new_allowance);
            return Ok(());
        } else {
            return Err(4);
        }
    }

    fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256) -> Result<(), u32> {
        let ret: Result<(), u32> = self.make_transfer(owner, recipient, amount);
        if ret.is_ok() {
            let allowances = Allowances::instance();
            let spender_allowance: U256 = allowances.get(&owner, &self.get_caller());
            let new_allowance: U256 = spender_allowance
                .checked_sub(amount)
                .ok_or(ApiError::User(FailureCode::Thirteen as u16))
                .unwrap_or_revert();
            if new_allowance >= 0.into()
                && new_allowance < spender_allowance
                && owner != self.get_caller()
            {
                self._approve(owner, self.get_caller(), new_allowance);
                return Ok(());
            } else {
                return Err(4);
            }
        }
        ret
    }

    fn allowance(&mut self, owner: Key, spender: Key) -> U256 {
        Allowances::instance().get(&owner, &spender)
    }

    fn skim(&mut self, to: Key) {
        let lock = data::get_lock();
        if lock != 0 {
            //UniswapV2: Locked
            runtime::revert(ApiError::User(FailureCode::TwentySix as u16));
        }
        data::set_lock(1);
        let token0: Key = self.get_token0();
        let token1: Key = self.get_token1();
        let reserve0: U128 = data::get_reserve0();
        let reserve1: U128 = data::get_reserve1();
        let pair_address: Key = Key::from(data::get_package_hash());
        //convert Key to ContractHash
        let token0_hash_add_array = match token0 {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token0_contract_hash = ContractHash::new(token0_hash_add_array);
        //convert Key to ContractHash
        let token1_hash_add_array = match token1 {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token1_contract_hash = ContractHash::new(token1_hash_add_array);
        let balance0: U256 = runtime::call_contract(
            token0_contract_hash,
            "balance_of",
            runtime_args! {"owner" => pair_address},
        );

        let balance1: U256 = runtime::call_contract(
            token1_contract_hash,
            "balance_of",
            runtime_args! {"owner" => pair_address},
        );

        let balance0_conversion: U128 = U128::from(balance0.as_u128());
        let balance1_conversion: U128 = U128::from(balance1.as_u128());

        let _ret: Result<(), u32> = runtime::call_contract(
            token0_contract_hash,
            "transfer",
            runtime_args! {"recipient" => to,"amount" => U256::from((balance0_conversion - reserve0).as_u128())},
        );
        match _ret {
            Ok(()) => {
                let _ret: Result<(), u32> = runtime::call_contract(
                    token1_contract_hash,
                    "transfer",
                    runtime_args! {"recipient" => to,"amount" => U256::from((balance1_conversion - reserve1).as_u128()), },
                );
                match _ret {
                    Ok(()) => data::set_lock(0),
                    Err(e) => runtime::revert(e),
                }
            }
            Err(e) => runtime::revert(e),
        }
    }

    fn sync(&mut self) {
        let lock = data::get_lock();
        if lock != 0 {
            //UniswapV2: Locked
            runtime::revert(ApiError::User(FailureCode::TwentySix as u16));
        }
        data::set_lock(1);
        let token0: Key = self.get_token0();
        let token1: Key = self.get_token1();
        let reserve0: U128 = data::get_reserve0();
        let reserve1: U128 = data::get_reserve1();
        let pair_address: Key = Key::from(data::get_package_hash());
        //convert Key to ContractHash
        let token0_hash_add_array = match token0 {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token0_contract_hash = ContractHash::new(token0_hash_add_array);
        //convert Key to ContractHash
        let token1_hash_add_array = match token1 {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token1_contract_hash = ContractHash::new(token1_hash_add_array);
        let balance0: U256 = runtime::call_contract(
            token0_contract_hash,
            "balance_of",
            runtime_args! {"owner" => pair_address},
        );
        let balance1: U256 = runtime::call_contract(
            token1_contract_hash,
            "balance_of",
            runtime_args! {"owner" => pair_address},
        );
        self.update(balance0, balance1, reserve0, reserve1);
        data::set_lock(0);
    }

    fn swap(&mut self, amount0_out: U256, amount1_out: U256, to: Key, data: String) {
        let pair_address: Key = Key::from(data::get_package_hash());
        let zero: U256 = 0.into();
        if amount0_out > zero || amount1_out > zero {
            let (reserve0, reserve1, _block_timestamp_last) = self.get_reserves(); // gas savings
            if amount0_out < U256::from(reserve0.as_u128())
                && amount1_out < U256::from(reserve1.as_u128())
            {
                let token0: Key = self.get_token0();
                let token1: Key = self.get_token1();
                if to != token0 && to != token1 {
                    if amount0_out > zero {
                        //convert Key to ContractHash
                        // let token0_hash_add_array = match token0 {
                        //     Key::Hash(package) => package,
                        //     _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                        // };
                        // let token0_contract_hash = ContractHash::new(token0_hash_add_array);
                        let ret: Result<(), u32> = runtime::call_contract(
                            // token0_contract_hash,
                            token0.into_hash().unwrap_or_revert().into(),
                            "transfer",
                            runtime_args! {
                                "recipient" => to,
                                "amount" => amount0_out
                            }, // optimistically transfer tokens
                        );
                        match ret {
                            Ok(()) => {}
                            Err(e) => runtime::revert(e),
                        }
                    }
                    if amount1_out > zero {
                        //convert Key to ContractHash
                        let token1_hash_add_array = match token1 {
                            Key::Hash(package) => package,
                            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                        };
                        let token1_contract_hash = ContractHash::new(token1_hash_add_array);
                        let _ret: Result<(), u32> = runtime::call_contract(
                            token1_contract_hash,
                            "transfer",
                            runtime_args! {"recipient" => to,"amount" => amount1_out}, // optimistically transfer tokens
                        );
                        match _ret {
                            Ok(()) => {}
                            Err(e) => runtime::revert(e),
                        }
                    }
                    if data.len() > 0 {
                        let uniswap_v2_callee_address: Key = to;
                        //convert Key to ContractHash
                        let uniswap_v2_callee_address_hash_add_array =
                            match uniswap_v2_callee_address {
                                Key::Hash(package) => package,
                                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                            };
                        let uniswap_v2_callee_contract_hash =
                            ContractHash::new(uniswap_v2_callee_address_hash_add_array);

                        let _result: () = runtime::call_contract(
                            uniswap_v2_callee_contract_hash,
                            "uniswap_v2_call",
                            runtime_args! {"sender" => data::get_callee_contract_hash(),"amount0" => amount0_out,"amount1" => amount1_out,"data" => data},
                        );
                    }
                    //convert Key to ContractHash
                    let token0_hash_add_array = match token0 {
                        Key::Hash(package) => package,
                        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                    };
                    let token0_contract_hash = ContractHash::new(token0_hash_add_array);
                    //convert Key to ContractHash
                    let token1_hash_add_array = match token1 {
                        Key::Hash(package) => package,
                        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                    };
                    let token1_contract_hash = ContractHash::new(token1_hash_add_array);
                    let balance0: U256 = runtime::call_contract(
                        token0_contract_hash,
                        "balance_of",
                        runtime_args! {"owner" => pair_address},
                    );
                    let balance1: U256 = runtime::call_contract(
                        token1_contract_hash,
                        "balance_of",
                        runtime_args! {"owner" => pair_address},
                    );
                    let mut amount0_in: U256 = 0.into();
                    let mut amount1_in: U256 = 0.into();

                    if balance0 > (U256::from(reserve0.as_u128()) - amount0_out) {
                        amount0_in = balance0 - (U256::from(reserve0.as_u128()) - amount0_out);
                    }
                    if balance1 > (U256::from(reserve1.as_u128()) - amount1_out) {
                        amount1_in = balance1 - (U256::from(reserve1.as_u128()) - amount1_out);
                    }
                    if amount0_in > zero || amount1_in > zero {
                        let amount_1000: U256 = 1000.into();
                        let amount_3: U256 = 3.into();
                        let balance0_adjusted: U256 =
                            (balance0 * amount_1000) - (amount0_in * amount_3);
                        let balance1_adjusted: U256 =
                            (balance1 * amount_1000) - (amount1_in * amount_3);
                        let reserve0_conversion: U256 = U256::from(reserve0.as_u128());
                        let reserve1_conversion: U256 = U256::from(reserve1.as_u128());
                        let reserve_multiply: U256 = (1000 ^ 2).into();
                        if (balance0_adjusted * balance1_adjusted)
                            >= (reserve0_conversion * reserve1_conversion * reserve_multiply)
                        {
                            self.update(balance0, balance1, reserve0, reserve1);
                            let eventpair: Key = Key::from(data::get_hash());
                            self.emit(&PAIREvent::Swap {
                                sender: self.get_caller(),
                                amount0_in: amount0_in,
                                amount1_in: amount1_in,
                                amount0_out: amount0_out,
                                amount1_out: amount1_out,
                                to: to,
                                from: self.get_caller(),
                                pair: eventpair,
                            });
                        } else {
                            //UniswapV2: K
                            runtime::revert(ApiError::User(FailureCode::Twenty as u16));
                        }
                    } else {
                        //UniswapV2: INSUFFICIENT_INPUT_AMOUNT
                        runtime::revert(ApiError::User(FailureCode::Ninteen as u16));
                    }
                } else {
                    //UniswapV2: INVALID_TO
                    runtime::revert(ApiError::User(FailureCode::Eighteen as u16));
                }
            } else {
                //UniswapV2: INSUFFICIENT_LIQUIDITY
                runtime::revert(ApiError::User(FailureCode::Seventeen as u16));
            }
        } else {
            //UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT
            runtime::revert(ApiError::User(FailureCode::Sixteen as u16));
        }
    }

    /// This function is to get signer and verify if it is equal
    /// to the signer public key or not.
    ///
    /// # Parameters
    ///
    /// * `public_key` - A string slice that holds the public key of the meta transaction signer
    ///
    /// * `signature` - A string slice that holds the signature of the meta transaction
    ///
    /// * `digest` - A u8 array that holds the digest
    ///
    /// * `owner` - An Accounthash that holds the account address of the signer
    ///

    fn ecrecover(
        &mut self,
        public_key: String,
        signature: String,
        digest: [u8; 32],
        owner: Key,
    ) -> bool {
        let public_key_without_spaces: String = public_key.split_whitespace().collect();
        let public_key_string: Vec<&str> = public_key_without_spaces.split(',').collect();
        let mut public_key_vec: Vec<u8> = Vec::new();
        let mut public_counter: usize = 0;
        while public_counter < 32 {
            public_key_vec.push(public_key_string[public_counter].parse::<u8>().unwrap());
            public_counter = public_counter + 1;
        }
        let signature_without_spaces: String = signature.split_whitespace().collect();
        let signature_string: Vec<&str> = signature_without_spaces.split(',').collect();
        let mut signature_vec: Vec<u8> = Vec::new();
        let mut signature_counter: usize = 0;
        while signature_counter < 64 {
            signature_vec.push(signature_string[signature_counter].parse::<u8>().unwrap());
            signature_counter = signature_counter + 1;
        }
        let result: bool = ed25519::verify(&digest, &public_key_vec, &signature_vec);
        let verify_key: String = format!("{}{}", "VERIFY", owner);
        set_key(&verify_key, result);
        return result;
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

    fn permit(
        &mut self,
        public_key: String,
        signature: String,
        owner: Key,
        spender: Key,
        value: U256,
        deadline: u64,
    ) {
        let domain_separator: String = data::get_domain_separator();
        let permit_type_hash: String = data::get_permit_type_hash();
        let nonce: U256 = self.nonce(Key::from(self.get_caller()));
        let deadline_into_blocktime: BlockTime = BlockTime::new(deadline * 1000);
        let blocktime: BlockTime = runtime::get_blocktime();
        if deadline_into_blocktime >= blocktime {
            let data: String = format!(
                "{}{}{}{}{}{}",
                permit_type_hash, owner, spender, value, nonce, deadline
            );
            let hash: [u8; 32] = keccak256(data.as_bytes());
            let hash_string: String = hex::encode(hash);
            let encode_packed: String = format!("{}{}", domain_separator, hash_string);
            let digest: [u8; 32] = hash_message(encode_packed);
            let digest_string: String = hex::encode(digest);
            let digest_key: String = format!("{}{}", "digest_", owner);
            set_key(&digest_key, digest_string);
            self.set_nonce(Key::from(self.get_caller()));
            let result: bool =
                self.ecrecover(public_key, signature, digest, Key::from(self.get_caller()));
            if result == true {
                Allowances::instance().set(&owner, &spender, value);
                self.emit(&PAIREvent::Approval {
                    owner: owner,
                    spender: spender,
                    value: value,
                });
            } else {
                //signature verification failed
                runtime::revert(ApiError::User(FailureCode::Fourteen as u16));
            }
        } else {
            //deadline is equal to or greater than blocktime
            runtime::revert(ApiError::User(FailureCode::Twelve as u16));
        }
    }

    fn mint(&mut self, recipient: Key, amount: U256) {
        let balances = Balances::instance();
        let balance = balances.get(&recipient);
        balances.set(
            &recipient,
            balance
                .checked_add(amount)
                .ok_or(ApiError::User(FailureCode::TwentyThree as u16))
                .unwrap_or_revert(),
        );
        data::set_total_supply(
            self.total_supply()
                .checked_add(amount)
                .ok_or(ApiError::User(FailureCode::TwentyThree as u16))
                .unwrap_or_revert(),
        );
        let address_0: Key = Key::from_formatted_str(
            "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();
        let eventpair: Key = Key::from(data::get_hash());
        self.emit(&PAIREvent::Transfer {
            from: address_0,
            to: recipient,
            value: amount,
            pair: eventpair,
        });
    }

    fn burn(&mut self, recipient: Key, amount: U256) {
        let balances = Balances::instance();
        let balance = balances.get(&recipient);
        if balance >= amount {
            balances.set(
                &recipient,
                balance
                    .checked_sub(amount)
                    .ok_or(ApiError::User(FailureCode::TwentyFour as u16))
                    .unwrap_or_revert(),
            );
            data::set_total_supply(
                self.total_supply()
                    .checked_sub(amount)
                    .ok_or(ApiError::User(FailureCode::TwentyFour as u16))
                    .unwrap_or_revert(),
            );
            let address_0: Key = Key::from_formatted_str(
                "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap();
            let eventpair: Key = Key::from(data::get_hash());
            self.emit(&PAIREvent::Transfer {
                from: recipient,
                to: address_0,
                value: amount,
                pair: eventpair,
            });
        } else {
            runtime::revert(MintError::InsufficientFunds)
        }
    }

    fn set_nonce(&mut self, recipient: Key) {
        let nonces = Nonces::instance();
        let nonce = nonces.get(&recipient);
        nonces.set(&recipient, nonce + U256::from(1));
    }

    fn make_transfer(&mut self, sender: Key, recipient: Key, amount: U256) -> Result<(), u32> {
        if sender == recipient {
            return Err(4); // Same sender recipient error
        }

        if amount.is_zero() {
            return Err(5); // Amount to transfer is 0
        }

        let balances: Balances = Balances::instance();
        let sender_balance: U256 = balances.get(&sender);
        let recipient_balance: U256 = balances.get(&recipient);
        balances.set(
            &sender,
            sender_balance
                .checked_sub(amount)
                .ok_or(ApiError::User(FailureCode::TwentyFour as u16))
                .unwrap_or_revert(),
        );
        balances.set(
            &recipient,
            recipient_balance
                .checked_add(amount)
                .ok_or(ApiError::User(FailureCode::TwentyThree as u16))
                .unwrap_or_revert(),
        );
        let eventpair: Key = Key::from(data::get_hash());
        self.emit(&PAIREvent::Transfer {
            from: sender,
            to: recipient,
            value: amount,
            pair: eventpair,
        });

        Ok(())
    }

    fn set_treasury_fee_percent(&mut self, treasury_fee: U256) {
        if treasury_fee < 30.into() && treasury_fee > 3.into() {
            data::set_treasury_fee(treasury_fee);
        } else if treasury_fee >= 30.into() {
            data::set_treasury_fee(30.into());
        } else {
            data::set_treasury_fee(3.into());
        }
    }

    fn set_reserve0(&mut self, reserve0: U128) {
        data::set_reserve0(reserve0);
    }

    fn set_reserve1(&mut self, reserve1: U128) {
        data::set_reserve1(reserve1);
    }

    fn total_supply(&mut self) -> U256 {
        data::total_supply()
    }

    fn get_treasury_fee(&mut self) -> U256 {
        data::get_treasury_fee()
    }

    fn get_minimum_liquidity(&mut self) -> U256 {
        data::get_minimum_liquidity()
    }

    fn get_token0(&mut self) -> Key {
        data::get_token0()
    }

    fn get_token1(&mut self) -> Key {
        data::get_token1()
    }

    fn get_factory_hash(&mut self) -> Key {
        data::get_factory_hash()
    }

    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }

    fn mint_helper(&mut self, to: Key) -> U256 {
        let (reserve0, reserve1, _block_timestamp_last) = self.get_reserves(); // gas savings
        let token0: Key = data::get_token0();
        let token1: Key = data::get_token1();
        let pair_contract_hash1: Key = Key::from(data::get_package_hash());
        let pair_contract_hash2: Key = Key::from(data::get_package_hash());
        let token0_hash_add_array = match token0 {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token0_hash_add = ContractHash::new(token0_hash_add_array);
        let token1_hash_add_array = match token1 {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token1_hash_add = ContractHash::new(token1_hash_add_array);
        let balance0: U256 = runtime::call_contract(
            token0_hash_add,
            "balance_of",
            runtime_args! {"owner" => pair_contract_hash1},
        );
        let balance1: U256 = runtime::call_contract(
            token1_hash_add,
            "balance_of",
            runtime_args! {"owner" => pair_contract_hash2},
        );
        let amount0: U256 = balance0
            .checked_sub(U256::from(reserve0.as_u128()))
            .ok_or(ApiError::User(FailureCode::TwentySeven as u16))
            .unwrap_or_revert();
        let amount1: U256 = balance1
            .checked_sub(U256::from(reserve1.as_u128()))
            .ok_or(ApiError::User(FailureCode::TwentySeven as u16))
            .unwrap_or_revert();
        let fee_on: bool = self.mint_fee(reserve0, reserve1);
        let total_supply: U256 = self.total_supply(); // gas savings, must be defined here since totalSupply can update in mint_fee
        let minimum_liquidity: U256 = data::get_minimum_liquidity();
        let mut liquidity: U256 = 0.into();
        if total_supply == 0.into() {
            liquidity = self.sqrt(amount0 * amount1).checked_sub(U256::from(minimum_liquidity.as_u128()))
            .ok_or(ApiError::User(FailureCode::TwentyEight as u16))
            .unwrap_or_revert();
            self.mint(
                Key::from_formatted_str(
                    "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
                )
                .unwrap(),
                minimum_liquidity,
            );
        } else {
            let x: U256 = (amount0 * total_supply) / U256::from(reserve0.as_u128());
            let y: U256 = (amount1 * total_supply) / U256::from(reserve1.as_u128());
            liquidity = self.min(x, y);
        }
        if liquidity > 0.into() {
            self.mint(to, liquidity);
            self.update(balance0, balance1, reserve0, reserve1);
            if fee_on {
                let k_last: U256 = U256::from((reserve0 * reserve1).as_u128()); // reserve0 and reserve1 are up-to-date
                data::set_k_last(k_last);
            }
            data::set_liquidity(liquidity); // return liquidity
            let eventpair: Key = Key::from(data::get_hash());
            self.emit(&PAIREvent::Mint {
                sender: self.get_caller(),
                amount0: amount0,
                amount1: amount1,
                pair: eventpair,
            });
            liquidity // return liquidity
        } else {
            //UniswapV2: INSUFFICIENT_LIQUIDITY_MINTED
            runtime::revert(ApiError::User(FailureCode::TwentyOne as u16));
        }
    }

    fn burn_helper(&mut self, to: Key) -> (U256, U256) {
        let (reserve0, reserve1, _block_timestamp_last) = self.get_reserves(); // gas savings
        let token0: Key = data::get_token0();
        let token1: Key = data::get_token1();
        let token0_hash_add_array = match token0 {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token0_hash_add = ContractHash::new(token0_hash_add_array);
        let token1_hash_add_array = match token1 {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token1_hash_add = ContractHash::new(token1_hash_add_array);
        let balance0: U256 = runtime::call_contract(
            token0_hash_add,
            "balance_of",
            runtime_args! {"owner" => Key::from(data::get_package_hash())},
        );
        let balance1: U256 = runtime::call_contract(
            token1_hash_add,
            "balance_of",
            runtime_args! {"owner" => Key::from(data::get_package_hash())},
        );
        let liquidity: U256 = self.balance_of(Key::from(data::get_package_hash()));
        let fee_on: bool = self.mint_fee(reserve0, reserve1);
        let total_supply: U256 = self.total_supply();
        let amount0: U256 = (liquidity * balance0) / total_supply;
        let amount1: U256 = (liquidity * balance1) / total_supply;
        if amount0 > 0.into() && amount1 > 0.into() {
            self.burn(Key::from(data::get_package_hash()), liquidity);
            // set_key("amount0",amount0);
            // set_key("amount1",amount1);
            let _ret: Result<(), u32> = runtime::call_contract(
                token0_hash_add,
                "transfer",
                runtime_args! {"recipient" => to,"amount" => amount0 },
            );
            match _ret {
                Ok(()) => {}
                Err(e) => runtime::revert(e),
            }
            let _ret: Result<(), u32> = runtime::call_contract(
                token1_hash_add,
                "transfer",
                runtime_args! {"recipient" => to,"amount" => amount1 },
            );
            match _ret {
                Ok(()) => {}
                Err(e) => runtime::revert(e),
            }

            let token0_hash_add_array = match token0 {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let token0_hash_add = ContractHash::new(token0_hash_add_array);
            let token1_hash_add_array = match token1 {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let token1_hash_add = ContractHash::new(token1_hash_add_array);
            let balance0: U256 = runtime::call_contract(
                token0_hash_add,
                "balance_of",
                runtime_args! {"owner" => Key::from(data::get_package_hash())},
            );
            let balance1: U256 = runtime::call_contract(
                token1_hash_add,
                "balance_of",
                runtime_args! {"owner" => Key::from(data::get_package_hash())},
            );
            self.update(balance0, balance1, reserve0, reserve1);
            if fee_on {
                let k_last: U256 = U256::from((reserve0 * reserve1).as_u128()); // reserve0 and reserve1 are up-to-date
                data::set_k_last(k_last);
            }
            data::set_amount0(amount0);
            data::set_amount1(amount1);
            let eventpair: Key = Key::from(data::get_hash());
            self.emit(&PAIREvent::Burn {
                sender: self.get_caller(),
                amount0: amount0,
                amount1: amount1,
                to: to,
                pair: eventpair,
            });
            (amount0, amount1)
        } else {
            //UniswapV2: INSUFFICIENT_LIQUIDITY_BURNED
            runtime::revert(ApiError::User(FailureCode::TwentyTwo as u16));
        }
    }

    // if fee is on, mint liquidity equivalent to 1/6th of the growth in sqrt(k)
    fn mint_fee(&mut self, reserve0: U128, reserve1: U128) -> bool {
        let factory_hash: Key = self.get_factory_hash();
        let factory_hash_add_array = match factory_hash {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let factory_hash_add = ContractHash::new(factory_hash_add_array);
        let fee_to: Key = runtime::call_contract(factory_hash_add, "fee_to", runtime_args! {});
        let mut fee_on: bool = false;
        if fee_to
            != Key::from_formatted_str(
                "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap()
        {
            fee_on = true;
        }
        let k_last: U256 = data::get_k_last(); // gas savings
        let treasury_fee: U256 = data::get_treasury_fee();
        if fee_on {
            if k_last != 0.into() {
                let mul_val: U256 = U256::from((reserve1 * reserve0).as_u128());
                let root_k: U256 = self.sqrt(mul_val);
                let root_k_last: U256 = self.sqrt(k_last);
                if root_k > root_k_last {
                    let subtracted_root_k: U256 = root_k - root_k_last;
                    let numerator: U256 = self.total_supply() * subtracted_root_k;
                    let denominator: U256 = (root_k * treasury_fee) + root_k_last;
                    if denominator > U256::from(0) {
                        let liquidity: U256 = numerator / denominator;
                        if liquidity > 0.into() {
                            self.mint(fee_to, liquidity)
                        }
                    } else {
                        //UniswapV2: DENOMINATOR IS ZERO
                        runtime::revert(ApiError::User(FailureCode::TwentyFive as u16));
                    }
                }
            }
        } else if k_last != 0.into() {
            data::set_k_last(0.into());
        }
        return fee_on;
    }

    fn initialize(&mut self, token0: Key, token1: Key, factory_hash: Key) {
        let factory_hash_getter: Key = self.get_factory_hash();
        if factory_hash == factory_hash_getter {
            data::set_token0(token0);
            data::set_token1(token1);
        } else {
            //(UniswapV2: FORBIDDEN)
            runtime::revert(ApiError::User(FailureCode::Thirteen as u16));
        }
    }

    fn get_reserves(&mut self) -> (U128, U128, u64) {
        let reserve0: U128 = data::get_reserve0();
        let reserve1: U128 = data::get_reserve1();
        let block_timestamp_last: u64 = data::get_block_timestamp_last();
        return (reserve0, reserve1, block_timestamp_last);
    }

    fn sqrt(&mut self, y: U256) -> U256 {
        let mut z: U256 = 0.into();
        if y > 3.into() {
            z = y;
            let mut x: U256 = y / 2 + 1;
            while x < z {
                z = x;
                x = (y / x + x) / 2;
            }
        } else if y != 0.into() {
            z = 1.into();
        }
        return z;
    }

    fn min(&mut self, x: U256, y: U256) -> U256 {
        if x < y {
            x
        } else {
            y
        }
    }

    /// encode a U128 as a U256
    fn encode(&mut self, y: U128) -> U256 {
        let q128: U256 = (2 ^ 128).into();
        let y_u256: U256 = U256::from(y.as_u128());
        let z: U256 = y_u256 * q128; // never overflows
        return z;
    }

    /// divide a U256 by a U128, returning a U256
    fn uqdiv(&mut self, x: U256, y: U128) -> U256 {
        let y_u256: U256 = U256::from(y.as_u128());
        let z: U256 = x / y_u256;
        return z;
    }

    /// encode_uqdiv
    fn encode_uqdiv(
        &mut self,
        encode_reserve: U128,
        uqdiv_reserve: U128,
        mut general_price_cumulative_last: U256,
        time_elapsed: u64,
    ) -> U256 {
        let encode_result: U256 = self.encode(encode_reserve);
        let uqdive_result: U256 = self.uqdiv(encode_result, uqdiv_reserve);
        general_price_cumulative_last =
            general_price_cumulative_last + (uqdive_result * time_elapsed);
        return general_price_cumulative_last;
    }

    fn update(&mut self, balance0: U256, balance1: U256, reserve0: U128, reserve1: U128) {
        let one: U128 = 1.into();
        let overflow_check: U256 = U256::from(((U128::MAX) - one).as_u128());
        if balance0 <= overflow_check && balance1 <= overflow_check {
            let block_timestamp: u64 = runtime::get_blocktime().into();
            let block_timestamp_last: u64 = data::get_block_timestamp_last();
            let time_elapsed: u64 = block_timestamp - block_timestamp_last; // overflow is desired
            if time_elapsed > 0 && reserve0 != 0.into() && reserve1 != 0.into() {
                // * never overflows, and + overflow is desired
                let price0_cumulative_last: U256 = data::get_price0_cumulative_last();
                let price1_cumulative_last: U256 = data::get_price1_cumulative_last();
                let price0_cumulative_last_result: U256 =
                    self.encode_uqdiv(reserve1, reserve0, price0_cumulative_last, time_elapsed);
                data::set_price0_cumulative_last(price0_cumulative_last_result);
                let price1_cumulative_last_result: U256 =
                    self.encode_uqdiv(reserve0, reserve1, price1_cumulative_last, time_elapsed);
                data::set_price1_cumulative_last(price1_cumulative_last_result);
            }
            let reserve0_conversion: U128 = U128::from(balance0.as_u128());
            let reserve1_conversion: U128 = U128::from(balance1.as_u128());
            data::set_reserve0(reserve0_conversion);
            data::set_reserve1(reserve1_conversion);
            data::set_block_timestamp_last(block_timestamp);
            let eventpair: Key = Key::from(data::get_hash());
            self.emit(&PAIREvent::Sync {
                reserve0: reserve0_conversion,
                reserve1: reserve1_conversion,
                pair: eventpair,
            });
        } else {
            //UniswapV2: OVERFLOW
            runtime::revert(ApiError::User(FailureCode::Fifteen as u16));
        }
    }
    fn emit(&mut self, pair_event: &PAIREvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match pair_event {
            PAIREvent::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", pair_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            PAIREvent::Transfer {
                from,
                to,
                value,
                pair,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", pair_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                event.insert("pair", pair.to_string());
                events.push(event);
            }
            PAIREvent::Mint {
                sender,
                amount0,
                amount1,
                pair,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", pair_event.type_name());
                event.insert("sender", sender.to_string());
                event.insert("amount0", amount0.to_string());
                event.insert("amount1", amount1.to_string());
                event.insert("pair", pair.to_string());
                events.push(event);
            }
            PAIREvent::Burn {
                sender,
                amount0,
                amount1,
                to,
                pair,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", pair_event.type_name());
                event.insert("sender", sender.to_string());
                event.insert("amount0", amount0.to_string());
                event.insert("amount1", amount1.to_string());
                event.insert("to", to.to_string());
                event.insert("pair", pair.to_string());
                events.push(event);
            }
            PAIREvent::Swap {
                sender,
                amount0_in,
                amount1_in,
                amount0_out,
                amount1_out,
                to,
                from,
                pair,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", pair_event.type_name());
                event.insert("sender", sender.to_string());
                event.insert(" amount0In", amount0_in.to_string());
                event.insert(" amount1In", amount1_in.to_string());
                event.insert("amount0Out", amount0_out.to_string());
                event.insert("amount1Out", amount1_out.to_string());
                event.insert("to", to.to_string());
                event.insert("from", from.to_string());
                event.insert("pair", pair.to_string());
                events.push(event);
            }
            PAIREvent::Sync {
                reserve0,
                reserve1,
                pair,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", pair_event.type_name());
                event.insert("reserve0", reserve0.to_string());
                event.insert("reserve1", reserve1.to_string());
                event.insert("pair", pair.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
