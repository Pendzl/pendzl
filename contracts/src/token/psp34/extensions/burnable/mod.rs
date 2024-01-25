// SPDX-License-Identifier: MIT
include!("burnable_trait.rs");

#[cfg(feature = "psp34_burnable_impl")]
pub mod implementation;
