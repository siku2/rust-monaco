//! Rust WASM bindings for the Monaco Editor.
#[macro_use]
mod macros;
#[cfg(feature = "api")]
pub mod api;
pub mod sys;
#[cfg(feature = "workers")]
pub mod workers;
#[cfg(feature = "yew-components")]
pub mod yew;
