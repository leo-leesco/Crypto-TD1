pub fn state_to_string(state: [u64; 25]) -> String {
    state.iter().fold("".to_string(), |acc, word| {
        format!("{acc}{word:08X}").to_string()
    })
}
