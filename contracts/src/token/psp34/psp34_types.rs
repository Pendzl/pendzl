// SPDX-License-Identifier: MIT
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum Id {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Bytes(Vec<u8>),
}

impl Id {
    fn to_id(&self) -> Self {
        match self {
            Id::Bytes(v) => Id::Bytes(v.clone()),
            Id::U8(v) => Id::U8(*v),
            Id::U16(v) => Id::U16(*v),
            Id::U32(v) => Id::U32(*v),
            Id::U64(v) => Id::U64(*v),
            Id::U128(v) => Id::U128(*v),
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::U8(0)
    }
}
