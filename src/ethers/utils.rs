use std::u8;

use ethers::abi::{self, Token};
use ethers::utils::{hex, keccak256};
pub struct AbiHelper {}

impl AbiHelper {
    pub fn encode(args: &[Token]) -> Vec<u8> {
        abi::encode(args)
    }
}

pub struct HexHelper {}

impl HexHelper {
    pub fn encode<T>(data: T) -> String
    where
        T: AsRef<[u8]>,
    {
        hex::encode(data)
    }
}

/// keccak256(abi.encode(args))
pub fn keccak256_abi_encode(args: &[Token]) -> [u8; 32] {
    let encoded = abi::encode(args);
    keccak256(encoded)
}

#[cfg(test)]
mod utils_test {
    use super::*;
    use ethers::abi::Token;
    use ethers::types::H256;

    #[test]
    fn test_keccak256_abi_encode() {
        let args = vec![Token::String("MARKET_LIST".to_owned())];
        let encoded = keccak256_abi_encode(&args);
        let encoded = H256::from(encoded);
        assert_eq!(
            "cdac201abd09598973b1365dbbaeb65ff0f370d30bb5c7440dc3341f570b2e38".to_string(),
            hex::encode(encoded.as_bytes())
        );
    }
}
