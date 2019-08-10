use crate::device::*;
use crate::{Handle, Instance, ProfilePath, Result, Session, SourcePath, Subpath};
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
    values: HashMap<SourcePath, ActionState>,

    // map profiles to source paths
    sources: HashMap<ProfilePath, Vec<SourcePath>>,
}

pub struct ActionStateBoolean {}

pub struct ActionStateFloat {}

impl ActionStateFloat {
    fn new() -> Self {
        ActionStateFloat {}
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
        &self,
        interaction_profile: ProfilePath,
        bindings: &[ActionSuggestedBinding],
    ) {
        if interaction_profile != self.desktop_profile {
            // ignore, not supported
            return;
        }

        for binding in bindings {
            let mut action = binding.action;
            let sources = action
                .sources
                .entry(interaction_profile)
                .or_insert(Vec::new());
            sources.push(binding.binding);
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
        let mut msg = mem::uninitialized();
        while PeekMessageW(&mut msg, self.hwnd, WM_INPUT, WM_INPUT, PM_REMOVE) != 0 {
            let input = map_raw_input(msg.lParam as _);
            match input.header.dwType {
                RIM_TYPEMOUSE => {
                    if let Some(Device {
                        state:
                            DeviceState::Mouse {
                                ref mut delta_x,
                                ref mut delta_y,
                            },
                    }) = self.devices.get_mut(&DeviceTy::Mouse)
                    {
                        let mouse = input.data.mouse();
                        *delta_x += mouse.lLastX;
                        *delta_y += mouse.lLastY;
                    }
                }
                RIM_TYPEKEYBOARD => {
                    if let Some(device) = self.devices.get_mut(&DeviceTy::Keyboard) {
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
        Ok(ActionStateFloat {})
    }

    pub unsafe fn get_action_state_boolean(
        &self,
        action: Handle<Action>,
        subpath: Subpath,
    ) -> Result<ActionStateBoolean> {
        Ok(ActionStateBoolean {})
    }
}
