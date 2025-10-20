//! aiTOML module for ARCADIA
//! Parses and validates TOML-based workflow specifications for AI-driven game dynamics

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AiTomlError {
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),
    
    #[error("Failed to parse TOML: {0}")]
    ParseError(#[from] toml::de::Error),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Workflow not found: {0}")]
    WorkflowNotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiToml {
    #[serde(default)]
    pub metadata: Metadata,
    
    #[serde(default)]
    pub vector_index: VectorIndexConfig,
    
    #[serde(default)]
    pub authentication: AuthenticationConfig,
    
    #[serde(default)]
    pub game_elements: HashMap<String, GameElement>,
    
    #[serde(default)]
    pub workflows: HashMap<String, Workflow>,
    
    #[serde(default)]
    pub code_dna: Option<CodeDnaConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorIndexConfig {
    pub url: String,
    pub api_key: String,
    #[serde(default)]
    pub qdrant_url: Option<String>,
    #[serde(default = "default_collection_name")]
    pub collection_name: String,
    #[serde(default = "default_embedding_model")]
    pub embedding_model: String,
    #[serde(default = "default_vector_dimension")]
    pub vector_dimension: usize,
}

fn default_collection_name() -> String {
    "arcadia_vectors".to_string()
}

fn default_embedding_model() -> String {
    "text-embedding-3-small".to_string()
}

fn default_vector_dimension() -> usize {
    1536
}

impl Default for VectorIndexConfig {
    fn default() -> Self {
        Self {
            url: "https://api.openai.com".to_string(),
            api_key: String::new(),
            qdrant_url: Some("http://localhost:6334".to_string()),
            collection_name: default_collection_name(),
            embedding_model: default_embedding_model(),
            vector_dimension: default_vector_dimension(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    pub provider: String,
    pub credentials: Credentials,
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    #[serde(default = "default_token_expiry")]
    pub token_expiry_hours: i64,
}

fn default_jwt_secret() -> String {
    "change-me-in-production".to_string()
}

fn default_token_expiry() -> i64 {
    24
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            provider: "google".to_string(),
            credentials: Credentials::default(),
            jwt_secret: default_jwt_secret(),
            token_expiry_hours: default_token_expiry(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    #[serde(default)]
    pub redirect_url: Option<String>,
    #[serde(default)]
    pub auth_url: Option<String>,
    #[serde(default)]
    pub token_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameElement {
    pub element_type: String,
    #[serde(default)]
    pub properties: HashMap<String, String>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    #[serde(default)]
    pub triggers: Vec<String>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub action: String,
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub conditions: Vec<Condition>,
    #[serde(default)]
    pub next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDnaConfig {
    pub setting: String,
    pub technology: String,
    #[serde(default)]
    pub physics_laws: Vec<String>,
    #[serde(default)]
    pub themes: Vec<String>,
    #[serde(default = "default_time_scale")]
    pub time_scale: f32,
    #[serde(default = "default_entropy_rate")]
    pub entropy_rate: f32,
    #[serde(default)]
    pub natural_laws: Vec<String>,
}

fn default_time_scale() -> f32 {
    1.0
}

fn default_entropy_rate() -> f32 {
    0.01
}

pub struct AiTomlParser {
    config: AiToml,
}

impl AiTomlParser {
    /// Load and parse an aiTOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, AiTomlError> {
        let contents = fs::read_to_string(path)?;
        Self::from_str(&contents)
    }
    
    /// Parse aiTOML from a string
    pub fn from_str(contents: &str) -> Result<Self, AiTomlError> {
        let config: AiToml = toml::from_str(contents)?;
        let parser = Self { config };
        parser.validate()?;
        Ok(parser)
    }
    
    /// Validate the configuration
    fn validate(&self) -> Result<(), AiTomlError> {
        // Validate vector index
        if self.config.vector_index.api_key.is_empty() {
            return Err(AiTomlError::ValidationError(
                "Vector index API key is required".to_string(),
            ));
        }
        
        // Validate authentication
        if self.config.authentication.credentials.client_id.is_empty() {
            return Err(AiTomlError::ValidationError(
                "Authentication client_id is required".to_string(),
            ));
        }
        
        // Validate workflows
        for (name, workflow) in &self.config.workflows {
            if workflow.steps.is_empty() {
                return Err(AiTomlError::ValidationError(format!(
                    "Workflow '{}' has no steps",
                    name
                )));
            }
            
            // Validate step references
            for step in &workflow.steps {
                if let Some(ref next) = step.next {
                    if !workflow.steps.iter().any(|s| &s.id == next) {
                        return Err(AiTomlError::ValidationError(format!(
                            "Workflow '{}' step '{}' references non-existent step '{}'",
                            name, step.id, next
                        )));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Get the configuration
    pub fn config(&self) -> &AiToml {
        &self.config
    }
    
    /// Get a specific workflow
    pub fn get_workflow(&self, name: &str) -> Result<&Workflow, AiTomlError> {
        self.config
            .workflows
            .get(name)
            .ok_or_else(|| AiTomlError::WorkflowNotFound(name.to_string()))
    }
    
    /// Execute a workflow step
    pub async fn execute_workflow(
        &self,
        workflow_name: &str,
        context: &mut HashMap<String, serde_json::Value>,
    ) -> Result<(), AiTomlError> {
        let workflow = self.get_workflow(workflow_name)?;
        
        if !workflow.enabled {
            return Ok(());
        }
        
        let mut current_step_id = workflow.steps[0].id.clone();
        
        loop {
            let step = workflow
                .steps
                .iter()
                .find(|s| s.id == current_step_id)
                .ok_or_else(|| {
                    AiTomlError::ValidationError(format!("Step '{}' not found", current_step_id))
                })?;
            
            // Check conditions
            let mut all_conditions_met = true;
            for condition in &step.conditions {
                if !self.evaluate_condition(condition, context) {
                    all_conditions_met = false;
                    break;
                }
            }
            
            if !all_conditions_met {
                break;
            }
            
            // Execute step action (placeholder for actual implementation)
            tracing::info!(
                "Executing workflow '{}' step '{}': {}",
                workflow_name,
                step.id,
                step.action
            );
            
            // Move to next step
            match &step.next {
                Some(next_id) => current_step_id = next_id.clone(),
                None => break,
            }
        }
        
        Ok(())
    }
    
    /// Evaluate a condition
    fn evaluate_condition(
        &self,
        condition: &Condition,
        context: &HashMap<String, serde_json::Value>,
    ) -> bool {
        let field_value = match context.get(&condition.field) {
            Some(v) => v,
            None => return false,
        };
        
        match condition.operator.as_str() {
            "eq" => field_value == &condition.value,
            "ne" => field_value != &condition.value,
            "gt" => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    a > b
                } else {
                    false
                }
            }
            "lt" => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    a < b
                } else {
                    false
                }
            }
            "gte" => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    a >= b
                } else {
                    false
                }
            }
            "lte" => {
                if let (Some(a), Some(b)) = (field_value.as_f64(), condition.value.as_f64()) {
                    a <= b
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_minimal_config() {
        let toml = r#"
            [metadata]
            name = "test"
            version = "0.1.0"
            
            [vector_index]
            url = "https://api.openai.com"
            api_key = "test-key"
            
            [authentication]
            provider = "google"
            
            [authentication.credentials]
            client_id = "test-client-id"
            client_secret = "test-secret"
        "#;
        
        let parser = AiTomlParser::from_str(toml).unwrap();
        assert_eq!(parser.config().metadata.name, "test");
        assert_eq!(parser.config().vector_index.api_key, "test-key");
    }
    
    #[test]
    fn test_workflow_validation() {
        let toml = r#"
            [vector_index]
            api_key = "test-key"
            
            [authentication.credentials]
            client_id = "test-id"
            client_secret = "test-secret"
            
            [workflows.test]
            name = "Test Workflow"
            description = "A test workflow"
            enabled = true
            
            [[workflows.test.steps]]
            id = "step1"
            action = "test"
            next = "step2"
            
            [[workflows.test.steps]]
            id = "step2"
            action = "test2"
        "#;
        
        let parser = AiTomlParser::from_str(toml).unwrap();
        let workflow = parser.get_workflow("test").unwrap();
        assert_eq!(workflow.steps.len(), 2);
    }
}
