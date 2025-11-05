use std::fmt::Debug;

/// Trait for visiting and processing WIT annotations during code generation.
///
/// This trait allows users to provide custom logic for processing WIT annotations
/// and modifying the generated code based on those annotations.
///
/// This is a placeholder trait that will be implemented in the future to support
/// annotation processing during WIT binding generation.
pub trait WitVisitor: Send + Debug {
    // Methods for visiting different WIT constructs will be added here
    // For example:
    // fn visit_interface(&mut self, interface: &Interface);
    // fn visit_function(&mut self, function: &Function);
    // fn visit_type(&mut self, type_def: &TypeDef);
}
