pub use monaco::*;

mod monaco;
#[cfg(feature = "embed_workers")]
mod workers;
#[cfg(feature = "yew")]
pub mod yew;

pub fn init_environment() {
    #[cfg(feature = "embed_workers")]
    set_environment(&workers::build_environment());
}
