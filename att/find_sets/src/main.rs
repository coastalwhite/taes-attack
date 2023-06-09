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
    lui     sp,0x107
    j       main
"#
);

#[no_mangle]
fn main() {
    IO::start();

    let ttable_ptr = 0x0010_7000 as *mut taes::TTable;
    let ttable = unsafe { ttable_ptr.as_mut().unwrap_unchecked() };
    taes::fill_ttable(ttable);

    loop {
        prime();

        let p0 = IO::read_word();
        let p1 = IO::read_word();
        let p2 = IO::read_word();
        let p3 = IO::read_word();

        // VICTIM
        taes::tt_forward(&mut [p0, p1, p2, p3], &ttable);
        
        // Reference accesses
        // unsafe {
        //     ((0x0010_7000 | (((p0 >> 24) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p0 >> 16) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p0 >>  8) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p0 >>  0) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p1 >> 24) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p1 >> 16) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p1 >>  8) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p1 >>  0) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p2 >> 24) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p2 >> 16) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p2 >>  8) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p2 >>  0) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p3 >> 24) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p3 >> 16) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p3 >>  8) & 0xFF) << 2)) as *const u32).read_volatile();
        //     ((0x0010_7000 | (((p3 >>  0) & 0xFF) << 2)) as *const u32).read_volatile();
        // }

        probe();
    }

    // IO::end();
}

#[inline(never)]
fn prime() {
    for i in 0..128 {
        unsafe {
            ((0x0010_0000 | (i << 4)) as *const u32).read_volatile();
        }
    }
}

#[inline(never)]
fn probe() {
    for i in 0..64 {
        let time = IO::time_addr_read((0x0010_0000 | (i << 4)) as *const u32);
        IO::write_word(time);
    }
}
