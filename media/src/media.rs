#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sdl2;
use std::time::Duration;

use config;

use sdl2::pixels::Color;

/// background color
const BG: Color = Color::RGB(0x22, 0x22, 0x22);
/// dark ground (status bar)
const DG: Color = Color::RGB(0x11, 0x11, 0x11);

fn game(argv: &String) {
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
    canvas.set_draw_color(BG);
    canvas.clear();
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32));
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    game(&argv[0]);
}
