use types::Vector2;
use std::collections::HashMap;

// ===== Mouse =====

bitflags! {
    pub struct MouseButton: u8 {
        const NONE  = 0b0000;
        const LEFT  = 0b0001;
        const MID   = 0b0010;
        const RIGHT = 0b0100;
    }
}

#[derive(Clone)]
pub struct MouseState {
    pub position: Vector2,
    pub buttons:  u8,
}

impl MouseState {
    pub fn new() -> Self {
        MouseState {
            position: Vector2::zero(),
            buttons:  0,
        }
    }
}

// ===== Gamepad =====

bitflags! {
    pub struct GamepadButton: u32 {
        const NONE  = 0;
        const START = 1;
        const BACK  = 1 << 1;
        const A     = 1 << 2;
        const B     = 1 << 3;
        const X     = 1 << 4;
        const Y     = 1 << 5;
        const LS    = 1 << 6;
        const RS    = 1 << 7;
        const D_UP  = 1 << 8;
        const D_DWN = 1 << 9;
        const D_LFT = 1 << 10;
        const D_RGT = 1 << 11;
        const LB    = 1 << 12;
        const LT    = 1 << 13;
        const RB    = 1 << 14;
        const RT    = 1 << 15;
    }
}

bitflags! {
    pub struct GamepadAxisSim: u16 {
        const NONE  = 0;
        const L_UP  = 1;
        const L_DWN = 1 << 1;
        const L_LFT = 1 << 2;
        const L_RGT = 1 << 3;
        const R_UP  = 1 << 4;
        const R_DWN = 1 << 5;
        const R_LFT = 1 << 6;
        const R_RGT = 1 << 7;
    }
}

#[derive(Clone)]
pub struct GamepadState {
    pub lstick:   Vector2,
    pub rstick:   Vector2,
    pub buttons:  u32,

    mappings: HashMap<&'static str, GamepadButton>,
}

impl GamepadState {
    pub fn new() -> Self {
        GamepadState {
            lstick:   Vector2::zero(),
            rstick:   Vector2::zero(),
            buttons:  0,
            mappings: HashMap::new(),
        }
    }

    // Setters
    pub fn set_button(&mut self, button: GamepadButton, state: bool) {
        if state {
            self.buttons = self.buttons | button.bits;
        } else {
            self.buttons = self.buttons & !button.bits;
        }
    }

    pub fn set_button_mapped(&mut self, key: &str, state: bool) {
        let button = 
            match self.mappings.get(key) {
                Some(button) => *button,
                None => GamepadButton::NONE,
            };
        self.set_button(button, state);
    }

    pub fn set_stick_sim(&mut self, state: GamepadAxisSim) {
        unimplemented!();
    }


    // Mappings
    pub fn map_button(&mut self, key: &'static str, button: GamepadButton) {
        if button != GamepadButton::NONE {
            self.mappings.insert(key, button);
        }
    }

    // Getters
    pub fn left_stick(&self) -> Vector2 { self.lstick }
    pub fn right_stick(&self) -> Vector2 { self.rstick }
    pub fn button_pressed(&self, button: GamepadButton) -> bool {
        self.buttons & button.bits != 0
    }
}


// ===== General =====

#[derive(Clone)]
pub struct InputState {
    pub mouse:   MouseState,
    pub gamepad: GamepadState,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            mouse:   MouseState::new(),
            gamepad: GamepadState::new(),
        }
    }
}
