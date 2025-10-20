//! Symbolic Computing Module
//!
//! This module implements abstract concept processing, relationship graphs,
//! logical inference, and knowledge representation for AI systems.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents an abstract concept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub attributes: HashMap<String, ConceptValue>,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
}

/// Value types for concept attributes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConceptValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<String>),
    Concept(Uuid),
}

/// Relationship between concepts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: Uuid,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub relationship_type: RelationType,
    pub strength: f64,
    pub bidirectional: bool,
    pub metadata: HashMap<String, String>,
}

/// Types of relationships between concepts
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    IsA,           // Inheritance
    PartOf,        // Composition
    HasProperty,   // Property
    CausedBy,      // Causation
    EnabledBy,     // Enablement
    Requires,      // Dependency
    SimilarTo,     // Similarity
    OppositeOf,    // Opposition
    Custom(String),
}

/// Logical rule for inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRule {
    pub id: Uuid,
    pub name: String,
    pub premises: Vec<Premise>,
    pub conclusion: Conclusion,
    pub confidence: f64,
}

/// Premise for inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Premise {
    pub concept_pattern: String,
    pub relationship_type: Option<RelationType>,
    pub conditions: Vec<String>,
}

/// Conclusion from inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conclusion {
    pub inferred_concept: Option<String>,
    pub inferred_relationship: Option<RelationType>,
    pub target_concepts: Vec<Uuid>,
    pub confidence_modifier: f64,
}

/// Query for knowledge base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub concept_name: Option<String>,
    pub relationship_type: Option<RelationType>,
    pub attributes: HashMap<String, ConceptValue>,
    pub depth: usize,
}

/// Result of a query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub concepts: Vec<Concept>,
    pub relationships: Vec<Relationship>,
    pub inference_chain: Vec<String>,
    pub confidence: f64,
}

/// Symbolic computing system
pub struct SymbolicComputing {
    /// Knowledge base of concepts
    concepts: Arc<RwLock<HashMap<Uuid, Concept>>>,

    /// Concept name to ID mapping
    concept_index: Arc<RwLock<HashMap<String, Uuid>>>,

    /// Relationship graph
    relationships: Arc<RwLock<Vec<Relationship>>>,

    /// Inference rules
    inference_rules: Arc<RwLock<Vec<InferenceRule>>>,

    /// Category taxonomy
    categories: Arc<RwLock<HashMap<String, Vec<String>>>>,

    /// Reasoning history
    reasoning_history: Arc<RwLock<VecDeque<String>>>,

    /// Max history size
    max_history_size: usize,
}

impl SymbolicComputing {
    /// Create a new symbolic computing system
    pub fn new() -> Self {
        Self {
            concepts: Arc::new(RwLock::new(HashMap::new())),
            concept_index: Arc::new(RwLock::new(HashMap::new())),
            relationships: Arc::new(RwLock::new(Vec::new())),
            inference_rules: Arc::new(RwLock::new(Vec::new())),
            categories: Arc::new(RwLock::new(HashMap::new())),
            reasoning_history: Arc::new(RwLock::new(VecDeque::new())),
            max_history_size: 1000,
        }
    }

    /// Add a concept to the knowledge base
    pub fn add_concept(&self, name: String, category: String, attributes: HashMap<String, ConceptValue>) -> Uuid {
        let concept = Concept {
            id: Uuid::new_v4(),
            name: name.clone(),
            category: category.clone(),
            attributes,
            confidence: 1.0,
            created_at: Utc::now(),
        };

        let id = concept.id;
        self.concepts.write().insert(id, concept);
        self.concept_index.write().insert(name, id);

        // Update category taxonomy
        self.categories.write()
            .entry(category)
            .or_insert_with(Vec::new)
            .push(id.to_string());

        id
    }

    /// Get concept by ID
    pub fn get_concept(&self, id: Uuid) -> Option<Concept> {
        self.concepts.read().get(&id).cloned()
    }

    /// Get concept by name
    pub fn get_concept_by_name(&self, name: &str) -> Option<Concept> {
        let index = self.concept_index.read();
        let id = index.get(name)?;
        self.concepts.read().get(id).cloned()
    }

