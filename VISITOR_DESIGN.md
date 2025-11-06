# WIT Visitor Trait Design Document

## Table of Contents

- [Overview](#overview)
- [Design Goals](#design-goals)
- [Architecture: Core vs Language-Specific](#architecture-core-vs-language-specific)
- [Understanding Opts: The Configuration System](#understanding-opts-the-configuration-system)
- [The Core Visitor Trait](#the-core-visitor-trait)
- [Rust Implementation](#rust-implementation)
  - [The Rust Visitor Trait](#the-rust-visitor-trait)
  - [Context Types](#context-types)
  - [Contribution APIs](#contribution-apis)
  - [Feature Gating](#feature-gating)
- [Cross-Language Examples](#cross-language-examples)
- [Default Behavior](#default-behavior)
- [Integration Points](#integration-points)
- [Usage Examples](#usage-examples)
- [Implementation Approach](#implementation-approach)

## Overview

This document describes the design of a flexible **cross-language visitor trait system** for wit-bindgen code generation. The visitor pattern allows users to customize and augment generated code across all language backends (Rust, C, C++, C#, etc.) without modifying wit-bindgen's core code.

The design uses a **two-layer architecture**:
- **Core layer** (`wit-bindgen-core`): Generic `Visitor` trait that works with WIT types
- **Language layer** (per-backend): Language-specific contribution types and implementations

This allows each language backend to provide idiomatic customization (e.g., Rust derives, C attributes, C# annotations) while sharing the core visitor pattern.

### Current Architecture Summary

wit-bindgen generates Rust code from WIT (WebAssembly Interface Type) files through a multi-layered architecture:

- **`RustWasm`** (`lib.rs`) - Top-level orchestrator, implements `WorldGenerator`
- **`InterfaceGenerator`** (`interface.rs`) - Generates types and functions for interfaces
- **`FunctionBindgen`** (`bindgen.rs`) - Generates low-level function ABI code
- **Code is string-based** - Uses `Source` (wrapper around `String`) with formatting utilities

The visitor trait will integrate into this existing architecture by providing hooks at strategic points during code generation.

## Design Goals

1. **Cross-language**: Support all wit-bindgen backends (Rust, C, C++, C#, etc.)
2. **Non-invasive**: Default behavior generates identical code to current wit-bindgen
3. **Flexible**: Support multiple use cases (derives, logging, validation, framework integration)
4. **Ergonomic**: Easy to implement only the hooks you need (default implementations for all methods)
5. **Type-safe**: Rich context objects provide access to WIT definitions and metadata
6. **Phased**: Support before/during/after hooks for maximum control
7. **Composable**: Multiple visitors can be combined
8. **Language-idiomatic**: Each backend provides natural contribution APIs for that language

## Architecture: Core vs Language-Specific

The visitor system uses a **two-layer architecture** to balance genericity with language-specific power.

### Layer 1: Core Visitor (in `wit-bindgen-core`)

The core defines a **generic visitor trait** that works with WIT types directly:

```rust
// In wit-bindgen-core/src/visitor.rs

use wit_parser::*;

/// Generic visitor trait for all language backends.
pub trait Visitor {
    /// Associated type for type-level contributions (structs, enums, etc.)
    type TypeContribution;

    /// Associated type for field-level contributions.
    type FieldContribution;

    /// Associated type for function-level contributions.
    type FunctionContribution;

    /// Associated type for module-level contributions.
    type ModuleContribution;

    // === Type Hooks ===

    fn before_record(&mut self, record: &Record, type_id: TypeId) -> VisitAction {
        VisitAction::Continue
    }

    fn augment_record(&mut self, record: &Record, type_id: TypeId, contrib: &mut Self::TypeContribution) {}

    fn after_record(&mut self, record: &Record, type_id: TypeId, contrib: &mut Self::TypeContribution) {}

    fn before_variant(&mut self, variant: &Variant, type_id: TypeId) -> VisitAction {
        VisitAction::Continue
    }

    fn augment_variant(&mut self, variant: &Variant, type_id: TypeId, contrib: &mut Self::TypeContribution) {}

    fn after_variant(&mut self, variant: &Variant, type_id: TypeId, contrib: &mut Self::TypeContribution) {}

    // ... similar for enum, flags, resource ...

    // === Field/Variant Hooks ===

    fn augment_field(&mut self, field: &Field, contrib: &mut Self::FieldContribution) {}

    // === Function Hooks ===

    fn before_function(&mut self, func: &Function) -> VisitAction {
        VisitAction::Continue
    }

    fn augment_function(&mut self, func: &Function, contrib: &mut Self::FunctionContribution) {}

    fn after_function(&mut self, func: &Function, contrib: &mut Self::FunctionContribution) {}

    // === Module/Interface Hooks ===

    fn after_interface(&mut self, interface: Option<&Interface>, contrib: &mut Self::ModuleContribution) {}
}

/// Action returned from before_* hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitAction {
    /// Continue with default generation.
    Continue,
    /// Skip default generation (visitor provides everything).
    Skip,
}
```

**Key characteristics:**
- Works directly with WIT types (`Record`, `Variant`, `Function`, etc.)
- Uses **associated types** for contributions (language-specific)
- No language-specific concepts (no "derive", no "attribute", etc.)
- Available to all backends

### Layer 2: Language-Specific Implementation

Each backend implements its own contribution types and enriched context:

**For Rust:**
```rust
// In wit-bindgen-rust/src/visitor/contribution.rs

pub struct RustTypeContribution {
    derives: Vec<String>,
    attributes: Vec<String>,
    doc_comments: Vec<String>,
    additional_code: Vec<String>,
}

impl RustTypeContribution {
    pub fn add_derive(&mut self, derive: impl Into<String>) { ... }
    pub fn add_attribute(&mut self, attr: impl Into<String>) { ... }
    pub fn add_code(&mut self, code: impl Into<String>) { ... }
}

// Similar for RustFieldContribution, RustFunctionContribution, etc.
```

**For C:**
```rust
// In wit-bindgen-c/src/visitor/contribution.rs

pub struct CTypeContribution {
    attributes: Vec<String>,     // __attribute__((...))
    pragmas: Vec<String>,         // #pragma ...
    typedef_mods: Vec<String>,    // const, volatile, etc.
    doc_comments: Vec<String>,
}

impl CTypeContribution {
    pub fn add_attribute(&mut self, attr: impl Into<String>) { ... }
    pub fn add_pragma(&mut self, pragma: impl Into<String>) { ... }
}
```

**For C#:**
```rust
// In wit-bindgen-csharp/src/visitor/contribution.rs

pub struct CSharpTypeContribution {
    attributes: Vec<String>,      // [Attribute]
    base_classes: Vec<String>,    // : BaseClass
    interfaces: Vec<String>,      // : IInterface
    doc_comments: Vec<String>,
}

impl CSharpTypeContribution {
    pub fn add_attribute(&mut self, attr: impl Into<String>) { ... }
    pub fn add_interface(&mut self, iface: impl Into<String>) { ... }
}
```

### How It Works Together

```
┌─────────────────────────────────────┐
│ wit-bindgen-core                    │
│                                     │
│  pub trait Visitor {                │
│    type TypeContribution;           │
│    fn augment_record(&mut self,     │
│      record: &Record,               │
│      contrib: &mut Self::Contrib)   │
│  }                                  │
└──────────────┬──────────────────────┘
               │
               │ Implemented by each language
               │
       ┌───────┴────────┬─────────────────┬──────────────┐
       │                │                 │              │
       ▼                ▼                 ▼              ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Rust Backend │ │  C Backend   │ │ C++ Backend  │ │ C# Backend   │
│              │ │              │ │              │ │              │
│ type TC =    │ │ type TC =    │ │ type TC =    │ │ type TC =    │
│   RustType   │ │   CType      │ │   CppType    │ │   CSharpType │
│   Contribu-  │ │   Contribu-  │ │   Contribu-  │ │   Contribu-  │
│   tion       │ │   tion       │ │   tion       │ │   tion       │
│              │ │              │ │              │ │              │
│ - derives    │ │ - attributes │ │ - templates  │ │ - attributes │
│ - attrs      │ │ - pragmas    │ │ - concepts   │ │ - interfaces │
└──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘
```

### Benefits of This Approach

✅ **Language independence** - Core pattern works for all languages
✅ **Type safety** - Associated types ensure contributions match language
✅ **Idiomatic APIs** - Each language gets natural contribution methods
✅ **Shared infrastructure** - All backends use the same visitor pattern
✅ **Extensibility** - New backends automatically get visitor support

## Understanding Opts: The Configuration System

### What is Opts?

`Opts` is a **configuration struct** (defined in `crates/rust/src/lib.rs:148-277`) that controls how wit-bindgen generates Rust code from WIT files. It's the main way users customize the code generation process.

### How Users Interact with Opts

There are **three main ways** users provide options:

#### 1. Via the `generate!` macro (most common for library users)

```rust
wit_bindgen::generate!({
    world: "my-world",
    // Optional configuration:
    ownership: Owning,
    additional_derives: [Hash, Clone],
    runtime_path: "my_crate::wit_runtime",
    with: {
        "wasi:http/types": generate,
        "my:custom/interface": my_module::bindings,
    },
});
```

The macro (in `crates/guest-rust/macro/src/lib.rs`) parses this and builds an `Opts` struct.

#### 2. Via CLI (for command-line usage)

```bash
wit-bindgen rust ./wit \
    --ownership borrowing \
    -d Hash \
    -d Clone \
    --additional-derive-ignore ignoreme
```

#### 3. Programmatically (for build scripts or advanced use cases)

```rust
use wit_bindgen_rust::Opts;

let opts = Opts {
    format: true,
    ownership: Ownership::Owning,
    additional_derive_attributes: vec!["Hash".to_string()],
    ..Default::default()
};

let generator = opts.build(); // Returns Box<dyn WorldGenerator>
```

### Key Fields in Opts

| Field | Type | Purpose | Example |
|-------|------|---------|---------|
| `format` | `bool` | Use `prettyplease` to format generated code | `true` |
| `std_feature` | `bool` | Gate std-dependent code with `#[cfg(feature = "std")]` | `true` |
| `ownership` | `Ownership` | Control whether types are owning or borrowing | `Owning` / `Borrowing` |
| `additional_derive_attributes` | `Vec<String>` | Add derives to all generated types | `["Hash", "serde::Serialize"]` |
| `additional_derive_ignore` | `Vec<String>` | Exclude specific types from extra derives | `["problematic-type"]` |
| `with` | `Vec<(String, WithOption)>` | Map WIT interfaces/types to Rust paths | See examples |
| `runtime_path` | `Option<String>` | Custom path to wit_bindgen runtime | `"my_crate::runtime"` |
| `skip` | `Vec<String>` | Functions to skip generating | `["debug-only-fn"]` |
| `stubs` | `bool` | Generate stub implementations for exports | `true` |
| `async_` | `AsyncFilterSet` | Configure async support | Complex filtering |

### The Flow: Opts → RustWasm → Code Generation

```
┌─────────────────┐
│  User Code      │
│  generate!({    │
│    world: "x",  │
│    ...opts...   │
│  })             │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Opts struct    │  ← Parsed from macro/CLI/code
│  (config)       │
└────────┬────────┘
         │
         │ opts.build()
         ▼
┌─────────────────┐
│  RustWasm       │  ← Main generator, stores Opts
│  {              │
│    opts: Opts   │  ← Opts stored here
│    ...          │
│  }              │
└────────┬────────┘
         │
         │ Throughout generation, code checks self.opts.field
         ▼
┌─────────────────────────────────────┐
│  Generated Rust Code                │
│  - Formatted if opts.format         │
│  - Extra derives if configured      │
│  - Borrowing types if specified     │
└─────────────────────────────────────┘
```

### How Opts is Used During Generation

The `RustWasm` struct stores the `Opts`:

```rust
struct RustWasm {
    types: Types,
    src: Source,
    opts: Opts,  // ← Stored here and checked throughout generation
    // ... other fields
}
```

### Where the Visitor Fits

For the visitor trait design, we'd add a new field to `Opts`:

```rust
pub struct Opts {
    // ... existing fields ...

    /// Optional visitor to customize code generation.
    pub visitor: Option<Box<dyn WitVisitor>>,
}
```

This flows through the same path: User → Opts → RustWasm → InterfaceGenerator → Called at generation points

## The Core Visitor Trait

The core `Visitor` trait (defined in `wit-bindgen-core/src/visitor.rs`) is language-agnostic and works directly with WIT types. All language backends implement this trait with their own contribution types.

### Complete Core Trait Definition

```rust
// In wit-bindgen-core/src/visitor.rs

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
```

**Key characteristics:**
- Uses WIT types directly (`Record`, `Variant`, `Function`) - works with any language
- Associated types allow each language to define its own contributions
- No language-specific concepts (no "derive", "attribute", etc.)
- All methods have default empty implementations
- Simple, focused API with one hook per element type

## Rust Implementation

The Rust backend implements the core `Visitor` trait with Rust-specific contribution types and enriched context objects.

### File Structure

```
crates/rust/src/visitor/
├── mod.rs              # Re-exports and integration
├── context.rs          # Rust-enriched context types
└── contribution.rs     # Rust contribution types
```

### Rust-Specific Visitor Wrapper

While Rust code generators can work directly with the core `Visitor` trait, the Rust backend provides **enriched context types** that include additional Rust-specific metadata (like generated type names, ownership modes, etc.):

```rust
// In wit-bindgen-rust/src/visitor/mod.rs

/// Rust-specific extension of the core Visitor trait.
///
/// This trait provides the same hooks as the core Visitor but with enriched
/// context types that include Rust-specific information like generated names,
/// ownership modes, etc.
pub trait RustVisitor: Visitor<
    TypeContribution = RustTypeContribution,
    FieldContribution = RustFieldContribution,
    VariantCaseContribution = RustVariantCaseContribution,
    FunctionContribution = RustFunctionContribution,
    ModuleContribution = RustModuleContribution,
> {
    // Rust can provide additional hooks with richer context if needed
}
```

### Context Types

Context objects provide read-only access to WIT definitions **plus Rust-specific generation metadata**. They enable type-safe, informed decisions in visitor implementations.

### RecordContext

```rust
/// Context for record (struct) type generation.
pub struct RecordContext<'a> {
    /// The WIT record definition.
    pub record: &'a Record,

    /// The type ID in the WIT resolve.
    pub type_id: TypeId,

    /// The generated Rust type name.
    pub type_name: &'a str,

    /// Whether this is being generated in an import or export context.
    pub direction: Direction,

    /// The interface this type belongs to (if any).
    pub interface: Option<InterfaceId>,

    /// The world being generated.
    pub world_id: WorldId,

    /// Access to the WIT resolver for looking up types.
    pub resolve: &'a Resolve,

    /// Whether this is an "owned" or "borrowed" variant of the type.
    pub ownership_mode: OwnershipMode,
}
```

### VariantContext

```rust
/// Context for variant (tagged union) type generation.
pub struct VariantContext<'a> {
    /// The WIT variant definition.
    pub variant: &'a Variant,

    /// The type ID in the WIT resolve.
    pub type_id: TypeId,

    /// The generated Rust type name.
    pub type_name: &'a str,

    /// Whether this is being generated in an import or export context.
    pub direction: Direction,

    /// The interface this type belongs to (if any).
    pub interface: Option<InterfaceId>,

    /// The world being generated.
    pub world_id: WorldId,

    /// Access to the WIT resolver for looking up types.
    pub resolve: &'a Resolve,

    /// Whether this is an "owned" or "borrowed" variant of the type.
    pub ownership_mode: OwnershipMode,
}
```

### EnumContext

```rust
/// Context for enum type generation.
pub struct EnumContext<'a> {
    /// The WIT enum definition.
    pub enum_: &'a Enum,

    /// The type ID in the WIT resolve.
    pub type_id: TypeId,

    /// The generated Rust type name.
    pub type_name: &'a str,

    /// Whether this is being generated in an import or export context.
    pub direction: Direction,

    /// The interface this type belongs to (if any).
    pub interface: Option<InterfaceId>,

    /// The world being generated.
    pub world_id: WorldId,

    /// Access to the WIT resolver for looking up types.
    pub resolve: &'a Resolve,
}
```

### FlagsContext

```rust
/// Context for flags (bitflags) type generation.
pub struct FlagsContext<'a> {
    /// The WIT flags definition.
    pub flags: &'a Flags,

    /// The type ID in the WIT resolve.
    pub type_id: TypeId,

    /// The generated Rust type name.
    pub type_name: &'a str,

    /// Whether this is being generated in an import or export context.
    pub direction: Direction,

    /// The interface this type belongs to (if any).
    pub interface: Option<InterfaceId>,

    /// The world being generated.
    pub world_id: WorldId,

    /// Access to the WIT resolver for looking up types.
    pub resolve: &'a Resolve,
}
```

### ResourceContext

```rust
/// Context for resource type generation.
pub struct ResourceContext<'a> {
    /// The resource type ID.
    pub resource: TypeId,

    /// The generated Rust type name.
    pub type_name: &'a str,

    /// Whether this is being generated in an import or export context.
    pub direction: Direction,

    /// The interface this type belongs to (if any).
    pub interface: Option<InterfaceId>,

    /// The world being generated.
    pub world_id: WorldId,

    /// Access to the WIT resolver for looking up types.
    pub resolve: &'a Resolve,
}
```

### FieldContext

```rust
/// Context for a field within a record.
pub struct FieldContext<'a> {
    /// The parent record context.
    pub record: &'a RecordContext<'a>,

    /// The field definition.
    pub field: &'a Field,

    /// The field index within the record.
    pub index: usize,

    /// The generated field name.
    pub field_name: &'a str,
}
```

### VariantCaseContext

```rust
/// Context for a case within a variant or enum.
pub struct VariantCaseContext<'a> {
    /// The parent variant/enum context.
    pub parent: VariantOrEnum<'a>,

    /// The case definition.
    pub case: &'a Case,

    /// The case index.
    pub index: usize,

    /// The generated variant name.
    pub case_name: &'a str,
}

/// Helper enum for variant or enum parent context.
pub enum VariantOrEnum<'a> {
    Variant(&'a VariantContext<'a>),
    Enum(&'a EnumContext<'a>),
}
```

### FunctionContext

```rust
/// Context for function generation.
pub struct FunctionContext<'a> {
    /// The WIT function definition.
    pub func: &'a Function,

    /// The generated Rust function name.
    pub func_name: &'a str,

    /// Whether this is an import or export.
    pub direction: Direction,

    /// The interface this function belongs to (if any).
    pub interface: Option<InterfaceId>,

    /// The world being generated.
    pub world_id: WorldId,

    /// Access to the WIT resolver.
    pub resolve: &'a Resolve,

    /// Whether this function is async.
    pub is_async: bool,
}
```

### InterfaceContext

```rust
/// Context for interface module generation.
pub struct InterfaceContext<'a> {
    /// The interface ID (if this is a named interface).
    pub interface_id: Option<InterfaceId>,

    /// The generated module name.
    pub module_name: &'a str,

    /// Whether this is an import or export module.
    pub direction: Direction,

    /// The world being generated.
    pub world_id: WorldId,

    /// Access to the WIT resolver.
    pub resolve: &'a Resolve,
}
```

### WorldContext

```rust
/// Context for world generation.
pub struct WorldContext<'a> {
    /// The world being generated.
    pub world: &'a World,

    /// The world ID.
    pub world_id: WorldId,

    /// Access to the WIT resolver.
    pub resolve: &'a Resolve,
}
```

### Helper Enums

```rust
/// Direction of code generation (import vs export).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Import,
    Export,
}

/// Ownership mode for type generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipMode {
    Owned,
    Borrowed,
}
```

## Contribution APIs

Builder-style APIs for adding attributes, derives, and code snippets to generated elements.

### TypeContribution

```rust
/// Contributions for type definitions (records, variants, enums, flags, resources).
#[derive(Default)]
pub struct TypeContribution {
    attributes: Vec<String>,
    derives: Vec<String>,
    doc_comments: Vec<String>,
    additional_code: Vec<String>,
}

impl TypeContribution {
    /// Add a raw attribute line (e.g., "#[serde(rename_all = \"camelCase\")]").
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Add a derive macro (e.g., "serde::Serialize").
    /// The derive will be added to the `#[derive(...)]` attribute.
    pub fn add_derive(&mut self, derive: impl Into<String>) {
        self.derives.push(derive.into());
    }

    /// Add a doc comment line (without the "///").
    pub fn add_doc_comment(&mut self, comment: impl Into<String>) {
        self.doc_comments.push(comment.into());
    }

    /// Add additional code after the type definition (e.g., impl blocks, trait impls).
    pub fn add_code(&mut self, code: impl Into<String>) {
        self.additional_code.push(code.into());
    }
}
```

### FieldContribution

```rust
/// Contributions for field definitions within records.
#[derive(Default)]
pub struct FieldContribution {
    attributes: Vec<String>,
    doc_comments: Vec<String>,
}

impl FieldContribution {
    /// Add a field-level attribute (e.g., "#[serde(skip)]").
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Add a doc comment for the field.
    pub fn add_doc_comment(&mut self, comment: impl Into<String>) {
        self.doc_comments.push(comment.into());
    }
}
```

### VariantCaseContribution

```rust
/// Contributions for variant cases (enum variants).
#[derive(Default)]
pub struct VariantCaseContribution {
    attributes: Vec<String>,
    doc_comments: Vec<String>,
}

impl VariantCaseContribution {
    /// Add a variant-level attribute (e.g., "#[serde(rename = \"foo\")]").
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Add a doc comment for the variant.
    pub fn add_doc_comment(&mut self, comment: impl Into<String>) {
        self.doc_comments.push(comment.into());
    }
}
```

### FunctionContribution

```rust
/// Contributions for function definitions.
#[derive(Default)]
pub struct FunctionContribution {
    attributes: Vec<String>,
    doc_comments: Vec<String>,
    body_prefix: Vec<String>,
    body_suffix: Vec<String>,
}

impl FunctionContribution {
    /// Add a function-level attribute (e.g., "#[inline]", "#[tracing::instrument]").
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Add a doc comment for the function.
    pub fn add_doc_comment(&mut self, comment: impl Into<String>) {
        self.doc_comments.push(comment.into());
    }

    /// Prepend code to the function body (runs before generated code).
    pub fn prepend_body(&mut self, code: impl Into<String>) {
        self.body_prefix.push(code.into());
    }

    /// Append code to the function body (runs after generated code, before return).
    pub fn append_body(&mut self, code: impl Into<String>) {
        self.body_suffix.push(code.into());
    }
}
```

### ModuleContribution

```rust
/// Contributions for module-level code.
#[derive(Default)]
pub struct ModuleContribution {
    additional_code: Vec<String>,
    use_statements: Vec<String>,
}

impl ModuleContribution {
    /// Add additional code to the module.
    pub fn add_code(&mut self, code: impl Into<String>) {
        self.additional_code.push(code.into());
    }

    /// Add a use statement to the module.
    pub fn add_use(&mut self, use_stmt: impl Into<String>) {
        self.use_statements.push(use_stmt.into());
    }
}
```

## Cross-Language Examples

This section demonstrates how different language backends would implement the core `Visitor` trait with their own language-specific contribution types.

### C Backend Example

```rust
// In wit-bindgen-c/src/visitor/contribution.rs

/// C-specific type contributions.
pub struct CTypeContribution {
    attributes: Vec<String>,      // __attribute__((...))
    pragmas: Vec<String>,          // #pragma ...
    typedef_modifiers: Vec<String>, // const, volatile, restrict, etc.
    forward_declarations: Vec<String>,
    doc_comments: Vec<String>,
}

impl CTypeContribution {
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    pub fn add_pragma(&mut self, pragma: impl Into<String>) {
        self.pragmas.push(pragma.into());
    }

    pub fn add_typedef_modifier(&mut self, modifier: impl Into<String>) {
        self.typedef_modifiers.push(modifier.into());
    }
}

/// C-specific function contributions.
pub struct CFunctionContribution {
    attributes: Vec<String>,       // __attribute__((...))
    inline_modifier: Option<String>, // inline, static inline, etc.
    calling_convention: Option<String>, // __stdcall, __cdecl, etc.
}

// Implement Visitor for a C-specific visitor
struct CSerializationVisitor;

impl Visitor for CSerializationVisitor {
    type TypeContribution = CTypeContribution;
    type FieldContribution = CFieldContribution;
    type VariantCaseContribution = CVariantCaseContribution;
    type FunctionContribution = CFunctionContribution;
    type ModuleContribution = CModuleContribution;

    fn visit_record(&mut self, _record: &Record, _type_id: TypeId, contrib: &mut CTypeContribution) {
        // Add packed attribute for binary compatibility
        contrib.add_attribute("__attribute__((packed))");

        // Add alignment
        contrib.add_pragma("pack(push, 1)");
    }

    fn visit_function(&mut self, _func: &Function, contrib: &mut CFunctionContribution) {
        // Use cdecl calling convention for all functions
        contrib.calling_convention = Some("__cdecl".to_string());
    }
}
```

**Usage in C:**
```c
// Generated code with visitor contributions

#pragma pack(push, 1)
typedef struct __attribute__((packed)) MyRecord {
    int32_t field1;
    uint64_t field2;
} MyRecord;
#pragma pack(pop)

int32_t __cdecl my_function(MyRecord *record);
```

### C++ Backend Example

```rust
// In wit-bindgen-cpp/src/visitor/contribution.rs

/// C++-specific type contributions.
pub struct CppTypeContribution {
    attributes: Vec<String>,       // [[nodiscard]], [[maybe_unused]], etc.
    template_params: Vec<String>,  // template<typename T>
    concepts: Vec<String>,         // requires ...
    base_classes: Vec<String>,     // : public Base
    friend_declarations: Vec<String>,
    doc_comments: Vec<String>,
}

impl CppTypeContribution {
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    pub fn add_base_class(&mut self, base: impl Into<String>) {
        self.base_classes.push(base.into());
    }

    pub fn add_concept(&mut self, concept: impl Into<String>) {
        self.concepts.push(concept.into());
    }
}

struct CppModernVisitor;

impl Visitor for CppModernVisitor {
    type TypeContribution = CppTypeContribution;
    type FieldContribution = CppFieldContribution;
    type VariantCaseContribution = CppVariantCaseContribution;
    type FunctionContribution = CppFunctionContribution;
    type ModuleContribution = CppModuleContribution;

    fn visit_record(&mut self, _record: &Record, _type_id: TypeId, contrib: &mut CppTypeContribution) {
        // Make all types default-constructible and movable
        contrib.add_attribute("[[nodiscard]]");

        // Add standard constructors/operators via base class
        contrib.add_base_class("public std::move_only_wrapper");
    }

    fn visit_function(&mut self, func: &Function, contrib: &mut CppFunctionContribution) {
        // Mark all functions as [[nodiscard]] if they return a value
        if !func.results.is_empty() {
            contrib.add_attribute("[[nodiscard]]");
        }

        // Add noexcept if function doesn't use Result
        if !has_result_type(func) {
            contrib.add_exception_spec("noexcept");
        }
    }
}
```

**Usage in C++:**
```cpp
// Generated code with visitor contributions

[[nodiscard]]
class MyRecord : public std::move_only_wrapper {
public:
    int32_t field1;
    uint64_t field2;
};

[[nodiscard]] int32_t my_function(const MyRecord& record) noexcept;
```

### C# Backend Example

```rust
// In wit-bindgen-csharp/src/visitor/contribution.rs

/// C#-specific type contributions.
pub struct CSharpTypeContribution {
    attributes: Vec<String>,      // [Attribute]
    interfaces: Vec<String>,      // : IInterface
    base_class: Option<String>,   // : BaseClass
    constraints: Vec<String>,     // where T : constraint
    doc_comments: Vec<String>,    // /// <summary>
    modifiers: Vec<String>,       // partial, sealed, etc.
}

impl CSharpTypeContribution {
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    pub fn add_interface(&mut self, interface: impl Into<String>) {
        self.interfaces.push(interface.into());
    }

    pub fn set_modifier(&mut self, modifier: impl Into<String>) {
        self.modifiers.push(modifier.into());
    }
}

struct CSharpSerializationVisitor;

impl Visitor for CSharpSerializationVisitor {
    type TypeContribution = CSharpTypeContribution;
    type FieldContribution = CSharpFieldContribution;
    type VariantCaseContribution = CSharpVariantCaseContribution;
    type FunctionContribution = CSharpFunctionContribution;
    type ModuleContribution = CSharpModuleContribution;

    fn visit_record(&mut self, _record: &Record, _type_id: TypeId, contrib: &mut CSharpTypeContribution) {
        // Add JSON serialization attributes
        contrib.add_attribute("[System.Text.Json.Serialization.JsonSerializable]");

        // Implement IEquatable for value equality
        contrib.add_interface("IEquatable<MyRecord>");

        // Make it a record type
        contrib.set_modifier("record");
    }

    fn visit_function(&mut self, func: &Function, contrib: &mut CSharpFunctionContribution) {
        // Add async support
        if is_async_function(func) {
            contrib.set_async(true);
        }
    }
}
```

**Usage in C#:**
```csharp
// Generated code with visitor contributions

[System.Text.Json.Serialization.JsonSerializable]
public record MyRecord : IEquatable<MyRecord>
{
    public int Field1 { get; set; }
    public long Field2 { get; set; }
}

public async Task<int> MyFunctionAsync(MyRecord record);
```

### Cross-Language Summary

| Backend | Contribution Types | Common Use Cases |
|---------|-------------------|------------------|
| **Rust** | Derives, attributes, code injection | serde, Debug, validation traits |
| **C** | Attributes (`__attribute__`), pragmas | Packing, alignment, calling conventions |
| **C++** | Attributes (`[[...]]`), concepts, inheritance | Modern C++ features, RAII, noexcept |
| **C#** | Attributes (`[...]`), interfaces, modifiers | Serialization, async, nullable |

Each backend provides idiomatic contribution APIs while sharing the core visitor pattern infrastructure.

## Default Behavior

When no visitor is provided or a visitor doesn't override a method:

1. **No changes**: Generated code is identical to current wit-bindgen output
2. **Zero overhead**: Visitor is `Option<Box<dyn WitVisitor>>`, so no performance impact when `None`
3. **All hooks are optional**: Default trait implementations do nothing
4. **Backward compatible**: Existing code continues to work without modification

This is enforced by:
- All trait methods having default `{}` or `BeforeAction::Continue` implementations
- Visitor stored as `Option<Box<dyn WitVisitor>>` in `Opts`
- Visitor only called when `Some(...)`

## Feature Gating

The visitor functionality is **feature-gated** behind the `visitor` feature flag. This means users must explicitly enable it in their `Cargo.toml` to use visitor functionality.

### Why Feature Gate?

1. **Zero binary overhead** - Users who don't need visitors don't pay for the functionality
2. **Opt-in complexity** - Visitors add conceptual complexity; users opt into it deliberately
3. **Smaller compilation units** - When disabled, visitor code isn't compiled at all
4. **Clear separation** - Makes it obvious which code is part of the visitor system

### Enabling the Visitor Feature

Users who want to use visitors must enable the feature:

**In `Cargo.toml`:**
```toml
[dependencies]
wit-bindgen = { version = "0.47", features = ["visitor"] }
```

**Or when using the macro:**
```rust
// This will only work if the "visitor" feature is enabled
wit_bindgen::generate!({
    world: "my-world",
    visitor: Box::new(MyVisitor),
});
```

### Implementation Strategy

The feature gate is implemented using Rust's conditional compilation (`#[cfg(feature = "visitor")]`):

#### 1. Feature Declaration

**File:** `crates/rust/Cargo.toml`

```toml
[features]
default = []
visitor = []  # No dependencies needed, just a flag
```

#### 2. Module Gating

**File:** `crates/rust/src/lib.rs`

```rust
// Only compile the visitor module when the feature is enabled
#[cfg(feature = "visitor")]
pub mod visitor;

#[cfg(feature = "visitor")]
pub use visitor::{
    WitVisitor, BeforeAction,
    RecordContext, VariantContext, EnumContext, FlagsContext, ResourceContext,
    FieldContext, VariantCaseContext, FunctionContext, InterfaceContext, WorldContext,
    TypeContribution, FieldContribution, VariantCaseContribution,
    FunctionContribution, ModuleContribution,
    Direction, OwnershipMode,
};
```

#### 3. Helper Method Pattern

To keep the main generation code clean, use helper methods that change behavior based on the feature:

```rust
impl InterfaceGenerator<'_> {
    // When feature is enabled, call the visitor
    #[cfg(feature = "visitor")]
    fn visitor_before_record(&mut self, ctx: &RecordContext) -> bool {
        if let Some(visitor) = self.gen.visitor.as_mut() {
            visitor.before_record(ctx) == BeforeAction::Skip
        } else {
            false
        }
    }

    // When feature is disabled, always return false (don't skip)
    #[cfg(not(feature = "visitor"))]
    fn visitor_before_record(&mut self, _ctx: &RecordContext) -> bool {
        false
    }

    #[cfg(feature = "visitor")]
    fn visitor_augment_record(&mut self, ctx: &RecordContext, contrib: &mut TypeContribution) {
        if let Some(visitor) = self.gen.visitor.as_mut() {
            visitor.augment_record(ctx, contrib);
        }
    }

    #[cfg(not(feature = "visitor"))]
    fn visitor_augment_record(&mut self, _ctx: &RecordContext, _contrib: &mut TypeContribution) {}
}
```

This pattern has several advantages:
- The `#[cfg]` attributes are isolated to helper methods
- Main generation code is clean and readable
- Both feature states have explicit implementations
- No runtime overhead in either case

#### 4. Context and Contribution Types

Context creation and contribution objects are also feature-gated:

```rust
fn print_typedef_record(&mut self, id: TypeId, record: &Record, docs: &Docs) {
    let type_name = self.type_name(id);

    // Create context only when feature is enabled
    #[cfg(feature = "visitor")]
    let ctx = RecordContext {
        record,
        type_id: id,
        type_name: &type_name,
        direction: if self.in_import { Direction::Import } else { Direction::Export },
        interface: self.identifier.interface_id(),
        world_id: self.identifier.world_id(),
        resolve: self.resolve,
        ownership_mode: OwnershipMode::Owned,
    };

    // Check before hook using helper method
    #[cfg(feature = "visitor")]
    if self.visitor_before_record(&ctx) {
        return;
    }

    #[cfg(feature = "visitor")]
    let mut contribution = TypeContribution::default();

    #[cfg(feature = "visitor")]
    self.visitor_augment_record(&ctx, &mut contribution);

    // ... existing generation code ...

    // Apply contributions
    #[cfg(feature = "visitor")]
    {
        for doc in &contribution.doc_comments {
            uwriteln!(self.src, "/// {doc}");
        }
        for attr in &contribution.attributes {
            uwriteln!(self.src, "{attr}");
        }
    }
}
```

### Testing Both Configurations

The implementation must be tested with both feature configurations:

**In CI/CD:**
```bash
# Test without visitor feature (default)
cargo test

# Test with visitor feature
cargo test --features visitor

# Test all features
cargo test --all-features
```

**In local development:**
```bash
# Check that code compiles without visitor
cargo check --no-default-features

# Check that code compiles with visitor
cargo check --features visitor
```

### Documentation Considerations

1. **Feature flag must be documented** in the main README
2. **Examples must show feature enablement** in Cargo.toml
3. **API docs should use `#[cfg_attr]`** to indicate feature requirement:

```rust
#[cfg_attr(docsrs, doc(cfg(feature = "visitor")))]
pub trait WitVisitor {
    // ...
}
```

This makes docs.rs show a badge indicating the feature requirement.

## Integration Points

This section describes the major modifications needed in existing code to integrate the visitor system.

### 1. Add Visitor to Opts (lib.rs)

**Location:** `crates/rust/src/lib.rs:148-277`

**Change:** Add feature-gated visitor field to `Opts` struct

```rust
pub struct Opts {
    // ... existing fields ...

    #[cfg(feature = "visitor")]
    /// Optional visitor to customize code generation.
    /// Only available when the "visitor" feature is enabled.
    pub visitor: Option<Box<dyn WitVisitor>>,
}
```

### 2. Thread Visitor Through RustWasm (lib.rs)

**Location:** `crates/rust/src/lib.rs:28-55`

**Change:** Store feature-gated visitor in `RustWasm` and pass to `InterfaceGenerator`

```rust
struct RustWasm {
    // ... existing fields ...

    #[cfg(feature = "visitor")]
    visitor: Option<Box<dyn WitVisitor>>,
}

impl RustWasm {
    fn new() -> RustWasm {
        RustWasm {
            // ... existing initialization ...

            #[cfg(feature = "visitor")]
            visitor: None,
        }
    }
}

impl Opts {
    pub fn build(self) -> Box<dyn WorldGenerator> {
        let mut r = RustWasm::new();
        r.skip = self.skip.iter().cloned().collect();

        #[cfg(feature = "visitor")]
        {
            r.visitor = self.visitor;
        }

        r.opts = self;
        Box::new(r)
    }
}
```

### 3. Add Hooks in InterfaceGenerator for Types (interface.rs)

**Location:** `crates/rust/src/interface.rs`

**Note:** The examples below show the direct inline approach. For cleaner code, use the **helper method pattern** described in the [Feature Gating](#feature-gating) section, which isolates `#[cfg]` attributes to helper functions and keeps the main generation code cleaner.

**Example for Records:** Modify `print_typedef_record` (line ~1942)

```rust
fn print_typedef_record(&mut self, id: TypeId, record: &Record, docs: &Docs) {
    let type_name = self.type_name(id);

    // Create context
    let ctx = RecordContext {
        record,
        type_id: id,
        type_name: &type_name,
        direction: if self.in_import { Direction::Import } else { Direction::Export },
        interface: self.identifier.interface_id(),
        world_id: self.identifier.world_id(),
        resolve: self.resolve,
        ownership_mode: OwnershipMode::Owned, // or Borrowed based on context
    };

    // Before hook
    if let Some(visitor) = self.gen.visitor.as_mut() {
        if visitor.before_record(&ctx) == BeforeAction::Skip {
            return;
        }
    }

    let mut contribution = TypeContribution::default();

    // Augment hook
    if let Some(visitor) = self.gen.visitor.as_mut() {
        visitor.augment_record(&ctx, &mut contribution);
    }

    // Generate docs
    self.print_docs(docs);

    // Add contributed doc comments
    for doc in &contribution.doc_comments {
        uwriteln!(self.src, "/// {doc}");
    }

    // Combine default derives with contributed derives
    let mut all_derives = vec!["Clone"]; // default derives

    // Add existing additional_derive_attributes logic here
    if !self.gen.opts.additional_derive_attributes.is_empty()
        && !self.gen.opts.additional_derive_ignore.contains(&record.name)
    {
        for derive in &self.gen.opts.additional_derive_attributes {
            all_derives.push(derive.as_str());
        }
    }

    // Add visitor-contributed derives
    all_derives.extend(contribution.derives.iter().map(|s| s.as_str()));

    uwriteln!(self.src, "#[derive({})]", all_derives.join(", "));

    // Add contributed attributes
    for attr in &contribution.attributes {
        uwriteln!(self.src, "{attr}");
    }

    // Generate struct definition
    uwriteln!(self.src, "pub struct {type_name} {{");

    for (idx, field) in record.fields.iter().enumerate() {
        let field_name = to_rust_ident(&field.name);

        // Field context
        let field_ctx = FieldContext {
            record: &ctx,
            field,
            index: idx,
            field_name: &field_name,
        };

        let mut field_contrib = FieldContribution::default();

        // Augment field hook
        if let Some(visitor) = self.gen.visitor.as_mut() {
            visitor.augment_field(&field_ctx, &mut field_contrib);
        }

        // Add field doc comments
        for doc in &field_contrib.doc_comments {
            uwriteln!(self.src, "    /// {doc}");
        }

        // Add field attributes
        for attr in &field_contrib.attributes {
            uwriteln!(self.src, "    {attr}");
        }

        // Generate field
        let ty = self.print_ty(&field.ty, TypeMode::Owned);
        uwriteln!(self.src, "    pub {field_name}: {ty},");
    }

    uwriteln!(self.src, "}}");

    // After hook (before Debug impl, etc.)
    if let Some(visitor) = self.gen.visitor.as_mut() {
        visitor.after_record(&ctx, &mut contribution);
    }

    // Add contributed additional code
    for code in &contribution.additional_code {
        uwriteln!(self.src, "{code}");
    }

    // ... rest of existing code (Debug impl, etc.) ...
}
```

**Similar changes needed for:**
- `print_typedef_variant` (line ~2032)
- `print_typedef_enum` (line ~2199)
- `type_flags` (line ~2790)
- `type_resource` (line ~2580)

### 4. Add Hooks in InterfaceGenerator for Functions (interface.rs)

**Location:** `crates/rust/src/interface.rs`

**Example for Import Functions:** Modify `generate_guest_import` (line ~688)

```rust
fn generate_guest_import(&mut self, func: &Function) {
    let func_name = rust_function_name(func);

    // Create context
    let ctx = FunctionContext {
        func,
        func_name: &func_name,
        direction: Direction::Import,
        interface: self.identifier.interface_id(),
        world_id: self.identifier.world_id(),
        resolve: self.resolve,
        is_async: self.is_async(func),
    };

    // Before hooks
    if let Some(visitor) = self.gen.visitor.as_mut() {
        if visitor.before_function(&ctx) == BeforeAction::Skip {
            return;
        }
        if visitor.before_import_function(&ctx) == BeforeAction::Skip {
            return;
        }
    }

    let mut contribution = FunctionContribution::default();

    // Augment hook
    if let Some(visitor) = self.gen.visitor.as_mut() {
        visitor.augment_function(&ctx, &mut contribution);
    }

    // Generate function...

    // Add contributed doc comments
    for doc in &contribution.doc_comments {
        uwriteln!(self.src, "/// {doc}");
    }

    // Add contributed attributes
    for attr in &contribution.attributes {
        uwriteln!(self.src, "{attr}");
    }

    // Generate signature
    // ... existing signature generation code ...

    uwriteln!(self.src, "{{");

    // Add body prefix
    for code in &contribution.body_prefix {
        uwriteln!(self.src, "    {code}");
    }

    // ... existing function body generation ...

    // Add body suffix (before return/end of function)
    for code in &contribution.body_suffix {
        uwriteln!(self.src, "    {code}");
    }

    uwriteln!(self.src, "}}");

    // After hooks
    if let Some(visitor) = self.gen.visitor.as_mut() {
        visitor.after_function(&ctx, &mut contribution);
        visitor.after_import_function(&ctx, &mut contribution);
    }
}
```

**Similar changes needed for:**
- `generate_guest_export` (line ~1035)

### 5. Add Module/World Hooks (lib.rs and interface.rs)

Add hooks in appropriate locations:
- Before/after interface generation in `RustWasm::import_interface` and `RustWasm::export_interface`
- Before/after world generation in `RustWasm::finish`

## Usage Examples

**Prerequisites:** All examples require enabling the `visitor` feature:

```toml
# In Cargo.toml
[dependencies]
wit-bindgen = { version = "0.47", features = ["visitor"] }
```

### Example 1: Add Serde Derives to All Types

```rust
use wit_bindgen_core::Visitor;
use wit_bindgen_rust::{RustTypeContribution};
use wit_parser::*;

struct SerdeVisitor;

impl Visitor for SerdeVisitor {
    type TypeContribution = RustTypeContribution;
    type FieldContribution = RustFieldContribution;
    type VariantCaseContribution = RustVariantCaseContribution;
    type FunctionContribution = RustFunctionContribution;
    type ModuleContribution = RustModuleContribution;

    fn visit_record(&mut self, _record: &Record, _type_id: TypeId, contrib: &mut RustTypeContribution) {
        contrib.add_derive("serde::Serialize");
        contrib.add_derive("serde::Deserialize");
        contrib.add_attribute("#[serde(rename_all = \"camelCase\")]");
    }

    fn visit_variant(&mut self, _variant: &Variant, _type_id: TypeId, contrib: &mut RustTypeContribution) {
        contrib.add_derive("serde::Serialize");
        contrib.add_derive("serde::Deserialize");
        contrib.add_attribute("#[serde(tag = \"type\", content = \"value\")]");
    }

    fn visit_enum(&mut self, _enum: &Enum, _type_id: TypeId, contrib: &mut RustTypeContribution) {
        contrib.add_derive("serde::Serialize");
        contrib.add_derive("serde::Deserialize");
    }
}

// Use with macro:
wit_bindgen::generate!({
    world: "my-world",
    visitor: Box::new(SerdeVisitor),
});
```

### Example 2: Add Tracing to Functions

```rust
struct TracingVisitor;

impl WitVisitor for TracingVisitor {
    fn augment_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {
        if ctx.direction == Direction::Export {
            contrib.add_attribute("#[tracing::instrument]");
        }
    }

    fn after_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {
        if ctx.direction == Direction::Import {
            contrib.prepend_body(format!(
                "tracing::debug!(\"Calling import function: {}\");",
                ctx.func.name
            ));
        }
    }
}
```

### Example 3: Add Validation Methods

```rust
struct ValidationVisitor;

impl WitVisitor for ValidationVisitor {
    fn after_record(&mut self, ctx: &RecordContext, contrib: &mut TypeContribution) {
        // Add a validate() method to all records
        let type_name = ctx.type_name;
        contrib.add_code(format!(
            r#"
impl {type_name} {{
    pub fn validate(&self) -> Result<(), String> {{
        // Custom validation logic could be added here
        Ok(())
    }}
}}
            "#
        ));
    }

    fn after_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {
        // Add validation calls for parameters
        for param in &ctx.func.params {
            // Check if param type is a custom record (simplified check)
            if let Type::Id(id) = param.1 {
                let type_def = &ctx.resolve.types[*id];
                if matches!(type_def.kind, TypeDefKind::Record(_)) {
                    contrib.prepend_body(format!(
                        "{}.validate().map_err(|e| format!(\"Invalid {}: {{}}\", e))?;",
                        to_rust_ident(&param.0),
                        param.0
                    ));
                }
            }
        }
    }
}
```

### Example 4: Conditional Derives Based on Context

```rust
struct SmartDerivesVisitor;

impl WitVisitor for SmartDerivesVisitor {
    fn augment_record(&mut self, ctx: &RecordContext, contrib: &mut TypeContribution) {
        // Only add serde to import types
        if ctx.direction == Direction::Import {
            contrib.add_derive("serde::Serialize");
            contrib.add_derive("serde::Deserialize");
        }

        // Add Debug and Clone to all types
        contrib.add_derive("Debug");
        contrib.add_derive("Clone");

        // Add specific interface customizations
        if let Some(interface_id) = ctx.interface {
            let interface = &ctx.resolve.interfaces[interface_id];
            if let Some(name) = &interface.name {
                if name.contains("http") {
                    contrib.add_doc_comment("HTTP-related type");
                }
            }
        }
    }
}
```

### Example 5: Combining Multiple Visitors

```rust
/// Composite visitor that delegates to multiple visitors.
struct CompositeVisitor {
    visitors: Vec<Box<dyn WitVisitor>>,
}

impl WitVisitor for CompositeVisitor {
    fn augment_record(&mut self, ctx: &RecordContext, contrib: &mut TypeContribution) {
        for visitor in &mut self.visitors {
            visitor.augment_record(ctx, contrib);
        }
    }

    fn augment_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {
        for visitor in &mut self.visitors {
            visitor.augment_function(ctx, contrib);
        }
    }

    // ... implement for all other methods ...
}

// Usage:
let visitor = CompositeVisitor {
    visitors: vec![
        Box::new(SerdeVisitor),
        Box::new(TracingVisitor),
        Box::new(ValidationVisitor),
    ],
};
```

## Implementation Approach

Step-by-step plan for implementing the visitor system:

### Phase 1: Core Infrastructure (No Tests)

1. **Add feature flag** (`crates/rust/Cargo.toml`)
   - Add `visitor = []` to `[features]` section
   - Ensure it's not in the default features

2. **Create visitor module** (`crates/rust/src/visitor.rs`)
   - Define `WitVisitor` trait with all methods
   - Add `#[cfg_attr(docsrs, doc(cfg(feature = "visitor")))]` to public items
   - Define `BeforeAction` enum
   - Create submodules: `context`, `contribution`

3. **Create context types** (`crates/rust/src/visitor/context.rs`)
   - Implement all context structs (RecordContext, FunctionContext, etc.)
   - Implement helper enums (Direction, OwnershipMode, VariantOrEnum)

4. **Create contribution types** (`crates/rust/src/visitor/contribution.rs`)
   - Implement TypeContribution, FieldContribution, etc.
   - Implement builder methods

5. **Add feature-gated visitor to Opts** (`crates/rust/src/lib.rs`)
   - Add `#[cfg(feature = "visitor")]` module declaration
   - Add `#[cfg(feature = "visitor")]` pub use statements
   - Add `#[cfg(feature = "visitor")] visitor: Option<Box<dyn WitVisitor>>` field to Opts
   - Thread through to RustWasm with feature gates

### Phase 2: Integration

6. **Create visitor helper methods** (`crates/rust/src/interface.rs`)
   - Add feature-gated helper methods to InterfaceGenerator (visitor_before_record, visitor_augment_record, etc.)
   - Create both `#[cfg(feature = "visitor")]` and `#[cfg(not(feature = "visitor"))]` versions
   - Follow the helper method pattern from the Feature Gating section

7. **Add hooks to InterfaceGenerator for types** (`crates/rust/src/interface.rs`)
   - Modify `print_typedef_record` using helper methods
   - Modify `print_typedef_variant` using helper methods
   - Modify `print_typedef_enum` using helper methods
   - Add hooks to `type_flags`
   - Add hooks to `type_resource`
   - Add field-level hooks in record generation

8. **Add hooks to function generation** (`crates/rust/src/interface.rs`)
   - Create helper methods for function hooks
   - Modify `generate_guest_import` using helper methods
   - Modify `generate_guest_export` using helper methods

9. **Add module/world hooks** (`crates/rust/src/lib.rs`)
   - Add interface before/after hooks
   - Add world before/after hooks

### Phase 3: Testing and Documentation

10. **Test feature configurations**
    - Ensure code compiles without visitor feature: `cargo check --no-default-features`
    - Ensure code compiles with visitor feature: `cargo check --features visitor`
    - Run tests without feature: `cargo test`
    - Run tests with feature: `cargo test --features visitor`
    - Update CI to test both configurations

11. **Add inline documentation**
    - Document all trait methods with `#[cfg_attr(docsrs, doc(cfg(feature = "visitor")))]`
    - Document all context types
    - Document contribution APIs
    - Add feature requirement notes to module-level docs

12. **Create example visitors**
    - Add examples directory with common patterns
    - Create serde example
    - Create tracing example
    - Create validation example
    - Ensure all examples show feature enablement in comments

### Key Considerations

- **Lifetime Management**: Context types use `'a` lifetime to reference WIT data
- **Mutability**: Visitor is `&mut self` to allow stateful visitors
- **Error Handling**: Visitors should not panic; use Result types if needed
- **Performance**: Visitor checks are guarded by `Option` and only run when present, plus feature gating ensures zero overhead when feature is disabled
- **Feature Gating**: All visitor functionality is behind the `visitor` feature flag; use helper methods to keep code clean
- **Testing**: Must test both with and without the feature enabled to ensure correctness
- **Backward Compatibility**: All changes are additive; existing code unaffected

---

## Summary

This visitor trait system provides:

✅ **Cross-language support** - Works with all wit-bindgen backends (Rust, C, C++, C#, etc.)
✅ **Language-idiomatic** - Each backend provides natural contribution APIs for that language
✅ **Flexibility** - Support for attributes, derives, code injection, and framework integration
✅ **Type Safety** - Associated types ensure contributions match the target language
✅ **Ergonomics** - Default implementations, only override what you need
✅ **Composability** - Multiple visitors can be combined
✅ **Performance** - Zero overhead when not used (feature-gated per-backend and Option-based)
✅ **Opt-in** - Feature flags ensure users explicitly choose to enable visitor functionality
✅ **Backward Compatibility** - Existing code unchanged, no breaking changes
✅ **Shared infrastructure** - Core pattern in wit-bindgen-core, implemented once per backend

The design uses a **two-layer architecture**:
- **Core layer** (`wit-bindgen-core`): Generic `Visitor` trait with associated types
- **Language layer** (per-backend): Language-specific contribution types and implementations

This integrates naturally with wit-bindgen's existing architecture by:
- Leveraging configuration systems (like Rust's `Opts`) per-backend
- Hooking into strategic points during code generation
- Using associated types for type-safe, language-specific contributions
- Feature-gating per-backend to ensure zero overhead when disabled

The result is a **first-class wit-bindgen feature** that all backends can leverage to provide customization without modifying core code.
