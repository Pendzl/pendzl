// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use ink::env::call::{build_call, ExecutionInput};
use ink::env::DefaultEnvironment;
use ink::primitives::AccountId;
use ink::ToAccountId;
use pendzl::math::{errors::MathError, operations::*};
use pendzl::traits::{Balance, DefaultEnv, StorageFieldGetter};

use super::{Deposit, PSP22VaultInternal, PSP22VaultStorage, Withdraw};
use crate::token::psp22::implementation::PSP22Data;
use crate::token::psp22::{PSP22Error, PSP22};
use crate::token::psp22::{PSP22Internal, PSP22Ref, PSP22Storage};

use ink::prelude::{string::ToString, vec::*};

use pendzl::math::operations::Rounding;

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct PSP22VaultData {
    #[lazy]
    pub asset: PSP22Ref,
    #[lazy]
    pub underlying_decimals: u8,
}

impl PSP22VaultData {
    pub fn new(asset: AccountId, underlying_decimals: Option<u8>) -> Self {
        let mut instance: PSP22VaultData = Default::default();
        instance.asset.set(&asset.into());
        if let Some(underlying_decimals) = underlying_decimals {
            instance.underlying_decimals.set(&underlying_decimals);
        } else {
            let (success, decimals) = {
                let call = build_call::<DefaultEnvironment>()
                    .call(asset)
                    .exec_input(ExecutionInput::new(
                        ink::env::call::Selector::new(ink::selector_bytes!(
                            "PSP22Metadata::token_decimals"
                        )),
                    ))
                    .returns::<u8>();

                match call.try_invoke() {
                    Err(_) => (false, 0),
                    Ok(v) => match v {
                        Err(_) => (false, 0),
                        Ok(v) => (true, v),
                    },
                }
            };
            if success {
                instance.underlying_decimals.set(&decimals);
            } else {
                instance.underlying_decimals.set(&12);
            }
        }
        instance
    }
}

impl PSP22VaultStorage for PSP22VaultData {
    fn asset(&self) -> PSP22Ref {
        self.asset.get().unwrap()
    }
    fn underlying_decimals(&self) -> u8 {
        self.underlying_decimals.get().unwrap()
    }
}

pub trait PSP22VaultInternalDefaultImpl:
    StorageFieldGetter<PSP22Data>
    + StorageFieldGetter<PSP22VaultData>
    + PSP22Internal
    + PSP22VaultInternal
where
    PSP22Data: PSP22Storage,
    PSP22VaultData: PSP22VaultStorage,
{
    fn _decimals_offset_default_impl(&self) -> u8 {
        0
    }

    fn _try_get_asset_decimals_default_impl(&self) -> (bool, u8) {
        let call = build_call::<DefaultEnvironment>()
            .call(self.data::<PSP22VaultData>().asset().to_account_id())
            .exec_input(ExecutionInput::new(ink::env::call::Selector::new(
                ink::selector_bytes!("PSP22Metadata::token_decimals"),
            )))
            .returns::<u8>();

        match call.try_invoke() {
            Err(_) => (false, 0),
            Ok(v) => match v {
                Err(_) => (false, 0),
                Ok(v) => (true, v),
            },
        }
    }

    fn _asset_default_impl(&self) -> PSP22Ref {
        self.data::<PSP22VaultData>().asset()
    }

    fn _total_assets_default_impl(&self) -> Balance {
        self._asset().balance_of(Self::env().account_id())
    }

    fn _convert_to_shares_default_impl(
        &self,
        assets: &Balance,
        round: Rounding,
    ) -> Result<Balance, MathError> {
        let total_shares = self._total_supply();
        let total_assets = self._total_assets();
        let decimals_offset = 10_u128.pow(self._decimals_offset() as u32);
        mul_div(
            *assets,
            total_shares
                .checked_add(decimals_offset)
                .ok_or(MathError::Overflow)?,
            total_assets.checked_add(1).ok_or(MathError::Overflow)?,
            round,
        )
    }

    fn _convert_to_assets_default_impl(
        &self,
        shares: &Balance,
        round: Rounding,
    ) -> Result<Balance, MathError> {
        let total_shares = self._total_supply();
        let total_assets = self._total_assets();
        let decimals_offset = 10_u128.pow(self._decimals_offset() as u32);
        mul_div(
            *shares,
            total_assets.checked_add(1).ok_or(MathError::Overflow)?,
            total_shares
                .checked_add(decimals_offset)
                .ok_or(MathError::Overflow)?,
            round,
        )
    }

    fn _max_deposit_default_impl(&self, _to: &AccountId) -> Balance {
        u128::MAX
    }

    fn _max_mint_default_impl(&self, _to: &AccountId) -> Balance {
        u128::MAX
    }

    fn _max_withdraw_default_impl(&self, owner: &AccountId) -> Balance {
        let owner_balance = self._balance_of(&owner);
        self._convert_to_assets(&owner_balance, Rounding::Down)
            .unwrap()
    }
    fn _max_redeem_default_impl(&self, owner: &AccountId) -> Balance {
        self._balance_of(&owner)
    }
    fn _preview_deposit_default_impl(
        &self,
        assets: &Balance,
    ) -> Result<Balance, MathError> {
        self._convert_to_shares(&assets, Rounding::Down)
    }

    fn _preview_mint_default_impl(
        &self,
        shares: &Balance,
    ) -> Result<Balance, MathError> {
        self._convert_to_assets(&shares, Rounding::Up)
    }

    fn _preview_withdraw_default_impl(
        &self,
        assets: &Balance,
    ) -> Result<Balance, MathError> {
        self._convert_to_shares(&assets, Rounding::Up)
    }

    fn _preview_redeem_default_impl(
        &self,
        shares: &Balance,
    ) -> Result<Balance, MathError> {
        self._convert_to_assets(&shares, Rounding::Down)
    }

    fn _deposit_default_impl(
        &mut self,
        caller: &AccountId,
        receiver: &AccountId,
        assets: &Balance,
        shares: &Balance,
    ) -> Result<(), PSP22Error> {
        ink::env::debug_println!(
            "deposit_default_impl: assets: {}, shares: {}",
            assets,
            shares
        );
        self._asset().transfer_from(
            *caller,
            Self::env().account_id(),
            *assets,
            Vec::<u8>::new(),
        )?;
        self._mint_to(receiver, shares)?;

        Self::env().emit_event(Deposit {
            sender: *caller,
            owner: *receiver,
            assets: *assets,
            shares: *shares,
        });

        Ok(())
    }

    fn _withdraw_default_impl(
        &mut self,
        caller: &AccountId,
        receiver: &AccountId,
        owner: &AccountId,
        assets: &Balance,
        shares: &Balance,
    ) -> Result<(), PSP22Error> {
        if caller != owner {
            self._decrease_allowance_from_to(owner, caller, shares)?;
        }

        self._burn_from(owner, shares)?;
        self._asset()
            .transfer(*receiver, *assets, Vec::<u8>::new())?;

        Self::env().emit_event(Withdraw {
            sender: *caller,
            receiver: *receiver,
            owner: *owner,
            assets: *assets,
            shares: *shares,
        });
        Ok(())
    }
}

