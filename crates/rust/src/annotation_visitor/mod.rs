mod contribution_types;
use wit_bindgen_core::{ContributionTypes, Visitor};

pub use contribution_types::{
    RustFieldContribution, RustFunctionContribution, RustModuleContribution, RustTypeContribution,
    RustVariantCaseContribution,
};

/// Type family for Rust contribution types
pub struct RustContributions;

impl ContributionTypes for RustContributions {
    type Type = RustTypeContribution;
    type Field = RustFieldContribution;
    type VariantCase = RustVariantCaseContribution;
    type Function = RustFunctionContribution;
    type Module = RustModuleContribution;
}

// Rust-specific visitor trait
pub trait RustVisitor: Visitor<Contributions = RustContributions> {}

// any type that implements Visitor with the right associated types automatically implements RustVisitor
impl<T> RustVisitor for T where T: Visitor<Contributions = RustContributions> {}
