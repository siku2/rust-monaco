pub use enums::*;
pub use interfaces::*;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

mod enums;
mod interfaces;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type Editor;

    #[wasm_bindgen(method)]
    pub fn create(
        this: &Editor,
        dom_element: &HtmlElement,
        options: Option<&IStandaloneEditorConstructionOptions>,
        // overrides: Option<&EditorOverrideServices>,
    ) -> JsValue;
}
