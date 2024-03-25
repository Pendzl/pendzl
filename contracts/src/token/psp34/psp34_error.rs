// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use pendzl::math::errors::MathError;
use pendzl::traits::String;

/// The PSP34 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP34Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if owner approves self
    SelfApprove,
    /// Returned if the caller doesn't have allowance for transferring.
    NotApproved,
    /// Returned if the owner already own the token.
    TokenExists,
    /// Returned if  the token doesn't exist
    TokenNotExists,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
}

#[cfg(feature = "ownable")]
use crate::access::ownable::OwnableError;
#[cfg(feature = "ownable")]
impl From<OwnableError> for PSP34Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => {
                PSP34Error::Custom(String::from("O::CallerIsNotOwner"))
            }
            OwnableError::ActionRedundant => {
                PSP34Error::Custom(String::from("O::ActionRedundant"))
            }
        }
    }
}

#[cfg(feature = "access_control")]
use crate::access::access_control::AccessControlError;
#[cfg(feature = "access_control")]
impl From<AccessControlError> for PSP34Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => {
                PSP34Error::Custom(String::from("AC::MissingRole"))
            }
            AccessControlError::RoleRedundant => {
                PSP34Error::Custom(String::from("AC::RoleRedundant"))
            }
            AccessControlError::InvalidCaller => {
                PSP34Error::Custom(String::from("AC::InvalidCaller"))
            }
        }
    }
}

#[cfg(feature = "pausable")]
use crate::security::pausable::PausableError;
#[cfg(feature = "pausable")]
impl From<PausableError> for PSP34Error {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => {
                PSP34Error::Custom(String::from("P::Paused"))
            }
            PausableError::NotPaused => {
                PSP34Error::Custom(String::from("P::NotPaused"))
            }
        }
    }
}

impl From<MathError> for PSP34Error {
    fn from(err: MathError) -> Self {
        match err {
            MathError::Overflow => {
                PSP34Error::Custom(String::from("M::Overflow"))
            }
            MathError::Underflow => {
                PSP34Error::Custom(String::from("M::Underflow"))
            }
            MathError::DivByZero => {
                PSP34Error::Custom(String::from("M::DivByZero"))
            }
        }
    }
}
