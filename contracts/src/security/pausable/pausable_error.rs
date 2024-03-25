// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Represents errors that occur in Pausable operations.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PausableError {
    /// Error indicating the contract is already paused.
    Paused,
    /// Error indicating the contract is not currently paused.
    NotPaused,
}
