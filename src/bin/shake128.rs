use std::{io::Read, usize};

use TD1::print::*;

fn main() {
    let hash_size: usize = std::env::args() // hash_size is a number of bytes
        .nth(1)
        .expect("No hash size provided")
        .parse()
        .expect("Argument is not an integer");

    let mut byt_input = Vec::new();
    let _ = std::io::stdin().read_to_end(&mut byt_input);
    let states = bytes_to_states(&byt_input);
}
