// SPDX-License-Identifier: MIT

#[ink::event]
pub struct Transfer {
    pub from: Option<AccountId>,
    pub to: Option<AccountId>,
    pub id: Id,
}

#[ink::event]
pub struct Approval {
    pub from: AccountId,
    pub to: AccountId,
    pub id: Option<Id>,
    pub approved: bool,
}
