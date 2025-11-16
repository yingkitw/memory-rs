//! Prompt templates for LLM operations

use std::collections::HashMap;

/// Prompt template for memory operations
#[derive(Debug, Clone)]
pub struct PromptTemplate {
    /// Template name
    pub name: String,
    /// Template content with placeholders
    pub template: String,
    /// Variables in template
    pub variables: Vec<String>,
}

impl PromptTemplate {
    /// Create a new prompt template
    pub fn new(name: String, template: String) -> Self {
        let variables = Self::extract_variables(&template);
        Self {
            name,
            template,
            variables,
        }
    }

    /// Extract variables from template (format: {variable_name})
    fn extract_variables(template: &str) -> Vec<String> {
        let mut vars = Vec::new();
        let mut in_var = false;
        let mut current_var = String::new();

        for ch in template.chars() {
            match ch {
                '{' => in_var = true,
                '}' => {
                    if in_var && !current_var.is_empty() {
                        vars.push(current_var.clone());
                        current_var.clear();
                    }
                    in_var = false;
                }
                _ if in_var => current_var.push(ch),
                _ => {}
            }
        }

        vars
    }

    /// Render template with variables
    pub fn render(&self, variables: &HashMap<String, String>) -> Result<String, String> {
        let mut result = self.template.clone();

        for var in &self.variables {
            let value = variables
                .get(var)
                .ok_or_else(|| format!("Missing variable: {}", var))?;
            result = result.replace(&format!("{{{}}}", var), value);
        }

        Ok(result)
    }
}

/// Prompt manager
pub struct PromptManager {
    templates: HashMap<String, PromptTemplate>,
}

impl PromptManager {
    /// Create a new prompt manager with default templates
    pub fn new() -> Self {
        let mut manager = Self {
            templates: HashMap::new(),
        };
        manager.load_defaults();
        manager
    }

    /// Load default templates
    fn load_defaults(&mut self) {
        // Extract facts from conversation
        self.register(PromptTemplate::new(
            "extract_facts".to_string(),
            "Extract key facts from the following conversation:\n\n{conversation}\n\nFacts:".to_string(),
        ));

        // Generate insights
        self.register(PromptTemplate::new(
            "generate_insights".to_string(),
            "Based on the following facts about the user:\n\n{facts}\n\nGenerate insights about their preferences and behavior:\n\nInsights:".to_string(),
        ));

        // Summarize memories
        self.register(PromptTemplate::new(
            "summarize_memories".to_string(),
            "Summarize the following memories into a concise profile:\n\n{memories}\n\nSummary:".to_string(),
        ));

        // Answer question with context
        self.register(PromptTemplate::new(
            "answer_with_context".to_string(),
            "Answer the following question based on the provided context:\n\nContext:\n{context}\n\nQuestion: {question}\n\nAnswer:".to_string(),
        ));

        // Classify memory type
        self.register(PromptTemplate::new(
            "classify_memory".to_string(),
            "Classify the following text into one of these categories: fact, preference, insight, goal, or other.\n\nText: {text}\n\nCategory:".to_string(),
        ));
    }

    /// Register a template
    pub fn register(&mut self, template: PromptTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    /// Get a template
    pub fn get(&self, name: &str) -> Option<&PromptTemplate> {
        self.templates.get(name)
    }

    /// Render a template
    pub fn render(&self, name: &str, variables: &HashMap<String, String>) -> Result<String, String> {
        let template = self.get(name)
            .ok_or_else(|| format!("Template not found: {}", name))?;
        template.render(variables)
    }

    /// List all template names
    pub fn list_templates(&self) -> Vec<&str> {
        self.templates.keys().map(|k| k.as_str()).collect()
    }

    /// Get template count
    pub fn template_count(&self) -> usize {
        self.templates.len()
    }
}

impl Default for PromptManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_variables() {
        let template = "Hello {name}, you are {age} years old";
        let vars = PromptTemplate::extract_variables(template);
        assert_eq!(vars, vec!["name", "age"]);
    }

    #[test]
    fn test_render_template() {
        let template = PromptTemplate::new(
            "greeting".to_string(),
            "Hello {name}, welcome to {place}".to_string(),
        );

        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Alice".to_string());
        vars.insert("place".to_string(), "Rust".to_string());

        let result = template.render(&vars).unwrap();
        assert_eq!(result, "Hello Alice, welcome to Rust");
    }

    #[test]
    fn test_render_missing_variable() {
        let template = PromptTemplate::new(
            "greeting".to_string(),
            "Hello {name}".to_string(),
        );

        let vars = HashMap::new();
        assert!(template.render(&vars).is_err());
    }

    #[test]
    fn test_prompt_manager() {
        let manager = PromptManager::new();
        assert!(manager.template_count() > 0);
        assert!(manager.get("extract_facts").is_some());
    }

    #[test]
    fn test_render_from_manager() {
        let manager = PromptManager::new();
        let mut vars = HashMap::new();
        vars.insert("conversation".to_string(), "User: I like coffee".to_string());

        let result = manager.render("extract_facts", &vars);
        assert!(result.is_ok());
    }
}
