pub use uri::*;

macro_rules! object_get {
    ($obj:ident.$key:ident) => {
        ::js_sys::Reflect::get($obj.as_ref(), &JsValue::from(stringify!($key)))
    };
}

macro_rules! object_set {
    ($obj:ident.$key:ident = $value:expr) => {{
        ::js_sys::Reflect::set(
            $obj.as_ref(),
            &JsValue::from(stringify!($key)),
            &JsValue::from($value),
        )
        .expect("setting key on object must not fail");
    }};
}

pub mod editor;
mod uri;
