#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::mem::{size_of, size_of_val};
use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile},
    *,
};
use beryllium::video::GlWindow;
use ogl33::{GL_ARRAY_BUFFER, GL_ARRAY_BUFFER_BINDING, GL_FALSE, GL_FLOAT, GL_STATIC_DRAW, GL_VERTEX_SHADER, glBindBuffer, glBufferData, glClearColor, glCompileShader, glCreateShader, glEnableVertexAttribArray, glGenBuffers, glGenVertexArrays, glShaderSource, GLuint, glVertexAttribPointer, load_gl_with};

const WINDOW_TITLE: &str = "Hello Window";

type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

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

fn generate_vertex_array_object() {
    unsafe {
        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
    }
}

fn generate_vertex_buffer_object() {
    unsafe {
        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
    }
}


fn send_data() {
    unsafe {
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
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

fn create_vertex_shader() {
    let vert_shader_script: &str =
        &*fs::read_to_string("shaders/vert_shader.glsl")
            .expect("Missing shader!");
    unsafe {
        let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        glShaderSource(
            vertex_shader,
            1,
            &(vert_shader_script.as_bytes().as_ptr().cast()),
            &(vert_shader_script.len().try_into().unwrap()),
        );
        glCompileShader(vertex_shader);
    }
}

fn main() {
    let sdl = setup_gl_context();
    let win = create_window(&sdl);

    load_open_gl(&win);
    generate_vertex_array_object();
    generate_vertex_buffer_object();
    send_data();
    create_vertex_shader();

    'main_loop: loop {
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => ()
            }
        }
    }
}
