use std::fs::File;
use std::io::{self, Read};
use clap::Parser;
use std::path::Path;

#[derive(Parser)]

struct Cli {
    #[arg(default_value = "", short, long, help = "enter file to read from")]
    file: String,
}

fn main() {
    let lines = io::stdin().lines();
    // this gets stdin from pipeline
    for line in lines {
        println!("{}", line.unwrap());
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