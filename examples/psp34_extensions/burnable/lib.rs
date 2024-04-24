// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// An PSP34 contract that allows anyone to burn any token.
/// On construction create 3 tokens with ids 0, 1, 2 and mint them to the caller.
// inject PSP34 trait's default implementation (PSP34DefaultImpl & PSP34InternalDefaultImpl)
// and PSP34Burnable trait's default implementation (PSP34BurnableDefaultImpl)
// which reduces the amount of boilerplate code required to implement trait messages drastically
#[pendzl::implementation(PSP34, PSP34Burnable)]
#[ink::contract]
pub mod my_psp34_burnable {
    use pendzl::contracts::psp34::*;
    #[ink(storage)]
    // derive explained below
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        // apply the storage_field attribute so it's accessible via `self.data::<PSP34>()` (provided by StorageFieldGetter derive)
        #[storage_field]
        // PSP34Data is a struct that implements PSP34Storage - required by PSP34InternalDefaultImpl trait
        // note it's not strictly required by PSP34 trait - just the default implementation
        // name of the field is arbitrary
        psp34: PSP34Data,
    }

    impl Contract {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            // mint 3 tokens to the caller using _mint_to from PSP34Internal (implemented by PSP34DefaultImpl)
            instance
                ._mint_to(&Self::env().caller(), &Id::U8(0u8))
                .expect("Should mint token with id 0");
            instance
                ._mint_to(&Self::env().caller(), &Id::U8(1u8))
                .expect("Should mint token with id 1");
            instance
                ._mint_to(&Self::env().caller(), &Id::U8(2u8))
                .expect("Should mint token with id 2");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::AccountKeyring::Alice;

        use ink_e2e::account_id;
        use ink_e2e::ContractsBackend;
        use test_helpers::balance_of;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn burn_works(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate(
                    "my_psp34_burnable",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            assert_eq!(balance_of!(client, contract, Alice), 3);

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.burn(account_id(Alice), Id::U8(0u8)),
                )
                .submit()
                .await
                .expect("call failed")
                .return_value();

            assert_eq!(result, Ok(()));
            assert_eq!(balance_of!(client, contract, Alice), 2);

            Ok(())
        }

        #[ink_e2e::test]
        async fn burn_from_works(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate(
                    "my_psp34_burnable",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            assert_eq!(balance_of!(client, contract, Alice), 3);

            let result = client
                .call(
                    &ink_e2e::bob(),
                    &contract.burn(account_id(Alice), Id::U8(0u8)),
                )
                .submit()
                .await
                .expect("call failed")
                .return_value();

            assert_eq!(result, Ok(()));
            assert_eq!(balance_of!(client, contract, Alice), 2);

            Ok(())
        }
    }
}
