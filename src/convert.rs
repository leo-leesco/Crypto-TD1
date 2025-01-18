/// slices a u64 as 8 hexadecimal u8, ordered from least significant to most significant
pub fn unpack_bytes(packed_bytes: u64) -> [u8; 8] {
    let mut reducing = packed_bytes;
    let mut byte_array: [u8; 8] = [0; 8];
    for i in 0..8 {
        byte_array[i] = reducing as u8;
        reducing >>= 8 * size_of::<u8>(); // since size_of returns the size in bytes, you have to
                                          // count the number of bits per byte
        eprint!("{reducing:02X}");
        eprintln!(", extracted = {:02X}", byte_array[i]);
    }
    byte_array
}
