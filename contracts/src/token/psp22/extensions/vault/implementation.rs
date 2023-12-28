use ink::env::call::{build_call, ExecutionInput};
use ink::env::DefaultEnvironment;
use ink::primitives::AccountId;
use ink::ToAccountId;
use pendzl::math::errors::MathError;
use pendzl::traits::{Balance, DefaultEnv, Storage};

use super::{Deposit, PSP22VaultInternal, PSP22VaultStorage};
use crate::token::psp22::implementation::Data as PSP22Data;
use crate::token::psp22::{PSP22Error, PSP22};
use crate::token::psp22::{PSP22Internal, PSP22Ref, PSP22Storage};

use ink::prelude::{string::ToString, vec::*};

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct Data {
    #[lazy]
    pub asset: PSP22Ref,
    #[lazy]
    pub underlying_decimals: u8,
}

impl PSP22VaultStorage for Data {
    fn asset(&self) -> PSP22Ref {
        self.asset.get().unwrap()
    }
    fn underlying_decimals(&self) -> u8 {
        self.underlying_decimals.get().unwrap()
    }
}
use ethnum::U256;
fn mul_div(x: u128, y: u128, denominator: u128) -> Result<u128, MathError> {
    if denominator == 0 {
        return Err(MathError::DivByZero);
    }

    if x == 0 || y == 0 {
        return Ok(0);
    }

    let x_u256 = U256::try_from(x).unwrap();
    let y_u256 = U256::try_from(y).unwrap();
    let denominator_u256 = U256::try_from(denominator).unwrap();

    // these can not fail overflow and denom is not 0
    let res: U256 = x_u256.checked_mul(y_u256).unwrap();
    let res = res.checked_div(denominator_u256).unwrap();
    match u128::try_from(res) {
        Ok(v) => Ok(v),
        _ => Err(MathError::Overflow),
    }
}

pub trait PSP22VaultInternalDefaultImpl:
    Storage<PSP22Data> + Storage<Data> + PSP22Internal + PSP22VaultInternal
where
    PSP22Data: PSP22Storage,
    Data: PSP22VaultStorage,
{
    fn _decimals_offset_default_impl(&self) -> u8 {
        0
    }

    fn _try_get_asset_decimals_default_impl(&self) -> (bool, u8) {
        let call = build_call::<DefaultEnvironment>()
            .call(self.data::<Data>().asset().to_account_id())
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
        self.data::<Data>().asset()
    }

    fn _total_assets_default_impl(&self) -> Balance {
        self._asset().balance_of(Self::env().account_id())
    }

    fn _convert_to_shares_default_impl(&self, assets: &Balance) -> Result<Balance, MathError> {
        let ret_val = mul_div(
            *assets,
            (self
                ._total_supply()
                .checked_add(1)
                .ok_or(MathError::Overflow)?)
            .checked_mul(10_u128.pow(self._decimals_offset() as u32))
            .ok_or(MathError::Overflow)?,
            self._total_assets()
                .checked_add(1)
                .ok_or(MathError::Overflow)?,
        )?;
        ink::env::debug_println!(
            "convert_to_shares_default_impl: assets: {}, decimals_offset: {}, ret_val: {}",
            assets,
            self._decimals_offset(),
            ret_val
        );

        Ok(ret_val)
    }

    fn _convert_to_assets_default_impl(&self, shares: &Balance) -> Result<Balance, MathError> {
        mul_div(
            *shares,
            self._total_assets()
                .checked_add(1)
                .ok_or(MathError::Overflow)?,
            (self
                ._total_supply()
                .checked_add(1)
                .ok_or(MathError::Overflow)?)
            .checked_mul(10_u128.pow(self._decimals_offset() as u32))
            .ok_or(MathError::Overflow)?,
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
        self._convert_to_assets(&owner_balance).unwrap()
    }
    fn _max_redeem_default_impl(&self, owner: &AccountId) -> Balance {
        self._balance_of(&owner)
    }
    fn _preview_deposit_default_impl(&self, assets: &Balance) -> Result<Balance, MathError> {
        self._convert_to_shares(&assets)
    }

    fn _preview_mint_default_impl(&self, shares: &Balance) -> Result<Balance, MathError> {
        self._convert_to_assets(&shares)?
            .checked_add(1)
            .ok_or(MathError::Overflow)
    }

    fn _preview_withdraw_default_impl(&self, assets: &Balance) -> Result<Balance, MathError> {
        self._convert_to_shares(&assets)
    }

    fn _preview_redeem_default_impl(&self, shares: &Balance) -> Result<Balance, MathError> {
        self._convert_to_assets(&shares)?
            .checked_add(1)
            .ok_or(MathError::Overflow)
    }

    fn _deposit_default_impl(
        &mut self,
        caller: &AccountId,
        receiver: &AccountId,
        assets: &Balance,
        shares: &Balance,
    ) -> Result<(), PSP22Error> {
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
        Ok(())
    }
}

pub trait PSP22VaultDefaultImpl: PSP22VaultInternal + PSP22Internal + DefaultEnv {
    fn asset_default_impl(&self) -> AccountId {
        self._asset().to_account_id()
    }

    fn total_assets_default_impl(&self) -> Balance {
        self._total_assets()
    }

    fn convert_to_shares_default_impl(&self, assets: Balance) -> Result<Balance, MathError> {
        self._convert_to_shares(&assets)
    }

    fn convert_to_assets_default_impl(&self, shares: Balance) -> Result<Balance, MathError> {
        self._convert_to_shares(&shares)
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

    fn preview_deposit_default_impl(&self, assets: Balance) -> Result<Balance, MathError> {
        self._preview_deposit(&assets)
    }

    fn preview_mint_default_impl(&self, shares: Balance) -> Result<Balance, MathError> {
        self._preview_mint(&shares)
    }

    fn preview_withdraw_default_impl(&self, assets: Balance) -> Result<Balance, MathError> {
        self._preview_withdraw(&assets)
    }

    fn preview_redeem_default_impl(&self, shares: Balance) -> Result<Balance, MathError> {
        self._preview_redeem(&shares)
    }

    fn deposit_default_impl(
        &mut self,
        assets: Balance,
        receiver: AccountId,
    ) -> Result<Balance, PSP22Error> {
        if assets > self._max_deposit(&receiver) {
            return Err(PSP22Error::Custom("Vault: Max".to_string()));
        }
        let shares = self._preview_deposit(&assets)?;
        self._deposit(&Self::env().caller(), &receiver, &assets, &shares)?;
        Ok(shares)
    }

    fn mint_default_impl(
        &mut self,
        shares: Balance,
        receiver: AccountId,
    ) -> Result<Balance, PSP22Error> {
        if shares > self._max_mint(&receiver) {
            return Err(PSP22Error::Custom("Vault: Max".to_string()));
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
        if assets > self._max_withdraw(&receiver) {
            return Err(PSP22Error::Custom("Vault: Max".to_string()));
        }
        let shares = self._preview_withdraw(&assets)?;
        self._withdraw(&Self::env().caller(), &receiver, &owner, &assets, &shares)?;
        Ok(assets)
    }

    fn redeem_default_impl(
        &mut self,
        shares: Balance,
        receiver: AccountId,
        owner: AccountId,
    ) -> Result<Balance, PSP22Error> {
        if shares > self._max_redeem(&receiver) {
            return Err(PSP22Error::Custom("Vault: Max".to_string()));
        }
        let assets = self._preview_redeem(&shares)?;
        self._withdraw(&Self::env().caller(), &receiver, &owner, &assets, &shares)?;
        Ok(assets)
    }
}
