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
    #[allow(unused_variables)]
    fn visit_record(
        &mut self,
        annotation: &String,
        record: &Record,
        type_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    #[allow(unused_variables)]
    fn visit_variant(
        &mut self,
        annotation: &String,
        variant: &Variant,
        type_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    #[allow(unused_variables)]
    fn visit_enum(
        &mut self,
        annotation: &String,
        r#enum: &Enum,
        type_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    #[allow(unused_variables)]
    fn visit_flags(
        &mut self,
        annotation: &String,
        flags: &Flags,
        type_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    #[allow(unused_variables)]
    fn visit_resource(
        &mut self,
        annotation: &String,
        resource_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    // ==================== Field/Variant Member Hooks ====================
    #[allow(unused_variables)]
    fn visit_field(
        &mut self,
        annotation: &String,
        field: &Field,
        field_index: usize,
    ) -> Option<<Self::Contributions as ContributionTypes>::Field> {
        None
    }

    #[allow(unused_variables)]
    fn visit_variant_case(
        &mut self,
        annotation: &String,
        case: &Case,
        case_index: usize,
    ) -> Option<<Self::Contributions as ContributionTypes>::VariantCase> {
        None
    }

    // ==================== Function Hooks ====================
    #[allow(unused_variables)]
    fn visit_function(
        &mut self,
        annotation: &String,
        func: &Function,
    ) -> Option<<Self::Contributions as ContributionTypes>::Function> {
        None
    }

    // ==================== Module/Interface Hooks ====================
    #[allow(unused_variables)]
    fn visit_interface(
        &mut self,
        annotation: &String,
        interface: Option<&Interface>,
    ) -> Option<<Self::Contributions as ContributionTypes>::Module> {
        None
    }

    #[allow(unused_variables)]
    fn visit_world(
        &mut self,
        annotation: &String,
        world: &World,
    ) -> Option<<Self::Contributions as ContributionTypes>::Module> {
        None
    }
}
