use anyhow::Result;
use args::Args;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod png;

fn main() -> Result<()> {
    let args = Args::parse();
    Ok(())
}
