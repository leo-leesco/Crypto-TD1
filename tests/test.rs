#[test]
fn print_zero() {
    let state: [u64; 25] = [0; 25];
    let out = TD1::print::state_to_string(state);
    println!("{out}, length = {}", out.len());
    assert_eq!(out.len(), 25 * 8);
}
