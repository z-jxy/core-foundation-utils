use core_foundation_utils::mds::query_file_attr;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: mdls <filename>");
        std::process::exit(1);
    }
    let file_name = &args[1];

    let data: Vec<String> = query_file_attr(file_name, "kMDItemContentTypeTree").unwrap();
    println!("kMDItemContentType: {:?}", data);
}
