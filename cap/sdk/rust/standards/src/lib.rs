#![allow(clippy::all)]

//! Contains standards for cap.
//!
//! Current alpha standards (not fully implemented and verified):
//! - `alpha-xtc`
//! - `alpha-sbt`
//! - `alpha-dip20`
//!     - `alpha-dip20-dank`

#[cfg(feature = "alpha-xtc")]
pub mod xtc;

#[cfg(feature = "alpha-sbt")]
pub mod sbt;

#[cfg(feature = "alpha-dip20")]
pub mod dip20;
