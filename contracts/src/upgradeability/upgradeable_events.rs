// SPDX-License-Identifier: MIT

#[ink::event]
pub struct CodeHashChanged {
    pub old_code: Hash,
    pub new_code: Hash,
}
