use openbrush::contracts::traits::{
    ownable::*,
    psp22::{
        extensions::{
            burnable::*,
            metadata::*,
            mintable::*,
        },
        *,
    },
};

use ink::{
    contract_ref,
    env::DefaultEnvironment,
};
pub type SharesRef = contract_ref!(
    PSP22 + PSP22Mintable + PSP22Burnable + PSP22Metadata + Ownable,
    DefaultEnvironment
);

#[ink::trait_definition]
pub trait Shares: PSP22 + PSP22Mintable + PSP22Burnable + PSP22Metadata + Ownable {}
