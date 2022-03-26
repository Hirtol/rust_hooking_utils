//! This library contains utilities and re-exports the dependencies.

#![cfg_attr(doc_cfg, feature(doc_auto_cfg))]

pub use detour;
#[cfg(feature = "launching")]
pub use dll_syringe;
pub use patternscan;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

#[cfg(feature = "launching")]
pub mod launching;

#[cfg(feature = "patching")]
pub mod patching;

pub mod dinput8;

/// Retrieves the system directory of the current user.
///
/// Here 'pristine' `DLL`s can be found and loaded for proxying.
pub fn get_system_directory() -> anyhow::Result<PathBuf> {
    let mut buffer = [0; 512];
    // SAFETY: If the buffer is too small the written bytes will be larger than `buffer.len()`, and we will return an Err.
    // If for some reason the function fails, it will return `0`, and we return an Err.
    let written_bytes =
        unsafe { windows::Win32::System::SystemInformation::GetSystemDirectoryW(&mut buffer) };

    if written_bytes == 0 || written_bytes > buffer.len() as u32 {
        Err(anyhow::anyhow!(
            "Failed to get system directory, written_bytes: {}",
            written_bytes
        ))
    } else {
        Ok(PathBuf::from(OsString::from_wide(
            &buffer[..written_bytes as usize],
        )))
    }
}
