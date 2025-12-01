// Tests all contribution types for Rust visitors

#![cfg(feature = "annotations")]

use wit_bindgen_rust::annotation_visitor::*;

#[test]
fn test_rust_type_contribution_creation() {
    let contrib = RustTypeContribution::new();
    assert!(contrib.is_empty());
    assert_eq!(contrib.derives.len(), 0);
    assert_eq!(contrib.attributes.len(), 0);
}

#[test]
fn test_rust_type_contribution_add_derive() {
    let mut contrib = RustTypeContribution::new();

    contrib.add_derive("Debug");
    contrib.add_derive("Clone");
    contrib.add_derive("PartialEq");

    assert!(!contrib.is_empty());
    assert_eq!(contrib.derives.len(), 3);
    assert!(contrib.derives.contains(&"Debug".to_string()));
    assert!(contrib.derives.contains(&"Clone".to_string()));
    assert!(contrib.derives.contains(&"PartialEq".to_string()));
}

#[test]
fn test_rust_type_contribution_add_attribute() {
    let mut contrib = RustTypeContribution::new();

    contrib.add_attribute("#[repr(C)]");
    contrib.add_attribute("#[deprecated]");

    assert!(!contrib.is_empty());
    assert_eq!(contrib.attributes.len(), 2);
    assert_eq!(contrib.attributes[0], "#[repr(C)]");
    assert_eq!(contrib.attributes[1], "#[deprecated]");
}

#[test]
fn test_rust_field_contribution() {
    let mut contrib = RustFieldContribution::new();
    assert!(contrib.is_empty());

    contrib.add_attribute("#[serde(rename = \"fieldName\")]");

    assert!(!contrib.is_empty());
    assert_eq!(contrib.attributes.len(), 1);
    assert_eq!(contrib.attributes[0], "#[serde(rename = \"fieldName\")]");
}

#[test]
fn test_rust_variant_case_contribution() {
    let mut contrib = RustVariantCaseContribution::new();
    assert!(contrib.is_empty());

    contrib.add_attribute("#[deprecated]");
    contrib.add_attribute("#[doc = \"Legacy variant\"]");

    assert!(!contrib.is_empty());
    assert_eq!(contrib.attributes.len(), 2);
}

#[test]
fn test_rust_function_contribution_attributes() {
    let mut contrib = RustFunctionContribution::new();
    assert!(contrib.is_empty());

    contrib.add_attribute("#[inline]");
    contrib.add_attribute("#[must_use]");

    assert!(!contrib.is_empty());
    assert_eq!(contrib.attributes.len(), 2);
    assert_eq!(contrib.attributes[0], "#[inline]");
    assert_eq!(contrib.attributes[1], "#[must_use]");
}

#[test]
fn test_rust_function_contribution_body_prefix() {
    let mut contrib = RustFunctionContribution::new();
    assert!(contrib.is_empty());

    contrib.add_body_prefix("println!(\"Function called\");");
    contrib.add_body_prefix("let start = std::time::Instant::now();");

    assert!(!contrib.is_empty());
    assert_eq!(contrib.body_prefix.len(), 2);
    assert_eq!(contrib.body_prefix[0], "println!(\"Function called\");");
}

#[test]
fn test_rust_function_contribution_combined() {
    let mut contrib = RustFunctionContribution::new();

    contrib.add_attribute("#[tracing::instrument]");
    contrib.add_body_prefix("tracing::info!(\"Starting function\");");

    assert!(!contrib.is_empty());
    assert_eq!(contrib.attributes.len(), 1);
    assert_eq!(contrib.body_prefix.len(), 1);
}

#[test]
fn test_rust_module_contribution_use_statements() {
    let mut contrib = RustModuleContribution::new();
    assert!(contrib.is_empty());

    contrib.add_use("use std::collections::HashMap");
    contrib.add_use("use serde::{Serialize, Deserialize}");

    assert!(!contrib.is_empty());
    assert_eq!(contrib.use_statements.len(), 2);
    assert_eq!(contrib.use_statements[0], "use std::collections::HashMap");
}

#[test]
fn test_rust_module_contribution_additional_code() {
    let mut contrib = RustModuleContribution::new();
    assert!(contrib.is_empty());

    contrib.add_code("// Module-level constants");
    contrib.add_code("const VERSION: &str = \"1.0.0\";");

    assert!(!contrib.is_empty());
    assert_eq!(contrib.additional_code.len(), 2);
}

#[test]
fn test_rust_module_contribution_combined() {
    let mut contrib = RustModuleContribution::new();

    contrib.add_use("use std::sync::Arc");
    contrib.add_code("// Thread-safe state");
    contrib.add_code("type State = Arc<Mutex<AppState>>;");

    assert!(!contrib.is_empty());
    assert_eq!(contrib.use_statements.len(), 1);
    assert_eq!(contrib.additional_code.len(), 2);
}

#[test]
fn test_type_contribution_is_empty() {
    let mut contrib = RustTypeContribution::new();
    assert!(contrib.is_empty());

    contrib.add_derive("Debug");
    assert!(!contrib.is_empty());

    let mut contrib2 = RustTypeContribution::new();
    contrib2.add_attribute("#[repr(C)]");
    assert!(!contrib2.is_empty());
}

#[test]
fn test_function_contribution_is_empty() {
    let mut contrib = RustFunctionContribution::new();
    assert!(contrib.is_empty());

    contrib.add_attribute("#[inline]");
    assert!(!contrib.is_empty());

    let mut contrib2 = RustFunctionContribution::new();
    contrib2.add_body_prefix("println!(\"test\");");
    assert!(!contrib2.is_empty());
}

#[test]
fn test_module_contribution_is_empty() {
    let mut contrib = RustModuleContribution::new();
    assert!(contrib.is_empty());

    contrib.add_use("use std::io");
    assert!(!contrib.is_empty());

    let mut contrib2 = RustModuleContribution::new();
    contrib2.add_code("const X: u32 = 42;");
    assert!(!contrib2.is_empty());
}

#[test]
fn test_contribution_into_string_conversion() {
    let mut contrib = RustTypeContribution::new();
    contrib.add_derive("Debug".to_string());
    contrib.add_derive(String::from("Clone"));

    assert_eq!(contrib.derives.len(), 2);
}

#[test]
fn test_multiple_attributes_order_preserved() {
    let mut contrib = RustFunctionContribution::new();

    contrib.add_attribute("#[first]");
    contrib.add_attribute("#[second]");
    contrib.add_attribute("#[third]");

    assert_eq!(contrib.attributes[0], "#[first]");
    assert_eq!(contrib.attributes[1], "#[second]");
    assert_eq!(contrib.attributes[2], "#[third]");
}

#[test]
fn test_body_prefix_order_preserved() {
    let mut contrib = RustFunctionContribution::new();

    contrib.add_body_prefix("let x = 1;");
    contrib.add_body_prefix("let y = 2;");
    contrib.add_body_prefix("let z = 3;");

    assert_eq!(contrib.body_prefix[0], "let x = 1;");
    assert_eq!(contrib.body_prefix[1], "let y = 2;");
    assert_eq!(contrib.body_prefix[2], "let z = 3;");
}
