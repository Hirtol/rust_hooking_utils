//! Contains a pre-made proxy for dinput8.dll
//!
//! Enabling the respective feature, (e.g. `proxy-dinput8`) will automatically export the pre-requisite functions if the
//! parent crate is compiled as a `crate-type = ["cdylib"]`.

use std::ffi::c_void;

use libloading::os::windows::Symbol;
use once_cell::sync::Lazy;
use windows::core::{GUID, HRESULT};
use windows::Win32::Foundation::HMODULE;

use crate::get_system_directory;

pub type DINPUT8CREATE = unsafe extern "system" fn(
    HMODULE,
    u32,
    *const GUID,
    *mut *mut c_void,
    *const c_void,
) -> HRESULT;
pub type DLLCANUNLOADNOW = unsafe extern "system" fn() -> HRESULT;
pub type DLLGETCLASSOBJECT =
    unsafe extern "system" fn(*const GUID, *const GUID, *const *const c_void) -> HRESULT;
pub type DLLREGISTERSERVER = unsafe extern "system" fn() -> HRESULT;

pub type DLLUNREGISTERSERVER = unsafe extern "system" fn() -> HRESULT;

pub static DINPUT_MANAGER: Lazy<DirectInputProxyManager> =
    Lazy::new(|| DirectInputProxyManager::new().expect("Failed to initialize DirectInput"));

pub struct DirectInputProxyManager {
    _lib: libloading::Library,
    direct_input_8_create: Symbol<DINPUT8CREATE>,
    dll_can_unload_now: Symbol<DLLCANUNLOADNOW>,
    dll_get_class_object: Symbol<DLLGETCLASSOBJECT>,
    dll_register_server: Symbol<DLLREGISTERSERVER>,
    dll_unregister_server: Symbol<DLLUNREGISTERSERVER>,
}

unsafe impl Sync for DirectInputProxyManager {}

impl DirectInputProxyManager {
    pub fn new() -> eyre::Result<Self> {
        let dinput_path = get_system_directory()?.join("dinput8.dll");
        log::debug!("Loading `dinput8.dll` from {:?}", dinput_path);

        unsafe {
            let lib = libloading::Library::new(dinput_path)?;
            // Couldn't manage to make the lifetimes work, so into raw they go!
            Ok(Self {
                direct_input_8_create: lib.get::<DINPUT8CREATE>(b"DirectInput8Create")?.into_raw(),
                dll_can_unload_now: lib.get::<DLLCANUNLOADNOW>(b"DllCanUnloadNow")?.into_raw(),
                dll_get_class_object: lib
                    .get::<DLLGETCLASSOBJECT>(b"DllGetClassObject")?
                    .into_raw(),
                dll_register_server: lib
                    .get::<DLLREGISTERSERVER>(b"DllRegisterServer")?
                    .into_raw(),
                dll_unregister_server: lib
                    .get::<DLLUNREGISTERSERVER>(b"DllUnregisterServer")?
                    .into_raw(),
                _lib: lib,
            })
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn DirectInput8Create(
    h_inst: HMODULE,
    dw_version: u32,
    rii_dltf: *const GUID,
    ppv_out: *mut *mut c_void,
    punk_outer: *const c_void,
) -> HRESULT {
    log::trace!("DirectInput8Create called");
    (DINPUT_MANAGER.direct_input_8_create)(h_inst, dw_version, rii_dltf, ppv_out, punk_outer)
}

#[no_mangle]
pub unsafe extern "system" fn DllCanUnloadNow() -> HRESULT {
    log::trace!("DllCanUnloadNow called");
    (DINPUT_MANAGER.dll_can_unload_now)()
}

#[no_mangle]
pub unsafe extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *const *const c_void,
) -> HRESULT {
    log::trace!("DllGetClassObject called");
    (DINPUT_MANAGER.dll_get_class_object)(rclsid, riid, ppv)
}

#[no_mangle]
pub unsafe extern "system" fn DllRegisterServer() -> HRESULT {
    log::trace!("DllRegisterServer called");
    (DINPUT_MANAGER.dll_register_server)()
}

#[no_mangle]
pub unsafe extern "system" fn DllUnregisterServer() -> HRESULT {
    log::trace!("DllUnregisterServer called");
    (DINPUT_MANAGER.dll_unregister_server)()
}
