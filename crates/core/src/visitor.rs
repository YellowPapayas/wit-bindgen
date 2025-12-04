use std::collections::HashMap;

use wit_parser::*;

/// Trait that groups related contribution types together.
///
/// This allows language backends to define all their contribution types as a cohesive family,
/// reducing boilerplate in visitor implementations.
pub trait ContributionTypes {
    /// Language-specific contribution type for types (records, variants, enums, etc.)
    type Type;

    /// Language-specific contribution type for fields within records.
    type Field;

    /// Language-specific contribution type for variant cases.
    type VariantCase;

    /// Language-specific contribution type for functions.
    type Function;

    /// Language-specific contribution type for modules/interfaces.
    type Module;
}

/// Generic visitor trait for all language backends.
///
/// This trait uses nested associated types to allow each language backend to define
/// its own contribution types while sharing the core visitor pattern.
///
/// All methods have default implementations that return `None`, so implementations
/// only need to override the hooks they care about.
///
/// Each `visit_*` method is called during code generation for that element,
/// and can optionally return language-specific contributions (attributes,
/// derives, additional code, etc.).
pub trait Visitor {
    /// The family of contribution types for this visitor.
    /// Language backends should define a type that implements `ContributionTypes`
    /// with their specific contribution types.
    type Contributions: ContributionTypes;

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
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    #[allow(unused)]
    fn visit_variant(
        &mut self,
        annotation: &String,
        variant: &Variant,
        type_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    #[allow(unused)]
    fn visit_enum(
        &mut self,
        annotation: &String,
        enum_: &Enum,
        type_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    #[allow(unused)]
    fn visit_flags(
        &mut self,
        annotation: &String,
        flags: &Flags,
        type_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    #[allow(unused)]
    fn visit_resource(
        &mut self,
        annotation: &String,
        resource_id: TypeId,
    ) -> Option<<Self::Contributions as ContributionTypes>::Type> {
        None
    }

    // ==================== Field/Variant Member Hooks ====================
    #[allow(unused)]
    fn visit_field(
        &mut self,
        annotation: &String,
        field: &Field,
        field_index: usize,
    ) -> Option<<Self::Contributions as ContributionTypes>::Field> {
        None
    }

    #[allow(unused)]
    fn visit_variant_case(
        &mut self,
        annotation: &String,
        case: &Case,
        case_index: usize,
    ) -> Option<<Self::Contributions as ContributionTypes>::VariantCase> {
        None
    }

    // ==================== Function Hooks ====================
    #[allow(unused)]
    fn visit_function(
        &mut self,
        annotation: &String,
        func: &Function,
    ) -> Option<<Self::Contributions as ContributionTypes>::Function> {
        None
    }

    // ==================== Module/Interface Hooks ====================
    #[allow(unused)]
    fn visit_interface(
        &mut self,
        annotation: &String,
        interface: Option<&Interface>,
    ) -> Option<<Self::Contributions as ContributionTypes>::Module> {
        None
    }

    #[allow(unused)]
    fn visit_world(
        &mut self,
        annotation: &String,
        world: &World,
    ) -> Option<<Self::Contributions as ContributionTypes>::Module> {
        None
    }
}

pub trait FindVisitorWithWarning<T: ?Sized> {
    fn find_visitor_with_warning(&mut self, target: &str) -> Option<&mut Box<T>>;
}

impl<T: ?Sized> FindVisitorWithWarning<T> for HashMap<String, Box<T>> {
    fn find_visitor_with_warning(&mut self, target: &str) -> Option<&mut Box<T>> {
        let result = self.get_mut(target);

        if result.is_none() {
            println!("cargo::warning=Warning: No visitor registered for annotation target '{}'", target);
        }

        result
    }
}
