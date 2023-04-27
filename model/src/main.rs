const TRIES: usize = 1000000;

use model::{find_sets, NoiseType, are_correct_sets};

fn main() {
    let k = [b'b', b'u', b'p', b'e', b'r', b's', b'e', b'c', b'r', b'e', b't', b'k', b'e', b'y', b'!', b'!'];
    let noise = NoiseType {
        bits: 0,
        is_constant: true,
    };

    let mut all_queries = Vec::with_capacity(TRIES as usize);

    for _ in 0..TRIES {
        let (sets, queries) = find_sets(&k, noise);
        assert!(are_correct_sets(&k, &sets));
        
        all_queries.push(queries as f64);
    }

    let avg_queries = all_queries.iter().sum::<f64>() / TRIES as f64;
    let stddev_queries = (all_queries.iter().map(|x| {
        let v = x - avg_queries;
        v * v
    }).sum::<f64>() / (TRIES - 1) as f64).sqrt();

    println!("Queries: {avg_queries} (+/- {stddev_queries})");
}