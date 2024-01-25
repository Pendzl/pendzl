// SPDX-License-Identifier: MIT
include!("metadata_events.rs");
include!("metadata_trait.rs");

#[cfg(feature = "psp34_metadata_impl")]
pub mod implementation;
