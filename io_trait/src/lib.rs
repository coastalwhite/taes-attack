#![no_std]

pub trait InputOutput {
    fn start() {}

    fn read_byte() -> u8;
    fn write_byte(b: u8);
    
    fn read_word() -> u32;
    fn write_word(w: u32);

    fn finish_round() {}

    fn time_addr_read(addr: *const u32) -> u32;

    fn end() -> ! {
        loop {}
    }
}

