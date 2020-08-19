use crate::sys::editor::{
    self,
    BuiltinTheme,
    IStandaloneCodeEditor,
    IStandaloneEditorConstructionOptions,
};
use serde::Serialize;
use std::borrow::Borrow;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlElement;

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

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeEditorOptions {
    pub value: Option<String>,
    pub language: Option<String>,
    pub theme: Option<String>,
}
impl CodeEditorOptions {
    builder_methods! {
        pub with value(String);
        pub with language(String);
        pub with theme(String);
    }

    pub fn with_builtin_theme(self, theme: BuiltinTheme) -> Self {
        self.with_theme(theme.to_value().to_owned())
    }
}
impl Into<IStandaloneEditorConstructionOptions> for &CodeEditorOptions {
    fn into(self) -> IStandaloneEditorConstructionOptions {
        JsValue::from_serde(self).unwrap().unchecked_into()
    }
}

#[must_use = "editor is disposed when dropped"]
#[derive(Debug)]
pub struct CodeEditor {
    editor: IStandaloneCodeEditor,
}
impl CodeEditor {
    pub fn create<OPT>(element: &HtmlElement, options: Option<OPT>) -> Self
    where
        OPT: Borrow<CodeEditorOptions>,
    {
        #[cfg(feature = "workers")]
        crate::workers::ensure_environment_set();

        let options = options.as_ref().map(Borrow::borrow).map(Into::into);
        let editor = editor::create(element, options.as_ref(), None);
        Self { editor }
    }
}
impl Drop for CodeEditor {
    fn drop(&mut self) {
        self.editor.dispose();
    }
}
