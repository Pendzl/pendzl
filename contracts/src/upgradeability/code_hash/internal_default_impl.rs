use pendzl::traits::DefaultEnv;
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
