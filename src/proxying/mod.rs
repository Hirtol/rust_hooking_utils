pub mod dinput8;

/// Export a `DllMain` function for the current library.
///
/// This is the standard starting point for creating a proxy DLL.
///
/// Will spawn a new thread for the `attach` function to run in, as well as catch any panics that the `attach`/`detach`
/// function calls may cause.
///
/// # Example
/// ```norun
/// fn attach(hinst_dll: windows::Win32::Foundation::HMODULE) -> eyre::Result<()> {
///     println!("Hello World!");
///     Ok(())
/// }
///
/// fn detach(hinst_dll: windows::Win32::Foundation::HMODULE) -> eyre::Result<()> {
///     println!("Goodbye World!");
///     Ok(())
/// }
///
/// rust_hooking_utils::dll_main!(attach, detach)
/// ```
#[macro_export]
macro_rules! dll_main {
    ($attach:path, $detach:path) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "system" fn DllMain(
            hinst_dll: windows::Win32::Foundation::HMODULE,
            fdw_reason: u32,
            lpv_reserved: *const std::ffi::c_void,
        ) -> i32 {
            // Hack to get around sending the pointer cross-thread.
            let hinst_pointer = hinst_dll.0 as usize;

            match fdw_reason {
                windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH => {
                    // start loading
                    let _ =
                        windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls(hinst_dll);

                    if let Err(e) = std::panic::catch_unwind(|| {
                        std::thread::spawn(move || {
                            let hinst = windows::Win32::Foundation::HMODULE(
                                hinst_pointer as *mut core::ffi::c_void,
                            );
                            match $attach(hinst) {
                                Ok(_) => {}
                                Err(e) => eprintln!("`dll_attach` returned an Err: {:#?}", e),
                            }
                        })
                    }) {
                        eprintln!("`dll_attach` has panicked: {:#?}", e);
                    }

                    true as i32
                }
                windows::Win32::System::SystemServices::DLL_PROCESS_DETACH => {
                    // lpv_reserved is null, then we're still in a consistent state and we can clean up safely.
                    if lpv_reserved.is_null() {
                        match std::panic::catch_unwind(|| {
                            let hinst = windows::Win32::Foundation::HMODULE(
                                hinst_pointer as *mut core::ffi::c_void,
                            );
                            $detach(hinst)
                        }) {
                            Err(e) => {
                                eprintln!("`dll_detach` has panicked: {:#?}", e);
                            }
                            Ok(r) => match r {
                                Ok(()) => {}
                                Err(e) => {
                                    eprintln!("`dll_detach` returned an Err: {:#?}", e);
                                }
                            },
                        }
                    }

                    true as i32
                }
                _ => true as i32,
            }
        }
    };
}
