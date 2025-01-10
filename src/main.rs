#![allow(dead_code)]
#![allow(unused_variables)]

fn state_to_string(state: [u64; 25]) -> String {
    state.iter().fold("".to_string(), |acc, word| {
        format!("{acc}{word:X}").to_string()
    })
}

fn main() {
    let state: [u64; 25];
    let b: usize = 1600; // 1600 = 64 x 5 x 5
    let r: usize;
    let c: usize;
    println!("Hello, world!");
}
