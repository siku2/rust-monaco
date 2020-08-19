//! Simplified API.
//! Requires the `api` feature.
use crate::sys::IDisposable;
pub use editor::*;
pub use model::*;
use wasm_bindgen::closure::Closure;

mod editor;
mod model;

/// A [`Closure`] that is tied to an [`IDisposable`].
#[must_use = "immediately disposed when dropped"]
#[derive(Debug)]
pub struct DisposableClosure<T: ?Sized> {
    closure: Closure<T>,
    js_disposable: IDisposable,
}
impl<T: ?Sized> DisposableClosure<T> {
    fn new(closure: Closure<T>, js_disposable: IDisposable) -> Self {
        Self {
            closure,
            js_disposable,
        }
    }
}
impl<T: ?Sized> Drop for DisposableClosure<T> {
    fn drop(&mut self) {
        self.js_disposable.dispose();
    }
}
