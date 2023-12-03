// SPDX-License-Identifier: MIT
#[ink::event]
#[derive(Debug)]
pub struct Transfer {
    #[ink(topic)]
    pub from: Option<AccountId>,
    #[ink(topic)]
    pub to: Option<AccountId>,
    pub value: Balance,
}

#[ink::event]
#[derive(Debug)]
pub struct Approval {
    #[ink(topic)]
    pub owner: AccountId,
    #[ink(topic)]
    pub spender: AccountId,
    pub value: Balance,
}
