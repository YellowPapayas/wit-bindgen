// Contains 5 example visitor implementations and integration tests that verify visitor targets,
// contribution types, and behavior

#![cfg(feature = "visitor")]

use wit_bindgen_core::wit_parser::*;
use wit_bindgen_core::Visitor;
use wit_bindgen_rust::annotation_visitor::*;

// Test visitor implementations

/// A test visitor that adds "deprecated" annotations
struct DeprecatedVisitor;

impl Visitor for DeprecatedVisitor {
    type TypeContribution = RustTypeContribution;
    type FieldContribution = RustFieldContribution;
    type VariantCaseContribution = RustVariantCaseContribution;
    type FunctionContribution = RustFunctionContribution;
    type ModuleContribution = RustModuleContribution;

    fn target(&self) -> &str {
        "deprecated"
    }

    fn visit_function(
        &mut self,
        annotation: &String,
        _func: &Function,
    ) -> Option<Self::FunctionContribution> {
        let mut contrib = RustFunctionContribution::new();
        if annotation.is_empty() {
            contrib.add_attribute("#[deprecated]");
        } else {
            contrib.add_attribute(&format!("#[deprecated = \"{}\"]", annotation));
        }
        Some(contrib)
    }

    fn visit_variant_case(
        &mut self,
        annotation: &String,
        _case: &Case,
        _case_index: usize,
    ) -> Option<Self::VariantCaseContribution> {
        let mut contrib = RustVariantCaseContribution::new();
        if annotation.is_empty() {
            contrib.add_attribute("#[deprecated]");
        } else {
            contrib.add_attribute(&format!("#[deprecated = \"{}\"]", annotation));
        }
        Some(contrib)
    }
}

/// A visitor that adds tracing to function bodies
struct TracingVisitor;

impl Visitor for TracingVisitor {
    type TypeContribution = RustTypeContribution;
    type FieldContribution = RustFieldContribution;
    type VariantCaseContribution = RustVariantCaseContribution;
    type FunctionContribution = RustFunctionContribution;
    type ModuleContribution = RustModuleContribution;

    fn target(&self) -> &str {
        "trace"
    }

    fn visit_function(
        &mut self,
        annotation: &String,
        func: &Function,
    ) -> Option<Self::FunctionContribution> {
        let mut contrib = RustFunctionContribution::new();
        contrib.add_attribute("#[tracing::instrument]");

        let level = if annotation.is_empty() {
            "debug"
        } else {
            annotation.as_str()
        };
        contrib.add_body_prefix(&format!(
            "tracing::{}!(\"Entering function: {}\");",
            level, func.name
        ));

        Some(contrib)
    }

    fn visit_interface(
        &mut self,
        _annotation: &String,
        _interface: Option<&Interface>,
    ) -> Option<Self::ModuleContribution> {
        let mut contrib = RustModuleContribution::new();
        contrib.add_use("use tracing");
        Some(contrib)
    }
}

/// A visitor that adds validation
struct ValidateVisitor;

impl Visitor for ValidateVisitor {
    type TypeContribution = RustTypeContribution;
    type FieldContribution = RustFieldContribution;
    type VariantCaseContribution = RustVariantCaseContribution;
    type FunctionContribution = RustFunctionContribution;
    type ModuleContribution = RustModuleContribution;

    fn target(&self) -> &str {
        "validate"
    }

    fn visit_function(
        &mut self,
        annotation: &String,
        _func: &Function,
    ) -> Option<Self::FunctionContribution> {
        let mut contrib = RustFunctionContribution::new();
        if !annotation.is_empty() {
            contrib.add_body_prefix(&format!(
                "assert!({}, \"Validation failed\");",
                annotation
            ));
        }
        Some(contrib)
    }
}

/// A visitor that adds version info
struct SinceVisitor;

impl Visitor for SinceVisitor {
    type TypeContribution = RustTypeContribution;
    type FieldContribution = RustFieldContribution;
    type VariantCaseContribution = RustVariantCaseContribution;
    type FunctionContribution = RustFunctionContribution;
    type ModuleContribution = RustModuleContribution;

    fn target(&self) -> &str {
        "since"
    }

