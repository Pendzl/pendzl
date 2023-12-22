// SPDX-License-Identifier: MIT

use ink::primitives::AccountId;

use ink::{contract_ref, env::DefaultEnvironment};
pub type PausableRef = contract_ref!(Pausable, DefaultEnvironment);

/// Contract trait, which allows children to implement an emergency stop
/// mechanism that an authorized account can trigger.
#[ink::trait_definition]
pub trait Pausable {
    /// Returns true if the contract is paused, and false otherwise.
    #[ink(message)]
    fn paused(&self) -> bool;
}

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

/// A trait that should be implemented by the storage contract item to use default Internal implementation.
pub trait PausableStorage {
    /// Retrieves the paused state.
    fn paused(&self) -> bool;

    /// Sets the paused state.
    fn set_paused(&mut self, pause: bool);
}
