// Copyright (c) 2023 Brushfam
// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::traits::errors::GovernanceError;

/// Extension of `Governor` for settings updatable through governance.
#[ink::trait_definition]
pub trait GovernorSettings {
    /// Sets the voting delay
    #[ink(message)]
    fn set_voting_delay(&mut self, new_voting_delay: u64) -> Result<(), GovernanceError>;

    /// Sets the voting period
    #[ink(message)]
    fn set_voting_period(&mut self, new_voting_period: u64) -> Result<(), GovernanceError>;

    /// Sets the proposal threshold
    #[ink(message)]
    fn set_proposal_threshold(&mut self, new_proposal_threshold: u128) -> Result<(), GovernanceError>;

    /// Returns the voting delay
    #[ink(message)]
    fn voting_delay(&self) -> u64;

    /// Returns the voting period
    #[ink(message)]
    fn voting_period(&self) -> u64;

    /// Returns the proposal threshold
    #[ink(message)]
    fn proposal_threshold(&self) -> u128;
}

use ink::{
    contract_ref,
    env::DefaultEnvironment,
};
pub type GovernorSettingsRef = contract_ref!(GovernorSettings, DefaultEnvironment);
