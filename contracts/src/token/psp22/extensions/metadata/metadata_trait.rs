// SPDX-License-Identifier: MIT

use ink::{contract_ref, env::DefaultEnvironment, prelude::string::String};

pub type PSP22MetadataRef = contract_ref!(PSP22Metadata, DefaultEnvironment);
/// Trait that contains metadata
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
