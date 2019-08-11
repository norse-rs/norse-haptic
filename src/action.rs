use crate::device::*;
use crate::{
    Handle, InputPath, Instance, ProfilePath, Result, Session, SourcePath, Subpath, UserPath,
};
use std::collections::HashMap;
use std::mem;
use winapi::um::winuser::*;

pub struct ActionSet;

impl ActionSet {
    pub unsafe fn create_action(
        &self,
        name: &str,
        ty: ActionType,
        subpaths: &[Subpath],
    ) -> Result<Handle<Action>> {
        let mut values = HashMap::new();
        if subpaths.is_empty() {
            values.insert(Subpath::NULL, ActionState::new(ty));
        } else {
            for path in subpaths {
                values.insert(*path, ActionState::new(ty));
            }
        }

        Ok(Handle::new(Action {
            values,
            sources: HashMap::new(),
        }))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ActionType {
    BooleanInput,
    FloatInput,
    Vec2Input,
    VibrationOutput,
}

pub struct Action {
    values: HashMap<Subpath, ActionState>,

    // map profiles to source paths
    sources: HashMap<ProfilePath, Vec<(UserPath, InputPath)>>,
}

#[derive(Debug)]
pub struct ActionStateBoolean {
    pub current_state: bool,
}

impl ActionStateBoolean {
    fn new() -> Self {
        ActionStateBoolean {
            current_state: false,
        }
    }

    fn apply_value(&mut self, value: ActionValue) {
        match value {
            ActionValue::Boolean(v) => {
                // TODO
                self.current_state = v;
            }
            ActionValue::Float(v) => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct ActionStateFloat {
    pub current_state: f32,
}

impl ActionStateFloat {
    fn new() -> Self {
        ActionStateFloat { current_state: 0.0 }
    }

    fn apply_value(&mut self, value: ActionValue) {
        match value {
            ActionValue::Boolean(v) => unimplemented!(),
            ActionValue::Float(v) => {
                // TODO: exact rules
                self.current_state = v;
            }
        }
    }
}

pub enum ActionState {
    Boolean { current_state: bool },
    Float(f32),
    Vec2 { x: f32, y: f32 },
}

impl ActionState {
    fn new(ty: ActionType) -> Self {
        match ty {
            ActionType::BooleanInput => ActionState::Boolean {
                current_state: false,
            },
            ActionType::FloatInput => ActionState::Float(0.0),
            ActionType::Vec2Input => ActionState::Vec2 { x: 0.0, y: 0.0 },
            ActionType::VibrationOutput => unimplemented!(),
        }
    }
}

pub struct ActionSuggestedBinding {
    pub action: Handle<Action>,
    pub binding: SourcePath,
}

impl Instance {
    pub unsafe fn create_action_set(&self, name: &str) -> Result<Handle<ActionSet>> {
        Ok(Handle::new(ActionSet))
    }

    pub unsafe fn suggest_interaction_profile_bindings(
        &mut self,
        interaction_profile: ProfilePath,
        bindings: &[ActionSuggestedBinding],
    ) {
        if interaction_profile != self.profiles.desktop {
            // ignore, not supported
            return;
        }

        for binding in bindings {
            let mut action = binding.action;
            let sources = action
                .sources
                .entry(interaction_profile)
                .or_insert(Vec::new());
            let (user, input) = {
                let path_str = self.interner.untern(binding.binding);
                let mut split = path_str.split("/input/");
                let user = split.next().unwrap();
                let input = split.next().unwrap();

                (self.interner.intern(user), self.interner.intern(input))
            };
            sources.push((user, input));
        }
    }
}

fn map_raw_input(handle: HRAWINPUT) -> RAWINPUT {
    let mut input: RAWINPUT = unsafe { mem::zeroed() };
    let mut size = mem::size_of::<RAWINPUT>() as _;
    let header_size = mem::size_of::<RAWINPUTHEADER>() as _;
    unsafe {
        GetRawInputData(
            handle,
            RID_INPUT,
            &mut input as *mut _ as *mut _,
            &mut size,
            header_size,
        )
    };
    input
}

impl Session {
    pub unsafe fn attach_action_sets(&mut self, sets: &[Handle<ActionSet>]) {}

    pub unsafe fn sync_actions(&mut self, sets: &[Handle<ActionSet>]) {
        // reset devices
        for device in self.devices.values_mut() {
            device.reset();
        }

        // apply updates
        let mut msg = mem::uninitialized();
        while PeekMessageW(&mut msg, self.hwnd, WM_INPUT, WM_INPUT, PM_REMOVE) != 0 {
            let input = map_raw_input(msg.lParam as _);
            match input.header.dwType {
                RIM_TYPEMOUSE => {
                    if let Some(Device {
                        state: DeviceState::Mouse(ref mut mouse),
                        ..
                    }) = self.devices.get_mut(&self.user_paths.mouse)
                    {
                        let raw_mouse = input.data.mouse();
                        mouse.delta_x += raw_mouse.lLastX;
                        mouse.delta_y += raw_mouse.lLastY;
                    }
                }
                RIM_TYPEKEYBOARD => {
                    if let Some(device) = self.devices.get_mut(&self.user_paths.keyboard) {
                        let keyboard = input.data.keyboard();
                        // TODO
                    }
                }
                _ => (),
            }
        }
    }

    pub unsafe fn get_action_state_float(
        &self,
        action: Handle<Action>,
        subpath: Subpath,
    ) -> Result<ActionStateFloat> {
        let mut state = ActionStateFloat::new();
        let sources = &action.sources[&self.active_profile];
        for &(user, input) in sources {
            if let Some(device) = self.devices.get(&user) {
                if let Some(mapping) = device.mappings.get(&input) {
                    state.apply_value(mapping(&device.state));
                }
            }
        }
        Ok(state)
    }

    pub unsafe fn get_action_state_boolean(
        &self,
        action: Handle<Action>,
        subpath: Subpath,
    ) -> Result<ActionStateBoolean> {
        let mut state = ActionStateBoolean::new();
        let sources = &action.sources[&self.active_profile];
        for &(user, input) in sources {
            if let Some(device) = self.devices.get(&user) {
                if let Some(mapping) = device.mappings.get(&input) {
                    state.apply_value(mapping(&device.state));
                }
            }
        }
        Ok(state)
    }
}
