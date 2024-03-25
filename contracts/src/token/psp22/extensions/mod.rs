// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
#[cfg(feature = "psp22_burnable")]
pub mod burnable;
#[cfg(any(feature = "psp22_metadata", feature = "psp22_vault_metadata"))]
pub mod metadata;
#[cfg(feature = "psp22_mintable")]
pub mod mintable;
#[cfg(feature = "psp22_vault")]
pub mod vault;
