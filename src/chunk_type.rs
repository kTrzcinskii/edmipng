use std::{fmt::Display, str::FromStr};

use anyhow::{bail, Error};

// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, PartialEq, Eq)]
struct ChunkType {
    ancillary_byte: u8,
    private_byte: u8,
    reserved_byte: u8,
    safe_to_copy_byte: u8,
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        [
            self.ancillary_byte,
            self.private_byte,
            self.reserved_byte,
            self.safe_to_copy_byte,
        ]
    }

    fn is_valid(&self) -> bool {
        self.bytes().iter().all(|byte| byte.is_ascii_alphabetic()) && self.is_reserved_bit_valid()
    }

    fn is_critical(&self) -> bool {
        self.ancillary_byte.is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        self.private_byte.is_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.reserved_byte.is_ascii_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        self.safe_to_copy_byte.is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().any(|byte| !byte.is_ascii_alphabetic()) {
            bail!("All bytes must be valid ascii letters.");
        }
        let ancillary_byte = value[0];
        let private_byte = value[1];
        let reserved_byte = value[2];
        let safe_to_copy_byte = value[3];

        Ok(ChunkType {
            ancillary_byte,
            private_byte,
            reserved_byte,
            safe_to_copy_byte,
        })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() == 4 {
            let fixed_size_bytes: [u8; 4] = bytes.try_into()?;
            ChunkType::try_from(fixed_size_bytes)
        } else {
            bail!("Chunk type must always have 4 ascii letters");
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes_string = String::from_utf8(self.bytes().into()).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", bytes_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
