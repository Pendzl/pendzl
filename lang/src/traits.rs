// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use ::ink::env::{DefaultEnvironment, Environment};
pub use const_format;
use core::mem::ManuallyDrop;
use ink::storage::traits::{Storable, StorageKey};
pub use pendzl_lang_macro::StorageFieldGetter;
pub use xxhash_rust::const_xxh32::xxh32;

/// Aliases for types of the default environment
pub type AccountId = <DefaultEnvironment as Environment>::AccountId;
pub type Balance = <DefaultEnvironment as Environment>::Balance;
pub type Hash = <DefaultEnvironment as Environment>::Hash;
pub type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
pub type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
pub type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
pub type EnvAccess = ::ink::EnvAccess<'static, DefaultEnvironment>;
pub type String = ink::prelude::string::String;

/// Each object has access to default environment via `Self::env()`.
/// It can be used for interaction with host functions of the blockchain.
pub trait DefaultEnv {
    #[inline(always)]
    fn env() -> EnvAccess {
        Default::default()
    }
}

impl<T> DefaultEnv for T {}

/// DefaultImplementation of the trait means that the type stores some `Data` inside.
/// It is stored in one exemplar, and reference can be retrieved from the object by `get` or
/// `get_mut` methods. The trait is helpful for generics implementations when you don't know
/// precisely the final type, but it is enough for you to know that it has some `Data` inside.
///
/// The trait is used as bound in pendzl to provide a generic implementation for contracts'
/// traits. The user of pendzl can "inherit" the default implementation by implementing the
/// `StorageFieldGetter<Data>` trait.
///
/// In most cases, the trait is implemented automatically by the derive macro.
/// The trait methods should not be used directly. Instead use the `data` method of
/// `StorageAsRef` or `StorageAsMut`.
pub trait StorageFieldGetter<Data>
where
    Self: Flush + StorageAsRef + StorageAsMut + DefaultEnv,
{
    #[deprecated(note = "please use `StorageAsRef::data` instead")]
    fn get(&self) -> &Data;

    #[deprecated(note = "please use `StorageAsMut::data` instead")]
    fn get_mut(&mut self) -> &mut Data;
}

/// Helper trait for `StorageFieldGetter` to provide user-friendly API to retrieve data as reference.
pub trait StorageAsRef {
    #[inline(always)]
    fn data<Data>(&self) -> &Data
    where
        Self: StorageFieldGetter<Data>,
    {
        #[allow(deprecated)]
        <Self as StorageFieldGetter<Data>>::get(self)
    }
}

/// Helper trait for `StorageFieldGetter` to provide user-friendly API to retrieve data as mutable reference.
pub trait StorageAsMut: StorageAsRef {
    #[inline(always)]
    fn data<Data>(&mut self) -> &mut Data
    where
        Self: StorageFieldGetter<Data>,
    {
        #[allow(deprecated)]
        <Self as StorageFieldGetter<Data>>::get_mut(self)
    }
}

impl<T> StorageAsRef for T {}
impl<T: StorageAsRef> StorageAsMut for T {}

/// This trait is automatically implemented for storage structs.
pub trait Flush: Storable + Sized + StorageKey {
    /// Method flushes the current state of `Self` into storage with its `StorageKey`.
    /// So if you want to flush the correct state of the contract,
    /// you have to this method on storage struct.
    fn flush(&self) {
        ink::env::set_contract_storage(&Self::KEY, self);
    }

    /// Method loads the current state of `Self` from storage with its `StorageKey`.
    /// So if you want to load the correct state of the contract,
    /// you have to this method on storage struct.
    fn load(&mut self) {
        let mut state = ink::env::get_contract_storage(&Self::KEY)
            .unwrap_or_else(|error| {
                panic!("Failed to load contract state: {:?}", error)
            })
            .unwrap_or_else(|| panic!("Contract state is not initialized"));
        core::mem::swap(self, &mut state);
        let _ = ManuallyDrop::new(state);
    }
}

impl<T: Storable + Sized + StorageKey> Flush for T {}

/// The value 0 is a valid seed.
const XXH32_SEED: u32 = 0;

pub struct ConstHasher;

impl ConstHasher {
    pub const fn hash(str: &str) -> u32 {
        xxh32(str.as_bytes(), XXH32_SEED)
    }
}
