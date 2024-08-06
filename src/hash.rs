use std::{u64, u8};

use sha2::{Digest, Sha256};

/*
 * Add 7 to the Number ->
 * This step ensures that any number that isn't already a multiple of 8 will be incremented
 * to the next multiple of 8 when the bitwise AND operation is performed
 * Bitwise AND with !7 ->
 * 7 in binary is 00000111.
 * !7 in binary is 11111000, which is the mask to clear the last 3 bits
*/
pub fn round_up_to_next_multiple_of_8(n: u64) -> u64 {
    // for handling overflow
    match n.checked_add(7) {
        Some(x) => (x + 7) & !7,
        // special case for 2^64
        _ => u64::MAX - 7,
    }
}

// Sha256 returns a vec of bytes, need to convert it to u64
pub fn get_hash(word: String, suffix: String, range: u64) -> u64 {
    let mut hasher_1 = Sha256::new();
    hasher_1.update(word.clone() + &suffix);
    let result_1 = hasher_1.finalize();
    // 8 bytes = 64 bits
    let bytes = &result_1[0..8];
    let number = u64::from_be_bytes(bytes.try_into().expect("slice with incorrect length"));
    number % range
}

pub fn hash(word: String, range: u64, hash_count: u8) -> Vec<u64> {
    let h1 = get_hash(word.to_string(), "key_1".to_string(), range);
    let h2 = get_hash(word, "key_2".to_string(), range);
    let mut hashes: Vec<u64> = vec![h1, h2];
    // we use  Kirsch-Mitzenmacher optimization, which suggests the following:
    // hash_i = hash1 + i * hash2, 0 ≤ i ≤ k - 1
    for i in 0..(hash_count - 2) {
        let h_i: u64 = h1 + ((i as u64) * h2);
        if h_i > range {
            hashes.push(h_i % range);
        } else {
            hashes.push(h_i);
        }
    }
    hashes
}
