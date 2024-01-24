// SPDX-License-Identifier: MIT
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Rounding {
    Up,
    Down,
}
