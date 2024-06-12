use aes_siv::aead::{generic_array::GenericArray, Aead, KeyInit};
use aes_siv::{Aes128SivAead, Nonce};
use hex::decode;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;
//command for encrypt: cargo run --bin rustcrypto_siv -- enc /Users/olivieradjonyo/Desktop/Rust-Workshop/p74/src/bin/input.txt /Users/olivieradjonyo/Desktop/Rust-Workshop/p74/src/bin/output.enc e6d52fced5c1234b7e50eae9f365b4783b94e7a58fd3208f7db23f0872f7b8da
//command for decrypt: cargo run --bin rustcrypto_siv -- dec /Users/olivieradjonyo/Desktop/Rust-Workshop/p74/src/bin/output.enc /Users/olivieradjonyo/Desktop/Rust-Workshop/p74/src/bin/input.txt e6d52fced5c1234b7e50eae9f365b4783b94e7a58fd3208f7db23f0872f7b8da
fn main() -> io::Result<()> {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "Usage: {} <enc|dec> <input_file> <output_file> <key>",
            args[0]
        );
        process::exit(1);
    }

    let operation = &args[1];
    let input_file_path = &args[2];
    let output_file_path = &args[3];
    let key_hex = &args[4];

    // Decode the hex-encoded key
    let key_bytes = match decode(key_hex) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to decode key: {}", e);
            process::exit(1);
        }
    };

    if key_bytes.len() != 32 {
        eprintln!("Invalid key length: expected 32 bytes for AES-128-SIV");
        process::exit(1);
    }

    // Create the AES-128-SIV cipher instance
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes128SivAead::new(key);

    // Read the input file
    let mut input_file = File::open(input_file_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    // Encrypt or decrypt
    let result: Vec<u8> = match operation.as_str() {
        "enc" => {
            // Encrypt

            match cipher.encrypt(Nonce::from_slice(&[0u8; 16]), &buffer[..]) {
                Ok(encrypted_data) => encrypted_data,
                Err(e) => {
                    eprintln!("Encryption failed: {}", e);
                    process::exit(1);
                }
            }
        }
        "dec" => {
            // Decrypt

            match cipher.decrypt(Nonce::from_slice(&[0u8; 16]), &buffer[..]) {
                Ok(decrypted_data) => decrypted_data,
                Err(e) => {
                    eprintln!("Decryption failed: {}", e);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unsupported operation: {}", operation);
            process::exit(1);
        }
    };

    // Write the result to the output file
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(&result)?;
    output_file.flush()?;

    Ok(())
}
