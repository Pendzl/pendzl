// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
include!("psp34_types.rs");
include!("psp34_error.rs");
include!("psp34_events.rs");
include!("psp34_trait.rs");

#[cfg(feature = "psp34_impl")]
mod implementation;

#[cfg(feature = "psp34_impl")]
pub use implementation::*;

mod extensions;

#[cfg(feature = "psp34_burnable")]
pub use extensions::burnable;
#[cfg(feature = "psp34_metadata")]
pub use extensions::metadata;
#[cfg(feature = "psp34_mintable")]
pub use extensions::mintable;
