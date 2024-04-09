use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    filename: String,

    #[arg(short='c', long=None, help="Print the byte count")]
    print_count: bool,
}

fn main() {
    let args = Args::parse();

    let metadata = fs::metadata(&args.filename).expect("Failed to read file metadata");

    println!("{} {}", metadata.len(), args.filename);
}
