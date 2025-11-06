use wit_parser::*;

/// Generic visitor trait for all language backends.
///
/// This trait uses associated types to allow each language backend to define
/// its own contribution types while sharing the core visitor pattern.
///
/// All methods have default implementations that do nothing, so implementations
/// only need to override the hooks they care about.
///
/// Each `visit_*` method is called during code generation for that element,
/// allowing the visitor to add language-specific contributions (attributes,
/// derives, additional code, etc.) via the contribution object.
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

    // ==================== Type Definition Hooks ====================

    /// Called when generating a record (struct/class) type.
    fn visit_record(&mut self, _record: &Record, _type_id: TypeId, _contrib: &mut Self::TypeContribution) {}

    /// Called when generating a variant (tagged union) type.
    fn visit_variant(&mut self, _variant: &Variant, _type_id: TypeId, _contrib: &mut Self::TypeContribution) {}

    /// Called when generating a simple enum type.
    fn visit_enum(&mut self, _enum: &Enum, _type_id: TypeId, _contrib: &mut Self::TypeContribution) {}

    /// Called when generating a flags (bitflags/bitset) type.
    fn visit_flags(&mut self, _flags: &Flags, _type_id: TypeId, _contrib: &mut Self::TypeContribution) {}

    /// Called when generating a resource type.
    fn visit_resource(&mut self, _resource_id: TypeId, _contrib: &mut Self::TypeContribution) {}

    // ==================== Field/Variant Member Hooks ====================

    /// Called for each field in a record.
    fn visit_field(&mut self, _field: &Field, _field_index: usize, _contrib: &mut Self::FieldContribution) {}

    /// Called for each case in a variant or enum.
    fn visit_variant_case(&mut self, _case: &Case, _case_index: usize, _contrib: &mut Self::VariantCaseContribution) {}

    // ==================== Function Hooks ====================

    /// Called when generating a function.
    fn visit_function(&mut self, _func: &Function, _contrib: &mut Self::FunctionContribution) {}

    // ==================== Module/Interface Hooks ====================

    /// Called when generating an interface module.
    fn visit_interface(&mut self, _interface: Option<&Interface>, _contrib: &mut Self::ModuleContribution) {}

    /// Called when generating the world.
    fn visit_world(&mut self, _world: &World, _contrib: &mut Self::ModuleContribution) {}
}
