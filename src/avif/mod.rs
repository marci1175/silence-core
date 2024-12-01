//! Provides AV1 encoding for images.
//! [AV1](https://en.wikipedia.org/wiki/AV1) (AOMedia Video 1) is a high efficiency video codec. It was originally made to transmit video calls.
pub mod encoding;

//Re-export the ravif crate.
pub use ravif;