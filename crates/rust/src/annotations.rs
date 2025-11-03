use wit_bindgen_core::wit_parser::Annotations;
use std::collections::HashMap;

/// Collect all annotations for a specific target language from a WIT item's annotations
/// Since there can be multiple #rust(...) annotations, this combines all key-value pairs
/// from all matching annotations into a single HashMap
pub fn get_all_annotations_for_language(
    annotations: &Annotations,
    language: &str,
) -> HashMap<String, String> {
    let mut combined = HashMap::new();
    for (lang, map) in annotations {
        if lang == language {
            combined.extend(map.clone());
        }
    }
    combined
}

/// Extract a specific annotation value from annotations for a target language
/// e.g., get_annotation_value(annotations, "rust", "derive") returns the derive value
pub fn get_annotation_value(
    annotations: &Annotations,
    language: &str,
    key: &str,
) -> Option<String> {
    for (lang, map) in annotations {
        if lang == language {
            if let Some(value) = map.get(key) {
                return Some(value.clone());
            }
        }
    }
    None
}

/// Check whether any annotations exist for a specific target language
pub fn has_annotations_for_language(annotations: &Annotations, language: &str) -> bool {
    annotations.iter().any(|(lang, _)| lang == language)
}
