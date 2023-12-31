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

pub use crate::traits::access_control::*;
use pendzl::traits::AccountId;

use ink::{
    contract_ref,
    env::DefaultEnvironment,
};
pub type AccessControlEnumerableRef = contract_ref!(AccessControlEnumerable, DefaultEnvironment);

/// Extension of AccessControl that allows enumerating the members of each role.
#[ink::trait_definition]
pub trait AccessControlEnumerable {
    /// Returns one of the accounts that have `role`.
    ///
    /// Role bearers are not sorted in any particular way, and their
    /// ordering may change at any point.
    #[ink(message)]
    fn get_role_member(&self, role: RoleType, index: u32) -> Option<AccountId>;

    /// Returns the number of accounts that have `role`.
    /// Can be used together with {get_role_member} to enumerate
    /// all bearers of a role.
    #[ink(message)]
    fn get_role_member_count(&self, role: RoleType) -> u32;
}
