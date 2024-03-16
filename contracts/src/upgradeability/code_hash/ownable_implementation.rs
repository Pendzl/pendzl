use super::UpgradeableError;
use super::UpgradeableInternal;
use pendzl::traits::DefaultEnv;
use pendzl::traits::Hash;

pub trait UpgradeableInternalDefaultImpl: Sized {
    fn _set_code_hash_default_impl(
        &mut self,
        code_hash: Hash,
    ) -> Result<(), UpgradeableError> {
        match Self::env().set_code_hash(&code_hash) {
            Ok(_) => {}
            Err(_) => return Err(UpgradeableError::SetCodeHashFailed),
        }
        Ok(())
    }
}

use crate::access::ownable::OwnableInternal;

pub trait UpgradeableDefaultImpl:
    OwnableInternal + Sized + UpgradeableInternal
{
    fn set_code_hash_default_impl(
        &mut self,
        code_hash: Hash,
    ) -> Result<(), UpgradeableError> {
        OwnableInternal::_only_owner(self)?;
        UpgradeableInternal::_set_code_hash(self, code_hash)
    }
}

pub trait UpgradeableDefaultImpl: Sized + UpgradeableInternal {
    fn set_code_hash_default_impl(
        &mut self,
        code_hash: Hash,
    ) -> Result<(), UpgradeableError> {
        UpgradeableInternal::_set_code_hash(self, code_hash)
    }
}
