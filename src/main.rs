use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    filenames: Vec<String>,

    #[arg(short='c', long=None, help="Print the byte count")]
    print_count: bool,
}

fn main() {
    let args = Args::parse();
    let total_files = args.filenames.len();
    let mut total_bytes: u64 = 0;

    for filename in args.filenames {
        let metadata = fs::metadata(&filename).expect("Failed to read file metadata");
        total_bytes += metadata.len();
        println!("{} {}", metadata.len(), filename);
    }

    if total_files > 1 {
        println!("{} total", total_bytes);
    }
}
