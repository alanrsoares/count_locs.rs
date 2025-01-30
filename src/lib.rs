use globwalk;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

/// Enum representing the available commands
pub enum Command {
    Help,
    Version,
    Count { dir: String, patterns: Vec<String> },
}

/// Handle a command and dispatch the appropriate action
pub fn run(command: Command) {
    match command {
        Command::Help => print_help(),
        Command::Version => print_version(),
        Command::Count { dir, patterns } => process_input(&dir, &patterns),
    }
}

/// Parse command-line arguments into a Command
pub fn parse_command(args: &[String]) -> Result<Command, String> {
    match args {
        // Help command
        [_, flag] if flag == "--help" || flag == "-h" => Ok(Command::Help),

        // Version command
        [_, flag] if flag == "--version" || flag == "-v" => Ok(Command::Version),

        // Insufficient arguments
        [_] => Err(String::from("Usage: count_locs <directory> <glob-patterns>...")),

        // Count command
        [_, dir, patterns @ ..] => Ok(Command::Count {
            dir: dir.to_string(),
            patterns: patterns.iter().map(String::from).collect(),
        }),

        // Fallback
        _ => Err(String::from("Invalid arguments. Use --help for usage information.")),
    }
}

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
    println!("{}", HELP_MESSAGE);
}

/// Print program version
pub fn print_version() {
    println!("count_locs version {}", env!("CARGO_PKG_VERSION"));
}

/// Print an error message to stderr
pub fn print_error(message: &str) {
    eprintln!("{}", message);
}

/// Process input: count lines of code and print results
pub fn process_input(dir: &str, patterns: &[String]) {
    // Start measuring time
    let start_time = Instant::now();

    let root = std::fs::canonicalize(dir).expect("Failed to resolve directory");
    let patterns_vec: Vec<String> = patterns.iter().map(String::from).collect();
    let results = count_locs(&root, &patterns_vec);

    let total_lines: usize = results.values().copied().sum();

    if patterns_vec.len() > 1 {
        println!("Breakdown of Lines of Code by Glob:\n");
        for (pattern, &lines) in &results {
            println!("  {}: {}", pattern, lines);
        }
        println!();
    }

    println!(
        "Total:\t{} lines of code\n\n{:.2?}", 
        total_lines, 
        start_time.elapsed()
    );
}
