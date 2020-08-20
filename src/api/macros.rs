// creates `with_xxx` methods
macro_rules! builder_methods {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis with $ident:ident($ty:ty);
        )+
    ) => {
        $(
            ::paste::paste! {
                $vis fn [<with_ $ident>](mut self, val: $ty) -> Self {
                    self.$ident = Some(val);
                    self
                }
            }
        )*
    };
}

macro_rules! event_methods {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis $ident:ident($($ty:tt)*);
        )*
    ) => {
        $(
            $(#[$meta])*
            $vis fn $ident(
                &self,
                listener: impl $($ty)* + 'static,
            ) -> $crate::api::DisposableClosure<dyn $($ty)*> {
                let cb = ::wasm_bindgen::closure::Closure::wrap(Box::new(listener) as Box<dyn $($ty)*>);
                let js_disposable = self
                    .as_ref()
                    .$ident(::wasm_bindgen::JsCast::unchecked_ref(cb.as_ref()));
                $crate::api::DisposableClosure::new(cb, js_disposable)
            }
        )*
    };
}
