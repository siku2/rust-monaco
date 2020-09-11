use crate::sys::{
    editor::{
        self,
        IModelContentChangedEvent,
        IModelLanguageChangedEvent,
        IModelOptionsChangedEvent,
        ITextModel,
    },
    Uri,
};
use wasm_bindgen::{JsCast, JsValue};

/// Models are a more abstract representation of files that can be "opened"
/// (attached) to an editor.
///
/// Cloning this type is cheap to clone.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextModel {
    js_model: ITextModel,
}
impl TextModel {
    event_methods! {
        /// An event emitted when the contents of the model have changed.
        pub on_did_change_content(FnMut(IModelContentChangedEvent));
        /// An event emitted when decorations of the model have changed.
        pub on_did_change_decorations(FnMut(JsValue));
        /// An event emitted when the language associated with the model has changed.
        pub on_did_change_language(FnMut(IModelLanguageChangedEvent));
        /// An event emitted when the language configuration associated with the model has changed.
        pub on_did_change_language_configuration(FnMut(JsValue));
        /// An event emitted when the model options have changed.
        pub on_did_change_options(FnMut(IModelOptionsChangedEvent));
        /// An event emitted right before disposing the model.
        pub on_will_dispose(FnMut());
    }

    /// Create a new model.
    pub fn create(value: &str, language: Option<&str>, uri: Option<&Uri>) -> Result<Self, JsValue> {
        editor::create_model(value, language, uri).map(Self::from)
    }

    /// Get the model that has `uri` if it exists.
    pub fn get(uri: &Uri) -> Option<Self> {
        editor::get_model(uri).map(Self::from)
    }

    /// Get the model that has `uri` if it exists or create a new one.
    /// If the model exists the given `value` and `language` will be set on it.
    pub fn get_or_create(uri: &Uri, value: &str, language: Option<&str>) -> Result<Self, JsValue> {
        if let Some(model) = Self::get(uri) {
            model.set_value(value);
            if let Some(language) = language {
                model.set_language(language);
            }
            Ok(model)
        } else {
            Self::create(value, language, Some(uri))
        }
    }

    /// Get all the created models.
    pub fn get_all() -> Vec<Self> {
        editor::get_models()
            .iter()
            .map(JsCast::unchecked_into::<ITextModel>)
            .map(Self::from)
            .collect()
    }

    /// A unique identifier associated with this model.
    pub fn id(&self) -> String {
        self.js_model.id()
    }

    /// Gets the resource associated with this editor model.
    pub fn uri(&self) -> Uri {
        self.js_model.uri()
    }

    /// Get the language for this model.
    pub fn get_language(&self) -> String {
        self.js_model.get_mode_id()
    }

    /// Change the language for this model.
    pub fn set_language(&self, language_id: &str) {
        editor::set_model_language(self.as_ref(), language_id);
    }

    /// Get the text stored in this model.
    pub fn get_value(&self) -> String {
        self.js_model.get_value(None, None)
    }

    /// Replace the entire text buffer value contained in this model.
    pub fn set_value(&self, value: &str) {
        self.js_model.set_value(value)
    }
}

impl AsRef<ITextModel> for TextModel {
    fn as_ref(&self) -> &ITextModel {
        &self.js_model
    }
}
impl From<ITextModel> for TextModel {
    fn from(js_model: ITextModel) -> Self {
        Self { js_model }
    }
}
