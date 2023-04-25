#![no_std]

use core::arch::asm;

use io_trait::InputOutput;

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
        let mut mask: u32;
        loop {
            unsafe {
                asm!(
                    "ecall",
                    inout ("a7") 0x7FFF_150A => _,
                    inout ("a0") 0 => mask,
                    options(nostack),
                );
            }

            if mask & 0xFF00 != 0 {
                break;
            }
        }

        (mask & 0xFF) as u8
    }

    #[inline]
    fn write_byte(b: u8) {
        loop {
            let mut result: u32;
            unsafe {
                asm!(
                    "ecall",
                    inout ("a7") 0x7FFF_1509 => _,
                    inout ("a0") b as u32 => result,
                    options(nostack),
                );
            }
            
            if result == 0 {
                break;
            }
        }
    }

    fn read_word() -> u32 {
        return ((hex_to_nible(Self::read_byte()) as u32) << 28)
            | ((hex_to_nible(Self::read_byte()) as u32) << 24)
            | ((hex_to_nible(Self::read_byte()) as u32) << 20)
            | ((hex_to_nible(Self::read_byte()) as u32) << 16)
            | ((hex_to_nible(Self::read_byte()) as u32) << 12)
            | ((hex_to_nible(Self::read_byte()) as u32) << 8)
            | ((hex_to_nible(Self::read_byte()) as u32) << 4)
            | ((hex_to_nible(Self::read_byte()) as u32) << 0);
    }

    fn write_word(w: u32) {
        Self::write_byte(HEX_TABLE[(w.overflowing_shr(28).0 & 0xF) as usize]);
        Self::write_byte(HEX_TABLE[(w.overflowing_shr(24).0 & 0xF) as usize]);
        Self::write_byte(HEX_TABLE[(w.overflowing_shr(20).0 & 0xF) as usize]);
        Self::write_byte(HEX_TABLE[(w.overflowing_shr(16).0 & 0xF) as usize]);
        Self::write_byte(HEX_TABLE[(w.overflowing_shr(12).0 & 0xF) as usize]);
        Self::write_byte(HEX_TABLE[(w.overflowing_shr(8).0 & 0xF) as usize]);
        Self::write_byte(HEX_TABLE[(w.overflowing_shr(4).0 & 0xF) as usize]);
        Self::write_byte(HEX_TABLE[(w.overflowing_shr(0).0 & 0xF) as usize]);
        Self::write_byte(b'\n');
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
