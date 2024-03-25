// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use ink::primitives::AccountId;

pub type RoleType = u32;
pub const DEFAULT_ADMIN_ROLE: RoleType = 0;

use ink::{contract_ref, env::DefaultEnvironment};
pub type AccessControlRef = contract_ref!(AccessControl, DefaultEnvironment);

/// AccessControl trait that provides a framework for implementing role-based
/// access control mechanisms in smart contracts. This lightweight version is
/// designed without the neccesity to enumerate role members directly within the
/// contract. Role membership can instead be tracked off-chain through the
/// analysis of contract event logs.
///
/// Roles can be granted and revoked dynamically via the `grant_role` and
/// `revoke_role`. functions. Each role has an associated admin role, and only
/// accounts that have a role's admin role can call `grant_role` and `revoke_role`.
#[ink::trait_definition]
pub trait AccessControl {
    /// Returns `true` if `account` has been granted `role`.
    #[ink(message)]
    fn has_role(&self, role: RoleType, address: Option<AccountId>) -> bool;

    /// Returns the admin role that controls `role`. See `grant_role` and `revoke_role`.
    #[ink(message)]
    fn get_role_admin(&self, role: RoleType) -> RoleType;

    /// Grants `role` to `account`.
    ///
    /// On success a `RoleGranted` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `MissingRole` error if caller can't grant the role.
    /// Returns with `RoleRedundant` error `account` has `role`.
    #[ink(message)]
    fn grant_role(
        &mut self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> Result<(), AccessControlError>;

    /// Revokes `role` from `account`.
    ///
    /// On success a `RoleRevoked` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `MissingRole` error if caller can't grant the `role` or if `account` doesn't have `role`.
    #[ink(message)]
    fn revoke_role(
        &mut self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> Result<(), AccessControlError>;

    /// Revokes `role` from the calling account.
    /// Roles are often managed via `grant_role` and `revoke_role`: this function's
    /// purpose is to provide a mechanism for accounts to lose their privileges
    /// if they are compromised (such as when a trusted device is misplaced).
    ///
    /// On success a `RoleRevoked` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `InvalidCaller` error if caller is not `account`.
    /// Returns with `MissingRole` error if `account` doesn't have `role`.
    #[ink(message)]
    fn renounce_role(
        &mut self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> Result<(), AccessControlError>;

    /// Sets a admin of a specific `role` to as admin role.
    #[ink(message)]
    fn set_role_admin(
        &mut self,
        role: RoleType,
        new_admin: RoleType,
    ) -> Result<(), AccessControlError>;
}

/// trait that must be implemented by exactly one storage field of a contract storage
/// so the Pendzl AccessControlInternal and AccessControl implementation can be derived.
pub trait AccessControlStorage {
    /// Checks if `account` has the specified `role`.
    fn has_role(&self, role: RoleType, account: &Option<AccountId>) -> bool;

    /// Assigns the `role` to `account`.
    fn add(&mut self, role: RoleType, account: &Option<AccountId>);

    /// Removes the `role` from `account`.
    fn remove(&mut self, role: RoleType, account: &Option<AccountId>);

    /// Retrieves the admin role for the given `role`.
    fn get_role_admin(&self, role: RoleType) -> Option<RoleType>;

    /// Sets the admin role for a specific `role` to `new_admin`.
    fn set_role_admin(&mut self, role: RoleType, new_admin: RoleType);
}

/// trait that is derived by Pendzl AccessControl implementation macro assuming StorageFieldGetter<AccessControlStorage> is implemented
///
/// functions of this trait are recomended to use while writing ink::messages
pub trait AccessControlInternal {
    // returns the default admin role
    fn _default_admin() -> RoleType;

    /// Returns `true` if `account` has been granted `role`.
    fn _has_role(&self, role: RoleType, account: Option<AccountId>) -> bool;

    // Grants `role` to `account`.
    ///
    /// On success a `RoleGranted` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `MissingRole` error if caller can't grant the role.
    /// Returns with `RoleRedundant` error `account` has `role`.
    fn _grant_role(
        &mut self,
        role: RoleType,
        member: Option<AccountId>,
    ) -> Result<(), AccessControlError>;

    /// Revokes `role` from `account`.
    ///
    /// On success a `RoleRevoked` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `MissingRole` error if the `account` doesn't have `role`.
    fn _do_revoke_role(
        &mut self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> Result<(), AccessControlError>;

    /// Returns the admin role that controls `role`. See `grant_role` and `revoke_role`.
    fn _get_role_admin(&self, role: RoleType) -> RoleType;

    /// Sets a admin of a specific `role` to `new_admin`.
    ///
    /// # Errors
    ///
    /// Returns with `MissingRole` error if caller is not admin.
    fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType);

    /// Ensures that a `account`` has a specific `role``.
    ///
    /// # Errors
    ///
    /// Returns `MissingRole` if the account does not have the specified role.
    fn _ensure_has_role(
        &self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> Result<(), AccessControlError>;
}
