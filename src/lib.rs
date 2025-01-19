#![allow(non_snake_case)]

pub const BITS_PER_BYTE: usize = 8;
/// 8 u8 bytes per u64 chunk
pub const BYTES_PER_CHUNK: usize = 8;
/// 2 digits in base 16 to represent one byte
pub const DIGITS_PER_BYTE: usize = 2;

/// number of chunks in a state
pub const STATE_SIZE: usize = 25;
/// in chunks ([`BYTES_PER_CHUNK`])
pub const CAPACITY: usize = 4;
/// in chunks ([`BYTES_PER_CHUNK`])
pub const RATE: usize = STATE_SIZE - CAPACITY;

pub mod convert;
pub mod keccak;
