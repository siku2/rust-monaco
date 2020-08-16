use crate::sys;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

pub struct CodeEditorOptions {}

#[derive(Debug)]
pub struct CodeEditor {
    editor: JsValue,
}
impl CodeEditor {
    pub fn create(element: &HtmlElement) -> Self {
        #[cfg(feature = "embed_workers")]
        crate::embedded::ensure_environment_set();

        let editor = sys::editor.create(element, None, None);
        Self { editor }
    }
}
impl Drop for CodeEditor {
    fn drop(&mut self) {
        // dispose of the editor
    }
}
