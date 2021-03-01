#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {

    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct Erc20 {
        /// Stores a single `bool` value on the storage.
        total_supply: Balance,
        balances: StorageHashMap<AccountId, Balance>,
        allowance: StorageHashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InSufficientBalance,
        NoAllowTransfer,
        FailTransferBalance
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut ballances = StorageHashMap::new();
            ballances.insert(caller, total_supply);
            let instance = Self {
                total_supply: total_supply,
                balances: ballances,
                allowance: StorageHashMap::new()
            };

            instance
        }

        // 合约的公共方法有两种  1.读  2.写
        /// 查询用户余额
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// 查询账户余额
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }

        /// 设置spender 账户可以转账的额度 value
        #[ink(message)]
        pub fn set_allowance(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            // let key = (sender, spender);
            self.allowance.insert((sender, spender), value);

            Ok(())
        }

        /// 操作账户 额度
        #[ink(message)]
        pub fn allowance_of(&self, owner: AccountId, spender: AccountId) -> Balance {
            *self.allowance.get(&(owner, spender)).unwrap_or( &0)
        }
        
        /// 转账
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let who = self.env().caller();
            self.transfer_help(who, to, value)
        }

        fn transfer_help(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let from_balance = self.balance_of(from);

            if from_balance < value {
                return Err(Error::InSufficientBalance);
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);

            self.env().emit_event(Transfer {
                from: from ,
                to: to,
                value: value
            });

            Ok(())
        }
    
        /// 操作 from 账户转账
        #[ink(message)]
        pub fn trasfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();

            let alloc_balance = self.allowance_of(from, sender);
            // 额度受限
            if alloc_balance < value {
                return Err(Error::NoAllowTransfer);
            }

            self.transfer_help(from, to, value)
        }

        // #[ink(message)]
        // pub fn burn(...) {

        // }

        // #[ink(message)]
        // pub fn issue(...) {

        // }
    }


    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn create_contract_works() {
            let erc20 = Erc20::new(1000);
            
            assert_eq!(erc20.total_supply(), 1000);
        }
    }
}