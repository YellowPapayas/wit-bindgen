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

    // ==================== Type Definition Hooks ====================

    /// Called when generating a record (struct/class) type.
    /// Return `Some(contribution)` to add language-specific customizations.
    fn visit_record(&mut self, _record: &Record, _type_id: TypeId) -> Option<Self::TypeContribution> {
        None
    }

    /// Called when generating a variant (tagged union) type.
    /// Return `Some(contribution)` to add language-specific customizations.
    fn visit_variant(&mut self, _variant: &Variant, _type_id: TypeId) -> Option<Self::TypeContribution> {
        None
    }

    /// Called when generating a simple enum type.
    /// Return `Some(contribution)` to add language-specific customizations.
    fn visit_enum(&mut self, _enum: &Enum, _type_id: TypeId) -> Option<Self::TypeContribution> {
        None
    }

    /// Called when generating a flags (bitflags/bitset) type.
    /// Return `Some(contribution)` to add language-specific customizations.
    fn visit_flags(&mut self, _flags: &Flags, _type_id: TypeId) -> Option<Self::TypeContribution> {
        None
    }

    /// Called when generating a resource type.
    /// Return `Some(contribution)` to add language-specific customizations.
    fn visit_resource(&mut self, _resource_id: TypeId) -> Option<Self::TypeContribution> {
        None
    }

    // ==================== Field/Variant Member Hooks ====================

    /// Called for each field in a record.
    /// Return `Some(contribution)` to add field-level customizations.
    fn visit_field(&mut self, _field: &Field, _field_index: usize) -> Option<Self::FieldContribution> {
        None
    }

    /// Called for each case in a variant.
    /// Return `Some(contribution)` to add variant-case-level customizations.
    fn visit_variant_case(&mut self, _case: &Case, _case_index: usize) -> Option<Self::VariantCaseContribution> {
        None
    }

    /// Called for each case in an enum.
    /// Return `Some(contribution)` to add enum-case-level customizations.
    fn visit_enum_case(&mut self, _case: &EnumCase, _case_index: usize) -> Option<Self::VariantCaseContribution> {
        None
    }

    // ==================== Function Hooks ====================

    /// Called when generating a function.
    /// Return `Some(contribution)` to add function-level customizations.
    fn visit_function(&mut self, _func: &Function) -> Option<Self::FunctionContribution> {
        None
    }

    // ==================== Module/Interface Hooks ====================

    /// Called when generating an interface module.
    /// Return `Some(contribution)` to add module-level customizations.
    fn visit_interface(&mut self, _interface: Option<&Interface>) -> Option<Self::ModuleContribution> {
        None
    }

    /// Called when generating the world.
    /// Return `Some(contribution)` to add world-level customizations.
    fn visit_world(&mut self, _world: &World) -> Option<Self::ModuleContribution> {
        None
    }
}
