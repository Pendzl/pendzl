// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use ink::contract_ref;
use ink::{prelude::vec::Vec, primitives::AccountId};
pub use pendzl::traits::Balance;

include!("general_vest_error.rs");
include!("general_vest_events.rs");
include!("general_vest_types.rs");
include!("general_vest_trait.rs");

#[cfg(feature = "provide_vest_schedule_info")]
include!("provide_vest_schedule_info_trait.rs");

/// implementation of the traits
#[cfg(feature = "general_vest_impl")]
mod implementation;

#[cfg(feature = "general_vest_impl")]
pub use implementation::*;
