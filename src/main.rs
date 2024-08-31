#![allow(dead_code)]
#![allow(unused_variables)]
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

/// data stack
static mut D: &'static mut [isize] = &mut [0; Dsz];

/// check memory sizes
fn check_memory() {
    println!("M:0x{Msz:x} R:0x{Rsz:x} D:0x{Dsz:x}");
    const Mmin: usize = 0x10;
    const Rmin: usize = 0x10;
    const Dmin: usize = 0x10;
    assert!(Msz >= Mmin);
    assert!(Rsz >= Rmin);
    assert!(Dsz >= Dmin);
}

fn main() {
    check_memory();
    unsafe {
        vm();
    }
}

unsafe fn vm() {
    loop {
        fetch();
    }
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
