use std::fs::File;
use std::io::{self, Read};
use clap::Parser;
use std::path::Path;

#[derive(Parser)]

struct Cli {
    #[arg(default_value = "", short, long, help = "enter file to read from")]
    file: String,
    input: Vec<String>,
}

fn main() {
    let stdin = io::stdin();
    // this gets stdin from pipeline
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Error reading stdin");
        println!("{}", input);
        input.clear();
    }
    let cli = Cli::parse();

    if cli.file != "" {
        let path = Path::new(&cli.file);
        // let display = path.display();

        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
}