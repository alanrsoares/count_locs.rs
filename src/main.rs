use std::env;
use count_locs::count_locs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <directory> <glob-patterns>...", args[0]);
        std::process::exit(1);
    }

    let root = std::fs::canonicalize(&args[1]).expect("Failed to resolve directory");
    let patterns = args[2..].to_vec();

    let results = count_locs(&root, &patterns);

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
