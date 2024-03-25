// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Emitted when vested tokens are released
#[ink::event]
#[derive(Debug)]
pub struct TokenReleased {
    /// The account that triggered the release.
    #[ink(topic)]
    pub caller: AccountId,
    /// The account to which the tokens are sent.
    #[ink(topic)]
    pub to: AccountId,
    /// The locked asset.
    #[ink(topic)]
    pub asset: Option<AccountId>,
    /// The amount of tokens released.
    pub amount: Balance,
}
/// Emitted when general_vest schedule is created
#[ink::event]
#[derive(Debug)]
pub struct VestingScheduled {
    // creator of the general_vest schedule
    #[ink(topic)]
    pub creator: AccountId,
    /// The locked asset.
    #[ink(topic)]
    pub asset: Option<AccountId>,
    /// The account to which the tokens will be sent.
    #[ink(topic)]
    pub receiver: AccountId,
    /// The amount of tokens released.
    pub amount: Balance,
    // The general_vest schedule.
    pub schedule: VestingSchedule,
}
