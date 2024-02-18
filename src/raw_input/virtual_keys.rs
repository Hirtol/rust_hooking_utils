/// All virtual keys as listed on [Microsoft's Website](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
///
/// These are able to be serialized/deserialized by [serde].
#[allow(non_camel_case_types)]
#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq, Hash,
)]
pub enum VirtualKey {
    VK_LBUTTON,
    VK_RBUTTON,
    VK_CANCEL,
    VK_MBUTTON,
    VK_XBUTTON1,
    VK_XBUTTON2,
    VK_BACK,
    VK_TAB,
    VK_CLEAR,
    VK_RETURN,
    VK_SHIFT,
    VK_CONTROL,
    VK_MENU,
    VK_PAUSE,
    VK_CAPITAL,
    VK_KANA,
    VK_HANGUEL,
    VK_HANGUL,
    VK_IME_ON,
    VK_JUNJA,
    VK_FINAL,
    VK_HANJA,
    VK_KANJI,
    VK_IME_OFF,
    VK_ESCAPE,
    VK_CONVERT,
    VK_NONCONVERT,
    VK_ACCEPT,
    VK_MODECHANGE,
    VK_SPACE,
    VK_PRIOR,
    VK_NEXT,
    VK_END,
    VK_HOME,
    VK_LEFT,
    VK_UP,
    VK_RIGHT,
    VK_DOWN,
    VK_SELECT,
    VK_PRINT,
    VK_EXECUTE,
    VK_SNAPSHOT,
    VK_INSERT,
    VK_DELETE,
    VK_HELP,
    VK_0,
    VK_1,
    VK_2,
    VK_3,
    VK_4,
    VK_5,
    VK_6,
    VK_7,
    VK_8,
    VK_9,
    VK_A,
    VK_B,
    VK_C,
    VK_D,
    VK_E,
    VK_F,
    VK_G,
    VK_H,
    VK_I,
    VK_J,
    VK_K,
    VK_L,
    VK_M,
    VK_N,
    VK_O,
    VK_P,
    VK_Q,
    VK_R,
    VK_S,
    VK_T,
    VK_U,
    VK_V,
    VK_W,
    VK_X,
    VK_Y,
    VK_Z,
    VK_LWIN,
    VK_RWIN,
    VK_APPS,
    VK_SLEEP,
    VK_NUMPAD0,
    VK_NUMPAD1,
    VK_NUMPAD2,
    VK_NUMPAD3,
    VK_NUMPAD4,
    VK_NUMPAD5,
    VK_NUMPAD6,
    VK_NUMPAD7,
    VK_NUMPAD8,
    VK_NUMPAD9,
    VK_MULTIPLY,
    VK_ADD,
    VK_SEPARATOR,
    VK_SUBTRACT,
    VK_DECIMAL,
    VK_DIVIDE,
    VK_F1,
    VK_F2,
    VK_F3,
    VK_F4,
    VK_F5,
    VK_F6,
    VK_F7,
    VK_F8,
    VK_F9,
    VK_F10,
    VK_F11,
    VK_F12,
    VK_F13,
    VK_F14,
    VK_F15,
    VK_F16,
    VK_F17,
    VK_F18,
    VK_F19,
    VK_F20,
    VK_F21,
    VK_F22,
    VK_F23,
    VK_F24,
    VK_NUMLOCK,
    VK_SCROLL,
    VK_LSHIFT,
    VK_RSHIFT,
    VK_LCONTROL,
    VK_RCONTROL,
    VK_LMENU,
    VK_RMENU,
    VK_BROWSER_BACK,
    VK_BROWSER_FORWARD,
    VK_BROWSER_REFRESH,
    VK_BROWSER_STOP,
    VK_BROWSER_SEARCH,
    VK_BROWSER_FAVORITES,
    VK_BROWSER_HOME,
    VK_VOLUME_MUTE,
    VK_VOLUME_DOWN,
    VK_VOLUME_UP,
    VK_MEDIA_NEXT_TRACK,
    VK_MEDIA_PREV_TRACK,
    VK_MEDIA_STOP,
    VK_MEDIA_PLAY_PAUSE,
    VK_LAUNCH_MAIL,
    VK_LAUNCH_MEDIA_SELECT,
    VK_LAUNCH_APP1,
    VK_LAUNCH_APP2,
    VK_OEM_1,
    VK_OEM_PLUS,
    VK_OEM_COMMA,
    VK_OEM_MINUS,
    VK_OEM_PERIOD,
    VK_OEM_2,
    VK_OEM_3,
    VK_OEM_4,
    VK_OEM_5,
    VK_OEM_6,
    VK_OEM_7,
    VK_OEM_8,
    VK_OEM_102,
    VK_PROCESSKEY,
    VK_PACKET,
    VK_ATTN,
    VK_CRSEL,
    VK_EXSEL,
    VK_EREOF,
    VK_PLAY,
    VK_ZOOM,
    VK_PA1,
    VK_OEM_CLEAR,
}

