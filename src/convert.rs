use crate::{BITS_PER_BYTE, BYTES_PER_CHUNK, DIGITS_PER_BYTE, RATE};
use itertools::Itertools;

pub fn state_to_string(state: &[u64], separator: &str) -> String {
    state
        .into_iter()
        .map(|word| {
            format!(
                "{word:0>width$X}",
                width = DIGITS_PER_BYTE * BYTES_PER_CHUNK
            )
        })
        .collect::<Vec<String>>()
        .join(separator)
}

/// takes a string of the form `1F 08 44 55 67` and converts it into chunks, with correct
/// endianness
/// ```
/// # use TD1::convert::*;
/// assert_eq!(read_example("1F 08 44 55 00 00 00 00 67"), "000000005544081F 0000000000000067".to_string());
/// ```
pub fn read_example(example: &str) -> String {
    example
        .split_whitespace()
        .chunks(8)
        .into_iter()
        .map(|chunk| {
            format!(
                "{:0>width$}",
                chunk.collect::<Vec<_>>().into_iter().rev().join(""),
                width = DIGITS_PER_BYTE * BYTES_PER_CHUNK
            )
        })
        .join(" ")
}

#[allow(unused)]
/// slices a u64 as 8 hexadecimal u8, ordered from least significant to most significant
fn unpack_bytes(packed_bytes: &u64) -> [u8; BITS_PER_BYTE] {
    let mut reducing = *packed_bytes;
    let mut byte_array: [u8; BITS_PER_BYTE] = [0; BITS_PER_BYTE];
    for i in 0..BITS_PER_BYTE {
        byte_array[i] = reducing as u8;
        reducing >>= BITS_PER_BYTE * size_of::<u8>(); // since size_of returns the size in bytes, you have to
                                                      // count the number of bits per byte
        #[cfg(test)]
        eprintln!(
            "{reducing:0width$X}, extracted = {:0width$X}",
            byte_array[i],
            width = DIGITS_PER_BYTE
        );
    }
    byte_array
}

fn pack_bytes(unpacked_bytes: &[u8]) -> u64 {
    assert!(unpacked_bytes.len() == BITS_PER_BYTE);
    unpacked_bytes.iter().rev().fold(0, |packed, &byt| {
        //#[cfg(test)]
        //eprint!("{packed:0width$X}", width = DIGITS_PER_BYTE);
        (packed << (BITS_PER_BYTE * size_of::<u8>())) + (byt as u64)
    })
}

/// takes in (less than 168) bytes, and possibly appends 0b111110…01 to pad to 168 bytes
/// equivalently, the 168 byte-array first non filled byte is XORed with 0x1F, and the last byte
/// with 0x80
fn bytes_to_chunk(b: &[u8]) -> [u64; RATE] {
    assert!(b.len() <= RATE * BYTES_PER_CHUNK);
    if b.len() == RATE * BYTES_PER_CHUNK {
        b.chunks(BYTES_PER_CHUNK)
            .map(pack_bytes)
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap()
    } else {
        #[cfg(test)]
        eprintln!("{b:x?}");
        let mut padded = [0u8; RATE * BYTES_PER_CHUNK];
        padded[..b.len()].copy_from_slice(b);
        padded[padded.into_iter().position(|byte| byte == 0u8).unwrap_or(0)] ^= 0x1F;
        padded[padded.len() - 1] ^= 0x80;
        bytes_to_chunk(&padded)
    }
}

pub fn bytes_to_chunks(b: &[u8]) -> Vec<[u64; RATE]> {
    {
        if !b.is_empty() {
            b
        } else {
            #[cfg(test)]
            eprintln!("Empty string given here");
            &[0]
        }
    }
    .chunks(RATE * BYTES_PER_CHUNK)
    .map(bytes_to_chunk)
    .collect::<Vec<[u64; RATE]>>()
    .try_into()
    .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    const ZERO_STATE: [u64; STATE_SIZE] = [0; STATE_SIZE];

    #[test]
    fn print_zero() {
        let out = state_to_string(&ZERO_STATE, "");
        eprintln!("{out}, length = {}", out.len());
        assert_eq!(out.len(), STATE_SIZE * DIGITS_PER_BYTE * BYTES_PER_CHUNK);
    }

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

    #[test]
    #[should_panic]
    fn hex() {
        assert_eq!([0x13], "13".as_bytes());
    }

    #[test]
    //#[ignore = "For some reason, it looks like the test case has about 30 chunks, instead of the 21 we are expecting."]
    fn padding_zero() {
        let empty = "".as_bytes();
        let empty_padded= "1F 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80 ";
        assert_eq!(
            bytes_to_chunks(empty)
                .into_iter()
                .map(|chunk| state_to_string(&chunk, " "))
                .collect::<Vec<String>>()
                .join(" "),
            read_example(empty_padded)
        );
    }

    #[test]
    #[ignore = "This case is quite possibly problematic since it may not be correctly aligned"]
    fn padding_0x13() {
        let message = [0x53, 0x58, 0x7B, 0x19];
        let padded= "53 58 7B D9 07 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80 ";
        assert_eq!(
            bytes_to_chunks(&message)
                .into_iter()
                .map(|chunk| state_to_string(&chunk, " "))
                .collect::<Vec<String>>()
                .join(" "),
            read_example(padded)
        );
    }

    #[test]
    fn padding_0xA3_200_times() {
        let message = [0xA3; 168];
        let padded= "A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 ";
        assert_eq!(
            bytes_to_chunks(&message)
                .into_iter()
                .map(|chunk| state_to_string(&chunk, " "))
                .collect::<Vec<String>>()
                .join(" "),
            read_example(padded)
        );
    }

    #[test]
    #[ignore = "This cannot work since we process the chunks directly, but we compare against the first input, which is XORed with only the first chunk"]
    fn padding_0xA3_201_times() {
        let mut message = [0xA3; 169];
        message[168] = 0x03;
        let padded= "A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 A3 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ";
        assert_eq!(
            bytes_to_chunks(&message)
                .into_iter()
                .map(|chunk| state_to_string(&chunk, " "))
                .collect::<Vec<String>>()
                .join(" "),
            read_example(padded)
        );
    }
}
