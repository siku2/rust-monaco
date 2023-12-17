pub mod exports {
    pub use wasm_bindgen::{
        convert::{FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi},
        describe::WasmDescribe,
        JsValue,
    };
}

macro_rules! _lit_enum_commons {
    (
        $from_ty:ty, $to_ty:ty;
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
                #[allow(unreachable_patterns)]
                pub fn from_value(val: $from_ty) -> Option<Self> {
                    match val {
                        $(
                            $variant_value => Some(Self::$variant_name),
                        )*
                        _ => None,
                    }
                }

                /// Get the value of the variant.
                pub fn to_value(&self) -> $to_ty {
                    match self {
                        $(
                            Self::$variant_name => $variant_value
                        ),*
                    }
                }
            }
            impl $crate::macros::exports::WasmDescribe for $name {
                fn describe() {
                    <$crate::macros::exports::JsValue as $crate::macros::exports::WasmDescribe>::describe()
                }
            }
            impl $crate::macros::exports::OptionFromWasmAbi for $name {
                fn is_none(abi: &Self::Abi) -> bool {
                    // SAFETY: this isn't any more unsafe than the FromWasmAbi implementation is in the first place.
                    let js_value = unsafe { <$crate::macros::exports::JsValue as $crate::macros::exports::FromWasmAbi>::from_abi(*abi) };
                    js_value.is_null() || js_value.is_undefined()
                }
            }
            impl $crate::macros::exports::OptionIntoWasmAbi for $name {
                fn none() -> Self::Abi {
                    let value = $crate::macros::exports::JsValue::undefined();
                    <$crate::macros::exports::JsValue as $crate::macros::exports::IntoWasmAbi>::into_abi(value)
                }
            }
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
        _lit_enum_commons! {
            i32, i32;
            $(#[$meta])*
            $vis enum $name {
                $(
                    $(#[$variant_meta])*
                    $variant_name = $variant_value,
                )*
            }
        }
        impl $crate::macros::exports::FromWasmAbi for $name {
            type Abi = <$crate::macros::exports::JsValue as $crate::macros::exports::FromWasmAbi>::Abi;

            unsafe fn from_abi(abi: Self::Abi) -> Self {
                let js_value = <$crate::macros::exports::JsValue as $crate::macros::exports::FromWasmAbi>::from_abi(abi);
                let value = js_value.as_f64().expect("received non-number") as i32;
                Self::from_value(value).expect("received value outside of enum")
            }
        }
        impl ::wasm_bindgen::convert::IntoWasmAbi for $name {
            type Abi = <$crate::macros::exports::JsValue as $crate::macros::exports::IntoWasmAbi>::Abi;

            fn into_abi(self) -> Self::Abi {
                let value = Into::<$crate::macros::exports::JsValue>::into(self.to_value());
                <$crate::macros::exports::JsValue as $crate::macros::exports::IntoWasmAbi>::into_abi(value)
            }
        }
    };
}
