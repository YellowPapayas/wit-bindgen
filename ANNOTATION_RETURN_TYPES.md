# Annotation Return Types (Simplified)

## Overview

A minimal return type for annotation processing, designed to support the annotations in the test files:
- `annotations-basic.wit`
- `annotations-derive.wit`

## Core Type

```rust
pub struct AnnotationResult {
    pub derives: Vec<String>,
    pub attributes: Vec<String>,
    pub field_attributes: HashMap<String, Vec<String>>,
    pub function_body_prefix: Vec<String>,
    pub action: VisitAction,
}
```

## Usage Examples

### Processing #derive Annotation

```rust
use wit_bindgen_rust::annotation_visitor::{AnnotationResult, VisitAction};

fn process_derive(annotation: &str) -> AnnotationResult {
    // Input: "#derive(Clone, Debug, serde::Serialize)"
    let mut result = AnnotationResult::new();

    // Extract derives: ["Clone", "Debug", "serde::Serialize"]
    for derive in parse_derives(annotation) {
        result.add_derive(derive);
    }

    result.action = VisitAction::Continue;
    result
}
```

### Processing #repr Annotation

```rust
fn process_repr(annotation: &str) -> AnnotationResult {
    // Input: "#repr(C)" or "#repr(u8)"
    let mut result = AnnotationResult::new();

    // Just add the attribute directly
    result.add_attribute(format!("#[repr({})]", extract_repr_value(annotation)));

    result
}
```

### Processing Field Annotations

```rust
fn process_field_annotations(field_name: &str, annotations: &[String]) -> AnnotationResult {
    let mut result = AnnotationResult::new();

    for annotation in annotations {
        if annotation.starts_with("#serde") {
            // Add field-specific attribute
            result.add_field_attribute(
                field_name,
                format!("#[{}]", convert_annotation(annotation))
            );
        }
    }

    result
}
```

### Complete Example from Test Files

```wit
#derive(Debug, Clone, PartialEq, Eq)
#serde(rename_all = "camelCase")
record results {
    success: bool,
    #serde(rename = "msg")
    message: string,
}
```

Expected `AnnotationResult`:

```rust
AnnotationResult {
    derives: vec![
        "Debug".to_string(),
        "Clone".to_string(),
        "PartialEq".to_string(),
        "Eq".to_string(),
    ],
    attributes: vec![
        "#[serde(rename_all = \"camelCase\")]".to_string(),
    ],
    field_attributes: {
        "message": vec![
            "#[serde(rename = \"msg\")]".to_string(),
        ],
    },
    function_body_prefix: vec![],
    action: VisitAction::Continue,
}
```

## Implementation (~130 lines)

- `AnnotationResult` - Simple struct with 5 fields
- `VisitAction` - Just Continue/Skip
- Helper methods: `new()`, `add_derive()`, `add_attribute()`, `add_field_attribute()`, `add_body_prefix()`, `is_empty()`
- 3 unit tests

## Benefits

✅ **Simple** - Only 130 lines vs 750 in complex version
✅ **Focused** - Supports exactly what test files need
✅ **Flexible** - Raw strings allow any attribute format
✅ **Easy to extend** - Just add more Vec<String> fields
✅ **No complex types** - No enums, just strings and hashmaps

## Supported Annotations

From **annotations-basic.wit**:
- `#derive(...)` ✅
- `#serde(...)` ✅
- `#repr(...)` ✅
- `#inline` ✅
- `#must_use` ✅
- `#cfg(...)` ✅
- `#assert(...)` ✅

From **annotations-derive.wit**:
- `#align(...)` ✅
- `#validate_utf8` ✅
- `#non_empty` ✅
- `#max_length(...)` ✅
- `#email_format` ✅
- `#range(...)` ✅
- `#finite` ✅
- `#trace_calls` ✅
- `#memoize` ✅
- `#cache_result(...)` ✅
- `#experimental`, `#deprecated`, `#stable` ✅

All as raw attribute strings!
