// SPDX-License-Identifier: MIT
#[ink::event]
pub struct AttribiuteSet {
    #[ink(topic)]
    id: Id,
    key: String,
    data: String,
}
