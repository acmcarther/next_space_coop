use std::cmp::max;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::AsRawFd;

use byteorder::{WriteBytesExt, NativeEndian};

use tempfile::tempfile;

use wayland_client::{Proxy, EventQueueHandle, Init};
use wayland_client::protocol::{wl_surface, wl_shell, wl_compositor, wl_buffer, wl_subsurface,
                               wl_seat, wl_shm, wl_pointer, wl_shell_surface,
                               wl_subcompositor, wl_shm_pool, wl_output};

use super::themed_pointer::ThemedPointer;

// The surfaces handling the borders, 8 total, are organised this way:
//
//        0
// ---|-------|---
//    |       |
//  3 | user  | 1
//    |       |
// ---|-------|---
//        2
//
pub const BORDER_TOP         : usize = 0;
pub const BORDER_RIGHT       : usize = 1;
pub const BORDER_BOTTOM      : usize = 2;
pub const BORDER_LEFT        : usize = 3;

const DECORATION_SIZE     : i32 = 8;
const DECORATION_TOP_SIZE : i32 = 24;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum PtrLocation {
    None,
    Top,
    Right,
    Bottom,
    Left
}

enum Pointer {
    Plain(wl_pointer::WlPointer),
    Themed(ThemedPointer),
    None
}

struct PointerState {
    surfaces: Vec<wl_surface::WlSurface>,
    location: PtrLocation,
    coordinates: (f64, f64),
    cornered: bool,
    topped: bool,
    surface_width: i32,
    pointer: Pointer
}

impl PointerState {
    fn pointer_entered(&mut self, surface: &wl_surface::WlSurface, serial: u32) {
        if self.surfaces[BORDER_TOP].equals(surface) {
            self.location = PtrLocation::Top;
        } else if self.surfaces[BORDER_RIGHT].equals(surface) {
            self.location = PtrLocation::Right
        } else if self.surfaces[BORDER_BOTTOM].equals(surface) {
            self.location = PtrLocation::Bottom;
        } else if self.surfaces[BORDER_LEFT].equals(surface) {
            self.location = PtrLocation::Left
        } else {
            // A surface that we don't manage
            self.location = PtrLocation::None;
            return
        }
        self.update(Some(serial), true);
    }

    fn pointer_left(&mut self, serial: u32) {
        self.location = PtrLocation::None;
        self.change_pointer("left_ptr", Some(serial))
    }

    fn update(&mut self, serial: Option<u32>, force: bool) {
        let old_cornered = self.cornered;
        self.cornered = (self.location == PtrLocation::Top || self.location == PtrLocation::Bottom) &&
                        (self.coordinates.0 <= DECORATION_SIZE as f64 ||
                         self.coordinates.0 >= (self.surface_width + DECORATION_SIZE) as f64);
        let old_topped = self.topped;
        self.topped = self.location == PtrLocation::Top && self.coordinates.1 <= DECORATION_SIZE as f64;
        if force || (self.cornered ^ old_cornered) || (old_topped ^ self.topped) {
            let name = if self.cornered {
                match self.location {
                    PtrLocation::Top => if self.coordinates.0 <= DECORATION_SIZE as f64 {
                        "top_left_corner"
                    } else {
                        "top_right_corner"
                    },
                    PtrLocation::Bottom => if self.coordinates.0 <= DECORATION_SIZE as f64 {
                        "bottom_left_corner"
                    } else {
                        "bottom_right_corner"
                    },
                    _ => unreachable!()
                }
            } else {
                match self.location {
                    PtrLocation::Top => if self.topped { "top_side" } else { "left_ptr" },
                    PtrLocation::Bottom => "bottom_side",
                    PtrLocation::Right => "right_side",
                    PtrLocation::Left => "left_side",
                    _ => "left_ptr"
                }
            };
            self.change_pointer(name, serial)
        }
    }

    fn change_pointer(&self, name: &str, serial: Option<u32>) {
        if let Pointer::Themed(ref themed) = self.pointer {
            themed.set_cursor(name, serial);
        }
    }
}

/// A wrapper for a decorated surface.
///
/// This is the main object of this crate. It wraps a user provided
/// wayland surface into a `ShellSurface` and gives you acces to it
/// via the `.get_shell()` method.
///
/// It also handles the drawing of minimalistic borders allowing the
/// resizing and moving of the window. See the root documentation of
/// this crate for explanations about how to use it.
pub struct DecoratedSurface<H: Handler> {
    shell_surface: wl_shell_surface::WlShellSurface,
    border_subsurfaces: Vec<wl_subsurface::WlSubsurface>,
    buffers: Vec<wl_buffer::WlBuffer>,
    tempfile: File,
    pool: wl_shm_pool::WlShmPool,
    height: i32,
    width: i32,
    buffer_capacity: usize,
    pointer_state: PointerState,
    seat: Option<wl_seat::WlSeat>,
    handler: Option<H>,
    decorate: bool
}

