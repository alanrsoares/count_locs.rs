use std::env;
use count_locs::{run, parse_command, print_error};

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_command(&args) {
        Ok(command) => run(command),
        Err(err_message) => {
            print_error(&err_message);
            std::process::exit(1);
        }
    }
}