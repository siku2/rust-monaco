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

macro_rules! int_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant_name:ident = $variant_value:literal,
            )*
        }
    ) => {
            $(#[$meta])*
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            $vis enum $name {
                $(
                    $(#[$variant_meta])*
                    $variant_name
                ),*
            }
            impl $name {
                /// Get the variant for the value.
                /// Returns `None` if the value isn't part of the enum.
                pub fn from_value(val: usize) -> Option<Self> {
                    match val {
                        $(
                            $variant_value => Some(Self::$variant_name),
                        )*
                        _ => None,
                    }
                }

                /// Get the value of the variant.
                pub fn value(&self) -> usize {
                    match self {
                        $(
                            Self::$variant_name => $variant_value
                        ),*
                    }
                }
            }
            impl ::wasm_bindgen::describe::WasmDescribe for $name {
                fn describe() {
                    <usize as ::wasm_bindgen::describe::WasmDescribe>::describe()
                }
            }
            impl ::wasm_bindgen::convert::IntoWasmAbi for $name {
                type Abi = <usize as ::wasm_bindgen::convert::IntoWasmAbi>::Abi;
                fn into_abi(self) -> Self::Abi {
                    <usize as ::wasm_bindgen::convert::IntoWasmAbi>::into_abi(self.value())
                }
            }
            impl ::wasm_bindgen::convert::FromWasmAbi for $name {
                type Abi = <usize as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
                unsafe fn from_abi(js: Self::Abi) -> Self {
                    Self::from_value(<usize as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(js)).expect("received value outside of enum")
                }
            }
        };
}