impl<H: Handler> DecoratedSurface<H> {
    /// Resizes the borders to given width and height.
    ///
    /// These values should be the dimentions of the internal surface of the
    /// window (the decorated window will thus be a little larger).
    pub fn resize(&mut self, width: i32, height: i32) {
        // flush buffers
        for b in self.buffers.drain(..) {
            b.destroy();
        }

        self.width = width;
        self.height = height;

        // skip if not decorating
        if !self.decorate {
            for s in &self.pointer_state.surfaces {
                s.attach(None, 0, 0);
                s.commit();
            }
            return
        }

        // actually update the decorations
        let new_pxcount = max(DECORATION_TOP_SIZE * (DECORATION_SIZE * 2 + width),
            max(DECORATION_TOP_SIZE * width, DECORATION_SIZE * height)
        ) as usize;
        if new_pxcount * 4 > self.buffer_capacity {
            // reallocation needed !
            self.tempfile.set_len((new_pxcount * 4) as u64).unwrap();
            self.pool.resize((new_pxcount * 4) as i32);
            self.buffer_capacity = new_pxcount * 4;
        }
        self.pointer_state.surface_width = width;
        // rewrite the data
        self.tempfile.seek(SeekFrom::Start(0)).unwrap();
        for _ in 0..(new_pxcount*4) {
            // write a dark gray
            let _ = self.tempfile.write_u32::<NativeEndian>(0xFF444444);
        }
        self.tempfile.flush().unwrap();
        // resize the borders
        // top
        {
            let buffer = self.pool.create_buffer(
                0,
                self.width as i32 + (DECORATION_SIZE as i32) * 2,
                DECORATION_TOP_SIZE as i32,
                (self.width as i32 + (DECORATION_SIZE as i32) * 2) * 4,
                wl_shm::Format::Argb8888
            ).expect("Pool was destroyed!");
            self.pointer_state.surfaces[BORDER_TOP].attach(Some(&buffer), 0, 0);
            self.border_subsurfaces[BORDER_TOP].set_position(
                -(DECORATION_SIZE as i32),
                -(DECORATION_TOP_SIZE as i32)
            );
            self.buffers.push(buffer);
        }
        // right
        {
            let buffer = self.pool.create_buffer(
                0, DECORATION_SIZE as i32,
                self.height as i32, (DECORATION_SIZE*4) as i32,
                wl_shm::Format::Argb8888
            ).expect("Pool was destroyed!");
            self.pointer_state.surfaces[BORDER_RIGHT].attach(Some(&buffer), 0, 0);
            self.border_subsurfaces[BORDER_RIGHT].set_position(self.width as i32, 0);
            self.buffers.push(buffer);
        }
        // bottom
        {
            let buffer = self.pool.create_buffer(
                0,
                self.width as i32 + (DECORATION_SIZE as i32) * 2,
                DECORATION_SIZE as i32,
                (self.width as i32 + (DECORATION_SIZE as i32) * 2) * 4,
                wl_shm::Format::Argb8888
            ).expect("Pool was destroyed!");
            self.pointer_state.surfaces[BORDER_BOTTOM].attach(Some(&buffer), 0, 0);
            self.border_subsurfaces[BORDER_BOTTOM].set_position(-(DECORATION_SIZE as i32), self.height as i32);
            self.buffers.push(buffer);
        }
        // left
        {
            let buffer = self.pool.create_buffer(
                0, DECORATION_SIZE as i32,
                self.height as i32, (DECORATION_SIZE*4) as i32,
                wl_shm::Format::Argb8888
            ).expect("Pool was destroyed!");
            self.pointer_state.surfaces[BORDER_LEFT].attach(Some(&buffer), 0, 0);
            self.border_subsurfaces[BORDER_LEFT].set_position(-(DECORATION_SIZE as i32), 0);
            self.buffers.push(buffer);
        }

        for s in &self.pointer_state.surfaces { s.commit(); }
    }

