use std::env;
use count_locs::{count_locs, print_error, print_help,HELP_MESSAGE};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        // Handle the help flag
        [_, flag] if flag == "--help" || flag == "-h" => {
            print_help();
        }

        // Handle the version flag
        [_, flag] if flag == "--version" || flag == "-v" => {
            println!("count_locs version {}", env!("CARGO_PKG_VERSION"));
        }

        // Handle insufficient arguments
        [_] if args.len() < 3 => {
            print_error(HELP_MESSAGE);
            std::process::exit(1);
        }

        // Handle valid input with a directory and glob patterns
        [_, dir, patterns @ ..] => {
            let root = std::fs::canonicalize(dir).expect("Failed to resolve directory");
            let results = count_locs(&root, &patterns.iter().map(String::from).collect::<Vec<_>>());

            let total_lines: usize = results.values().copied().sum();

            if patterns.len() > 1 {
                println!("Breakdown of Lines of Code by Glob:");
                for (pattern, &lines) in &results {
                    println!("  {}: {}", pattern, lines);
                }
                println!();
            }

            println!("Total lines of code: {}", total_lines);
        }

        // Fallback (this should never happen because of the above cases)
        _ => unreachable!(),
    }
}
