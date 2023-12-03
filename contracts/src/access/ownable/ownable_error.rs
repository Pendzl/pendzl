// SPDX-License-Identifier: MIT
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OwnableError {
    CallerIsNotOwner,
    ActionRedundant,
}