    /// Creates a new decorated window around given surface.
    pub fn new(surface: &wl_surface::WlSurface, width: i32, height: i32,
               compositor: &wl_compositor::WlCompositor,
               subcompositor: &wl_subcompositor::WlSubcompositor,
               shm: &wl_shm::WlShm,
               shell: &wl_shell::WlShell,
               seat: Option<wl_seat::WlSeat>,
               decorate: bool)
        -> Result<DecoratedSurface<H>, ()>
    {
        // handle Shm
        let pxcount = max(DECORATION_TOP_SIZE * DECORATION_SIZE,
            max(DECORATION_TOP_SIZE * width, DECORATION_SIZE * height)
        ) as usize;

        let tempfile = match tempfile() {
            Ok(t) => t,
            Err(_) => return Err(())
        };

        match tempfile.set_len((pxcount *4) as u64) {
            Ok(()) => {},
            Err(_) => return Err(())
        };

        let pool = shm.create_pool(tempfile.as_raw_fd(), (pxcount * 4) as i32).expect("Shm cannot be destroyed");

        // create surfaces
        let border_surfaces: Vec<_> = (0..4).map(|_| compositor.create_surface()
                                                               .expect("Compositor cannot be destroyed")
                                            )
                                            .collect();
        let border_subsurfaces: Vec<_> = border_surfaces.iter()
                                                        .map(|s| subcompositor.get_subsurface(&s, surface)
                                                                              .expect("Subcompositor cannot be destroyed")
                                                        )
                                                        .collect();
        for s in &border_subsurfaces { s.set_desync(); }

        let shell_surface = shell.get_shell_surface(surface).expect("Shell cannot be destroyed");
        shell_surface.set_toplevel();

        // Pointer
        let pointer_state = {
            let surfaces = border_surfaces;
            let pointer = seat.as_ref().map(|seat| seat.get_pointer().expect("Seat cannot be dead!"));

            let pointer = match pointer.map(|pointer| ThemedPointer::load(pointer, None, &compositor, &shm)) {
                Some(Ok(themed)) => Pointer::Themed(themed),
                Some(Err(plain)) => Pointer::Plain(plain),
                None => Pointer::None
            };
            PointerState {
                surfaces: surfaces,
                location: PtrLocation::None,
                coordinates: (0., 0.),
                surface_width: width,
                cornered: false,
                topped: false,
                pointer: pointer
            }
        };

        let mut me = DecoratedSurface {
            shell_surface: shell_surface,
            border_subsurfaces: border_subsurfaces,
            buffers: Vec::new(),
            tempfile: tempfile,
            pool: pool,
            height: height,
            width: width,
            buffer_capacity: pxcount * 4,
            pointer_state: pointer_state,
            seat: seat,
            handler: None,
            decorate: decorate
        };

        me.resize(width, height);

        Ok(me)
    }

    /// Set a short title for the surface.
    ///
    /// This string may be used to identify the surface in a task bar, window list, or other user
    /// interface elements provided by the compositor.
    pub fn set_title(&self, title: String) {
        self.shell_surface.set_title(title);
    }

    /// Set a class for the surface.
    ///
    /// The surface class identifies the general class of applications to which the surface
    /// belongs. A common convention is to use the file name (or the full path if it is a
    /// non-standard location) of the application's .desktop file as the class.
    pub fn set_class(&self, class: String) {
        self.shell_surface.set_class(class);
    }

    /// Turn on or off decoration of this surface
    ///
    /// Automatically disables fullscreen mode if it was set.
    pub fn set_decorate(&mut self, decorate: bool) {
        self.shell_surface.set_toplevel();
        self.decorate = decorate;
        // trigger redraw
        let (w, h) = (self.width, self.height);
        self.resize(w, h);
    }

    /// Sets this surface as fullscreen (see `wl_shell_surface` for details)
    ///
    /// Automatically disables decorations.
    pub fn set_fullscreen(&mut self,
        method: wl_shell_surface::FullscreenMethod,
        framerate: u32,
        output: Option<&wl_output::WlOutput>) {
        self.shell_surface.set_fullscreen(method, framerate, output);
        self.decorate = false;
        // trigger redraw
        let (w, h) = (self.width, self.height);
        self.resize(w, h);
    }

    pub fn handler(&mut self) -> &mut Option<H> {
        &mut self.handler
    }
}

impl<H: Handler + ::std::any::Any + Send + 'static> Init for DecoratedSurface<H> {
    fn init(&mut self, evqh: &mut EventQueueHandle, my_index: usize) {
        evqh.register::<_, DecoratedSurface<H>>(&self.shell_surface, my_index);
        match self.pointer_state.pointer {
            Pointer::Plain(ref pointer) => evqh.register::<_, DecoratedSurface<H>>(pointer, my_index),
            Pointer::Themed(ref pointer) => evqh.register::<_, DecoratedSurface<H>>(&**pointer, my_index),
            Pointer::None => {}
        }
    }
}

