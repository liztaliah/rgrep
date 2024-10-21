use std::fs::read_to_string;
use std::io::{self};
use clap::Parser;
use regex::Regex;
use colored::Colorize;

#[derive(Parser)]

struct Cli {
    #[arg(default_value = "", short, long, help = "enter file to read from")]
    file: String,
    query: String,
}

fn main() {
    let cli = Cli::parse();
    let re = Regex::new(&cli.query).unwrap();
    let lines = io::stdin().lines();
    if cli.file != "" {
        for line in read_to_string(cli.file).unwrap().lines() {
            if re.is_match(&line) {
                println!("{}", line);
            }
        }
    } else {
        // this gets stdin from pipeline
        for line in lines {
            if re.is_match(line.as_ref().unwrap().as_str()) {
                // what the fuck do i do here
                let output: Vec<&str> = line.unwrap().as_str().split(&cli.query).collect();
                println!("{}", output.join(" "));
                // println!("{}", line.unwrap());
            }
        }
    }
}