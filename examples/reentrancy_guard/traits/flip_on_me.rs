pub use pendzl::contracts::reentrancy_guard::*;
use pendzl::traits::AccountId;

use ink::{
    contract_ref,
    env::DefaultEnvironment,
};
pub type FlipOnMeRef = contract_ref!(FlipOnMe, DefaultEnvironment);

#[ink::trait_definition]
pub trait FlipOnMe {
    #[ink(message)]
    fn flip_on_target(&mut self, callee: AccountId) -> Result<(), ReentrancyGuardError>;
}
