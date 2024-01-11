#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::mem::{size_of, size_of_val};

use beryllium::{
    *,
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile, GlWindow},
};
use beryllium::video::GlSwapInterval;
use ogl33::{GL_ARRAY_BUFFER, GL_COMPILE_STATUS, GL_FALSE, GL_FLOAT, GL_FRAGMENT_SHADER, GL_STATIC_DRAW, GL_VERTEX_SHADER, glAttachShader, glBindBuffer, glBindVertexArray, glBufferData, glClearColor, glCompileShader, glCreateProgram, glCreateShader, glDeleteBuffers, glDeleteShader, glEnableVertexAttribArray, glGenBuffers, glGenVertexArrays, glGetShaderInfoLog, glGetShaderiv, glLinkProgram, glShaderSource, GLuint, glVertexAttribPointer, load_gl_with};

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

    let win = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");
    win.set_swap_interval(GlSwapInterval::Vsync).expect("Unable to set VSYNC!");
    win
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
        glBindVertexArray(vao);
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

fn create_vertex_shader() -> GLuint {
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

        let mut success = 0;
        glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(
                vertex_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
        vertex_shader
    }
}

fn create_fragment_shader() -> GLuint {
    unsafe {
        let fragment_shader_script =
            &*fs::read_to_string("shaders/vert_shader.glsl")
                .expect("Missing shader!");
        let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        glShaderSource(
            fragment_shader,
            1,
            &(fragment_shader_script.as_bytes().as_ptr().cast()),
            &(fragment_shader_script.len().try_into().unwrap()),
        );
        glCompileShader(fragment_shader);
        let mut success = 0;
        glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(
                fragment_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }
        fragment_shader
    }
}

fn attach_shaders(vertex_shader: GLuint, fragment_shader: GLuint) {
    unsafe {
        let shader_program = glCreateProgram();
        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glLinkProgram(shader_program);
        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);
    }
}

fn main() {
    let sdl = setup_gl_context();
    let win = create_window(&sdl);

    load_open_gl(&win);
    generate_vertex_array_object();
    generate_vertex_buffer_object();
    send_data();

    let vertex_shader = create_vertex_shader();
    let fragment_shader = create_fragment_shader();

    attach_shaders(vertex_shader, fragment_shader);

    'main_loop: loop {
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => ()
            }
        }
    }
}
