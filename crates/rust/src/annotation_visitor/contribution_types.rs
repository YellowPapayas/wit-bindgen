/// Contributions for type definitions (passed as a mutable reference to visitor methods)
#[cfg_attr(docsrs, doc(cfg(feature = "visitor")))]
#[derive(Default, Debug, Clone)]
pub struct RustTypeContribution {
    /// Derive macros to add
    pub derives: Vec<String>,

    /// Attributes to add
    pub attributes: Vec<String>,
}

impl RustTypeContribution {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a derive macro
    pub fn add_derive(&mut self, derive: impl Into<String>) {
        self.derives.push(derive.into());
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.derives.is_empty() && self.attributes.is_empty()
    }
}

/// Contributions for field definitions within records
#[cfg_attr(docsrs, doc(cfg(feature = "visitor")))]
#[derive(Default, Debug, Clone)]
pub struct RustFieldContribution {
    /// Field-level attributes
    pub attributes: Vec<String>,
}

impl RustFieldContribution {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }
}

/// Contributions for variant/enum cases
#[cfg_attr(docsrs, doc(cfg(feature = "visitor")))]
#[derive(Default, Debug, Clone)]
pub struct RustVariantCaseContribution {
    /// Case-level attributes
    pub attributes: Vec<String>,
}

impl RustVariantCaseContribution {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }
}

/// Contributions for function definitions
#[cfg_attr(docsrs, doc(cfg(feature = "visitor")))]
#[derive(Default, Debug, Clone)]
pub struct RustFunctionContribution {
    /// Function-level attributes
    pub attributes: Vec<String>,

    /// Code to prepend to function body
    pub body_prefix: Vec<String>,
    // TODO: Code to append to function body (body_postfix)
}

impl RustFunctionContribution {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an attribute
    pub fn add_attribute(&mut self, attr: impl Into<String>) {
        self.attributes.push(attr.into());
    }

    /// Add code to prepend to function body
    pub fn add_body_prefix(&mut self, code: impl Into<String>) {
        self.body_prefix.push(code.into());
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty() && self.body_prefix.is_empty()
    }
}

/// Contributions for module-level code
#[cfg_attr(docsrs, doc(cfg(feature = "visitor")))]
#[derive(Default, Debug, Clone)]
pub struct RustModuleContribution {
    /// Use statements to add
    pub use_statements: Vec<String>,

    /// Additional code to add to module
    pub additional_code: Vec<String>,
}

impl RustModuleContribution {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a use statement
    pub fn add_use(&mut self, use_stmt: impl Into<String>) {
        self.use_statements.push(use_stmt.into());
    }

    /// Add code to module
    pub fn add_code(&mut self, code: impl Into<String>) {
        self.additional_code.push(code.into());
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.use_statements.is_empty() && self.additional_code.is_empty()
    }
}
