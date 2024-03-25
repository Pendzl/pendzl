// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use ink::env::call::{build_call, ExecutionInput};
use ink::env::DefaultEnvironment;

use pendzl::traits::DefaultEnv;
use pendzl::traits::Timestamp;
use scale::{Decode, Encode};

#[derive(Debug, Encode, Decode, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct ExternalTimeConstraint {
    pub account: AccountId,
    pub fallback_values: (Timestamp, Timestamp),
}

impl ExternalTimeConstraint {
    fn get_waiting_and_duration_times(&self) -> (Timestamp, Timestamp) {
        let call = build_call::<DefaultEnvironment>()
            .call(self.account)
            .exec_input(ExecutionInput::new(ink::env::call::Selector::new(
                ink::selector_bytes!("ProvideVestScheduleInfo::get_waiting_and_vesting_durations"),
            )))
            .returns::<(Timestamp, Timestamp)>();

        match call.try_invoke() {
            Err(_) => self.fallback_values,
            Ok(v) => match v {
                Err(_) => self.fallback_values,
                Ok(v) => v,
            },
        }
    }
}

#[derive(Debug, Encode, PartialEq, Eq, Decode, Clone)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum VestingSchedule {
    Constant(Timestamp, Timestamp),
    External(ExternalTimeConstraint),
}

impl VestingSchedule {
    fn get_waiting_and_duration_times(&self) -> (Timestamp, Timestamp) {
        match self {
            VestingSchedule::Constant(waiting_duration, vesting_duration) => {
                (*waiting_duration, *vesting_duration)
            }
            VestingSchedule::External(external_constraint) => {
                external_constraint.get_waiting_and_duration_times()
            }
        }
    }
}

#[derive(Debug, Encode, Decode, Clone)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct VestingData {
    pub creation_time: Timestamp,
    pub schedule: VestingSchedule,
    pub amount: Balance,
    pub released: Balance,
}

impl VestingData {
    pub fn collect_releasable_rdown(
        &mut self,
    ) -> Result<Balance, VestingError> {
        let amount_releaseable = self.amount_releaseable_rdown()?;
        self.released = self
            .released
            .checked_add(amount_releaseable)
            .ok_or(MathError::Overflow)?;
        Ok(amount_releaseable)
    }
    pub fn amount_releaseable_rdown(&self) -> Result<Balance, VestingError> {
        let now: Timestamp = Self::env().block_timestamp();
        let (waiting_duration, vesting_duration) =
            self.schedule.get_waiting_and_duration_times();
        let start_time = self
            .creation_time
            .checked_add(waiting_duration)
            .ok_or(MathError::Overflow)?;
        let end_time = start_time
            .checked_add(vesting_duration)
            .ok_or(MathError::Overflow)?;

        let is_overdue = self.is_overdue()?;
        if is_overdue {
            return Ok(self.amount - self.released);
        }
        if now <= start_time {
            return Ok(0);
        }
        if start_time == end_time && now > start_time {
            return Ok(self.amount - self.released);
        }
        let total_to_release = self
            .amount
            .checked_mul(u128::try_from(now - start_time).unwrap())
            .ok_or(MathError::Overflow)?
            .checked_div(u128::try_from(end_time - start_time).unwrap())
            .ok_or(MathError::DivByZero)?
            .checked_sub(1)
            .ok_or(MathError::Underflow)?; //TODO ??
        let amount_releaseable = total_to_release - self.released;
        Ok(amount_releaseable)
    }
    pub fn is_overdue(&self) -> Result<bool, VestingError> {
        let now: Timestamp = Self::env().block_timestamp();
        let (waiting_duration, vesting_duration) =
            self.schedule.get_waiting_and_duration_times();
        let start_time = self
            .creation_time
            .checked_add(waiting_duration)
            .ok_or(MathError::Overflow)?;
        let end_time = start_time
            .checked_add(vesting_duration)
            .ok_or(MathError::Overflow)?;

        Ok(now >= end_time)
    }
}
