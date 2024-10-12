use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::time::Duration;
use std::{ffi::OsString, mem, os::windows::ffi::OsStringExt};

use thiserror::Error;
use windows::Win32::Foundation::{CloseHandle, BOOL, HANDLE, HMODULE, HWND, LPARAM};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, MODULEENTRY32W, TH32CS_SNAPMODULE,
    TH32CS_SNAPMODULE32,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetForegroundWindow, GetWindow, GetWindowTextW, GetWindowThreadProcessId,
    IsWindow, IsWindowVisible, GW_OWNER,
};

pub type Result<T> = std::result::Result<T, ProcessErrorKind>;

#[derive(Debug, Error)]
pub enum ProcessErrorKind {
    #[error("The requested resource did not exist: {0:#X}")]
    MemoryRead(usize),

    #[error("The requested write could not be processed: {0:#X}")]
    MemoryWrite(usize),

    #[error("CreateToolhelp32Snapshot returned INVALID_HANDLE_VALUE")]
    InvalidHandleValue,
    #[error("Module is not a local module")]
    ModuleNotLocal,

    #[error("Unknown module: {0}")]
    UnknownModule(String),

    #[error(transparent)]
    OtherErr(#[from] windows::core::Error),

    #[error(transparent)]
    Any(#[from] eyre::Error),
}

#[derive(Debug, Clone, Copy)]
pub struct GameProcess {
    pub handle: HANDLE,
    pub pid: u32,
}

impl GameProcess {
    pub fn current_process() -> Self {
        Self::new(unsafe { windows::Win32::System::Threading::GetCurrentProcess() })
    }

    pub fn new(handle: HANDLE) -> Self {
        let pid = unsafe { windows::Win32::System::Threading::GetProcessId(handle) };
        GameProcess { handle, pid }
    }

    pub fn is_current(&self) -> bool {
        self.pid == unsafe { windows::Win32::System::Threading::GetCurrentProcessId() }
    }

    /// Read from `ptr` into `buf` up to `buf.len()` bytes.
    ///
    /// If fewer than `buf.len()` bytes are read an error is returned.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `ptr` is valid within the process' memory space.
    #[inline]
    pub unsafe fn read_absolute_buffer(&self, ptr: *mut u8, buffer: &mut [u8]) -> Result<usize> {
        let mut amount_read: usize = 0;

        ReadProcessMemory(
            self.handle,
            ptr as *const _,
            buffer.as_mut_ptr() as *mut _,
            buffer.len(),
            Some(&mut amount_read as *mut _),
        )?;

        if amount_read != 0 {
            Ok(amount_read)
        } else {
            Err(ProcessErrorKind::MemoryRead(ptr as usize))
        }
    }

    /// Write to the absolute pointer within the process' memory space.
    ///
    /// # Safety
    /// The caller must ensure the `ptr` is within the bounds of the process.
    #[inline]
    pub unsafe fn write_absolute_buffer(&mut self, ptr: *mut u8, buffer: &[u8]) -> Result<()> {
        let mut amount_read: usize = 0;

        WriteProcessMemory(
            self.handle,
            ptr as *const _,
            buffer.as_ptr() as *const _,
            buffer.len(),
            Some(&mut amount_read as *mut _),
        )?;

        if amount_read != buffer.len() {
            Err(ProcessErrorKind::MemoryWrite(ptr as usize))
        } else {
            Ok(())
        }
    }

    /// Get all modules from the process
    pub fn get_modules(&self) -> Result<Vec<Module>> {
        let module: HANDLE = unsafe {
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, self.pid)
                .map_err(|_| ProcessErrorKind::InvalidHandleValue)?
        };

        let mut entry = MODULEENTRY32W::default();

        entry.dwSize = mem::size_of::<MODULEENTRY32W>() as u32;
        let mut result = vec![];

        while let Ok(()) = unsafe { Module32NextW(module, &mut entry) } {
            match Module::new(*self, entry) {
                Ok(module) => result.push(module),
                Err(err) => log::debug!("Failed module initialization: {}", err),
            }
        }
        // Cleanup handle
        unsafe { CloseHandle(module)? };

        Ok(result)
    }

    /// Get the module with the given name from the process
    pub fn get_module(&self, module_name: &str) -> Result<Module> {
        let modules = self.get_modules()?;

        for module in modules {
            let name = OsString::from_wide(&module.entry.szModule[..]).into_string();
            let name = match name {
                Err(e) => {
                    log::warn!("Couldn't convert into String: {:?}", e);
                    continue;
                }
                Ok(s) => s,
            };

            if name.contains(module_name) {
                log::trace!(
                    "Base address of {}: {:#X} @ size of {:#X}",
                    name,
                    module.base() as usize,
                    module.size()
                );

                return Ok(module);
            }
        }

        Err(ProcessErrorKind::UnknownModule(module_name.into()))
    }

    /// Get the base module of this process.
    pub fn get_base_module(&self) -> Result<Module> {
        let snapshot: HANDLE = unsafe {
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, self.pid)
                .map_err(|_| ProcessErrorKind::InvalidHandleValue)?
        };

        let mut entry = MODULEENTRY32W::default();

        entry.dwSize = mem::size_of::<MODULEENTRY32W>() as u32;

        unsafe { Module32FirstW(snapshot, &mut entry)? };

        let final_module = Module::new(*self, entry)?;

        // Cleanup handle
        unsafe { CloseHandle(snapshot)? };

        Ok(final_module)
    }

