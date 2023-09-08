const WINDOW_TITLE: &str = "Hello Window";
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

use beryllium::*;

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    let _win = sdl.create_gl_window(video::CreateWinArgs{
        title: WINDOW_TITLE,
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        allow_high_dpi: false,
        borderless: false,
        resizable: true,
    }).expect("couldn't make a window and context");

    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event.0 {
                events::Event::Quit => break 'main_loop,
                _ => (),
            }
        }

        // here's where we could change the world state and draw.
    }
}