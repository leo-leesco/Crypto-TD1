#![allow(non_snake_case)]

pub const BYTES_PER_WORD: usize = 8; // 8 u8 bytes per u64 word

/// number of words in a state
pub const STATE_SIZE: usize = 25;
/// in words ([`BYTES_PER_WORD`])
pub const CAPACITY: usize = 4;
/// in words ([`BYTES_PER_WORD`])
pub const RATE: usize = STATE_SIZE - CAPACITY;

pub mod convert;
pub mod keccak;
