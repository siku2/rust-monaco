use super::DisposableClosure;
use crate::sys::{
    editor::{self, IModelContentChangedEvent, ITextModel},
    Uri,
};
use wasm_bindgen::{closure::Closure, JsCast};

#[must_use = "model is disposed when dropped"]
#[derive(Debug)]
pub struct TextModel {
    js_model: ITextModel,
}
impl TextModel {
    /// Create a new model.
    pub fn create(value: &str, language: Option<&str>, uri: Option<&Uri>) -> Self {
        let js_model = editor::create_model(value, language, uri);
        Self::from(js_model)
    }

    pub fn on_did_change_content(
        &self,
        listener: impl 'static + FnMut(IModelContentChangedEvent),
    ) -> DisposableClosure<dyn FnMut(IModelContentChangedEvent)> {
        let cb = Closure::wrap(Box::new(listener) as Box<dyn FnMut(IModelContentChangedEvent)>);
        let js_disposable = self
            .js_model
            .on_did_change_content(cb.as_ref().unchecked_ref());
        DisposableClosure::new(cb, js_disposable)
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
