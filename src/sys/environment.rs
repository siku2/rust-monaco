use js_sys::Object;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Worker;

pub type GetWorkerFn = dyn FnMut(String, String) -> Worker;
pub type GetWorkerUrlFn = dyn FnMut(String, String) -> String;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = Object)]
    pub type Environment;
}
impl Environment {
    pub fn base_url(self, val: &str) -> Self {
        object_set!(self.baseUrl = val);
        self
    }

    pub fn get_worker(self, val: &Closure<GetWorkerFn>) -> Self {
        object_set!(self.getWorker = val.as_ref());
        self
    }

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
