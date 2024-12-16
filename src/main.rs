use std::fs::File;
use std::io::Write;

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
            input_filename,
            output_filename,
        } => {
            let data = std::fs::read(input_filename).unwrap();
            let mut writer: Box<dyn Write> = if let Some(output_filename) = output_filename {
                Box::new(File::create(output_filename).unwrap())
            }
            else {
                Box::new(std::io::stdout())
            };
            printer::print_data(&data, columns, column_len, from, to, no_decode, hide_ranges, &mut writer);
        },
        Command::Patch {
            input_filename,
            patch_filename,
            output_filename,
        } => {
            let mut data = std::fs::read(input_filename).unwrap();
            let mut patch = std::fs::read(patch_filename).unwrap();
            let mut patcher = IpsPatcher::new(&mut patch, 0);
            patcher.patch(&mut data).unwrap();
            std::fs::write(output_filename, data).unwrap();
        },
    }
}
