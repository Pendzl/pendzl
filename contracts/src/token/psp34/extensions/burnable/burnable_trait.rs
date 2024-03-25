// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use ink::{contract_ref, env::DefaultEnvironment, primitives::AccountId};

use crate::token::psp34::{Id, PSP34Error};
pub type PSP34BurnableRef = contract_ref!(PSP34Burnable, DefaultEnvironment);

/// trait extending PSP34 with burn functionality
#[ink::trait_definition]
pub trait PSP34Burnable {
    /// Destroys token with id equal to `id` from `account`
    ///
    /// Caller must be approved to transfer tokens from `account`
    /// or to transfer token with `id`
    #[ink(message)]
    fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}
