use wit_bindgen_core::wit_parser::Stability;

/// extract all annotations for a specific target language from a WIT item's stability attribute
/// filters annotations by language key and returns a list of corresponding values
pub fn get_annotations_for_language(stability: &Stability, language: &str) -> Vec<String> {
    match stability {
        Stability::Annotated { annotations } => annotations
            .iter()
            // filter to only annotations matching target language
            .filter(|(key, _)| key == language)
            // extract just the annotation values, discard language key
            .map(|(_, value)| value.clone())
            .collect(),
        // no annotations to extract
        _ => Vec::new(),
    }
}

/// get all annotations from a stability attribute as language-value pairs
pub fn get_all_annotations(stability: &Stability) -> Vec<(String, String)> {
    match stability {
        Stability::Annotated { annotations } => annotations.clone(),
        _ => Vec::new(),
    }
}

/// check whether any annotations exist for a specific target language
pub fn has_annotations_for_language(stability: &Stability, language: &str) -> bool {
    match stability {
        Stability::Annotated { annotations } => annotations.iter().any(|(key, _)| key == language),
        _ => false,
    }
}

pub fn get_rust_annotations(stability: &Stability) -> Vec<String> {
    match stability {
        Stability::Annotated { annotations } => {
            annotations
                .iter()
                .filter(|(lang, _)| lang == "rust")
                .map(|(_, value)| value.clone())
                .collect()
        }
        _ => Vec::new(),
    }
}
