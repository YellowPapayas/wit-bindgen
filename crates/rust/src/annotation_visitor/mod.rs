mod contribution_types;
use wit_bindgen_core::{ContributionTypes, Visitor};

pub use contribution_types::{
    RustFieldContribution, RustFunctionContribution, RustModuleContribution, RustTypeContribution,
    RustVariantCaseContribution,
};

/// The family of Rust contribution types.
/// This type is used with the nested associated type pattern in the Visitor trait.
pub struct RustContributions;

impl ContributionTypes for RustContributions {
    type Type = RustTypeContribution;
    type Field = RustFieldContribution;
    type VariantCase = RustVariantCaseContribution;
    type Function = RustFunctionContribution;
    type Module = RustModuleContribution;
}

/// Rust-specific visitor type alias.
/// This is just a convenient alias for Visitor with RustContributions.
pub type RustVisitor = dyn Visitor<Contributions = RustContributions>;
