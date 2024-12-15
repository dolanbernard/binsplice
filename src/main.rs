use clap::Parser;

use args::Command;
use patcher::IpsPatcher;

mod args;
mod error;
mod patcher;
mod printer;

fn main() {
    let config = args::Args::parse();
    match config.command {
        Command::Dump {
            columns,
            column_len,
            from,
            to,
            no_decode,
            hide_ranges,
            input_filename
        } => {
            let data = std::fs::read(input_filename).unwrap();
            printer::print_data(&data, columns, column_len, from, to, no_decode, hide_ranges)
                .into_iter()
                .for_each(|line| println!("{line}"));
        },
        Command::Patch {
            input_filename,
            patch_filename,
            output_filename
        } => {
            let mut data = std::fs::read(input_filename).unwrap();
            let mut patch = std::fs::read(patch_filename).unwrap();
            let mut patcher = IpsPatcher::new(&mut patch, 0);
            println!("{} bytes patched", patcher.patch(&mut data).unwrap());
            std::fs::write(output_filename, data).unwrap();
        },
    }
}
