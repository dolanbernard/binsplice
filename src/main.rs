use clap::Parser;

mod args;

fn main() {
    let config = args::Args::parse();
    println!("{:?}", config);
}
