use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::System::Console::{
    AllocConsole, CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, FreeConsole, GetConsoleMode,
    GetStdHandle, STD_OUTPUT_HANDLE, SetConsoleMode,
};

static CONSOLE_ALLOCATED: AtomicBool = AtomicBool::new(false);

/// Allocate a Windows console.
pub fn alloc_console() -> eyre::Result<()> {
    if !CONSOLE_ALLOCATED.swap(true, Ordering::SeqCst) {
        unsafe { AllocConsole()? };
    }

    Ok(())
}

/// Enable console colors if the console is allocated.
pub fn enable_console_colors() {
    if CONSOLE_ALLOCATED.load(Ordering::SeqCst) {
        unsafe {
            // Get the stdout handle
            let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE).unwrap();

            // Call GetConsoleMode to get the current mode of the console
            let mut current_console_mode = CONSOLE_MODE(0);
            GetConsoleMode(stdout_handle, &mut current_console_mode).unwrap();

            // Set the new mode to include ENABLE_VIRTUAL_TERMINAL_PROCESSING for ANSI
            // escape sequences
            current_console_mode.0 |= ENABLE_VIRTUAL_TERMINAL_PROCESSING.0;

            // Call SetConsoleMode to set the new mode
            SetConsoleMode(stdout_handle, current_console_mode).unwrap();
        }
    }
}

/// Free the previously allocated Windows console.
pub fn free_console() -> eyre::Result<()> {
    if CONSOLE_ALLOCATED.swap(false, Ordering::SeqCst) {
        unsafe { FreeConsole()? };
    }

    Ok(())
}
