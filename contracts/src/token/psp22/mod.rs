// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
include!("psp22_error.rs");
include!("psp22_events.rs");
include!("psp22_trait.rs");

#[cfg(feature = "psp22_impl")]
mod implementation;

#[cfg(feature = "psp22_impl")]
pub use implementation::*;

mod extensions;

#[cfg(feature = "psp22_burnable")]
pub use extensions::burnable;
#[cfg(any(feature = "psp22_metadata", feature = "psp22_vault_metadata"))]
pub use extensions::metadata;
#[cfg(feature = "psp22_mintable")]
pub use extensions::mintable;
#[cfg(feature = "psp22_vault")]
pub use extensions::vault;