    /// Get the main window of this process
    ///
    /// Will keep checking every 100ms until either the `timeout` has been reached, or a [Window] has been found.
    pub fn get_main_window_blocking(&self, timeout: Option<Duration>) -> Option<Window> {
        let start = std::time::Instant::now();
        let timeout = timeout.unwrap_or_else(|| Duration::MAX);
        loop {
            let wnd = self.get_main_window();

            if wnd.is_some() {
                break wnd;
            } else {
                std::thread::sleep(Duration::from_millis(100));
                if start.elapsed() > timeout {
                    break None;
                }
            }
        }
    }

    /// Get the main window of this process
    ///
    /// If no such window yet exists this will return [None]
    pub fn get_main_window(&self) -> Option<Window> {
        struct HandleData {
            process_id: u32,
            console_window: Option<HWND>,
            window: Option<HWND>,
        }

        unsafe extern "system" fn enum_callback(handle: HWND, param: LPARAM) -> BOOL {
            unsafe fn is_main_window(handle: HWND) -> bool {
                // Check that there is no parent window for this handle AND that it is visible.
                GetWindow(handle, GW_OWNER).is_err() && IsWindowVisible(handle).as_bool()
            }

            let data = &mut *(param.0 as *mut HandleData);
            let mut proc_id = 0;
            let _ = GetWindowThreadProcessId(handle, Some(&mut proc_id));

            if data.process_id != proc_id
                || data
                    .console_window
                    .as_ref()
                    .map(|&w| w == handle)
                    .unwrap_or_default()
                || !is_main_window(handle)
            {
                true.into()
            } else {
                data.window = Some(handle);
                false.into()
            }
        }

        let mut data = HandleData {
            process_id: self.pid,
            console_window: unsafe {
                let wnd = windows::Win32::System::Console::GetConsoleWindow();
                if wnd.is_invalid() {
                    None
                } else {
                    Some(wnd)
                }
            },
            window: None,
        };

        unsafe {
            // Explicitly ignore the result as this function returns an `Err` whenever we find the main function...
            let _ = EnumWindows(
                Some(enum_callback),
                LPARAM((&mut data as *mut HandleData) as isize),
            );
        }

        data.window.map(Window)
    }

