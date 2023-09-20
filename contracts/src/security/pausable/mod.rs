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
    pausable,
    traits::pausable::*,
};
pub use pausable::{
    Internal as _,
    InternalImpl as _,
    PausableImpl as _,
};
use pendzl::traits::{
    AccountId,
    Storage,
};

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct Data {
    #[lazy]
    pub paused: bool,
}

pub trait PausableImpl: Storage<Data> + Internal {
    fn paused_impl(&self) -> bool {
        self._paused()
    }
}

pub trait Internal {
    /// User must override those methods in their contract.
    fn _emit_paused_event(&self, _account: AccountId);

    fn _emit_unpaused_event(&self, _account: AccountId);

    fn _paused(&self) -> bool;

    /// Triggers stopped state.
    ///
    /// On success a `Paused` event is emitted.
    fn _pause(&mut self) -> Result<(), PausableError>;

    /// Returns to normal state.
    ///
    /// On success a `Unpaused` event is emitted.
    fn _unpause(&mut self) -> Result<(), PausableError>;

    /// Function which changes state to unpaused if paused and vice versa
    fn _switch_pause(&mut self) -> Result<(), PausableError>;

    fn _ensure_paused(&self) -> Result<(), PausableError>;

    fn _ensure_not_paused(&self) -> Result<(), PausableError>;
}

pub trait InternalImpl: Storage<Data> + Internal {
    fn _emit_paused_event_impl(&self, _account: AccountId) {}

    fn _emit_unpaused_event_impl(&self, _account: AccountId) {}

    fn _paused_impl(&self) -> bool {
        self.data().paused.get_or_default()
    }

    fn _pause_impl(&mut self) -> Result<(), PausableError> {
        Internal::_ensure_not_paused(self)?;
        self.data().paused.set(&true);
        Internal::_emit_paused_event(self, Self::env().caller());
        Ok(())
    }

    fn _unpause_impl(&mut self) -> Result<(), PausableError> {
        Internal::_ensure_paused(self)?;
        self.data().paused.set(&false);
        Internal::_emit_unpaused_event(self, Self::env().caller());
        Ok(())
    }

    fn _switch_pause_impl(&mut self) -> Result<(), PausableError> {
        if Internal::_paused(self) {
            Internal::_unpause(self)
        } else {
            Internal::_pause(self)
        }
    }

    fn _ensure_paused_impl(&self) -> Result<(), PausableError> {
        if !self.data().paused.get_or_default() {
            return Err(From::from(PausableError::NotPaused))
        }

        Ok(())
    }

    fn _ensure_not_paused_impl(&self) -> Result<(), PausableError> {
        if self.data().paused.get_or_default() {
            return Err(From::from(PausableError::Paused))
        }

        Ok(())
    }
}
