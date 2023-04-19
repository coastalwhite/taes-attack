#![no_main]
#![no_std]

#[cfg(not(any(feature = "gem5", feature = "cw305")))]
compile_error!("Either `gem5` or `cw305` must be selected as a feature");

use io_trait::InputOutput;

#[cfg(feature = "gem5")]
use gem5_io::IO;

#[cfg(feature = "cw305")]
use cw305_io::IO;

use core::arch::{asm, global_asm};
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

const MAGIC_VALUES: [u32; 8] = [
    0x1234_5678,
    0x9ABC_DEF0,
    0x4243_4445,
    0x1337_1509,
    0x0403_0201,
    0x0806_0402,
    0x0C09_0603,
    0x100C_0804,
];

#[no_mangle]
fn main() {
    IO::start();

    write_success(simple_write_read());
    write_success(write_read_cache_line());

    flush();
    write_success(simple_write_read());
    flush();
    write_success(write_read_cache_line());
    flush();
    write_success(write_evict_read());
    flush();
    write_success(consequentive_array_write_reads());

    write_success(does_cache());
    write_success(does_flush());


    IO::end();
}

fn write_success(is_success: bool) {
    if is_success {
        IO::write_word(1)
    } else {
        IO::write_word(0)
    }
}

fn flush() {
    for i in 0..128 {
        unsafe {
            ((0x0010_0000 | (i << 4)) as *const u32).read_volatile();
        }
    }
}

fn simple_write_read() -> bool {
    let mut out_value: u32;
    unsafe {
        asm!("
            sw  {magic},0({addr})
            lw  {out_value},0({addr})
        ",
        addr = in(reg) 0x0010_7000,
        magic = in(reg) MAGIC_VALUES[0],
        out_value = out(reg) out_value);
    }

    out_value == MAGIC_VALUES[0]
}

fn write_read_cache_line() -> bool {
    let mut out_value0: u32;
    let mut out_value1: u32;
    let mut out_value2: u32;
    let mut out_value3: u32;
    unsafe {
        asm!("
            sw  {magic0},0({addr})
            sw  {magic1},4({addr})
            sw  {magic2},8({addr})
            sw  {magic3},12({addr})
            lw  {out_value0},0({addr})
            lw  {out_value1},4({addr})
            lw  {out_value2},8({addr})
            lw  {out_value3},12({addr})
        ",
        addr = in(reg) 0x0010_7000,
        magic0 = in(reg) MAGIC_VALUES[0],
        magic1 = in(reg) MAGIC_VALUES[1],
        magic2 = in(reg) MAGIC_VALUES[2],
        magic3 = in(reg) MAGIC_VALUES[3],
        out_value0 = out(reg) out_value0,
        out_value1 = out(reg) out_value1,
        out_value2 = out(reg) out_value2,
        out_value3 = out(reg) out_value3,
        );
    }

    out_value0 == MAGIC_VALUES[0] &&
    out_value1 == MAGIC_VALUES[1] &&
    out_value2 == MAGIC_VALUES[2] &&
    out_value3 == MAGIC_VALUES[3] 
}

fn write_evict_read() -> bool {
    let mut p0 = MAGIC_VALUES[0];
    unsafe {
        asm!("
            lui     t1, 0x107
            sw  {p0},0(t1)
            lui     t1, 0x106
            lw  zero,0(t1)
            lui     t1, 0x105
            lw  zero,0(t1)
            lui     t1, 0x107
            lw  {p0},0(t1)
        ", p0 = inout(reg) p0, out("t1") _);
    }

    p0 == MAGIC_VALUES[0]
}

fn consequentive_array_write_reads() -> bool {
    let mut is_success = true;
    let array = unsafe {
        (0x0010_7000 as *mut [u32; 32]).as_mut().unwrap_unchecked()
    };

    for _ in 0..10 {
        for j in 0..8 {
            for i in 0..4 {
                array[j*4+i] = MAGIC_VALUES[(j+i) % 8];
            }
        }

        for j in 0..8 {
            for i in 0..4 {
                if array[j*4+i] != MAGIC_VALUES[(j+i) % 8] {
                    is_success = false;
                }
            }
        }
    }

    is_success
}

fn does_cache() -> bool {
    flush();

    let tta_1 = IO::time_addr_read(0x0010_7000 as *const u32);
    let tta_2 = IO::time_addr_read(0x0010_7000 as *const u32);

    tta_2 < tta_1
}

fn does_flush() -> bool {
    flush();

    IO::time_addr_read(0x0010_7000 as *const u32);
    let tta_1 = IO::time_addr_read(0x0010_7000 as *const u32);
    IO::time_addr_read(0x0010_6000 as *const u32);
    IO::time_addr_read(0x0010_5000 as *const u32);
    let tta_2 = IO::time_addr_read(0x0010_7000 as *const u32);

    tta_2 > tta_1
}
