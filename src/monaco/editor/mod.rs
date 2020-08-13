pub use editor_construction_options::*;
pub use editor_options::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

mod editor_construction_options;
mod editor_options;

// TODO
type EditorOverrideServices = str;
type StandaloneCodeEditor = String;

#[wasm_bindgen(module = "/js/monaco.js")]
extern "C" {
    #[wasm_bindgen(js_namespace = editor)]
    pub fn create(
        dom_element: &HtmlElement,
        options: Option<&IStandaloneEditorConstructionOptions>,
        overrides: Option<&EditorOverrideServices>,
    ) -> StandaloneCodeEditor;
}
