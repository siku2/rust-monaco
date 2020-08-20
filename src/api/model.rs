use super::DisposableClosure;
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
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

macro_rules! event_methods {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis $ident:ident($($ty:tt)*);
        )*
    ) => {
        $(
            $(#[$meta])*
            $vis fn $ident(
                &self,
                listener: impl $($ty)* + 'static,
            ) -> DisposableClosure<dyn $($ty)*> {
                let cb = Closure::wrap(Box::new(listener) as Box<dyn $($ty)*>);
                let js_disposable = self
                    .as_ref()
                    .$ident(cb.as_ref().unchecked_ref());
                DisposableClosure::new(cb, js_disposable)
            }
        )*
    };
}

#[must_use = "model is disposed when dropped"]
#[derive(Debug)]
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
    pub fn create(value: &str, language: Option<&str>, uri: Option<&Uri>) -> Self {
        let js_model = editor::create_model(value, language, uri);
        Self::from(js_model)
    }

    /// Get the text stored in this model.
    pub fn get_value(&self) -> String {
        self.js_model.get_value(None, None)
    }
}
impl Drop for TextModel {
    fn drop(&mut self) {
        self.js_model.dispose();
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
