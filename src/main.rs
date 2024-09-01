#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_mut)]

pub mod config;

/// main memory
static mut M: &'static mut [u8] = &mut [0; config::Msz];
/// compiler pointer
static mut Cp: usize = 0;
/// instruction pointer
static mut Ip: usize = 0;

/// return stack
static mut R: &'static mut [usize] = &mut [0; config::Rsz];
/// return stack pointer
static mut Rp: u16 = 0;

/// data stack
static mut D: &'static mut [isize] = &mut [0; config::Dsz];
/// data stack pointer
static mut Dp: u8 = 0;

/// check memory sizes
fn check_memory() {
    eprintln!(
        "M:0x{:x} R:0x{:x} D:0x{:x}",
        config::Msz,
        config::Rsz,
        config::Dsz
    );
    const Mmin: usize = 0x10;
    const Mmax: usize = u16::MAX as usize + 1;
    assert!(config::Msz >= Mmin);
    assert!(config::Msz <= Mmax);
    const Rmin: usize = 0x10;
    const Rmax: usize = u16::MAX as usize + 1;
    assert!(config::Rsz >= Rmin);
    assert!(config::Rsz <= Rmax);
    const Dmin: usize = 0x10;
    const Dmax: usize = u8::MAX as usize + 1;
    assert!(config::Dsz >= Dmin);
    assert!(config::Dsz <= Dmax);
}

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
    check_memory();
    unsafe {
        vm();
    }
    tutorial();
    game(&argv[0]);
}

unsafe fn vm() {
    // loop {
    //     fetch();
    // }
}

unsafe fn fetch() {
    assert!(Ip < Cp);
    let op: u8 = M[Ip];
    Ip += 1;
}

/// `( -- )` empty command: do nothing
fn nop() {}
/// `( -- )` stop system
fn halt() {
    std::process::exit(0);
}

fn tutorial() {
    // https://rutube.ru/video/b24398e527ed2154aac4cd9081b316f6/?r=wd
    let (mut x, y) = (0, 0);
    [x, _] = [1, 2];
    eprintln!("x:{x}");
}

extern crate sdl2;
use std::time::Duration;

use sdl2::pixels::Color;

fn game(argv: &String) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(&argv, config::W, config::H)
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    canvas.set_draw_color(Color::RGB(0x22, 0x22, 0x22));
    canvas.clear();
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1));
}
