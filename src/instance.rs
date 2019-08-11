use crate::{
    error::Result,
    intern::{Interner, Path},
    ProfilePath, NORSE_DESKTOP_INTERACTION_PROFILE_NAME,
};
use std::ffi::OsStr;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::*;
use winapi::um::winuser::{DefWindowProcW, RegisterClassExW, WNDCLASSEXW};

pub struct Instance {
    pub(crate) class_name: Vec<u16>,
    pub(crate) interner: Interner,
    pub(crate) profiles: Profiles,
}

impl Instance {
    pub unsafe fn create() -> Result<Self> {
        let mut class: WNDCLASSEXW = mem::zeroed();
        let class_name = OsStr::new("norse-input")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        class.cbSize = mem::size_of::<WNDCLASSEXW>() as UINT;
        class.lpszClassName = class_name.as_ptr();
        class.lpfnWndProc = Some(DefWindowProcW);

        RegisterClassExW(&class);

        let mut interner = Interner::new();
        let profiles = Profiles {
            desktop: interner.intern(NORSE_DESKTOP_INTERACTION_PROFILE_NAME),
        };

        Ok(Instance {
            class_name,
            interner,
            profiles,
        })
    }

    pub fn string_to_path(&mut self, string: &str) -> Path {
        self.interner.intern(string)
    }
}

pub(crate) struct Profiles {
    pub desktop: ProfilePath,
}
