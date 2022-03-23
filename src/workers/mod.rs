//! Embedded Javascript code for Monaco's language Web Workers.
//! Requires the "workers" feature (enabled by default).
use crate::sys::{Environment, GetWorkerFn};
use js_sys::Array;
use std::iter;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{Blob, Url, Worker};

#[cfg(debug_assertions)]
macro_rules! include_worker {
    ($name: literal) => {
        include_str!(concat!("../../js/debug/", $name))
    };
}

#[cfg(not(debug_assertions))]
macro_rules! include_worker {
    ($name: literal) => {
        include_str!(concat!("../../js/release/", $name))
    };
}

const EDITOR_WORKER: &str = include_worker!("editor.worker.js");

const CSS_WORKER: &str = include_worker!("css.worker.js");
const HTML_WORKER: &str = include_worker!("html.worker.js");
const JSON_WORKER: &str = include_worker!("json.worker.js");

fn create_worker(source: &str) -> Result<Worker, JsValue> {
    let array: Array = iter::once(JsValue::from_str(source)).collect();
    let blob = Blob::new_with_str_sequence(&array)?;
    let url = Url::create_object_url_with_blob(&blob)?;
    Worker::new(&url)
}

fn get_worker(_id: String, label: String) -> Worker {
    let worker = match label.as_str() {
        "css" => CSS_WORKER,
        "html" => HTML_WORKER,
        "json" => JSON_WORKER,
        _ => EDITOR_WORKER,
    };
    create_worker(worker).expect("failed to create worker")
}

fn build_environment() -> Environment {
    let cb = Closure::wrap(Box::new(get_worker) as Box<GetWorkerFn>);
    let env = Environment::default();
    env.set_get_worker(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    env
}

/// Initialize the Monaco environment.
pub fn set_environment() {
    let window = web_sys::window().expect("no global window exists");
    object_set!(window.MonacoEnvironment = build_environment());
}

/// Check if the Monaco environment is set.
pub fn is_environment_set() -> bool {
    if let Some(window) = web_sys::window() {
        if let Ok(value) = object_get!(try window.MonacoEnvironment) {
            return value.is_truthy();
        }
    }
    false
}

/// Set up the environment if it's not already set up.
pub fn ensure_environment_set() {
    if !is_environment_set() {
        set_environment();
    }
}
