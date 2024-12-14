use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    //#[command(subcommand)]
    //pub command: Command,
    #[arg(short='c', long="cols", required = false, default_value_t=2)]
    pub columns: usize,
    #[arg(short='l', long="group-len", required = false, default_value_t=8)]
    pub group_len: usize,
    #[arg(short='f', long="from", required = false, default_value=None)]
    pub from: Option<usize>,
    #[arg(short='t', long="to", required = false, default_value=None)]
    pub to: Option<usize>,
    //#[arg(short='d', action = clap::ArgAction::SetFalse, default_value_t = true)]
    //pub decode: bool,
    #[arg(short='n', long="no-decode", required=false)]
    pub no_decode: bool,
    #[arg(short='i', long="input", required=true)]
    pub input_filename: String,
    #[arg(short='o', long="output", required=false, default_value = None)]
    pub output_filename: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Dump,
    Patch {
        start_index: usize,
    },
}

enum _DecodeStrategy {
    Ascii,
    Asmx86,
}
