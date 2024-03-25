// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Emitted when the contract is paused.
#[ink::event]
pub struct Paused {
    /// The account that initiated the pause action.
    #[ink(topic)]
    pub account: AccountId,
}

/// Emitted when the contract is unpaused.
#[ink::event]
pub struct Unpaused {
    /// The account that initiated the unpause action.
    #[ink(topic)]
    pub account: AccountId,
}
