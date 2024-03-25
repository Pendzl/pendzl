// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use super::{
    Approval, Balance, PSP22Error, PSP22Internal, PSP22Storage, Transfer,
};
use ink::{prelude::vec::Vec, primitives::AccountId, storage::Mapping};
use pendzl::math::errors::MathError;
use pendzl::traits::{DefaultEnv, StorageFieldGetter};

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct PSP22Data {
    #[lazy]
    pub total_supply: Balance,
    pub balances: Mapping<AccountId, Balance>,
    pub allowances: Mapping<(AccountId, AccountId), Balance>,
}

impl PSP22Storage for PSP22Data {
    fn total_supply(&self) -> Balance {
        self.total_supply.get_or_default()
    }
    fn increase_total_supply(
        &mut self,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        let new_total_supply = self
            .total_supply
            .get_or_default()
            .checked_add(*amount)
            .ok_or(MathError::Overflow)?;
        self.total_supply.set(&new_total_supply);
        Ok(())
    }
    fn decrease_total_supply(
        &mut self,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        let new_total_supply = self
            .total_supply()
            .checked_sub(*amount)
            .ok_or(MathError::Underflow)?;
        self.total_supply.set(&new_total_supply);
        Ok(())
    }

    fn balance_of(&self, account: &AccountId) -> Balance {
        self.balances.get(account).unwrap_or_default()
    }
    fn increase_balance_of(
        &mut self,
        account: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        let new_balance = self
            .balance_of(account)
            .checked_add(*amount)
            .ok_or(MathError::Overflow)?;
        self.balances.insert(account, &new_balance);
        Ok(())
    }
    fn decrease_balance_of(
        &mut self,
        account: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        let new_balance = self
            .balance_of(account)
            .checked_sub(*amount)
            .ok_or(PSP22Error::InsufficientBalance)?;
        self.balances.insert(account, &new_balance);
        Ok(())
    }

    fn allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
        self.allowances.get(&(*owner, *spender)).unwrap_or_default()
    }
    fn set_allowance(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        value: &Balance,
    ) {
        self.allowances.insert(&(*owner, *spender), value);
    }
    fn increase_allowance(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<Balance, PSP22Error> {
        let new_allowance = self
            .allowance(owner, spender)
            .checked_add(*amount)
            .ok_or(MathError::Overflow)?;
        self.allowances.insert(&(*owner, *spender), &new_allowance);
        Ok(new_allowance)
    }
    fn decrease_allowance(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<Balance, PSP22Error> {
        let new_allowance = self
            .allowance(owner, spender)
            .checked_sub(*amount)
            .ok_or(PSP22Error::InsufficientAllowance)?;
        self.allowances.insert(&(*owner, *spender), &new_allowance);
        Ok(new_allowance)
    }
}

pub trait PSP22DefaultImpl: DefaultEnv + PSP22Internal {
    fn total_supply_default_impl(&self) -> Balance {
        self._total_supply()
    }

    fn balance_of_default_impl(&self, owner: AccountId) -> Balance {
        self._balance_of(&owner)
    }

    fn allowance_default_impl(
        &self,
        owner: AccountId,
        spender: AccountId,
    ) -> Balance {
        self._allowance(&owner, &spender)
    }

    fn transfer_default_impl(
        &mut self,
        to: AccountId,
        value: Balance,
        _data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        let from = Self::env().caller();
        self._update(Some(&from), Some(&to), &value)?;
        Ok(())
    }

    fn transfer_from_default_impl(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        _data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        let caller = Self::env().caller();
        self._decrease_allowance_from_to(&from, &caller, &value)?;
        self._update(Some(&from), Some(&to), &value)?;
        Ok(())
    }

    fn approve_default_impl(
        &mut self,
        spender: AccountId,
        value: Balance,
    ) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        self._approve(&owner, &spender, &value)?;
        Ok(())
    }

    fn increase_allowance_default_impl(
        &mut self,
        spender: AccountId,
        delta_value: Balance,
    ) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        self._increase_allowance_from_to(&owner, &spender, &delta_value)
    }

    fn decrease_allowance_default_impl(
        &mut self,
        spender: AccountId,
        delta_value: Balance,
    ) -> Result<(), PSP22Error> {
        let owner = Self::env().caller();
        self._decrease_allowance_from_to(&owner, &spender, &delta_value)
    }
}

pub trait PSP22InternalDefaultImpl: StorageFieldGetter<PSP22Data>
where
    PSP22Data: PSP22Storage,
{
    fn _total_supply_default_impl(&self) -> Balance {
        self.data().total_supply()
    }

    fn _balance_of_default_impl(&self, owner: &AccountId) -> Balance {
        self.data().balance_of(owner)
    }

    fn _allowance_default_impl(
        &self,
        owner: &AccountId,
        spender: &AccountId,
    ) -> Balance {
        self.data().allowance(owner, spender)
    }

    fn _update_default_impl(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        if let Some(from) = from {
            self.data().decrease_balance_of(from, amount)?;
        } else {
            self.data().increase_total_supply(amount)?;
        }

        if let Some(to) = to {
            self.data().increase_balance_of(to, amount)?;
        } else {
            self.data().decrease_total_supply(amount)?;
        }

        Self::env().emit_event(Transfer {
            from: from.and_then(|v| Some(*v)),
            to: to.and_then(|v| Some(*v)),
            value: *amount,
        });
        Ok(())
    }

    fn _transfer_default_impl(
        &mut self,
        from: &AccountId,
        to: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        self._update_default_impl(Some(from), Some(to), amount)
    }

    fn _mint_to_default_impl(
        &mut self,
        to: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        self._update_default_impl(None, Some(to), amount)
    }

    fn _burn_from_default_impl(
        &mut self,
        from: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        self._update_default_impl(Some(from), None, amount)
    }

    fn _approve_default_impl(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        self.data().set_allowance(owner, spender, amount);
        Self::env().emit_event(Approval {
            owner: *owner,
            spender: *spender,
            value: *amount,
        });
        Ok(())
    }

    fn _decrease_allowance_from_to_default_impl(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        let new_allowance =
            self.data().decrease_allowance(owner, spender, amount)?;
        Self::env().emit_event(Approval {
            owner: *owner,
            spender: *spender,
            value: new_allowance,
        });
        Ok(())
    }
    fn _increase_allowance_from_to_default_impl(
        &mut self,
        owner: &AccountId,
        spender: &AccountId,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        let new_allowance =
            self.data().increase_allowance(owner, spender, amount)?;
        Self::env().emit_event(Approval {
            owner: *owner,
            spender: *spender,
            value: new_allowance,
        });
        Ok(())
    }
}
