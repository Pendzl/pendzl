// SPDX-License-Identifier: MIT
use pendzl::traits::Hash;

#[ink::trait_definition]
pub trait Upgradeable {
    #[ink(message)]
    fn set_code_hash(
        &mut self,
        code_hash: Hash,
    ) -> Result<(), UpgradeableError>;
}

pub trait UpgradeableInternal {
    fn _set_code_hash(
        &mut self,
        code_hash: Hash,
    ) -> Result<(), UpgradeableError>;
}
