// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use ink::primitives::AccountId;

use crate::token::psp22::PSP22Error;
pub use crate::token::psp22::PSP22Internal;

pub use pendzl::traits::Balance;

pub trait PSP22BurnableDefaultImpl: PSP22Internal {
    fn burn_default_impl(
        &mut self,
        account: AccountId,
        amount: Balance,
    ) -> Result<(), PSP22Error> {
        self._burn_from(&account, &amount)
    }
}
