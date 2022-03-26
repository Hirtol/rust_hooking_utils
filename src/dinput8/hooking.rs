//! Helpers to create and manage DirectInput8 hooks.

use anyhow::Context;
use windows::core::Interface;
use windows::Win32::Devices::HumanInterfaceDevice::{
    GUID_SysKeyboard, GUID_SysMouse, IDirectInput8W, IDirectInputDevice8W,
    IDirectInputDevice8W_Vtbl, DI8DEVCLASS_ALL, DI8DEVCLASS_KEYBOARD, DIDEVICEINSTANCEW,
    DIDEVICEOBJECTDATA, DIEDFL_ATTACHEDONLY, DIK_P, DIK_W, DIRECTINPUT_VERSION,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

#[derive(Debug)]
pub enum DeviceType {
    Keyboard,
    Mouse,
}

/// Create a [IDirectInput8W]
///
/// In case the `proxy-dinput8` feature is enabled this will instead opt to call our proxy to call the function
///
/// [MDSN Docs](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/ee417799(v=vs.85))
pub fn get_dinput_interface() -> anyhow::Result<IDirectInput8W> {
    let executor_module = unsafe { GetModuleHandleW(None) };

    let mut direct_input: Option<IDirectInput8W> = None;

    #[cfg(not(feature = "proxy-dinput8"))]
    unsafe {
        windows::Win32::Devices::HumanInterfaceDevice::DirectInput8Create(
            executor_module,
            DIRECTINPUT_VERSION,
            &IDirectInput8W::IID,
            &mut direct_input as *mut _ as *mut *mut _,
            None,
        )
        .map_err(|e| anyhow::anyhow!("Failed to create DirectInput8 interface: {}", e))?;
    }

    #[cfg(feature = "proxy-dinput8")]
    unsafe {
        // We don't use windows::Win32::Devices::HumanInterfaceDevice::DirectInput8Create because it creates link
        // errors due to the duplicate definition of our own DirectInput8Create in `dinput8.rs`
        crate::dinput8::proxy::DirectInput8Create(
            executor_module,
            DIRECTINPUT_VERSION,
            &IDirectInput8W::IID,
            &mut direct_input as *mut _ as _,
            std::ptr::null_mut(),
        )
        .ok()
        .context("Failed to create DirectInput8")?;
    }

    direct_input.ok_or_else(|| anyhow::anyhow!("Failed to create DirectInput8"))
}

/// Create a [IDirectInputDevice8W].
///
/// For acquiring a `direct_input` instance refer to [get_dinput_interface].
///
/// [MDSN Docs](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/ee417816(v=vs.85))
pub fn create_dinput_device(
    direct_input: &IDirectInput8W,
    device_type: DeviceType,
) -> anyhow::Result<IDirectInputDevice8W> {
    let mut device = None;

    let guid = match device_type {
        DeviceType::Keyboard => &GUID_SysKeyboard,
        DeviceType::Mouse => &GUID_SysMouse,
    };

    unsafe {
        direct_input.CreateDevice(guid, &mut device, None)?;
    }

    device.ok_or_else(|| anyhow::anyhow!("Failed to create {:?} device", device_type))
}
