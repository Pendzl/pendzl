use super::UpgradeableError;
use super::UpgradeableInternal;
use pendzl::traits::Hash;

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
