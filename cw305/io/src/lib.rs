#![no_std]

use io_trait::InputOutput;

pub struct IO;

impl InputOutput for IO {
    #[inline]
    fn start() {
        ext_io::led(false);
    }

    #[inline]
    fn read_byte() -> u8 {
        ext_io::read()
    }

    #[inline]
    fn write_byte(b: u8) {
        ext_io::write(b)
    }

    #[inline]
    fn write_word(w: u32) {
        ext_io::write_word(w)
    }

    #[inline]
    fn read_word() -> u32 {
        ext_io::read_word()
    }

    #[inline(never)]
    fn time_addr_read(addr: *const u32) -> u32 {
        ext_io::Timer::reset();
        ext_io::Timer::start();
        unsafe { addr.read_volatile(); }
        ext_io::Timer::stop();

        ext_io::Timer::value()
    }
}
