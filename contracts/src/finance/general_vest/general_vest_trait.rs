// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub type GeneralVestRef = contract_ref!(GeneralVest, DefaultEnvironment);

/// GeneralVest trait that provides a framework for implementing vesting
/// mechanisms in smart contracts. This lightweight version is designed to
/// provide a simple and flexible interface for managing vesting schedules and
/// releasing vested tokens.
///
/// Vesting schedules are defined by the `VestingSchedule` enum, which can be
/// used to specify a variety of vesting schedules, including linear,
/// and custom schedules. Vesting schedules are associated with a unique `vest_id`
/// that can be used to reference a specific vesting schedule.
#[ink::trait_definition]
pub trait GeneralVest {
    /// Creates a new vesting schedule for `receiver` with `amount` of tokens
    /// from `asset` according to the specified `schedule`. The `data` parameter
    /// can be used to pass additional information to the contract.
    ///
    /// On success, a `VestingScheduled` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `PSP22Error` if the transfer_from of the asset fails.
    /// Returns with `NativeTransferFailed` if the transfer of the native token fails.
    /// Returns with `InvalidAmountPaid` if the amount paid is invalid.
    /// Returns with `Custom` when custom error is returned.
    #[ink(message, payable)]
    fn create_vest(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        schedule: VestingSchedule,
        data: Vec<u8>,
    ) -> Result<(), VestingError>;

    /// Releases total amount available of vested tokens to `receiver` from `asset`. The `data` parameter
    /// can be used to pass additional information to the contract.
    /// Returns the total available amount of tokens released.
    ///
    /// On success, a `TokenReleased` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `PSP22Error` if the transfer of the asset fails.
    /// Returns with `NativeTransferFailed` if the transfer of the native token fails.
    /// Returns with `Custom` when custom error is returned.
    #[ink(message)]
    fn release(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        data: Vec<u8>,
    ) -> Result<u128, VestingError>;

    /// Releases the amount available of vested tokens to `receiver` from `asset`
    /// for the `VestingSchedule` of specified `vest_id`.
    /// The `data` parameter can be used to pass additional information to the contract.
    ///
    /// On success, a `TokenReleased` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `PSP22Error` if the transfer of the asset fails.
    /// Returns with `NativeTransferFailed` if the transfer of the native token fails.
    /// Returns with `Custom` when custom error is returned.
    #[ink(message)]
    fn release_by_vest_id(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        id: u32,
        data: Vec<u8>,
    ) -> Result<(), VestingError>;

    /// Returns the `VestingSchedule` for `receiver` from `asset` for the `vest_id`.
    /// The `data` parameter can be used to pass additional information to the contract.
    ///
    /// Returns `None` if the `vest_id` does not exist.
    /// Returns the `VestingSchedule` if the `vest_id` exists.
    #[ink(message)]
    fn vesting_schedule_of(
        &self,
        of: AccountId,
        asset: Option<AccountId>,
        id: u32,
        data: Vec<u8>,
    ) -> Option<VestingData>;

