#![no_std]

use core::arch::asm;

use io_trait::InputOutput;

static mut WRITE_BUF: [u8; 16] = [0; 16];
static mut READ_BUF: [u8; 16] = [0; 16];

#[inline]
fn hex_to_nible(h: u8) -> u8 {
    (((h & 0x40) >> 6) * 9) + (h & 0xF)
}

static HEX_TABLE: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
];

pub struct IO;

impl InputOutput for IO {
    #[inline]
    fn start() {
        for b in b"*** START\n" {
            Self::write_byte(*b);
        }
    }

    #[inline]
    fn end() -> ! {
        unsafe {
            asm!(
                "li a7,93", // Syscall = Exit
                "li a0,0",  // STDOUT
                "ecall",
                options(noreturn)
            );
        }
    }

    #[inline]
    fn read_byte() -> u8 {
        loop {
            let mut size: u32;
            unsafe {
                asm!(
                    "ecall",
                    inout ("a7") 63 => _,
                    inout ("a0") 0 => size,
                    inout ("a1") &READ_BUF as *const u8 => _,
                    inout ("a2") 1 => _,
                    options(nostack),
                );
            }

            if size == 1 {
                break;
            }
        }

        unsafe { READ_BUF[0] }
    }

    #[inline]
    fn write_byte(b: u8) {
        unsafe {
            WRITE_BUF[0] = b;
        }

        loop {
            let mut size: u32;
            unsafe {
                asm!(
                    "ecall",
                    inout ("a7") 64 => _,
                    inout ("a0") 1 => size,
                    inout ("a1") &WRITE_BUF as *const u8 => _,
                    inout ("a2") 1 => _,
                    options(nostack),
                );
            }

            if size == 1 {
                break;
            }
        }
    }

    fn read_word() -> u32 {
        let b1 = loop {
            match Self::read_byte() {
                0 => continue,
                b => break b,
            }
        };

        loop {
            let mut size: u32;
            unsafe {
                asm!(
                    "ecall",
                    inout ("a7") 63 => _,
                    inout ("a0") 0 => size,
                    inout ("a1") &READ_BUF as *const u8 => _,
                    inout ("a2") 7 => _,
                    options(nostack),
                );
            }

            if size == 7 {
                break;
            }
        }

        unsafe {
            return ((hex_to_nible(b1) as u32) << 28)
                | ((hex_to_nible(READ_BUF[0]) as u32) << 24)
                | ((hex_to_nible(READ_BUF[1]) as u32) << 20)
                | ((hex_to_nible(READ_BUF[2]) as u32) << 16)
                | ((hex_to_nible(READ_BUF[3]) as u32) << 12)
                | ((hex_to_nible(READ_BUF[4]) as u32) << 8)
                | ((hex_to_nible(READ_BUF[5]) as u32) << 4)
                | ((hex_to_nible(READ_BUF[6]) as u32) << 0);
        }
    }

    fn write_word(w: u32) {
        unsafe {
            WRITE_BUF[0] = HEX_TABLE[(w.overflowing_shr(28).0 & 0xF) as usize];
            WRITE_BUF[1] = HEX_TABLE[(w.overflowing_shr(24).0 & 0xF) as usize];
            WRITE_BUF[2] = HEX_TABLE[(w.overflowing_shr(20).0 & 0xF) as usize];
            WRITE_BUF[3] = HEX_TABLE[(w.overflowing_shr(16).0 & 0xF) as usize];
            WRITE_BUF[4] = HEX_TABLE[(w.overflowing_shr(12).0 & 0xF) as usize];
            WRITE_BUF[5] = HEX_TABLE[(w.overflowing_shr(8).0 & 0xF) as usize];
            WRITE_BUF[6] = HEX_TABLE[(w.overflowing_shr(4).0 & 0xF) as usize];
            WRITE_BUF[7] = HEX_TABLE[(w.overflowing_shr(0).0 & 0xF) as usize];
            WRITE_BUF[8] = b'\n';
        }

        loop {
            let mut size: u32;

            unsafe {
                asm!(
                    "ecall",
                    inout ("a7") 64 => _,
                    inout ("a0") 1 => size,
                    inout ("a1") &WRITE_BUF as *const u8 => _,
                    inout ("a2") 9 => _,
                    options(nostack),
                );
            }

            if size == 9 {
                break;
            }
        }
    }

    fn time_addr_read(addr: *const u32) -> u32 {
        let value;

        unsafe {
            asm!(
                r#"
                    rdcycle {duration}
                    lw      {loaded_value},0({addr}) 
                    rdcycle {after}
                    sub     {duration},{after},{duration}
                "#,
                addr = inout (reg) addr => _,
                duration = out (reg) value,
                loaded_value = out (reg) _,
                after = out (reg) _,
            )
        }

        value
    }
}
