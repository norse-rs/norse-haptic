mod action;
mod device;
mod error;
mod handle;
mod instance;
mod intern;
mod physical_device;

pub use crate::{action::*, error::*, instance::*, intern::*, physical_device::*};

use handle::Handle;
pub use intern::Path;
use std::collections::HashMap;

use crate::device::*;
use std::{mem, ptr};
use winapi::shared::hidusage::*;
use winapi::shared::windef::*;
use winapi::um::libloaderapi::*;
use winapi::um::winuser::*;

const NORSE_DESKTOP_INTERACTION_PROFILE_NAME: &'static str = "/interaction_profiles/norse/desktop";

type ProfilePath = Path;
type SourcePath = Path;
pub type Subpath = Path;

// e.g /user/hand/left
type UserPath = Path;

// e.g /input/mouse/left/click
type InputPath = Path;

impl Instance {
    pub fn create_system(&self) -> Result<System> {
        Ok(System)
    }

    pub unsafe fn create_session(&mut self, system: &System) -> Result<Session> {
        let instance = GetModuleHandleW(ptr::null());
        let hwnd = CreateWindowExW(
            0,
            self.class_name.as_ptr(),
            std::ptr::null(),
            0,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            instance,
            std::ptr::null_mut(),
        );

        let raw_devices = [
            RAWINPUTDEVICE {
                usUsagePage: HID_USAGE_PAGE_GENERIC,
                usUsage: HID_USAGE_GENERIC_KEYBOARD,
                dwFlags: RIDEV_DEVNOTIFY | RIDEV_INPUTSINK,
                hwndTarget: hwnd,
            },
            RAWINPUTDEVICE {
                usUsagePage: HID_USAGE_PAGE_GENERIC,
                usUsage: HID_USAGE_GENERIC_MOUSE,
                dwFlags: RIDEV_DEVNOTIFY | RIDEV_INPUTSINK,
                hwndTarget: hwnd,
            },
        ];
        RegisterRawInputDevices(
            raw_devices.as_ptr(),
            raw_devices.len() as _,
            mem::size_of::<RAWINPUTDEVICE>() as _,
        );

        let user_paths = UserPaths {
            mouse: self.interner.intern("/user/mouse"),
            keyboard: self.interner.intern("/user/keyboard"),
        };

        let mut devices = HashMap::new();
        devices.insert(user_paths.keyboard, Device::new_keyboard());
        devices.insert(user_paths.mouse, Device::new_mouse(&mut self.interner));

        Ok(Session {
            hwnd,
            user_paths,
            devices,
            active_profile: self.profiles.desktop,
        })
    }
}

pub enum FormFactor {
    Desktop,
}

pub enum Event {}

pub struct System;

pub(crate) struct UserPaths {
    pub mouse: UserPath,
    pub keyboard: UserPath,
}

pub struct Session {
    hwnd: HWND,
    user_paths: UserPaths,
    devices: HashMap<UserPath, Device>,
    active_profile: ProfilePath,
}
