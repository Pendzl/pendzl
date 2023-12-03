// SPDX-License-Identifier: MIT

use super::super::{Balance, PSP22Error};
use ink::{contract_ref, env::DefaultEnvironment, primitives::AccountId};
pub type PSP22BurnableRef = contract_ref!(PSP22Burnable, DefaultEnvironment);

#[ink::trait_definition]
pub trait PSP22Burnable {
    /// Destroys `amount` tokens from `account`, deducting from the caller's
    /// allowance.
    ///
    /// See [`PSP22::_burn_from`].
    #[ink(message)]
    fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error>;
}
