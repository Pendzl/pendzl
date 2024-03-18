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
