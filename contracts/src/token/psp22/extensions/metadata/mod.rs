// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
include!("metadata_trait.rs");

#[cfg(all(feature = "psp22_metadata_impl", not(feature = "psp22_vault_impl")))]
mod implementation;

#[cfg(all(
    feature = "psp22_metadata_impl",
    not(feature = "psp22_vault_impl")
))]
pub use implementation::*;

#[cfg(all(feature = "psp22_metadata_impl", feature = "psp22_vault_impl"))]
mod vault_implementation;

#[cfg(all(feature = "psp22_metadata_impl", feature = "psp22_vault_impl"))]
pub use vault_implementation::*;
