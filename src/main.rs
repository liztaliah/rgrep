use std::fs::File;
use std::io::Read;
use clap::Parser;
use std::env;
use std::path::Path;

#[derive(Parser)]

struct Cli {
    #[arg(default_value = "", short, long, help = "enter file to read from")]
    file: String,
    input: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    // let args: Vec<String> = env::args().skip(1).collect();

    if cli.file != "" {
        let path = Path::new(&cli.file);
        // let display = path.display();

        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
}
