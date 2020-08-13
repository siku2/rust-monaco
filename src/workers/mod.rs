use crate::Environment;
use js_sys::Array;
use wasm_bindgen::{closure::Closure, JsValue};
use web_sys::{Blob, Url, Worker};

macro_rules! include_worker {
    ($name: literal) => {
        include_str!(concat!("../../js/", $name))
    };
}

const EDITOR_WORKER: &str = include_worker!("monaco.js");

fn create_worker(source: &str) -> Result<Worker, JsValue> {
    let array: Array = std::iter::once(JsValue::from_str(source)).collect();
    let blob = Blob::new_with_str_sequence(&array)?;
    let url = Url::create_object_url_with_blob(&blob)?;
    Worker::new(&url)
}

fn get_worker(_id: String, _label: String) -> Worker {
    create_worker(EDITOR_WORKER).unwrap()
}

pub fn build_environment() -> Environment {
    let cb = Closure::wrap(Box::new(get_worker) as Box<dyn FnMut(String, String) -> Worker>);
    let env = Environment::default().get_worker(&cb);
    cb.forget();
    env
}
