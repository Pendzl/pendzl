// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Emitted when ownership of the contract is transferred.
#[ink::event]
pub struct OwnershipTransferred {
    /// The new owner's account address. `None` if ownership is renounced.
    #[ink(topic)]
    pub new: Option<AccountId>,
}
