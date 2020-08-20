use super::TextModel;
use crate::sys::{
    editor::{
        self,
        BuiltinTheme,
        ConfigurationChangedEvent,
        EditorLayoutInfo,
        IContentSizeChangedEvent,
        ICursorPositionChangedEvent,
        ICursorSelectionChangedEvent,
        IEditorMouseEvent,
        IModelChangedEvent,
        IModelContentChangedEvent,
        IModelLanguageChangedEvent,
        IModelOptionsChangedEvent,
        IPasteEvent,
        IStandaloneCodeEditor,
        IStandaloneEditorConstructionOptions,
    },
    IKeyboardEvent,
    IScrollEvent,
};
use serde::Serialize;
use std::borrow::Borrow;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlElement;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeEditorOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    js_editor: IStandaloneCodeEditor,
}
impl CodeEditor {
    event_methods! {
        /// An event emitted on a "contextmenu".
        pub on_context_menu(FnMut(IEditorMouseEvent));
        /// An event emitted when the text inside this editor lost focus (i.e. cursor stops blinking).
        pub on_did_blur_editor_text(FnMut());
        /// An event emitted when the text inside this editor or an editor widget lost focus.
        pub on_did_blur_editor_widget(FnMut());
        /// An event emitted when the configuration of the editor has changed. (e.g. editor.updateOptions())
        pub on_did_change_configuration(FnMut(ConfigurationChangedEvent));
        /// An event emitted when the cursor position has changed.
        pub on_did_change_cursor_position(FnMut(ICursorPositionChangedEvent));
        /// An event emitted when the cursor selection has changed.
        pub on_did_change_cursor_selection(FnMut(ICursorSelectionChangedEvent));
        /// An event emitted when the model of this editor has changed (e.g. editor.setModel()).
        pub on_did_change_model(FnMut(IModelChangedEvent));
        /// An event emitted when the content of the current model has changed.
        pub on_did_change_model_content(FnMut(IModelContentChangedEvent));
        /// An event emitted when the decorations of the current model have changed.
        pub on_did_change_model_decorations(FnMut(JsValue));
        /// An event emitted when the language of the current model has changed.
        pub on_did_change_model_language(FnMut(IModelLanguageChangedEvent));
        /// An event emitted when the language configuration of the current model has changed.
        pub on_did_change_model_language_configuration(FnMut(JsValue));
        /// An event emitted when the options of the current model has changed.
        pub on_did_change_model_options(FnMut(IModelOptionsChangedEvent));
        /// An event emitted when the content width or content height in the editor has changed.
        pub on_did_content_size_change(FnMut(IContentSizeChangedEvent));
        /// An event emitted when the editor has been disposed.
        pub on_did_dispose(FnMut());
        /// An event emitted when the text inside this editor gained focus (i.e. cursor starts blinking).
        pub on_did_focus_editor_text(FnMut());
        /// An event emitted when the text inside this editor or an editor widget gained focus.
        pub on_did_focus_editor_widget(FnMut());
        /// An event emitted when the layout of the editor has changed.
        pub on_did_layout_change(FnMut(EditorLayoutInfo));
        /// An event emitted when users paste text in the editor.
        pub on_did_paste(FnMut(IPasteEvent));
        /// An event emitted when the scroll in the editor has changed.
        pub on_did_scroll_change(FnMut(IScrollEvent));
        /// An event emitted on a "keydown".
        pub on_key_down(FnMut(IKeyboardEvent));
        /// An event emitted on a "keyup".
        pub on_key_up(FnMut(IKeyboardEvent));
        /// An event emitted on a "mousedown".
        pub on_mouse_down(FnMut(IEditorMouseEvent));
        /// An event emitted on a "mouseleave".
        pub on_mouse_leave(FnMut(IEditorMouseEvent));
        /// An event emitted on a "mousemove".
        pub on_mouse_move(FnMut(IEditorMouseEvent));
        /// An event emitted on a "mouseup".
        pub on_mouse_up(FnMut(IEditorMouseEvent));
    }

    /// Create a new editor under `domElement`.
    /// `domElement` should be empty (not contain other dom nodes).
    /// The editor will read the size of `domElement`.
    pub fn create<OPT>(element: &HtmlElement, options: Option<OPT>) -> Self
    where
        OPT: Borrow<CodeEditorOptions>,
    {
        #[cfg(feature = "workers")]
        crate::workers::ensure_environment_set();

        let options = options.as_ref().map(Borrow::borrow).map(Into::into);
        let js_editor = editor::create(element, options.as_ref(), None);
        Self::from(js_editor)
    }

    /// Gets the current model attached to this editor.
    pub fn get_model(&self) -> Option<TextModel> {
        self.js_editor.get_model().map(TextModel::from)
    }
}
impl Drop for CodeEditor {
    fn drop(&mut self) {
        self.js_editor.dispose();
    }
}

impl AsRef<IStandaloneCodeEditor> for CodeEditor {
    fn as_ref(&self) -> &IStandaloneCodeEditor {
        &self.js_editor
    }
}
impl From<IStandaloneCodeEditor> for CodeEditor {
    fn from(js_editor: IStandaloneCodeEditor) -> Self {
        Self { js_editor }
    }
}
