use std::{fs, str::FromStr};

use anyhow::{Context, Result};

use crate::{
    args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
    source::Source,
};

pub fn encode(arguments: EncodeArgs) -> Result<()> {
    let source = Source::from_str(&arguments.source)?;
    let mut png = Png::try_from(&source)?;

    let chunk_type =
        ChunkType::from_str(&arguments.chunk_type).context("Coulnd't parse chunk type.")?;
    let chunk = Chunk::new(chunk_type, arguments.message.into());
    png.append_chunk(chunk);

    let output_path = arguments
        .output_file
        .unwrap_or(source.get_output_file_path().clone());
    fs::write(output_path, png.as_bytes()).context("Couldn't write to png file.")?;

    Ok(())
}

pub fn decode(arguments: DecodeArgs) -> Result<()> {
    let source = Source::from_str(&arguments.source)?;
    let png = Png::try_from(&source)?;

    let chunk = png.chunk_by_type(&arguments.chunk_type);

    match chunk {
        Some(chunk) => {
            println!("{}", chunk);
        }
        None => {
            println!(
                "Chunk with given type ({}) doesn't exist",
                arguments.chunk_type
            );
        }
    }

    Ok(())
}

pub fn remove(arguments: RemoveArgs) -> Result<()> {
    let source = Source::from_str(&arguments.source)?;
    let mut png = Png::try_from(&source)?;

    png.remove_first_chunk(&arguments.chunk_type)
        .context("Couldn't remove chunk")?;

    let output_path = arguments
        .output_file
        .unwrap_or(source.get_output_file_path().clone());
    fs::write(output_path, png.as_bytes()).context("Couldn't write to png file")?;

    Ok(())
}

pub fn print(arguments: PrintArgs) -> Result<()> {
    let source = Source::from_str(&arguments.source)?;
    let png = Png::try_from(&source)?;
    println!("Special chunk types inside file (private + ancillary):");
    println!("{}", png);
    Ok(())
}
