// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(Pausable)]
#[ink::contract]
pub mod my_pausable {
    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
        pause: PausableData,

        count: u32,
        drastic_measure_taken: bool,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn count(&self) -> u32 {
            self.count
        }

        #[ink(message)]
        pub fn drastic_measure_taken(&self) -> bool {
            self.drastic_measure_taken
        }

        #[ink(message)]
        pub fn pause(&mut self) -> Result<(), PausableError> {
            self._pause()
        }

        #[ink(message)]
        pub fn unpause(&mut self) -> Result<(), PausableError> {
            self._unpause()
        }

        #[ink(message)]
        pub fn normal_process(&mut self) -> Result<(), PausableError> {
            self._ensure_not_paused()?;
            self.count += 1;
            Ok(())
        }

        #[ink(message)]
        pub fn drastic_measure(&mut self) -> Result<(), PausableError> {
            self._ensure_paused()?;
            self.drastic_measure_taken = true;
            Ok(())
        }
    }
}