    /// Update concept attributes
    pub fn update_concept(&self, id: Uuid, attributes: HashMap<String, ConceptValue>) -> bool {
        if let Some(concept) = self.concepts.write().get_mut(&id) {
            concept.attributes.extend(attributes);
            true
        } else {
            false
        }
    }

    /// Add a relationship between concepts
    pub fn add_relationship(
        &self,
        source_id: Uuid,
        target_id: Uuid,
        relationship_type: RelationType,
        strength: f64,
        bidirectional: bool,
    ) -> Uuid {
        let relationship = Relationship {
            id: Uuid::new_v4(),
            source_id,
            target_id,
            relationship_type,
            strength: strength.clamp(0.0, 1.0),
            bidirectional,
            metadata: HashMap::new(),
        };

        let id = relationship.id;
        self.relationships.write().push(relationship);

        self.log_reasoning(format!(
            "Added relationship: {:?} between {} and {}",
            relationship_type, source_id, target_id
        ));

        id
    }

    /// Get all relationships for a concept
    pub fn get_relationships(&self, concept_id: Uuid) -> Vec<Relationship> {
        self.relationships.read()
            .iter()
            .filter(|r| r.source_id == concept_id || (r.bidirectional && r.target_id == concept_id))
            .cloned()
            .collect()
    }

    /// Get relationships by type
    pub fn get_relationships_by_type(&self, concept_id: Uuid, rel_type: &RelationType) -> Vec<Relationship> {
        self.relationships.read()
            .iter()
            .filter(|r| {
                r.relationship_type == *rel_type &&
                (r.source_id == concept_id || (r.bidirectional && r.target_id == concept_id))
            })
            .cloned()
            .collect()
    }

    /// Add an inference rule
    pub fn add_inference_rule(&self, name: String, premises: Vec<Premise>, conclusion: Conclusion, confidence: f64) -> Uuid {
        let rule = InferenceRule {
            id: Uuid::new_v4(),
            name,
            premises,
            conclusion,
            confidence: confidence.clamp(0.0, 1.0),
        };

        let id = rule.id;
        self.inference_rules.write().push(rule);
        id
    }

    /// Perform logical inference
    pub fn infer(&self, starting_concepts: Vec<Uuid>) -> Vec<Concept> {
        let mut inferred_concepts = Vec::new();
        let rules = self.inference_rules.read();

        for concept_id in &starting_concepts {
            for rule in rules.iter() {
                if self.check_premises(rule, *concept_id) {
                    if let Some(new_concept) = self.apply_conclusion(rule, *concept_id) {
                        inferred_concepts.push(new_concept);
                        self.log_reasoning(format!("Inferred concept using rule: {}", rule.name));
                    }
                }
            }
        }

        inferred_concepts
    }

    /// Check if premises are satisfied
    fn check_premises(&self, rule: &InferenceRule, concept_id: Uuid) -> bool {
        let concept = match self.get_concept(concept_id) {
            Some(c) => c,
            None => return false,
        };

        for premise in &rule.premises {
            // Check concept pattern
            if !concept.name.contains(&premise.concept_pattern) &&
               !concept.category.contains(&premise.concept_pattern) {
                return false;
            }

            // Check relationship requirements
            if let Some(rel_type) = &premise.relationship_type {
                let rels = self.get_relationships_by_type(concept_id, rel_type);
                if rels.is_empty() {
                    return false;
                }
            }

            // Check conditions (simplified check)
            for condition in &premise.conditions {
                if !self.check_condition(&concept, condition) {
                    return false;
                }
            }
        }

        true
    }

    /// Check a condition against a concept
    fn check_condition(&self, concept: &Concept, condition: &str) -> bool {
        // Simple condition checking - can be extended
        let parts: Vec<&str> = condition.split('=').collect();
        if parts.len() != 2 {
            return false;
        }

        let attr_name = parts[0].trim();
        let expected_value = parts[1].trim();

        if let Some(value) = concept.attributes.get(attr_name) {
            match value {
                ConceptValue::String(s) => s == expected_value,
                ConceptValue::Boolean(b) => expected_value == b.to_string(),
                _ => false,
            }
        } else {
            false
        }
    }

