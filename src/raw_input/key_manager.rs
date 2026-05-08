use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY};
use windows::Win32::UI::Input::XboxController::{
    XInputGetState, XINPUT_STATE, XINPUT_GAMEPAD_A, XINPUT_GAMEPAD_B, XINPUT_GAMEPAD_X,
    XINPUT_GAMEPAD_Y, XINPUT_GAMEPAD_RIGHT_SHOULDER, XINPUT_GAMEPAD_LEFT_SHOULDER,
    XINPUT_GAMEPAD_DPAD_UP, XINPUT_GAMEPAD_DPAD_DOWN, XINPUT_GAMEPAD_DPAD_LEFT,
    XINPUT_GAMEPAD_DPAD_RIGHT, XINPUT_GAMEPAD_START, XINPUT_GAMEPAD_BACK,
    XINPUT_GAMEPAD_LEFT_THUMB, XINPUT_GAMEPAD_RIGHT_THUMB,
    XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE, XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE,
    XINPUT_GAMEPAD_TRIGGER_THRESHOLD
};

pub struct KeyboardManager {
    keys: [KeyState; 256],
    next_frame: [KeyState; 256],
}

impl KeyboardManager {
    pub fn new() -> Self {
        KeyboardManager {
            keys: [KeyState::Released; 256],
            next_frame: [KeyState::Released; 256],
        }
    }

    pub fn get_key_state(&mut self, key: VIRTUAL_KEY) -> KeyState {
        if key.0 > 256 {
            panic!("Virtual keys can't have an index of more than 256!")
        }

        let new_state = get_key_state(key);
        let old_state = self.keys[key.0 as usize];

        if old_state != new_state {
            self.next_frame[key.0 as usize] = new_state;
            new_state
        } else if new_state == KeyState::Pressed {
            KeyState::Down
        } else {
            KeyState::Up
        }
    }

    pub fn end_frame(&mut self) {
        self.keys = self.next_frame;
    }

    /// Check if the given key is either [KeyState::Pressed] or [KeyState::Down]
    pub fn has_pressed(&mut self, key: VIRTUAL_KEY) -> bool {
        let state = self.get_key_state(key);

        state == KeyState::Pressed || state == KeyState::Down
    }

    /// Returns `true` if all given `keys` are either [KeyState::Down] or [KeyState::Pressed], with at least *one* [
    pub fn all_pressed(&mut self, keys: impl Iterator<Item = VIRTUAL_KEY>) -> bool {
        let states = keys.map(|key| self.get_key_state(key)).collect::<Vec<_>>();

        states.iter().any(|key| *key == KeyState::Pressed)
            && states
                .iter()
                .all(|key| *key == KeyState::Pressed || *key == KeyState::Down)
    }

    /// Returns `true` if any of the keys are [KeyState::Released]
    pub fn any_released(&mut self, keys: impl Iterator<Item = VIRTUAL_KEY>) -> bool {
        let states = keys.map(|key| self.get_key_state(key)).collect::<Vec<_>>();

        states.iter().any(|key| *key == KeyState::Released)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum KeyState {
    Pressed,
    Down,
    Up,
    Released,
}

pub fn get_key_state(key: VIRTUAL_KEY) -> KeyState {
    if (0xC3..=0xDA).contains(&key.0) {
        if is_gamepad_key_pressed(key.0) {
            return KeyState::Pressed;
        } else {
            return KeyState::Released;
        }
    }

    unsafe {
        let value = GetAsyncKeyState(key.0 as i32) as u16;

        if value & 0x8000 != 0 {
            KeyState::Pressed
        } else {
            KeyState::Released
        }
    }
}

fn is_gamepad_key_pressed(key: u16) -> bool {
    let mut state = XINPUT_STATE::default();
    
    for i in 0..4 {
        unsafe {
            if XInputGetState(i, &mut state) == 0 {
                let gamepad = &state.Gamepad;
                let w_buttons = gamepad.wButtons.0;
                
                let is_pressed = match key {
                    0xC3 => (w_buttons & XINPUT_GAMEPAD_A.0) != 0,
                    0xC4 => (w_buttons & XINPUT_GAMEPAD_B.0) != 0,
                    0xC5 => (w_buttons & XINPUT_GAMEPAD_X.0) != 0,
                    0xC6 => (w_buttons & XINPUT_GAMEPAD_Y.0) != 0,
                    0xC7 => (w_buttons & XINPUT_GAMEPAD_RIGHT_SHOULDER.0) != 0,
                    0xC8 => (w_buttons & XINPUT_GAMEPAD_LEFT_SHOULDER.0) != 0,
                    0xC9 => gamepad.bLeftTrigger > XINPUT_GAMEPAD_TRIGGER_THRESHOLD.0 as u8,
                    0xCA => gamepad.bRightTrigger > XINPUT_GAMEPAD_TRIGGER_THRESHOLD.0 as u8,
                    0xCB => (w_buttons & XINPUT_GAMEPAD_DPAD_UP.0) != 0,
                    0xCC => (w_buttons & XINPUT_GAMEPAD_DPAD_DOWN.0) != 0,
                    0xCD => (w_buttons & XINPUT_GAMEPAD_DPAD_LEFT.0) != 0,
                    0xCE => (w_buttons & XINPUT_GAMEPAD_DPAD_RIGHT.0) != 0,
                    0xCF => (w_buttons & XINPUT_GAMEPAD_START.0) != 0,
                    0xD0 => (w_buttons & XINPUT_GAMEPAD_BACK.0) != 0,
                    0xD1 => (w_buttons & XINPUT_GAMEPAD_LEFT_THUMB.0) != 0,
                    0xD2 => (w_buttons & XINPUT_GAMEPAD_RIGHT_THUMB.0) != 0,
                    0xD3 => gamepad.sThumbLY > XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE.0 as i16,
                    0xD4 => gamepad.sThumbLY < -(XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE.0 as i16),
                    0xD5 => gamepad.sThumbLX > XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE.0 as i16,
                    0xD6 => gamepad.sThumbLX < -(XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE.0 as i16),
                    0xD7 => gamepad.sThumbRY > XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE.0 as i16,
                    0xD8 => gamepad.sThumbRY < -(XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE.0 as i16),
                    0xD9 => gamepad.sThumbRX > XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE.0 as i16,
                    0xDA => gamepad.sThumbRX < -(XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE.0 as i16),
                    _ => false,
                };
                if is_pressed {
                    return true;
                }
            }
        }
    }
    false
}
