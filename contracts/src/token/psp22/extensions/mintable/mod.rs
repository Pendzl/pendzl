// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
include!("mintable_trait.rs");

#[cfg(feature = "psp22_mintable_impl")]
pub mod implementation;

#[cfg(feature = "psp22_mintable_impl")]
pub use implementation::*;
