use js_sys::Object;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    /// Options which apply for all editors.
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = Object)]
    pub type IGlobalEditorOptions;
}
impl IGlobalEditorOptions {
    /// Controls whether tabSize and insertSpaces will be automatically detected
    /// when a file is opened based on the file contents. Defaults to true.
    pub fn detect_indentation(self, val: bool) -> Self {
        object_set!(self.detectIndentation = val);
        self
    }

    /// Insert spaces when pressing Tab. This setting is overridden based on the
    /// file contents when detectIndentation is on. Defaults to true.
    pub fn insert_spaces(self, val: bool) -> Self {
        object_set!(self.insertSpaces = val);
        self
    }

    /// Special handling for large files to disable certain memory intensive
    /// features. Defaults to true.
    pub fn large_file_optimizations(self, val: bool) -> Self {
        object_set!(self.largeFileOptimizations = val);
        self
    }

    /// Lines above this length will not be tokenized for performance reasons.
    /// Defaults to 20000.
    pub fn max_tokenization_line_length(self, val: u32) -> Self {
        object_set!(self.maxTokenizationLineLength = val);
        self
    }

    /// Keep peek editors open even when double clicking their content or when
    /// hitting Escape. Defaults to false.
    pub fn stable_peek(self, val: bool) -> Self {
        object_set!(self.stablePeek = val);
        self
    }

    /// The number of spaces a tab is equal to. This setting is overridden based
    /// on the file contents when detectIndentation is on. Defaults to 4.
    pub fn tab_size(self, val: u32) -> Self {
        object_set!(self.tabSize = val);
        self
    }

    /// Remove trailing auto inserted whitespace. Defaults to true.
    pub fn trim_auto_whitespace(self, val: bool) -> Self {
        object_set!(self.trimAutoWhitespace = val);
        self
    }

    /// Controls whether completions should be computed based on words in the
    /// document. Defaults to true.
    pub fn word_based_suggestions(self, val: bool) -> Self {
        object_set!(self.wordBasedSuggestions = val);
        self
    }
}
impl Default for IGlobalEditorOptions {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

// TODO: IEditorOptions
#[wasm_bindgen]
extern "C" {
    /// Configuration options for the editor.
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorOptions;
}
impl Default for IEditorOptions {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

// TODO: IDiffEditorOptions
