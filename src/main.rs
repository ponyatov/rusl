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
    vm();
}

fn vm() {
    /// main memory
    let mut M: [u8; Msz];
    /// compiler pointer
    let mut Cp: usize = 0;
    /// instruction pointer
    let mut Ip: usize = 0;
}

/// `( -- )` empty command: do nothing
fn nop() {}
/// `( -- )` stop system
fn halt() {
    std::process::exit(0);
}
