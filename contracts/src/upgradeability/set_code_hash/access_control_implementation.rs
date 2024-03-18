use super::{SetCodeHashError, SetCodeHashInternal};
use crate::access::access_control::{AccessControlInternal, RoleType};
use pendzl::traits::{DefaultEnv, Hash};

pub const CODE_UPGRADER: RoleType = ink::selector_id!("CODE_UPGRADER"); // 1_198_282_211_u32

pub trait SetCodeHashDefaultImpl:
    AccessControlInternal + Sized + SetCodeHashInternal
{
    fn set_code_hash_default_impl(
        &mut self,
        code_hash: Hash,
    ) -> Result<(), SetCodeHashError> {
        AccessControlInternal::_ensure_has_role(
            self,
            CODE_UPGRADER,
            Some(Self::env().caller()),
        )?;
        SetCodeHashInternal::_set_code_hash(self, code_hash)
    }
}