    /// Returns the next available `vest_id` for `receiver` for given `asset`.
    /// The `data` parameter can be used to pass additional information to the contract.
    /// The next available id signifies the total amount of vesting schedules created for the `receiver` for the given `asset`.
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
    /// Creates a new vesting schedule for `receiver` with `amount` of tokens
    /// from `asset` according to the specified `schedule`. The `data` parameter
    /// can be used to pass additional information to the contract.
    ///
    /// # Errors
    ///
    /// Return `MathError` if meets any math error.
    /// Returns `Custom` when custom error is returned.
    fn create(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        schedule: VestingSchedule,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;

    /// Releases total amount available of vested tokens to `receiver` from `asset`. The `data` parameter
    /// can be used to pass additional information to the contract.
    /// Returns the total available amount of tokens released.
    ///
    ///
    /// # Errors
    ///
    /// Returns `MathError` if meets any math error.
    /// Returns with `Custom` when custom error is returned.
    fn release(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        data: &Vec<u8>,
    ) -> Result<Balance, VestingError>;

    /// Releases the amount available of vested tokens to `receiver` from `asset`
    /// for the `VestingSchedule` of specified `vest_id`.
    /// The `data` parameter can be used to pass additional information to the contract.
    ///
    ///
    /// # Errors
    ///
    /// Returns `MathError` if meets any math error.
    /// Returns with `Custom` when custom error is returned.
    fn release_by_vest_id(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        id: u32,
        data: &Vec<u8>,
    ) -> Result<(bool, Balance), VestingError>;

    /// Returns the `VestingSchedule` for `receiver` from `asset` for the `vest_id`.
    /// The `data` parameter can be used to pass additional information to the contract.
    /// Returns `None` if the `vest_id` does not exist.
    /// Returns the `VestingSchedule` if the `vest_id` exists.
    ///
    /// # Errors
    ///
    /// Returns `MathError` if meets any math error.
    /// Returns with `Custom` when custom error is returned.
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
    /// Creates a new vesting schedule for `receiver` with `amount` of tokens
    /// from `asset` according to the specified `schedule`. The `data` parameter
    /// can be used to pass additional information to the contract.
    ///
    /// On success, a `VestingScheduled` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `PSP22Error` if the transfer_from of the asset fails.
    /// Returns with `NativeTransferFailed` if the transfer of the native token fails.
    /// Returns with `InvalidAmountPaid` if the amount paid is invalid.
    /// Returns with `Custom` when custom error is returned.
    fn _create_vest(
        &mut self,
        receiver: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        schedule: VestingSchedule,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;

    /// Releases total amount available of vested tokens to `receiver` from `asset`. The `data` parameter
    /// can be used to pass additional information to the contract.
    /// Returns the total available amount of tokens released.
    ///
    /// On success, a `TokenReleased` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `PSP22Error` if the transfer of the asset fails.
    /// Returns with `NativeTransferFailed` if the transfer of the native token fails.
    /// Returns with `Custom` when custom error is returned.
    fn _release(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        data: &Vec<u8>,
    ) -> Result<u128, VestingError>;
    /// Releases the amount available of vested tokens to `receiver` from `asset`
    /// for the `VestingSchedule` of specified `vest_id`.
    /// The `data` parameter can be used to pass additional information to the contract.
    ///
    /// On success, a `TokenReleased` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns with `PSP22Error` if the transfer of the asset fails.
    /// Returns with `NativeTransferFailed` if the transfer of the native token fails.
    /// Returns with `Custom` when custom error is returned.
    fn _release_by_vest_id(
        &mut self,
        receiver: Option<AccountId>,
        asset: Option<AccountId>,
        id: u32,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;

    /// Handles the transfer of tokens into the contract.
    /// The `data` parameter can be used to pass additional information to the contract.
    ///
    /// # Errors
    ///
    /// Returns with `PSP22Error` if the transfer_from of the asset fails.
    /// Returns with `NativeTransferFailed` if the transfer of the native token fails.
    fn _handle_transfer_in(
        &mut self,
        asset: Option<AccountId>,
        from: AccountId,
        amount: Balance,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;

    /// Handles the transfer of tokens out of the contract.
    /// The `data` parameter can be used to pass additional information to the contract.
    ///
    /// # Errors
    ///
    /// Returns with `PSP22Error` if the transfer of the asset fails.
    /// Returns with `NativeTransferFailed` if the transfer of the native token fails.
    fn _handle_transfer_out(
        &mut self,
        asset: Option<AccountId>,
        to: AccountId,
        amount: Balance,
        data: &Vec<u8>,
    ) -> Result<(), VestingError>;

    /// Returns the `VestingSchedule` for `receiver` from `asset` for the `vest_id`.
    /// The `data` parameter can be used to pass additional information to the contract.
    /// Returns `None` if the `vest_id` does not exist.
    /// Returns the `VestingSchedule` if the `vest_id` exists.
    fn _vesting_schedule_of(
        &self,
        of: AccountId,
        asset: Option<AccountId>,
        id: u32,
        data: &Vec<u8>,
    ) -> Option<VestingData>;

    /// Returns the next available `vest_id` for `receiver` for given `asset`.
    /// The `data` parameter can be used to pass additional information to the contract.
    /// The next available id signifies the total amount of vesting schedules created for the `receiver` for the given `asset`.
    /// Returns the next available `vest_id`.
    fn _next_id_vest_of(
        &self,
        of: AccountId,
        asset: Option<AccountId>,
        data: &Vec<u8>,
    ) -> u32;
}
