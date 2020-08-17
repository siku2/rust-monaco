//! Rust bindings for the Monaco text editor.
#[macro_use]
mod macros;
mod editor;
#[cfg(feature = "embed_workers")]
pub mod embedded;
pub mod sys;
#[cfg(feature = "yew")]
pub mod yew;
