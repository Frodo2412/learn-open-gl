use std::mem::size_of;

use beryllium::init::InitFlags;
use beryllium::Sdl;
use beryllium::video::{CreateWinArgs, GlContextFlags, GlProfile, GlSwapInterval, GlWindow};
use ogl33::{GL_FALSE, GL_FLOAT, glBufferData, glClearColor, glEnableVertexAttribArray, GLenum, glVertexAttribPointer, load_gl_with};

use crate::{Vertex, WINDOW_TITLE};
use crate::buffers::BufferType;

pub fn clear_color(red: f32, green: f32, blue: f32, opacity: f32) {
    unsafe { glClearColor(red, green, blue, opacity) }
}


/// Places a slice of data into a previously-bound buffer.
pub fn buffer_data(ty: BufferType, data: &[u8], usage: GLenum) {
    unsafe {
        glBufferData(
            ty as GLenum,
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
            usage,
        );
    }
}

pub fn setup_gl_context() -> Sdl {
    let sdl = Sdl::init(InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(GlProfile::Core).unwrap();
    let mut flags = GlContextFlags::default();
    if cfg!(target_os = "macos") {
        flags |= GlContextFlags::FORWARD_COMPATIBLE;
    }
    if cfg!(debug_asserts) {
        flags |= GlContextFlags::DEBUG;
    }
    sdl.set_gl_context_flags(flags).unwrap();
    sdl
}

pub fn create_window(sdl: &Sdl) -> GlWindow {
    let win_args = CreateWinArgs {
        title: WINDOW_TITLE,
        width: 800,
        height: 600,
        ..Default::default()
    };

    let win = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");
    win.set_swap_interval(GlSwapInterval::Vsync).expect("Unable to set VSYNC!");
    win
}

pub fn load_open_gl(win: &GlWindow) {
    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name.cast()));
    }
    clear_color(0.2, 0.3, 0.3, 1.0);
}

pub fn send_data() {
    unsafe {
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }
}
