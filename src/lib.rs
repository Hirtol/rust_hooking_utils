//! This library contains utilities and re-exports the dependencies.

#![cfg_attr(doc_cfg, feature(doc_auto_cfg))]

pub use detour;
#[cfg(feature = "launching")]
pub use dll_syringe;
pub use patternscan;

#[cfg(feature = "launching")]
pub mod launching;

#[cfg(feature = "patching")]
pub mod patching;

#[cfg(feature = "proxy")]
pub mod proxy;
