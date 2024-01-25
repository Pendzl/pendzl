// SPDX-License-Identifier: MIT
include!("mintable_trait.rs");

#[cfg(feature = "psp22_mintable_impl")]
pub mod implementation;
