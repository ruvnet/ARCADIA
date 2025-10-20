//! Integration tests for validation framework

use arcadia::validation::*;
use arcadia::*;

#[test]
fn test_complete_system_validation() {
    // Create and validate CodeDNA
    let dna = code_dna::CodeDNA::default_scifi();
    let dna_report = config_validator::ConfigValidator::validate_code_dna(&dna);
    assert!(!dna_report.has_errors());

    // Create and validate VectorIndexConfig
    let vec_config = vector_index::VectorIndexConfig::new(
        "https://api.example.com".to_string(),
        "valid_api_key_12345678".to_string(),
    );
    let vec_report = config_validator::ConfigValidator::validate_vector_index_config(&vec_config);
    assert!(!vec_report.has_errors());

    // Create and validate AuthenticationConfig
    let auth_config = authentication::AuthenticationConfig::new(
        "oauth2".to_string(),
        authentication::Credentials::new("client_id".to_string(), "client_secret".to_string()),
    );
    let auth_report = config_validator::ConfigValidator::validate_authentication_config(&auth_config);
    assert!(!auth_report.has_errors());
}

#[test]
fn test_component_validation_workflow() {
    // Create and validate functional component
    let component = game_elements::FunctionalComponent::new(
        "test_character".to_string(),
        game_elements::ComponentType::Character,
        (1.0, 2.0, 3.0),
    );

    let report = component_validator::ComponentValidator::validate_functional_component(&component);
    assert!(!report.has_errors());

    // Validate non-functional components
    let nfc = game_elements::NonFunctionalComponents::new();
    let nfc_report = component_validator::ComponentValidator::validate_non_functional_components(&nfc);
    assert!(!nfc_report.has_errors());
}

#[test]
fn test_data_integrity_validation() {
    // Validate vector data
    let data = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let report = data_integrity::DataIntegrityValidator::validate_vector_data(&data);
    assert!(!report.has_errors());

    // Validate probability values
    let prob_report = data_integrity::DataIntegrityValidator::validate_probability(0.75, "test_probability");
    assert!(!prob_report.has_errors());

    // Test invalid data
    let invalid_data = vec![1.0, f32::NAN, 3.0];
    let invalid_report = data_integrity::DataIntegrityValidator::validate_vector_data(&invalid_data);
    assert!(invalid_report.has_errors());
}

#[test]
fn test_security_validation_workflow() {
    // Validate API key
    let api_key = "SecureApiKey123456!@#";
    let api_report = security_validator::SecurityValidator::validate_api_key(api_key);
    assert!(!api_report.has_errors());

    // Validate password strength
    let password = "MySecure@Pass123";
    let pwd_report = security_validator::SecurityValidator::validate_password_strength(password);
    assert!(!pwd_report.has_errors());

    // Test weak password
    let weak_password = "123";
    let weak_report = security_validator::SecurityValidator::validate_password_strength(weak_password);
    assert!(weak_report.has_errors());
}

#[test]
fn test_validation_report_aggregation() {
    let mut report = ValidationReport::new();

    // Add various validation results
    report.add(ValidationResult::new(
        ValidationLevel::Error,
        "Component1".to_string(),
        "Critical error".to_string(),
    ));

    report.add(ValidationResult::new(
        ValidationLevel::Warning,
        "Component2".to_string(),
        "Warning message".to_string(),
    ));

    report.add(ValidationResult::new(
        ValidationLevel::Info,
        "Component3".to_string(),
        "Info message".to_string(),
    ));

    assert!(report.has_errors());
    assert_eq!(report.error_count(), 1);
    assert_eq!(report.warning_count(), 1);
    assert_eq!(report.results().len(), 3);

    let summary = report.summary();
    assert!(summary.contains("1 errors"));
    assert!(summary.contains("1 warnings"));
}

#[test]
fn test_end_to_end_validation() {
    // Create complete game setup
    let dna = code_dna::CodeDNA::default_fantasy();
    let game = game_elements::GameElements::new(dna.clone());

    // Validate everything
    let dna_report = config_validator::ConfigValidator::validate_code_dna(&dna);
    let nfc_report = component_validator::ComponentValidator::validate_non_functional_components(
        &game.non_functional_components,
    );

    // Aggregate reports
    let total_errors = dna_report.error_count() + nfc_report.error_count();
    assert_eq!(total_errors, 0);
}
