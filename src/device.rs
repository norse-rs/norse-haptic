use crate::intern::Interner;
use crate::InputPath;
use std::collections::HashMap;

pub(crate) enum ActionValue {
    Float(f32),
    Boolean(bool),
}

pub(crate) type Mapping = fn(&DeviceState) -> ActionValue;
pub(crate) type MapSet = HashMap<InputPath, Mapping>;

fn build_mappings(intern: &mut Interner, mappings: &[(&str, Mapping)]) -> MapSet {
    mappings
        .into_iter()
        .map(|(name, mapping)| (intern.intern(name), *mapping))
        .collect()
}

pub(crate) struct Device {
    pub mappings: MapSet,
    pub state: DeviceState,
}

impl Device {
    pub fn new_mouse(intern: &mut Interner) -> Self {
        let mappings = build_mappings(
            intern,
            &[
                ("left/click", |_| {
                    ActionValue::Boolean(true) // TODO
                }),
                ("delta_x/scalar", |state| {
                    ActionValue::Float(state.get_mouse().delta_x as _)
                }),
                ("delta_y/scalar", |state| {
                    ActionValue::Float(state.get_mouse().delta_y as _)
                }),
            ],
        );
        Device {
            mappings,
            state: DeviceState::Mouse(Mouse {
                delta_x: 0,
                delta_y: 0,
            }),
        }
    }

    pub fn new_keyboard() -> Self {
        Device {
            mappings: HashMap::new(),
            state: DeviceState::Keyboard,
        }
    }

    pub fn reset(&mut self) {
        match self.state {
            DeviceState::Mouse(ref mut mouse) => {
                mouse.delta_x = 0;
                mouse.delta_y = 0;
            }

            _ => (), // TODO
        }
    }
}

pub(crate) struct Mouse {
    pub delta_x: i32,
    pub delta_y: i32,
}

pub(crate) enum DeviceState {
    Mouse(Mouse),
    Keyboard,
}

impl DeviceState {
    fn get_mouse(&self) -> &Mouse {
        match *self {
            DeviceState::Mouse(ref mouse) => mouse,
            _ => unreachable!(),
        }
    }
}
