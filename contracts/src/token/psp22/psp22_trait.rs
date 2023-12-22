// SPDX-License-Identifier: MIT

use ink::{contract_ref, env::DefaultEnvironment, prelude::vec::Vec, primitives::AccountId};
pub type PSP22Ref = contract_ref!(PSP22, DefaultEnvironment);

pub use pendzl::traits::Balance;

/// Trait implemented by all PSP-22 respecting smart traits.
#[ink::trait_definition]
pub trait PSP22 {
    /// Returns the total token supply.
    #[ink(message)]
    fn total_supply(&self) -> Balance;

    /// Returns the account Balance for the specified `owner`.
    ///
    /// Returns `0` if the account is non-existent.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Balance;

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    ///
    /// Returns `0` if no allowance has been set `0`.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

    /// Transfers `value` amount of tokens from the caller's account to account `to`
    /// with additional `data` in unspecified format.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientBalance` error if there are not enough tokens on
    /// the caller's account Balance.
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error>;

    /// Transfers `value` tokens on the behalf of `from` to the account `to`
    /// with additional `data` in unspecified format.
    ///
    /// This can be used to allow a contract to transfer tokens on ones behalf and/or
    /// to charge fees in sub-currencies, for example.
    ///
    /// On success a `Transfer` and `Approval` events are emitted.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
    /// for the caller to withdraw from `from`.
    ///
    /// Returns `InsufficientBalance` error if there are not enough tokens on
    /// the the account Balance of `from`.
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error>;

    /// Allows `spender` to withdraw from the caller's account multiple times, up to
    /// the `value` amount.
    ///
    /// If this function is called again it overwrites the current allowance with `value`.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error>;

    /// Atomically increases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn increase_allowance(
        &mut self,
        spender: AccountId,
        delta_value: Balance,
    ) -> Result<(), PSP22Error>;

    /// Atomically decreases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientAllowance` error if there are not enough tokens allowed
    /// by owner for `spender`.
    ///
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    ///
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn decrease_allowance(
        &mut self,
        spender: AccountId,
        delta_value: Balance,
    ) -> Result<(), PSP22Error>;
}

pub trait PSP22Storage {
    fn total_supply(&self) -> Balance;
    fn increase_total_supply(&mut self, amount: &Balance) -> Result<(), PSP22Error>;
    fn decrease_total_supply(&mut self, amount: &Balance) -> Result<(), PSP22Error>;

    fn balance_of(&self, account: &AccountId) -> Balance;
    fn increase_balance_of(
        &mut self,
        account: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;
    fn decrease_balance_of(
        &mut self,
        account: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;

    fn allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance;
    fn set_allowance(&mut self, owner: &AccountId, spender: &AccountId, value: &Balance);
    fn increase_allowance(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<Balance, PSP22Error>;
    fn decrease_allowance(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<Balance, PSP22Error>;
}

pub trait PSP22Internal {
    /// Retrieves the total supply of tokens.
    fn _total_supply(&self) -> Balance;

    /// Retrieves the token balance for a specified owner.
    fn _balance_of(&self, owner: &AccountId) -> Balance;

    /// Retrieves the remaining allowance that a spender has from an owner.
    fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance;

    /// Internal function to update balances of 'from' and 'to' by 'amount' and total supply.
    /// It can be used to transfer, mint and burn depending if from and to are Some or None.
    ///
    /// On success emits a `Transfer` event.
    ///
    /// # Errors
    /// Returns `InsufficientBalance` if 'from' doesn't have enought balance.
    fn _update(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;

    /// Transfer 'amount' 'from' 'to'.
    ///
    /// On success emits a `Transfer` event.
    ///
    /// # Errors
    /// Returns `InsufficientBalance` if 'from' doesn't have enought balance.
    fn _transfer(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;

    /// Mints 'amount' 'to'.
    ///
    /// On success emits a `Transfer` event.
    fn _mint_to(&mut self, to: &AccountId, amount: &Balance) -> Result<(), PSP22Error>;

    /// Burns 'amount' 'from'.
    ///
    /// On success emits a `Transfer` event.
    ///
    /// # Errors
    /// Returns `InsufficientBalance` if 'from' doesn't have enought balance.
    fn _burn_from(&mut self, from: &AccountId, amount: &Balance) -> Result<(), PSP22Error>;

    /// Sets allowance of `spender` to spend `amount` of tokens of `owner`.
    ///
    /// On success emits `Approval` event.
    fn _approve(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;

    /// Decrease an allowance of `spender` to spend tokens of `owner` by `amount`.
    ///
    /// On success emits `Approval` event.
    ///
    /// # Errors
    /// - Returns `InsufficientAllowance` if the current allowance is smaller than `amount`.
    fn _decrease_allowance_from_to(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;

    /// Increases an allowance of `spender` to spend tokens of `owner` by `amount`.
    ///
    /// On success emits `Approval` event.
    fn _increase_allowance_from_to(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;
}