impl VirtualKey {
    /// Using the [Into] implementation to convert this [VirtualKey] to a `usize`
    pub fn to_usize(self) -> usize {
        let val: u16 = self.into();
        val as usize
    }

    pub fn to_virtual_key(self) -> windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY {
        self.into()
    }
}

impl From<VirtualKey> for windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY {
    fn from(value: VirtualKey) -> Self {
        windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(value.into())
    }
}

impl From<VirtualKey> for u16 {
    fn from(value: VirtualKey) -> Self {
        use VirtualKey::*;
        match value {
            VK_LBUTTON => 0x01,
            VK_RBUTTON => 0x02,
            VK_CANCEL => 0x03,
            VK_MBUTTON => 0x04,
            VK_XBUTTON1 => 0x05,
            VK_XBUTTON2 => 0x06,
            VK_BACK => 0x08,
            VK_TAB => 0x09,
            VK_CLEAR => 0x0C,
            VK_RETURN => 0x0D,
            VK_SHIFT => 0x10,
            VK_CONTROL => 0x11,
            VK_MENU => 0x12,
            VK_PAUSE => 0x13,
            VK_CAPITAL => 0x14,
            VK_KANA => 0x15,
            VK_HANGUEL => 0x15,
            VK_HANGUL => 0x15,
            VK_IME_ON => 0x16,
            VK_JUNJA => 0x17,
            VK_FINAL => 0x18,
            VK_HANJA => 0x19,
            VK_KANJI => 0x19,
            VK_IME_OFF => 0x1A,
            VK_ESCAPE => 0x1B,
            VK_CONVERT => 0x1C,
            VK_NONCONVERT => 0x1D,
            VK_ACCEPT => 0x1E,
            VK_MODECHANGE => 0x1F,
            VK_SPACE => 0x20,
            VK_PRIOR => 0x21,
            VK_NEXT => 0x22,
            VK_END => 0x23,
            VK_HOME => 0x24,
            VK_LEFT => 0x25,
            VK_UP => 0x26,
            VK_RIGHT => 0x27,
            VK_DOWN => 0x28,
            VK_SELECT => 0x29,
            VK_PRINT => 0x2A,
            VK_EXECUTE => 0x2B,
            VK_SNAPSHOT => 0x2C,
            VK_INSERT => 0x2D,
            VK_DELETE => 0x2E,
            VK_HELP => 0x2F,
            VK_0 => 0x30,
            VK_1 => 0x31,
            VK_2 => 0x32,
            VK_3 => 0x33,
            VK_4 => 0x34,
            VK_5 => 0x35,
            VK_6 => 0x36,
            VK_7 => 0x37,
            VK_8 => 0x38,
            VK_9 => 0x39,
            VK_A => 0x41,
            VK_B => 0x42,
            VK_C => 0x43,
            VK_D => 0x44,
            VK_E => 0x45,
            VK_F => 0x46,
            VK_G => 0x47,
            VK_H => 0x48,
            VK_I => 0x49,
            VK_J => 0x4A,
            VK_K => 0x4B,
            VK_L => 0x4C,
            VK_M => 0x4D,
            VK_N => 0x4E,
            VK_O => 0x4F,
            VK_P => 0x50,
            VK_Q => 0x51,
            VK_R => 0x52,
            VK_S => 0x53,
            VK_T => 0x54,
            VK_U => 0x55,
            VK_V => 0x56,
            VK_W => 0x57,
            VK_X => 0x58,
            VK_Y => 0x59,
            VK_Z => 0x5A,
            VK_LWIN => 0x5B,
            VK_RWIN => 0x5C,
            VK_APPS => 0x5D,
            VK_SLEEP => 0x5F,
            VK_NUMPAD0 => 0x60,
            VK_NUMPAD1 => 0x61,
            VK_NUMPAD2 => 0x62,
            VK_NUMPAD3 => 0x63,
            VK_NUMPAD4 => 0x64,
            VK_NUMPAD5 => 0x65,
            VK_NUMPAD6 => 0x66,
            VK_NUMPAD7 => 0x67,
            VK_NUMPAD8 => 0x68,
            VK_NUMPAD9 => 0x69,
            VK_MULTIPLY => 0x6A,
            VK_ADD => 0x6B,
            VK_SEPARATOR => 0x6C,
            VK_SUBTRACT => 0x6D,
            VK_DECIMAL => 0x6E,
            VK_DIVIDE => 0x6F,
            VK_F1 => 0x70,
            VK_F2 => 0x71,
            VK_F3 => 0x72,
            VK_F4 => 0x73,
            VK_F5 => 0x74,
            VK_F6 => 0x75,
            VK_F7 => 0x76,
            VK_F8 => 0x77,
            VK_F9 => 0x78,
            VK_F10 => 0x79,
            VK_F11 => 0x7A,
            VK_F12 => 0x7B,
            VK_F13 => 0x7C,
            VK_F14 => 0x7D,
            VK_F15 => 0x7E,
            VK_F16 => 0x7F,
            VK_F17 => 0x80,
            VK_F18 => 0x81,
            VK_F19 => 0x82,
            VK_F20 => 0x83,
            VK_F21 => 0x84,
            VK_F22 => 0x85,
            VK_F23 => 0x86,
            VK_F24 => 0x87,
            VK_NUMLOCK => 0x90,
            VK_SCROLL => 0x91,
            VK_LSHIFT => 0xA0,
            VK_RSHIFT => 0xA1,
            VK_LCONTROL => 0xA2,
            VK_RCONTROL => 0xA3,
            VK_LMENU => 0xA4,
            VK_RMENU => 0xA5,
            VK_BROWSER_BACK => 0xA6,
            VK_BROWSER_FORWARD => 0xA7,
            VK_BROWSER_REFRESH => 0xA8,
            VK_BROWSER_STOP => 0xA9,
            VK_BROWSER_SEARCH => 0xAA,
            VK_BROWSER_FAVORITES => 0xAB,
            VK_BROWSER_HOME => 0xAC,
            VK_VOLUME_MUTE => 0xAD,
            VK_VOLUME_DOWN => 0xAE,
            VK_VOLUME_UP => 0xAF,
            VK_MEDIA_NEXT_TRACK => 0xB0,
            VK_MEDIA_PREV_TRACK => 0xB1,
            VK_MEDIA_STOP => 0xB2,
            VK_MEDIA_PLAY_PAUSE => 0xB3,
            VK_LAUNCH_MAIL => 0xB4,
            VK_LAUNCH_MEDIA_SELECT => 0xB5,
            VK_LAUNCH_APP1 => 0xB6,
            VK_LAUNCH_APP2 => 0xB7,
            VK_OEM_1 => 0xBA,
            VK_OEM_PLUS => 0xBB,
            VK_OEM_COMMA => 0xBC,
            VK_OEM_MINUS => 0xBD,
            VK_OEM_PERIOD => 0xBE,
            VK_OEM_2 => 0xBF,
            VK_OEM_3 => 0xC0,
            VK_OEM_4 => 0xDB,
            VK_OEM_5 => 0xDC,
            VK_OEM_6 => 0xDD,
            VK_OEM_7 => 0xDE,
            VK_OEM_8 => 0xDF,
            VK_OEM_102 => 0xE2,
            VK_PROCESSKEY => 0xE5,
            VK_PACKET => 0xE7,
            VK_ATTN => 0xF6,
            VK_CRSEL => 0xF7,
            VK_EXSEL => 0xF8,
            VK_EREOF => 0xF9,
            VK_PLAY => 0xFA,
            VK_ZOOM => 0xFB,
            VK_PA1 => 0xFD,
            VK_OEM_CLEAR => 0xFE,
        }
    }
}