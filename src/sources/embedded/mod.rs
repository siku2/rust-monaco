use wasm_bindgen::JsValue;
use web_sys::{Blob, Url, Worker};

const EDITOR: &str = include_str!("monaco.js");
const EDITOR_WORKER: &str = include_str!("editor.worker.js");

const CSS_WORKER: &str = include_str!("css.worker.js");
const HTML_WORKER: &str = include_str!("html.worker.js");

fn create_worker(source: &str) -> Result<Worker, JsValue> {
    let blob = Blob::new_with_str_sequence(&JsValue::from_str(source))?;
    let url = Url::create_object_url_with_blob(&blob)?;
    Worker::new(&url)
}
