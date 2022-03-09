use crate::data::{self, Allowances, Balances, Nonces};
use alloc::{format, string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    system::mint::Error as MintError, ApiError, BlockTime, ContractHash, ContractPackageHash, Key,
    URef, U256,
};
use contract_utils::{set_key, ContractContext, ContractStorage};
use cryptoxide::ed25519;
use hex::encode;
use renvm_sig::{hash_message, keccak256};

use crate::alloc::string::ToString;
use alloc::collections::BTreeMap;

pub enum ERC20Event {
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
}

impl ERC20Event {
    pub fn type_name(&self) -> String {
        match self {
            ERC20Event::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            ERC20Event::Transfer {
                from: _,
                to: _,
                value: _,
            } => "erc20_transfer",
        }
        .to_string()
    }
}

#[repr(u16)]
pub enum Error {
    /// 65,536 for UniswapV2CoreERC20EXPIRED
    UniswapV2CoreERC20EXPIRED = 0,
    /// 65,537 for UniswapV2CoreERC20SignatureVerificatFailed
    UniswapV2CoreERC20SignatureVerificatFailed = 1,
    /// 65,539 for UniswapV2CoreERC20OverFlow
    UniswapV2CoreERC20OverFlow = 2,
    /// 65,539 for UniswapV2CoreERC20UnderFlow
    UniswapV2CoreERC20UnderFlow1 = 3,
    UniswapV2CoreERC20UnderFlow2 = 29,
    UniswapV2CoreERC20UnderFlow3 = 30,
    UniswapV2CoreERC20UnderFlow4 = 31,
    UniswapV2CoreERC20UnderFlow5 = 32,
    UniswapV2CoreERC20UnderFlow6 = 33,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait ERC20<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        domain_separator: String,
        permit_type_hash: String,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        data::set_name(name);
        data::set_symbol(symbol);
        data::set_decimals(decimals);
        data::set_domain_separator(domain_separator);
        data::set_permit_type_hash(permit_type_hash);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
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
        self.emit(&ERC20Event::Approval {
            owner: owner,
            spender: spender,
            value: amount,
        });
    }

    fn allowance(&mut self, owner: Key, spender: Key) -> U256 {
        Allowances::instance().get(&owner, &spender)
    }

    fn increase_allowance(&mut self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowances = Allowances::instance();
        let balances = Balances::instance();

        let owner: Key = self.get_caller();

        let spender_allowance: U256 = allowances.get(&owner, &spender);
        let owner_balance: U256 = balances.get(&owner);

        let new_allowance: U256 = spender_allowance
            .checked_add(amount)
            .ok_or(Error::UniswapV2CoreERC20OverFlow)
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
            .ok_or(Error::UniswapV2CoreERC20UnderFlow1)
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
                .ok_or(Error::UniswapV2CoreERC20UnderFlow2)
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
                self.emit(&ERC20Event::Approval {
                    owner: owner,
                    spender: spender,
                    value: value,
                });
            } else {
                //signature verification failed
                runtime::revert(Error::UniswapV2CoreERC20SignatureVerificatFailed);
            }
        } else {
            //deadline is equal to or greater than blocktime
            runtime::revert(Error::UniswapV2CoreERC20EXPIRED);
        }
    }

    fn mint(&mut self, recipient: Key, amount: U256) {
        let balances: Balances = Balances::instance();
        let balance: U256 = balances.get(&recipient);
        balances.set(
            &recipient,
            balance
                .checked_add(amount)
                .ok_or(Error::UniswapV2CoreERC20OverFlow)
                .unwrap_or_revert(),
        );
        data::set_total_supply(
            data::total_supply()
                .checked_add(amount)
                .ok_or(Error::UniswapV2CoreERC20OverFlow)
                .unwrap_or_revert(),
        );
        let address_0: Key = Key::from_formatted_str(
            "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();
        self.emit(&ERC20Event::Transfer {
            from: address_0,
            to: recipient,
            value: amount,
        });
    }

    fn burn(&mut self, recipient: Key, amount: U256) {
        let balances: Balances = Balances::instance();
        let balance: U256 = balances.get(&recipient);
        if balance >= amount {
            balances.set(
                &recipient,
                balance
                    .checked_sub(amount)
                    .ok_or(Error::UniswapV2CoreERC20UnderFlow3)
                    .unwrap_or_revert(),
            );
            data::set_total_supply(
                data::total_supply()
                    .checked_sub(amount)
                    .ok_or(Error::UniswapV2CoreERC20UnderFlow4)
                    .unwrap_or_revert(),
            );
            let address_0: Key = Key::from_formatted_str(
                "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap();
            self.emit(&ERC20Event::Transfer {
                from: recipient,
                to: address_0,
                value: amount,
            });
        } else {
            // PosError::InsufficientPaymentForAmountSpent
            runtime::revert(MintError::InsufficientFunds)
        }
    }

    fn set_nonce(&mut self, recipient: Key) {
        let nonces: Nonces = Nonces::instance();
        let nonce: U256 = nonces.get(&recipient);
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
                .ok_or(Error::UniswapV2CoreERC20UnderFlow5)
                .unwrap_or_revert(),
        );
        balances.set(
            &recipient,
            recipient_balance
                .checked_add(amount)
                .ok_or(Error::UniswapV2CoreERC20OverFlow)
                .unwrap_or_revert(),
        );
        self.emit(&ERC20Event::Transfer {
            from: sender,
            to: recipient,
            value: amount,
        });
        Ok(())
    }

    fn total_supply(&mut self) -> U256 {
        data::total_supply()
    }

    fn name(&mut self) -> String {
        data::name()
    }

    fn symbol(&mut self) -> String {
        data::symbol()
    }

    fn get_permit_type_and_domain_separator(
        &mut self,
        name: &str,
        contract_hash: ContractHash,
    ) -> (String, String) {
        let eip_712_domain: &str =
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
        let permit_type: &str =
            "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)";
        let chain_id: &str = "101";
        let eip_domain_hash: [u8; 32] = keccak256(eip_712_domain.as_bytes()); // to take a byte hash of EIP712Domain
        let name_hash: [u8; 32] = keccak256(name.as_bytes()); // to take a byte hash of name
        let one_hash: [u8; 32] = keccak256("1".as_bytes()); // to take a byte hash of "1"
        let eip_domain_hash: String = encode(eip_domain_hash); // to encode and convert eip_domain_hash into string
        let name_hash: String = encode(name_hash); // to encode and convert name_hash into string
        let one_hash: String = encode(one_hash); // to encode and convert one_hash into string
        let concatenated_data: String = format!(
            "{}{}{}{}{}",
            eip_domain_hash, name_hash, one_hash, chain_id, contract_hash
        ); //string contactination
        let domain_separator: [u8; 32] = keccak256(concatenated_data.as_bytes()); //to take a byte hash of concatenated Data
        let permit_type_hash: [u8; 32] = keccak256(permit_type.as_bytes()); // to take a byte hash of Permit Type
        let domain_separator: String = encode(domain_separator);
        let permit_type_hash: String = encode(permit_type_hash);
        (domain_separator, permit_type_hash)
    }
    fn emit(&mut self, erc20_event: &ERC20Event) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match erc20_event {
            ERC20Event::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            ERC20Event::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }

    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }
}
