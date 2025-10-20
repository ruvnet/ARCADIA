//! Test fixtures and mock data for ARCADIA tests

use arcadia::*;
use std::collections::HashMap;

/// Create a test VectorIndexConfig
pub fn create_test_vector_config() -> vector_index::VectorIndexConfig {
    vector_index::VectorIndexConfig::new(
        "https://test.example.com".to_string(),
        "test_api_key_12345678".to_string(),
    )
}

/// Create a test AuthenticationConfig
pub fn create_test_auth_config() -> authentication::AuthenticationConfig {
    authentication::AuthenticationConfig::new(
        "oauth2".to_string(),
        authentication::Credentials::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
        ),
    )
}

/// Create a test CodeDNA with sci-fi theme
pub fn create_scifi_dna() -> code_dna::CodeDNA {
    code_dna::CodeDNA::default_scifi()
}

/// Create a test CodeDNA with fantasy theme
pub fn create_fantasy_dna() -> code_dna::CodeDNA {
    code_dna::CodeDNA::default_fantasy()
}

/// Create a test Vector with specified dimensions
pub fn create_test_vector(id: &str, dimension: usize) -> vector_index::Vector {
    let data: Vec<f32> = (0..dimension).map(|i| i as f32 / dimension as f32).collect();
    vector_index::Vector::new(id.to_string(), data)
}

/// Create multiple test vectors
pub fn create_test_vectors(count: usize, dimension: usize) -> Vec<vector_index::Vector> {
    (0..count)
        .map(|i| create_test_vector(&format!("vec_{}", i), dimension))
        .collect()
}

/// Create a test FunctionalComponent
pub fn create_test_component(id: &str) -> game_elements::FunctionalComponent {
    game_elements::FunctionalComponent::new(
        id.to_string(),
        game_elements::ComponentType::Character,
        (0.0, 0.0, 0.0),
    )
}

/// Create a test GameElements instance
pub fn create_test_game() -> game_elements::GameElements {
    let dna = create_scifi_dna();
    game_elements::GameElements::new(dna)
}

/// Create a populated GameElements instance with components
pub fn create_populated_game(component_count: usize) -> game_elements::GameElements {
    let mut game = create_test_game();
    
    for i in 0..component_count {
        let component = create_test_component(&format!("component_{}", i));
        game.add_component(component);
    }
    
    game
}

/// Create test metadata
pub fn create_test_metadata() -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    metadata.insert("type".to_string(), "test".to_string());
    metadata.insert("category".to_string(), "fixture".to_string());
    metadata.insert("version".to_string(), "1.0".to_string());
    metadata
}

/// Create a test Authentication instance
pub fn create_test_auth() -> authentication::Authentication {
    authentication::Authentication::new(create_test_auth_config())
}

/// Create a test VectorIndex with sample data
pub fn create_test_vector_index(vector_count: usize) -> vector_index::VectorIndex {
    let config = create_test_vector_config();
    let mut index = vector_index::VectorIndex::new(config);
    
    let vectors = create_test_vectors(vector_count, 128);
    for vector in vectors {
        index.insert(vector).unwrap();
    }
    
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_vector_config() {
        let config = create_test_vector_config();
        assert!(!config.url.is_empty());
        assert!(!config.api_key.is_empty());
    }

    #[test]
    fn test_create_test_auth_config() {
        let config = create_test_auth_config();
        assert_eq!(config.provider, "oauth2");
    }

    #[test]
    fn test_create_test_vectors() {
        let vectors = create_test_vectors(5, 10);
        assert_eq!(vectors.len(), 5);
        assert_eq!(vectors[0].dimension(), 10);
    }

    #[test]
    fn test_create_populated_game() {
        let game = create_populated_game(10);
        assert_eq!(game.component_count(), 10);
    }

    #[test]
    fn test_create_test_vector_index() {
        let index = create_test_vector_index(50);
        assert_eq!(index.size(), 50);
        assert_eq!(index.dimension(), Some(128));
    }
}
