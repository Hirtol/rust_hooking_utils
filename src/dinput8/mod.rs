//! Contains all utilities related to creating a `dinput8.dll` proxy dll and/or hooking related functionality.
#[cfg(feature = "proxy-dinput8")]
pub mod proxy;

#[cfg(feature = "hooking-dinput8")]
pub mod hooking;
