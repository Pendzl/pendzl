// SPDX-License-Identifier: MIT
use crate::token::psp22::PSP22Error;
use pendzl::math::errors::MathError;
/// Represents errors in general_vest-related operations.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum VestingError {
    PSP22Error(PSP22Error),
    InvalidScheduleKey,
    NativeTransferFailed,
    InvalidAmountPaid,
    CouldNotResolveTimeConstraint,
    MathError(MathError),
}

impl From<PSP22Error> for VestingError {
    fn from(error: PSP22Error) -> Self {
        VestingError::PSP22Error(error)
    }
}
