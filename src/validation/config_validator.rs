//! Configuration validation for ARCADIA

use super::{ValidationLevel, ValidationReport, ValidationResult};
use crate::vector_index::VectorIndexConfig;
use crate::authentication::AuthenticationConfig;
use crate::code_dna::CodeDNA;

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate_vector_index_config(config: &VectorIndexConfig) -> ValidationReport {
        let mut report = ValidationReport::new();

        if config.url.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "VectorIndexConfig".to_string(),
                "URL cannot be empty".to_string(),
            ));
        } else if !Self::is_valid_url(&config.url) {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "VectorIndexConfig".to_string(),
                format!("Invalid URL format: {}", config.url),
            ));
        }

        if config.api_key.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "VectorIndexConfig".to_string(),
                "API key cannot be empty".to_string(),
            ));
        } else if config.api_key.len() < 10 {
            report.add(ValidationResult::new(
                ValidationLevel::Warning,
                "VectorIndexConfig".to_string(),
                "API key seems too short (< 10 characters)".to_string(),
            ));
        }

        report
    }

    pub fn validate_authentication_config(config: &AuthenticationConfig) -> ValidationReport {
        let mut report = ValidationReport::new();

        if config.provider.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "AuthenticationConfig".to_string(),
                "Provider cannot be empty".to_string(),
            ));
        }

        if config.credentials.client_id.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "AuthenticationConfig".to_string(),
                "Client ID cannot be empty".to_string(),
            ));
        }

        if config.credentials.client_secret.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "AuthenticationConfig".to_string(),
                "Client secret cannot be empty".to_string(),
            ));
        }

        report
    }

    pub fn validate_code_dna(dna: &CodeDNA) -> ValidationReport {
        let mut report = ValidationReport::new();

        if dna.setting.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "CodeDNA".to_string(),
                "Setting cannot be empty".to_string(),
            ));
        }

        if dna.time_scale <= 0.0 {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "CodeDNA".to_string(),
                "Time scale must be positive".to_string(),
            ));
        }

        if dna.time_scale > 10.0 {
            report.add(ValidationResult::new(
                ValidationLevel::Warning,
                "CodeDNA".to_string(),
                "Time scale is unusually high (> 10.0)".to_string(),
            ));
        }

        if dna.entropy_rate < 0.0 || dna.entropy_rate > 1.0 {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "CodeDNA".to_string(),
                "Entropy rate must be between 0.0 and 1.0".to_string(),
            ));
        }

        if dna.physics_laws.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Warning,
                "CodeDNA".to_string(),
                "No physics laws defined".to_string(),
            ));
        }

        if dna.themes.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Warning,
                "CodeDNA".to_string(),
                "No themes defined".to_string(),
            ));
        }

        report
    }

    fn is_valid_url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::Credentials;

    #[test]
    fn test_validate_vector_index_config_valid() {
        let config = VectorIndexConfig::new(
            "https://example.com".to_string(),
            "valid_api_key_123".to_string(),
        );

        let report = ConfigValidator::validate_vector_index_config(&config);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_validate_vector_index_config_invalid_url() {
        let config = VectorIndexConfig::new(
            "invalid-url".to_string(),
            "valid_api_key_123".to_string(),
        );

        let report = ConfigValidator::validate_vector_index_config(&config);
        assert!(report.has_errors());
    }

    #[test]
    fn test_validate_authentication_config_valid() {
        let config = AuthenticationConfig::new(
            "oauth2".to_string(),
            Credentials::new("client123".to_string(), "secret456".to_string()),
        );

        let report = ConfigValidator::validate_authentication_config(&config);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_validate_code_dna_valid() {
        let dna = CodeDNA::default_scifi();
        let report = ConfigValidator::validate_code_dna(&dna);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_validate_code_dna_invalid_time_scale() {
        let mut dna = CodeDNA::default_scifi();
        dna.time_scale = -1.0;

        let report = ConfigValidator::validate_code_dna(&dna);
        assert!(report.has_errors());
    }
}
