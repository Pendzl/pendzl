// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub type ProvideVestScheduleInfoRef =
    contract_ref!(ProvideVestScheduleInfo, DefaultEnvironment);

#[ink::trait_definition]
pub trait ProvideVestScheduleInfo {
    #[ink(message)]
    fn get_waiting_and_vesting_durations(&self) -> (Timestamp, Timestamp);
}
