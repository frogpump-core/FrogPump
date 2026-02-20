use frogpump::config::{Network, Settings};
use frogpump::models::{Agent, Earning, LaunchType, Token};

pub fn mock_token() -> Token {
    Token {
        id: "tok_001".to_string(),
        mint_address: "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU".to_string(),
        name: "FrogCoin".to_string(),
        symbol: "FROG".to_string(),
        description: Some("A test frog token".to_string()),
        image_url: Some("https://example.com/frog.png".to_string()),
        agent_id: "agent_001".to_string(),
        launch_type: LaunchType::Gasless,
        verified: true,
        created_at: "2026-01-15T10:30:00Z".to_string(),
    }
}

pub fn mock_agent() -> Agent {
    Agent {
        id: "ag_001".to_string(),
        agent_id: "agent_001".to_string(),
        wallet_address: Some("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string()),
        created_at: "2026-01-10T08:00:00Z".to_string(),
    }
}

pub fn mock_earning() -> Earning {
    Earning {
        id: "earn_001".to_string(),
        token_id: "tok_001".to_string(),
        agent_id: "agent_001".to_string(),
        amount: 1.2345,
        claimed: false,
        claimed_at: None,
        created_at: "2026-01-20T12:00:00Z".to_string(),
    }
}

pub fn mock_settings() -> Settings {
    Settings {
        api_base_url: "https://api.frogpump.com".to_string(),
        agent_id: Some("agent_001".to_string()),
        wallet_address: Some("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string()),
        network: Network::Devnet,
        rpc_url: "https://api.devnet.solana.com".to_string(),
        verbose: false,
    }
}

// iteration 40
