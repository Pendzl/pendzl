// SPDX-License-Identifier: MIT

pub type GeneralVestRef = contract_ref!(GeneralVest, DefaultEnvironment);
#[ink::trait_definition]
pub trait GeneralVest {
    #[ink(message, payable)]
    fn create_vest(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        schedule: VestingSchedule,
        data: Vec<u8>,
    ) -> Result<(), VestingError>;
    #[ink(message)]
    fn release(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        data: Vec<u8>,
    ) -> Result<u128, VestingError>;
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
    ) -> Option<VestingData>;
    #[ink(message)]
    fn next_id_vest_of(
        &self,
        of: AccountId,
        asset: Option<AccountId>,
        data: Vec<u8>,
    ) -> u32;
}

/// trait that must be implemented by exactly one storage field of a contract storage
/// so the Pendzl GeneralVestInternal and GeneralVest implementation can be derived.
pub trait GeneralVestStorage {
    fn create(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        schedule: VestingSchedule,
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
    ) -> Option<VestingData>;
}

/// trait that is derived by Pendzl GeneralVest implementation macro assuming StorageFieldGetter<GeneralVestStorage> is implemented
///
/// functions of this trait are recomended to use while writing ink::messages
pub trait GeneralVestInternal {
    fn _create_vest(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        schedule: VestingSchedule,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;

    fn _release(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        data: &Vec<u8>,
    ) -> Result<u128, VestingError>;

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
    ) -> Option<VestingData>;
    fn _next_id_vest_of(
        &self,
        of: AccountId,
        asset: Option<AccountId>,
        data: &Vec<u8>,
    ) -> u32;
}
