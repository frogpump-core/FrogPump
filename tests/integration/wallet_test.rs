#[cfg(test)]
mod tests {
    use frogpump::utils::crypto;

    #[test]
    fn test_valid_solana_address() {
        let addr = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
        assert!(crypto::is_valid_solana_address(addr));
    }

    #[test]
    fn test_invalid_solana_address_bad_chars() {
        let addr = "0OIl_invalid_address!!!";
        assert!(!crypto::is_valid_solana_address(addr));
    }

    #[test]
    fn test_invalid_solana_address_too_short() {
        let addr = "abc";
        assert!(!crypto::is_valid_solana_address(addr));
    }

    #[test]
    fn test_valid_base58() {
        assert!(crypto::is_valid_base58("123ABCabc"));
        assert!(!crypto::is_valid_base58("0OIl"));
        assert!(!crypto::is_valid_base58(""));
    }

    #[test]
    fn test_base58_encode_decode_roundtrip() {
        let original = b"hello solana";
        let encoded = crypto::encode_base58(original);
        let decoded = crypto::decode_base58(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_decode_base58_invalid() {
        let result = crypto::decode_base58("0OOO_invalid");
        assert!(result.is_err());
    }
}
