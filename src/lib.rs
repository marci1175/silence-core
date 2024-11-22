#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//!
//! A crate to make voip services easier to create.
//! Silence-core provides core functions, helpers and type definitions for a voip service to work.
//! If you just want a quick and easy way to set up a voip service, you should use [silence](https://crates.io/crates/silence) instead as it provides a more complete (end-user friendly) approach.
//! This crate provides 2 main functionalities:
//!
//! * Type definitions:
//! This crate provides Type definitions and traits in order to make the handling of packets easier.
//!
//! * APIs for performing audio I/O:
//! The crate provides multiple ways to handle audio I/O on multiple platforms efficiently.
//!
//! A complete version of the documentation is available at [here](https://docs.rs/silence-core/latest).
//!

#[cfg(feature = "io")]
pub mod io;

#[doc(hidden)]
#[cfg(test)]
mod tests;

#[cfg(feature = "opus")]
pub mod opus;

#[cfg(feature = "opencv")]
pub mod cam;

#[cfg(feature = "av1")]
pub mod avif;