// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(Pausable)]
#[ink::contract]
pub mod my_pausable {
    // use pendzl::contracts::security::pausable::PausableError;
    // use pendzl::contracts::security::pausable::PausableInternal;

    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
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
            self._ensure_not_paused()?;
            self.flipped = !self.flipped;
            Ok(())
        }

        #[ink(message)]
        pub fn pause(&mut self) -> Result<(), PausableError> {
            self._pause()
        }

        #[ink(message)]
        pub fn unpause(&mut self) -> Result<(), PausableError> {
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
        async fn success_flip_when_not_paused(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_pausable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(method_call!(client, contract, flip), Ok(()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn success_pause_when_not_paused(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_pausable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(method_call!(client, contract, pause), Ok(()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn success_flip(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_pausable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(method_call!(client, contract, flip), Ok(()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn failed_double_pause(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_pausable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(method_call!(client, contract, pause), Ok(()));
            assert!(matches!(
                method_call_dry_run!(client, contract, pause),
                Err(_)
            ));

            Ok(())
        }

        #[ink_e2e::test]
        async fn success_pause_and_unpause(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_pausable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(method_call!(client, contract, pause), Ok(()));
            assert_eq!(method_call!(client, contract, unpause), Ok(()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn failed_unpause(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_pausable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert!(matches!(
                method_call_dry_run!(client, contract, unpause),
                Err(_)
            ));

            Ok(())
        }

        #[ink_e2e::test]
        async fn failed_flip_when_paused(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_pausable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(method_call!(client, contract, pause), Ok(()));
            assert!(matches!(
                method_call_dry_run!(client, contract, flip),
                Err(_)
            ));

            Ok(())
        }
    }
}
