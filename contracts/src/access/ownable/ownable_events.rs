// SPDX-License-Identifier: MIT
#[ink::event]
pub struct OwnershipTransferred {
    #[ink(topic)]
    pub new: Option<AccountId>,
}
