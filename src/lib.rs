pub use monaco::*;

macro_rules! object_get {
    ($obj:ident.$key:ident) => {
        ::js_sys::Reflect::get(
            $obj.as_ref(),
            &::wasm_bindgen::JsValue::from(stringify!($key)),
        )
    };
}

macro_rules! object_set {
    ($obj:ident.$key:ident = $value:expr) => {{
        ::js_sys::Reflect::set(
            $obj.as_ref(),
            &::wasm_bindgen::JsValue::from(stringify!($key)),
            &::wasm_bindgen::JsValue::from($value),
        )
        .expect("setting key on object must not fail");
    }};
}

mod monaco;
#[cfg(feature = "embed_workers")]
mod workers;
#[cfg(feature = "yew")]
pub mod yew;

// TODO find a better way to do this
pub fn init_environment() {
    #[cfg(feature = "embed_workers")]
    workers::init_environment();
}
