// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod access;
mod finance;
mod security;
mod token;
mod upgradeability;

#[cfg(feature = "access_control")]
pub use access::access_control;
#[cfg(feature = "ownable")]
pub use access::ownable;

#[cfg(any(
    feature = "general_vest",
    feature = "provide_vest_schedule_info"
))]
pub use finance::general_vest;

#[cfg(feature = "pausable")]
pub use security::pausable;

#[cfg(feature = "psp22")]
pub use token::psp22;
#[cfg(feature = "psp34")]
pub use token::psp34;

#[cfg(feature = "set_code_hash")]
pub use upgradeability::set_code_hash;
