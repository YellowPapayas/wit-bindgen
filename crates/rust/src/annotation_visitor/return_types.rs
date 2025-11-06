//! Contribution types for WIT visitor pattern
//!
//! These types are used with the core::Visitor trait to customize
//! generated Rust code. They are passed as mutable references to
//! visitor methods.

/// Contributions for type definitions (records, variants, enums, flags, resources).
///
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
    fn test_variant_case_contribution() {
        let mut contrib = VariantCaseContribution::new();
        assert!(contrib.is_empty());

        contrib.add_attribute("#[serde(rename = \"bar\")]");
        assert!(!contrib.is_empty());
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

    #[test]
    fn test_module_contribution() {
        let mut contrib = ModuleContribution::new();
        assert!(contrib.is_empty());

        contrib.add_use("use std::collections::HashMap;");
        contrib.add_code("pub const VERSION: u32 = 1;");
        assert!(!contrib.is_empty());
    }
}
