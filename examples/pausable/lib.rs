// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// Contract Summary:
/// A flipper contract with pausable module.
/// A flip can happen only if contract is not paused.
// ########################################################
// inject Pausable trait's default implementation (PausableDefaultImpl & PausableInternalDefaultImpl)
// which reduces the amount of boilerplate code required to implement trait messages drastically
#[pendzl::implementation(Pausable)]
#[ink::contract]
pub mod my_pausable {
    #[ink(storage)]
    // derive explained below
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        // apply the storage_field attribute so it's accessible via `self.data::<Pausable>()` (provided by StorageFieldGetter derive)
        #[storage_field]
        // PausableData is a struct that implements PausableStorage - required by PausableInternalDefaultImpl trait
        // note it's not strictly required by Pausable trait - just the default implementation
        // name of the field is arbitrary
        pause: PausableData,
        flipped: bool,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn flip(&mut self) -> Result<(), PausableError> {
            // ensure the contract is not paused by using _ensure_not_paused from PausableInternal (implemented by PausableDefaultImpl)
            self._ensure_not_paused()?;
            self.flipped = !self.flipped;
            Ok(())
        }

        #[ink(message)]
        pub fn pause(&mut self) -> Result<(), PausableError> {
            // use _pause to pause the contract from PausableInternal (implemented by PausableDefaultImpl)
            self._pause()
        }

        #[ink(message)]
        pub fn unpause(&mut self) -> Result<(), PausableError> {
            // use _unpause to unpause the contract from PausableInternal (implemented by PausableDefaultImpl)
            self._unpause()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::ContractsBackend;
        use ink_e2e::alice;

        use test_helpers::{method_call, method_call_dry_run};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn success_flip_when_not_paused(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_pausable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            assert_eq!(method_call!(client, contract, flip), Ok(()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn failed_flip_when_paused(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_pausable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            assert_eq!(method_call!(client, contract, pause), Ok(()));
            assert!(matches!(
                method_call_dry_run!(client, contract, flip),
                Err(_)
            ));

            Ok(())
        }
    }
}
