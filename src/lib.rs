#![doc = include_str!("../README.md")]
pub use error::*;
pub use model::*;

pub mod error;
pub mod model;

#[cfg(not(feature = "blocking"))]
pub use async_lib::*;
#[cfg(not(feature = "blocking"))]
pub mod async_lib;

#[cfg(feature = "blocking")]
pub use blocking_lib::*;
#[cfg(feature = "blocking")]
pub mod blocking_lib;
