use std::env;
use std::fs;

fn main() {
    let filename = env::args()
        .nth_back(0)
        .expect("Filename argument to be provided");

    let metadata = fs::metadata(&filename).expect("Failed to read file metadata");

    println!("{} {}", metadata.len(), filename);
}
