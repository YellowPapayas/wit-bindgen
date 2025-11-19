use wit_parser::*;

/// Generic visitor trait for all language backends.
///
/// This trait uses associated types to allow each language backend to define
/// its own contribution types while sharing the core visitor pattern.
///
/// All methods have default implementations that return `None`, so implementations
/// only need to override the hooks they care about.
///
/// Each `visit_*` method is called during code generation for that element,
/// and can optionally return language-specific contributions (attributes,
/// derives, additional code, etc.).
pub trait Visitor {
    /// Language-specific contribution type for types (records, variants, enums, etc.)
    type TypeContribution;

    /// Language-specific contribution type for fields within records.
    type FieldContribution;

    /// Language-specific contribution type for variant cases.
    type VariantCaseContribution;

    /// Language-specific contribution type for functions.
    type FunctionContribution;

    /// Language-specific contribution type for modules/interfaces.
    type ModuleContribution;
    
    /// Given the target string this visitor will receive
    fn target(&self) -> &str;

    // ==================== Type Definition Hooks ====================
    fn visit_record(&mut self, _annotation: &String, _record: &Record, _type_id: TypeId) -> Option<Self::TypeContribution> {
        None
    }

    fn visit_variant(&mut self, _annotation: &String, _variant: &Variant, _type_id: TypeId) -> Option<Self::TypeContribution> {
        None
    }

    fn visit_enum(&mut self, _annotation: &String, _enum: &Enum, _type_id: TypeId) -> Option<Self::TypeContribution> {
        None
    }

    fn visit_flags(&mut self, _annotation: &String, _flags: &Flags, _type_id: TypeId) -> Option<Self::TypeContribution> {
        None
    }
    
    fn visit_resource(&mut self, _annotation: &String, _resource_id: TypeId) -> Option<Self::TypeContribution> {
        None
    }

    // ==================== Field/Variant Member Hooks ====================
    fn visit_field(&mut self, _annotation: &String, _field: &Field, _field_index: usize) -> Option<Self::FieldContribution> {
        None
    }

    fn visit_variant_case(&mut self, _annotation: &String, _case: &Case, _case_index: usize) -> Option<Self::VariantCaseContribution> {
        None
    }

    // ==================== Function Hooks ====================
    fn visit_function(&mut self, _annotation: &String, _func: &Function) -> Option<Self::FunctionContribution> {
        None
    }

    // ==================== Module/Interface Hooks ====================
    fn visit_interface(&mut self, _annotation: &String, _interface: Option<&Interface>) -> Option<Self::ModuleContribution> {
        None
    }

    fn visit_world(&mut self, _annotation: &String, _world: &World) -> Option<Self::ModuleContribution> {
        None
    }
}
