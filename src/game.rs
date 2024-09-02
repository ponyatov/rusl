extern crate sdl2;
use std::time::Duration;

use crate::config;

pub fn game(argv: &String) {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let audio = sdl_context.audio().unwrap();
    let window = video
        .window(&argv, config::W, config::H)
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    canvas.set_draw_color(config::BG);
    canvas.clear();
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1));
}
