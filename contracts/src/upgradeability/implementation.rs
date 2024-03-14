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

#[cfg(feature = "ownable")]
use crate::access::ownable::OwnableInternal;
#[cfg(feature = "ownable")]
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

#[cfg(feature = "access_control")]
use crate::access::access_control::{AccessControlInternal, RoleType};
#[cfg(feature = "access_control")]
pub const CODE_UPGRADER: RoleType = ink::selector_id!("CODE_UPGRADER"); // 1_198_282_211_u32

#[cfg(feature = "access_control")]
pub trait UpgradeableDefaultImpl:
    AccessControlInternal + Sized + UpgradeableInternal
{
    fn set_code_hash_default_impl(
        &mut self,
        code_hash: Hash,
    ) -> Result<(), UpgradeableError> {
        AccessControlInternal::_ensure_has_role(
            self,
            CODE_UPGRADER,
            Some(Self::env().caller()),
        )?;
        UpgradeableInternal::_set_code_hash(self, code_hash)
    }
}

#[cfg(not(any(feature = "access_control", feature = "ownable")))]
pub trait UpgradeableDefaultImpl: Sized + UpgradeableInternal {
    fn set_code_hash_default_impl(
        &mut self,
        code_hash: Hash,
    ) -> Result<(), UpgradeableError> {
        UpgradeableInternal::_set_code_hash(self, code_hash)
    }
}
