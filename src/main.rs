#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile},
    *,
};
use beryllium::video::GlWindow;

const WINDOW_TITLE: &str = "Hello Window";

fn setup_gl_context() -> Sdl {
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

fn create_window(sdl: &Sdl) -> GlWindow {
    let win_args = CreateWinArgs {
        title: WINDOW_TITLE,
        width: 800,
        height: 600,
        ..Default::default()
    };

    sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context")
}

fn main() {
    let sdl = setup_gl_context();
    let _win = create_window(&sdl);

    'main_loop: loop {
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => ()
            }
        }
    }
}
