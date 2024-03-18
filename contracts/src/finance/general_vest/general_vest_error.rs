// SPDX-License-Identifier: MIT
use crate::token::psp22::PSP22Error;
use ink::prelude::string::String;
use pendzl::math::errors::MathError;
/// Represents errors in general_vest-related operations.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum VestingError {
    PSP22Error(PSP22Error),
    Custom(String),
    InvalidScheduleKey,
    NativeTransferFailed,
    InvalidAmountPaid,
    CouldNotResolveTimeConstraint,
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
