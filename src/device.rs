#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeviceTy {
    Keyboard,
    Mouse,
}

pub struct Device {
    pub state: DeviceState,
}

impl Device {
    pub fn new_mouse() -> Self {
        Device {
            state: DeviceState::Mouse {
                delta_x: 0,
                delta_y: 0,
            },
        }
    }

    pub fn new_keyboard() -> Self {
        Device {
            state: DeviceState::Keyboard,
        }
    }
}

pub enum DeviceState {
    Mouse { delta_x: i32, delta_y: i32 },
    Keyboard,
}
