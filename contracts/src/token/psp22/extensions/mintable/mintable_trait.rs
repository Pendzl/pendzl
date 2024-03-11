// SPDX-License-Identifier: MIT

use super::super::{Balance, PSP22Error};
use ink::{contract_ref, env::DefaultEnvironment, primitives::AccountId};

pub type PSP22MintableRef = contract_ref!(PSP22Mintable, DefaultEnvironment);

/// trait extending PSP22 with mint functionality
#[ink::trait_definition]
pub trait PSP22Mintable {
    /// Minting `amount` tokens to the account.
    ///
    /// See [`PSP22Internal::_mint_to`].
    #[ink(message)]
    fn mint(
        &mut self,
        account: AccountId,
        amount: Balance,
    ) -> Result<(), PSP22Error>;
}
