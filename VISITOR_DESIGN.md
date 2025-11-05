# WIT Visitor Trait Design Document

## Table of Contents

- [Overview](#overview)
- [Design Goals](#design-goals)
- [Understanding Opts: The Configuration System](#understanding-opts-the-configuration-system)
- [The WitVisitor Trait](#the-witvisitor-trait)
- [Context Types](#context-types)
- [Contribution APIs](#contribution-apis)
- [Default Behavior](#default-behavior)
- [Integration Points](#integration-points)
- [Usage Examples](#usage-examples)
- [Implementation Approach](#implementation-approach)

## Overview

This document describes the design of a flexible visitor trait system for wit-bindgen's Rust code generation. The visitor pattern allows users to customize and augment generated code with attributes, derive macros, logging, validation, custom code snippets, and framework integrations without modifying wit-bindgen's core code.

### Current Architecture Summary

wit-bindgen generates Rust code from WIT (WebAssembly Interface Type) files through a multi-layered architecture:

- **`RustWasm`** (`lib.rs`) - Top-level orchestrator, implements `WorldGenerator`
- **`InterfaceGenerator`** (`interface.rs`) - Generates types and functions for interfaces
- **`FunctionBindgen`** (`bindgen.rs`) - Generates low-level function ABI code
- **Code is string-based** - Uses `Source` (wrapper around `String`) with formatting utilities

The visitor trait will integrate into this existing architecture by providing hooks at strategic points during code generation.

## Design Goals

1. **Non-invasive**: Default behavior generates identical code to current wit-bindgen
2. **Flexible**: Support multiple use cases (derives, logging, validation, framework integration)
3. **Ergonomic**: Easy to implement only the hooks you need (default implementations for all methods)
4. **Type-safe**: Rich context objects provide access to WIT definitions and metadata
5. **Phased**: Support before/during/after hooks for maximum control
6. **Composable**: Multiple visitors can be combined

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

## The WitVisitor Trait

The main trait users implement to customize code generation. All methods have default no-op implementations.

```rust
/// Trait for visiting and augmenting generated Rust code during wit-bindgen code generation.
///
/// All methods have default implementations that do nothing, so users only need to
/// implement the hooks they care about.
///
/// The visitor follows a three-phase lifecycle for each element:
/// - `before_*` - Called before generation (can skip default generation)
/// - `augment_*` - Called during generation (add attributes, modify builders)
/// - `after_*` - Called after generation (inject additional code)
pub trait WitVisitor {
    // ==================== Type Definition Hooks ====================

    /// Called before generating a record (struct) type.
    /// Return `BeforeAction::Skip` to skip default generation.
    fn before_record(&mut self, ctx: &RecordContext) -> BeforeAction {
        BeforeAction::Continue
    }

    /// Called while generating a record to add attributes and modify generation.
    fn augment_record(&mut self, ctx: &RecordContext, contrib: &mut TypeContribution) {}

    /// Called after a record has been generated, allowing code injection.
    fn after_record(&mut self, ctx: &RecordContext, contrib: &mut TypeContribution) {}

    /// Called before generating a variant (enum with data) type.
    fn before_variant(&mut self, ctx: &VariantContext) -> BeforeAction {
        BeforeAction::Continue
    }

    fn augment_variant(&mut self, ctx: &VariantContext, contrib: &mut TypeContribution) {}

    fn after_variant(&mut self, ctx: &VariantContext, contrib: &mut TypeContribution) {}

    /// Called before generating a simple enum type.
    fn before_enum(&mut self, ctx: &EnumContext) -> BeforeAction {
        BeforeAction::Continue
    }

    fn augment_enum(&mut self, ctx: &EnumContext, contrib: &mut TypeContribution) {}

    fn after_enum(&mut self, ctx: &EnumContext, contrib: &mut TypeContribution) {}

    /// Called before generating a flags (bitflags) type.
    fn before_flags(&mut self, ctx: &FlagsContext) -> BeforeAction {
        BeforeAction::Continue
    }

    fn augment_flags(&mut self, ctx: &FlagsContext, contrib: &mut TypeContribution) {}

    fn after_flags(&mut self, ctx: &FlagsContext, contrib: &mut TypeContribution) {}

    /// Called before generating a resource type.
    fn before_resource(&mut self, ctx: &ResourceContext) -> BeforeAction {
        BeforeAction::Continue
    }

    fn augment_resource(&mut self, ctx: &ResourceContext, contrib: &mut TypeContribution) {}

    fn after_resource(&mut self, ctx: &ResourceContext, contrib: &mut TypeContribution) {}

    // ==================== Field/Variant Member Hooks ====================

    /// Called for each field in a record to add field-level attributes.
    fn augment_field(&mut self, ctx: &FieldContext, contrib: &mut FieldContribution) {}

    /// Called for each case in a variant/enum to add variant-level attributes.
    fn augment_variant_case(
        &mut self,
        ctx: &VariantCaseContext,
        contrib: &mut VariantCaseContribution
    ) {}

    // ==================== Function Hooks ====================

    /// Called before generating any function.
    fn before_function(&mut self, ctx: &FunctionContext) -> BeforeAction {
        BeforeAction::Continue
    }

    /// Called while generating a function to add attributes.
    fn augment_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {}

    /// Called after generating a function to inject code.
    fn after_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {}

    /// Called specifically for import functions (in addition to general function hooks).
    fn before_import_function(&mut self, ctx: &FunctionContext) -> BeforeAction {
        BeforeAction::Continue
    }

    fn after_import_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {}

    /// Called specifically for export functions (in addition to general function hooks).
    fn before_export_function(&mut self, ctx: &FunctionContext) -> BeforeAction {
        BeforeAction::Continue
    }

    fn after_export_function(&mut self, ctx: &FunctionContext, contrib: &mut FunctionContribution) {}

    // ==================== Module/Interface Hooks ====================

    /// Called before generating an interface module.
    fn before_interface(&mut self, ctx: &InterfaceContext) -> BeforeAction {
        BeforeAction::Continue
    }

    /// Called after generating an interface module to add additional code.
    fn after_interface(&mut self, ctx: &InterfaceContext, contrib: &mut ModuleContribution) {}

    /// Called before generating the world bindings.
    fn before_world(&mut self, ctx: &WorldContext) -> BeforeAction {
        BeforeAction::Continue
    }

    /// Called after generating the world bindings.
    fn after_world(&mut self, ctx: &WorldContext, contrib: &mut ModuleContribution) {}
}

/// Return value from `before_*` hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeforeAction {
    /// Continue with default generation.
    Continue,
    /// Skip default generation (visitor will provide everything).
    Skip,
}
```

## Context Types

Context objects provide read-only access to WIT definitions and generation metadata. They enable type-safe, informed decisions in visitor implementations.

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

## Integration Points

This section describes the major modifications needed in existing code to integrate the visitor system.

### 1. Add Visitor to Opts (lib.rs)

**Location:** `crates/rust/src/lib.rs:148-277`

**Change:** Add visitor field to `Opts` struct

```rust
pub struct Opts {
    // ... existing fields ...

    /// Optional visitor to customize code generation.
    pub visitor: Option<Box<dyn WitVisitor>>,
}
```

### 2. Thread Visitor Through RustWasm (lib.rs)

**Location:** `crates/rust/src/lib.rs:28-55`

**Change:** Store visitor in `RustWasm` and pass to `InterfaceGenerator`

```rust
struct RustWasm {
    // ... existing fields ...
    visitor: Option<Box<dyn WitVisitor>>,
}

impl RustWasm {
    fn new() -> RustWasm {
        RustWasm {
            // ... existing initialization ...
            visitor: None,
        }
    }
}

impl Opts {
    pub fn build(self) -> Box<dyn WorldGenerator> {
        let mut r = RustWasm::new();
        r.skip = self.skip.iter().cloned().collect();
        r.visitor = self.visitor; // ← Add this line
        r.opts = self;
        Box::new(r)
    }
}
```

### 3. Add Hooks in InterfaceGenerator for Types (interface.rs)

**Location:** `crates/rust/src/interface.rs`

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

### Example 1: Add Serde Derives to All Types

```rust
use wit_bindgen_rust::visitor::{WitVisitor, RecordContext, VariantContext, EnumContext};
use wit_bindgen_rust::visitor::{TypeContribution};

struct SerdeVisitor;

impl WitVisitor for SerdeVisitor {
    fn augment_record(&mut self, _ctx: &RecordContext, contrib: &mut TypeContribution) {
        contrib.add_derive("serde::Serialize");
        contrib.add_derive("serde::Deserialize");
        contrib.add_attribute("#[serde(rename_all = \"camelCase\")]");
    }

    fn augment_variant(&mut self, _ctx: &VariantContext, contrib: &mut TypeContribution) {
        contrib.add_derive("serde::Serialize");
        contrib.add_derive("serde::Deserialize");
        contrib.add_attribute("#[serde(tag = \"type\", content = \"value\")]");
    }

    fn augment_enum(&mut self, _ctx: &EnumContext, contrib: &mut TypeContribution) {
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

1. **Create visitor module** (`crates/rust/src/visitor.rs`)
   - Define `WitVisitor` trait with all methods
   - Define `BeforeAction` enum
   - Create submodules: `context`, `contribution`

2. **Create context types** (`crates/rust/src/visitor/context.rs`)
   - Implement all context structs (RecordContext, FunctionContext, etc.)
   - Implement helper enums (Direction, OwnershipMode, VariantOrEnum)

3. **Create contribution types** (`crates/rust/src/visitor/contribution.rs`)
   - Implement TypeContribution, FieldContribution, etc.
   - Implement builder methods

4. **Add visitor to Opts** (`crates/rust/src/lib.rs`)
   - Add `visitor: Option<Box<dyn WitVisitor>>` field
   - Thread through to RustWasm

### Phase 2: Integration

5. **Add hooks to InterfaceGenerator** (`crates/rust/src/interface.rs`)
   - Modify `print_typedef_record` with before/augment/after hooks
   - Modify `print_typedef_variant` with hooks
   - Modify `print_typedef_enum` with hooks
   - Add hooks to `type_flags`
   - Add hooks to `type_resource`
   - Add field-level hooks in record generation

6. **Add hooks to function generation** (`crates/rust/src/interface.rs`)
   - Modify `generate_guest_import` with hooks
   - Modify `generate_guest_export` with hooks

7. **Add module/world hooks** (`crates/rust/src/lib.rs`)
   - Add interface before/after hooks
   - Add world before/after hooks

### Phase 3: Documentation and Examples

8. **Add inline documentation**
   - Document all trait methods
   - Document all context types
   - Document contribution APIs

9. **Create example visitors**
   - Add examples directory with common patterns
   - Create serde example
   - Create tracing example
   - Create validation example

### Key Considerations

- **Lifetime Management**: Context types use `'a` lifetime to reference WIT data
- **Mutability**: Visitor is `&mut self` to allow stateful visitors
- **Error Handling**: Visitors should not panic; use Result types if needed
- **Performance**: Visitor checks are guarded by `Option` and only run when present
- **Backward Compatibility**: All changes are additive; existing code unaffected

---

## Summary

This visitor trait system provides:

✅ **Flexibility** - Support for attributes, derives, code injection, and framework integration
✅ **Type Safety** - Rich context objects prevent errors
✅ **Ergonomics** - Default implementations, only override what you need
✅ **Composability** - Multiple visitors can be combined
✅ **Performance** - Zero overhead when not used
✅ **Backward Compatibility** - Existing code unchanged

The design integrates naturally with wit-bindgen's existing architecture by leveraging the `Opts` configuration system and hooking into strategic points during code generation.
