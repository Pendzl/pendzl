// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(GeneralVest)]
#[ink::contract]
pub mod t_vester {
    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Vester {
        #[storage_field]
        general_vest: GeneralVestData,
    }

    impl Vester {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }
    }
}
