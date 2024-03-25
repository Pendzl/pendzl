// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use ink::{prelude::vec::Vec, primitives::AccountId};

/// # PSP-34: Token standard
/// https://github.com/inkdevhub/standards/blob/master/PSPs/psp-34.md
///
/// !!! Note
/// Pendzl implementation allows to use zero address as a valid address
/// and doen't revert ZeroAddress errors.
/// Pendzl implementation doesn't check if the recipient is a contract
/// and doesn't revert SafeTransferCheckFailed
/// Pendzl implementation returns 'TokenNotExists' error if token doesn't exist on approve.
#[ink::trait_definition]
pub trait PSP34 {
    /// Returns the collection `Id` of the NFT token.
    ///
    /// This can represents the relationship between tokens/contracts/pallets.
    #[ink(message)]
    fn collection_id(&self) -> Id;

    /// Returns the balance of the owner.
    ///
    /// This represents the amount of unique tokens the owner has.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u32;

    /// Returns the owner of the token if any.
    #[ink(message)]
    fn owner_of(&self, id: Id) -> Option<AccountId>;

    /// Returns `true` if the operator is approved by the owner to withdraw `id` token.
    /// If `id` is `None`, returns `true` if the operator is approved to withdraw all owner's tokens.
    #[ink(message)]
    fn allowance(
        &self,
        owner: AccountId,
        operator: AccountId,
        id: Option<Id>,
    ) -> bool;

    /// Approves `operator` to withdraw the `id` token from the caller's account.
    /// If `id` is `None` approves or disapproves the operator for all tokens of the caller.
    ///
    /// On success a `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `SelfApprove` error if it is self approve.
    /// Returns `TokenNotExists` error if token doesn't exist.
    /// Returns `NotApproved` error if caller is not owner of `id`.
    #[ink(message)]
    fn approve(
        &mut self,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    ) -> Result<(), PSP34Error>;

    /// Transfer approved or owned token from caller.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TokenNotExists` error if `id` does not exist.
    /// Returns `NotApproved` error if `from` doesn't have allowance for transferring.
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer(
        &mut self,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP34Error>;

    /// Returns current NFT total supply.
    #[ink(message)]
    fn total_supply(&self) -> u64;
}

/// trait that must be implemented by exactly one storage field of a contract storage
/// so the Pendzl PSP34Internal and PSP34 implementation can be derived.
pub trait PSP34Storage {
    /// Retrieves the balance of unique tokens for an owner.
    fn balance_of(&self, owner: &AccountId) -> u32;

    /// Retrieves the total supply of NFT tokens.
    fn total_supply(&self) -> u64;

    /// Retrieves the owner of a specific token Id.
    fn owner_of(&self, id: &Id) -> Option<AccountId>;

    /// Checks if an operator is approved to manage a specific token.
    fn allowance(
        &self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Option<Id>,
    ) -> bool;

    /// Sets the approval status of an operator for a specific token.
    fn set_operator_approval(
        &mut self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Option<Id>,
        approved: &bool,
    );

    /// Sets a token with `id` owner to `to`
    ///
    /// # Errors
    /// Returns 'TokenExists' if a token with `id` has an owner alraedy.
    fn insert_token_owner(
        &mut self,
        id: &Id,
        to: &AccountId,
    ) -> Result<(), PSP34Error>;

    /// Removes a token with `id` owner.
    ///
    /// # Errors
    /// - Returns `NotApproved` if `from` is not an owner of token with `id`.
    fn remove_token_owner(
        &mut self,
        id: &Id,
        from: &AccountId,
    ) -> Result<(), PSP34Error>;
}
/// trait that is derived by Pendzl PSP34 implementation macro assuming StorageFieldGetter<PSP34Storage> is implemented
///
/// functions of this trait are recomended to use while writing ink::messages
pub trait PSP34Internal {
    /// Retrieves the balance of unique tokens for an owner.
    fn _balance_of(&self, owner: &AccountId) -> u32;

    /// Retrieves the total supply of NFT tokens.
    fn _total_supply(&self) -> u64;

    /// Retrieves the owner of a specific token Id.
    fn _owner_of(&self, id: &Id) -> Option<AccountId>;

    /// Checks if an operator is approved to manage a specific token.
    fn _allowance(
        &self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Option<Id>,
    ) -> bool;

    /// Approves `operator` to withdraw the `id` token from the caller's account.
    /// If `id` is `None` approves or disapproves the operator for all tokens of the caller.
    ///
    /// On success a `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `SelfApprove` error if it is self approve.
    /// Returns `TokenNotExists` error if token doesn't exist.
    /// Returns `NotApproved` error if caller is not owner of `id`.
    fn _approve(
        &mut self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Option<Id>,
        approved: &bool,
    ) -> Result<(), PSP34Error>;

    /// Updates ownership of token identified by `id`.
    /// Depending if `from` is None and `to` is none operation corresponds to transfer, mint, burn.
    ///
    /// On success emits `Transfer` event.
    ///
    /// # Errors
    /// May returns `TokenExists` error if token already exist and from is None.
    /// Returns `NotApproved` error if `from`` is not owner of `id`.
    fn _update(
        &mut self,
        from: &Option<&AccountId>,
        to: &Option<&AccountId>,
        id: &Id,
    ) -> Result<(), PSP34Error>;

    /// Internal function to transfer a token.
    /// Emits a `Transfer` event on success.
    /// # Errors
    /// - Various errors as defined in `PSP34Error`.
    fn _transfer(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        id: &Id,
        data: &Vec<u8>,
    ) -> Result<(), PSP34Error>;

    /// Internal function to mint a new token.
    /// # Errors
    /// - Various errors as defined in `PSP34Error`.
    fn _mint_to(&mut self, to: &AccountId, id: &Id) -> Result<(), PSP34Error>;

    /// Internal function to burn an existing token.
    /// # Errors
    /// - Various errors as defined in `PSP34Error`.
    fn _burn_from(
        &mut self,
        from: &AccountId,
        id: &Id,
    ) -> Result<(), PSP34Error>;
}
