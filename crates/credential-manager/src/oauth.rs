use crate::error::{CredentialError, Result};
use crate::config::OAuthConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use url::Url;

/// OAuth token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub raw: HashMap<String, serde_json::Value>,
}

/// A stored OAuth token with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub provider: String,
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub scope: Option<String>,
    pub expires_at: u64,
    pub created_at: u64,
    pub last_refreshed: u64,
    pub id_token: Option<String>,
}

/// OAuth authorization request state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub state: String,
    pub code_verifier: String,
    pub provider: String,
    pub redirect_uri: Url,
    pub created_at: u64,
}

/// The OAuth2 flow manager
pub struct OAuthManager {
    config: OAuthConfig,
    http_client: reqwest::Client,
    pending_requests: HashMap<String, AuthRequest>,
}

impl OAuthManager {
    /// Create a new OAuth manager
    pub fn new(config: OAuthConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(config.client_timeout)
            .build()
            .unwrap_or_default();

        Self {
            config,
            http_client,
            pending_requests: HashMap::new(),
        }
    }

    /// Generate the authorization URL for a provider
    pub fn authorization_url(&mut self, provider_name: &str, state: &str) -> Result<Url> {
        let provider = self.config.get_oauth_provider(provider_name)
            .ok_or_else(|| CredentialError::NotFound(format!("OAuth provider '{}' not found", provider_name)))?;

        let code_verifier = self.generate_code_verifier();
        let code_challenge = self.generate_code_challenge(&code_verifier);

        let mut url = provider.auth_url.clone();
        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &provider.client_id)
            .append_pair("redirect_uri", self.config.redirect_uri.as_str())
            .append_pair("state", state)
            .append_pair("code_challenge_method", "S256")
            .append_pair("code_challenge", &code_challenge);

        if !provider.scopes.is_empty() {
            url.query_pairs_mut()
                .append_pair("scope", &provider.scopes.join(" "));
        }

        // Store the pending request
        let request = AuthRequest {
            state: state.to_string(),
            code_verifier,
            provider: provider_name.to_string(),
            redirect_uri: self.config.redirect_uri.clone(),
            created_at: current_timestamp(),
        };
        self.pending_requests.insert(state.to_string(), request);

        Ok(url)
    }

    /// Exchange an authorization code for tokens
    pub async fn exchange_code(&mut self, provider_name: &str, code: &str, state: &str) -> Result<StoredToken> {
        let request = self.pending_requests.remove(state)
            .ok_or_else(|| CredentialError::OAuth("No pending auth request for this state".to_string()))?;

        let provider = self.config.get_oauth_provider(provider_name)
            .ok_or_else(|| CredentialError::NotFound(format!("OAuth provider '{}' not found", provider_name)))?;

        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code".to_string());
        params.insert("code", code.to_string());
        params.insert("redirect_uri", request.redirect_uri.to_string());
        params.insert("client_id", provider.client_id.clone());
        params.insert("client_secret", provider.client_secret.clone());
        params.insert("code_verifier", request.code_verifier.clone());

        let response = self.http_client
            .post(provider.token_url.clone())
            .form(&params)
            .send()
            .await
            .map_err(|e| CredentialError::OAuth(format!("Token request failed: {}", e)))?;

        let status = response.status();
        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CredentialError::OAuth(format!("Failed to parse token response: {}", e)))?;

        if !status.is_success() {
            return Err(CredentialError::OAuth(format!(
                "Token request failed with status {}: {}",
                status, body
            )));
        }

        let token_response: TokenResponse = serde_json::from_value(body.clone())
            .map_err(|e| CredentialError::OAuth(format!("Invalid token response: {}", e)))?;

        let now = current_timestamp();
        let stored = StoredToken {
            provider: provider_name.to_string(),
            user_id: "unknown".to_string(), // Will be updated after userinfo
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            token_type: token_response.token_type,
            scope: token_response.scope,
            expires_at: now + token_response.expires_in,
            created_at: now,
            last_refreshed: now,
            id_token: token_response.id_token,
        };

        Ok(stored)
    }

    /// Refresh an access token
    pub async fn refresh_token(&self, stored: &StoredToken) -> Result<StoredToken> {
        let provider = self.config.get_oauth_provider(&stored.provider)
            .ok_or_else(|| CredentialError::NotFound(format!("OAuth provider '{}' not found", stored.provider)))?;

        let refresh_token = stored.refresh_token.as_ref()
            .ok_or_else(|| CredentialError::OAuth("No refresh token available".to_string()))?;

        let mut params = HashMap::new();
        params.insert("grant_type", "refresh_token".to_string());
        params.insert("refresh_token", refresh_token.clone());
        params.insert("client_id", provider.client_id.clone());
        params.insert("client_secret", provider.client_secret.clone());

        let response = self.http_client
            .post(provider.token_url.clone())
            .form(&params)
            .send()
            .await
            .map_err(|e| CredentialError::OAuth(format!("Token refresh failed: {}", e)))?;

        let status = response.status();
        let body: TokenResponse = response
            .json()
            .await
            .map_err(|e| CredentialError::OAuth(format!("Failed to parse refresh response: {}", e)))?;

        if !status.is_success() {
            return Err(CredentialError::OAuth(format!("Token refresh failed with status {}", status)));
        }

        let now = current_timestamp();
        let updated = StoredToken {
            access_token: body.access_token,
            refresh_token: body.refresh_token.or(stored.refresh_token.clone()),
            expires_at: now + body.expires_in,
            last_refreshed: now,
            scope: body.scope.or(stored.scope.clone()),
            ..stored.clone()
        };

        Ok(updated)
    }

    /// Generate a PKCE code verifier
    fn generate_code_verifier(&self) -> String {
        use rand::Rng;
        let mut rng = rand::rngs::OsRng;
        let charset: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~"
            .chars().collect();
        let verifier: String = (0..64)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx]
            })
            .collect();
        verifier
    }

    /// Generate a PKCE code challenge (S256)
    fn generate_code_challenge(&self, verifier: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let hash = hasher.finalize();
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hash)
    }

    /// Check if a token needs refresh
    pub fn needs_refresh(&self, stored: &StoredToken) -> bool {
        let threshold = self.config.refresh_threshold;
        let now = current_timestamp();
        stored.expires_at.saturating_sub(threshold.as_secs()) <= now
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkce_challenge() {
        let config = OAuthConfig {
            providers: vec![],
            redirect_uri: Url::parse("http://localhost:8080/callback").unwrap(),
            refresh_threshold: Duration::from_secs(300),
            client_timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay_ms: 1000,
        };
        let manager = OAuthManager::new(config);
        let verifier = manager.generate_code_verifier();
        assert_eq!(verifier.len(), 64);

        let challenge = manager.generate_code_challenge(&verifier);
        assert!(!challenge.is_empty());

        // Verify deterministic challenge
        let challenge2 = manager.generate_code_challenge(&verifier);
        assert_eq!(challenge, challenge2);
    }
}
