use crate::{BYTES_PER_WORD, RATE, STATE_SIZE};

#[allow(unused)]
/// slices a u64 as 8 hexadecimal u8, ordered from least significant to most significant
fn unpack_bytes(packed_bytes: &u64) -> [u8; 8] {
    let mut reducing = *packed_bytes;
    let mut byte_array: [u8; 8] = [0; 8];
    for i in 0..8 {
        byte_array[i] = reducing as u8;
        reducing >>= 8 * size_of::<u8>(); // since size_of returns the size in bytes, you have to
                                          // count the number of bits per byte
        #[cfg(test)]
        eprint!("{reducing:02X}");
        #[cfg(test)]
        eprintln!(", extracted = {:02X}", byte_array[i]);
    }
    byte_array
}

fn pack_bytes(unpacked_bytes: &[u8]) -> u64 {
    assert!(unpacked_bytes.len() == 8);
    unpacked_bytes.iter().rev().fold(0, |packed, &byt| {
        #[cfg(test)]
        eprint!("{packed:02X}");
        (packed << (8 * size_of::<u8>())) + (byt as u64)
    })
}

pub fn state_to_string(state: [u64; STATE_SIZE], separator: &str) -> String {
    state
        .map(|word| format!("{word:0width$X}", width = BYTES_PER_WORD))
        .join(separator)
}

/// takes in (less than 168) bytes, and possibly appends 0b111110…01 to pad to 168 bytes
/// equivalently, the 168 byte-array first non filled byte is XORed with 0x1F, and the last byte
/// with 0x80
fn chunk_to_state(b: &[u8]) -> [u64; RATE] {
    assert!(b.len() <= RATE);
    if b.len() == RATE {
        b.chunks(8)
            .map(pack_bytes)
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap()
    } else {
        let mut padded = [0u8; RATE];
        padded[..b.len()].copy_from_slice(b);
        padded[b.len()] ^= 0x1F;
        padded[padded.len() - 1] ^= 0x80;
        chunk_to_state(&padded)
    }
}

pub fn bytes_to_states(b: &[u8]) -> Vec<[u64; RATE]> {
    b.chunks(BYTES_PER_WORD)
        .map(chunk_to_state)
        .collect::<Vec<[u64; RATE]>>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const UNPACKED: [u8; 8] = [0x00, 0x0C, 0x11, 0xAA, 0x0A, 0xFF, 0x00, 0x10];
    const PACKED: u64 = 0x10_00_FF_0A_AA_11_0C_00;

    #[test]
    fn unpacking() {
        assert_eq!(unpack_bytes(&PACKED), UNPACKED);
    }

    #[test]
    fn packing() {
        assert_eq!(PACKED, pack_bytes(&UNPACKED));
    }

    const ZERO_STATE: [u64; STATE_SIZE] = [0; STATE_SIZE];
    const EMPTY_MESSAGE: &str = "";
    const EMPTY_PADDED:&str = "1F 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00";

    #[test]
    fn print_zero() {
        let out = state_to_string(ZERO_STATE, "");
        eprintln!("{out}, length = {}", out.len());
        assert_eq!(out.len(), STATE_SIZE * BYTES_PER_WORD);
    }

    #[test]
    fn padding_zero() {
        let byt = EMPTY_MESSAGE.as_bytes();
        let padded = EMPTY_PADDED
            .split_whitespace()
            .map(|val| u8::from_str_radix(val, 16));
        assert_eq!(bytes_to_states(byt),)
    }
}
