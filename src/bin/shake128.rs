use std::io::Read;

use itertools::Itertools;
use TD1::{
    convert::{bytes_to_chunks, state_to_string},
    keccak::shake128,
    BYTES_PER_CHUNK, DIGITS_PER_BYTE, RATE, STATE_SIZE,
};

#[allow(non_snake_case)]
fn partial_XOR(chunk: &[u64], mut state: [u64; STATE_SIZE]) -> [u64; STATE_SIZE] {
    assert!(chunk.len() == RATE);
    for b in 0..RATE {
        state[b] ^= chunk[b]
    }
    state
}

fn main() {
    let hash_size: usize = std::env::args() // hash_size is a number of bytes
        .nth(1)
        .expect("No hash size provided")
        .parse()
        .expect("Argument is not an integer");

    let mut byt_input = Vec::new();
    let _ = std::io::stdin().read_to_end(&mut byt_input);
    let chunks = bytes_to_chunks(&byt_input);

    let mut state = chunks.into_iter().fold([0u64; STATE_SIZE], |state, chunk| {
        shake128(partial_XOR(&chunk, state))
    });

    let nb_states_stored = hash_size / (RATE * BYTES_PER_CHUNK) + 1;
    let mut hash: Vec<[u64; RATE]> = Vec::with_capacity(nb_states_stored);
    for _ in 0..nb_states_stored {
        hash.push(state[..RATE].try_into().unwrap());
        state = shake128(state);
    }

    println!("{}", {
        let mut hash = hash
            .into_iter()
            .map(|state| state_to_string(&state, ""))
            .join("");
        hash.truncate(hash_size * DIGITS_PER_BYTE);
        hash
    });
}

//#[cfg(test)]
//mod test {
//    use TD1::{convert::bytes_to_chunks, keccak::shake128, RATE, STATE_SIZE};
//
//    const EMPTY: [u64; STATE_SIZE] = [0; STATE_SIZE];
//    const EMPTY_PADDED: [u64; STATE_SIZE] = [
//        0x1F,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0x8000000000000000,
//        0,
//        0,
//        0,
//        0,
//    ];
//
//    #[test]
//    fn check_empty() {
//        assert_eq!(
//            EMPTY_PADDED[..RATE],
//            bytes_to_chunks(EMPTY).first().unwrap()
//        );
//    }
//}
