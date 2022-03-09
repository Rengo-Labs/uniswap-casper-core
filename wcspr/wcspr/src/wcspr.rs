use crate::alloc::string::ToString;
use crate::data::{self, Allowances, Balances, WcsprEvents};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use casper_contract::{
    contract_api::{storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, ContractPackageHash, Key, URef, U256, U512};
use contract_utils::{ContractContext, ContractStorage};

#[repr(u16)]
pub enum Error {
    /// 65,540 for UniswapV2CoreWCSPROverFlow
    UniswapV2CoreWCSPROverFlow = 4,
    /// 65,541 for UniswapV2CoreWCSPRUnderFlow
    UniswapV2CoreWCSPRUnderFlow = 5,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
pub trait WCSPR<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        purse: URef,
    ) {
        data::set_name(name);
        data::set_symbol(symbol);
        data::set_hash(contract_hash);
        data::set_decimals(decimals);
        data::set_package_hash(package_hash);
        data::set_self_purse(purse);

        Balances::init();
        Allowances::init();
        data::set_totalsupply(0.into());
    }

    fn balance_of(&mut self, owner: Key) -> U256 {
        Balances::instance().get(&owner)
    }

    fn transfer(&mut self, recipient: Key, amount: U256) -> Result<(), u32> {
        self.make_transfer(self.get_caller(), recipient, amount)
    }

    fn approve(&mut self, spender: Key, amount: U256) {
        self._approve(self.get_caller(), spender, amount);
    }

    fn _approve(&mut self, owner: Key, spender: Key, amount: U256) {
        Allowances::instance().set(&owner, &spender, amount);
        self.emit(&WcsprEvents::Approval {
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
            .ok_or(Error::UniswapV2CoreWCSPROverFlow)
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
            .ok_or(Error::UniswapV2CoreWCSPRUnderFlow)
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
                .ok_or(Error::UniswapV2CoreWCSPRUnderFlow)
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

    fn deposit(&mut self, amount_to_transfer: U512, purse: URef) -> Result<(), u32> {
        let cspr_amount: U512 = system::get_purse_balance(purse).unwrap_or_revert(); // get amount of cspr from purse received
        let _cspr_amount_u256: U256 = U256::from(cspr_amount.as_u128()); // convert amount to U256
        let amount_to_transfer_u256: U256 = U256::from(amount_to_transfer.as_u128()); // convert amount_to_transfer to U256
        let contract_self_purse: URef = data::get_self_purse(); // get this contract's purse

        if amount_to_transfer.is_zero() {
            return Err(5); // Amount to transfer is 0
        }

        if cspr_amount >= amount_to_transfer {
            // save received cspr
            let _ = system::transfer_from_purse_to_purse(
                purse,
                contract_self_purse,
                amount_to_transfer,
                None,
            ); // transfers native cspr from source purse to destination purse

            // mint wcspr for the caller
            let caller = self.get_caller();
            let balances = Balances::instance();
            let balance = balances.get(&caller);
            balances.set(
                &caller,
                balance
                    .checked_add(amount_to_transfer_u256)
                    .ok_or(Error::UniswapV2CoreWCSPROverFlow)
                    .unwrap_or_revert(),
            );

            // update total supply
            data::set_totalsupply(
                data::get_totalsupply()
                    .checked_add(amount_to_transfer_u256)
                    .ok_or(Error::UniswapV2CoreWCSPROverFlow)
                    .unwrap_or_revert(),
            );

            self.emit(&WcsprEvents::Deposit {
                src_purse: purse,
                amount: amount_to_transfer,
            });
        } else {
            return Err(2); // insufficient balance
                           // runtime::revert(MintError::InsufficientFunds);
        }

        Ok(())
    }

    fn withdraw(&mut self, recipient_purse: URef, amount: U512) -> Result<(), u32> {
        let caller = self.get_caller();
        let balances = Balances::instance();
        let balance = balances.get(&caller); // get balance of the caller
        let cspr_amount_u256: U256 = U256::from(amount.as_u128()); // convert U512 to U256

        if amount.is_zero() {
            return Err(5); // Amount to transfer is 0
        }

        let contract_main_purse = data::get_self_purse();
        let main_purse_balance: U512 =
            system::get_purse_balance(contract_main_purse).unwrap_or_revert();

        if balance >= cspr_amount_u256 && amount <= main_purse_balance.into() {
            system::transfer_from_purse_to_purse(
                // transfer native cspr from purse to account
                contract_main_purse,
                recipient_purse,
                amount,
                None,
            )
            .unwrap_or_revert();

            balances.set(
                &caller,
                balance
                    .checked_sub(cspr_amount_u256)
                    .ok_or(Error::UniswapV2CoreWCSPRUnderFlow)
                    .unwrap_or_revert(),
            );

            // update total supply
            data::set_totalsupply(
                data::get_totalsupply()
                    .checked_sub(cspr_amount_u256)
                    .ok_or(Error::UniswapV2CoreWCSPROverFlow)
                    .unwrap_or_revert(),
            );

            self.emit(&WcsprEvents::Withdraw {
                recipient_purse: recipient_purse,
                amount: amount,
            });
        } else {
            return Err(2); // insufficient Balance
        }

        Ok(())
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
                .ok_or(Error::UniswapV2CoreWCSPRUnderFlow)
                .unwrap_or_revert(),
        );
        balances.set(
            &recipient,
            recipient_balance
                .checked_add(amount)
                .ok_or(Error::UniswapV2CoreWCSPROverFlow)
                .unwrap_or_revert(),
        );
        self.emit(&WcsprEvents::Transfer {
            from: sender,
            to: recipient,
            value: amount,
        });
        Ok(())
    }

    fn name(&mut self) -> String {
        data::name()
    }

    fn symbol(&mut self) -> String {
        data::symbol()
    }

    fn purse(&mut self) -> URef {
        data::get_self_purse()
    }

    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }

    // Events
    fn emit(&mut self, wcspr_event: &WcsprEvents) {
        let mut events = Vec::new();
        let package = data::get_package_hash();

        match wcspr_event {
            WcsprEvents::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", wcspr_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }

            WcsprEvents::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", wcspr_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }

            WcsprEvents::Deposit { src_purse, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", wcspr_event.type_name());
                event.insert("source_purse", src_purse.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }

            WcsprEvents::Withdraw {
                recipient_purse,
                amount,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", wcspr_event.type_name());
                event.insert("recipient_purse", recipient_purse.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
        };

        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
