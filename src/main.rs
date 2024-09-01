#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_mut)]

/// main memory size, bytes
const Msz: usize = 0x10000;
/// return stack size, cells
const Rsz: usize = 0x100;
/// data stack size, cells
const Dsz: usize = 0x10;

/// main memory
static mut M: &'static mut [u8] = &mut [0; Msz];
/// compiler pointer
static mut Cp: usize = 0;
/// instruction pointer
static mut Ip: usize = 0;

/// return stack
static mut R: &'static mut [usize] = &mut [0; Rsz];
/// return stack pointer
static mut Rp: u16 = 0;

/// data stack
static mut D: &'static mut [isize] = &mut [0; Dsz];
/// data stack pointer
static mut Dp: u8 = 0;

/// check memory sizes
fn check_memory() {
    eprintln!("M:0x{Msz:x} R:0x{Rsz:x} D:0x{Dsz:x}");
    const Mmin: usize = 0x10;
    const Mmax: usize = u16::MAX as usize + 1;
    assert!(Msz >= Mmin);
    assert!(Msz <= Mmax);
    const Rmin: usize = 0x10;
    const Rmax: usize = u16::MAX as usize + 1;
    assert!(Rsz >= Rmin);
    assert!(Rsz <= Rmax);
    const Dmin: usize = 0x10;
    const Dmax: usize = u8::MAX as usize + 1;
    assert!(Dsz >= Dmin);
    assert!(Dsz <= Dmax);
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
