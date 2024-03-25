// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use pendzl::traits::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum SetCodeHashError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if the upgrade failed
    SetCodeHashFailed,
    PermissionError(String),
}

#[cfg(feature = "ownable")]
use crate::access::ownable::OwnableError;
#[cfg(feature = "ownable")]
impl From<crate::access::ownable::OwnableError> for SetCodeHashError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => {
                SetCodeHashError::PermissionError(String::from(
                    "O::CallerIsNotOwner",
                ))
            }
            OwnableError::ActionRedundant => SetCodeHashError::PermissionError(
                String::from("O::ActionRedundant"),
            ),
        }
    }
}

#[cfg(feature = "access_control")]
use crate::access::access_control::AccessControlError;
#[cfg(feature = "access_control")]
impl From<AccessControlError> for SetCodeHashError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => {
                SetCodeHashError::PermissionError(String::from(
                    "AC::MissingRole",
                ))
            }
            AccessControlError::RoleRedundant => {
                SetCodeHashError::PermissionError(String::from(
                    "AC::RoleRedundant",
                ))
            }
            AccessControlError::InvalidCaller => {
                SetCodeHashError::PermissionError(String::from(
                    "AC::InvalidCaller",
                ))
            }
        }
    }
}
