mod error;
mod log_analyzer;

use crate::log_analyzer::read_file_and_print_table;
use std::env;
use std::fs::File;

fn cli_usage_and_exit(name: &str, error: &str) {
    let split = name.split('/').collect::<Vec<&str>>();
    let short_name = split.last().unwrap();

    eprintln!(
        "Error: {}.\nUsage: {n} filename\nExample: {n} log.txt",
        error,
        n = short_name
    );
    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Welcome to the Tiny Log Digits Reader CLI !");
    if args.len() != 2 {
        cli_usage_and_exit(&args[0], "One argument is required. No more, no less.")
    }
    let filename = args[1].to_lowercase();

    match File::open(&filename) {
        Err(e) => {
            let err_msg = format!("Could not open file. Reason: {}", e);
            cli_usage_and_exit(&args[0], &err_msg);
        }
        Ok(file) => {
            if let Err(e) = read_file_and_print_table(file) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
