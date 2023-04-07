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

    for _ in 0..10 {
        IO::write_word(IO::read_word() + IO::read_word());
    }

    IO::end();
}
