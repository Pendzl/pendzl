// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use ink::{contract_ref, env::DefaultEnvironment, prelude::string::String};

pub type PSP22MetadataRef = contract_ref!(PSP22Metadata, DefaultEnvironment);
/// trait extending PSP22 with metadata functionality
#[ink::trait_definition]
pub trait PSP22Metadata {
    /// Returns the token name.
    #[ink(message)]
    fn token_name(&self) -> Option<String>;

    /// Returns the token symbol.
    #[ink(message)]
    fn token_symbol(&self) -> Option<String>;

    /// Returns the token decimals.
    #[ink(message)]
    fn token_decimals(&self) -> u8;
}

/// trait that must be implemented by exactly one storage field of a contract storage
/// so the Pendzl PSP22Metadata implementation can be derived for a non-PSP22Vault contract.
pub trait PSP22MetadataStorage {
    fn token_name(&self) -> Option<String>;

    fn token_symbol(&self) -> Option<String>;

    fn token_decimals(&self) -> u8;
}

/// trait that must be implemented by exactly one storage field of a contract storage
/// so the Pendzl PSP22Metadata implementation can be derived for a PSP22Vault contract.

pub trait PSP22VaultMetadataStorage {
    fn token_name(&self) -> Option<String>;

    fn token_symbol(&self) -> Option<String>;
}
