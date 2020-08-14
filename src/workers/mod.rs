use crate::sys::{Environment, GetWorkerFn};
use js_sys::Array;
use wasm_bindgen::{closure::Closure, JsValue};
use web_sys::{Blob, Url, Worker};

macro_rules! include_worker {
    ($name: literal) => {
        include_str!(concat!("../../js/", $name))
    };
}

const EDITOR_WORKER: &str = include_worker!("editor.worker.js");

const CSS_WORKER: &str = include_worker!("css.worker.js");
const HTML_WORKER: &str = include_worker!("html.worker.js");

fn create_worker(source: &str) -> Result<Worker, JsValue> {
    let array: Array = std::iter::once(JsValue::from_str(source)).collect();
    let blob = Blob::new_with_str_sequence(&array)?;
    let url = Url::create_object_url_with_blob(&blob)?;
    Worker::new(&url)
}

fn get_worker(_id: String, label: String) -> Worker {
    let worker = match label.as_str() {
        "css" => CSS_WORKER,
        "html" => HTML_WORKER,
        _ => EDITOR_WORKER,
    };
    // TODO handle error
    create_worker(worker).unwrap()
}

fn build_environment() -> Environment {
    let cb = Closure::wrap(Box::new(get_worker) as Box<GetWorkerFn>);
    let env = Environment::default().get_worker(&cb);
    cb.forget();
    env
}

pub fn init_environment() {
    let window = web_sys::window().expect("no global window exists");
    object_set!(window.MonacoEnvironment = build_environment());
}
