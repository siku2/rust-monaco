macro_rules! object_get {
    ($obj:ident.$key:ident) => {
        ::js_sys::Reflect::get($obj.as_ref(), &stringify!($key).into())
            .expect("getting key from object must not fail")
    };
    ($obj:ident.$key:ident as Option<bool>) => {
        object_get!($obj.$key).as_bool()
    };
    ($obj:ident.$key:ident as Option<f64>) => {
        object_get!($obj.$key).as_f64()
    };
    ($obj:ident.$key:ident as Option<String>) => {
        object_get!($obj.$key).as_string()
    };
    ($obj:ident.$key:ident as $ty:ty) => {
        ::wasm_bindgen::JsCast::dyn_into(object_get!($obj.$key)).ok()
    };
    ($obj:ident.$key:ident as enum Option<$ty:ty>) => {
        object_get!($obj.$key).as_ref().map($ty::from_value)
    };
}

macro_rules! object_set {
    ($obj:ident.$key:ident = $value:expr) => {{
        ::js_sys::Reflect::set($obj.as_ref(), &stringify!($key).into(), &Into::into($value))
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

macro_rules! define_property {
    (
        $(#[$meta:meta])*
        readonly $name:ident: $ty:ty
    ) => {
        ::paste::paste! {
            #[doc = "Get the `" $name "` property."]
            $(#[$meta])*
            pub fn [<get_ $name:snake>](&self) -> $ty {
                object_get!(self.$name as $ty)
            }
        }
    };
    (
        $(#[$meta:meta])*
        writeonly $name:ident: $ty:ty
    ) => {
        ::paste::paste! {
            #[doc = "Set the `" $name "` property."]
            $(#[$meta])*
            pub fn [<set_ $name:snake>](&self, val: $ty) {
                object_set!(self.$name = val);
            }
        }
    };
    (
        $(#[$meta:meta])*
        $name:ident: get: $get_ty:ty, set: $set_ty:ty
    ) => {
        define_property!{
            $(#[$meta])*
            readonly $name: $get_ty
        }
        ::paste::paste! {
            #[doc = "Set the `" $name "` property."]
            pub fn [<set_ $name:snake>](&self, val: $set_ty) {
                object_set!(self.$name = val);
            }
        }
    };
    (
        $(#[$meta:meta])*
        $name:ident: Option<&str>
    ) => {
        define_property!{
            $(#[$meta])*
            $name: get: Option<String>, set: Option<&str>
        }
    };
    (
        $(#[$meta:meta])*
        $name:ident: Option<&$ty:ty>
    ) => {
        define_property!{
            $(#[$meta])*
            $name: get: Option<$ty>, set: Option<&$ty>
        }
    };
    (
        $(#[$meta:meta])*
        $name:ident: $ty:ty
    ) => {
        define_property!{
            $(#[$meta])*
            $name: get: $ty, set: $ty
        }
    };
}

macro_rules! define_object_interface {
    (
        $(#[$meta:meta])*
        type $name:ident extends $($extend:ty),+;
    ) => {
        #[wasm_bindgen]
        extern "C" {
            $(#[$meta])*
            #[derive(Clone, Debug, Eq, PartialEq)]
            $(#[wasm_bindgen(extends = $extend)])*
            pub type $name;
        }
        impl Default for $name {
            fn default() -> Self {
                ::wasm_bindgen::JsCast::unchecked_into(Object::new())
            }
        }
    };
    (
        $(#[$meta:meta])*
        type $name:ident;
    ) => {
        define_object_interface! {
            $(#[$meta])*
            type $name extends ::js_sys::Object;
        }
    };
}