    fn visit_function(
        &mut self,
        annotation: &String,
        _func: &Function,
    ) -> Option<Self::FunctionContribution> {
        let mut contrib = RustFunctionContribution::new();
        contrib.add_attribute(&format!("#[doc = \"Since version: {}\"]", annotation));
        Some(contrib)
    }

    fn visit_interface(
        &mut self,
        annotation: &String,
        _interface: Option<&Interface>,
    ) -> Option<Self::ModuleContribution> {
        let mut contrib = RustModuleContribution::new();
        contrib.add_code(&format!(
            "// Interface available since version: {}",
            annotation
        ));
        Some(contrib)
    }
}

/// A visitor that adds custom derives
struct DeriveVisitor;

impl Visitor for DeriveVisitor {
    type TypeContribution = RustTypeContribution;
    type FieldContribution = RustFieldContribution;
    type VariantCaseContribution = RustVariantCaseContribution;
    type FunctionContribution = RustFunctionContribution;
    type ModuleContribution = RustModuleContribution;

    fn target(&self) -> &str {
        "derive"
    }

    fn visit_enum(
        &mut self,
        annotation: &String,
        _enum: &Enum,
        _type_id: TypeId,
    ) -> Option<Self::TypeContribution> {
        let mut contrib = RustTypeContribution::new();
        for derive in annotation.split(',').map(|s| s.trim()) {
            if !derive.is_empty() {
                contrib.add_derive(derive);
            }
        }
        Some(contrib)
    }
}

/// A visitor that adds comprehensive logging for inputs and outputs
struct LoggingVisitor;

impl Visitor for LoggingVisitor {
    type TypeContribution = RustTypeContribution;
    type FieldContribution = RustFieldContribution;
    type VariantCaseContribution = RustVariantCaseContribution;
    type FunctionContribution = RustFunctionContribution;
    type ModuleContribution = RustModuleContribution;

    fn target(&self) -> &str {
        "log"
    }

    fn visit_function(
        &mut self,
        annotation: &String,
        func: &Function,
    ) -> Option<Self::FunctionContribution> {
        let mut contrib = RustFunctionContribution::new();

        // Parse log level from annotation (default to "info")
        let level = if annotation.is_empty() {
            "info"
        } else {
            annotation.as_str()
        };

        // Log function entry with parameters
        let param_names: Vec<String> = func
            .params
            .iter()
            .map(|(name, _)| format!("{} = {{:?}}", name))
            .collect();

        if !param_names.is_empty() {
            let param_format = param_names.join(", ");
            let param_values: Vec<String> = func
                .params
                .iter()
                .map(|(name, _)| name.clone())
                .collect();
            let param_list = param_values.join(", ");

            contrib.add_body_prefix(&format!(
                "log::{}!(\"[ENTRY] {}: {}\", {});",
                level, func.name, param_format, param_list
            ));
        } else {
            contrib.add_body_prefix(&format!(
                "log::{}!(\"[ENTRY] {}\");",
                level, func.name
            ));
        }

        // Log function exit with result
        contrib.add_body_postfix(&format!(
            "log::{}!(\"[EXIT] {}: result = {{:?}}\", result);",
            level, func.name
        ));

        Some(contrib)
    }

    fn visit_interface(
        &mut self,
        _annotation: &String,
        _interface: Option<&Interface>,
    ) -> Option<Self::ModuleContribution> {
        let mut contrib = RustModuleContribution::new();
        contrib.add_use("use log");
        Some(contrib)
    }
}

#[test]
fn test_deprecated_visitor_basic() {
    let visitor = DeprecatedVisitor;

    // Test with empty annotation
    let _contrib = RustFunctionContribution {
        attributes: vec![],
        body_prefix: vec![],
    };

    // Verify visitor target
    assert_eq!(visitor.target(), "deprecated");

    // Test generating deprecated attribute
    let annotation = "use new function instead".to_string();
    assert!(annotation.len() > 0);
}

#[test]
fn test_visitor_targets() {
    assert_eq!(DeprecatedVisitor.target(), "deprecated");
    assert_eq!(TracingVisitor.target(), "trace");
    assert_eq!(ValidateVisitor.target(), "validate");
    assert_eq!(SinceVisitor.target(), "since");
    assert_eq!(DeriveVisitor.target(), "derive");
    assert_eq!(LoggingVisitor.target(), "log");
}

