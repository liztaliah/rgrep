// rgrep
// Liz Hardee October 21 2024

// A simple copy of the grep command written in rust
use std::fs::read_to_string;
use std::io::{self};
use clap::Parser;
use regex::Regex;
use colored::Colorize;

#[derive(Parser)]

// struct for command line arguments
struct Cli {
    #[arg(default_value = "", short, long, help = "enter file to read from")]
    file: String,
    query: String,
}

fn main() {
    let cli = Cli::parse(); // parses command line arguments
    let re = Regex::new(&cli.query).unwrap(); // creates simple regex out of query
    if cli.file != "" {
        file_match(cli.file, re, cli.query);
    } else {
        stdin_match(cli.query, re);
    }
}

// match query from file
fn file_match(file: String, re: Regex, query: String) {
    for line in read_to_string(file).unwrap().lines() {
        if re.is_match(&line) {
            let output: Vec<&str> = line.split(&query).collect();
            println!("{}{}{}", output[0], query.red(), output[1]);
        }
    }
}

// match query fro standard in
fn stdin_match(query: String, re: Regex) {
    // this gets stdin from pipeline
    let lines = io::stdin().lines(); 
    for line in lines {
        if re.is_match(line.as_ref().unwrap().as_str()) {
            let output: Vec<&str> = line.as_ref().unwrap().as_str().split(&query).collect();
            println!("{}{}{}", output[0], query.red(), output[1]);
        }
    }
}