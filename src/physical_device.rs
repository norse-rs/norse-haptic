use crate::{error::Result, instance::Instance};
use std::{mem, ptr};
use winapi::um::winuser::*;

pub type PhysicalDevice = RAWINPUTDEVICELIST;

pub struct PhysicalDeviceGroupProperties {}

impl Instance {
    pub unsafe fn enumerate_physical_devices(&self) -> Result<Vec<PhysicalDevice>> {
        let mut num_devices = 0;
        GetRawInputDeviceList(
            ptr::null_mut(),
            &mut num_devices,
            mem::size_of::<RAWINPUTDEVICELIST>() as _,
        );
        dbg!(num_devices);

        let mut devices = Vec::with_capacity(num_devices as _);
        GetRawInputDeviceList(
            devices.as_mut_ptr(),
            &mut num_devices,
            mem::size_of::<RAWINPUTDEVICELIST>() as _,
        );
        devices.set_len(num_devices as _);
        Ok(devices)
    }

    pub unsafe fn enumerate_physical_device_groups(
        &self,
    ) -> Result<Vec<PhysicalDeviceGroupProperties>> {
        unimplemented!()
    }
}
