// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
include!("pausable_error.rs");
include!("pausable_events.rs");
include!("pausable_trait.rs");

#[cfg(feature = "pausable_impl")]
mod implementation;

#[cfg(feature = "pausable_impl")]
pub use implementation::*;
