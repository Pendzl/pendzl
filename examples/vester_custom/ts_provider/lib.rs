// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod ts_provider {
    use pendzl::contracts::finance::general_vest::ProvideVestScheduleInfo;

    #[ink(storage)]
    #[derive(Default)]
    pub struct TSProvider {
        waiting_duration: Timestamp,
        vesting_duration: Timestamp,
    }

    impl TSProvider {
        #[ink(constructor)]
        pub fn new(waiting_duration: Timestamp, vesting_duration: Timestamp) -> Self {
            Self {
                waiting_duration,
                vesting_duration,
            }
        }

        #[ink(message)]
        pub fn set_waiting_duration(&mut self, waiting_duration: Timestamp) {
            self.waiting_duration = waiting_duration;
        }

        #[ink(message)]
        pub fn set_vesting_duration(&mut self, vesting_duration: Timestamp) {
            self.vesting_duration = vesting_duration;
        }

        #[ink(message)]
        pub fn get_current_timestamp(&self) -> Timestamp {
            self.env().block_timestamp()
        }
    }

    impl ProvideVestScheduleInfo for TSProvider {
        #[ink(message)]
        fn waiting_duration(&self) -> Timestamp {
            self.waiting_duration
        }

        #[ink(message)]
        fn vesting_duration(&self) -> Timestamp {
            self.vesting_duration
        }
        #[ink(message)]
        fn get_waiting_and_vesting_durations(&self) -> (Timestamp, Timestamp) {
            (self.waiting_duration, self.vesting_duration)
        }
    }
}
