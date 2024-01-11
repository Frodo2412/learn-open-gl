#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile},
    *,
};
use beryllium::video::GlWindow;
use ogl33::{glClearColor, glGenVertexArrays, GLuint, load_gl_with};

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

fn load_open_gl(win: &GlWindow) {
    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name.cast()));
        glClearColor(0.2, 0.3, 0.3, 1.0);
    }
}

fn create_triangle_data() {
    unsafe {
        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
    }
}

fn main() {
    let sdl = setup_gl_context();
    let win = create_window(&sdl);

    load_open_gl(&win);


    'main_loop: loop {
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => ()
            }
        }
    }
}
