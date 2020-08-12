use super::{IEditorOptions, IGlobalEditorOptions};
use crate::Uri;
use js_sys::Object;
use wasm_bindgen::{prelude::*, JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = IEditorOptions)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type IEditorConstructionOptions;
}
impl IEditorConstructionOptions {
    pub fn with_editor_options(self, val: &IEditorOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }

    /// The initial editor dimension (to avoid measuring the container).
    pub fn dimension(self, dimension: IDimension) -> Self {
        object_set!(self.dimension = dimension);
        self
    }
}
impl Default for IEditorConstructionOptions {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type IDimension;
}
impl IDimension {
    pub fn height(self, val: u32) -> Self {
        object_set!(self.height = val);
        self
    }

    pub fn width(self, val: u32) -> Self {
        object_set!(self.width = val);
        self
    }
}
impl Default for IDimension {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

#[wasm_bindgen]
extern "C" {
    /// The options to create an editor.
    #[wasm_bindgen(extends = IEditorConstructionOptions, extends = IGlobalEditorOptions)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type IStandaloneEditorConstructionOptions;
}
impl IStandaloneEditorConstructionOptions {
    pub fn with_editor_construction_options(self, val: &IEditorConstructionOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }

    pub fn with_global_editor_options(self, val: &IGlobalEditorOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }

    /// An URL to open when Ctrl+H (Windows and Linux) or Cmd+H (OSX) is pressed
    /// in the accessibility help dialog in the editor. Defaults to "https://go.microsoft.com/fwlink/?linkid=852450"
    pub fn accessibility_help_url(self, val: &str) -> Self {
        object_set!(self.accessibilityHelpUrl = val);
        self
    }

    /// The initial language of the auto created model in the editor. To not
    /// create automatically a model, use model: null.
    pub fn language(self, val: bool) -> Self {
        object_set!(self.language = val);
        self
    }

    /// The initial model associated with this code editor.
    pub fn model(self, val: ITextModel) -> Self {
        object_set!(self.model = val);
        self
    }

    /// Initial theme to be used for rendering. The current out-of-the-box
    /// available themes are: 'vs' (default), 'vs-dark', 'hc-black'. You can
    /// create custom themes via monaco.editor.defineTheme. To switch a theme,
    /// use monaco.editor.setTheme
    pub fn theme(self, val: &str) -> Self {
        object_set!(self.theme = val);
        self
    }

    /// The initial value of the auto created model in the editor. To not create
    /// automatically a model, use model: null.
    pub fn value(self, val: &str) -> Self {
        object_set!(self.value = val);
        self
    }
}
impl Default for IStandaloneEditorConstructionOptions {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type ITextModel;
}
impl ITextModel {
    /// A unique identifier associated with this model.
    pub fn id(&self) -> Option<String> {
        object_get!(self.id)
            .ok()
            .as_ref()
            .and_then(JsValue::as_string)
    }

    pub fn set_id(&self, val: &str) {
        object_set!(self.id = val);
    }

    /// Gets the resource associated with this editor model.
    pub fn set_uri(&self, val: Uri) {
        object_set!(self.uri = val);
    }
}
impl Default for ITextModel {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

// TODO: Uri
