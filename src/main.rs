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

    let vertices_b: [f32; 3 * 3] = [
        -0.75, -0.75, 0.0,
        0.75, -0.75, 0.0,
        0.0, 0.75, 0.0,
    ];
    let vbo = gl_wrapper::BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32_data(&vertices_b);

    let position_attribute = gl_wrapper::VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    position_attribute.enable();

    let target_fps: u128 = 60;
    let us_per_update: u128 = 1000000 / target_fps;
    let mut time_start = Instant::now();
    while !window.should_close() {
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
            std::thread::sleep(Duration::from_micros((us_per_update - elapsed_micros) as u64));
        } else {
            println!("Frame took too long: {} us, from {} us max", elapsed_micros, us_per_update);
        }
        time_start = Instant::now();
    }
}
