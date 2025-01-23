use globwalk;
use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn count_lines(file_path: &Path) -> usize {
    fs::File::open(file_path)
        .ok()
        .map(|file| io::BufReader::new(file).lines().count())
        .unwrap_or(0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <directory> <glob-patterns>...", args[0]);
        std::process::exit(1); // Exit with a non-zero status
    }

    let root = std::fs::canonicalize(&args[1]).expect("Failed to resolve directory");
    let patterns = &args[2..];

    let results: HashMap<_, _> = patterns
        .iter()
        .map(|pattern| {
            let lines: usize = globwalk::GlobWalkerBuilder::from_patterns(&root, &[pattern])
                .build()
                .expect("Failed to build glob walker")
                .into_iter()
                .filter_map(Result::ok)
                .par_bridge()
                .map(|entry| count_lines(entry.path()))
                .sum();

            (pattern, lines)
        })
        .collect();

    let total_lines: usize = results.values().sum();

    // Output the results
    if patterns.len() > 1 {
        println!("Breakdown of Lines of Code by Glob:");
        results.iter().for_each(|(pattern, lines)| println!("  {}: {}", pattern, lines));
        println!();
    }

    println!("Total lines of code: {}", total_lines);
}
