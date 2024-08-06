use anyhow::{Context, Ok, Result};
use bit_vec::BitVec;

#[derive(Debug, PartialEq)]
pub struct Header {
    pub version: u8,
    pub bin_length: u64,
    pub hash_count: u8,
}

const HEADER_ENCODE_SIZE: usize = 10;

pub fn serialize(h: Header) -> BitVec {
    let mut bytes = Vec::new();
    bytes.extend(h.version.to_be_bytes());
    bytes.extend(h.bin_length.to_be_bytes());
    println!("{:?}", h.bin_length.to_be_bytes());
    bytes.extend(h.hash_count.to_be_bytes());
    BitVec::from_bytes(&bytes)
}

pub fn deserialize(bytes: [u8; HEADER_ENCODE_SIZE]) -> Result<Header> {
    let version = bytes[0];
    let bin_length = u64::from_be_bytes(
        bytes[1..9]
            .try_into()
            .context("couldnt find bytelength for bloomfilter")?,
    );
    let hash_count = bytes[9];

    Ok(Header {
        version,
        bin_length,
        hash_count,
    })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_serialize() {
        let encoded = serialize(Header {
            version: 1,
            hash_count: 4,
            bin_length: 99999,
        });
        let bytes: [u8; 10] = [1, 0, 0, 0, 0, 0, 1, 134, 159, 4];
        let result = BitVec::from_bytes(&bytes);
        assert_eq!(encoded, result);
    }

    #[test]
    fn test_deserialize() {
        let bytes: [u8; 10] = [1, 0, 0, 0, 0, 0, 1, 134, 159, 4];
        let result = Header {
            version: 1,
            hash_count: 4,
            bin_length: 99999,
        };
        assert_eq!(deserialize(bytes).unwrap(), result);
    }
}
