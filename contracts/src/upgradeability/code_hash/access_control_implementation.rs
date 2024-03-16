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

use crate::access::access_control::{AccessControlInternal, RoleType};
pub const CODE_UPGRADER: RoleType = ink::selector_id!("CODE_UPGRADER"); // 1_198_282_211_u32

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
