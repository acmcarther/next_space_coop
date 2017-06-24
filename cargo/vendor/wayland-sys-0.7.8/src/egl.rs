//! Bindings to the EGL library `libwayland-egl.so`
//!
//! This lib allows to create EGL surfaces out of wayland surfaces.
//!
//! The created handle is named `WAYLAND_EGl_HANDLE`.

use std::os::raw::c_int;
use client::wl_proxy;

pub enum wl_egl_window { }

external_library!(WaylandEgl, "wayland-egl",
    functions:
        fn wl_egl_window_create(*mut wl_proxy, c_int, c_int) -> *mut wl_egl_window,
        fn wl_egl_window_destroy(*mut wl_egl_window) -> (),
        fn wl_egl_window_resize(*mut wl_egl_window, c_int, c_int, c_int, c_int) -> (),
        fn wl_egl_window_get_attached_size(*mut wl_egl_window, *mut c_int, *mut c_int) -> (),
);

#[cfg(feature = "dlopen")]
lazy_static!(
    pub static ref WAYLAND_EGL_OPTION: Option<WaylandEgl> = { 
        WaylandEgl::open("libwayland-egl.so").ok()
    };
    pub static ref WAYLAND_EGL_HANDLE: &'static WaylandEgl = {
        WAYLAND_EGL_OPTION.as_ref().expect("Library libwayland-egl.so could not be loaded.")
    };
);

#[cfg(not(feature = "dlopen"))]
pub fn is_lib_available() -> bool { true }
#[cfg(feature = "dlopen")]
pub fn is_lib_available() -> bool { WAYLAND_EGL_OPTION.is_some() }
