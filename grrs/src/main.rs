#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path);

    match result {
        Ok(content) => { println!("File content: {}", content); }
        Err(error) => { println!("Oh noes: {}", error); }
    }

    println!("Hello, world! ${:?} ${:?}", args.pattern, args.path);
}
