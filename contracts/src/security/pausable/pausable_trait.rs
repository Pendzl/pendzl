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
    fn _paused(&self) -> bool;

    /// Triggers stopped state.
    ///
    /// On success a `Paused` event is emitted.
    fn _pause(&mut self) -> Result<(), PausableError>;

    /// Returns to normal state.
    ///
    /// On success a `Unpaused` event is emitted.
    fn _unpause(&mut self) -> Result<(), PausableError>;

    fn _ensure_paused(&self) -> Result<(), PausableError>;

    fn _ensure_not_paused(&self) -> Result<(), PausableError>;
}

pub trait PausableStorage {
    fn paused(&self) -> bool;

    fn set_paused(&mut self, pause: bool);
}
