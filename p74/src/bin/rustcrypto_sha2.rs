use hex;
use sha2::{Digest, Sha256, Sha512};
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <algorithm> <file_path>", args[0]);
        std::process::exit(1);
    }

    let algorithm = &args[1];
    let file_path = &args[2];

    // Open and read the file
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Compute the hash
    let hash = match algorithm.as_str() {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            hasher.finalize().to_vec()
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(&buffer);
            hasher.finalize().to_vec()
        }
        _ => {
            eprintln!("Unsupported algorithm: {}", algorithm);
            std::process::exit(1);
        }
    };

    // Print the hash as a hex-encoded string
    println!("{}", hex::encode(hash));

    Ok(())
}
