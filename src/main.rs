use clap::Parser;
use std::collections::HashMap;
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

    let mut byte_map: HashMap<String, u64> = HashMap::new();
    let mut total_bytes: u64 = 0;
    let mut print_count = args.print_count;

    if !(args.print_count) {
        print_count = true;
    }

    for filename in &args.filenames {
        if print_count {
            let metadata = fs::metadata(&filename).expect("Failed to read file metadata");
            byte_map.insert(filename.to_string(), metadata.len());
            total_bytes += metadata.len();
        }
    }

    let total_bytes_width = total_bytes.to_string().len();

    for filename in &args.filenames {
        if print_count {
            let bytes = byte_map.get(filename).unwrap();
            println!("{:width$} {}", bytes, filename, width = total_bytes_width);
        }
    }
    if total_files > 1 {
        if print_count {
            println!("{:width$} total", total_bytes, width = total_bytes_width);
        }
    }
}
