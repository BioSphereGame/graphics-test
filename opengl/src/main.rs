#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const WINDOW_TITLE: &str = "Hello Window";
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

use beryllium::*;
use core::{
    convert::TryInto,
    mem::{size_of, size_of_val},
};
use ogl33::*;

type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] =[[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
const VERT_SHADER: &str = r#"#version 330 core
    layout (location = 0) in vec3 pos;

    void main() {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    }
"#;
const FRAG_SHADER: &str = r#"#version 330 core
    out vec4 final_color;

    void main() {
        final_color = vec4(1.0, 0.5, 0.2, 1.0);
    }
"#;

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    let win = sdl.create_gl_window(video::CreateWinArgs{
        title: WINDOW_TITLE,
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        allow_high_dpi: false,
        borderless: false,
        resizable: true,
    }).expect("Couldn't make a window and context");
    win.set_swap_interval(video::GlSwapInterval::Vsync).expect("Couldn't set swap interval");

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name as *const u8));
        glClearColor(0.2, 0.3, 0.3, 1.0);

        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
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

        let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        glShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );
        glCompileShader(vertex_shader);
        let mut success = 0;
        glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        glGetShaderInfoLog(
            vertex_shader,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());

        let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        glShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
        glCompileShader(fragment_shader);
        let mut success = 0;
        glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        glGetShaderInfoLog(
            fragment_shader,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());

        let shader_program = glCreateProgram();
        assert_ne!(shader_program, 0);
        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glLinkProgram(shader_program);
        let mut success = 0;
        glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0_i32;
        glGetProgramInfoLog(
            shader_program,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
        );
        v.set_len(log_len.try_into().unwrap());
        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);
    }

    'main_loop: loop {
        let start = std::time::Instant::now();

        while let Some(event) = sdl.poll_events() {
            match event.0 {
                events::Event::Quit => break 'main_loop,
                _ => (),
            }
        }

        // Here's where we could change the world state.

        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
        win.swap_window();

        let end = std::time::Instant::now();
        let elapsed: std::time::Duration = end - start;
        if elapsed < std::time::Duration::from_millis(16) {
            std::thread::sleep(std::time::Duration::from_millis(16) - elapsed);
        }
        println!("Update took `{}us`, from `{}us` possible", elapsed.as_micros(), 16_000);
    }
}