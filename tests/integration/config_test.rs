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

