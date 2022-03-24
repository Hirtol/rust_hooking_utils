//! This library contains utilities and re-exports the dependencies.
pub use detour;
#[cfg(feature = "injection")]
pub use dll_syringe;
pub use patternscan;

#[cfg(feature = "injection")]
pub mod launching;

#[cfg(feature = "injection")]
pub mod injecting;
