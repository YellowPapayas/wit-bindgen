//! Contribution APIs for visitor pattern
//!
//! This module provides builder-style APIs for adding attributes, derives,
//! and code snippets to generated elements during WIT code generation.
//!
//! These types are passed as mutable references to visitor methods, allowing
//! visitors to contribute modifications without needing to return complex values.

/// Contributions for type definitions (records, variants, enums, flags, resources).
///
/// # Example
/// ```ignore
/// fn augment_record(&mut self, ctx: &RecordContext, contrib: &mut TypeContribution) {
///     contrib.add_derive("serde::Serialize");
///     contrib.add_derive("serde::Deserialize");
///     contrib.add_attribute("#[serde(rename_all = \"camelCase\")]");
///     contrib.add_doc_comment("This type is serializable");
/// }
/// ```
#[derive(Default, Debug, Clone)]
pub struct TypeContribution {
    /// Raw attribute lines to add (e.g., "#[serde(rename_all = \"camelCase\")]")
    pub(crate) attributes: Vec<String>,

    /// Derive macros to add (e.g., "serde::Serialize", "Clone", "Debug")
    pub(crate) derives: Vec<String>,

    /// Doc comment lines to add (without the "///")
    pub(crate) doc_comments: Vec<String>,

    /// Additional code to add after the type definition (e.g., impl blocks, trait impls)
    pub(crate) additional_code: Vec<String>,
}

impl TypeContribution {
    /// Create a new empty TypeContribution
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a raw attribute line (e.g., "#[serde(rename_all = \"camelCase\")]").
    ///
    /// The attribute should include the full `#[...]` syntax.
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_attribute("#[non_exhaustive]");
    /// contrib.add_attribute("#[serde(rename_all = \"camelCase\")]");
    /// ```
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Add a derive macro (e.g., "serde::Serialize", "Clone").
    ///
    /// The derive will be added to the `#[derive(...)]` attribute.
    /// Do not include "derive" or brackets - just the trait name.
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_derive("serde::Serialize");
    /// contrib.add_derive("Clone");
    /// contrib.add_derive("Debug");
    /// // Generates: #[derive(serde::Serialize, Clone, Debug)]
    /// ```
    pub fn add_derive(&mut self, derive: impl Into<String>) {
        self.derives.push(derive.into());
    }

    /// Add a doc comment line (without the "///").
    ///
    /// Each call adds a single line. The `///` prefix will be added automatically.
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_doc_comment("This is a custom type.");
    /// contrib.add_doc_comment("It has special properties.");
    /// // Generates:
    /// // /// This is a custom type.
    /// // /// It has special properties.
    /// ```
    pub fn add_doc_comment(&mut self, comment: impl Into<String>) {
        self.doc_comments.push(comment.into());
    }

    /// Add additional code after the type definition.
    ///
    /// This is useful for adding impl blocks, trait implementations, or other
    /// related code that should appear near the type definition.
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_code(format!(
    ///     "impl {} {{\n    pub fn new() -> Self {{ Self::default() }}\n}}",
    ///     ctx.type_name
    /// ));
    /// ```
    pub fn add_code(&mut self, code: impl Into<String>) {
        self.additional_code.push(code.into());
    }

    /// Get all attributes
    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }

    /// Get all derives
    pub fn derives(&self) -> &[String] {
        &self.derives
    }

    /// Get all doc comments
    pub fn doc_comments(&self) -> &[String] {
        &self.doc_comments
    }

    /// Get all additional code
    pub fn additional_code(&self) -> &[String] {
        &self.additional_code
    }

    /// Check if this contribution has any modifications
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
            && self.derives.is_empty()
            && self.doc_comments.is_empty()
            && self.additional_code.is_empty()
    }
}

/// Contributions for field definitions within records.
///
/// # Example
/// ```ignore
/// fn augment_field(&mut self, ctx: &FieldContext, contrib: &mut FieldContribution) {
///     if ctx.field.name == "user-id" {
///         contrib.add_attribute("#[serde(rename = \"userId\")]");
///         contrib.add_doc_comment("The unique user identifier");
///     }
/// }
/// ```
#[derive(Default, Debug, Clone)]
pub struct FieldContribution {
    /// Field-level attributes (e.g., "#[serde(skip)]")
    pub(crate) attributes: Vec<String>,

    /// Doc comment lines for the field
    pub(crate) doc_comments: Vec<String>,
}

