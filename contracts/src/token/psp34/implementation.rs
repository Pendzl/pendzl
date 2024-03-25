// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::token::psp34::{
    Approval, Id, PSP34Error, PSP34Internal, PSP34Storage, Transfer,
};
use ink::{prelude::vec::Vec, primitives::AccountId, storage::Mapping};
use pendzl::math::errors::MathError;
use pendzl::traits::{DefaultEnv, StorageFieldGetter};

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct PSP34Data {
    owner_of: Mapping<Id, AccountId>,
    allowances: Mapping<(AccountId, AccountId, Option<Id>), ()>,
    owned_tokens_count: Mapping<AccountId, u32>,
    #[lazy]
    total_supply: u64,
}

impl PSP34Storage for PSP34Data {
    fn balance_of(&self, owner: &AccountId) -> u32 {
        self.owned_tokens_count.get(owner).unwrap_or(0)
    }

    fn total_supply(&self) -> u64 {
        self.total_supply.get().unwrap_or(0)
    }

    fn owner_of(&self, id: &Id) -> Option<AccountId> {
        self.owner_of.get(id)
    }

    fn allowance(
        &self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Option<Id>,
    ) -> bool {
        self.allowances.get(&(*owner, *operator, None)).is_some()
            || (id.is_some()
                && self
                    .allowances
                    .get(&(*owner, *operator, id.as_ref().map(|v| v.clone())))
                    .is_some())
    }

    fn set_operator_approval(
        &mut self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Option<Id>,
        approved: &bool,
    ) {
        if *approved {
            self.allowances.insert(
                &(*owner, *operator, id.as_ref().map(|v| v.clone())),
                &(),
            );
        } else {
            self.allowances.remove(&(
                *owner,
                *operator,
                id.as_ref().map(|v| v.clone()),
            ));
        }
    }

    fn insert_token_owner(
        &mut self,
        id: &Id,
        to: &AccountId,
    ) -> Result<(), PSP34Error> {
        if self.owner_of.get(id).is_some() {
            return Err(PSP34Error::TokenExists);
        }
        self.owner_of.insert(id, to);

        let balance = self.owned_tokens_count.get(to).unwrap_or(0);
        self.owned_tokens_count
            .insert(to, &(balance.checked_add(1).ok_or(MathError::Overflow)?));

        let total_suply = self.total_supply.get().unwrap_or(0);
        self.total_supply
            .set(&(total_suply.checked_add(1).ok_or(MathError::Overflow)?));

        Ok(())
    }

    fn remove_token_owner(
        &mut self,
        id: &Id,
        from: &AccountId,
    ) -> Result<(), PSP34Error> {
        match self.owner_of.get(id) {
            Some(v) => {
                if v != *from {
                    return Err(PSP34Error::NotApproved);
                }
            }
            None => return Err(PSP34Error::TokenNotExists),
        };
        self.owner_of.remove(id);
        let balance = self.owned_tokens_count.get(from).unwrap_or(0);
        self.owned_tokens_count.insert(from, &(balance - 1));

        let total_suply = self.total_supply.get().unwrap();
        self.total_supply.set(&(total_suply - 1));
        Ok(())
    }
}

pub trait PSP34DefaultImpl: PSP34Internal + DefaultEnv {
    fn collection_id_default_impl(&self) -> Id {
        let account_id = Self::env().account_id();
        Id::Bytes(<_ as AsRef<[u8; 32]>>::as_ref(&account_id).to_vec())
    }

    fn balance_of_default_impl(&self, owner: AccountId) -> u32 {
        self._balance_of(&owner)
    }

    fn owner_of_default_impl(&self, id: Id) -> Option<AccountId> {
        self._owner_of(&id)
    }

    fn total_supply_default_impl(&self) -> u64 {
        self._total_supply()
    }

    fn allowance_default_impl(
        &self,
        owner: AccountId,
        operator: AccountId,
        id: Option<Id>,
    ) -> bool {
        self._allowance(&owner, &operator, &id)
    }

    fn approve_default_impl(
        &mut self,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    ) -> Result<(), PSP34Error> {
        let caller = Self::env().caller();
        self._approve(&caller, &operator, &id, &approved)
    }

    fn transfer_default_impl(
        &mut self,
        to: AccountId,
        id: Id,
        data: Vec<u8>,
    ) -> Result<(), PSP34Error> {
        if let Some(owner) = self._owner_of(&id) {
            let caller = Self::env().caller();
            if caller == owner
                || self._allowance(&owner, &caller, &Some(id.clone()))
            {
                self._transfer(&owner, &to, &id, &data)
            } else {
                Err(PSP34Error::NotApproved)
            }
        } else {
            Err(PSP34Error::TokenNotExists)
        }
    }
}

pub trait PSP34InternalDefaultImpl: StorageFieldGetter<PSP34Data>
where
    PSP34Data: PSP34Storage,
{
    fn _balance_of_default_impl(&self, owner: &AccountId) -> u32 {
        self.data().owned_tokens_count.get(owner).unwrap_or(0)
    }

    fn _total_supply_default_impl(&self) -> u64 {
        self.data().total_supply()
    }

    fn _owner_of_default_impl(&self, id: &Id) -> Option<AccountId> {
        self.data().owner_of(id)
    }
    fn _approve_default_impl(
        &mut self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Option<Id>,
        approved: &bool,
    ) -> Result<(), PSP34Error> {
        let caller = Self::env().caller();
        if let Some(id) = id.clone() {
            let owner = self
                ._owner_of_default_impl(&id)
                .ok_or(PSP34Error::TokenNotExists)?;

            if owner == *operator {
                return Err(PSP34Error::SelfApprove);
            }

            if owner != caller {
                return Err(PSP34Error::NotApproved);
            }
        }
        self.data()
            .set_operator_approval(owner, operator, id, approved);

        Self::env().emit_event(Approval {
            owner: *owner,
            operator: *operator,
            id: id.as_ref().map(|v| v.clone()),
            approved: *approved,
        });

        Ok(())
    }

    fn _update_default_impl(
        &mut self,
        from: &Option<&AccountId>,
        to: &Option<&AccountId>,
        id: &Id,
    ) -> Result<(), PSP34Error> {
        if let Some(from) = from {
            self.data().remove_token_owner(&id, from)?;
        }

        if let Some(to) = to {
            self.data().insert_token_owner(&id, to)?;
        }

        Self::env().emit_event(Transfer {
            from: from.map(|v| *v),
            to: to.map(|v| *v),
            id: id.clone(),
        });
        Ok(())
    }

    fn _transfer_default_impl(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        id: &Id,
        _data: &Vec<u8>,
    ) -> Result<(), PSP34Error> {
        self._update_default_impl(&Some(from), &Some(to), id)
    }

    fn _mint_to_default_impl(
        &mut self,
        to: &AccountId,
        id: &Id,
    ) -> Result<(), PSP34Error> {
        self._update_default_impl(&None, &Some(to), id)
    }

    fn _burn_from_default_impl(
        &mut self,
        from: &AccountId,
        id: &Id,
    ) -> Result<(), PSP34Error> {
        self._update_default_impl(&Some(from), &None, id)
    }

    fn _allowance_default_impl(
        &self,
        owner: &AccountId,
        operator: &AccountId,
        id: &Option<Id>,
    ) -> bool {
        self.data().allowance(owner, operator, id)
    }
}
