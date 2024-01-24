// SPDX-License-Identifier: MIT
use crate::token::psp34::Id;
use ink::{prelude::string::String, storage::Mapping};
use pendzl::traits::Storage;

use super::{AttribiuteSet, PSP34MetadataStorage};
use ink::prelude::string::ToString;

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct PSP34MetadataData {
    pub attributes: Mapping<(Id, String), String>,
}

impl PSP34MetadataStorage for PSP34MetadataData {
    fn set_attribute(&mut self, id: &Id, key: &String, value: &String) {
        self.attributes
            .insert(&(id.to_id(), key.to_string()), value);
    }
}

pub trait PSP34MetadataDefaultImpl: Storage<PSP34MetadataData> {
    fn get_attribute_default_impl(&self, id: Id, key: String) -> Option<String> {
        self.data().attributes.get(&(id, key))
    }
}

pub trait PSP34MetadataInternalDefaultImpl: Storage<PSP34MetadataData> {
    fn _set_attribute_default_impl(&mut self, id: &Id, key: &String, value: &String) {
        self.data().set_attribute(id, key, value);

        Self::env().emit_event(AttribiuteSet {
            id: id.to_id(),
            key: key.to_string(),
            data: value.to_string(),
        })
    }
}
