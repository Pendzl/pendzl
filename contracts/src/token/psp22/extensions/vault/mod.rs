// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
pub use crate::token::psp22::{PSP22Error, PSP22Ref};
pub use ink::primitives::AccountId;
pub use pendzl::{
    math::{errors::MathError, operations::Rounding},
    traits::Balance,
};

include!("vault_events.rs");
include!("vault_trait.rs");

#[cfg(all(feature = "psp22_vault_impl"))]
mod implementation;

#[cfg(all(feature = "psp22_vault_impl"))]
pub use implementation::*;
