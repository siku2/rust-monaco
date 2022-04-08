//! Simplified API.
//! Requires the "api" feature (enabled by default).
//!
//! Most of the types here are simply wrappers around the Javascript types and
//! implement [`AsRef`] for them.
use crate::sys::IDisposable;
pub use editor::*;
pub use model::*;
use wasm_bindgen::closure::Closure;

#[macro_use]
mod macros;

mod editor;
mod model;

/// A [`Closure`] that is tied to an [`IDisposable`].
#[must_use = "immediately disposed when dropped"]
#[derive(Debug)]
pub struct DisposableClosure<T: ?Sized> {
    _closure: Closure<T>,
    js_disposable: IDisposable,
}
impl<T: ?Sized> DisposableClosure<T> {
    pub fn new(closure: Closure<T>, js_disposable: IDisposable) -> Self {
        Self {
            _closure: closure,
            js_disposable,
        }
    }
}
impl<T: ?Sized> Drop for DisposableClosure<T> {
    fn drop(&mut self) {
        self.js_disposable.dispose();
    }
}

impl<T> AsRef<IDisposable> for DisposableClosure<T> {
    fn as_ref(&self) -> &IDisposable {
        &self.js_disposable
    }
}
