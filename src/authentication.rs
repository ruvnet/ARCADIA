//! Authentication module for ARCADIA
//! 
//! Handles authentication, authorization, and credential management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
}

impl Credentials {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Credentials {
            client_id,
            client_secret,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.client_id.is_empty() {
            return Err("Client ID cannot be empty".to_string());
        }
        if self.client_secret.is_empty() {
            return Err("Client secret cannot be empty".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    pub provider: String,
    pub credentials: Credentials,
}

impl AuthenticationConfig {
    pub fn new(provider: String, credentials: Credentials) -> Self {
        AuthenticationConfig {
            provider,
            credentials,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.provider.is_empty() {
            return Err("Provider cannot be empty".to_string());
        }
        self.credentials.validate()
    }
}

#[derive(Debug)]
pub struct Authentication {
    config: AuthenticationConfig,
    sessions: HashMap<String, Session>,
}

#[derive(Debug, Clone)]
struct Session {
    user_id: String,
    token: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

impl Authentication {
    pub fn new(config: AuthenticationConfig) -> Self {
        Authentication {
            config,
            sessions: HashMap::new(),
        }
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<String, String> {
        // Simplified authentication - in production, this would verify against a database
        if username.is_empty() || password.is_empty() {
            return Err("Invalid credentials".to_string());
        }

        let token = uuid::Uuid::new_v4().to_string();
        let session = Session {
            user_id: username.to_string(),
            token: token.clone(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
        };

        self.sessions.insert(token.clone(), session);
        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<String, String> {
        match self.sessions.get(token) {
            Some(session) => {
                if session.expires_at > chrono::Utc::now() {
                    Ok(session.user_id.clone())
                } else {
                    Err("Token expired".to_string())
                }
            }
            None => Err("Invalid token".to_string()),
        }
    }

    pub fn revoke_token(&mut self, token: &str) -> Result<(), String> {
        self.sessions
            .remove(token)
            .map(|_| ())
            .ok_or_else(|| "Token not found".to_string())
    }

    pub fn get_provider(&self) -> &str {
        &self.config.provider
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> AuthenticationConfig {
        AuthenticationConfig::new(
            "test_provider".to_string(),
            Credentials::new("test_id".to_string(), "test_secret".to_string()),
        )
    }

    #[test]
    fn test_credentials_creation() {
        let creds = Credentials::new("id123".to_string(), "secret456".to_string());
        assert_eq!(creds.client_id, "id123");
        assert_eq!(creds.client_secret, "secret456");
    }

    #[test]
    fn test_credentials_validation() {
        let valid_creds = Credentials::new("id".to_string(), "secret".to_string());
        assert!(valid_creds.validate().is_ok());

        let invalid_creds = Credentials::new("".to_string(), "secret".to_string());
        assert!(invalid_creds.validate().is_err());
    }

    #[test]
    fn test_authentication_creation() {
        let config = create_test_config();
        let auth = Authentication::new(config);
        assert_eq!(auth.get_provider(), "test_provider");
    }

    #[test]
    fn test_authenticate() {
        let config = create_test_config();
        let mut auth = Authentication::new(config);
        
        let result = auth.authenticate("user1", "password123");
        assert!(result.is_ok());
        
        let token = result.unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_validate_token() {
        let config = create_test_config();
        let mut auth = Authentication::new(config);
        
        let token = auth.authenticate("user1", "password123").unwrap();
        let validation_result = auth.validate_token(&token);
        
        assert!(validation_result.is_ok());
        assert_eq!(validation_result.unwrap(), "user1");
    }

    #[test]
    fn test_invalid_token() {
        let config = create_test_config();
        let auth = Authentication::new(config);
        
        let result = auth.validate_token("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn test_revoke_token() {
        let config = create_test_config();
        let mut auth = Authentication::new(config);
        
        let token = auth.authenticate("user1", "password123").unwrap();
        assert!(auth.revoke_token(&token).is_ok());
        assert!(auth.validate_token(&token).is_err());
    }

    #[test]
    fn test_session_count() {
        let config = create_test_config();
        let mut auth = Authentication::new(config);
        
        assert_eq!(auth.session_count(), 0);
        
        auth.authenticate("user1", "pass1").unwrap();
        assert_eq!(auth.session_count(), 1);
        
        auth.authenticate("user2", "pass2").unwrap();
        assert_eq!(auth.session_count(), 2);
    }
}
