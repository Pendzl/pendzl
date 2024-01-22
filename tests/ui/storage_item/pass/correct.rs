// SPDX-License-Identifier: MIT
use pendzl::traits::AccountId;
#[derive(Debug)]
#[pendzl::storage_item]
pub struct OwnableData {
    #[lazy]
    pub owner: AccountId,
}

fn main() {}
