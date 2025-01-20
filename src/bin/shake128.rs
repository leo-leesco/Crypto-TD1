use std::io::Read;

use itertools::Itertools;
use TD1::{
    convert::bytes_to_chunks, keccak::shake128, BYTES_PER_CHUNK, DIGITS_PER_BYTE, RATE, STATE_SIZE,
};

#[allow(non_snake_case)]
fn partial_XOR(chunk: &[u64], mut state: [u64; STATE_SIZE]) -> [u64; STATE_SIZE] {
    assert!(chunk.len() == RATE);
    for b in 0..RATE {
        state[b] ^= chunk[b]
    }
    state
}

fn sponge(chunks: Vec<[u64; RATE]>) -> [u64; STATE_SIZE] {
    chunks.into_iter().fold([0u64; STATE_SIZE], |state, chunk| {
        shake128(partial_XOR(&chunk, state))
    })
}

fn squeeze(mut state: [u64; STATE_SIZE], hash_size: usize) -> Vec<[u64; RATE]> {
    let nb_states_stored = hash_size / (RATE * BYTES_PER_CHUNK) + 1;
    let mut hash: Vec<[u64; RATE]> = Vec::with_capacity(nb_states_stored);
    for _ in 0..nb_states_stored {
        hash.push(state[..RATE].try_into().unwrap());
        state = shake128(state);
    }
    hash
}

fn resize_hash(hash: Vec<[u64; RATE]>, hash_size: usize, separator: &str) -> String {
    format!("{}", {
        let mut hash = hash
            .into_iter()
            .map(|state| {
                state
                    .map(|word| {
                        word.to_le_bytes()
                            .map(|byt| format!("{byt:0width$X}", width = DIGITS_PER_BYTE))
                            .join(separator)
                    })
                    .join(separator)
            })
            .join(separator);
        hash.truncate(hash_size * DIGITS_PER_BYTE);
        hash
    })
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

    let state = sponge(chunks);
    let hash = resize_hash(squeeze(state, hash_size), hash_size, "");
    println!("{hash}");
}

#[cfg(test)]
mod test {
    use TD1::{
        convert::{bytes_to_chunks, example_to_state},
        RATE,
    };

    use crate::{resize_hash, sponge, squeeze};

    //const EMPTY: [u64; STATE_SIZE] = [0; STATE_SIZE];
    //const EMPTY_PADDED: [u64; STATE_SIZE] = [
    //    0x1F,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0,
    //    0x8000000000000000,
    //    0,
    //    0,
    //    0,
    //    0,
    //];

    const EMPTY: &str = "";

    #[test]
    fn in_to_init() {
        assert_eq!(
            example_to_state(INIT)[..RATE],
            bytes_to_chunks(EMPTY.as_bytes())
                .first()
                .unwrap()
                .to_owned()
        )
    }
    const INIT:&str = "1F 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ";

    #[test]
    fn init_to_sponge() {
        assert_eq!(
            example_to_state(SPONGE),
            sponge(vec!(example_to_state(INIT)[..RATE].try_into().unwrap()))
        );
    }
    const SPONGE:&str="7F 9C 2B A4 E8 8F 82 7D 61 60 45 50 76 05 85 3E D7 3B 80 93 F6 EF BC 88 EB 1A 6E AC FA 66 EF 26 3C B1 EE A9 88 00 4B 93 10 3C FB 0A EE FD 2A 68 6E 01 FA 4A 58 E8 A3 63 9C A8 A1 E3 F9 AE 57 E2 35 B8 CC 87 3C 23 DC 62 B8 D2 60 16 9A FA 2F 75 AB 91 6A 58 D9 74 91 88 35 D2 5E 6A 43 50 85 B2 BA DF D6 DF AA C3 59 A5 EF BB 7B CC 4B 59 D5 38 DF 9A 04 30 2E 10 C8 BC 1C BF 1A 0B 3A 51 20 EA 17 CD A7 CF AD 76 5F 56 23 47 4D 36 8C CC A8 AF 00 07 CD 9F 5E 4C 84 9F 16 7A 58 0B 14 AA BD EF AE E7 EE F4 7C B0 FC A9 4C CA AE BA 77 4E C2 0C FF 6A 94 85 A9 7B FC 65 AA 93 AA 4F C9 58 D1 ED B5 27 C0 2E 3A E5 7B CC ";

    #[test]
    fn sponge_to_squeeze() {
        assert_eq!(
            example_to_state(SQUEEZE)[..RATE],
            squeeze(example_to_state(SPONGE), 32)
                .first()
                .unwrap()
                .to_owned()
        );
    }
    const SQUEEZE:&str="7F 9C 2B A4 E8 8F 82 7D 61 60 45 50 76 05 85 3E D7 3B 80 93 F6 EF BC 88 EB 1A 6E AC FA 66 EF 26 3C B1 EE A9 88 00 4B 93 10 3C FB 0A EE FD 2A 68 6E 01 FA 4A 58 E8 A3 63 9C A8 A1 E3 F9 AE 57 E2 35 B8 CC 87 3C 23 DC 62 B8 D2 60 16 9A FA 2F 75 AB 91 6A 58 D9 74 91 88 35 D2 5E 6A 43 50 85 B2 BA DF D6 DF AA C3 59 A5 EF BB 7B CC 4B 59 D5 38 DF 9A 04 30 2E 10 C8 BC 1C BF 1A 0B 3A 51 20 EA 17 CD A7 CF AD 76 5F 56 23 47 4D 36 8C CC A8 AF 00 07 CD 9F 5E 4C 84 9F 16 7A 58 0B 14 AA BD EF AE E7 EE F4 7C B0 FC A9 4C CA AE BA 77 4E C2 0C FF 6A 94 85 A9 7B FC 65 AA 93 AA 4F C9 58 D1 ED B5 27 C0 2E 3A E5 7B CC ";

    #[test]
    fn squeeze_to_truncated() {
        assert_eq!(
            resize_hash(
                vec!(example_to_state(SQUEEZE)[..RATE].try_into().unwrap()),
                32,
                ""
            ),
            HASH
        );
    }
    const HASH: &str = "7F9C2BA4E88F827D616045507605853ED73B8093F6EFBC88EB1A6EACFA66EF26";
}
