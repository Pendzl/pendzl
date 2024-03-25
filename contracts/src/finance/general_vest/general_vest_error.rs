// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use crate::token::psp22::PSP22Error;
use ink::prelude::string::String;
use pendzl::math::errors::MathError;
/// Represents errors in general_vest-related operations.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum VestingError {
    /// Custom error type for unpredicted cases for custom implementation
    Custom(String),
    /// Returned if transfer of PSP22 token fails during creating vest or releasing tokens.
    PSP22Error(PSP22Error),
    /// Returned if transfer of native token fails during creating vest or releasing tokens.
    NativeTransferFailed,
    /// Returned if the amount paid is invalid. (applicable only to native tokens)
    InvalidAmountPaid,
}

impl From<PSP22Error> for VestingError {
    fn from(error: PSP22Error) -> Self {
        VestingError::PSP22Error(error)
    }
}

impl From<MathError> for VestingError {
    fn from(err: MathError) -> Self {
        match err {
            MathError::Overflow => {
                VestingError::Custom(String::from("M::Overflow"))
            }
            MathError::Underflow => {
                VestingError::Custom(String::from("M::Underflow"))
            }
            MathError::DivByZero => {
                VestingError::Custom(String::from("M::DivByZero"))
            }
        }
    }
}
