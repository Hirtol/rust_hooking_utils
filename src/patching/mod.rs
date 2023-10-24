pub mod process;

/// Can patch code in memory, so long as the pointers given are from the same memory space.
pub struct LocalPatcher {
    patches: Vec<Patch>,
}

struct Patch {
    address: *mut u8,
    original_bytes: Box<[u8]>,
}

impl Patch {
    fn original_bytes(&self) -> &[u8] {
        &*self.original_bytes
    }
}

impl LocalPatcher {
    pub fn new() -> Self {
        Self { patches: vec![] }
    }

    /// Writes the given `bytes` to the given `local_ptr`.
    ///
    /// The `local_ptr` should be valid within the current memory space.
    ///
    /// # Safety
    ///
    /// `local_ptr` must be valid within the current memory space.
    /// The caller should also have the rights to `VirtualProtect` the memory at `local_ptr`.
    pub unsafe fn safe_write(&self, local_ptr: *mut u8, bytes: &[u8]) {
        use windows::Win32::System::Memory::{
            VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
        };

        let mut old: PAGE_PROTECTION_FLAGS = Default::default();
        let len = bytes.len();

        let _ = VirtualProtect(local_ptr as _, len, PAGE_EXECUTE_READWRITE, &mut old);

        std::slice::from_raw_parts_mut(local_ptr, len).copy_from_slice(bytes);

        let _ = VirtualProtect(local_ptr as _, len, old, &mut old);
    }

    /// Reads the given `length` of bytes from the given `local_ptr`.
    ///
    /// The `local_ptr` should be valid within the current memory space.
    ///
    /// # Safety
    ///
    /// `local_ptr` must be valid within the current memory space.
    pub unsafe fn safe_read_slice(&self, local_ptr: *const u8, length: usize) -> &[u8] {
        std::slice::from_raw_parts(local_ptr, length)
    }

    /// Read an arbitrary value from memory
    ///
    /// The `local_ptr` should be valid within the current memory space.
    pub unsafe fn read<T>(&self, local_ptr: *const T) -> &T {
        &*local_ptr
    }

    /// Writes the given `bytes` to the given `local_ptr`.
    ///
    /// Saves the original bytes at `local_ptr` so that they can be restored later.
    ///
    /// # Safety
    ///
    /// See [`safe_write`](#method.safe_write).
    pub unsafe fn patch(&mut self, local_ptr: *mut u8, bytes: &[u8]) {
        self.patches.push(Patch {
            address: local_ptr,
            original_bytes: std::slice::from_raw_parts(local_ptr, bytes.len()).into(),
        });

        self.safe_write(local_ptr, bytes)
    }
}

impl Drop for LocalPatcher {
    fn drop(&mut self) {
        for patch in self.patches.iter().rev() {
            unsafe {
                self.safe_write(patch.address, patch.original_bytes());
            }
        }
    }
}
