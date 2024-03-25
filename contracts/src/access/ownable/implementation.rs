// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use super::{
    OwnableError, OwnableInternal, OwnableStorage, OwnershipTransferred,
};
use ink::env::DefaultEnvironment;
use pendzl::traits::{AccountId, StorageFieldGetter};

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct OwnableData {
    #[lazy]
    pub owner: Option<AccountId>,
}

impl OwnableData {
    pub fn new(owner: Option<AccountId>) -> Self {
        let mut instance: OwnableData = Default::default();
        if owner.is_some() {
            instance.set_owner(&owner);
            ink::env::emit_event::<DefaultEnvironment, OwnershipTransferred>(
                OwnershipTransferred { new: owner },
            )
        }

        instance
    }
}

impl OwnableStorage for OwnableData {
    fn owner(&self) -> Option<AccountId> {
        self.owner.get().unwrap_or(None)
    }

    fn set_owner(&mut self, new_owner: &Option<AccountId>) {
        self.owner.set(new_owner);
    }
}

pub trait OwnableDefaultImpl: OwnableInternal {
    fn owner_default_impl(&self) -> Option<AccountId> {
        self._owner()
    }

    fn renounce_ownership_default_impl(&mut self) -> Result<(), OwnableError> {
        self._only_owner()?;
        self._update_owner(&None);
        Ok(())
    }

    fn transfer_ownership_default_impl(
        &mut self,
        new_owner: AccountId,
    ) -> Result<(), OwnableError> {
        self._only_owner()?;
        self._update_owner(&Some(new_owner));
        Ok(())
    }
}

pub trait OwnableInternalDefaultImpl: StorageFieldGetter<OwnableData>
where
    OwnableData: OwnableStorage,
{
    fn _owner_default_impl(&self) -> Option<AccountId> {
        self.data().owner()
    }
    fn _update_owner_default_impl(&mut self, new: &Option<AccountId>) {
        self.data().set_owner(new);
        Self::env().emit_event(OwnershipTransferred { new: *new });
    }

    fn _only_owner_default_impl(&self) -> Result<(), OwnableError> {
        if Some(Self::env().caller()) != self.data().owner.get_or_default() {
            return Err(OwnableError::CallerIsNotOwner);
        }
        Ok(())
    }
}
