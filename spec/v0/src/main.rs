use std::process;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: rey-v0 <file.rey>");
        process::exit(1);
    }
    let filename = &args[1];
      if !filename.ends_with(".rey") {
        eprintln!("Error: expected a .rey file");
        process::exit(1);
    }

    let source = fs::read_to_string(filename)
        .unwrap_or_else(|_| {
            eprintln!("Error: could not read file '{}'", filename);
            process::exit(1);
        });

    println!("Loaded file '{}'", filename);
    println!("--- SOURCE ---");
    println!("{}", source);
}
