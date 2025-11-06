//! WIT Visitor Return Types
//!
//! This module provides return types for the WIT visitor pattern.
//! The visitor processes WIT annotations and returns what modifications
//! should be applied to generated Rust code.
//!
//! # Overview
//!
//! The visitor pattern processes WIT annotations (from #derive, #serde, #repr, etc.)
//! and returns an `AnnotationResult` that describes what to add to generated code:
//! - Derive macros
//! - Attributes
//! - Field-specific attributes
//! - Function body code
//!
//! # Return Types
//!
//! - [`AnnotationResult`] - Complete result from processing annotations
//! - [`VisitAction`] - Whether to continue or skip default generation
//!
//! # Usage Example
//!
//! ```ignore
//! use wit_bindgen_rust::annotation_visitor::{AnnotationResult, VisitAction};
//!
//! fn process_annotations(annotations: &Annotations) -> AnnotationResult {
//!     let mut result = AnnotationResult::new();
//!
//!     for annotation in &annotations.contents {
//!         if annotation.starts_with("#derive") {
//!             // Parse: "#derive(Clone, Debug)"
//!             for derive in parse_derives(annotation) {
//!                 result.add_derive(derive);
//!             }
//!         } else if annotation.starts_with("#repr") {
//!             // Add as attribute: "#[repr(C)]"
//!             result.add_attribute(format!("#[repr({})]", parse_repr(annotation)));
//!         } else if annotation.starts_with("#serde") {
//!             // Add as attribute: "#[serde(rename_all = \"camelCase\")]"
//!             result.add_attribute(format!("#[serde({})]", parse_serde(annotation)));
//!         }
//!     }
//!
//!     result.action = VisitAction::Continue;
//!     result
//! }
//! ```
//!
//! # Supported Annotations
//!
//! Based on test files `annotations-basic.wit` and `annotations-derive.wit`:
//!
//! ## Type-level
//! - `#derive(...)` - Add derive macros
//! - `#repr(...)` - Add repr attribute
//! - `#serde(...)` - Add serde attribute
//! - `#align(...)` - Add alignment attribute
//! - `#validate_utf8` - Add validation attribute
//!
//! ## Field-level
//! - `#serde(rename = "...")` - Rename field in serialization
//! - `#non_empty` - Validation annotation
//! - `#max_length(...)` - Validation annotation
//! - `#email_format` - Validation annotation
//! - `#range(...)` - Validation annotation
//! - `#finite` - Numeric validation
//!
//! ## Function-level
//! - `#inline` - Add inline attribute
//! - `#must_use` - Add must_use attribute
//! - `#trace_calls` - Add tracing
//! - `#memoize` - Add memoization
//! - `#assert(...)` - Add runtime assertion
//!
//! All annotations are stored as raw attribute strings for maximum flexibility.

mod return_types;

pub use return_types::{
    AnnotationResult, FieldContribution, FunctionContribution, ModuleContribution,
    TypeContribution, VariantCaseContribution, VisitAction,
};
