use std::os::windows::io::FromRawHandle;
use std::path::Path;

use anyhow::Context;
use dll_syringe::process::OwnedProcess;
pub use windows::Win32::System::Threading;
use windows::Win32::System::Threading::{PROCESS_INFORMATION, STARTUPINFOW};

pub mod injecting;

/// Launch the given executable within the provided `working_dir`.
///
/// # Returns
///
/// The owned process handle, which has full privileges within the spawned process' memory space.
pub fn launch_process(
    working_dir: &Path,
    exe_path: &Path,
    env: impl Iterator<Item = (String, String)>,
) -> anyhow::Result<OwnedProcess> {
    let env = std::env::vars()
        .chain(env)
        .fold(String::new(), |acc, (k, v)| format!("{}{}={}", acc, k, v))
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();

    let startup_info = STARTUPINFOW::default();
    let mut process_info = PROCESS_INFORMATION::default();

    unsafe {
        Threading::CreateProcessW(
            exe_path.as_os_str(),
            Default::default(),
            std::ptr::null(),
            std::ptr::null(),
            false,
            Threading::CREATE_UNICODE_ENVIRONMENT,
            env.as_ptr() as *const _,
            working_dir.as_os_str(),
            &startup_info,
            &mut process_info,
        )
        .ok()
        .context("Failed to create process")?;

        Ok(OwnedProcess::from_raw_handle(process_info.hProcess.0 as _))
    }
}