impl FieldContribution {
    /// Create a new empty FieldContribution
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a field-level attribute (e.g., "#[serde(skip)]").
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_attribute("#[serde(skip)]");
    /// contrib.add_attribute("#[serde(rename = \"userId\")]");
    /// contrib.add_attribute("#[deprecated]");
    /// ```
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Add a doc comment for the field.
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_doc_comment("The user's email address");
    /// contrib.add_doc_comment("Must be a valid email format");
    /// ```
    pub fn add_doc_comment(&mut self, comment: impl Into<String>) {
        self.doc_comments.push(comment.into());
    }

    /// Get all attributes
    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }

    /// Get all doc comments
    pub fn doc_comments(&self) -> &[String] {
        &self.doc_comments
    }

    /// Check if this contribution has any modifications
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty() && self.doc_comments.is_empty()
    }
}

/// Contributions for variant cases (enum variants).
///
/// # Example
/// ```ignore
/// fn augment_variant_case(&mut self, ctx: &VariantCaseContext, contrib: &mut VariantCaseContribution) {
///     if ctx.case.name == "error" {
///         contrib.add_attribute("#[serde(rename = \"err\")]");
///         contrib.add_doc_comment("An error occurred");
///     }
/// }
/// ```
#[derive(Default, Debug, Clone)]
pub struct VariantCaseContribution {
    /// Variant-level attributes (e.g., "#[serde(rename = \"foo\")]")
    pub(crate) attributes: Vec<String>,

    /// Doc comment lines for the variant
    pub(crate) doc_comments: Vec<String>,
}

impl VariantCaseContribution {
    /// Create a new empty VariantCaseContribution
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a variant-level attribute (e.g., "#[serde(rename = \"foo\")]").
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_attribute("#[serde(rename = \"success\")]");
    /// contrib.add_attribute("#[deprecated(since = \"1.0.0\")]");
    /// ```
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Add a doc comment for the variant.
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_doc_comment("Represents a successful operation");
    /// ```
    pub fn add_doc_comment(&mut self, comment: impl Into<String>) {
        self.doc_comments.push(comment.into());
    }

    /// Get all attributes
    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }

    /// Get all doc comments
    pub fn doc_comments(&self) -> &[String] {
        &self.doc_comments
    }

    /// Check if this contribution has any modifications
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty() && self.doc_comments.is_empty()
    }
}

/// Contributions for function definitions.
///
/// # Example
/// ```ignore
/// fn augment_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {
///     if ctx.direction == Direction::Export {
///         contrib.add_attribute("#[tracing::instrument]");
///     }
///
///     contrib.prepend_body("let _start = std::time::Instant::now();");
///     contrib.append_body("println!(\"Duration: {:?}\", _start.elapsed());");
/// }
/// ```
#[derive(Default, Debug, Clone)]
pub struct FunctionContribution {
    /// Function-level attributes (e.g., "#[inline]", "#[tracing::instrument]")
    pub(crate) attributes: Vec<String>,

    /// Doc comment lines for the function
    pub(crate) doc_comments: Vec<String>,

    /// Code to prepend to the function body (runs before generated code)
    pub(crate) body_prefix: Vec<String>,

    /// Code to append to the function body (runs after generated code, before return)
    pub(crate) body_suffix: Vec<String>,
}

impl FunctionContribution {
    /// Create a new empty FunctionContribution
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a function-level attribute (e.g., "#[inline]", "#[tracing::instrument]").
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_attribute("#[inline]");
    /// contrib.add_attribute("#[tracing::instrument]");
    /// contrib.add_attribute("#[must_use]");
    /// ```
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Add a doc comment for the function.
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_doc_comment("Performs an important operation");
    /// contrib.add_doc_comment("");
    /// contrib.add_doc_comment("# Errors");
    /// contrib.add_doc_comment("Returns an error if the operation fails");
    /// ```
    pub fn add_doc_comment(&mut self, comment: impl Into<String>) {
        self.doc_comments.push(comment.into());
    }

    /// Prepend code to the function body (runs before generated code).
    ///
    /// This is useful for adding logging, tracing, or validation that should
    /// happen at the start of the function.
    ///
    /// # Example
    /// ```ignore
    /// contrib.prepend_body("let _start = std::time::Instant::now();");
    /// contrib.prepend_body("tracing::debug!(\"Function called\");");
    /// ```
    pub fn prepend_body(&mut self, code: impl Into<String>) {
        self.body_prefix.push(code.into());
    }

    /// Append code to the function body (runs after generated code, before return).
    ///
    /// This is useful for adding cleanup, logging, or post-processing.
    ///
    /// # Example
    /// ```ignore
    /// contrib.append_body("println!(\"Duration: {:?}\", _start.elapsed());");
    /// contrib.append_body("tracing::debug!(\"Function completed\");");
    /// ```
    pub fn append_body(&mut self, code: impl Into<String>) {
        self.body_suffix.push(code.into());
    }

