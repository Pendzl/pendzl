// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// The errors that can occur during access control operations.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AccessControlError {
    /// The caller is not allowed to perform the operation.
    InvalidCaller,
    /// The role is missing.
    MissingRole,
    /// The role is redundant.
    RoleRedundant,
}
