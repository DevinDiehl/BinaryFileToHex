use std::env;
use std::fs;

fn main() {
    let path = env::args_os().nth(1).expect("Usage: BinaryFileToHex <file>");
    let bytes = fs::read(path).expect("could not read the file");
    println!("Read {} bytes", bytes.len());
    println!();
    print_hex(&bytes);
}

fn print_hex(bytes: &[u8]) {
    for chunk in bytes.chunks(16) {
        for byte in chunk {
            print!("{:02X} ", byte);
        }
        println!();
    }
}


