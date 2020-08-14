/// Rust bindings for the Monaco text editor.
#[macro_use]
mod macros;

mod editor;
pub mod sys;
#[cfg(feature = "embed_workers")]
mod workers;
#[cfg(feature = "yew")]
pub mod yew;

// TODO find a better way to do this
/// Initialize the Monaco environment.
/// If the feature "embed_workers" is given this will load the embedded workers.
pub fn init_environment() {
    #[cfg(feature = "embed_workers")]
    workers::init_environment();
}
