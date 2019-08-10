mod action;
mod device;
mod error;
mod handle;
mod instance;
mod intern;
mod physical_device;

pub use crate::{action::*, error::*, instance::*, intern::*, physical_device::*};

use handle::Handle;
use intern::Path;
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

impl Instance {
    pub fn create_system(&self) -> Result<System> {
        Ok(System)
    }

    pub unsafe fn create_session(&self, system: &System) -> Result<Session> {
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

        let mut devices = HashMap::new();
        devices.insert(DeviceTy::Keyboard, Device::new_keyboard());
        devices.insert(DeviceTy::Mouse, Device::new_mouse());

        Ok(Session {
            hwnd,
            devices,
            active_profile: self.desktop_profile,
        })
    }
}

pub enum FormFactor {
    Desktop,
}

pub enum Event {}

struct Profiles {
    norse: ProfilePath,
}

pub struct System;

pub struct Session {
    hwnd: HWND,
    devices: HashMap<DeviceTy, Device>,
    active_profile: ProfilePath,
}