    /// Apply conclusion from inference rule
    fn apply_conclusion(&self, rule: &InferenceRule, source_id: Uuid) -> Option<Concept> {
        if let Some(concept_name) = &rule.conclusion.inferred_concept {
            let mut attributes = HashMap::new();
            attributes.insert(
                "inferred_from".to_string(),
                ConceptValue::Concept(source_id),
            );

            let new_id = self.add_concept(
                concept_name.clone(),
                "inferred".to_string(),
                attributes,
            );

            let confidence = rule.confidence * rule.conclusion.confidence_modifier;

            // Create relationships with target concepts
            for target_id in &rule.conclusion.target_concepts {
                if let Some(rel_type) = &rule.conclusion.inferred_relationship {
                    self.add_relationship(
                        new_id,
                        *target_id,
                        rel_type.clone(),
                        confidence,
                        false,
                    );
                }
            }

            self.get_concept(new_id)
        } else {
            None
        }
    }

    /// Query the knowledge base
    pub fn query(&self, query: Query) -> QueryResult {
        let mut result_concepts = Vec::new();
        let mut result_relationships = Vec::new();
        let mut visited = HashSet::new();
        let mut inference_chain = Vec::new();

        // Start with direct matches
        let concepts = self.concepts.read();
        let initial_matches: Vec<_> = concepts.values()
            .filter(|c| self.matches_query(c, &query))
            .cloned()
            .collect();

        drop(concepts);

        for concept in initial_matches {
            if visited.insert(concept.id) {
                result_concepts.push(concept.clone());
                inference_chain.push(format!("Found: {}", concept.name));

                // Explore relationships up to specified depth
                if query.depth > 0 {
                    self.explore_related(&concept.id, query.depth, &mut visited, &mut result_concepts, &mut result_relationships, &mut inference_chain);
                }
            }
        }

        let avg_confidence = if result_concepts.is_empty() {
            0.0
        } else {
            result_concepts.iter().map(|c| c.confidence).sum::<f64>() / result_concepts.len() as f64
        };

        QueryResult {
            concepts: result_concepts,
            relationships: result_relationships,
            inference_chain,
            confidence: avg_confidence,
        }
    }

