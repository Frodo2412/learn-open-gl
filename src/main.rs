use beryllium::{init, Sdl, video};

fn setup_gl_context() -> Sdl {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    #[cfg(target_os = "macos")]
    {
        sdl
            .set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
            .unwrap();
    }
    sdl
}

fn create_window(sdl: Sdl) {
    let win_args = video::CreateWinArgs {
        title: "LearnOpenGL",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let _win = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");
}

fn main() {
    let sdl = setup_gl_context();
    create_window(sdl);
}
