use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None )]

struct Args {
    #[arg(short, long)]
    query: String,

    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.query);
    println!("Hello {}!", args.file_path);

    let file = args.file_path;
    println!("{}", file);

    let contents = fs::read_to_string(file).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
