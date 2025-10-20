//! Data integrity validation

use super::{ValidationLevel, ValidationReport, ValidationResult};
use std::collections::HashMap;

pub struct DataIntegrityValidator;

impl DataIntegrityValidator {
    pub fn validate_vector_data(data: &[f32]) -> ValidationReport {
        let mut report = ValidationReport::new();

        if data.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "VectorData".to_string(),
                "Vector data cannot be empty".to_string(),
            ));
            return report;
        }

        for (idx, &value) in data.iter().enumerate() {
            if value.is_nan() {
                report.add(ValidationResult::new(
                    ValidationLevel::Error,
                    "VectorData".to_string(),
                    format!("NaN value detected at index {}", idx),
                ));
            }

            if value.is_infinite() {
                report.add(ValidationResult::new(
                    ValidationLevel::Error,
                    "VectorData".to_string(),
                    format!("Infinite value detected at index {}", idx),
                ));
            }
        }

        report
    }

    pub fn validate_probability(value: f32, name: &str) -> ValidationReport {
        let mut report = ValidationReport::new();

        if value < 0.0 || value > 1.0 {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "Probability".to_string(),
                format!("{} must be between 0.0 and 1.0, got {}", name, value),
            ));
        }

        if value.is_nan() {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "Probability".to_string(),
                format!("{} is NaN", name),
            ));
        }

        report
    }

    pub fn validate_metadata(metadata: &HashMap<String, String>) -> ValidationReport {
        let mut report = ValidationReport::new();

        for (key, value) in metadata {
            if key.is_empty() {
                report.add(ValidationResult::new(
                    ValidationLevel::Warning,
                    "Metadata".to_string(),
                    "Empty key found in metadata".to_string(),
                ));
            }

            if value.is_empty() {
                report.add(ValidationResult::new(
                    ValidationLevel::Info,
                    "Metadata".to_string(),
                    format!("Empty value for key: {}", key),
                ));
            }

            if key.len() > 255 {
                report.add(ValidationResult::new(
                    ValidationLevel::Warning,
                    "Metadata".to_string(),
                    format!("Key exceeds recommended length: {}", key),
                ));
            }
        }

        report
    }

    pub fn check_duplicates<T: std::cmp::Eq + std::hash::Hash + Clone>(
        items: &[T],
        name: &str,
    ) -> ValidationReport
    where
        T: std::fmt::Debug,
    {
        let mut report = ValidationReport::new();
        let mut seen = std::collections::HashSet::new();

        for item in items {
            if !seen.insert(item.clone()) {
                report.add(ValidationResult::new(
                    ValidationLevel::Warning,
                    name.to_string(),
                    format!("Duplicate item found: {:?}", item),
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
    fn test_validate_vector_data_valid() {
        let data = vec![1.0, 2.0, 3.0];
        let report = DataIntegrityValidator::validate_vector_data(&data);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_validate_vector_data_nan() {
        let data = vec![1.0, f32::NAN, 3.0];
        let report = DataIntegrityValidator::validate_vector_data(&data);
        assert!(report.has_errors());
    }

    #[test]
    fn test_validate_probability_valid() {
        let report = DataIntegrityValidator::validate_probability(0.5, "test");
        assert!(!report.has_errors());
    }

    #[test]
    fn test_validate_probability_invalid() {
        let report = DataIntegrityValidator::validate_probability(1.5, "test");
        assert!(report.has_errors());
    }

    #[test]
    fn test_validate_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), "value1".to_string());

        let report = DataIntegrityValidator::validate_metadata(&metadata);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_check_duplicates() {
        let items = vec!["a", "b", "a", "c"];
        let report = DataIntegrityValidator::check_duplicates(&items, "TestItems");
        assert_eq!(report.warning_count(), 1);
    }
}
