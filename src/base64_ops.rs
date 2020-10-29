/// Operations on base64 strings
/// Allows encoding to and decoding from `base64 strings`.
/// 
extern crate base64;

use base64::{encode, decode, DecodeError};

/// Convert a `str` to a base64 string
pub fn str_to_base64(buffer: &str) -> String {
    encode(buffer)
}

/// Convert a base64 string to a `Vec<[u8]>`
pub fn base64_to_bytes(base64_string: &str) -> Result<Vec<u8>, DecodeError> {
    decode(base64_string)
}

pub fn run() {
    assert_eq!(
        str_to_base64("I'm killing your brain like a poisonous mushroom"),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
    assert_eq!(
        base64_to_bytes("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t").unwrap(),
        "I'm killing your brain like a poisonous mushroom".to_owned().as_bytes()
    )
}