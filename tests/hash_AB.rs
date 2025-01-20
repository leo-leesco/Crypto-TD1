use std::{
    fs::File,
    io::{Read, Write},
    process::{Command, Stdio},
};

#[test]
fn text() {
    eprintln!(
        "{}",
        String::from_utf8(Command::new("pwd").output().unwrap().stdout)
            .expect("Command output is not a valid UTF8 string"),
    );

    Command::new("cargo")
        .arg("build")
        .status()
        .expect("Could not build shake128");

    let mut text_data = Vec::new();
    let _ = File::open("tests/short-text.txt")
        .expect("Could not open file")
        .read_to_end(&mut text_data)
        .expect("Failed to read file");

    let mut shake128 = Command::new("./target/debug/shake128")
        .arg("32")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not execute shake128");

    shake128
        .stdin
        .as_mut()
        .expect("Could not access stdin")
        .write_all(&text_data)
        .expect("Failed to write to shake128's stdin");

    assert_eq!(
        String::from_utf8(
            shake128
                .wait_with_output()
                .expect("Failed to wait on child process")
                .stdout
        )
        .expect("Command output is not a valid UTF8 string")
        .trim(),
        "BA27CC6A7A85887A1888C0678C05CD7FCF619ED791DCE41B7E1A81C280BEC8BB"
    );
}
#[test]
fn binary() {
    eprintln!(
        "{}",
        String::from_utf8(Command::new("pwd").output().unwrap().stdout)
            .expect("Command output is not a valid UTF8 string"),
    );

    Command::new("cargo")
        .arg("build")
        .status()
        .expect("Could not build shake128");

    let mut bin_data = Vec::new();
    let _ = File::open("tests/short-binary.bin")
        .expect("Could not open file")
        .read_to_end(&mut bin_data)
        .expect("Failed to read file");

    let mut shake128 = Command::new("./target/debug/shake128")
        .arg("32")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not execute shake128");

    shake128
        .stdin
        .as_mut()
        .expect("Could not access stdin")
        .write_all(&bin_data)
        .expect("Failed to write to shake128's stdin");

    assert_eq!(
        String::from_utf8(
            shake128
                .wait_with_output()
                .expect("Failed to wait on child process")
                .stdout
        )
        .expect("Command output is not a valid UTF8 string")
        .trim(),
        "9B171CCF7FF6B9478CE02A54A5A558DDE55FEBC70E12F0ED402567639E404B74"
    );
}
