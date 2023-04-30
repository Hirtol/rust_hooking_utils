use retour::static_detour;
use std::ffi::c_void;
use windows::core::HRESULT;
use windows::Win32::Foundation::HWND;

#[repr(C)]
#[derive(Debug)]
pub struct RAWINPUTDEVICE {
    pub us_usage_page: u16,
    pub us_usage: u16,
    pub dw_flags: u32,
    pub hwnd_target: HWND,
}

static_detour! {
    pub static D_GET_RAW_INPUT_DATA: extern "system" fn(isize, u32, *mut c_void, *mut u32, u32) -> HRESULT;
    pub static D_REGISTER_RAW_INPUT_DEV: extern "system" fn(*const RAWINPUTDEVICE, u32, u32) -> windows::Win32::Foundation::BOOL;
}

pub unsafe fn hook_raw_input_data(
    hook: impl Fn(isize, u32, *mut c_void, *mut u32, u32) -> HRESULT + Send + 'static,
) -> anyhow::Result<()> {
    // Need to define a direct link here due to windows-rs forcing a link to dinput8.dll when it loads the ui_input feature.
    #[link(name = "user32")]
    extern "system" {
        fn GetRawInputData(
            hrawinput: isize,
            uicommand: u32,
            pdata: *mut ::core::ffi::c_void,
            pcbsize: *mut u32,
            cbsizeheader: u32,
        ) -> u32;
    }

    D_GET_RAW_INPUT_DATA.initialize(std::mem::transmute(GetRawInputData as *const c_void), hook)?;

    D_GET_RAW_INPUT_DATA.enable()?;

    Ok(())
}

pub unsafe fn hook_register_raw_input(
    hook: impl Fn(*const RAWINPUTDEVICE, u32, u32) -> windows::Win32::Foundation::BOOL + Send + 'static,
) -> anyhow::Result<()> {
    #[link(name = "user32")]
    extern "system" {
        fn RegisterRawInputDevices(
            prawinputdevices: *const RAWINPUTDEVICE,
            uinumdevices: u32,
            cbsize: u32,
        ) -> windows::Win32::Foundation::BOOL;
    }

    D_REGISTER_RAW_INPUT_DEV.initialize(
        std::mem::transmute(RegisterRawInputDevices as *const c_void),
        hook,
    )?;

    D_REGISTER_RAW_INPUT_DEV.enable()?;

    Ok(())
}
