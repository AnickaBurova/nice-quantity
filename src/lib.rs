//! Format quantities in to their specific units.
//! The fmt module has the function `custom_format` which can format any quantity if provided
//! with corresponding units and their factors.
pub mod seconds;
pub mod to_milliseconds;
#[cfg(feature = "micro")]
pub mod to_microseconds;

pub mod fmt;