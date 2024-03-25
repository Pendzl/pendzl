// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
/// Represents a deposit event in the Vault contract.
#[ink::event]
pub struct Deposit {
    #[ink(topic)]
    pub sender: AccountId,
    #[ink(topic)]
    pub owner: AccountId,
    pub assets: Balance,
    pub shares: Balance,
}

/// Represents a withdraw event in the Vault contract.
#[ink::event]
pub struct Withdraw {
    #[ink(topic)]
    pub sender: AccountId,
    #[ink(topic)]
    pub receiver: AccountId,
    #[ink(topic)]
    pub owner: AccountId,
    pub assets: Balance,
    pub shares: Balance,
}
