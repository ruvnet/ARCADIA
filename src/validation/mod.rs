//! Validation framework for ARCADIA
//! 
//! Provides comprehensive validation for all system components

pub mod component_validator;
pub mod config_validator;
pub mod data_integrity;
pub mod security_validator;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationLevel {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub level: ValidationLevel,
    pub message: String,
    pub component: String,
}

impl ValidationResult {
    pub fn new(level: ValidationLevel, component: String, message: String) -> Self {
        ValidationResult {
            level,
            message,
            component,
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self.level, ValidationLevel::Error | ValidationLevel::Critical)
    }
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:?}] {}: {}",
            self.level, self.component, self.message
        )
    }
}

pub struct ValidationReport {
    results: Vec<ValidationResult>,
}

impl ValidationReport {
    pub fn new() -> Self {
        ValidationReport {
            results: Vec::new(),
        }
    }

    pub fn add(&mut self, result: ValidationResult) {
        self.results.push(result);
    }

    pub fn has_errors(&self) -> bool {
        self.results.iter().any(|r| r.is_error())
    }

    pub fn error_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_error()).count()
    }

    pub fn warning_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.level == ValidationLevel::Warning)
            .count()
    }

    pub fn results(&self) -> &[ValidationResult] {
        &self.results
    }

    pub fn summary(&self) -> String {
        format!(
            "Validation Summary: {} errors, {} warnings, {} total checks",
            self.error_count(),
            self.warning_count(),
            self.results.len()
        )
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result() {
        let result = ValidationResult::new(
            ValidationLevel::Error,
            "TestComponent".to_string(),
            "Test error".to_string(),
        );
        
        assert!(result.is_error());
        assert_eq!(result.component, "TestComponent");
    }

    #[test]
    fn test_validation_report() {
        let mut report = ValidationReport::new();
        
        report.add(ValidationResult::new(
            ValidationLevel::Error,
            "Component1".to_string(),
            "Error 1".to_string(),
        ));
        
        report.add(ValidationResult::new(
            ValidationLevel::Warning,
            "Component2".to_string(),
            "Warning 1".to_string(),
        ));
        
        assert!(report.has_errors());
        assert_eq!(report.error_count(), 1);
        assert_eq!(report.warning_count(), 1);
    }
}
