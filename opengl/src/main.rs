extern crate gl;
extern crate glutin;

use gl::types::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::PossiblyCurrent;
use glutin::Context;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("OpenGL");
    let context = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let context = unsafe {
        context.make_current().unwrap()
    };

    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    // Vertices for two triangles forming a rectangle
    let vertices: [f32; 18] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0,
        
        0.0,  0.5, 0.0,
        0.5, -0.5, 0.0,
        0.5,  0.5, 0.0,
    ];

    let mut vbo: GLuint = 0;
    let mut vao: GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    loop {
        println!("Update!");
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        context.swap_buffers().unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = glutin::event_loop::ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    },
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
