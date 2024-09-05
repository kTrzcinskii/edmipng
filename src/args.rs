use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// EDMIPNG - Encode and Decode Messages In PNG
#[derive(Debug, Parser)]
#[clap(name = "edmipng")]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Encode new message into png chunk
    Encode {
        /// Path to png file
        path: PathBuf,
        /// Chunk type of the chunk to be created in which message will be encoded
        chunk_type: String,
        /// Message to be encoded inside the chunk
        message: String,
        /// Path to output file (if not provided, changes are made to the source file)
        output_file: Option<PathBuf>,
    },
    /// Decode message from png chunk
    Decode {
        /// Path to png file
        path: PathBuf,
        /// Chunk type of the chunk containg message to decode
        chunk_type: String,
    },
    /// Remove chunk with message from png
    Remove {
        /// Path to png file
        path: PathBuf,
        /// Chunk type of the chunk to be removed
        chunk_type: String,
    },
    /// Print all chunks with encoded messages
    Print {
        /// Path to png file
        path: PathBuf,
    },
}
