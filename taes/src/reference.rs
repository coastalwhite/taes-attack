use super::{mul, SBOX};


const RCON: [u32; 10] = [
    0x0100_0000,
    0x0200_0000,
    0x0400_0000,
    0x0800_0000,
    0x1000_0000,
    0x2000_0000,
    0x4000_0000,
    0x8000_0000,
    0x1B00_0000,
    0x3600_0000,
];

pub fn rotword(w: u32) -> u32 {
    w.rotate_left(8)
}

pub fn subword(w: u32) -> u32 {
    cc_bs![
        SBOX[((w & 0xFF00_0000) >> 24) as usize],
        SBOX[((w & 0x00FF_0000) >> 16) as usize],
        SBOX[((w & 0x0000_FF00) >> 8) as usize],
        SBOX[(w & 0x0000_00FF) as usize],
    ]
}

pub fn sub_bytes(bs: &mut [u32; 4]) {
    bs[0] = subword(bs[0]);
    bs[1] = subword(bs[1]);
    bs[2] = subword(bs[2]);
    bs[3] = subword(bs[3]);
}

pub fn key_schedule(ks: &mut [[u32; 4]; 11]) {
    for i in 1..11 {
        ks[i][0] = ks[i - 1][0] ^ subword(rotword(ks[i - 1][3])) ^ RCON[i - 1];
        ks[i][1] = ks[i - 1][1] ^ ks[i][0];
        ks[i][2] = ks[i - 1][2] ^ ks[i][1];
        ks[i][3] = ks[i - 1][3] ^ ks[i][2];
    }
}

pub fn shift_rows(cols: &mut [u32; 4]) {
    // a0 b0 c0 d0        a0 b0 c0 d0
    // a1 b1 c1 d1   >>   b1 c1 d1 a1
    // a2 b2 c2 d2   >>   c2 d2 a2 b2
    // a3 b3 c3 d3        d3 a3 b3 c3
    let a = cc_bs![
        (cols[0] >> 24) as u8,
        (cols[1] >> 16) as u8,
        (cols[2] >> 8) as u8,
        (cols[3] >> 0) as u8,
    ];
    let b = cc_bs![
        (cols[1] >> 24) as u8,
        (cols[2] >> 16) as u8,
        (cols[3] >> 8) as u8,
        (cols[0] >> 0) as u8,
    ];
    let c = cc_bs![
        (cols[2] >> 24) as u8,
        (cols[3] >> 16) as u8,
        (cols[0] >> 8) as u8,
        (cols[1] >> 0) as u8,
    ];
    let d = cc_bs![
        (cols[3] >> 24) as u8,
        (cols[0] >> 16) as u8,
        (cols[1] >> 8) as u8,
        (cols[2] >> 0) as u8,
    ];

    cols[0] = a;
    cols[1] = b;
    cols[2] = c;
    cols[3] = d;
}

pub fn mix_column(col: u32) -> u32 {
    let bs = col.to_be_bytes();

    cc_bs![
        mul(bs[0], 2) ^ mul(bs[1], 3) ^ mul(bs[2], 1) ^ mul(bs[3], 1),
        mul(bs[0], 1) ^ mul(bs[1], 2) ^ mul(bs[2], 3) ^ mul(bs[3], 1),
        mul(bs[0], 1) ^ mul(bs[1], 1) ^ mul(bs[2], 2) ^ mul(bs[3], 3),
        mul(bs[0], 3) ^ mul(bs[1], 1) ^ mul(bs[2], 1) ^ mul(bs[3], 2),
    ]
}

pub fn mix_columns(cols: &mut [u32; 4]) {
    cols[0] = mix_column(cols[0]);
    cols[1] = mix_column(cols[1]);
    cols[2] = mix_column(cols[2]);
    cols[3] = mix_column(cols[3]);
}

#[inline]
pub fn add_roundkey(cols: &mut [u32; 4], k: &[u32; 4]) {
    cols[0] ^= k[0];
    cols[1] ^= k[1];
    cols[2] ^= k[2];
    cols[3] ^= k[3];
}

pub fn round(cols: &mut [u32; 4], k: &[u32; 4]) {
    sub_bytes(cols);
    shift_rows(cols);
    mix_columns(cols);
    add_roundkey(cols, k);
}

pub fn encrypt(bs: &[u8; 16], k: &[u8; 16]) -> [u8; 16] {
    let mut ks = [[0u32; 4]; 11];

    ks[0][0] = cc_bs!(k[0], k[1], k[2], k[3]);
    ks[0][1] = cc_bs!(k[4], k[5], k[6], k[7]);
    ks[0][2] = cc_bs!(k[8], k[9], k[10], k[11]);
    ks[0][3] = cc_bs!(k[12], k[13], k[14], k[15]);

    let mut bs = [
        cc_bs!(bs[0], bs[1], bs[2], bs[3]),
        cc_bs!(bs[4], bs[5], bs[6], bs[7]),
        cc_bs!(bs[8], bs[9], bs[10], bs[11]),
        cc_bs!(bs[12], bs[13], bs[14], bs[15]),
    ];

    key_schedule(&mut ks);

    add_roundkey(&mut bs, &ks[0]);

    for i in 1..10 {
        round(&mut bs, &ks[i]);
    }

    sub_bytes(&mut bs);
    shift_rows(&mut bs);
    add_roundkey(&mut bs, &ks[10]);

    [
        (bs[0] >> 24) as u8,
        (bs[0] >> 16) as u8,
        (bs[0] >> 8) as u8,
        (bs[0] >> 0) as u8,
        (bs[1] >> 24) as u8,
        (bs[1] >> 16) as u8,
        (bs[1] >> 8) as u8,
        (bs[1] >> 0) as u8,
        (bs[2] >> 24) as u8,
        (bs[2] >> 16) as u8,
        (bs[2] >> 8) as u8,
        (bs[2] >> 0) as u8,
        (bs[3] >> 24) as u8,
        (bs[3] >> 16) as u8,
        (bs[3] >> 8) as u8,
        (bs[3] >> 0) as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_expansion() {
        let mut ks = [[0u32; 4]; 11];

        ks[0][0] = 0x2b7e1516;
        ks[0][1] = 0x28aed2a6;
        ks[0][2] = 0xabf71588;
        ks[0][3] = 0x09cf4f3c;

        key_schedule(&mut ks);

        assert_eq!(ks[1][0], 0xa0fafe17);
        assert_eq!(ks[1][1], 0x88542cb1);
        assert_eq!(ks[1][2], 0x23a33939);
        assert_eq!(ks[1][3], 0x2a6c7605);
    }

    #[test]
    fn encryption() {
        let input = 0x00112233445566778899aabbccddeeffu128;
        let key = 0x000102030405060708090a0b0c0d0e0fu128;
        let output = 0x69c4e0d86a7b0430d8cdb78070b4c55au128;

        assert_eq!(
            encrypt(&input.to_be_bytes(), &key.to_be_bytes()),
            output.to_be_bytes()
        );
    }
}