pub trait PSP22VaultDefaultImpl:
    PSP22VaultInternal + PSP22Internal + DefaultEnv
{
    fn asset_default_impl(&self) -> AccountId {
        self._asset().to_account_id()
    }

    fn total_assets_default_impl(&self) -> Balance {
        self._total_assets()
    }

    fn convert_to_shares_default_impl(
        &self,
        assets: Balance,
        round: Rounding,
    ) -> Result<Balance, MathError> {
        self._convert_to_shares(&assets, round)
    }

    fn convert_to_assets_default_impl(
        &self,
        shares: Balance,
        round: Rounding,
    ) -> Result<Balance, MathError> {
        self._convert_to_assets(&shares, round)
    }

    fn max_deposit_default_impl(&self, receiver: AccountId) -> Balance {
        self._max_deposit(&receiver)
    }

    fn max_mint_default_impl(&self, receiver: AccountId) -> Balance {
        self._max_mint(&receiver)
    }

    fn max_withdraw_default_impl(&self, owner: AccountId) -> Balance {
        self._max_withdraw(&owner)
    }

    fn max_redeem_default_impl(&self, owner: AccountId) -> Balance {
        self._max_redeem(&owner)
    }

    fn preview_deposit_default_impl(
        &self,
        assets: Balance,
    ) -> Result<Balance, MathError> {
        self._preview_deposit(&assets)
    }

    fn preview_mint_default_impl(
        &self,
        shares: Balance,
    ) -> Result<Balance, MathError> {
        self._preview_mint(&shares)
    }

    fn preview_withdraw_default_impl(
        &self,
        assets: Balance,
    ) -> Result<Balance, MathError> {
        self._preview_withdraw(&assets)
    }

    fn preview_redeem_default_impl(
        &self,
        shares: Balance,
    ) -> Result<Balance, MathError> {
        self._preview_redeem(&shares)
    }

    fn deposit_default_impl(
        &mut self,
        assets: Balance,
        receiver: AccountId,
    ) -> Result<Balance, PSP22Error> {
        if assets > self._max_deposit(&receiver) {
            return Err(PSP22Error::Custom("V:MaxDeposit".to_string()));
        }
        let shares = self._preview_deposit(&assets)?;
        ink::env::debug_println!(
            "deposit_default_impl: assets: {}, shares: {}",
            assets,
            shares
        );
        self._deposit(&Self::env().caller(), &receiver, &assets, &shares)?;
        Ok(shares)
    }

    fn mint_default_impl(
        &mut self,
        shares: Balance,
        receiver: AccountId,
    ) -> Result<Balance, PSP22Error> {
        if shares > self._max_mint(&receiver) {
            return Err(PSP22Error::Custom("V:MaxMint".to_string()));
        }
        let assets = self._preview_mint(&shares)?;
        self._deposit(&Self::env().caller(), &receiver, &assets, &shares)?;
        Ok(assets)
    }

    fn withdraw_default_impl(
        &mut self,
        assets: Balance,
        receiver: AccountId,
        owner: AccountId,
    ) -> Result<Balance, PSP22Error> {
        if assets > self._max_withdraw(&owner) {
            return Err(PSP22Error::Custom("V:MaxWithdraw".to_string()));
        }
        let shares = self._preview_withdraw(&assets)?;
        self._withdraw(
            &Self::env().caller(),
            &receiver,
            &owner,
            &assets,
            &shares,
        )?;
        Ok(assets)
    }

    fn redeem_default_impl(
        &mut self,
        shares: Balance,
        receiver: AccountId,
        owner: AccountId,
    ) -> Result<Balance, PSP22Error> {
        if shares > self._max_redeem(&owner) {
            return Err(PSP22Error::Custom("V:MaxRedeem".to_string()));
        }
        let assets = self._preview_redeem(&shares)?;
        self._withdraw(
            &Self::env().caller(),
            &receiver,
            &owner,
            &assets,
            &shares,
        )?;
        Ok(assets)
    }
}
