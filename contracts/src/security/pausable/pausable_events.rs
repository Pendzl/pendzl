// SPDX-License-Identifier: MIT

#[ink::event]
pub struct Paused {
    #[ink(topic)]
    pub account: AccountId,
}

#[ink::event]
pub struct Unpaused {
    #[ink(topic)]
    pub account: AccountId,
}
