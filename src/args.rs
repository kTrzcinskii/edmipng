use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// EDMIPNG - Encode and Decode Messages In PNG
#[derive(Debug, Parser)]
#[clap(name = "edmipng")]
pub struct EdmiArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Encode new message into png chunk
    Encode(EncodeArgs),
    /// Decode message from png chunk
    Decode(DecodeArgs),
    /// Remove chunk with message from png
    Remove(RemoveArgs),
    /// Print all chunks with encoded messages
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    /// Path to local png file / url to external png file
    pub source: String,
    /// Chunk type of the chunk to be created in which message will be encoded
    pub chunk_type: String,
    /// Message to be encoded inside the chunk
    pub message: String,
    /// Path to output file (if not provided, depending on the source of the file, changes are made to the source file or new file is automatically created)
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    /// Path to local png file / url to external png file
    pub source: String,
    /// Chunk type of the chunk containg message to decode
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Path to local png file / url to external png file
    pub source: String,
    /// Chunk type of the chunk to be removed
    pub chunk_type: String,
    /// Path to output file (if not provided, depending on the source of the file, changes are made to the source file or new file is automatically created)
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    /// Path to local png file / url to external png file
    pub source: String,
}
