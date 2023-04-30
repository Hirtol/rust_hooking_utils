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
/// fn attach() -> anyhow::Result<()> {
///     println!("Hello World!");
///     Ok(())
/// }
///
/// fn detach() -> anyhow::Result<()> {
///     println!("Goodbye World!");
///     Ok(())
/// }
///
/// rust_hooking_utils::dll_main!(attach, detach)
/// ```
#[macro_export]
macro_rules! dll_main {
    ($attach:path, $detach:path) => {
        #[no_mangle]
        pub unsafe extern "system" fn DllMain(
            hinst_dll: windows::Win32::Foundation::HMODULE,
            fdw_reason: u32,
            lpv_reserved: *const std::ffi::c_void,
        ) -> i32 {
            match fdw_reason {
                windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH => {
                    // start loading
                    windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls(hinst_dll);

                    if let Err(e) = std::panic::catch_unwind(|| {
                        std::thread::spawn(move || match $attach() {
                            Ok(_) => {}
                            Err(e) => eprintln!("`dll_attach` returned an Err: {:#?}", e),
                        })
                    }) {
                        eprintln!("`dll_attach` has panicked: {:#?}", e);
                    }

                    true as i32
                }
                windows::Win32::System::SystemServices::DLL_PROCESS_DETACH => {
                    // lpv_reserved is null, then we're still in a consistent state and we can clean up safely.
                    if lpv_reserved.is_null() {
                        match std::panic::catch_unwind(|| $detach()) {
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
