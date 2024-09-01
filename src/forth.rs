use std::process::{abort, exit};

use crate::config;

/// main memory
pub static mut M: &'static mut [u8] = &mut [0; config::Msz];
/// compiler pointer
pub static mut Cp: usize = 0;
/// instruction pointer
pub static mut Ip: usize = 0;
/// current opcode
pub static mut op: cmd = cmd::undef;

/// return stack
pub static mut R: &'static mut [usize] = &mut [0; config::Rsz];
/// return stack pointer
pub static mut Rp: u16 = 0;

/// data stack
pub static mut D: &'static mut [isize] = &mut [0; config::Dsz];
/// data stack pointer
pub static mut Dp: u8 = 0;

/// check memory sizes
pub fn check_memory() {
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

/// fetch next opcode
unsafe fn fetch() {
    assert!(Ip < Cp);
    eprint!("\n{:04x}: ", Ip);
    op = unsafe { ::std::mem::transmute(M[Ip]) };
    eprint!("{:02x} ", op as u8);
    Ip += 1;
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum cmd {
    nop = 0x00,
    halt = 0xff,
    undef = 0xee,
}

pub unsafe fn vm() {
    c(cmd::nop as u8);
    c(cmd::halt as u8);
    // loop {
    fetch();
    match op {
        cmd::nop => nop(),
        cmd::halt => halt(),
        _ => {
            eprintln!("unknown opcode");
            abort()
        }
    }
    // }
}

/// `( -- )` empty command: do nothing
fn nop() {
    eprintln!("nop");
}
/// `( -- )` stop system
fn halt() {
    eprintln!("halt");
    exit(0);
}

unsafe fn c(byte: u8) {
    M[Cp] = byte;
    Cp += 1;
    assert!(Cp < config::Msz);
}
