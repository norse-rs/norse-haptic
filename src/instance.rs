
use crate::{NORSE_DESKTOP_INTERACTION_PROFILE_NAME, error::Result, intern::{Interner, Path}, physical_device::PhysicalDevice};
use std::mem;
use std::ffi::OsStr;
use winapi::um::winuser::{RegisterClassExW, DefWindowProcW, WNDCLASSEXW};
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::*;

pub struct Instance {
    pub(crate) class_name: Vec<u16>,
    pub(crate) interner: Interner,
    pub(crate) desktop_profile: Path,
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
        let desktop_profile = interner.intern(NORSE_DESKTOP_INTERACTION_PROFILE_NAME);

        Ok(Instance {
            class_name,
            interner,
            desktop_profile,
        })
    }

    pub fn string_to_path(&mut self, string: &str) -> Path {
        self.interner.intern(string)
    }
}
