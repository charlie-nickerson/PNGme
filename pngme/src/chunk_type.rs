use std::str::FromStr;
use std::error::Error;
use std::fmt;
use std::fmt::Display;


#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    first_byte: u8,
    second_byte: u8,
    third_byte: u8,
    fourth_byte: u8,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;
    fn try_from(chunks: [u8; 4]) -> Result<Self, Self::Error> {

        Ok(ChunkType {
            first_byte: chunks[0],
            second_byte: chunks[1],
            third_byte: chunks[2],
            fourth_byte: chunks[3],
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChunkType {{ first byte: {}, second byte: {}, third byte: {}, fourth byte: {} }}", 
            self.first_byte, 
            self.second_byte, 
            self.third_byte, 
            self.fourth_byte)
    }
}

impl FromStr for ChunkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if the string is exactly 4 characters
        if s.len() != 4 {
            return Err(format!("Expected string of length 4, got {}", s.len()));
        }
        
        // Check if all characters are ASCII
        if !s.is_ascii() {
            return Err("String contains non-ASCII characters".to_string());
        }
        
        // Convert to bytes array
        let bytes: [u8; 4] = s.as_bytes()
            .try_into()
            .map_err(|_| "Failed to convert to fixed-size array".to_string())?;
            
        Ok(ChunkType { 
            first_byte: bytes[0],
            second_byte: bytes[1],
            third_byte: bytes[2],
            fourth_byte: bytes[3], 
        })
    }
}



impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        [self.first_byte, self.second_byte, self.third_byte, self.fourth_byte]
    }

    fn is_valid(&self) -> bool {
        todo!();
    }

    fn is_critical(&self) -> bool {
        todo!();
    }

    fn is_public(&self) -> bool {
        todo!();
    }

    fn is_reserved_bit_valid(&self) -> bool {
        todo!();
    }

    fn is_safe_to_copy(&self) -> bool {
        todo!();
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

