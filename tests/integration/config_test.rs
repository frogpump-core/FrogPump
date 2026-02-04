#[cfg(test)]
mod tests {
    use frogpump::config::{Network, Settings};

    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.api_base_url, "https://api.frogpump.com");
        assert!(settings.agent_id.is_none());
        assert!(settings.wallet_address.is_none());
        assert!(!settings.verbose);
    }

    #[test]
    fn test_settings_set_get_value() {
        let mut settings = Settings::default();
        settings.set_value("agent_id", "agent_xyz").unwrap();
        assert_eq!(settings.get_value("agent_id"), Some("agent_xyz".to_string()));

        settings.set_value("verbose", "true").unwrap();
        assert_eq!(settings.get_value("verbose"), Some("true".to_string()));
    }

    #[test]
    fn test_settings_set_invalid_key() {
        let mut settings = Settings::default();
        let result = settings.set_value("nonexistent_key", "value");
        assert!(result.is_err());
    }

    #[test]
    fn test_settings_get_unknown_key() {
        let settings = Settings::default();
        assert!(settings.get_value("unknown").is_none());
    }

    #[test]
    fn test_network_config_urls() {
        let mainnet = Network::Mainnet.config();
        assert_eq!(mainnet.name, "mainnet-beta");
        assert!(mainnet.rpc_url.contains("mainnet"));

        let devnet = Network::Devnet.config();
        assert_eq!(devnet.name, "devnet");
        assert!(devnet.rpc_url.contains("devnet"));

        let localnet = Network::Localnet.config();
        assert_eq!(localnet.name, "localnet");
        assert!(localnet.rpc_url.contains("localhost"));
    }

    #[test]
    fn test_network_default_rpc() {
        assert!(Network::Mainnet.default_rpc().contains("mainnet"));
        assert!(Network::Devnet.default_rpc().contains("devnet"));
        assert!(Network::Localnet.default_rpc().contains("localhost"));
    }
}
