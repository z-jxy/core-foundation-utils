use core_foundation_utils::ext::PathBufExt;

use std::path::PathBuf;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        eprintln!("Usage: attr <filename> <attr>");
        std::process::exit(1);
    }

    let file_name = &args[1];
    let attr_name = &args[2];

    let attr = PathBuf::from(file_name).attr::<String>(attr_name);

    println!("attr: {:?}", attr);
}
