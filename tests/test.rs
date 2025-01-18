use TD1::convert::*;
use TD1::print::*;

#[test]
fn print_zero() {
    let state: [u64; 25] = [0; 25];
    let out = state_to_string(state);
    eprintln!("{out}, length = {}", out.len());
    assert_eq!(out.len(), 25 * 8);
}

const UNPACKED: [u8; 8] = [0x00, 0x0C, 0x11, 0xAA, 0x0A, 0xFF, 0x00, 0x10];
const PACKED: u64 = 0x10_00_FF_0A_AA_11_0C_00;

#[test]
fn unpacking() {
    assert_eq!(unpack_bytes(PACKED), UNPACKED);
}
#[test]
fn packing() {
    assert_eq!(PACKED, pack_bytes(UNPACKED));
}
