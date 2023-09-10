pub mod logger;
pub mod graphics;

use gl::types::*;
use std::mem;
use std::ptr;
use std::time::{Instant, Duration};

use graphics::*;

fn main() {
    let mut window = window::Window::new(1080, 720, "OpenGL Windo-o-ow!");
    window.init_gl();

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0,
    ];

    let vao = gl_wrapper::Vao::new();
    vao.bind();

    let vbo = gl_wrapper::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32_data(&vertices);

    let position_attribute = gl_wrapper::VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    position_attribute.enable();

    let target_fps: i32 = 60;
    let us_per_update: u128 = 1000000 / 60;
    while !window.should_close() {
        let time_start = Instant::now();

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.update();

        let time_end = Instant::now();
        let elapsed = time_end - time_start;
        let elapsed_micros = elapsed.as_micros();
        if elapsed_micros < us_per_update {
            std::thread::sleep(Duration::from_micros((elapsed_micros - us_per_update) as u64));
        }
        println!("Update: {}us, max: {}us, free: {}us", elapsed_micros, us_per_update as i32, elapsed_micros as i32 - us_per_update as i32);
    }
}
