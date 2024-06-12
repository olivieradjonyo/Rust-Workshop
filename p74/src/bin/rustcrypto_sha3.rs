use hex;
use sha3::{
    digest::{Digest, ExtendableOutput, Update, XofReader},
    Sha3_256, Sha3_512, Shake128, Shake256,
};
use std::env;
use std::fs::File;
use std::io::{self, Read};

//command line for hash: cargo run --bin rustcrypto_sha3 -- shake256  /Users/olivieradjonyo/Desktop/Rust-Workshop/p74/src/bin/file.txt
// Generic function to compute the digest for non-XOF algorithms like SHA3-256 and SHA3-512
fn compute_digest_of_file<T: Digest>(buffer: &[u8]) -> Vec<u8> {
    let mut hasher = T::new();
    hasher.update(buffer);
    hasher.finalize().to_vec()
}
//command line for xof: cargo run --bin rustcrypto_sha3 -- shake256 10000  /Users/olivieradjonyo/Desktop/Rust-Workshop/p74/src/bin/file.txt
// Generic function to compute the XOF for algorithms like SHAKE128 and SHAKE256
fn compute_xof_of_file<T: Default + Update + ExtendableOutput>(
    buffer: &[u8],
    digest_byte_len: usize,
) -> Vec<u8> {
    let mut hasher = T::default();
    hasher.update(buffer);
    let mut reader = hasher.finalize_xof();
    let mut result = vec![0u8; digest_byte_len];
    reader.read(&mut result);
    result
}

fn main() -> io::Result<()> {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || (args[1].starts_with("shake") && args.len() != 4) {
        eprintln!("Usage: {} <algorithm> [<output_size>] <file_path>", args[0]);
        std::process::exit(1);
    }

    let algorithm = &args[1];
    let file_path = if algorithm.starts_with("shake") {
        &args[3]
    } else {
        &args[2]
    };
    let output_size = if algorithm.starts_with("shake") {
        args[2].parse::<usize>().unwrap_or(128) // Default output size if parsing fails
    } else {
        0
    };

    // Open and read the file
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Compute the hash
    let hash = match algorithm.as_str() {
        "sha3_256" => compute_digest_of_file::<Sha3_256>(&buffer),
        "sha3_512" => compute_digest_of_file::<Sha3_512>(&buffer),
        "shake128" => compute_xof_of_file::<Shake128>(&buffer, output_size),
        "shake256" => compute_xof_of_file::<Shake256>(&buffer, output_size),
        _ => {
            eprintln!("Unsupported algorithm: {}", algorithm);
            std::process::exit(1);
        }
    };

    // Print the hash as a hex-encoded string
    println!("{}", hex::encode(hash));

    Ok(())
}
