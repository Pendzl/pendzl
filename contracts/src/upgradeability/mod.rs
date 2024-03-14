include!("upgradeable_error.rs");
include!("upgradeable_trait.rs");

#[cfg(feature = "upgradeable_impl")]
pub mod implementation;
