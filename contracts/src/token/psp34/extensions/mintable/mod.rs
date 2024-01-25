// SPDX-License-Identifier: MIT
include!("mintable_trait.rs");

#[cfg(feature = "psp34_mintable_impl")]
pub mod implementation;
