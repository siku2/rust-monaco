use js_sys::Object;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::Worker;

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

    pub fn get_worker(self, val: &Closure<dyn FnMut(String, String) -> Worker>) -> Self {
        object_set!(self.getWorker = val.as_ref());
        self
    }

    pub fn get_worker_url(self, val: &Closure<dyn FnMut(String, String) -> String>) -> Self {
        object_set!(self.getWorkerUrl = val.as_ref());
        self
    }
}
impl Default for Environment {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}
