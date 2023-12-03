// SPDX-License-Identifier: MIT
use ink::{contract_ref, env::DefaultEnvironment, primitives::AccountId};

use crate::token::psp34::{Id, PSP34Error};
pub type PSP34BurnableRef = contract_ref!(PSP34Burnable, DefaultEnvironment);

#[ink::trait_definition]
pub trait PSP34Burnable {
    /// Destroys token with id equal to `id` from `account`
    ///
    /// Caller must be approved to transfer tokens from `account`
    /// or to transfer token with `id`
    #[ink(message)]
    fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}
