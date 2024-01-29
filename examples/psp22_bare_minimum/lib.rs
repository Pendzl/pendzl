// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[pendzl::implementation(PSP22)]
#[ink::contract]
pub mod my_psp22_bare_minimum {
    #[ink(storage)]
    #[derive(StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
        psp22: PSP22Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self {
                psp22: Default::default(),
            };

            instance
                ._mint_to(&Self::env().caller(), &total_supply)
                .expect("Should mint");
            instance
        }
    }
}
