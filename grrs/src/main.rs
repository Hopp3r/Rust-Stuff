#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
    let file: File = File::open(&args.path).expect("The specified file could not be opened.");
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        let size: Result<usize> = reader.read_line(&mut line);
        match size {
            Ok(0) => {
                println!("The pattern: {}\n Could not be found.", &args.pattern);
                break;
            }
            Ok(_) => {
                if line.contains(&args.pattern) {
                    println!(
                        "The following line matches the pattern: {}\n\n{}",
                        &args.pattern, line
                    );
                    break;
                }
            }
            Err(error) => println!("There was an error parsing a file line."),
        }
    }
}
