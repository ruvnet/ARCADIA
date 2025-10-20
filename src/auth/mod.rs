//! Authentication module for ARCADIA
//! Provides OAuth2 authentication and JWT token management

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("OAuth2 error: {0}")]
    OAuth2Error(String),
    
    #[error("JWT error: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Session not found")]
    SessionNotFound,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Unauthorized")]
    Unauthorized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    pub provider: String,
    pub credentials: Credentials,
    pub jwt_secret: String,
    pub token_expiry_hours: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: Option<String>,
    pub auth_url: Option<String>,
    pub token_url: Option<String>,
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            provider: "google".to_string(),
            credentials: Credentials {
                client_id: String::new(),
                client_secret: String::new(),
                redirect_url: Some("http://localhost:8080/auth/callback".to_string()),
                auth_url: Some("https://accounts.google.com/o/oauth2/v2/auth".to_string()),
                token_url: Some("https://oauth2.googleapis.com/token".to_string()),
            },
            jwt_secret: "change-me-in-production".to_string(),
            token_expiry_hours: 24,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub email: Option<String>,
    pub name: Option<String>,
}

pub struct Authentication {
    config: AuthenticationConfig,
    oauth_client: Option<BasicClient>,
    sessions: HashMap<String, Session>,
}

#[derive(Debug, Clone)]
struct Session {
    user_id: String,
    email: Option<String>,
    created_at: chrono::DateTime<Utc>,
    expires_at: chrono::DateTime<Utc>,
}

impl Authentication {
    /// Create a new Authentication instance
    pub fn new(config: AuthenticationConfig) -> Result<Self, AuthError> {
        let oauth_client = if config.credentials.auth_url.is_some()
            && config.credentials.token_url.is_some()
        {
            let client_id = ClientId::new(config.credentials.client_id.clone());
            let client_secret = ClientSecret::new(config.credentials.client_secret.clone());
            
            let auth_url = AuthUrl::new(
                config
                    .credentials
                    .auth_url
                    .clone()
                    .unwrap_or_default(),
            )
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;
            
            let token_url = TokenUrl::new(
                config
                    .credentials
                    .token_url
                    .clone()
                    .unwrap_or_default(),
            )
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;
            
            let redirect_url = RedirectUrl::new(
                config
                    .credentials
                    .redirect_url
                    .clone()
                    .unwrap_or_else(|| "http://localhost:8080/auth/callback".to_string()),
            )
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;
            
            Some(
                BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
                    .set_redirect_uri(redirect_url),
            )
        } else {
            None
        };
        
        Ok(Self {
            config,
            oauth_client,
            sessions: HashMap::new(),
        })
    }
    
    /// Generate OAuth2 authorization URL
    pub fn get_authorization_url(&self) -> Result<(String, String), AuthError> {
        let client = self
            .oauth_client
            .as_ref()
            .ok_or(AuthError::OAuth2Error("OAuth client not configured".to_string()))?;
        
        let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        
        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();
        
        Ok((auth_url.to_string(), csrf_token.secret().to_string()))
    }
    
    /// Exchange authorization code for access token
    pub async fn exchange_code(&self, code: &str) -> Result<String, AuthError> {
        let client = self
            .oauth_client
            .as_ref()
            .ok_or(AuthError::OAuth2Error("OAuth client not configured".to_string()))?;
        
        let token_result = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|e| AuthError::OAuth2Error(e.to_string()))?;
        
        Ok(token_result.access_token().secret().to_string())
    }
    
    /// Create a JWT token for a user
    pub fn create_jwt(&self, user_id: &str, email: Option<String>) -> Result<String, AuthError> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.config.token_expiry_hours);
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            email: email.clone(),
            name: None,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )?;
        
        Ok(token)
    }
    
    /// Validate and decode a JWT token
    pub fn validate_jwt(&self, token: &str) -> Result<Claims, AuthError> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &validation,
        )?;
        
        if token_data.claims.exp < Utc::now().timestamp() {
            return Err(AuthError::TokenExpired);
        }
        
        Ok(token_data.claims)
    }
    
    /// Create a session for a user
    pub fn create_session(&mut self, user_id: &str, email: Option<String>) -> String {
        let session_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + Duration::hours(self.config.token_expiry_hours);
        
        let session = Session {
            user_id: user_id.to_string(),
            email,
            created_at: now,
            expires_at,
        };
        
        self.sessions.insert(session_id.clone(), session);
        session_id
    }
    
    /// Validate a session
    pub fn validate_session(&self, session_id: &str) -> Result<String, AuthError> {
        let session = self
            .sessions
            .get(session_id)
            .ok_or(AuthError::SessionNotFound)?;
        
        if session.expires_at < Utc::now() {
            return Err(AuthError::TokenExpired);
        }
        
        Ok(session.user_id.clone())
    }
    
    /// Delete a session
    pub fn delete_session(&mut self, session_id: &str) -> Result<(), AuthError> {
        self.sessions
            .remove(session_id)
            .ok_or(AuthError::SessionNotFound)?;
        Ok(())
    }
    
    /// Clean up expired sessions
    pub fn cleanup_expired_sessions(&mut self) {
        let now = Utc::now();
        self.sessions.retain(|_, session| session.expires_at > now);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_and_validate_jwt() {
        let config = AuthenticationConfig::default();
        let auth = Authentication::new(config).unwrap();
        
        let token = auth
            .create_jwt("user123", Some("user@example.com".to_string()))
            .unwrap();
        
        let claims = auth.validate_jwt(&token).unwrap();
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.email, Some("user@example.com".to_string()));
    }
    
    #[test]
    fn test_session_management() {
        let config = AuthenticationConfig::default();
        let mut auth = Authentication::new(config).unwrap();
        
        let session_id = auth.create_session("user123", Some("user@example.com".to_string()));
        
        let user_id = auth.validate_session(&session_id).unwrap();
        assert_eq!(user_id, "user123");
        
        auth.delete_session(&session_id).unwrap();
        assert!(auth.validate_session(&session_id).is_err());
    }
}
