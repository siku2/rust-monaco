pub use editor_construction_options::*;
pub use editor_options::*;
use wasm_bindgen::prelude::*;

mod editor_construction_options;
mod editor_options;

#[wasm_bindgen]
extern "C" {
    // pub fn create(
    //     dom_element: HtmlElement,
    //     options: Option<IStandaloneEditorConstructionOptions>,
    //     overrides: Option<EditorOverrideServices>,
    // ) -> StandaloneCodeEditor;
}
