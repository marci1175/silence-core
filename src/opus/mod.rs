//! This feature allows opus encoding and decoding for efficient byte transfer, while not sacirifising audio quality.

pub mod decode;
pub mod encode;

/// Re-export the opus crate.
pub use opus;