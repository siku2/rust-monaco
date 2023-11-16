pub mod class;
pub mod r#enum;
pub mod interface;
pub mod namespace;
pub mod translate;

use std::{
    collections::BTreeMap,
    io::Write,
    ops::{Deref, DerefMut},
    rc::Rc,
};
use swc_common::comments::Comments;

pub struct ModuleContext {
    pub comments: Rc<dyn Comments>,
    pub data: Vec<u8>,
    pub modules: BTreeMap<String, ModuleContext>,
}

impl<'a> ModuleContext {
    pub fn new(comments: Rc<dyn Comments>) -> Self {
        Self {
            comments,
            data: vec![],
            modules: Default::default(),
        }
    }

    pub fn push(&mut self, name: impl Into<String>) -> &mut ModuleContext {
        self.modules
            .entry(name.into())
            .or_insert_with(|| Self::new(self.comments.clone()))
    }

    pub fn flush<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        w.write_all(&self.data)?;

        for (k, v) in &self.modules {
            writeln!(w, "pub mod {k} {{")?;
            writeln!(
                w,
                r#"
#[allow(unused)]
use wasm_bindgen::prelude::*;
#[allow(unused)]
use web_sys::*;
#[allow(unused)]
use js_sys::*;
#[allow(unused)]
use super::*;

#[allow(unused)]
pub type HTMLElement = HtmlElement;
#[allow(unused)]
pub type ReadonlyArray = Array;
#[allow(unused)]
pub type NonNullable = JsValue;
#[allow(unused)]
pub type Record = JsValue;
#[allow(unused)]
pub type PromiseLike = JsValue;

"#
            )?;
            v.flush(w)?;
            writeln!(w, "}}")?;
        }

        Ok(())
    }
}

impl Deref for ModuleContext {
    type Target = dyn Write;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for ModuleContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
