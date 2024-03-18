#![cfg_attr(not(feature = "std"), no_std, no_main)]
use ink::prelude::string::{String, ToString};

#[ink::event]
pub struct Flipped {
    #[ink(topic)]
    pub new_value: bool,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlipperError {
    SomeError(String),
    SomeError2,
    SomeError3,
}

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct SomeStructInner {
    pub x: bool,
    pub y: u128,
}
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct SomeStructInnerView {
    pub x: bool,
    pub y: u128,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct SomeStructView {
    pub a: bool,
    pub inner: SomeStructInnerView,
}

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct SomeStruct {
    pub a: bool,
    pub inner: SomeStructInner,
}

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct FlipUpgradeableStorageItem {
    pub val_v0: u128,
    #[lazy]
    pub struct_v0: SomeStruct,
}

#[pendzl::implementation(AccessControl, SetCodeHash)]
#[ink::contract]
mod t_flipper {

    use crate::*;

    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Flipper {
        value: bool,
        #[storage_field]
        access: AccessControlData,
        #[storage_field]
        upgradeable: FlipUpgradeableStorageItem,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            let mut init_upgradeable = FlipUpgradeableStorageItem {
                val_v0: 1337_u128,
                struct_v0: Default::default(),
            };
            init_upgradeable.struct_v0.set(&SomeStruct {
                a: true,
                inner: SomeStructInner { x: false, y: 42 },
            });
            Self {
                access: AccessControlData::new(Some(Self::env().caller())),
                value: init_value,
                upgradeable: init_upgradeable,
            }
        }

        #[ink(message)]
        pub fn get_val_v0(&self) -> u128 {
            self.upgradeable.val_v0
        }

        #[ink(message)]
        pub fn get_struct_v0(&self) -> SomeStructView {
            let struct_v0 =
                self.upgradeable.struct_v0.get().unwrap_or_default();
            SomeStructView {
                a: struct_v0.a,
                inner: SomeStructInnerView {
                    x: struct_v0.inner.x,
                    y: struct_v0.inner.y,
                },
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
            self.env().emit_event(Flipped {
                new_value: self.value,
            });
        }

        #[ink(message)]
        pub fn return_value(&mut self) -> Result<u128, FlipperError> {
            Ok(5)
        }

        #[ink(message)]
        pub fn return_error(&mut self) -> Result<u128, FlipperError> {
            Err(FlipperError::SomeError("Some error".to_string()))
        }

        #[ink(message)]
        pub fn do_panic(&mut self) {
            panic!("Some error")
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
