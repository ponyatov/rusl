#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_mut)]

pub mod config;
pub mod forth;

fn main() {
    fn arg(argc: usize, argv: &str) {
        eprintln!("argv[{argc}] = {argv:?}");
    }
    // \ arg
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();
    arg(0, &argv[0]);
    for (argc, argv) in argv.iter().enumerate().skip(1) {
        arg(argc, argv);
    }
    // /
    forth::check_memory();
    unsafe {
        forth::vm();
    }
    tutorial();
    game(&argv[0]);
}

fn tutorial() {
    // https://rutube.ru/video/b24398e527ed2154aac4cd9081b316f6/?r=wd
    let (mut x, y) = (0, 0);
    [x, _] = [1, 2];
    eprintln!("x:{x}");
}

extern crate sdl2;
use std::time::Duration;

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
    canvas.set_draw_color(config::BG);
    canvas.clear();
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1));
}
