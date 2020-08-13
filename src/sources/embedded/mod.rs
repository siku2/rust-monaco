use wasm_bindgen::JsValue;
use web_sys::{Blob, Url, Worker};

// const EDITOR: &str = include_str!("monaco.js");

fn create_worker(source: &str) -> Result<Worker, JsValue> {
    let blob = Blob::new_with_str_sequence(&JsValue::from_str(source))?;
    let url = Url::create_object_url_with_blob(&blob)?;
    Worker::new(&url)
}
