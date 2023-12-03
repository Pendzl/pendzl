// SPDX-License-Identifier: MIT
#[ink::event]
pub struct RoleAdminChanged {
    #[ink(topic)]
    pub role: RoleType,
    pub previous: RoleType,
    pub new: RoleType,
}

#[ink::event]
pub struct RoleGranted {
    #[ink(topic)]
    pub role: RoleType,
    #[ink(topic)]
    pub grantee: Option<AccountId>,
    #[ink(topic)]
    pub grantor: Option<AccountId>,
}

#[ink::event]
pub struct RoleRevoked {
    #[ink(topic)]
    pub role: RoleType,
    #[ink(topic)]
    pub account: Option<AccountId>,
    #[ink(topic)]
    pub sender: AccountId,
}
