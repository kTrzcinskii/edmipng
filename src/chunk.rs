use std::fmt::Display;

use crate::chunk_type::ChunkType;
use anyhow::{bail, Context, Error, Result};
use crc::{Crc, CRC_32_ISO_HDLC};

const LENGTH_FIELD_LEN: usize = 4;
const CHUNK_TYPE_FIELD_LEN: usize = 4;
const CRC_FIELD_LEN: usize = 4;

// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
pub struct Chunk {
    data: Vec<u8>,
    chunk_type: ChunkType,
    length: u32,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;
        let crc = Self::calculate_crc(&chunk_type, &data);
        Chunk {
            data,
            chunk_type,
            length,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        String::from_utf8(self.data.clone()).context("Data is not valid utf-8 string")
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let length =
            LENGTH_FIELD_LEN + CHUNK_TYPE_FIELD_LEN + self.length() as usize + CRC_FIELD_LEN;

        let mut bytes: Vec<u8> = Vec::with_capacity(length);
        bytes.extend(&self.length().to_be_bytes());
        bytes.extend(&self.chunk_type().bytes());
        bytes.extend(self.data());
        bytes.extend(&self.crc().to_be_bytes());
        bytes
    }

    fn calculate_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let crc_instance = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .copied()
            .chain(data.iter().copied())
            .collect();
        crc_instance.checksum(&bytes)
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut iter = value.iter();
        // Length
        let length_buffer: Vec<u8> = iter.by_ref().take(LENGTH_FIELD_LEN).cloned().collect();
        if length_buffer.len() < LENGTH_FIELD_LEN {
            bail!("Length should be exactly {} bytes", LENGTH_FIELD_LEN);
        }
        let length = u32::from_be_bytes(length_buffer[0..LENGTH_FIELD_LEN].try_into()?);

        // Chunk type
        let chunk_type_buffer: Vec<u8> =
            iter.by_ref().take(CHUNK_TYPE_FIELD_LEN).cloned().collect();
        if chunk_type_buffer.len() < CHUNK_TYPE_FIELD_LEN {
            bail!(
                "Chunk type should be exactly {} bytes",
                CHUNK_TYPE_FIELD_LEN
            );
        }
        let chunk_type_array: [u8; 4] = chunk_type_buffer[0..CHUNK_TYPE_FIELD_LEN].try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_array)?;

        // Data
        let data: Vec<u8> = iter.by_ref().take(length as usize).cloned().collect();
        if data.len() < length as usize {
            bail!("Data should be exactly {} bytes", length);
        }

        // Crc
        let crc_buffer: Vec<u8> = iter.by_ref().take(CRC_FIELD_LEN).cloned().collect();
        if crc_buffer.len() < CRC_FIELD_LEN {
            bail!("Crc should be exactly {} bytes", CRC_FIELD_LEN);
        }
        let crc = u32::from_be_bytes(crc_buffer[0..CRC_FIELD_LEN].try_into()?);

        // Check if crc is valid
        let valid_crc = Chunk::calculate_crc(&chunk_type, &data);
        if crc != valid_crc {
            bail!("Invalid crc value.");
        }

        Ok(Chunk {
            data,
            chunk_type,
            length,
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data_as_string().map_err(|_| std::fmt::Error)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
