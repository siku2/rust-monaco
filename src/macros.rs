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

macro_rules! define_obj_accessor {
    (
        $(#[$meta:meta])*
        build $name:ident: $ty:ty => $js_name:ident
    ) => {
        $(#[$meta])*
        pub fn $name(self, val: $ty) -> Self {
            object_set!(self.$js_name = val);
            self
        }
    }
}

macro_rules! define_interface_builder {
    (
        $(#[$meta:meta])*
        type $name:ident extends $($extend:ident),+ {
            $(
                $(#[$m_meta:meta])*
                $m_name:ident: $m_type:ty => $m_js_name:ident;
            )*
        }
    ) => {
        #[wasm_bindgen]
        extern "C" {
            $(#[$meta])*
            #[derive(Clone, Debug, Eq, PartialEq)]
            $(#[wasm_bindgen(extends = $extend)])*
            pub type $name;
        }
        impl $name {
            $(
                define_obj_accessor! {
                    $(#[$m_meta])*
                    build $m_name: $m_type => $m_js_name
                }
            )*
        }
        impl Default for $name {
            fn default() -> Self {
                JsCast::unchecked_into(Object::new())
            }
        }
    };
}
