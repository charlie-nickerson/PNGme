use std::str::FromStr;
use std::error::Error;
use std::fmt;


#[derive(PartialEq, Eq, Debug)]
struct ChunkType {
    a_bit: u8,
    p_bit: u8,
    r_bit: u8,
    s_bit: u8,
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Method 1: Display the struct fields as bits
        // write!(f, "ChunkType {{ a_bit: {}, p_bit: {}, r_bit: {}, s_bit: {} }}",
        //        self.a_bit, self.p_bit, self.r_bit, self.s_bit)
        
        // Method 2: If these bits represent ASCII characters or 
        // need to be combined into a specific representation
        // For example, creating a 4-character string
        
        // This is just an example - adjust based on how your bits should be displayed
        // Let's assume we want to construct 4 bytes where each of our bits is in the 5th position
        let byte1 = (self.a_bit << 5) | 0b01000001; // 'A' with 5th bit from a_bit
        let byte2 = (self.p_bit << 5) | 0b01000010; // 'B' with 5th bit from p_bit
        let byte3 = (self.r_bit << 5) | 0b01000011; // 'C' with 5th bit from r_bit
        let byte4 = (self.s_bit << 5) | 0b01000100; // 'D' with 5th bit from s_bit
        
        let bytes = [byte1, byte2, byte3, byte4];
        
        // Convert to a string (assuming valid ASCII)
        let s = String::from_utf8_lossy(&bytes);
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
struct ChunkTypeError {
    details: String,
}

impl ChunkTypeError {
    fn new(msg: &str) -> ChunkTypeError {
        ChunkTypeError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ChunkTypeError {}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        
        let a_bit = (bytes[0] >> 5) & 1;
        let p_bit = (bytes[1] >> 5) & 1;
        let r_bit = (bytes[2] >> 5) & 1;
        let s_bit = (bytes[3] >> 5) & 1;

        Ok(ChunkType {
            a_bit,
            p_bit,
            r_bit,
            s_bit,
        })
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if the string has exactly 4 bytes (4 ASCII characters)
        if s.len() != 4 {
            return Err(ChunkTypeError::new("ChunkType must be 4 characters long"));
        }

        // Check if all characters are ASCII
        if !s.chars().all(|c| c.is_ascii()) {
            return Err(ChunkTypeError::new("All characters must be ASCII"));
        }
        
        // Convert the string to bytes
        let bytes: [u8; 4] = s.as_bytes().try_into()
            .map_err(|_| ChunkTypeError::new("Failed to convert string to 4 bytes"))?;
        
        // Extract the relevant bits based on your requirements
        // For this example, we'll extract the 5th bit as in the previous example
        let a_bit = (bytes[0] >> 5) & 1;
        let p_bit = (bytes[1] >> 5) & 1;
        let r_bit = (bytes[2] >> 5) & 1;
        let s_bit = (bytes[3] >> 5) & 1;
        
        // You can add additional validation here
        // For example, checking if the r_bit has a specific value
        if r_bit != 0 {
            return Err(ChunkTypeError::new("r_bit must be 0"));
        }
        
        Ok(ChunkType {
            a_bit,
            p_bit,
            r_bit,
            s_bit,
        })
    }
}



impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        [self.a_bit, self.p_bit, self.r_bit, self.s_bit]
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









fn main() {
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
}
