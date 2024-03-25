// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use crate::token::psp34::{Id, PSP34Error};
use ink::{contract_ref, env::DefaultEnvironment, primitives::AccountId};

pub type PSP34MintableRef = contract_ref!(PSP34Mintable, DefaultEnvironment);
/// trait extending PSP34 with mint functionality
#[ink::trait_definition]
pub trait PSP34Mintable {
    /// Mints a new token with `id`.
    ///
    /// See [`PSP34::_mint_to`].
    #[ink(message)]
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}
