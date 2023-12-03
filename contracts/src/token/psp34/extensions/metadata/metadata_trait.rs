// SPDX-License-Identifier: MIT
use crate::token::psp34::Id;
use ink::{contract_ref, env::DefaultEnvironment, prelude::string::String};
pub type PSP34MetadataRef = contract_ref!(PSP34Metadata, DefaultEnvironment);

#[ink::trait_definition]
pub trait PSP34Metadata {
    /// Returns the attribute of `id` for the given `key`.
    ///
    /// If `id` is a collection id of the token, it returns attributes for collection.
    #[ink(message)]
    fn get_attribute(&self, id: Id, key: String) -> Option<String>;
}

pub trait PSP34MetadataInternal {
    fn _set_attribute(&mut self, id: &Id, key: &String, value: &String);
}

pub trait PSP34MetadataStorage {
    fn set_attribute(&mut self, id: &Id, key: &String, value: &String);
}
