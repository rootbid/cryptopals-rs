/// Operations on hex strings
/// Allows encoding to and decoding from `hex strings`
extern crate hex;

use hex::{decode, encode, FromHexError};

/// Convert `str` to a lowercase hex string.
pub fn buffer_to_hex(buffer: &str) -> String {
    encode(buffer)
}

/// Convert a lowercase hex string to a `Vec<u8>`.
pub fn hex_to_buffer(hex_string: &str) -> Result<Vec<u8>, FromHexError> {
    decode(hex_string)
}

pub fn run() {
    assert_eq!(
        buffer_to_hex("I'm killing your brain like a poisonous mushroom"),
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
    );
    assert_eq!(
        hex_to_buffer("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap(),
        "I'm killing your brain like a poisonous mushroom".to_owned().as_bytes()
    )
}