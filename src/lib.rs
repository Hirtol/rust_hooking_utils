//! This library contains utilities and re-exports the dependencies.
#[cfg(feature = "game-scanner")]
pub use game_scanner;
#[cfg(feature = "injection")]
pub use dll_syringe;

pub use detour;
pub use patternscan;