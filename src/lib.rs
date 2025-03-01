#![doc = include_str!("../README.md")]
#![deny(warnings)]
#![deny(missing_docs)]

pub mod common;
pub mod err;

#[cfg(feature = "cmd")]
pub mod cmd;

#[cfg(feature = "ssh")]
pub mod ssh;

#[cfg(feature = "uau")]
#[cfg(target_os = "linux")]
pub mod uau;

#[cfg(feature = "http")]
pub mod http;

// DO NOT SET features!
pub mod algo;

// DO NOT SET features!
pub mod ende;

/////////////////////////////////////////////

pub use err::*;
