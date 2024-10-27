use crate::patching::process::GameProcess;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
};
use windows::Win32::System::Threading::{
    GetCurrentThreadId, OpenThread, ResumeThread, SuspendThread, THREAD_ALL_ACCESS,
};

pub unsafe fn suspend_all_threads<T>(
    process: GameProcess,
    critical_section: impl FnOnce() -> eyre::Result<T>,
) -> eyre::Result<T> {
    let snapshot_handle: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, process.pid)?;
    let current_thread_id = GetCurrentThreadId();

    if !snapshot_handle.is_invalid() {
        let mut to_resume = Vec::new();
        let mut entry = THREADENTRY32::default();
        entry.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;

        Thread32First(snapshot_handle, &mut entry)?;

        loop {
            if entry.th32ThreadID != current_thread_id && entry.th32OwnerProcessID == process.pid {
                // tracing::info!("Going to pause: {} - {current_thread_id:?}", entry.th32ThreadID);
                let thread_handle = OpenThread(THREAD_ALL_ACCESS, None, entry.th32ThreadID)?;
                if !thread_handle.is_invalid() {
                    to_resume.push(entry.th32ThreadID);
                    SuspendThread(thread_handle);
                    CloseHandle(thread_handle)?;
                }
            } else if entry.th32ThreadID == current_thread_id {
                log::info!(
                    "Skipping pause: {} - {current_thread_id:?}",
                    entry.th32ThreadID
                );
            }

            entry.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;

            if let Err(_) = Thread32Next(snapshot_handle, &mut entry) {
                break;
            }
        }

        let out = critical_section()?;

        for thread_id in to_resume {
            let thread_handle = OpenThread(THREAD_ALL_ACCESS, None, thread_id)?;
            if !thread_handle.is_invalid() {
                ResumeThread(thread_handle);
                CloseHandle(thread_handle)?;
            }
        }

        CloseHandle(snapshot_handle)?;

        Ok(out)
    } else {
        CloseHandle(snapshot_handle)?;
        eyre::bail!("Failed");
    }
}

pub unsafe fn resume_all_threads(process: GameProcess) -> eyre::Result<()> {
    let snapshot_handle: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, process.pid)?;
    let current_thread_id = GetCurrentThreadId();

    if !snapshot_handle.is_invalid() {
        let mut entry = THREADENTRY32::default();
        entry.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;

        Thread32First(snapshot_handle, &mut entry)?;

        loop {
            // tracing::info!("Going to unpause: {}", entry.th32ThreadID);
            if entry.th32ThreadID != current_thread_id && entry.th32OwnerProcessID == process.pid {
                let thread_handle = OpenThread(THREAD_ALL_ACCESS, None, entry.th32ThreadID)?;
                if !thread_handle.is_invalid() {
                    ResumeThread(thread_handle);
                    CloseHandle(thread_handle)?;
                }
            }

            entry.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;

            if let Err(_) = Thread32Next(snapshot_handle, &mut entry) {
                break;
            }
        }
    }

    CloseHandle(snapshot_handle)?;

    Ok(())
}
