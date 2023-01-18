// The first assignment is here. Make a few functions that would implement chunk types
use std::str::{FromStr};
use std::fmt::{Display, Formatter, Result as Res};

#[derive(PartialEq, Eq, Debug)]
//----------STRUCT DEFINITION----------
pub struct ChunkType {
    ancillary: u8,
    private: u8,
    reserved: u8,
    safe: u8,
}

// ----------Other useful functions here for abstraction----------

/// This function takes a byte and dissects it into a string of bits. Then, it finds the value of the nth bit
/// (starting at least significant bit as 0) through index.
fn bit_value(input: u8, index: u32) -> bool {
    input & (1 << index) != 0
}

fn valid_bytes(input: &[u8]) -> bool {
    for &i in input {
        if ((i >= 65) && (i <= 90)) || ((i >= 97) && (i <= 122)) {
            continue;
        } else {
            return false;
        }
    }
    true
}
//--------------------------------------------------

// ----------Standard implicated functions----------
impl ChunkType {
    // For ease of use
    pub fn bytes(&self) -> [u8; 4] {
        [self.ancillary, self.private, self.reserved, self.safe]
    }

    pub fn is_valid(&self) -> bool {
        for i in self.bytes() {
            if ((i >= 65) && (i <= 90)) || ((i >= 97) && (i <= 122)) {
                continue;
            } else {
                return false;
            }
        }
        true
    }

    pub fn is_critical(&self) -> bool {
        // Check the 5th significant bit for the ancillary byte
        if bit_value(self.ancillary, 5) {
            // If the 5th bit is true/1, then it is ancillary and not critical
            false
        } else {
            true
        }
    }

    pub fn is_public(&self) -> bool {
        if bit_value(self.private, 5) {
            // If the 5th bit is true, then it is private. Thus, it is not public.
            false
        } else {
            true
        }
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        if bit_value(self.reserved, 5) {
            // If the 5th bit is 1, then it represents a lowercase. Convention dictates that it should be uppercase
            false
        } else {
            true
        }
    }

    pub fn is_safe_to_copy(&self) -> bool {
        if bit_value(self.safe, 5) {
            // If the 5th bit is 1, then it is safe to copy
            true
        } else {
            false
        }
    }

    /// Writes the information to a string
    pub fn to_string(&self) -> String {
        self.bytes()
            .into_iter()
            .map(|byte| byte as char)
            .collect::<String>()
    }
}
//---------------------------------------------------------------------

//----------Implementing some traits for the created struct.----------
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if valid_bytes(&value) {
            Ok(
                ChunkType {
                    ancillary: value[0],
                    private: value[1],
                    reserved: value[2],
                    safe: value[3],
                }
            )
        } else {
            Err("Failed to resolve input.")
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let byte_array = s.as_bytes();
        if valid_bytes(byte_array) {
            Ok(
                ChunkType {
                    ancillary: byte_array[0],
                    private: byte_array[1],
                    reserved: byte_array[2],
                    safe: byte_array[3],
                }
            )
        } else {
            Err("Failed to resolve input.")
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Res {
        write!(f, "{}{}{}{}", self.ancillary, self.private, self.reserved, self.safe)
    }
}









//----------TESTS HERE----------
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
        assert!(chunk.is_valid());

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