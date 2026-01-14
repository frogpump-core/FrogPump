/// Default base URL for the FrogPump API.
pub const BASE_URL: &str = "https://api.frogpump.fun/v1";

/// Endpoint paths (appended to base URL).
pub const LAUNCH: &str = "/launch";
pub const LAUNCH_SELF_FUNDED: &str = "/launch/self-funded";
pub const TOKENS: &str = "/tokens";
pub const EARNINGS: &str = "/earnings";
pub const CLAIM: &str = "/claim";
pub const WALLET: &str = "/wallet";
pub const LEADERBOARD: &str = "/leaderboard";
pub const STATS: &str = "/stats";
pub const UPLOAD: &str = "/upload";

/// Build a full URL from a base, endpoint path, and optional query parameters.
///
/// Parameters are appended as `?key1=value1&key2=value2`.
pub fn build_url(base: &str, endpoint: &str, params: &[(&str, &str)]) -> String {
    let base = base.trim_end_matches('/');
    let mut url = format!("{}{}", base, endpoint);

    if !params.is_empty() {
        let query: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        url.push('?');
        url.push_str(&query.join("&"));
    }

