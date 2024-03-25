// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

/// Emitted when a token is transferred.
#[ink::event]
pub struct Transfer {
    /// The account from which the token is transferred. `None` for minting.
    pub from: Option<AccountId>,
    /// The account to which the token is transferred. `None` for burning.
    pub to: Option<AccountId>,
    /// The Id of the token being transferred.
    pub id: Id,
}

/// Emitted when a token approval is granted or revoked.
#[ink::event]
pub struct Approval {
    /// The account granting or revoking approval.
    pub owner: AccountId,
    /// The account being approved or disapproved.
    pub operator: AccountId,
    /// The Id of the token for specific approval. `None` for global approval.
    pub id: Option<Id>,
    /// The approval status.
    pub approved: bool,
}
