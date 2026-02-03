#[cfg(test)]
mod tests {
    use frogpump::models::{LaunchType, Token};

    fn build_token(name: &str, symbol: &str, launch_type: LaunchType) -> Token {
        Token {
            id: "tok_test".to_string(),
            mint_address: "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU".to_string(),
            name: name.to_string(),
            symbol: symbol.to_string(),
            description: None,
            image_url: None,
            agent_id: "agent_test".to_string(),
            launch_type,
            verified: false,
            created_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_token_name_display() {
        let token = build_token("TestToken", "TST", LaunchType::Gasless);
        assert!(token.to_string().contains("TestToken"));
        assert!(token.to_string().contains("TST"));
    }

    #[test]
    fn test_token_symbol_in_display() {
        let token = build_token("Alpha", "ALPHA", LaunchType::SelfFunded);
        let display = token.to_string();
        assert!(display.contains("ALPHA"));
        assert!(display.contains("Self-Funded"));
    }

