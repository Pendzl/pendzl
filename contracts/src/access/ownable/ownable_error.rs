// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Represents errors in ownership-related operations.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OwnableError {
    /// Error when the caller is not the current owner.
    CallerIsNotOwner,
    /// Error when an action is redundant, such as transferring ownership to the current owner.
    ActionRedundant,
}
