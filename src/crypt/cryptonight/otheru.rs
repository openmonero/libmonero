/*
 * This file is part of Monume's library libmonero
 *
 * Copyright (c) 2023-2024, Monume (monume.xyz)
 * All Rights Reserved
 * The code is distributed under MIT license, see LICENSE file for details.
 * Generated by Monume
 *
 */

use blake_hash::{Blake256, Digest as bhd};
use digest::KeyInit;
use groestl::Groestl256;
use jh::Jh256;
use sha3::Digest;
use skein::{Skein256, consts::U32};

pub(crate) fn turn_to_u8_16(u64p: [u64; 2]) -> [u8; 16] {
    let mut u8_16 = [0u8; 16];
    u8_16[0..8].copy_from_slice(&u64p[0].to_le_bytes());
    u8_16[8..16].copy_from_slice(&u64p[1].to_le_bytes());
    u8_16
}

pub(crate) fn turn_to_u64_2(u8_16: [u8; 16]) -> [u64; 2] {
    let mut u64_2 = [0u64; 2];
    u64_2[0] = u64::from_le_bytes(u8_16[0..8].try_into().unwrap());
    u64_2[1] = u64::from_le_bytes(u8_16[8..16].try_into().unwrap());
    u64_2
}

pub(crate) fn xor_pair_u64_2(a: [u64; 2], b: [u64; 2]) -> [u64; 2] {
    let mut res = [0u64; 2];
    res[0] = a[0] ^ b[0];
    res[1] = a[1] ^ b[1];
    res
}

pub(crate) fn turn_to_u64(u8_8: &[u8]) -> u64 {
    u64::from_le_bytes(u8_8.try_into().unwrap())
}

// ORIGINAL QUOTE:
// Where, the 8byte_add function represents each of the arguments as a
// pair of 64-bit little-endian values and adds them together,
// component-wise, modulo 2^64. The result is converted back into 16
// bytes.
pub(crate) fn add_pair_u64_2(a: [u64; 2], b: [u64; 2]) -> [u64; 2] {
    let mut res = [0u64; 2];
    res[0] = a[0].wrapping_add(b[0]);
    res[1] = a[1].wrapping_add(b[1]);
    res
}

// ORIGINAL QUOTE:
// The 8byte_mul function, however, uses only the first 8 bytes of each
// argument, which are interpreted as unsigned 64-bit little-endian
// integers and multiplied together. The result is converted into 16
// bytes, and finally the two 8-byte halves of the result are swapped.
pub(crate) fn mul_pair_u64_2(a: [u64; 2], b: [u64; 2]) -> [u64; 2] {
    let a = u128::from(a[0]);
    let b = u128::from(b[0]);
    let r = a * b;
    let res: [u64; 2] = [(r >> 64) as u64, r as u64];
    res
}

// HASHES

pub(crate) fn blake256_hash(input: [u8; 200]) -> [u8; 32] {
    let mut hasher = Blake256::new();
    hasher.update(input);
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&hasher.finalize().as_slice());
    hash
}

pub(crate) fn groestl256_hash(input: [u8; 200]) -> [u8; 32] {
    let mut hasher = Groestl256::default();
    hasher.update(input);
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&hasher.finalize());
    hash
}

pub(crate) fn jh256_hash(input: [u8; 200]) -> [u8; 32] {
    let mut hasher = Jh256::new();
    hasher.update(input);
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&hasher.finalize());
    hash
}

pub(crate) fn skein256_hash(input: [u8; 200]) -> [u8; 32] {
    let mut hasher = Skein256::<U32>::new();
    sha3::Digest::update(&mut hasher, input);
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&hasher.finalize());
    hash
}