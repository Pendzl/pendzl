// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Emitted when the admin role for a specific role is changed.
///
/// This event is triggered whenever a `role`'s admin role is updated.
/// It logs the `role` being modified, the `previous` admin role, and the `new` admin role set for that `role`.
#[ink::event]
pub struct RoleAdminChanged {
    /// The `RoleType` for which the admin role is changed. This is the role being modified.
    #[ink(topic)]
    pub role: RoleType,
    /// The `RoleType` representing the previous admin role for the `role`. Indicates the admin role before the change.
    pub previous: RoleType,
    /// The `RoleType` representing the new admin role set for the `role`. Indicates the updated admin role.
    pub new: RoleType,
}

/// Emitted when a role is granted to an account.
///
/// This event occurs when a new `role` is assigned to an `grantee`.
/// The `grantor` who assigned the role is also logged.
#[ink::event]
pub struct RoleGranted {
    /// The `RoleType` that is granted. This field identifies the specific role being assigned.
    #[ink(topic)]
    pub role: RoleType,
    /// The `AccountId` of the account receiving the `role`. Represents the beneficiary of the role assignment.
    #[ink(topic)]
    pub grantee: Option<AccountId>,
    /// The `AccountId` of the account that granted the `role`. This is the account responsible for the role assignment.
    #[ink(topic)]
    pub grantor: Option<AccountId>,
}

/// Emitted when a role is revoked from an account.
///
/// This event is triggered when an existing `role` is removed from an `account`.
/// The `sender` who performed the revocation is also included.
#[ink::event]
pub struct RoleRevoked {
    /// The `RoleType` that is revoked. Specifies the role being removed from the account.
    #[ink(topic)]
    pub role: RoleType,
    /// The `AccountId` of the account from which the `role` is being removed. Denotes the account losing the role.
    #[ink(topic)]
    pub account: Option<AccountId>,
    /// The `AccountId` of the account that performed the role revocation. Indicates who initiated the removal of the role.
    #[ink(topic)]
    pub sender: AccountId,
}
