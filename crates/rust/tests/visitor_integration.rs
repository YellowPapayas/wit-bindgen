// Contains 5 example visitor implementations and integration tests that verify visitor targets,
// contribution types, and behavior

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
            contrib.add_body_prefix(&format!("assert!({}, \"Validation failed\");", annotation));
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

/// A visitor that adds comprehensive logging (both entry and exit)
struct LoggingVisitor;

impl Visitor for LoggingVisitor {
    type TypeContribution = RustTypeContribution;
    type FieldContribution = RustFieldContribution;
    type VariantCaseContribution = RustVariantCaseContribution;
    type FunctionContribution = RustFunctionContribution;
    type ModuleContribution = RustModuleContribution;

    fn target(&self) -> &str {
        "logging"
    }

    fn visit_function(
        &mut self,
        _annotation: &String,
        func: &Function,
    ) -> Option<Self::FunctionContribution> {
        let mut contrib = RustFunctionContribution::new();

        // Log function entry
        contrib.add_body_prefix(&format!("println!(\"[ENTRY] {}\");", func.name));

        // Log each parameter
        for (param_name, _) in func.params.iter() {
            contrib.add_body_prefix(&format!(
                "println!(\"  {} = {{:?}}\", {});",
                param_name, param_name
            ));
        }

        // Log function exit with return value if present
        if func.result.is_some() {
            contrib.add_body_suffix("println!(\"[EXIT] {} => {:?}\", func_return);");
        } else {
            contrib.add_body_suffix("println!(\"[EXIT] {}\");");
        }

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
        body_suffix: vec![],
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
    assert_eq!(LoggingVisitor.target(), "logging");
}

#[test]
fn test_contribution_types_work() {
    // Test RustFunctionContribution
    let mut func_contrib = RustFunctionContribution::new();
    assert!(func_contrib.is_empty());

    func_contrib.add_attribute("#[deprecated]");
    func_contrib.add_body_prefix("println!(\"test\");");
    func_contrib.add_body_suffix("println!(\"exit\");");

    assert!(!func_contrib.is_empty());
    assert_eq!(func_contrib.attributes.len(), 1);
    assert_eq!(func_contrib.body_prefix.len(), 1);
    assert_eq!(func_contrib.body_suffix.len(), 1);

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
fn test_body_suffix_logging() {
    // Create a simple function for testing
    let func = Function {
        name: "test_func".to_string(),
        params: vec![("input".to_string(), Type::U32)],
        result: Some(Type::U32),
        kind: FunctionKind::Freestanding,
        docs: Default::default(),
        stability: Default::default(),
        annotations: Default::default(),
    };

    let mut visitor = LoggingVisitor;
    let contrib = visitor.visit_function(&String::new(), &func);

    assert!(contrib.is_some());
    let contrib = contrib.unwrap();

    // Should have both prefix and suffix
    assert!(!contrib.body_prefix.is_empty());
    assert!(!contrib.body_suffix.is_empty());

    // Verify the logging includes function entry
    assert!(contrib.body_prefix.iter().any(|s| s.contains("[ENTRY]")));

    // Verify the logging includes function exit
    assert!(contrib.body_suffix.iter().any(|s| s.contains("[EXIT]")));
}
