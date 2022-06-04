use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(name = "Git Cloner")]
#[clap(author = "Craig R Broughton <crbroughton@posteo.uk>")]
#[clap(version = "0.0.1")]
#[clap(about = "Clones git repositories", long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}