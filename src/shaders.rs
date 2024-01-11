use std::fs;
use ogl33::{GL_COMPILE_STATUS, GL_FRAGMENT_SHADER, GL_INFO_LOG_LENGTH, GL_TRUE, GL_VERTEX_SHADER, glCompileShader, glCreateShader, glDeleteShader, GLenum, glGetShaderInfoLog, glGetShaderiv, glShaderSource, GLuint};

/// The types of shader object.
pub enum ShaderType {
    /// Vertex shaders determine the position of geometry within the screen.
    Vertex = GL_VERTEX_SHADER as isize,
    /// Fragment shaders determine the color output of geometry.
    ///
    /// Also other values, but mostly color.
    Fragment = GL_FRAGMENT_SHADER as isize,
}

/// A handle to a [Shader
/// Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Shader_objects)
pub struct Shader(pub GLuint);

impl Shader {
    fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { glCreateShader(ty as GLenum) };
        if shader != 0 {
            Some(Self(shader))
        } else {
            None
        }
    }

    fn set_source(self, src: &str) {
        unsafe {
            glShaderSource(
                self.0,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap()),
            );
        }
    }


    fn compile(self) {
        unsafe {
            glCompileShader(self.0);
        }
    }

    pub(crate) fn delete(self) {
        unsafe { glDeleteShader(self.0) };
    }

    fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { glGetShaderiv(self.0, GL_INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            glGetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { glGetShaderiv(self.0, GL_COMPILE_STATUS, &mut compiled) };
        compiled == i32::from(GL_TRUE)
    }

    pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
        let script = &*fs::read_to_string(source)
            .expect(("Unable to read shader from ".to_owned() + source).as_str());
        let shader = Shader::new(ty).ok_or_else(|| "Couldn't allocate shader".to_string())?;
        shader.set_source(script);
        shader.compile();
        if shader.compile_success() {
            Ok(shader)
        } else {
            let out = shader.info_log();
            shader.delete();
            Err(out)
        }
    }
}

