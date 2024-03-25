// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
include!("metadata_events.rs");
include!("metadata_trait.rs");

#[cfg(feature = "psp34_metadata_impl")]
mod implementation;

#[cfg(feature = "psp34_metadata_impl")]
pub use implementation::*;
