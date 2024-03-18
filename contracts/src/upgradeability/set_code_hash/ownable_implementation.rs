use super::SetCodeHashError;
use super::SetCodeHashInternal;
use pendzl::traits::Hash;

use crate::access::ownable::OwnableInternal;

pub trait SetCodeHashDefaultImpl:
    OwnableInternal + Sized + SetCodeHashInternal
{
    fn set_code_hash_default_impl(
        &mut self,
        code_hash: Hash,
    ) -> Result<(), SetCodeHashError> {
        OwnableInternal::_only_owner(self)?;
        SetCodeHashInternal::_set_code_hash(self, code_hash)
    }
}
