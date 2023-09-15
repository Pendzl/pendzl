use openbrush::contracts::traits::psp22::{
    extensions::{
        metadata::*,
        mintable::*,
    },
    *,
};

use ink::{
    contract_ref,
    env::DefaultEnvironment,
};
pub type StableCoinRef = contract_ref!(PSP22 + PSP22Metadata + PSP22Mintable, DefaultEnvironment);

#[ink::trait_definition]
pub trait StableCoin: PSP22 + PSP22Metadata + PSP22Mintable {}
