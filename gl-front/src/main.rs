#![feature(tool_attributes)]
extern crate gl;
extern crate glutin;

#[macro_use]
extern crate game_derive;

#[macro_use]
extern crate failure;

mod render_gl;
mod resources;

use glutin::{dpi::*, GlContext};
use render_gl::data;
use resources::Resources;
use std::path::Path;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    clr: data::f32_f32_f32,
}

//impl Vertex {
//    fn vertex_attrib_pointers(gl: &gl::Gl) {
//        let stride = std::mem::size_of::<Self,>();
//        let location = 0;
//        let offset = 0;
//
//        unsafe {
//            data::f32_f32_f32::vertex_attrib_pointer(gl, stride, location, offset,);
//        }
//        let location = 1;
//        let offset = offset + std::mem::size_of::<data::f32_f32_f32,>();
//
//        unsafe {
//            data::f32_f32_f32::vertex_attrib_pointer(gl, stride, location, offset,);
//        }
//    }
//}

fn main() {
    let res = run();
    match res {
        Err(e,) => println!("{}", failure_to_string(e)),
        _ => {}
    }
}

#[rustfmt::skip]
fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets",),)?;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!",)
        .with_dimensions(LogicalSize::new(900.0, 700.0,),);
    let context = glutin::ContextBuilder::new().with_vsync(true,);

    let gl_window = glutin::GlWindow::new(window, context, &events_loop,).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    let gl = gl::Gl::load_with(|symbol| gl_window.get_proc_address(symbol,) as *const _,);
    unsafe {
        gl.Viewport(0, 0, 900, 700,); // set viewport
        gl.ClearColor(0.3, 0.3, 0.5, 1.0,);
    }

    let mut running = true;

    let shader_program = render_gl::Program::from_res(&gl, &res, "shaders/triangle",)?;
    shader_program.set_used();

    let vertices: Vec<Vertex> = vec![
        // positions      // colors
        Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
        Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() },
        Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },
    ];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo,);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo,);
        gl.BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0,); // unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao,);
    }

    unsafe {
        gl.BindVertexArray(vao,);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo,);
        Vertex::vertex_attrib_pointers(&gl);
        gl.BindBuffer(gl::ARRAY_BUFFER, 0,);
        gl.BindVertexArray(0,);
    }

    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                glutin::WindowEvent::Resized(logical_size,) => {
                    let dpi_factor = gl_window.get_hidpi_factor();
                    gl_window.resize(logical_size.to_physical(dpi_factor,),);
                }
                _ => (),
            },
            _ => (),
        },);

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT,);
        }

        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(vao,);
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }
        gl_window.swap_buffers().unwrap();
    }
    Ok((),)
}

pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    for (i, cause,) in e
        .iter_chain()
        .collect::<Vec<_,>>()
        .into_iter()
        .rev()
        .enumerate()
    {
        if i > 0 {
            let _ = writeln!(&mut result, "   Which caused the following issue:");
        }
        let _ = write!(&mut result, "{}", cause);
        if let Some(backtrace,) = cause.backtrace() {
            let backtrace_str = format!("{}", backtrace);
            if backtrace_str.len() > 0 {
                let _ = writeln!(&mut result, " This happened at {}", backtrace);
            } else {
                let _ = writeln!(&mut result);
            }
        } else {
            let _ = writeln!(&mut result);
        }
    }

    result
}
