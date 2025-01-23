use globwalk;
use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
/// Filter out empty lines and lines that contain only whitespace
fn is_valid_line(line: Result<String, std::io::Error>) -> Option<String> {
    line.ok().filter(|l| !l.trim().is_empty())
}

/// Count the number of lines in a file
fn count_lines(file_path: &Path) -> usize {
    match File::open(file_path) {
        Ok(file) => BufReader::new(file)
            .lines()
            .filter_map(is_valid_line)
            .count(),
        Err(_) => 0,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <directory> <glob-patterns>...", args[0]);
        std::process::exit(1);
    }

    let root = std::fs::canonicalize(&args[1]).expect("Failed to resolve directory");
    let patterns = &args[2..];

    let mut results = HashMap::new();

    for pattern in patterns {
        let walker = globwalk::GlobWalkerBuilder::from_patterns(&root, &[pattern])
            .build()
            .expect("Failed to build glob walker");

        let entries: Vec<_> = walker.into_iter().filter_map(Result::ok).collect();

        let lines: usize = entries
            .par_iter() // Parallelize over the collected entries
            .map(|entry| count_lines(entry.path()))
            .sum();

        results.insert(pattern.as_str(), lines);
    }

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
