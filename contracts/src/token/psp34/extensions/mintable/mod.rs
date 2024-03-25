// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
include!("mintable_trait.rs");

#[cfg(feature = "psp34_mintable_impl")]
mod implementation;

#[cfg(feature = "psp34_mintable_impl")]
pub use implementation::*;
