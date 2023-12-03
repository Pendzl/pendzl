// SPDX-License-Identifier: MIT
use ink::{prelude::vec::Vec, primitives::AccountId};

/// Contract module which provides a basic implementation of non fungible token.
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
    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool;

    /// Approves `operator` to withdraw the `id` token from the caller's account.
    /// If `id` is `None` approves or disapproves the operator for all tokens of the caller.
    ///
    /// On success a `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `SelfApprove` error if it is self approve.
    ///
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
    ///
    /// Returns `NotApproved` error if `from` doesn't have allowance for transferring.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error>;

    /// Returns current NFT total supply.
    #[ink(message)]
    fn total_supply(&self) -> u64;
}

pub trait PSP34Internal {
    fn _balance_of(&self, owner: &AccountId) -> u32;

    fn _total_supply(&self) -> u64;

    fn _owner_of(&self, id: &Id) -> Option<AccountId>;

    fn _allowance(&self, owner: &AccountId, operator: &AccountId, id: &Option<Id>) -> bool;

    fn _approve(
        &mut self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Option<Id>,
        approved: &bool,
    ) -> Result<(), PSP34Error>;

    fn _update(
        &mut self,
        from: &Option<&AccountId>,
        to: &Option<&AccountId>,
        id: &Id,
    ) -> Result<(), PSP34Error>;

    fn _transfer(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        id: &Id,
        data: &Vec<u8>,
    ) -> Result<(), PSP34Error>;

    fn _mint_to(&mut self, to: &AccountId, id: &Id) -> Result<(), PSP34Error>;

    fn _burn_from(&mut self, from: &AccountId, id: &Id) -> Result<(), PSP34Error>;
}

pub trait PSP34Storage {
    fn balance_of(&self, owner: &AccountId) -> u32;

    fn total_supply(&self) -> u64;

    fn owner_of(&self, id: &Id) -> Option<AccountId>;

    fn allowance(&self, owner: &AccountId, operator: &AccountId, id: &Option<Id>) -> bool;

    fn set_operator_approval(
        &mut self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Option<Id>,
        approved: &bool,
    );

    fn insert_token_owner(&mut self, id: &Id, to: &AccountId) -> Result<(), PSP34Error>;

    fn remove_token_owner(&mut self, id: &Id, from: &AccountId) -> Result<(), PSP34Error>;
}
