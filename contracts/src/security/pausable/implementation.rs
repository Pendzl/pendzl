// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use super::{
    Pausable, PausableError, PausableInternal, PausableStorage, Paused,
    Unpaused,
};
use pendzl::traits::StorageFieldGetter;

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct PausableData {
    #[lazy]
    pub paused: bool,
}

impl PausableStorage for PausableData {
    fn paused(&self) -> bool {
        self.paused.get().unwrap_or(false)
    }

    fn set_paused(&mut self, pause: bool) {
        self.paused.set(&pause);
    }
}

pub trait PausableDefaultImpl: PausableInternal {
    fn paused_default_impl(&self) -> bool {
        self._paused()
    }
}

pub trait PausableInternalDefaultImpl:
    StorageFieldGetter<PausableData>
where
    PausableData: PausableStorage,
{
    fn _paused_default_impl(&self) -> bool {
        self.data().paused()
    }

    fn _pause_default_impl(&mut self) -> Result<(), PausableError> {
        self._ensure_not_paused_default_impl()?;
        self.data().set_paused(true);
        Self::env().emit_event(Paused {
            account: Self::env().caller(),
        });
        Ok(())
    }

    fn _unpause_default_impl(&mut self) -> Result<(), PausableError> {
        self._ensure_paused_default_impl()?;
        self.data().set_paused(false);
        Self::env().emit_event(Unpaused {
            account: Self::env().caller(),
        });
        Ok(())
    }

    fn _ensure_paused_default_impl(&self) -> Result<(), PausableError> {
        if !self.data().paused.get_or_default() {
            return Err(From::from(PausableError::NotPaused));
        }

        Ok(())
    }

    fn _ensure_not_paused_default_impl(&self) -> Result<(), PausableError> {
        if self.data().paused.get_or_default() {
            return Err(From::from(PausableError::Paused));
        }

        Ok(())
    }
}
