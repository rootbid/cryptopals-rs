extern crate cryptopals_rs;

use std::str;
use cryptopals_rs::hex_ops::hex_to_buffer;
use cryptopals_rs::base64_ops::str_to_base64;

fn main() {
    println!("Cryptopals Crypto Challenges\n============================");
    
    // Set 1 -> Challenge 1
    println!("1.1 Convert hex to base64");
    println!("Result: {}", hex_to_base64());    
}

fn hex_to_base64() -> String {
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("Using `hex` input:\n{}", hex_string);
    str_to_base64(str::from_utf8(&hex_to_buffer(hex_string).unwrap()).unwrap())
}