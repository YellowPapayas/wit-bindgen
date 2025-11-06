use std::collections::HashMap;

// Contribution Types (for core::Visitor trait - passed by mutable reference)

/// Contributions for type definitions (records, variants, enums, flags, resources).
/// This type is passed as a mutable reference to visitor methods.
#[derive(Default, Debug, Clone)]
pub struct TypeContribution {
    /// Derive macros to add (e.g., "Clone", "Debug", "serde::Serialize")
    pub derives: Vec<String>,

    /// Raw attributes to add (e.g., "#[repr(C)]", "#[serde(...)]")
    pub attributes: Vec<String>,
}

impl TypeContribution {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a derive macro
    pub fn add_derive(&mut self, derive: impl Into<String>) {
        self.derives.push(derive.into());
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.derives.is_empty() && self.attributes.is_empty()
    }
}

/// Contributions for field definitions within records.
#[derive(Default, Debug, Clone)]
pub struct FieldContribution {
    /// Field-level attributes (e.g., "#[serde(rename = \"foo\")]")
    pub attributes: Vec<String>,
}

impl FieldContribution {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }
}

/// Contributions for variant/enum cases.
#[derive(Default, Debug, Clone)]
pub struct VariantCaseContribution {
    /// Case-level attributes (e.g., "#[serde(rename = \"foo\")]")
    pub attributes: Vec<String>,
}

impl VariantCaseContribution {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }
}

/// Contributions for function definitions.
#[derive(Default, Debug, Clone)]
pub struct FunctionContribution {
    /// Function-level attributes (e.g., "#[inline]", "#[must_use]")
    pub attributes: Vec<String>,

    /// Code to prepend to function body
    pub body_prefix: Vec<String>,
}

impl FunctionContribution {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Add code to prepend to function body
    pub fn add_body_prefix(&mut self, code: impl Into<String>) {
        self.body_prefix.push(code.into());
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty() && self.body_prefix.is_empty()
    }
}

/// Contributions for module-level code.
#[derive(Default, Debug, Clone)]
pub struct ModuleContribution {
    /// Use statements to add
    pub use_statements: Vec<String>,

    /// Additional code to add to module
    pub additional_code: Vec<String>,
}

impl ModuleContribution {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a use statement
    pub fn add_use(&mut self, use_stmt: impl Into<String>) {
        self.use_statements.push(use_stmt.into());
    }

    /// Add code to module
    pub fn add_code(&mut self, code: impl Into<String>) {
        self.additional_code.push(code.into());
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.use_statements.is_empty() && self.additional_code.is_empty()
    }
}

// AnnotationResult (aggregate result for annotation processing)

/// Complete result from processing annotations on a WIT element.
/// This aggregates all contributions from processing annotations
/// and can be used to build the final contribution objects.
#[derive(Default, Debug, Clone)]
pub struct AnnotationResult {
    /// Type-level contributions
    pub type_contrib: TypeContribution,

    /// Field-specific contributions, indexed by field name
    pub field_contribs: HashMap<String, FieldContribution>,

    /// Case-specific contributions, indexed by case name
    pub case_contribs: HashMap<String, VariantCaseContribution>,

    /// Function-level contributions
    pub function_contrib: FunctionContribution,

    /// Module-level contributions
    pub module_contrib: ModuleContribution,

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

    /// Add a derive to the type contribution
    pub fn add_derive(&mut self, derive: impl Into<String>) {
        self.type_contrib.add_derive(derive);
    }

    /// Add an attribute to the type contribution
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.type_contrib.add_attribute(attr);
    }

    /// Add an attribute for a specific field
    pub fn add_field_attribute(&mut self, field_name: impl Into<String>, attr: impl Into<String>) {
        self.field_contribs
            .entry(field_name.into())
            .or_insert_with(FieldContribution::new)
            .add_attribute(attr);
    }

    /// Add an attribute for a specific case
    pub fn add_case_attribute(&mut self, case_name: impl Into<String>, attr: impl Into<String>) {
        self.case_contribs
            .entry(case_name.into())
            .or_insert_with(VariantCaseContribution::new)
            .add_attribute(attr);
    }

    /// Add code to prepend to function body
    pub fn add_body_prefix(&mut self, code: impl Into<String>) {
        self.function_contrib.add_body_prefix(code);
    }

    /// Add a function attribute
    pub fn add_function_attribute(&mut self, attr: impl Into<String>) {
        self.function_contrib.add_attribute(attr);
    }

    /// Check if this result has any modifications
    pub fn is_empty(&self) -> bool {
        self.type_contrib.is_empty()
            && self.field_contribs.is_empty()
            && self.case_contribs.is_empty()
            && self.function_contrib.is_empty()
            && self.module_contrib.is_empty()
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
    fn test_type_contribution() {
        let mut contrib = TypeContribution::new();
        assert!(contrib.is_empty());

        contrib.add_derive("Clone");
        contrib.add_attribute("#[repr(C)]");
        assert!(!contrib.is_empty());

        assert_eq!(contrib.derives.len(), 1);
        assert_eq!(contrib.attributes.len(), 1);
    }

    #[test]
    fn test_field_contribution() {
        let mut contrib = FieldContribution::new();
        assert!(contrib.is_empty());

        contrib.add_attribute("#[serde(rename = \"foo\")]");
        assert!(!contrib.is_empty());
        assert_eq!(contrib.attributes.len(), 1);
    }

    #[test]
    fn test_annotation_result() {
        let mut result = AnnotationResult::new();
        assert!(result.is_empty());
        assert_eq!(result.action, VisitAction::Continue);

        result.add_derive("Clone");
        result.add_field_attribute("username", "#[serde(rename = \"user\")]");
        assert!(!result.is_empty());

        assert_eq!(result.type_contrib.derives.len(), 1);
        assert_eq!(result.field_contribs.get("username").unwrap().attributes.len(), 1);
    }

    #[test]
    fn test_visit_action() {
        let cont = AnnotationResult::continue_with();
        assert_eq!(cont.action, VisitAction::Continue);

        let skip = AnnotationResult::skip();
        assert_eq!(skip.action, VisitAction::Skip);
    }

    #[test]
    fn test_function_contribution() {
        let mut contrib = FunctionContribution::new();
        assert!(contrib.is_empty());

        contrib.add_attribute("#[inline]");
        contrib.add_body_prefix("println!(\"start\");");
        assert!(!contrib.is_empty());

        assert_eq!(contrib.attributes.len(), 1);
        assert_eq!(contrib.body_prefix.len(), 1);
    }
}