    /// Return all windows associated with this process.
    pub fn get_windows(&self) -> Result<Vec<Window>> {
        struct HandleData {
            process_id: u32,
            windows: Vec<Window>,
        }

        unsafe extern "system" fn enum_callback(handle: HWND, param: LPARAM) -> BOOL {
            let data = &mut *(param.0 as *mut HandleData);
            let mut proc_id = 0;
            let _ = GetWindowThreadProcessId(handle, Some(&mut proc_id));

            if data.process_id == proc_id {
                data.windows.push(Window(handle))
            }

            true.into()
        }

        let mut data = HandleData {
            process_id: self.pid,
            windows: Default::default(),
        };

        unsafe {
            EnumWindows(
                Some(enum_callback),
                LPARAM((&mut data as *mut HandleData) as isize),
            )?;
        }

        Ok(data.windows)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Window(pub HWND);

impl Window {
    /// Return the current title of the window
    ///
    /// Will return a lossy `String` conversion.
    pub fn title(&self) -> String {
        self.title_os().to_string_lossy().into_owned()
    }

    /// Return the current title of the window
    pub fn title_os(&self) -> OsString {
        let mut text = [0; 256];
        let len = unsafe { GetWindowTextW(self.0, &mut text) };
        OsString::from_wide(&text[0..len as usize])
    }

    /// Check whether this current window is on the foreground.
    pub fn is_foreground_window(&self) -> bool {
        unsafe { GetForegroundWindow() == self.0 }
    }

    /// Check whether this [Window] is still valid (e.g., it exists)
    ///
    /// # Note
    ///
    /// Technically this [HWND] could be re-used for a different window, so this is not a guarantee of correctness!
    pub fn is_valid(&self) -> bool {
        unsafe { IsWindow(self.0).as_bool() }
    }

    /// Check whether this [Window] is visible (e.g., it exists)
    pub fn is_visible(&self) -> bool {
        unsafe { IsWindowVisible(self.0).as_bool() }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub parent: GameProcess,
    pub entry: MODULEENTRY32W,
    name: String,
}

impl Module {
    /// Create a new module from the given handle and module entry
    ///
    /// The `parent_handle` should refer to the owning process.
    pub fn new(parent: GameProcess, entry: MODULEENTRY32W) -> eyre::Result<Self> {
        let name = OsString::from_wide(&entry.szModule[..])
            .into_string()
            .map_err(|e| eyre::eyre!("Failed to convert name: {:?}", e))?;

        Ok(Module {
            entry,
            parent,
            name,
        })
    }

    pub fn is_local(&self) -> bool {
        self.parent.is_current()
    }

    /// The base address of the module in the parent process' address space
    pub fn base(&self) -> *mut u8 {
        self.entry.modBaseAddr
    }

    /// The total size of the module in the parent process' address space
    pub fn size(&self) -> usize {
        self.entry.modBaseSize as usize
    }

    /// The name of the module
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The path in the filesystem to the module
    pub fn module_path(&self) -> PathBuf {
        PathBuf::from(OsString::from_wide(&self.entry.szExePath[..]))
    }

    /// Returns the handle to this module
    pub fn module_handle(&self) -> HMODULE {
        self.entry.hModule
    }

    unsafe fn ptr_to_relative_addr(&self, ptr: *mut u8) -> isize {
        ptr.offset_from(self.base())
    }

    unsafe fn offset_to_absolute_addr(&self, offset: isize) -> *mut u8 {
        self.base().offset(offset)
    }
}

impl Module {
    /// Turn this module into a [LocalModule] reference, if this [Module] was taken from a local process.
    pub fn to_local(self) -> Result<LocalModule> {
        self.try_into()
    }

    /// Read relative to this module's base address
    ///
    /// # Safety
    ///
    /// The caller must ensure the `offset` is within the bounds of the module & parent process.
    pub unsafe fn read_relative<T: Sized>(&self, offset: usize) -> Result<T> {
        let offset_ptr = self.offset_to_absolute_addr(offset as isize);

        self.read_absolute(offset_ptr)
    }

    /// Read from an absolute pointer within the parent process' memory space.
    ///
    /// # Safety
    ///
    /// The caller must ensure the `ptr` is within the bounds of the parent process.
    pub unsafe fn read_absolute<T: Sized>(&self, ptr: *mut u8) -> Result<T> {
        let mut read = std::mem::MaybeUninit::uninit();
        // Aliasing `read` here, probably not good >.>
        let buffer =
            std::slice::from_raw_parts_mut(read.as_mut_ptr() as *mut u8, std::mem::size_of::<T>());

        let _ = self.read_absolute_buffer(ptr, buffer)?;

        Ok(read.assume_init())
    }

    /// Read from an absolute pointer within the parent process' memory space.
    ///
    /// # Safety
    ///
    /// The caller must ensure the `ptr` is within the bounds of the module & parent process.
    #[inline]
    pub unsafe fn read_absolute_buffer(&self, ptr: *mut u8, buffer: &mut [u8]) -> Result<usize> {
        self.parent.read_absolute_buffer(ptr, buffer)
    }

    /// Write to the absolute pointer within the parent process' memory space.
    ///
    /// # Safety
    /// The caller must ensure the `ptr` is within the bounds of the module & parent process.
    pub unsafe fn write_absolute_buffer(&mut self, ptr: *mut u8, buffer: &mut [u8]) -> Result<()> {
        self.parent.write_absolute_buffer(ptr, buffer)
    }

    pub unsafe fn write_absolute<T: Sized>(&mut self, ptr: *mut u8, item: &T) -> Result<()> {
        let byte_slice =
            std::slice::from_raw_parts(item as *const T as *const u8, std::mem::size_of::<T>());
        self.parent.write_absolute_buffer(ptr, byte_slice)
    }
}

/// A [Module] which is loaded in the current process' address space.
///
/// Used for scanning for patterns until we figure out how to do something like
/// [this from SO](https://stackoverflow.com/questions/13666110/efficiently-scanning-memory-of-a-process)
/// or
/// [this from Guided Hacking](https://guidedhacking.com/threads/external-internal-pattern-scanning-guide.14112/)
///
#[repr(transparent)]
pub struct LocalModule(Module);

impl LocalModule {
    /// Try to obtain a local module from the given general [Module]
    ///
    /// Will fail if said module was not obtained from a local process.
    pub fn new(module: Module) -> Result<Self> {
        Self::try_from(module)
    }

    /// Returns this module's address space as a byte slice.
    ///
    /// # Safety
    ///
    /// This is unsafe since at any time the parent process could drop the module, and this byte slice would then
    /// refer to invalid memory.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.base(), self.size()) }
    }

    /// Scan for a particular byte pattern in the module.
    /// Will return a pointer to the first occurrence of the pattern.
    pub fn scan_for_pattern(&self, pattern: &str) -> eyre::Result<*mut u8> {
        let offset = patternscan::scan_first_match(std::io::Cursor::new(self.as_bytes()), pattern)?
            .ok_or_else(|| eyre::eyre!("Couldn't find pattern"))?;

        unsafe { Ok(self.base().add(offset)) }
    }

    /// Will scan for a particular pattern after the provided pointer.
    ///
    /// # Safety
    ///
    /// The provided pointer must be within the bounds of the module.
    pub unsafe fn scan_for_pattern_after(
        &self,
        after: *mut u8,
        pattern: &str,
    ) -> eyre::Result<*mut u8> {
        let base_offset = self.ptr_to_relative_addr(after) as usize;
        let to_scan = &self.as_bytes()[base_offset..];

        let offset = patternscan::scan_first_match(std::io::Cursor::new(to_scan), pattern)?
            .ok_or_else(|| eyre::eyre!("Couldn't find pattern"))?;

        Ok(self.base().add(base_offset + offset))
    }

    /// Scan for a particular byte pattern in the module.
    /// Will return all occurrences of the pattern.
    pub fn scan_for_all_pattern(&self, pattern: &str) -> eyre::Result<Vec<*mut u8>> {
        let offsets = patternscan::scan(std::io::Cursor::new(self.as_bytes()), pattern)?;

        Ok(offsets
            .into_iter()
            .map(|offset| unsafe { self.base().add(offset) })
            .collect())
    }
}

impl TryFrom<Module> for LocalModule {
    type Error = ProcessErrorKind;

    fn try_from(module: Module) -> std::result::Result<Self, Self::Error> {
        if module.is_local() {
            Ok(Self(module))
        } else {
            Err(ProcessErrorKind::ModuleNotLocal)
        }
    }
}

impl Deref for LocalModule {
    type Target = Module;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LocalModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
