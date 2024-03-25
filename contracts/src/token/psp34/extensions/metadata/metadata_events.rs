// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
#[ink::event]
pub struct AttribiuteSet {
    #[ink(topic)]
    id: Id,
    key: String,
    data: String,
}
