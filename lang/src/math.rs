// SPDX-License-Identifier: MIT
pub mod errors {

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum MathError {
        Underflow,
        Overflow,
        DivByZero,
    }
}
