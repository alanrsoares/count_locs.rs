use globwalk;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Filter out empty lines and lines that contain only whitespace
pub fn is_valid_line(line: Result<String, std::io::Error>) -> Option<String> {
    line.ok().filter(|l| !l.trim().is_empty())
}

/// Count the number of lines in a file
pub fn count_lines(file_path: &Path) -> usize {
    match File::open(file_path) {
        Ok(file) => BufReader::new(file)
            .lines()
            .filter_map(is_valid_line)
            .count(),
        Err(_) => 0,
    }
}

/// Count lines of code recursively for multiple glob patterns
pub fn count_locs(
    root: &Path,
    patterns: &[String],
) -> HashMap<String, usize> {
    let mut results = HashMap::new();

    for pattern in patterns {
        let walker = globwalk::GlobWalkerBuilder::from_patterns(root, &[pattern])
            .build()
            .expect("Failed to build glob walker");

        let entries: Vec<_> = walker.into_iter().filter_map(Result::ok).collect();

        let lines: usize = entries
            .par_iter() // Parallelize over the collected entries
            .map(|entry| count_lines(entry.path()))
            .sum();

        results.insert(pattern.clone(), lines);
    }

    results
}

pub const HELP_MESSAGE: &str = "Usage: count_locs <directory> <glob-patterns>...\n\n\
Options:\n\
-h, --help       Show this help message\n\
-v, --version    Show version information\n\n\
Examples:\n\
count_locs ./src \"**/*.rs\" \"**/*.ts\"\n\
count_locs ./ \"**/*.css\"";


/// Print program help
pub fn print_help() {
    println!(
        "{}",
        HELP_MESSAGE
    );
}

pub fn print_version() {
    println!("count_locs version {}", env!("CARGO_PKG_VERSION"));
}

pub fn print_error(message: &str) {
    eprintln!("{}", message);
}