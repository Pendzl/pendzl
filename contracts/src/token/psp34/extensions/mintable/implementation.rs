// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use ink::primitives::AccountId;

use crate::token::psp34::{Id, PSP34Error, PSP34Internal};

pub trait PSP34MintableDefaultImpl: PSP34Internal {
    fn mint_default_impl(
        &mut self,
        account: AccountId,
        id: Id,
    ) -> Result<(), PSP34Error> {
        self._mint_to(&account, &id)
    }
}
