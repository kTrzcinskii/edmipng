use anyhow::Result;
use args::EdmiArgs;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() -> Result<()> {
    let args = EdmiArgs::parse();

    match args.command {
        args::Command::Encode(encode_args) => commands::encode(encode_args),
        args::Command::Decode(decode_args) => commands::decode(decode_args),
        args::Command::Remove(remove_args) => commands::remove(remove_args),
        args::Command::Print(print_args) => commands::print(print_args),
    }
}
