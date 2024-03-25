// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Emitted when tokens are transferred, including zero value transfers.
#[ink::event]
#[derive(Debug)]
pub struct Transfer {
    /// The account from which the tokens are transferred. `None` for minting operations.
    #[ink(topic)]
    pub from: Option<AccountId>,
    /// The account to which the tokens are transferred. `None` for burning operations.
    #[ink(topic)]
    pub to: Option<AccountId>,
    /// The amount of tokens transferred.
    pub value: Balance,
}

/// Emitted when the allowance of a `spender` for an `owner` is set or changed.
#[ink::event]
#[derive(Debug)]
pub struct Approval {
    /// The account of the token owner.
    #[ink(topic)]
    pub owner: AccountId,
    /// The account of the authorized spender.
    #[ink(topic)]
    pub spender: AccountId,
    /// The new allowance amount.
    pub value: Balance,
}
