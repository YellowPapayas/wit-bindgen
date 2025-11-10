mod return_types;
use wit_bindgen_core::Visitor;

pub use return_types::{
    FieldContribution, FunctionContribution, ModuleContribution, TypeContribution,
    VariantCaseContribution,
};

// Rust-specific visitor trait
pub trait RustVisitor: Visitor<
    TypeContribution = TypeContribution,
    FieldContribution = FieldContribution,
    VariantCaseContribution = VariantCaseContribution,
    FunctionContribution = FunctionContribution,
    ModuleContribution = ModuleContribution,
> {}

// any type that implements Visitor with the right associated types automatically implements RustVisitor
impl<T> RustVisitor for T where T: Visitor<
    TypeContribution = TypeContribution,
    FieldContribution = FieldContribution,
    VariantCaseContribution = VariantCaseContribution,
    FunctionContribution = FunctionContribution,
    ModuleContribution = ModuleContribution,
    >
{}