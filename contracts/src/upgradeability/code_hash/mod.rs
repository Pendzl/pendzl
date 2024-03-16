include!("upgradeable_trait.rs");
include!("upgradeable_error.rs");

// #[cfg(all(feature = "upgradeable_impl", feature = "access_control_impl "))]
mod access_control_implementation;

#[cfg(all(feature = "upgradeable_impl", feature = "ownable_impl "))]
mod ownable_implementation;

// #[cfg(all(
//     feature = "upgradeable_impl",
//     feature = "access_control_impl "
// ))]
pub use access_control_implementation::*;

#[cfg(all(feature = "upgradeable_impl", feature = "ownable_impl "))]
pub use ownable_implementation::*;
