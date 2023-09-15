#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP22, Upgradeable, Ownable)]
#[pendzl::contract]
pub mod contract_v1 {
    use pendzl::traits::Storage;

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            psp22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply).expect("Should mint");
            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());

            instance
        }
    }
}
