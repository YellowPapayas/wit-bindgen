//! Contribution APIs for WIT Visitor Pattern
//!
//! This module provides builder-style contribution types for customizing
//! generated Rust code during WIT bindings generation.
//!
//! # Overview
//!
//! The visitor pattern allows users to augment generated code with:
//! - Additional derives and attributes
//! - Doc comments
//! - Custom code snippets
//! - Function body instrumentation
//! - Module-level additions
//!
//! # Contribution Types
//!
//! This module provides several contribution types that are passed as mutable
//! references to visitor methods:
//!
//! - [`TypeContribution`] - For type definitions (records, variants, enums, flags, resources)
//! - [`FieldContribution`] - For struct fields
//! - [`VariantCaseContribution`] - For enum/variant cases
//! - [`FunctionContribution`] - For function definitions
//! - [`ModuleContribution`] - For module-level code
//!
//! # Usage Pattern
//!
//! Visitor methods receive context and contribution objects:
//!
//! ```ignore
//! impl WitVisitor for MyVisitor {
//!     fn augment_record(&mut self, ctx: &RecordContext, contrib: &mut TypeContribution) {
//!         // Add derives
//!         contrib.add_derive("serde::Serialize");
//!         contrib.add_derive("serde::Deserialize");
//!
//!         // Add attributes
//!         contrib.add_attribute("#[serde(rename_all = \"camelCase\")]");
//!
//!         // Add doc comments
//!         contrib.add_doc_comment("This type is serializable");
//!
//!         // Add additional code (impl blocks, etc.)
//!         contrib.add_code(format!("impl {} {{ /* custom methods */ }}", ctx.type_name));
//!     }
//!
//!     fn augment_field(&mut self, ctx: &FieldContext, contrib: &mut FieldContribution) {
//!         if ctx.field.name == "user-id" {
//!             contrib.add_attribute("#[serde(rename = \"userId\")]");
//!             contrib.add_doc_comment("The unique user identifier");
//!         }
//!     }
//!
//!     fn augment_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {
//!         if ctx.direction == Direction::Export {
//!             contrib.add_attribute("#[tracing::instrument]");
//!         }
//!         contrib.prepend_body("tracing::debug!(\"Function called\");");
//!     }
//! }
//! ```
//!
//! # Examples
//!
//! ## Adding Serde Support
//!
//! ```ignore
//! use wit_bindgen_rust::annotation_visitor::*;
//!
//! struct SerdeVisitor;
//!
//! impl WitVisitor for SerdeVisitor {
//!     fn augment_record(&mut self, _ctx: &RecordContext, contrib: &mut TypeContribution) {
//!         contrib.add_derive("serde::Serialize");
//!         contrib.add_derive("serde::Deserialize");
//!         contrib.add_attribute("#[serde(rename_all = \"camelCase\")]");
//!     }
//!
//!     fn augment_variant(&mut self, _ctx: &VariantContext, contrib: &mut TypeContribution) {
//!         contrib.add_derive("serde::Serialize");
//!         contrib.add_derive("serde::Deserialize");
//!         contrib.add_attribute("#[serde(tag = \"type\", content = \"value\")]");
//!     }
//!
//!     fn augment_enum(&mut self, _ctx: &EnumContext, contrib: &mut TypeContribution) {
//!         contrib.add_derive("serde::Serialize");
//!         contrib.add_derive("serde::Deserialize");
//!     }
//! }
//! ```
//!
//! ## Adding Tracing to Functions
//!
//! ```ignore
//! struct TracingVisitor;
//!
//! impl WitVisitor for TracingVisitor {
//!     fn augment_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {
//!         if ctx.direction == Direction::Export {
//!             contrib.add_attribute("#[tracing::instrument]");
//!         }
//!     }
//!
//!     fn after_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {
//!         if ctx.direction == Direction::Import {
//!             contrib.prepend_body(format!(
//!                 "tracing::debug!(\"Calling import function: {}\");",
//!                 ctx.func.name
//!             ));
//!         }
//!     }
//! }
//! ```
//!
//! ## Adding Validation Methods
//!
//! ```ignore
//! struct ValidationVisitor;
//!
//! impl WitVisitor for ValidationVisitor {
//!     fn after_record(&mut self, ctx: &RecordContext, contrib: &mut TypeContribution) {
//!         let type_name = ctx.type_name;
//!         contrib.add_code(format!(
//!             r#"
//! impl {type_name} {{
//!     pub fn validate(&self) -> Result<(), String> {{
//!         // Custom validation logic
//!         Ok(())
//!     }}
//! }}
//!             "#
//!         ));
//!     }
//! }
//! ```
//!
//! # Design Philosophy
//!
//! 1. **Simple & Focused**: Each contribution type has a clear, focused purpose
//! 2. **String-Based**: Flexible string-based API for maximum compatibility
//! 3. **Builder Style**: Ergonomic builder methods for easy construction
//! 4. **Non-Invasive**: Contributions are purely additive
//! 5. **Composable**: Multiple visitors can contribute to the same elements
//!
//! # Integration
//!
//! This module is designed to work with the broader visitor system defined in
//! VISITOR_DESIGN.md. The contribution types are passed to visitor trait methods
//! which modify them in-place.
//!
//! # Deletion Instructions
//!
//! This module is designed to be easily removable:
//!
//! 1. Delete this directory: `rm -rf crates/rust/src/annotation_visitor/`
//! 2. Remove the module declaration from `crates/rust/src/lib.rs` (if added)
//!
//! No other changes should be necessary.

mod contribution;

pub use contribution::{
    FieldContribution, FunctionContribution, ModuleContribution, TypeContribution,
    VariantCaseContribution,
};
