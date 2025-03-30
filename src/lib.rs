//! This library contains utilities and re-exports the dependencies.
#![allow(unsafe_op_in_unsafe_fn)]

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

#[cfg(feature = "launching")]
pub use dll_syringe;

pub use patternscan;
pub use retour;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::{
    GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, GetModuleFileNameW, GetModuleHandleExW,
};

pub mod proxying;

#[cfg(feature = "launching")]
pub mod launching;

#[cfg(feature = "patching")]
pub mod patching;

pub mod raw_input;

pub mod pausing;
pub mod pointer;

/// Retrieves the system directory of the current user.
///
/// Here 'pristine' `DLL`s can be found and loaded for proxying.
pub fn get_system_directory() -> eyre::Result<PathBuf> {
    let mut buffer = [0; 512];
    // SAFETY: If the buffer is too small the written bytes will be larger than `buffer.len()`, and we will return an Err.
    // If for some reason the function fails, it will return `0`, and we return an Err.
    let written_bytes = unsafe {
        windows::Win32::System::SystemInformation::GetSystemDirectoryW(Some(&mut buffer))
    };

    if written_bytes == 0 || written_bytes > buffer.len() as u32 {
        Err(eyre::eyre!(
            "Failed to get system directory, written_bytes: {}",
            written_bytes
        ))
    } else {
        Ok(PathBuf::from(OsString::from_wide(
            &buffer[..written_bytes as usize],
        )))
    }
}

/// Retrieves the path to the given DLL module.
pub fn get_current_dll_path(
    hinst_dll: windows::Win32::Foundation::HMODULE,
) -> eyre::Result<PathBuf> {
    let mut file_path = [0; 512];

    unsafe {
        let path_len = GetModuleFileNameW(Some(hinst_dll), &mut file_path) as usize;
        let path = String::from_utf16(&file_path[0..path_len])?;
        Ok(path.into())
    }
}

/// Retrieves the current module, if it exists.
pub fn get_current_module() -> eyre::Result<HMODULE> {
    let mut result = HMODULE::default();

    unsafe {
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            windows::core::w!("get_current_module"),
            &mut result,
        )?
    }

    Ok(result)
}
