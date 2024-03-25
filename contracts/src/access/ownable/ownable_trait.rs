// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use ink::primitives::AccountId;

use ink::{contract_ref, env::DefaultEnvironment};
pub type OwnableRef = contract_ref!(Ownable, DefaultEnvironment);

/// Ownable trait that provides a framework for implementing
/// access mechanisms in smart contracts. One account can be granted an "ownership" of the contract.
/// The ownership can be transferred and revoked with the `transfer_ownership` and `renounce_ownership` functions.
#[ink::trait_definition]
pub trait Ownable {
    /// Returns the address of the current owner.
    #[ink(message)]
    fn owner(&self) -> Option<AccountId>;

    /// Leaves the contract without owner. It will not be possible to call
    /// owner's functions anymore. Can only be called by the current owner.
    ///
    /// NOTE: Renouncing ownership will leave the contract without an owner,
    /// thereby removing any functionality that is only available to the owner.
    ///
    /// On success a `OwnershipTransferred` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `CallerIsNotOwner` error if caller is not owner
    #[ink(message)]
    fn renounce_ownership(&mut self) -> Result<(), OwnableError>;

    /// Transfers ownership of the contract to a `new_owner`.
    /// Can only be called by the current owner.
    ///
    /// On success a `OwnershipTransferred` event is emitted.
    ///
    /// # Errors
    ///
    /// Panics with `CallerIsNotOwner` error if caller is not owner.
    ///
    /// Panics with `NewOwnerIsZero` error if new owner's address is zero.
    #[ink(message)]
    fn transfer_ownership(
        &mut self,
        new_owner: AccountId,
    ) -> Result<(), OwnableError>;
}

/// trait that must be implemented by exactly one storage field of a contract storage
/// so the Pendzl OwnableInternal and Ownable implementation can be derived.
pub trait OwnableStorage {
    /// Returns the current owner.
    fn owner(&self) -> Option<AccountId>;

    /// Sets a new owner.
    fn set_owner(&mut self, new_owner: &Option<AccountId>);
}

/// trait that is derived by Pendzl Ownable implementation macro assuming StorageFieldGetter<OwnableStorage> is implemented
///
/// functions of this trait are recomended to use while writing ink::messages
pub trait OwnableInternal {
    /// Retrieves the current owner.
    fn _owner(&self) -> Option<AccountId>;

    /// Updates the owner to the `owner`.
    ///
    /// On success emits `OwnershipTransferred` event.
    fn _update_owner(&mut self, owner: &Option<AccountId>);

    /// Verifies that the caller is the current owner.
    ///
    /// #Errors
    ///
    /// Returns `CallerIsNotOwner` error if caller is not owner.
    fn _only_owner(&self) -> Result<(), OwnableError>;
}
