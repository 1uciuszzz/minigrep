use colored::Colorize;
use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(error) => {
            println!("{} {}", "Error:".red().bold(), error.red().bold());
            process::exit(0);
        }
    };

    minigrep::run(config);
}
