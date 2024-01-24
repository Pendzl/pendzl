// SPDX-License-Identifier: MIT
pub use pendzl::traits::Balance;

use ink::contract_ref;
use ink::{prelude::vec::Vec, primitives::AccountId};

pub type VestingRef = contract_ref!(Vesting, DefaultEnvironment);
#[ink::trait_definition]
pub trait Vesting {
    #[ink(message, payable)]
    fn create_vest(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: VestingTimeConstraint,
        vesting_end: VestingTimeConstraint,
        data: Vec<u8>,
    ) -> Result<(), VestingError>;
    #[ink(message)]
    fn release(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        data: Vec<u8>,
    ) -> Result<(), VestingError>;
    #[ink(message)]
    fn release_by_vest_id(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        id: u32,
        data: Vec<u8>,
    ) -> Result<(), VestingError>;
    #[ink(message)]
    fn vesting_schedule_of(
        &self,
        of: AccountId,
        asset: Option<AccountId>,
        id: u32,
        data: Vec<u8>,
    ) -> Option<VestingSchedule>;
    #[ink(message)]
    fn next_id_vest_of(&self, of: AccountId, asset: Option<AccountId>, data: Vec<u8>) -> u32;
}

pub trait VestingInternal {
    fn _create_vest(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: VestingTimeConstraint,
        vesting_end: VestingTimeConstraint,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;

    fn _release(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;

    fn _release_by_vest_id(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        id: u32,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;
    fn _handle_transfer_in(
        &mut self,
        asset: Option<AccountId>,
        from: AccountId,
        amount: Balance,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;
    fn _handle_transfer_out(
        &mut self,
        asset: Option<AccountId>,
        to: AccountId,
        amount: Balance,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;

    fn _vesting_schedule_of(
        &self,
        of: AccountId,
        asset: Option<AccountId>,
        id: u32,
        data: &Vec<u8>,
    ) -> Option<VestingSchedule>;
    fn _next_id_vest_of(&self, of: AccountId, asset: Option<AccountId>, data: &Vec<u8>) -> u32;
}
pub trait VestingStorage {
    fn create(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        vesting_start: VestingTimeConstraint,
        vesting_end: VestingTimeConstraint,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;

    fn release(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        data: &Vec<u8>,
    ) -> Result<Balance, VestingError>;

    fn release_by_vest_id(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        id: u32,
        data: &Vec<u8>,
    ) -> Result<(bool, Balance), VestingError>;

    fn get_schedule_by_id(
        &self,
        receiver: AccountId,
        asset: Option<AccountId>,
        id: u32,
        data: &Vec<u8>,
    ) -> Option<VestingSchedule>;
}
