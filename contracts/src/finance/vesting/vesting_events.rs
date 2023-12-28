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
/// Emitted when vesting schedule is created
#[ink::event]
#[derive(Debug)]
pub struct VestingScheduled {
    // creator of the vesting schedule
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
    // The vesting start time.
    pub vesting_start: Timestamp,
    // The vesting end time.
    pub vesting_end: Timestamp,
}