impl<H: Handler> wl_pointer::Handler for DecoratedSurface<H> {
    fn enter(&mut self, _: &mut EventQueueHandle, _: &wl_pointer::WlPointer, serial: u32, surface: &wl_surface::WlSurface, x: f64, y: f64) {
        self.pointer_state.coordinates = (x, y);
        self.pointer_state.pointer_entered(surface, serial);
    }
    fn leave(&mut self, _: &mut EventQueueHandle, _: &wl_pointer::WlPointer, serial: u32, _: &wl_surface::WlSurface) {
        self.pointer_state.pointer_left(serial);
    }
    fn motion(&mut self, _: &mut EventQueueHandle, _: &wl_pointer::WlPointer, _: u32, x: f64, y: f64) {
        self.pointer_state.coordinates = (x, y);
        self.pointer_state.update(None, false);
    }
    fn button(&mut self, _: &mut EventQueueHandle, _: &wl_pointer::WlPointer, serial: u32, _: u32, button: u32, state: wl_pointer::ButtonState) {
        if button != 0x110 { return; }
        if let wl_pointer::ButtonState::Released = state { return; }
        let (x, y) = self.pointer_state.coordinates;
        let w = self.pointer_state.surface_width;
        let (direction, resize) = match self.pointer_state.location {
            PtrLocation::Top => {
                if y < DECORATION_SIZE as f64 {
                    if x < DECORATION_SIZE as f64 {
                        (wl_shell_surface::TopLeft, true)
                    } else if x > w as f64 + DECORATION_SIZE as f64 {
                        (wl_shell_surface::TopRight, true)
                    } else {
                        (wl_shell_surface::Top, true)
                    }
                } else {
                    if x < DECORATION_SIZE as f64 {
                        (wl_shell_surface::Left, true)
                    } else if x > w as f64 + DECORATION_SIZE as f64 {
                        (wl_shell_surface::Right, true)
                    } else {
                        (wl_shell_surface::None, false)
                    }
                }
            },
            PtrLocation::Bottom => {
                if x < DECORATION_SIZE as f64 {
                    (wl_shell_surface::BottomLeft, true)
                } else if x > w as f64 + DECORATION_SIZE as f64 {
                    (wl_shell_surface::BottomRight, true)
                } else {
                    (wl_shell_surface::Bottom, true)
                }
            },
            PtrLocation::Left => (wl_shell_surface::Left, true),
            PtrLocation::Right => (wl_shell_surface::Right, true),
            PtrLocation::None => {
                // pointer is not on a border, we must ignore the event
                return
            }
        };
        if let Some(ref seat) = self.seat {
            if resize {
                self.shell_surface.resize(&seat, serial, direction);
            } else {
                self.shell_surface._move(&seat, serial);
            }
        }
    }
}

unsafe impl<H: Handler> ::wayland_client::Handler<wl_pointer::WlPointer> for DecoratedSurface<H> {
    unsafe fn message(&mut self, evq: &mut EventQueueHandle, proxy: &wl_pointer::WlPointer, opcode: u32, args: *const ::wayland_client::sys::wl_argument) -> Result<(),()> {
        <DecoratedSurface<H> as ::wayland_client::protocol::wl_pointer::Handler>::__message(self, evq, proxy, opcode, args)
    }
}

pub trait Handler {
    fn configure(&mut self, evqh: &mut EventQueueHandle, edges: wl_shell_surface::Resize, width: i32, height: i32);
}

impl<H: Handler> wl_shell_surface::Handler for DecoratedSurface<H> {
    fn ping(&mut self, _: &mut EventQueueHandle, me: &wl_shell_surface::WlShellSurface, serial: u32) {
        me.pong(serial);
    }
    fn configure(&mut self, evqh: &mut EventQueueHandle, _: &wl_shell_surface::WlShellSurface, edges: wl_shell_surface::Resize, width: i32, height: i32) {
        if let Some(ref mut handler) = self.handler {
            let (w, h) = substract_borders(width, height);
            handler.configure(evqh, edges, w, h)
        }
    }
}

unsafe impl<H: Handler> ::wayland_client::Handler<wl_shell_surface::WlShellSurface> for DecoratedSurface<H> {
    unsafe fn message(&mut self, evq: &mut EventQueueHandle, proxy: &wl_shell_surface::WlShellSurface, opcode: u32, args: *const ::wayland_client::sys::wl_argument) -> Result<(),()> {
        <DecoratedSurface<H> as ::wayland_client::protocol::wl_shell_surface::Handler>::__message(self, evq, proxy, opcode, args)
    }
}

/// Substracts the border dimensions from the given dimensions.
pub fn substract_borders(width: i32, height: i32) -> (i32, i32) {
    (
        width - 2*(DECORATION_SIZE as i32),
        height - DECORATION_SIZE as i32 - DECORATION_TOP_SIZE as i32
    )
}

/// Adds the border dimensions to the given dimensions.
pub fn add_borders(width: i32, height: i32) -> (i32, i32) {
    (
        width + 2*(DECORATION_SIZE as i32),
        height + DECORATION_SIZE as i32 + DECORATION_TOP_SIZE as i32
    )
}