    /// Get all attributes
    pub fn attributes(&self) -> &[String] {
        &self.attributes
    }

    /// Get all doc comments
    pub fn doc_comments(&self) -> &[String] {
        &self.doc_comments
    }

    /// Get all body prefix code
    pub fn body_prefix(&self) -> &[String] {
        &self.body_prefix
    }

    /// Get all body suffix code
    pub fn body_suffix(&self) -> &[String] {
        &self.body_suffix
    }

    /// Check if this contribution has any modifications
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
            && self.doc_comments.is_empty()
            && self.body_prefix.is_empty()
            && self.body_suffix.is_empty()
    }
}

/// Contributions for module-level code.
///
/// # Example
/// ```ignore
/// fn after_interface(&mut self, ctx: &InterfaceContext, contrib: &mut ModuleContribution) {
///     contrib.add_use("use serde::{Serialize, Deserialize};");
///     contrib.add_code(r#"
///         pub fn helper_function() {
///             // Custom helper for this module
///         }
///     "#);
/// }
/// ```
#[derive(Default, Debug, Clone)]
pub struct ModuleContribution {
    /// Additional code to add to the module
    pub(crate) additional_code: Vec<String>,

    /// Use statements to add to the module
    pub(crate) use_statements: Vec<String>,
}

impl ModuleContribution {
    /// Create a new empty ModuleContribution
    pub fn new() -> Self {
        Self::default()
    }

    /// Add additional code to the module.
    ///
    /// This code will be added at the module level, typically after type and
    /// function definitions.
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_code("pub const MODULE_VERSION: &str = \"1.0.0\";");
    /// contrib.add_code(r#"
    ///     pub fn helper_function() {
    ///         println!("Helper");
    ///     }
    /// "#);
    /// ```
    pub fn add_code(&mut self, code: impl Into<String>) {
        self.additional_code.push(code.into());
    }

    /// Add a use statement to the module.
    ///
    /// Use statements will be added near the top of the module, after any
    /// module-level attributes or doc comments.
    ///
    /// # Example
    /// ```ignore
    /// contrib.add_use("use serde::{Serialize, Deserialize};");
    /// contrib.add_use("use std::collections::HashMap;");
    /// ```
    pub fn add_use(&mut self, use_stmt: impl Into<String>) {
        self.use_statements.push(use_stmt.into());
    }

    /// Get all additional code
    pub fn additional_code(&self) -> &[String] {
        &self.additional_code
    }

    /// Get all use statements
    pub fn use_statements(&self) -> &[String] {
        &self.use_statements
    }

    /// Check if this contribution has any modifications
    pub fn is_empty(&self) -> bool {
        self.additional_code.is_empty() && self.use_statements.is_empty()
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
        contrib.add_derive("Debug");
        contrib.add_attribute("#[non_exhaustive]");
        contrib.add_doc_comment("This is a test type");
        contrib.add_code("impl TestType { }");

        assert!(!contrib.is_empty());
        assert_eq!(contrib.derives().len(), 2);
        assert_eq!(contrib.attributes().len(), 1);
        assert_eq!(contrib.doc_comments().len(), 1);
        assert_eq!(contrib.additional_code().len(), 1);
    }

    #[test]
    fn test_field_contribution() {
        let mut contrib = FieldContribution::new();
        assert!(contrib.is_empty());

        contrib.add_attribute("#[serde(skip)]");
        contrib.add_doc_comment("A field");

        assert!(!contrib.is_empty());
        assert_eq!(contrib.attributes().len(), 1);
        assert_eq!(contrib.doc_comments().len(), 1);
    }

    #[test]
    fn test_function_contribution() {
        let mut contrib = FunctionContribution::new();
        assert!(contrib.is_empty());

        contrib.add_attribute("#[inline]");
        contrib.prepend_body("let x = 1;");
        contrib.append_body("println!(\"done\");");

        assert!(!contrib.is_empty());
        assert_eq!(contrib.attributes().len(), 1);
        assert_eq!(contrib.body_prefix().len(), 1);
        assert_eq!(contrib.body_suffix().len(), 1);
    }

    #[test]
    fn test_module_contribution() {
        let mut contrib = ModuleContribution::new();
        assert!(contrib.is_empty());

        contrib.add_use("use std::collections::HashMap;");
        contrib.add_code("pub const VERSION: u32 = 1;");

        assert!(!contrib.is_empty());
        assert_eq!(contrib.use_statements().len(), 1);
        assert_eq!(contrib.additional_code().len(), 1);
    }
}
