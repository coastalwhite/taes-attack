use rand::Rng;

#[derive(Clone, Copy)]
pub struct NoiseType {
    pub bits: u8,
    pub is_constant: bool,
}

fn get_mask(pt: &[u8; 16], k: &[u8; 16]) -> u64 {
    let mut mask = 0;

    for i in 0..16 {
        mask |= 1 << ((pt[i] ^ k[i]) >> 2);
    }

    mask
}

fn generate_noise(bits: u8) -> u64 {
    let mut mask = 0;
    let mut done = 0;

    while done < bits {
        let o = rand::random::<u64>() & 0x3F;
        let o = 1 << o;

        if mask & o != 0 {
            continue;
        }

        mask |= o;
        done += 1;
    }

    mask
}

const CERTAINTY: usize = 10;

pub fn count_one_frequency(ns: [u64; CERTAINTY]) -> [u8; 64] {
    let mut out = [0; 64];

    for mut n in ns {
        for i in 0..64 {
            if n & 1 != 0 {
                out[i] += 1;
            }
            n >>= 1;
        }
    }

    out
}

pub fn freq_to_num(freqs: &[u8; 64]) -> u64 {
    let mut num = 0;

    for (i, freq) in freqs.iter().enumerate() {
        if *freq >= CERTAINTY as u8 / 2 {
            num |= 1 << i;
        }
    }

    num
}

/// Find a number of sets for a `k` with some noise.
pub fn find_sets(k: &[u8; 16], noise: NoiseType) -> ([u8; 16], u64) {
    let mut num_mappings = 0;
    let mut mappings = [None; 16];

    let mut queries = 0;

    let mut rng = rand::thread_rng();

    let const_mask_in = generate_noise(noise.bits);

    let w_get_mask = |pt: &[u8; 16], k: &[u8; 16]| {
        let mask = get_mask(pt, k);
        let mask_in = if noise.is_constant {
            const_mask_in
        } else {
            generate_noise(noise.bits)
        };
        mask | mask_in
    };

    let mut pt = [0; 16];
    while num_mappings != 16 {
        for i in 0..16 {
            pt[i] = rng.gen();
        }

        queries += 1;
        let base_mask = w_get_mask(&pt, k);

        for i in 0..16 {
            if mappings[i].is_some() {
                continue;
            }

            for j in 0..=u8::MAX {
                if j == pt[i] {
                    continue;
                }

                let mut updated_pt = pt.clone();
                updated_pt[i] = j;
                queries += 1;
                let updated_mask = w_get_mask(&updated_pt, k);

                let diff_mask = base_mask ^ updated_mask;

                if diff_mask == 0 {
                    continue;
                }

                // This triggers in two cases
                // 1. It jumps from a shared spot to a individual spot
                // 2. It jumps from a individual spot to a shared spot
                if diff_mask.count_ones() == 1 {
                    let bit_idx = diff_mask.trailing_zeros() as u8;

                    // If it jumped from a individual spot, we know that spot belonged to
                    // this byte
                    if diff_mask & base_mask != 0 {
                        mappings[i] = Some(bit_idx ^ (pt[i] >> 2));
                        num_mappings += 1;
                        break;
                    }

                    // If it jumped from a individual spot, we know that spot belonged to
                    // this byte
                    if diff_mask & updated_mask != 0 {
                        mappings[i] = Some(bit_idx ^ (updated_pt[i] >> 2));
                        num_mappings += 1;
                        break;
                    }
                }

                // This triggers when it jumps from an individual spot to another
                // individual spot
                if diff_mask.count_ones() == 2 {
                    let original_bit = diff_mask & base_mask;
                    assert!(original_bit.count_ones() == 1);

                    let bit_idx = diff_mask.trailing_zeros() as u8;
                    mappings[i] = Some(bit_idx ^ (pt[i] >> 2));
                    num_mappings += 1;
                    break;
                }
            }
        }
    }

    (std::array::from_fn(|i| mappings[i].unwrap()), queries)
}

pub fn are_correct_sets(k: &[u8; 16], sets: &[u8; 16]) -> bool {
    for i in 0..16 {
        let m = sets[i];

        let start = m * 4;
        let end = start + 3;

        if k[i] < start && k[i] > end {
            return false;
        }
    }

    true
}
