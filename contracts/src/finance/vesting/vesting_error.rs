// SPDX-License-Identifier: MIT

use crate::token::psp22::PSP22Error;
/// Represents errors in vesting-related operations.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum VestingError {
    PSP22Error(PSP22Error),
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
