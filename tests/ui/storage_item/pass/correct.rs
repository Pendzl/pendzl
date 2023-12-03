// SPDX-License-Identifier: MIT
use pendzl::traits::AccountId;
#[derive(Debug)]
#[pendzl::storage_item]
pub struct OwnableData {
    #[lazy]
    pub owner: AccountId,
}

#[derive(Debug)]
#[pendzl::storage_item]
pub struct ProxyData {
    pub forward: AccountId,
}

fn main() {}
