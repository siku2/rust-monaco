macro_rules! object_get {
    (try $obj:ident.$key:ident) => {
        ::js_sys::Reflect::get($obj.as_ref(), &stringify!($key).into())
    };
    ($obj:ident.$key:ident) => {
        object_get!(try $obj.$key).expect("getting key from object must not fail")
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
    ($obj:ident.$key:ident as Option<$ty:ty>) => {
        ::wasm_bindgen::JsCast::dyn_into(object_get!($obj.$key)).ok()
    };
}

macro_rules! object_set {
    ($obj:ident.$key:ident = $value:expr) => {
        ::js_sys::Reflect::set($obj.as_ref(), &stringify!($key).into(), &Into::into($value))
            .expect("setting key on object must not fail");
    };
}

macro_rules! define_property {
    // get
    (
        $(#[$meta:meta])*
        get $name:ident: $ty:ty
    ) => {
        ::paste::paste! {
            #[doc = "Get the `" $name "` property."]
            $(#[$meta])*
            pub fn [<get_ $name:snake>](&self) -> $ty {
                object_get!(self.$name as $ty)
            }
        }
    };
    // get option enum
    (
        $(#[$meta:meta])*
        get enum $name:ident: Option<$ty:ty>
    ) => {
        ::paste::paste! {
            #[doc = "Get the `" $name "` property."]
            $(#[$meta])*
            pub fn [<get_ $name:snake>](&self) -> Option<$ty> {
                object_get!(self.$name as Option<String>).and_then(|v| $ty::from_value(&v))
            }
        }
    };
    // set
    (
        $(#[$meta:meta])*
        set $name:ident: $ty:ty
    ) => {
        ::paste::paste! {
            #[doc = "Set the `" $name "` property."]
            $(#[$meta])*
            pub fn [<set_ $name:snake>](&self, val: $ty) {
                object_set!(self.$name = val);
            }
        }
    };
    // set option enum
    (
        $(#[$meta:meta])*
        set enum $name:ident: Option<$ty:ty>
    ) => {
        ::paste::paste! {
            #[doc = "Set the `" $name "` property."]
            $(#[$meta])*
            pub fn [<set_ $name:snake>](&self, val: Option<$ty>) {
                object_set!(self.$name = val.map(|v| v.value()));
            }
        }
    };
    // shorthand option string
    (
        $(#[$meta:meta])*
        $name:ident: Option<String>
    ) => {
        define_property!{
            $(#[$meta])*
            get $name: Option<String>
        }
        define_property!{
            set $name: Option<&str>
        }
    };
    // shorthand
    (
        $(#[$meta:meta])*
        $name:ident: $ty:ty
    ) => {
        define_property!{
            $(#[$meta])*
            get $name: $ty
        }
        define_property!{
            set $name: $ty
        }
    };
    // shorthand option enum
    (
        $(#[$meta:meta])*
        enum $name:ident: Option<$ty:ty>
    ) => {
        define_property!{
            $(#[$meta])*
            get enum $name: Option<$ty>
        }
        define_property!{
            set enum $name: Option<$ty>
        }
    };
    // shorthand option ref
    (
        $(#[$meta:meta])*
        ref $name:ident: Option<$ty:ty>
    ) => {
        define_property!{
            $(#[$meta])*
            get $name: Option<$ty>
        }
        define_property!{
            set $name: Option<&$ty>
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
