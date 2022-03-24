//! This library contains utilities and re-exports the dependencies.
pub use detour;
#[cfg(feature = "launching")]
pub use dll_syringe;
pub use patternscan;

#[cfg(feature = "launching")]
pub mod launching;

#[cfg(feature = "patching")]
pub mod patching;
