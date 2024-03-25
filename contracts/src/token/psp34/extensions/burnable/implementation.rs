// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use ink::primitives::AccountId;

use crate::token::psp34::{Id, PSP34Error, PSP34Internal};

pub trait PSP34BurnableDefaultImpl: PSP34Internal {
    fn burn_default_impl(
        &mut self,
        account: AccountId,
        id: Id,
    ) -> Result<(), PSP34Error> {
        self._burn_from(&account, &id)
    }
}
