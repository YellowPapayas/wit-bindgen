use std::collections::HashMap;

/// stores what to add to the generated code
#[derive(Default, Debug, Clone)]
pub struct AnnotationResult {
    /// Derive macros to add (e.g., ["Clone", "Debug", "serde::Serialize"])
    pub derives: Vec<String>,

    /// Raw attributes to add (e.g., "#[repr(C)]", "#[inline]")
    pub attributes: Vec<String>,

    /// Field-specific attributes, indexed by field name
    /// Example: {"username" => ["#[serde(rename = \"user\")]"]}
    pub field_attributes: HashMap<String, Vec<String>>,

    /// Code to prepend to function body
    pub function_body_prefix: Vec<String>,

    /// Action to take (Continue with generation or Skip it)
    pub action: VisitAction,
}

impl AnnotationResult {
    /// Create a new empty result
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a result that continues with generation
    pub fn continue_with() -> Self {
        Self {
            action: VisitAction::Continue,
            ..Default::default()
        }
    }

    /// Create a result that skips generation
    pub fn skip() -> Self {
        Self {
            action: VisitAction::Skip,
            ..Default::default()
        }
    }

    /// Add a derive macro
    pub fn add_derive(&mut self, derive: impl Into<String>) {
        self.derives.push(derive.into());
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Add an attribute for a specific field
    pub fn add_field_attribute(&mut self, field_name: impl Into<String>, attr: impl Into<String>) {
        self.field_attributes
            .entry(field_name.into())
            .or_insert_with(Vec::new)
            .push(attr.into());
    }

    /// Add code to prepend to function body
    pub fn add_body_prefix(&mut self, code: impl Into<String>) {
        self.function_body_prefix.push(code.into());
    }

    /// Check if this result has any modifications
    pub fn is_empty(&self) -> bool {
        self.derives.is_empty()
            && self.attributes.is_empty()
            && self.field_attributes.is_empty()
            && self.function_body_prefix.is_empty()
    }
}

/// Action to take after processing annotations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitAction {
    /// Continue with default generation, applying modifications
    Continue,

    /// Skip default generation (visitor provides everything)
    Skip,
}

impl Default for VisitAction {
    fn default() -> Self {
        VisitAction::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annotation_result_basics() {
        let mut result = AnnotationResult::new();
        assert!(result.is_empty());
        assert_eq!(result.action, VisitAction::Continue);

        result.add_derive("Clone");
        result.add_derive("Debug");
        assert!(!result.is_empty());
        assert_eq!(result.derives.len(), 2);
    }

    #[test]
    fn test_field_attributes() {
        let mut result = AnnotationResult::new();
        result.add_field_attribute("username", "#[serde(rename = \"user\")]");
        result.add_field_attribute("username", "#[validate(non_empty)]");

        assert_eq!(result.field_attributes.get("username").unwrap().len(), 2);
    }

    #[test]
    fn test_visit_action() {
        let cont = AnnotationResult::continue_with();
        assert_eq!(cont.action, VisitAction::Continue);

        let skip = AnnotationResult::skip();
        assert_eq!(skip.action, VisitAction::Skip);
    }
}
