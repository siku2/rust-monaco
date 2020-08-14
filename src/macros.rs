macro_rules! object_get {
    ($obj:ident.$key:ident) => {
        ::js_sys::Reflect::get(
            $obj.as_ref(),
            &Into::<::wasm_bindgen::JsValue>::into(stringify!($key)),
        )
    };
}

macro_rules! object_set {
    ($obj:ident.$key:ident = $value:expr) => {{
        ::js_sys::Reflect::set(
            $obj.as_ref(),
            &Into::<::wasm_bindgen::JsValue>::into(stringify!($key)),
            &Into::<::wasm_bindgen::JsValue>::into($value),
        )
        .expect("setting key on object must not fail");
    }};
}

macro_rules! str_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident = $($variant_name:ident: $variant_value:literal)|+;
    ) => {
            $(#[$meta])*
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            $vis enum $name {
                $($variant_name),*
            }
            impl $name {
                /// Build from the string value.
                /// Returns `None` if the value isn't part of the enum.
                pub fn from_value(val: &str) -> Option<Self> {
                    match val {
                        $(
                            $variant_value => Some(Self::$variant_name),
                        )*
                        _ => None,
                    }
                }

                /// Get the string value of the variant.
                pub fn value(&self) -> &'static str {
                    match self {
                        $(
                            Self::$variant_name => $variant_value
                        ),*
                    }
                }
            }
            impl Into<::wasm_bindgen::JsValue> for $name {
                fn into(self) -> ::wasm_bindgen::JsValue {
                    ::wasm_bindgen::JsValue::from_str(self.value())
                }
            }
        };
}
