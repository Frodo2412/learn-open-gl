#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use beryllium::{
    *,
    events::Event,
};
use ogl33::{GL_COLOR_BUFFER_BIT, GL_STATIC_DRAW, GL_TRIANGLES, glClear, glDrawArrays};

use crate::buffers::{Buffer, BufferType};
use crate::safe_gl::{buffer_data, create_window, load_open_gl, send_data, setup_gl_context};
use crate::shader_program::ShaderProgram;
use crate::vertex_array::VertexArray;

mod vertex_array;
mod buffers;
mod shaders;
mod shader_program;
mod safe_gl;

const WINDOW_TITLE: &str = "Hello Window";

type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

fn main() {
    let sdl = setup_gl_context();
    let win = create_window(&sdl);

    load_open_gl(&win);

    let vao = VertexArray::new().expect("Failure creating vao");
    vao.bind();

    let vbo = Buffer::new().expect("Failure creating vbo");
    vbo.bind(BufferType::Array);
    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
        GL_STATIC_DRAW,
    );

    send_data();

    let shader_program =
        ShaderProgram::from_vert_frag("shaders/vert_shader.glsl", "shaders/frag_shader.glsl");
    shader_program.unwrap().use_program();

    'main_loop: loop {
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => ()
            }
        }
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
        win.swap_window();
    }
}
