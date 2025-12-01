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

    /// The target string for the annotations this visitor is designed to accept
    /// i.e 'serde' would be the target in the annotations #serde(Serialize, Deserialize)
    fn target(&self) -> &str;

    // ==================== Type Definition Hooks ====================
    #[allow(unused)]
    fn visit_record(
        &mut self,
        annotation: &String,
        record: &Record,
        type_id: TypeId,
    ) -> Option<Self::TypeContribution> {
        None
    }

    #[allow(unused)]
    fn visit_variant(
        &mut self,
        annotation: &String,
        variant: &Variant,
        type_id: TypeId,
    ) -> Option<Self::TypeContribution> {
        None
    }

    #[allow(unused)]
    fn visit_enum(
        &mut self,
        annotation: &String,
        enum_: &Enum,
        type_id: TypeId,
    ) -> Option<Self::TypeContribution> {
        None
    }

    #[allow(unused)]
    fn visit_flags(
        &mut self,
        annotation: &String,
        flags: &Flags,
        type_id: TypeId,
    ) -> Option<Self::TypeContribution> {
        None
    }

    #[allow(unused)]
    fn visit_resource(
        &mut self,
        annotation: &String,
        resource_id: TypeId,
    ) -> Option<Self::TypeContribution> {
        None
    }

    // ==================== Field/Variant Member Hooks ====================
    #[allow(unused)]
    fn visit_field(
        &mut self,
        annotation: &String,
        field: &Field,
        field_index: usize,
    ) -> Option<Self::FieldContribution> {
        None
    }

    #[allow(unused)]
    fn visit_variant_case(
        &mut self,
        annotation: &String,
        case: &Case,
        case_index: usize,
    ) -> Option<Self::VariantCaseContribution> {
        None
    }

    // ==================== Function Hooks ====================
    #[allow(unused)]
    fn visit_function(
        &mut self,
        annotation: &String,
        func: &Function,
    ) -> Option<Self::FunctionContribution> {
        None
    }

    // ==================== Module/Interface Hooks ====================
    #[allow(unused)]
    fn visit_interface(
        &mut self,
        annotation: &String,
        interface: Option<&Interface>,
    ) -> Option<Self::ModuleContribution> {
        None
    }

    #[allow(unused)]
    fn visit_world(
        &mut self,
        annotation: &String,
        world: &World,
    ) -> Option<Self::ModuleContribution> {
        None
    }
}
