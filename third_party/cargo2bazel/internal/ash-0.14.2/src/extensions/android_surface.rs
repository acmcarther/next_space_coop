#![allow(dead_code)]
use prelude::*;
use std::mem;
use vk;
use std::ffi::CStr;
use ::RawPtr;
use version::{EntryV1_0, InstanceV1_0};

#[derive(Clone)]
pub struct AndroidSurface {
    handle: vk::Instance,
    android_surface_fn: vk::AndroidSurfaceFn,
}

impl AndroidSurface {
    pub fn new<E: EntryV1_0, I: InstanceV1_0>(entry: &E,
                                              instance: &I)
                                              -> Result<AndroidSurface, Vec<&'static str>> {
        let surface_fn = vk::AndroidSurfaceFn::load(|name| {
            unsafe {
                mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
            }
        })?;
        Ok(AndroidSurface {
            handle: instance.handle(),
            android_surface_fn: surface_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_android_surface\0").expect("Wrong extension string")
    }

    pub unsafe fn create_android_surface_khr(&self,
                                           create_info: &vk::AndroidSurfaceCreateInfoKHR,
                                           allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                           -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::uninitialized();
        let err_code = self.android_surface_fn
            .create_android_surface_khr(self.handle,
                                        create_info,
                                        allocation_callbacks.as_raw_ptr(),
                                        &mut surface);
        match err_code {
            vk::Result::Success => Ok(surface),
            _ => Err(err_code),
        }
    }
}
