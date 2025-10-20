//! Component validation for ARCADIA game elements

use super::{ValidationLevel, ValidationReport, ValidationResult};
use crate::game_elements::*;

pub struct ComponentValidator;

impl ComponentValidator {
    pub fn validate_functional_component(component: &FunctionalComponent) -> ValidationReport {
        let mut report = ValidationReport::new();

        // Validate ID
        if component.id.is_empty() {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "FunctionalComponent".to_string(),
                "Component ID cannot be empty".to_string(),
            ));
        }

        // Validate position (check for NaN or infinity)
        if !Self::is_valid_position(component.position) {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                format!("FunctionalComponent({})", component.id),
                "Invalid position values detected".to_string(),
            ));
        }

        // Validate properties
        for (key, value) in &component.properties {
            if value.is_nan() || value.is_infinite() {
                report.add(ValidationResult::new(
                    ValidationLevel::Warning,
                    format!("FunctionalComponent({})", component.id),
                    format!("Invalid property value for key: {}", key),
                ));
            }
        }

        report
    }

    pub fn validate_non_functional_components(nfc: &NonFunctionalComponents) -> ValidationReport {
        let mut report = ValidationReport::new();

        if nfc.performance_target_fps <= 0.0 {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "NonFunctionalComponents".to_string(),
                "Performance target FPS must be positive".to_string(),
            ));
        }

        if nfc.performance_target_fps < 30.0 {
            report.add(ValidationResult::new(
                ValidationLevel::Warning,
                "NonFunctionalComponents".to_string(),
                "Performance target FPS is below recommended 30 FPS".to_string(),
            ));
        }

        if nfc.reliability_uptime < 0.0 || nfc.reliability_uptime > 1.0 {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                "NonFunctionalComponents".to_string(),
                "Reliability uptime must be between 0 and 1".to_string(),
            ));
        }

        if nfc.scalability_max_players == 0 {
            report.add(ValidationResult::new(
                ValidationLevel::Warning,
                "NonFunctionalComponents".to_string(),
                "Max players is set to 0".to_string(),
            ));
        }

        report
    }

    pub fn validate_entropy_object(obj: &EntropyObject) -> ValidationReport {
        let mut report = ValidationReport::new();

        if obj.durability < 0.0 || obj.durability > 1.0 {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                format!("EntropyObject({})", obj.id),
                "Durability must be between 0 and 1".to_string(),
            ));
        }

        if obj.age < 0.0 {
            report.add(ValidationResult::new(
                ValidationLevel::Error,
                format!("EntropyObject({})", obj.id),
                "Age cannot be negative".to_string(),
            ));
        }

        if obj.durability < 0.1 {
            report.add(ValidationResult::new(
                ValidationLevel::Warning,
                format!("EntropyObject({})", obj.id),
                "Object is nearly destroyed (durability < 0.1)".to_string(),
            ));
        }

        report
    }

    fn is_valid_position(pos: (f32, f32, f32)) -> bool {
        !pos.0.is_nan()
            && !pos.0.is_infinite()
            && !pos.1.is_nan()
            && !pos.1.is_infinite()
            && !pos.2.is_nan()
            && !pos.2.is_infinite()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_functional_component_valid() {
        let component = FunctionalComponent::new(
            "test1".to_string(),
            ComponentType::Character,
            (1.0, 2.0, 3.0),
        );

        let report = ComponentValidator::validate_functional_component(&component);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_validate_functional_component_empty_id() {
        let component = FunctionalComponent::new(
            "".to_string(),
            ComponentType::Character,
            (1.0, 2.0, 3.0),
        );

        let report = ComponentValidator::validate_functional_component(&component);
        assert!(report.has_errors());
    }

    #[test]
    fn test_validate_non_functional_components_valid() {
        let nfc = NonFunctionalComponents::new();
        let report = ComponentValidator::validate_non_functional_components(&nfc);
        assert!(!report.has_errors());
    }

    #[test]
    fn test_validate_entropy_object() {
        let obj = EntropyObject {
            id: "obj1".to_string(),
            durability: 0.5,
            age: 10.0,
        };

        let report = ComponentValidator::validate_entropy_object(&obj);
        assert!(!report.has_errors());
    }
}
