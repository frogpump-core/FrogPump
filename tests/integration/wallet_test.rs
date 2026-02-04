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