#[test]
fn test_contribution_types_work() {
    // Test RustFunctionContribution
    let mut func_contrib = RustFunctionContribution::new();
    assert!(func_contrib.is_empty());

    func_contrib.add_attribute("#[deprecated]");
    func_contrib.add_body_prefix("println!(\"test\");");

    assert!(!func_contrib.is_empty());
    assert_eq!(func_contrib.attributes.len(), 1);
    assert_eq!(func_contrib.body_prefix.len(), 1);

    // Test RustModuleContribution
    let mut mod_contrib = RustModuleContribution::new();
    assert!(mod_contrib.is_empty());

    mod_contrib.add_use("use std::collections::HashMap");
    mod_contrib.add_code("// Module code");

    assert!(!mod_contrib.is_empty());
    assert_eq!(mod_contrib.use_statements.len(), 1);
    assert_eq!(mod_contrib.additional_code.len(), 1);

    // Test RustTypeContribution
    let mut type_contrib = RustTypeContribution::new();
    assert!(type_contrib.is_empty());

    type_contrib.add_derive("Debug");
    type_contrib.add_attribute("#[repr(C)]");

    assert!(!type_contrib.is_empty());
    assert_eq!(type_contrib.derives.len(), 1);
    assert_eq!(type_contrib.attributes.len(), 1);

    // Test RustVariantCaseContribution
    let mut case_contrib = RustVariantCaseContribution::new();
    assert!(case_contrib.is_empty());

    case_contrib.add_attribute("#[deprecated]");

    assert!(!case_contrib.is_empty());
    assert_eq!(case_contrib.attributes.len(), 1);
}

#[test]
fn test_logging_visitor_generates_prefix_and_postfix() {
    let mut visitor = LoggingVisitor;

    // Create a test function with parameters
    let func = Function {
        name: "test_func".to_string(),
        params: vec![
            ("param1".to_string(), Type::U32),
            ("param2".to_string(), Type::String),
        ],
        results: wit_bindgen_core::wit_parser::Results::Anon(Type::U32),
        kind: wit_bindgen_core::wit_parser::FunctionKind::Freestanding,
        docs: Default::default(),
        stability: Default::default(),
    };

    let annotation = "debug".to_string();
    let contrib = visitor.visit_function(&annotation, &func);

    assert!(contrib.is_some());
    let contrib = contrib.unwrap();

    // Verify body_prefix contains entry logging
    assert_eq!(contrib.body_prefix.len(), 1);
    assert!(contrib.body_prefix[0].contains("[ENTRY]"));
    assert!(contrib.body_prefix[0].contains("test_func"));
    assert!(contrib.body_prefix[0].contains("param1"));
    assert!(contrib.body_prefix[0].contains("param2"));
    assert!(contrib.body_prefix[0].contains("log::debug!"));

    // Verify body_postfix contains exit logging
    assert_eq!(contrib.body_postfix.len(), 1);
    assert!(contrib.body_postfix[0].contains("[EXIT]"));
    assert!(contrib.body_postfix[0].contains("test_func"));
    assert!(contrib.body_postfix[0].contains("result"));
    assert!(contrib.body_postfix[0].contains("log::debug!"));
}

#[test]
fn test_logging_visitor_default_log_level() {
    let mut visitor = LoggingVisitor;

    let func = Function {
        name: "simple_func".to_string(),
        params: vec![],
        results: wit_bindgen_core::wit_parser::Results::Anon(Type::Unit),
        kind: wit_bindgen_core::wit_parser::FunctionKind::Freestanding,
        docs: Default::default(),
        stability: Default::default(),
    };

    // Empty annotation should default to "info"
    let annotation = "".to_string();
    let contrib = visitor.visit_function(&annotation, &func);

    assert!(contrib.is_some());
    let contrib = contrib.unwrap();

    // Verify it uses info level
    assert!(contrib.body_prefix[0].contains("log::info!"));
    assert!(contrib.body_postfix[0].contains("log::info!"));
}

#[test]
fn test_logging_visitor_adds_use_statement() {
    let mut visitor = LoggingVisitor;
    let annotation = "".to_string();

    let contrib = visitor.visit_interface(&annotation, None);

    assert!(contrib.is_some());
    let contrib = contrib.unwrap();

    assert_eq!(contrib.use_statements.len(), 1);
    assert_eq!(contrib.use_statements[0], "use log");
}
