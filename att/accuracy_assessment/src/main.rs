#![no_main]
#![no_std]

#[cfg(not(any(feature = "gem5", feature = "cw305")))]
compile_error!("Either `gem5` or `cw305` must be selected as a feature");

use io_trait::InputOutput;

#[cfg(feature = "gem5")]
use gem5_io::IO;

#[cfg(feature = "cw305")]
use cw305_io::IO;

use core::arch::global_asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

global_asm!(
    r#"
.text
.globl _start
_start:
    lui     sp,0x108
    j       main
"#
);

#[no_mangle]
fn main() {
    IO::start();

    IO::time_addr_read(0x0010_4000 as *const u32);

    flush();

    // Write back the miss and hit time
    IO::write_word(IO::time_addr_read(0x0010_4000 as *const u32));
    IO::write_word(IO::time_addr_read(0x0010_4000 as *const u32));
    
    flush();

    for _ in 0..1000 {
        let offset = (IO::read_word() & 0xFF) << 2;
        let ptr = (0x0010_4000 + offset) as *const u32;

        let tta = IO::time_addr_read(ptr);

        IO::write_word(tta);
    }

    IO::end();
}

fn flush() {
    for i in 0..128 {
        unsafe {
            ((0x0010_0000 | (i << 4)) as *const u32).read_volatile();
        }
    }
}
