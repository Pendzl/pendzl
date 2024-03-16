// SPDX-License-Identifier: MIT

use pendzl::traits::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum UpgradeableError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if the upgrade failed
    SetCodeHashFailed,
    PermissionError(String),
}

#[cfg(feature = "ownable")]
use crate::access::ownable::OwnableError;
#[cfg(feature = "ownable")]
impl From<crate::access::ownable::OwnableError> for UpgradeableError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => {
                UpgradeableError::PermissionError(String::from(
                    "O::CallerIsNotOwner",
                ))
            }
            OwnableError::ActionRedundant => UpgradeableError::PermissionError(
                String::from("O::ActionRedundant"),
            ),
        }
    }
}

#[cfg(feature = "access_control")]
use crate::access::access_control::AccessControlError;
#[cfg(feature = "access_control")]
impl From<AccessControlError> for UpgradeableError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => {
                UpgradeableError::PermissionError(String::from(
                    "AC::MissingRole",
                ))
            }
            AccessControlError::RoleRedundant => {
                UpgradeableError::PermissionError(String::from(
                    "AC::RoleRedundant",
                ))
            }
            AccessControlError::InvalidCaller => {
                UpgradeableError::PermissionError(String::from(
                    "AC::InvalidCaller",
                ))
            }
        }
    }
}
