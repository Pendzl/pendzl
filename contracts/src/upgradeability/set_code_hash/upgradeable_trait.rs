// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use pendzl::traits::Hash;

#[ink::trait_definition]
pub trait SetCodeHash {
    #[ink(message)]
    fn set_code_hash(
        &mut self,
        code_hash: Hash,
    ) -> Result<(), SetCodeHashError>;
}

pub trait SetCodeHashInternal {
    fn _set_code_hash(
        &mut self,
        code_hash: Hash,
    ) -> Result<(), SetCodeHashError>;
}
