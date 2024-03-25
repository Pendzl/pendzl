// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use pendzl::math::errors::MathError;
use pendzl::traits::String;

/// The PSP22 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
    /// Returned if recipient's address is zero.
    ZeroRecipientAddress,
    /// Returned if sender's address is zero.
    ZeroSenderAddress,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
    /// Returned if permit signature is invalid
    PermitInvalidSignature,
    /// Returned if permit deadline is expired
    PermitExpired,
}

/// Compatibility with pendzl::math::errors::MathError.
impl From<MathError> for PSP22Error {
    fn from(err: MathError) -> Self {
        match err {
            MathError::Overflow => {
                PSP22Error::Custom(String::from("M::Overflow"))
            }
            MathError::Underflow => {
                PSP22Error::Custom(String::from("M::Underflow"))
            }
            MathError::DivByZero => {
                PSP22Error::Custom(String::from("M::DivByZero"))
            }
        }
    }
}

/// Compatibility with Ownable Errors.
#[cfg(feature = "ownable")]
use crate::access::ownable::OwnableError;
#[cfg(feature = "ownable")]
impl From<OwnableError> for PSP22Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => {
                PSP22Error::Custom(String::from("O::CallerIsNotOwner"))
            }
            OwnableError::ActionRedundant => {
                PSP22Error::Custom(String::from("O::ActionRedundant"))
            }
        }
    }
}

/// Compatibility with AccessControl Errors.
#[cfg(feature = "access_control")]
use crate::access::access_control::AccessControlError;
#[cfg(feature = "access_control")]
impl From<AccessControlError> for PSP22Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => {
                PSP22Error::Custom(String::from("AC::MissingRole"))
            }
            AccessControlError::RoleRedundant => {
                PSP22Error::Custom(String::from("AC::RoleRedundant"))
            }
            AccessControlError::InvalidCaller => {
                PSP22Error::Custom(String::from("AC::InvalidCaller"))
            }
        }
    }
}

/// Compatibility with Pausable Errors.
#[cfg(feature = "pausable")]
use crate::security::pausable::PausableError;
#[cfg(feature = "pausable")]
impl From<PausableError> for PSP22Error {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => {
                PSP22Error::Custom(String::from("P::Paused"))
            }
            PausableError::NotPaused => {
                PSP22Error::Custom(String::from("P::NotPaused"))
            }
        }
    }
}
