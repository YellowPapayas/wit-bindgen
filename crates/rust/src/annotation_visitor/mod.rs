mod return_types;
use wit_bindgen_core::Visitor;

pub use return_types::{
    RustFieldContribution, RustFunctionContribution, RustModuleContribution, RustTypeContribution,
    RustVariantCaseContribution,
};

// Rust-specific visitor trait
pub trait RustVisitor: Visitor<
    TypeContribution = RustTypeContribution,
    FieldContribution = RustFieldContribution,
    VariantCaseContribution = RustVariantCaseContribution,
    FunctionContribution = RustFunctionContribution,
    ModuleContribution = RustModuleContribution,
> {}

// any type that implements Visitor with the right associated types automatically implements RustVisitor
impl<T> RustVisitor for T where T: Visitor<
    TypeContribution = RustTypeContribution,
    FieldContribution = RustFieldContribution,
    VariantCaseContribution = RustVariantCaseContribution,
    FunctionContribution = RustFunctionContribution,
    ModuleContribution = RustModuleContribution,
    >
{}