use core_foundation_utils::{prelude::KMDItemTypes, spotlight::SpotlightApi};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: mdquery <filename>");
        std::process::exit(1);
    }

    let file_name = &args[1];

    for item in SpotlightApi::raw_query(
        format!("kMDItemDisplayName = '*{file_name}*'"),
        KMDItemTypes::Path,
    ) {
        println!("[*] {}", item);
    }
}
