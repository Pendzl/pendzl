// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use super::super::PSP22Error;
use ink::{contract_ref, env::DefaultEnvironment, primitives::AccountId};
pub use pendzl::traits::Balance;

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
