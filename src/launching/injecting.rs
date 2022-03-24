use anyhow::Context;
use dll_syringe::process::{BorrowedProcess, OwnedProcess, OwnedProcessModule, Process};
use dll_syringe::Syringe;
use std::path::Path;

/// Inject the given `payload_dll` into a running process.
///
/// # Returns
///
/// The [OwnedProcessModule] of the injected dll if successful.
pub fn inject_into_running_process(
    process: BorrowedProcess<'_>,
    payload_dll: &Path,
) -> anyhow::Result<OwnedProcessModule> {
    let syringe = Syringe::for_process(process.try_to_owned()?);

    let injected_module = syringe.find_or_inject(payload_dll)?;

    Ok(injected_module.try_to_owned()?)
}

/// Inject the given `payload_dll` into a running process.
///
/// # Returns
///
/// The [OwnedProcessModule] of the injected dll if successful.
///
/// If the given name could not be found then an `Err` is returned instead.
pub fn inject_into_running_process_by_name(
    process_name: impl AsRef<str>,
    payload_dll: &Path,
) -> anyhow::Result<OwnedProcessModule> {
    let existing_process = OwnedProcess::find_first_by_name(process_name.as_ref())
        .context("Could not find a process with the given name")?;

    inject_into_running_process(existing_process.borrowed(), payload_dll)
}
