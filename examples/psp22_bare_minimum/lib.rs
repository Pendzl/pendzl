// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// Contract Summary:
/// A simple PSP22 contract.
// ########################################################
// inject PSP22 trait's default implementation (PSP22DefaultImpl & PSP22InternalDefaultImpl)
// which reduces the amount of boilerplate code required to implement trait messages drastically
#[pendzl::implementation(PSP22)]
#[ink::contract]
pub mod my_psp22_bare_minimum {
    #[ink(storage)]
    // derive explained below
    #[derive(StorageFieldGetter)]
    pub struct Contract {
        // apply the storage_field attribute so it's accessible via `self.data::<PSP22>()` (provided by StorageFieldGetter derive)
        #[storage_field]
        // PSP22Data is a struct that implements PSP22Storage - required by PSP22InternalDefaultImpl trait
        // note it's not strictly required by PSP22 trait - just the default implementation
        // name of the field is arbitrary
        psp22: PSP22Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self {
                psp22: Default::default(),
            };

            // mint total_supply to the caller using _mint_to from PSP22Internal (implemented by PSP22DefaultImpl)
            instance
                ._mint_to(&Self::env().caller(), &total_supply)
                .expect("Should mint");
            instance
        }
    }
}
