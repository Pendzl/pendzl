// SPDX-License-Identifier: MIT
include!("metadata_trait.rs");

#[cfg(any(
    feature = "psp22_metadata_impl",
    feature = "psp22_vault_metadata_impl"
))]
pub mod implementation;

#[cfg(any(feature = "psp22_vault_metadata_impl"))]
pub mod vault_implementation;
