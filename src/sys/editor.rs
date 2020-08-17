//! Bindings for the `monaco.editor` namespace.
use js_sys::{Array, Function, Object};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent, Worker};

// DANGER: Generated code ahead. Keep out!

#[wasm_bindgen(module = "/js/editor.js")]
extern "C" {
    #[wasm_bindgen(js_namespace = editor)]
    pub fn create(dom_element: &HtmlElement) -> JsValue;
}
