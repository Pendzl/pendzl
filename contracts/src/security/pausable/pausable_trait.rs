// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use ink::primitives::AccountId;

use ink::{contract_ref, env::DefaultEnvironment};
pub type PausableRef = contract_ref!(Pausable, DefaultEnvironment);

/// trait that should be implemented by the contract to use pausable functionality
#[ink::trait_definition]
pub trait Pausable {
    /// Returns true if the contract is paused, and false otherwise.
    #[ink(message)]
    fn paused(&self) -> bool;
}

/// trait that must be implemented by exactly one storage field of a contract storage
/// so the Pendzl PSP22Internal and PSP22 implementation can be derived.
pub trait PausableStorage {
    /// Retrieves the paused state.
    fn paused(&self) -> bool;

    /// Sets the paused state.
    fn set_paused(&mut self, pause: bool);
}

/// trait that is derived by Pendzl Pausable implementation macro assuming StorageFieldGetter<PausableStorage> is implemented
///
/// functions of this trait are recomended to use while writing ink::messages
pub trait PausableInternal {
    /// Internal function to check the paused state.
    fn _paused(&self) -> bool;

    /// Pauses the contract. Emits a `Paused` event on success.
    ///
    /// On success emits Paused event
    ///
    /// # Errors
    /// Returns `Paused` error if the contract is already paused.
    fn _pause(&mut self) -> Result<(), PausableError>;

    /// Unpauses the contract. Emits an `Unpaused` event on success.
    ///
    /// On success emits Unpaused event.
    ///
    /// # Errors
    /// Returns `Unpaused` error if the contract is not currently paused.
    fn _unpause(&mut self) -> Result<(), PausableError>;

    /// Ensures the contract is currently paused.
    ///
    /// # Errors
    /// Returns `Unpaused` error if the contract is not paused.
    fn _ensure_paused(&self) -> Result<(), PausableError>;

    /// Ensures the contract is not currently paused.
    ///
    /// # Errors
    /// Returns `Paused` error if the contract is paused.
    fn _ensure_not_paused(&self) -> Result<(), PausableError>;
}
