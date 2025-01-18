use TD1::convert::unpack_bytes;

#[test]
fn print_zero() {
    let state: [u64; 25] = [0; 25];
    let out = TD1::print::state_to_string(state);
    eprintln!("{out}, length = {}", out.len());
    assert_eq!(out.len(), 25 * 8);
}

#[test]
fn packing() {
    let bytes = [0x00, 0x0C, 0x11, 0xAA, 0x0A, 0xFF, 0x00, 0x10];
    let packed = 0x10_00_FF_0A_AA_11_0C_00;
    eprintln!("{:X}", packed);
    eprintln!("Unpacking…");
    let unpacked = unpack_bytes(packed);
    assert_eq!(unpacked, bytes);
}
