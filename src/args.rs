use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
    
    //#[arg(short='d', action = clap::ArgAction::SetFalse, default_value_t = true)]
    //pub decode: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Dump {
        #[arg(short='c', long="cols", required = false, default_value_t=2)]
        columns: usize,
        #[arg(short='l', long="col-len", required = false, default_value_t=8)]
        column_len: usize,
        #[arg(short='f', long="from", required = false, default_value=None)]
        from: Option<usize>,
        #[arg(short='t', long="to", required = false, default_value=None)]
        to: Option<usize>,
        #[arg(short='n', long="no-decode", required=false)]
        no_decode: bool,
        #[arg(short='g', long="hide-ranges", required=false)]
        hide_ranges: bool,
        #[arg(short='i', long="input", required=true)]
        input_filename: String,
        #[arg(short='o', long="output", required=false, default_value=None)]
        output_filename: Option<String>,
    },
    Patch {
        #[arg(short='i', long="input", required=true)]
        input_filename: String,
        #[arg(short='p', long="patch", required=true)]
        patch_filename: String,
        #[arg(short='o', long="output", required=true)]
        output_filename: String,
    },
}

enum _DecodeStrategy {
    Ascii,
    Asmx86,
}
