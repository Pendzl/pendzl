// SPDX-License-Identifier: MIT
#[ink::contract]
mod base_psp22 {
    use pendzl::traits::StorageFieldGetter;

    #[derive(StorageFieldGetter)]
    pub struct PSP22Struct {
        pub value: bool,
    }

    impl PSP22Struct {
        #[ink(constructor)]
        pub fn new(value: bool) -> Self {
            Self { value }
        }

        #[ink(message)]
        pub fn set_value(&mut self, value: bool) {
            self.value = value;
        }

        #[ink(message)]
        pub fn get_value(&self) -> bool {
            self.value
        }
    }
}

fn main() {}
