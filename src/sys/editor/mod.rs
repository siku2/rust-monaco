pub use enums::*;
pub use interfaces::*;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

mod enums;
mod interfaces;

#[wasm_bindgen]
extern "C" {
    /// Namespace `monaco.editor`
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type Editor;

    /// Create a new editor under domElement. domElement should be empty (not
    /// contain other dom nodes). The editor will read the size of domElement.
    #[wasm_bindgen(method)]
    pub fn create(
        this: &Editor,
        dom_element: &HtmlElement,
        options: Option<&IStandaloneEditorConstructionOptions>,
        // TODO maybe this should be a separate type?
        overrides: Option<&Object>,
    ) -> JsValue;
}
