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
    psp22,
    psp22::extensions::flashmint,
    traits::{
        flashloan::*,
        psp22::*,
    },
};
pub use flashmint::Internal as _;
use ink::{
    env::{
        call::{
            build_call,
            ExecutionInput,
        },
        CallFlags,
        DefaultEnvironment,
    },
    prelude::vec::Vec,
};
use pendzl::traits::{
    AccountId,
    Balance,
    Storage,
    String,
};
pub use psp22::{
    Internal as _,
    InternalImpl as _,
    PSP22Impl,
};

pub trait FlashLenderImpl: Storage<psp22::Data> + psp22::Internal + Internal {
    fn max_flashloan_impl(&mut self, token: AccountId) -> Balance {
        if token == Self::env().account_id() {
            Balance::MAX - self._total_supply()
        } else {
            0
        }
    }

    fn flash_fee_impl(&self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
        if token != Self::env().account_id() {
            return Err(FlashLenderError::WrongTokenAddress)
        }
        Ok(self._get_fee(amount))
    }

    fn flashloan_impl(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError> {
        let fee = self.flash_fee_impl(token, amount)?;
        self._mint_to(receiver_account, amount)?;
        Internal::_on_flashloan(self, receiver_account, token, fee, amount, data)?;
        let this = Self::env().account_id();
        let current_allowance = self._allowance(&receiver_account, &this);
        if current_allowance < amount + fee {
            return Err(FlashLenderError::AllowanceDoesNotAllowRefund)
        }
        psp22::Internal::_approve_from_to(self, receiver_account, this, current_allowance - amount - fee)?;
        psp22::Internal::_burn_from(self, receiver_account, amount + fee)?;
        Ok(())
    }
}

pub trait Internal {
    fn _get_fee(&self, _amount: Balance) -> Balance;

    fn _on_flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        fee: Balance,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError>;
}

pub trait InternalImpl: Storage<psp22::Data> + Internal {
    fn _get_fee_impl(&self, _amount: Balance) -> Balance {
        0
    }

    fn _on_flashloan_impl(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        fee: Balance,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError> {
        let call_result = build_call::<DefaultEnvironment>()
            .call(receiver_account)
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .exec_input(
                ExecutionInput::new(ink::env::call::Selector::new(ink::selector_bytes!(
                    "FlashBorrower::on_flashloan"
                )))
                .push_arg(&Self::env().caller())
                .push_arg(token)
                .push_arg(amount)
                .push_arg(fee)
                .push_arg(data),
            )
            .returns::<Result<(), FlashBorrowerError>>()
            .try_invoke();

        let result = match call_result {
            Ok(Ok(Ok(_))) => Ok(()),
            Ok(Ok(Err(FlashBorrowerError::FlashloanRejected(message)))) => {
                Err(FlashLenderError::BorrowerRejected(message))
            }
            // Means unknown method
            Ok(Err(ink::LangError::CouldNotReadInput)) => Ok(()),
            // `NotCallable` means that the receiver is not a contract.
            Err(ink::env::Error::NotCallable) => Ok(()),
            _ => {
                Err(FlashLenderError::BorrowerRejected(String::from(
                    "Error while performing the `on_flashloan`",
                )))
            }
        };

        result
    }
}