    /// Check if concept matches query
    fn matches_query(&self, concept: &Concept, query: &Query) -> bool {
        // Check name
        if let Some(name) = &query.concept_name {
            if !concept.name.contains(name) {
                return false;
            }
        }

        // Check attributes
        for (key, value) in &query.attributes {
            if let Some(concept_value) = concept.attributes.get(key) {
                if concept_value != value {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Explore related concepts
    fn explore_related(
        &self,
        concept_id: &Uuid,
        depth: usize,
        visited: &mut HashSet<Uuid>,
        result_concepts: &mut Vec<Concept>,
        result_relationships: &mut Vec<Relationship>,
        inference_chain: &mut Vec<String>,
    ) {
        if depth == 0 {
            return;
        }

        let relationships = self.get_relationships(*concept_id);

        for rel in relationships {
            result_relationships.push(rel.clone());

            let next_id = if rel.source_id == *concept_id {
                rel.target_id
            } else {
                rel.source_id
            };

            if visited.insert(next_id) {
                if let Some(concept) = self.get_concept(next_id) {
                    inference_chain.push(format!(
                        "Related via {:?}: {}",
                        rel.relationship_type, concept.name
                    ));
                    result_concepts.push(concept);
                    self.explore_related(&next_id, depth - 1, visited, result_concepts, result_relationships, inference_chain);
                }
            }
        }
    }

    /// Find path between two concepts
    pub fn find_path(&self, start_id: Uuid, end_id: Uuid) -> Option<Vec<Uuid>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();

        queue.push_back(start_id);
        visited.insert(start_id);

        while let Some(current) = queue.pop_front() {
            if current == end_id {
                // Reconstruct path
                let mut path = Vec::new();
                let mut node = end_id;

                while let Some(&prev) = parent.get(&node) {
                    path.push(node);
                    node = prev;
                }
                path.push(start_id);
                path.reverse();

                return Some(path);
            }

            let relationships = self.get_relationships(current);
            for rel in relationships {
                let next = if rel.source_id == current {
                    rel.target_id
                } else {
                    rel.source_id
                };

                if visited.insert(next) {
                    parent.insert(next, current);
                    queue.push_back(next);
                }
            }
        }

        None
    }

    /// Get all concepts in a category
    pub fn get_concepts_in_category(&self, category: &str) -> Vec<Concept> {
        self.concepts.read()
            .values()
            .filter(|c| c.category == category)
            .cloned()
            .collect()
    }

    /// Log reasoning step
    fn log_reasoning(&self, step: String) {
        let mut history = self.reasoning_history.write();
        history.push_back(step);

        while history.len() > self.max_history_size {
            history.pop_front();
        }
    }

    /// Get reasoning history
    pub fn get_reasoning_history(&self, count: usize) -> Vec<String> {
        self.reasoning_history.read()
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }

    /// Build hierarchical taxonomy
    pub fn build_taxonomy(&self, root_concept: Uuid) -> HashMap<Uuid, Vec<Uuid>> {
        let mut taxonomy = HashMap::new();
        self.build_taxonomy_recursive(root_concept, &mut taxonomy);
        taxonomy
    }

    /// Recursive taxonomy builder
    fn build_taxonomy_recursive(&self, concept_id: Uuid, taxonomy: &mut HashMap<Uuid, Vec<Uuid>>) {
        let children = self.get_relationships_by_type(concept_id, &RelationType::IsA);

        let child_ids: Vec<Uuid> = children.iter()
            .map(|r| r.target_id)
            .collect();

        taxonomy.insert(concept_id, child_ids.clone());

        for child_id in child_ids {
            self.build_taxonomy_recursive(child_id, taxonomy);
        }
    }

    /// Get concept count
    pub fn get_concept_count(&self) -> usize {
        self.concepts.read().len()
    }

    /// Get relationship count
    pub fn get_relationship_count(&self) -> usize {
        self.relationships.read().len()
    }
}

impl Default for SymbolicComputing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbolic_creation() {
        let symbolic = SymbolicComputing::new();
        assert_eq!(symbolic.get_concept_count(), 0);
    }

    #[test]
    fn test_concept_management() {
        let symbolic = SymbolicComputing::new();

        let mut attrs = HashMap::new();
        attrs.insert("color".to_string(), ConceptValue::String("red".to_string()));

        let id = symbolic.add_concept("Apple".to_string(), "Fruit".to_string(), attrs);

        let concept = symbolic.get_concept(id);
        assert!(concept.is_some());
        assert_eq!(concept.unwrap().name, "Apple");

        let by_name = symbolic.get_concept_by_name("Apple");
        assert!(by_name.is_some());
    }

    #[test]
    fn test_relationships() {
        let symbolic = SymbolicComputing::new();

        let fruit_id = symbolic.add_concept("Fruit".to_string(), "Category".to_string(), HashMap::new());
        let apple_id = symbolic.add_concept("Apple".to_string(), "Fruit".to_string(), HashMap::new());

        symbolic.add_relationship(apple_id, fruit_id, RelationType::IsA, 1.0, false);

        let rels = symbolic.get_relationships(apple_id);
        assert_eq!(rels.len(), 1);
        assert_eq!(rels[0].relationship_type, RelationType::IsA);
    }

    #[test]
    fn test_query() {
        let symbolic = SymbolicComputing::new();

        symbolic.add_concept("Dog".to_string(), "Animal".to_string(), HashMap::new());
        symbolic.add_concept("Cat".to_string(), "Animal".to_string(), HashMap::new());
        symbolic.add_concept("Car".to_string(), "Vehicle".to_string(), HashMap::new());

        let query = Query {
            concept_name: None,
            relationship_type: None,
            attributes: HashMap::new(),
            depth: 0,
        };

        let results = symbolic.query(query);
        assert_eq!(results.concepts.len(), 3);
    }

    #[test]
    fn test_path_finding() {
        let symbolic = SymbolicComputing::new();

        let a = symbolic.add_concept("A".to_string(), "test".to_string(), HashMap::new());
        let b = symbolic.add_concept("B".to_string(), "test".to_string(), HashMap::new());
        let c = symbolic.add_concept("C".to_string(), "test".to_string(), HashMap::new());

        symbolic.add_relationship(a, b, RelationType::Custom("connects".to_string()), 1.0, true);
        symbolic.add_relationship(b, c, RelationType::Custom("connects".to_string()), 1.0, true);

        let path = symbolic.find_path(a, c);
        assert!(path.is_some());
        assert_eq!(path.unwrap().len(), 3);
    }
}
