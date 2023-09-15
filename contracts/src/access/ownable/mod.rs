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

pub use crate::{
    ownable,
    traits::ownable::*,
};
pub use ownable::Internal as _;
use pendzl::traits::{
    AccountId,
    Storage,
};

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct Data {
    #[lazy]
    pub owner: Option<AccountId>,
}

/// Throws if called by any account other than the owner.
// #[modifier_definition]
// pub fn only_owner<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
// where
//     T: Storage<Data>,
//     F: FnOnce(&mut T) -> Result<R, E>,
//     E: From<OwnableError>,
// {
//     if instance.data().owner.get_or_default() != Some(T::env().caller()) {
//         return Err(From::from(OwnableError::CallerIsNotOwner))
//     }
//     body(instance)
// }

pub trait OwnableImpl {
    fn owner(&self) -> Option<AccountId>;
    fn renounce_ownership(&mut self) -> Result<(), OwnableError>;
    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError>;
}

impl<T: Storage<Data> + Internal> OwnableImpl for T {
    fn owner(&self) -> Option<AccountId> {
        self.data().owner.get_or_default()
    }

    fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
        self._only_owner()?;
        let old_owner = self.data().owner.get_or_default();
        self.data().owner.set(&None);
        self._emit_ownership_transferred_event(old_owner, None);
        Ok(())
    }

    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
        self._only_owner()?;
        let old_owner = self.data().owner.get_or_default();
        self.data().owner.set(&Some(new_owner));
        self._emit_ownership_transferred_event(old_owner, Some(new_owner));
        Ok(())
    }
}

pub trait Internal {
    /// User must override this method in their contract.
    fn _emit_ownership_transferred_event(&self, _previous: Option<AccountId>, _new: Option<AccountId>);

    fn _init_with_owner(&mut self, owner: AccountId);

    fn _only_owner(&self) -> Result<(), OwnableError>;
}

pub trait InternalImpl: Storage<Data> + Internal {
    fn _emit_ownership_transferred_event(&self, _previous: Option<AccountId>, _new: Option<AccountId>) {}

    fn _init_with_owner(&mut self, owner: AccountId) {
        self.data().owner.set(&Some(owner));
        Internal::_emit_ownership_transferred_event(self, None, Some(owner));
    }

    fn _only_owner(&self) -> Result<(), OwnableError> {
        if Some(Self::env().caller()) != self.data().owner.get_or_default() {
            return Err(OwnableError::CallerIsNotOwner)
        }
        Ok(())
    }
}
