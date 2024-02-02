// SPDX-License-Identifier: MIT
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

use ink::env::hash::{Blake2x256, CryptoHash, HashOutput};

#[cfg(feature = "std")]
use crate::traits::{AccountId, Balance, Timestamp};
#[cfg(feature = "std")]
use ink::env::{test::DefaultAccounts, DefaultEnvironment, Environment};
use ink::primitives::{Clear, Hash};

pub fn encoded_into_hash<T>(entity: &T) -> Hash
where
    T: scale::Encode,
{
    let mut result = Hash::CLEAR_HASH;
    let len_result = result.as_ref().len();
    let encoded = entity.encode();
    let len_encoded = encoded.len();
    if len_encoded <= len_result {
        result.as_mut()[..len_encoded].copy_from_slice(&encoded);
        return result;
    }
    let mut hash_output =
        <<Blake2x256 as HashOutput>::Type as Default>::default();
    <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
    let copy_len = core::cmp::min(hash_output.len(), len_result);
    result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
    result
}

/// For calculating the event topic hash.
pub struct PrefixedValue<'a, 'b, T> {
    pub prefix: &'a [u8],
    pub value: &'b T,
}

impl<X> scale::Encode for PrefixedValue<'_, '_, X>
where
    X: scale::Encode,
{
    #[inline]
    fn size_hint(&self) -> usize {
        self.prefix.size_hint() + self.value.size_hint()
    }

    #[inline]
    fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
        self.prefix.encode_to(dest);
        self.value.encode_to(dest);
    }
}

#[cfg(feature = "std")]
pub fn accounts() -> DefaultAccounts<DefaultEnvironment> {
    ink::env::test::default_accounts::<DefaultEnvironment>()
}

#[cfg(feature = "std")]
pub fn change_caller(
    new_caller: <DefaultEnvironment as Environment>::AccountId,
) {
    ink::env::test::set_caller::<ink::env::DefaultEnvironment>(new_caller);
}

#[cfg(feature = "std")]
pub fn set_account_balance(account: AccountId, balance: Balance) {
    ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(
        account, balance,
    );
}

#[cfg(feature = "std")]
pub fn get_account_balance(account: AccountId) -> Balance {
    ink::env::test::get_account_balance::<DefaultEnvironment>(account)
        .expect("Cannot get account balance")
}

#[cfg(feature = "std")]
pub fn set_value_transferred(value: Balance) {
    ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(
        value,
    );
}

#[cfg(feature = "std")]
pub fn set_block_timestamp(timestamp: Timestamp) {
    ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(
        timestamp,
    );
}
