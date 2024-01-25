// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod ts_provider {

    #[ink(storage)]
    #[derive(Default)]
    pub struct TSProvider {
        start_time: Timestamp,
        end_time: Timestamp,
    }

    impl TSProvider {
        #[ink(constructor)]
        pub fn new(start_time: Timestamp, end_time: Timestamp) -> Self {
            Self {
                start_time,
                end_time,
            }
        }

        #[ink(message)]
        pub fn start_time(&self) -> Timestamp {
            self.start_time
        }

        #[ink(message)]
        pub fn set_start_time(&mut self, start_time: Timestamp) {
            self.start_time = start_time;
        }

        #[ink(message)]
        pub fn end_time(&self) -> Timestamp {
            self.end_time
        }

        #[ink(message)]
        pub fn set_end_time(&mut self, end_time: Timestamp) {
            self.end_time = end_time;
        }

        #[ink(message)]
        pub fn get_current_timestamp(&self) -> Timestamp {
            self.env().block_timestamp()
        }
    }
}
