extern crate gl;
extern crate glutin;

use glutin::dpi::*;
use glutin::GlContext;

mod render_gl;

fn main() {
  let mut events_loop = glutin::EventsLoop::new();
  let window = glutin::WindowBuilder::new()
      .with_title("Hello, world!")
      .with_dimensions(LogicalSize::new(900.0, 700.0));
  let context = glutin::ContextBuilder::new()
      .with_vsync(true);

  let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

  unsafe {
    gl_window.make_current().unwrap();
  }

  let gl = gl::Gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
  unsafe {
    gl.Viewport(0, 0, 900, 700); // set viewport
    gl.ClearColor(0.3, 0.3, 0.5, 1.0);
  }

  let mut running = true;

  use std::ffi::CString;

  let vert_shader = render_gl::Shader::from_vert_source(
    &gl,
    &CString::new(include_str!("triangle.vert")).unwrap(),
  ).unwrap();

  let frag_shader = render_gl::Shader::from_frag_source(
    &gl,
    &CString::new(include_str!("triangle.frag")).unwrap(),
  ).unwrap();

  let shader_program = render_gl::Program::from_shaders(
    &gl,
    &[vert_shader, frag_shader],
  ).unwrap();
  shader_program.set_used();

  let vertices: Vec<f32> = vec![
    -0.5, -0.5, 0.0,
    0.5, -0.5, 0.0,
    0.0, 0.5, 0.0
  ];

  let mut vbo: gl::types::GLuint = 0;
  unsafe {
    gl.GenBuffers(1, &mut vbo);
  }

  unsafe {
    gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl.BufferData(
      gl::ARRAY_BUFFER, // target
      (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
      vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
      gl::STATIC_DRAW, // usage
    );
    gl.BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
  }

  let mut vao: gl::types::GLuint = 0;
  unsafe {
    gl.GenVertexArrays(1, &mut vao);
  }

  unsafe {
    gl.BindVertexArray(vao);
    gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    gl.VertexAttribPointer(
      0, // index of the generic vertex attribute ("layout (location = 0)")
      3, // the number of components per generic vertex attribute
      gl::FLOAT, // data type
      gl::FALSE, // normalized (int-to-float conversion)
      (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
      std::ptr::null(), // offset of the first component
    );
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    gl.BindVertexArray(0);
  }


  while running {
    events_loop.poll_events(|event| {
      match event {
        glutin::Event::WindowEvent { event, .. } => match event {
          glutin::WindowEvent::CloseRequested => running = false,
          glutin::WindowEvent::Resized(logical_size) => {
            let dpi_factor = gl_window.get_hidpi_factor();
            gl_window.resize(logical_size.to_physical(dpi_factor));
          }
          _ => ()
        },
        _ => ()
      }
    });

    unsafe {
      gl.Clear(gl::COLOR_BUFFER_BIT);
    }

    shader_program.set_used();
    unsafe {
      gl.BindVertexArray(vao);
      gl.DrawArrays(
        gl::TRIANGLES, // mode
        0, // starting index in the enabled arrays
        3, // number of indices to be rendered
      );
    }
    gl_window.swap_buffers().unwrap();
  }
}
