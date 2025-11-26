use wit_parser::*;

/// Trait for grouping all contribution types together as a type family.
///
/// Each language backend creates a struct implementing this trait to define
/// its contribution types in one place.
pub trait ContributionTypes {
    /// Contribution type for type definitions (records, variants, enums, flags, resources)
    type Type;

    /// Contribution type for fields within records
    type Field;

    /// Contribution type for variant cases
    type VariantCase;

    /// Contribution type for functions
    type Function;

    /// Contribution type for modules/interfaces
    type Module;
}

/// Generic visitor trait for all language backends.
///
/// This trait uses a nested associated type pattern where implementations
/// specify a `Contributions` type that bundles all contribution types together.
///
/// All methods have default implementations that return `None`, so implementations
/// only need to override the hooks they care about.
///
/// Each `visit_*` method is called during code generation for that element,
/// and can optionally return language-specific contributions (attributes,
/// derives, additional code, etc.).
pub trait Visitor {
    /// The type family containing all contribution types for this visitor
    type Contributions: ContributionTypes;

    /// Given the target string this visitor will receive
    fn target(&self) -> &str;

    // ==================== Type Definition Hooks ====================
    fn visit_record(
        &mut self,
        _annotation: &String,
        _record: &Record,
        _type_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    fn visit_variant(
        &mut self,
        _annotation: &String,
        _variant: &Variant,
        _type_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    fn visit_enum(
        &mut self,
        _annotation: &String,
        _enum: &Enum,
        _type_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    fn visit_flags(
        &mut self,
        _annotation: &String,
        _flags: &Flags,
        _type_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    fn visit_resource(
        &mut self,
        _annotation: &String,
        _resource_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    // ==================== Field/Variant Member Hooks ====================
    fn visit_field(
        &mut self,
        _annotation: &String,
        _field: &Field,
        _field_index: usize,
    ) -> Option<<Self::Contributions as ContributionTypes>::Field> {
        None
    }

    fn visit_variant_case(
        &mut self,
        _annotation: &String,
        _case: &Case,
        _case_index: usize,
    ) -> Option<<Self::Contributions as ContributionTypes>::VariantCase> {
        None
    }

    // ==================== Function Hooks ====================
    fn visit_function(
        &mut self,
        _annotation: &String,
        _func: &Function,
    ) -> Option<<Self::Contributions as ContributionTypes>::Function> {
        None
    }

    // ==================== Module/Interface Hooks ====================
    fn visit_interface(
        &mut self,
        _annotation: &String,
        _interface: Option<&Interface>,
    ) -> Option<<Self::Contributions as ContributionTypes>::Module> {
        None
    }

    fn visit_world(
        &mut self,
        _annotation: &String,
        _world: &World,
    ) -> Option<<Self::Contributions as ContributionTypes>::Module> {
        None
    }
}
