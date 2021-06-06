#![allow(unused_imports)]

use piston_window::{
    AdvancedWindow, Button, Event, EventLoop, Key, OpenGL, PistonWindow, PressEvent, Size, Window,
    WindowSettings,
};
use sdl2::video::FullscreenType;
use sdl2_window::Sdl2Window;

#[tokio::main]
async fn main() {
    let gl = OpenGL::V4_5;
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("Downloader", [1., 1.])
        .exit_on_esc(true)
        .samples(16)
        .vsync(true)
        .graphics_api(gl)
        .build()
        .expect("Couldn't create a window");
    window.set_capture_cursor(false);
    window.set_max_fps(60);
    window.set_ups(30);
    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            if let Button::Keyboard(key) = button {
                match key {
                    Key::R => println!("{}", "This"),
                    Key::Q => break,
                    Key::F | Key::F12 => fullscreen(&mut window),
                    _ => {}
                }
            }
        }
    }
}
fn fullscreen(window: &mut PistonWindow<Sdl2Window>) {
    match window.window.window.fullscreen_state() {
        FullscreenType::Off => &window.window.window.set_fullscreen(FullscreenType::Desktop),
        FullscreenType::True => &window.window.window.set_fullscreen(FullscreenType::Desktop),
        FullscreenType::Desktop => &window.window.window.set_fullscreen(FullscreenType::Off),
    };
}
