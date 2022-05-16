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
        IDimension,
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
use std::borrow::Borrow;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

/// Switches to a theme.
pub fn set_global_theme(theme: &str) {
    editor::set_theme(theme);
}

/// Switches to a built-in theme.
pub fn set_global_builtin_theme(theme: BuiltinTheme) {
    set_global_theme(theme.to_value())
}

macro_rules! simple_setters {
    ($target:ident => ) => {};
    ($target:ident => ref $key:ident, $($tail:tt)*) => {
        ::paste::paste! {
            $target.[<set_ $key>]($key.as_ref().map(|v| v.as_ref()));
        }
        simple_setters!($target => $($tail)*);
    };
    ($target:ident => $key:ident, $($tail:tt)*) => {
        ::paste::paste! {
            $target.[<set_ $key>](*$key);
        }
        simple_setters!($target => $($tail)*);
    };
}

/// Options for creating a new editor. This represents a simplified version of
/// [`IStandaloneEditorConstructionOptions`].
///
/// If you need an option that isn't present you can use
/// [`to_sys_options`](Self::to_sys_options) to
/// build the [`IStandaloneEditorConstructionOptions`] object and expand it.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CodeEditorOptions {
    pub dimension: Option<IDimension>,
    pub theme: Option<String>,
    pub model: Option<TextModel>,
    pub language: Option<String>,
    pub value: Option<String>,
    pub scroll_beyond_last_line: Option<bool>,
    pub automatic_layout: Option<bool>,
}
impl CodeEditorOptions {
    builder_methods! {
        pub with dimension(IDimension);
        pub with theme(String);
        pub with model(TextModel);
        pub with language(String);
        pub with value(String);
        pub with scroll_beyond_last_line(bool);
        pub with automatic_layout(bool);
    }

    pub fn with_builtin_theme(self, theme: BuiltinTheme) -> Self {
        self.with_theme(theme.to_value().to_owned())
    }

    pub fn with_new_dimension(self, width: impl Into<f64>, height: impl Into<f64>) -> Self {
        self.with_dimension(IDimension::new(width, height))
    }

    /// Convert into [`IStandaloneEditorConstructionOptions`].
    pub fn to_sys_options(&self) -> IStandaloneEditorConstructionOptions {
        let options = IStandaloneEditorConstructionOptions::default();

        // this helps ensure we don't miss any members
        let CodeEditorOptions {
            dimension,
            theme,
            model,
            language,
            value,
            scroll_beyond_last_line,
            automatic_layout,
        } = self;

        simple_setters! {
            options =>
                ref language,
                ref dimension,
                ref theme,
                ref model,
                ref value,
                scroll_beyond_last_line,
                automatic_layout,
        }

        options
    }
}

impl From<CodeEditorOptions> for IStandaloneEditorConstructionOptions {
    fn from(options: CodeEditorOptions) -> Self {
        options.to_sys_options()
    }
}

/// Monaco code editor.
///
/// This struct should be the sole owner of the underlying
/// [`IStandaloneCodeEditor`] because it will call
/// [`dispose`](IStandaloneCodeEditor::dispose) when dropped.
/// This is only an issue when using the [`From`] trait.
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

    /// Create a new editor under `element`.
    /// `element` should be empty (not contain other dom nodes).
    /// The editor will read the size of `element`.
    pub fn create<OPT>(element: &HtmlElement, options: Option<OPT>) -> Self
    where
        OPT: Into<IStandaloneEditorConstructionOptions>,
    {
        #[cfg(feature = "workers")]
        crate::workers::ensure_environment_set();

        let ioptions = options.map(|x| x.into());
        let options = ioptions.as_ref().map(Borrow::borrow);
        let js_editor = editor::create(element, options, None);
        Self::from(js_editor)
    }

    /// Create a new editor under `element`.
    /// `element` should be empty (not contain other dom nodes).
    /// The editor will read the size of `element`.
    #[deprecated(since = "0.3.0", note = "Use `create` instead")]
    pub fn create_with_sys_options(
        element: &HtmlElement,
        options: Option<IStandaloneEditorConstructionOptions>,
    ) -> Self {
        Self::create(element, options)
    }

    /// Gets the current model attached to this editor.
    pub fn get_model(&self) -> Option<TextModel> {
        self.js_editor.get_model().map(TextModel::from)
    }

    /// Sets the current model attached to this editor.
    /// If the previous model was created by the editor via the `value` key in
    /// the options, it will be destroyed. Otherwise, if the previous model was
    /// set via this method, or the `model` key in the options, the
    /// previous model will not be destroyed.
    pub fn set_model(&self, model: &TextModel) {
        self.js_editor.set_model(Some(model.as_ref()))
    }

    /// Detaches the current model from the editor and returns it.
    /// The handling of the model is the same as described in
    /// [`set_model`](Self::set_model). This operation acts like
    /// `setModel(null)` in the Javascript API.
    pub fn detach_model(&self) -> Option<TextModel> {
        let model = self.get_model();
        self.js_editor.set_model(None);
        model
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
