#![allow(non_upper_case_globals)]

/// @name FORTH

/// main memory size, bytes
pub const Msz: usize = 0x10000;
/// return stack size, cells
pub const Rsz: usize = 0x100;
/// data stack size, cells
pub const Dsz: usize = 0x10;

/// @name SDL

/// screen width, pixels
pub const W: u32 = 240;
/// screen height, pixels
pub const H: u32 = 320;

/// management server binding
pub const server: &str = "localhost:12345";
