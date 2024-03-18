#![cfg_attr(not(feature = "std"), no_std, no_main)]
use ink::prelude::string::{String, ToString};
use pendzl::contracts::ownable::OwnableError;

#[ink::event]
pub struct Flipped {
    #[ink(topic)]
    pub new_value: bool,
}

#[ink::event]
pub struct ModifiedV0Inner {
    #[ink(topic)]
    pub x: bool,
    #[ink(topic)]
    pub y: u128,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FlipperError {
    OwnableError(OwnableError),
    SomeError(String),
    SomeError2,
    SomeError3,
    NewError,
}

impl From<OwnableError> for FlipperError {
    fn from(e: OwnableError) -> Self {
        Self::OwnableError(e)
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct SomeStructInnerView {
    pub x: bool,
    pub y: u128,
}

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct SomeStructInner {
    pub x: bool,
    pub y: u128,
}
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct SomeStructInnerNewFieldView {
    pub ab: bool,
    pub cd: u128,
}

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct SomeStructInnerNewField {
    pub ab: bool,
    pub cd: u128,
}
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct SomeStructView {
    pub a: bool,
    pub inner: SomeStructInnerView,
    pub new_field: SomeStructInnerNewFieldView,
}

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct SomeStruct {
    pub a: bool,
    pub inner: SomeStructInner,
    #[lazy]
    pub new_field: SomeStructInnerNewField,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct NewStructView {
    pub a: bool,
    pub b: u128,
}
#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct NewStruct {
    pub a: bool,
    pub b: u128,
}

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct FlipUpgradeableStorageItem {
    pub val_v0: u128,
    #[lazy]
    pub struct_v0: SomeStruct,
    #[lazy]
    pub struct_v1: NewStruct,
}

#[pendzl::implementation(Ownable, SetCodeHash)]
#[ink::contract]
mod t_flipper {

    use crate::*;

    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Flipper {
        value: bool,
        #[storage_field]
        ownable: OwnableData,
        #[storage_field]
        upgradeable: FlipUpgradeableStorageItem,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn deploy() -> Self {
            //used only to deploy code
            Self::default()
        }

        #[ink(message)]
        pub fn get_val_v0(&self) -> u128 {
            self.upgradeable.val_v0
        }

        #[ink(message)]
        pub fn set_val_v0(&mut self, val: u128) {
            self.upgradeable.val_v0 = val;
        }

        #[ink(message)]
        pub fn set_struct_v0_inner(&mut self, x: bool, y: u128) {
            let mut struct_v0 =
                self.upgradeable.struct_v0.get().unwrap_or_default();
            struct_v0.inner.x = x;
            struct_v0.inner.y = y;
            self.upgradeable.struct_v0.set(&struct_v0);
        }

        #[ink(message)]
        pub fn set_struct_v0_new_field(&mut self, ab: bool, cd: u128) {
            let mut struct_v0 =
                self.upgradeable.struct_v0.get().unwrap_or_default();
            let mut new_field = struct_v0.new_field.get().unwrap_or_default();
            new_field.ab = ab;
            new_field.cd = cd;
            struct_v0.new_field.set(&new_field);
            self.upgradeable.struct_v0.set(&struct_v0);
        }

        #[ink(message)]
        pub fn get_struct_v0(&self) -> SomeStructView {
            let struct_v0 =
                self.upgradeable.struct_v0.get().unwrap_or_default();
            let new_field = struct_v0.new_field.get().unwrap_or_default();
            SomeStructView {
                a: struct_v0.a,
                inner: SomeStructInnerView {
                    x: struct_v0.inner.x,
                    y: struct_v0.inner.y,
                },
                new_field: SomeStructInnerNewFieldView {
                    ab: new_field.ab,
                    cd: new_field.cd,
                },
            }
        }

        #[ink(message)]
        pub fn get_struct_v1(&self) -> NewStructView {
            let struct_v1 =
                self.upgradeable.struct_v1.get().unwrap_or_default();
            NewStructView {
                a: struct_v1.a,
                b: struct_v1.b,
            }
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
            self.env().emit_event(Flipped {
                new_value: self.value,
            });
        }

        #[ink(message)]
        pub fn set_value(&mut self, val: u128) -> Result<(), FlipperError> {
            self._only_owner()?;
            self.upgradeable.val_v0 = val;
            Ok(())
        }

        #[ink(message)]
        pub fn return_value(&mut self) -> Result<u128, FlipperError> {
            Ok(self.upgradeable.val_v0)
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
