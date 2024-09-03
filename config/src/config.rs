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

pub mod server {
    use const_format::formatcp;

    /// management server binding
    pub const ip: &str = "localhost";
    pub const port: u16 = 12345;
    pub const url: &str = formatcp!("{ip}:{port}");
}
