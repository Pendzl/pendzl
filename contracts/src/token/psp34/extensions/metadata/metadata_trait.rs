// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use crate::token::psp34::Id;
use ink::{contract_ref, env::DefaultEnvironment, prelude::string::String};
pub type PSP34MetadataRef = contract_ref!(PSP34Metadata, DefaultEnvironment);

/// trait extending PSP34 with metadata functionality
#[ink::trait_definition]
pub trait PSP34Metadata {
    /// Returns the attribute of `id` for the given `key`.
    ///
    /// If `id` is a collection id of the token, it returns attributes for collection.
    #[ink(message)]
    fn get_attribute(&self, id: Id, key: String) -> Option<String>;
}
/// trait that must be implemented by exactly one storage field of a contract storage
/// so the Pendzl PSP34MetadataInternal and PSP34Metadata implementation can be derived.
pub trait PSP34MetadataStorage {
    fn set_attribute(&mut self, id: &Id, key: &String, value: &String);
}

/// trait that is derived by Pendzl PSP34Metadata implementation macro assuming StorageFieldGetter<PSP34MetadataStorage> is implemented
///
/// functions of this trait are recomended to use while writing ink::messages
pub trait PSP34MetadataInternal {
    fn _set_attribute(&mut self, id: &Id, key: &String, value: &String);
}
