use js_sys::Object;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Worker;

/// Signature for the `Environment::get_worker` function.
pub type GetWorkerFn = dyn FnMut(String, String) -> Worker;
/// Signature for the `Environment::get_worker_url` function.
pub type GetWorkerUrlFn = dyn FnMut(String, String) -> String;

#[wasm_bindgen]
extern "C" {
    /// Global Monaco environment.
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = Object)]
    pub type Environment;
}
impl Environment {
    /// The base url.
    pub fn base_url(self, val: &str) -> Self {
        object_set!(self.baseUrl = val);
        self
    }

    /// Function which is called to build a `Worker` instance.
    pub fn get_worker(self, val: &Closure<GetWorkerFn>) -> Self {
        object_set!(self.getWorker = val.as_ref());
        self
    }

    /// Function which is called to get the source url for a worker.
    pub fn get_worker_url(self, val: &Closure<GetWorkerUrlFn>) -> Self {
        object_set!(self.getWorkerUrl = val.as_ref());
        self
    }
}
impl Default for Environment {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

#[wasm_bindgen]
extern "C" {
    /// Something that can be disposed
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = Object)]
    pub type IDisposable;

    /// Dispose of the object.
    #[wasm_bindgen(method)]
    pub fn dispose(this: &IDisposable);
}
