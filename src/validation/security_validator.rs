//! Security validation for ARCADIA

use super::{ValidationLevel, ValidationReport, ValidationResult};

pub struct SecurityValidator;

impl SecurityValidator {
    pub fn validate_api_key(api_key: &str) -> ValidationReport {
        let mut report = ValidationReport::new();

        if api_key.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "Security".to_string(),
                "API key cannot be empty".to_string(),
            ));
            return report;
        }

        if api_key.len() < 16 {
            report.add(ValidationResult::new(
                ValidationLevel::Warning,
                "Security".to_string(),
                "API key is shorter than recommended 16 characters".to_string(),
            ));
        }

        if api_key.chars().all(|c| c.is_ascii_digit()) {
            report.add(ValidationResult::new(
                ValidationLevel::Warning,
                "Security".to_string(),
                "API key contains only digits (low entropy)".to_string(),
            ));
        }

        if api_key.to_lowercase() == api_key || api_key.to_uppercase() == api_key {
            report.add(ValidationResult::new(
                ValidationLevel::Info,
                "Security".to_string(),
                "API key uses only one case (consider mixed case)".to_string(),
            ));
        }

        report
    }

    pub fn validate_password_strength(password: &str) -> ValidationReport {
        let mut report = ValidationReport::new();

        if password.len() < 8 {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "Security".to_string(),
                "Password must be at least 8 characters".to_string(),
            ));
        }

        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());

        let strength = [has_uppercase, has_lowercase, has_digit, has_special]
            .iter()
            .filter(|&&x| x)
            .count();

        match strength {
            0..=1 => report.add(ValidationResult::new(
                ValidationLevel::Error,
                "Security".to_string(),
                "Password is too weak".to_string(),
            )),
            2 => report.add(ValidationResult::new(
                ValidationLevel::Warning,
                "Security".to_string(),
                "Password strength is moderate".to_string(),
            )),
            3 => report.add(ValidationResult::new(
                ValidationLevel::Info,
                "Security".to_string(),
                "Password strength is good".to_string(),
            )),
            _ => {}
        }

        report
    }

    pub fn validate_token(token: &str) -> ValidationReport {
        let mut report = ValidationReport::new();

        if token.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "Security".to_string(),
                "Token cannot be empty".to_string(),
            ));
        }

        // Check for common test/placeholder tokens
        let test_tokens = ["test", "token", "placeholder", "example", "demo"];
        if test_tokens.iter().any(|&t| token.to_lowercase().contains(t)) {
            report.add(ValidationResult::new(
                ValidationLevel::Warning,
                "Security".to_string(),
                "Token appears to be a test/placeholder value".to_string(),
            ));
        }

        report
    }

    pub fn scan_for_secrets(text: &str) -> ValidationReport {
        let mut report = ValidationReport::new();

        let sensitive_patterns = [
            "password",
            "secret",
            "api_key",
            "private_key",
            "token",
            "credential",
        ];

        for pattern in &sensitive_patterns {
            if text.to_lowercase().contains(pattern) {
                report.add(ValidationResult::new(
                    ValidationLevel::Warning,
                    "Security".to_string(),
                    format!("Potential sensitive data detected: {}", pattern),
                ));
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_api_key_strong() {
        let report = SecurityValidator::validate_api_key("AbCd1234!@#$XyZ9");
        assert!(!report.has_errors());
    }

    #[test]
    fn test_validate_api_key_weak() {
        let report = SecurityValidator::validate_api_key("123");
        assert!(report.has_errors() || report.warning_count() > 0);
    }

    #[test]
    fn test_validate_password_strength_strong() {
        let report = SecurityValidator::validate_password_strength("MyP@ssw0rd!");
        assert!(!report.has_errors());
    }

    #[test]
    fn test_validate_password_strength_weak() {
        let report = SecurityValidator::validate_password_strength("pass");
        assert!(report.has_errors());
    }

    #[test]
    fn test_validate_token() {
        let report = SecurityValidator::validate_token("valid-token-12345");
        assert!(!report.has_errors());
    }

    #[test]
    fn test_scan_for_secrets() {
        let text = "This contains a password=secret123";
        let report = SecurityValidator::scan_for_secrets(text);
        assert!(report.warning_count() > 0);
    }
}
