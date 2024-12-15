use clap::Parser;

mod args;
mod error;
mod patcher;
mod printer;

fn main() {
    let config = args::Args::parse();
    let data = std::fs::read(&config.input_filename).unwrap();
    printer::print_data(&data, &config);
}
