// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use ink::{
    contract_ref, env::DefaultEnvironment, prelude::vec::Vec,
    primitives::AccountId,
};
pub type PSP22Ref = contract_ref!(PSP22, DefaultEnvironment);

pub use pendzl::traits::Balance;

/// # PSP-22: Token standard
/// https://github.com/inkdevhub/standards/blob/master/PSPs/psp-22.md
///
/// !!! Note
/// Pendzl implementation allows to use zero address as a valid address
/// and doen't revert ZeroAddress errors.
/// Pendzl implementation doesn't check if the recipient is a contract
/// and doesn't revert SafeTransferCheckFailed
#[ink::trait_definition]
pub trait PSP22 {
    /// Returns the total token supply.
    #[ink(message)]
    fn total_supply(&self) -> Balance;

    /// Returns the account balance for the specified `owner`.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Balance;

    /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

    /// Transfers `value` amount of tokens from the caller's account to account `to`
    /// with additional `data` in unspecified format.
    ///
    /// On success a `Transfer` event is emitted.,
    ///
    /// # Errors,
    ///
    /// Returns with error `InsufficientBalance` if there are not enough tokens on
    /// the caller's account Balance.,
    /// Returns with error `ZeroSenderAddress` if sender's address is zero.
    /// Returns with error `ZeroRecipientAddress` if recipient's address is zero.
    /// Returns with error `SafeTransferCheckFailed` if the recipient is a contract and rejected the transfer.
    #[ink(message)]
    fn transfer(
        &mut self,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error>;

    /// Transfers `value` tokens on the behalf of `from` to the account `to`
    /// with additional `data` in unspecified format.
    /// This can be used to allow a contract to transfer tokens on ones behalf and/or
    /// to charge fees in sub-currencies, for example.
    ///
    /// On success a `Transfer` and `Approval` events are emitted.
    ///
    /// # Errors
    /// Returns with error `InsufficientAllowance` if there are not enough tokens allowed
    /// for the caller to withdraw from `from`.
    /// Returns with error `InsufficientBalance` if there are not enough tokens on
    /// the the account Balance of `from`.
    /// Returns with error `ZeroSenderAddress` if sender's address is zero.
    /// Returns with error `ZeroRecipientAddress` if recipient's address is zero.
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
    /// If this function is called again it overwrites the current allowance with `value`.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn approve(
        &mut self,
        spender: AccountId,
        value: Balance,
    ) -> Result<(), PSP22Error>;

    /// Atomically increases the allowance granted to `spender` by the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
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
    /// Returns `ZeroSenderAddress` error if sender's address is zero.
    /// Returns `ZeroRecipientAddress` error if recipient's address is zero.
    #[ink(message)]
    fn decrease_allowance(
        &mut self,
        spender: AccountId,
        delta_value: Balance,
    ) -> Result<(), PSP22Error>;
}

/// trait that must be implemented by exactly one storage field of a contract storage
/// so the Pendzl PSP22Internal and PSP22 implementation can be derived.
pub trait PSP22Storage {
    /// Returns the total supply of tokens.
    fn total_supply(&self) -> Balance;

    /// Increases the total supply of tokens by the given `amount`.
    fn increase_total_supply(
        &mut self,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;

    /// Decreases the total supply of tokens by the given `amount`.
    fn decrease_total_supply(
        &mut self,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;

    /// Returns the balance of the `account`.
    fn balance_of(&self, account: &AccountId) -> Balance;

    /// Increases the balance of the `account` by the given `amount`.
    fn increase_balance_of(
        &mut self,
        account: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;

    /// Decreases the balance of the `account` by the given `amount`.
    fn decrease_balance_of(
        &mut self,
        account: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;

    /// Returns the allowance of `spender` to spend `owner`'s tokens.
    fn allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance;

    /// Sets the allowance of `spender` to spend `owner`'s tokens to the given `value`.
    fn set_allowance(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        value: &Balance,
    );

    /// Increases the allowance of `spender` to spend `owner`'s tokens by the given `amount`.
    fn increase_allowance(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<Balance, PSP22Error>;

    /// Decreases the allowance of `spender` to spend `owner`'s tokens by the given `amount`.
    fn decrease_allowance(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<Balance, PSP22Error>;
}

/// trait that is derived by Pendzl PSP22 implementation macro assuming StorageFieldGetter<PSP22Storage> is implemented
///
/// functions of this trait are recomended to use while writing ink::messages
pub trait PSP22Internal {
    /// Returns the total supply of tokens.
    fn _total_supply(&self) -> Balance;

    /// Returns the token balance for a specified owner.
    fn _balance_of(&self, owner: &AccountId) -> Balance;

    /// Returns the remaining allowance that a spender has from an owner.
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
    fn _mint_to(
        &mut self,
        to: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;

    /// Burns 'amount' 'from'.
    ///
    /// On success emits a `Transfer` event.
    ///
    /// # Errors
    /// Returns `InsufficientBalance` if 'from' doesn't have enought balance.
    fn _burn_from(
        &mut self,
        from: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error>;

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
