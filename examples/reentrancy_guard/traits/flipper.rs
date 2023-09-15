pub use openbrush::contracts::reentrancy_guard::*;
use openbrush::{
    modifiers,
    traits::AccountId,
};

use ink::{
    contract_ref,
    env::DefaultEnvironment,
};
pub type FlipperRef = contract_ref!(Flipper, DefaultEnvironment);

#[ink::trait_definition]
pub trait Flipper {
    #[ink(message)]
    fn get_value(&self) -> bool;

    #[ink(message)]
    #[openbrush::modifiers(non_reentrant)]
    fn flip(&mut self) -> Result<(), ReentrancyGuardError>;

    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn call_flip_on_me(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError>;
}
