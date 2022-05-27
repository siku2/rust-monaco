//! Bindings for the `monaco.editor` namespace.
use super::{
    IDisposable,
    IMouseEvent,
    IPosition,
    IRange,
    ISelection,
    MarkerSeverity,
    Position,
    Range,
    Selection,
    Uri,
};
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlElement;

impl IDimension {
    /// Create a new dimension with the specified width and height
    pub fn new(width: impl Into<f64>, height: impl Into<f64>) -> Self {
        let dim: Self = Object::new().unchecked_into();
        dim.set_width(width.into());
        dim.set_height(height.into());
        dim
    }
}

macro_rules! impl_default_empty_obj {
    ($($ty:ty,)*) => {
        $(
            impl Default for $ty {
                fn default() -> Self {
                    Object::new().unchecked_into()
                }
            }
        )*
    };
}

impl_default_empty_obj![
    IColorizerElementOptions,
    IColorizerOptions,
    IDiffEditorConstructionOptions,
    IDiffNavigatorOptions,
    IGlobalEditorOptions,
    IModelDecorationOptions,
    ISuggestOptions,
    IEditorScrollbarOptions,
    IEditorFindOptions,
    IEditorMinimapOptions,
    IStandaloneEditorConstructionOptions,
];

// DANGER: Generated code ahead. Keep out!

#[cfg_attr(debug_assertions, wasm_bindgen(module = "/js/debug/editor.js"))]
#[cfg_attr(not(debug_assertions), wasm_bindgen(module = "/js/release/editor.js"))]
extern "C" {
    /// Create a new editor under `domElement`.
    /// `domElement` should be empty (not contain other dom nodes).
    /// The editor will read the size of `domElement`.
    ///
    /// # Arguments
    ///
    /// * `override` - `{ [index: string]: any }`
    #[wasm_bindgen(js_name = "create", js_namespace = editor)]
    pub fn create(
        dom_element: &HtmlElement,
        options: Option<&IStandaloneEditorConstructionOptions>,
        override_: Option<&Object>,
    ) -> IStandaloneCodeEditor;

    /// Emitted when an editor is created.
    /// Creating a diff editor might cause this listener to be invoked with the
    /// two editors. @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(codeEditor: ICodeEditor) => void`
    #[wasm_bindgen(js_name = "onDidCreateEditor", js_namespace = editor)]
    pub fn on_did_create_editor(listener: &Function) -> IDisposable;

    /// Create a new diff editor under `domElement`.
    /// `domElement` should be empty (not contain other dom nodes).
    /// The editor will read the size of `domElement`.
    ///
    /// # Arguments
    ///
    /// * `override` - `{ [index: string]: any }`
    #[wasm_bindgen(js_name = "createDiffEditor", js_namespace = editor)]
    pub fn create_diff_editor(
        dom_element: &HtmlElement,
        options: Option<&IDiffEditorConstructionOptions>,
        override_: Option<&Object>,
    ) -> IStandaloneDiffEditor;

    #[wasm_bindgen(js_name = "createDiffNavigator", js_namespace = editor)]
    pub fn create_diff_navigator(
        diff_editor: &IStandaloneDiffEditor,
        opts: Option<&IDiffNavigatorOptions>,
    ) -> IDiffNavigator;

    /// Create a new editor model.
    /// You can specify the language that should be set for this model or let
    /// the language be inferred from the `uri`.
    #[wasm_bindgen(js_name = "createModel", js_namespace = editor, catch)]
    pub fn create_model(
        value: &str,
        language: Option<&str>,
        uri: Option<&Uri>,
    ) -> Result<ITextModel, JsValue>;

    /// Change the language for a model.
    #[wasm_bindgen(js_name = "setModelLanguage", js_namespace = editor)]
    pub fn set_model_language(model: &ITextModel, language_id: &str);

    /// Set the markers for a model.
    ///
    /// # Arguments
    ///
    /// * `markers` - `IMarkerData[]`
    #[wasm_bindgen(js_name = "setModelMarkers", js_namespace = editor)]
    pub fn set_model_markers(model: &ITextModel, owner: &str, markers: &Array);

    /// Get markers for owner and/or resource
    ///
    /// @returns list of markers
    ///
    /// # Arguments
    ///
    /// * `filter` - `{ owner?: string; resource?: Uri; take?: number; }`
    ///
    /// # Returns
    ///
    /// `IMarker[]`
    #[wasm_bindgen(js_name = "getModelMarkers", js_namespace = editor)]
    pub fn get_model_markers(filter: &Object) -> Array;

    /// Get the model that has `uri` if it exists.
    #[wasm_bindgen(js_name = "getModel", js_namespace = editor)]
    pub fn get_model(uri: &Uri) -> Option<ITextModel>;

    /// Get all the created models.
    ///
    /// # Returns
    ///
    /// `ITextModel[]`
    #[wasm_bindgen(js_name = "getModels", js_namespace = editor)]
    pub fn get_models() -> Array;

    /// Emitted when a model is created.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(model: ITextModel) => void`
    #[wasm_bindgen(js_name = "onDidCreateModel", js_namespace = editor)]
    pub fn on_did_create_model(listener: &Function) -> IDisposable;

    /// Emitted right before a model is disposed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(model: ITextModel) => void`
    #[wasm_bindgen(js_name = "onWillDisposeModel", js_namespace = editor)]
    pub fn on_will_dispose_model(listener: &Function) -> IDisposable;

    /// Emitted when a different language is set to a model.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: { readonly model: ITextModel; readonly oldLanguage:
    ///   string; }) => void`
    #[wasm_bindgen(js_name = "onDidChangeModelLanguage", js_namespace = editor)]
    pub fn on_did_change_model_language(listener: &Function) -> IDisposable;

    /// Create a new web worker that has model syncing capabilities built in.
    /// Specify an AMD module to load that will `create` an object that will be
    /// proxied.
    #[wasm_bindgen(js_name = "createWebWorker", js_namespace = editor)]
    pub fn create_web_worker(opts: &IWebWorkerOptions) -> MonacoWebWorker;

    /// Colorize the contents of `domNode` using attribute `data-lang`.
    #[wasm_bindgen(js_name = "colorizeElement", js_namespace = editor)]
    pub fn colorize_element(dom_node: &HtmlElement, options: &IColorizerElementOptions) -> Promise;

    /// Colorize `text` using language `languageId`.
    #[wasm_bindgen(js_name = "colorize", js_namespace = editor)]
    pub fn colorize(text: &str, language_id: &str, options: &IColorizerOptions) -> Promise;

    /// Colorize a line in a model.
    #[wasm_bindgen(js_name = "colorizeModelLine", js_namespace = editor)]
    pub fn colorize_model_line(
        model: &ITextModel,
        line_number: f64,
        tab_size: Option<f64>,
    ) -> String;

    /// Tokenize `text` using language `languageId`
    ///
    /// # Returns
    ///
    /// `Token[][]`
    #[wasm_bindgen(js_name = "tokenize", js_namespace = editor)]
    pub fn tokenize(text: &str, language_id: &str) -> Array;

    /// Define a new theme or update an existing theme.
    #[wasm_bindgen(js_name = "defineTheme", js_namespace = editor, catch)]
    pub fn define_theme(theme_name: &str, theme_data: &IStandaloneThemeData)
        -> Result<(), JsValue>;

    /// Switches to a theme.
    #[wasm_bindgen(js_name = "setTheme", js_namespace = editor)]
    pub fn set_theme(theme_name: &str);

    /// Clears all cached font measurements and triggers re-measurement.
    #[wasm_bindgen(js_name = "remeasureFonts", js_namespace = editor)]
    pub fn remeasure_fonts();

    #[derive(Debug)]
    #[wasm_bindgen(js_namespace = editor)]
    pub type TextModelResolvedOptions;
    #[wasm_bindgen(method, js_class = "TextModelResolvedOptions", js_name = "_textModelResolvedOptionsBrand", getter = _textModelResolvedOptionsBrand)]
    pub fn _text_model_resolved_options_brand(this: &TextModelResolvedOptions);
    #[wasm_bindgen(method, js_class = "TextModelResolvedOptions", js_name = "tabSize", getter = tabSize)]
    pub fn tab_size(this: &TextModelResolvedOptions) -> f64;
    #[wasm_bindgen(method, js_class = "TextModelResolvedOptions", js_name = "indentSize", getter = indentSize)]
    pub fn indent_size(this: &TextModelResolvedOptions) -> f64;
    #[wasm_bindgen(method, js_class = "TextModelResolvedOptions", js_name = "insertSpaces", getter = insertSpaces)]
    pub fn insert_spaces(this: &TextModelResolvedOptions) -> bool;
    #[wasm_bindgen(method, js_class = "TextModelResolvedOptions", js_name = "defaultEOL", getter = defaultEOL)]
    pub fn default_eol(this: &TextModelResolvedOptions) -> DefaultEndOfLine;
    #[wasm_bindgen(method, js_class = "TextModelResolvedOptions", js_name = "trimAutoWhitespace", getter = trimAutoWhitespace)]
    pub fn trim_auto_whitespace(this: &TextModelResolvedOptions) -> bool;

    #[derive(Debug)]
    #[wasm_bindgen(js_namespace = editor)]
    pub type FindMatch;
    #[wasm_bindgen(method, js_class = "FindMatch", js_name = "_findMatchBrand", getter = _findMatchBrand)]
    pub fn _find_match_brand(this: &FindMatch);
    #[wasm_bindgen(method, js_class = "FindMatch", js_name = "range", getter = range)]
    pub fn range(this: &FindMatch) -> Range;
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "FindMatch", js_name = "matches", getter = matches)]
    pub fn matches(this: &FindMatch) -> Option<Array>;

    /// An event describing that the configuration of the editor has changed.
    #[derive(Debug)]
    #[wasm_bindgen(js_namespace = editor)]
    pub type ConfigurationChangedEvent;
    #[wasm_bindgen(method, js_class = "ConfigurationChangedEvent", js_name = "hasChanged")]
    pub fn has_changed(this: &ConfigurationChangedEvent, id: EditorOption) -> bool;

    #[derive(Debug)]
    #[wasm_bindgen(extends = BareFontInfo, js_namespace = editor)]
    pub type FontInfo;
    #[wasm_bindgen(method, js_class = "FontInfo", js_name = "_editorStylingBrand", getter = _editorStylingBrand)]
    pub fn _editor_styling_brand(this: &FontInfo);
    #[wasm_bindgen(method, js_class = "FontInfo", js_name = "isTrusted", getter = isTrusted)]
    pub fn is_trusted(this: &FontInfo) -> bool;
    #[wasm_bindgen(method, js_class = "FontInfo", js_name = "isMonospace", getter = isMonospace)]
    pub fn is_monospace(this: &FontInfo) -> bool;
    #[wasm_bindgen(method, js_class = "FontInfo", js_name = "typicalHalfwidthCharacterWidth", getter = typicalHalfwidthCharacterWidth)]
    pub fn typical_halfwidth_character_width(this: &FontInfo) -> f64;
    #[wasm_bindgen(method, js_class = "FontInfo", js_name = "typicalFullwidthCharacterWidth", getter = typicalFullwidthCharacterWidth)]
    pub fn typical_fullwidth_character_width(this: &FontInfo) -> f64;
    #[wasm_bindgen(method, js_class = "FontInfo", js_name = "canUseHalfwidthRightwardsArrow", getter = canUseHalfwidthRightwardsArrow)]
    pub fn can_use_halfwidth_rightwards_arrow(this: &FontInfo) -> bool;
    #[wasm_bindgen(method, js_class = "FontInfo", js_name = "spaceWidth", getter = spaceWidth)]
    pub fn space_width(this: &FontInfo) -> f64;
    #[wasm_bindgen(method, js_class = "FontInfo", js_name = "middotWidth", getter = middotWidth)]
    pub fn middot_width(this: &FontInfo) -> f64;
    #[wasm_bindgen(method, js_class = "FontInfo", js_name = "maxDigitWidth", getter = maxDigitWidth)]
    pub fn max_digit_width(this: &FontInfo) -> f64;

    #[derive(Debug)]
    #[wasm_bindgen(js_namespace = editor)]
    pub type BareFontInfo;
    #[wasm_bindgen(method, js_class = "BareFontInfo", js_name = "_bareFontInfoBrand", getter = _bareFontInfoBrand)]
    pub fn _bare_font_info_brand(this: &BareFontInfo);
    #[wasm_bindgen(method, js_class = "BareFontInfo", js_name = "zoomLevel", getter = zoomLevel)]
    pub fn zoom_level(this: &BareFontInfo) -> f64;
    #[wasm_bindgen(method, js_class = "BareFontInfo", js_name = "fontFamily", getter = fontFamily)]
    pub fn font_family(this: &BareFontInfo) -> String;
    #[wasm_bindgen(method, js_class = "BareFontInfo", js_name = "fontWeight", getter = fontWeight)]
    pub fn font_weight(this: &BareFontInfo) -> String;
    #[wasm_bindgen(method, js_class = "BareFontInfo", js_name = "fontSize", getter = fontSize)]
    pub fn font_size(this: &BareFontInfo) -> f64;
    #[wasm_bindgen(method, js_class = "BareFontInfo", js_name = "fontFeatureSettings", getter = fontFeatureSettings)]
    pub fn font_feature_settings(this: &BareFontInfo) -> String;
    #[wasm_bindgen(method, js_class = "BareFontInfo", js_name = "lineHeight", getter = lineHeight)]
    pub fn line_height(this: &BareFontInfo) -> f64;
    #[wasm_bindgen(method, js_class = "BareFontInfo", js_name = "letterSpacing", getter = letterSpacing)]
    pub fn letter_spacing(this: &BareFontInfo) -> f64;
}

str_enum! {
    pub enum BuiltinTheme {
        Vs = "vs",
        VsDark = "vs-dark",
        HcBlack = "hc-black",
    }
}

int_enum! {
    pub enum ScrollbarVisibility {
        Auto = 1,
        Hidden = 2,
        Visible = 3,
    }
}

int_enum! {
    pub enum OverviewRulerLane {
        Left = 1,
        Center = 2,
        Right = 4,
        Full = 7,
    }
}

int_enum! {
    pub enum MinimapPosition {
        Inline = 1,
        Gutter = 2,
    }
}

int_enum! {
    pub enum EndOfLinePreference {
        /// Use the end of line character identified in the text buffer.
        Textdefined = 0,
        /// Use line feed (\n) as the end of line character.
        Lf = 1,
        /// Use carriage return and line feed (\r\n) as the end of line character.
        Crlf = 2,
    }
}

int_enum! {
    pub enum DefaultEndOfLine {
        /// Use line feed (\n) as the end of line character.
        Lf = 1,
        /// Use carriage return and line feed (\r\n) as the end of line character.
        Crlf = 2,
    }
}

int_enum! {
    pub enum EndOfLineSequence {
        /// Use line feed (\n) as the end of line character.
        Lf = 0,
        /// Use carriage return and line feed (\r\n) as the end of line character.
        Crlf = 1,
    }
}

int_enum! {
    pub enum TrackedRangeStickiness {
        Alwaysgrowswhentypingatedges = 0,
        Nevergrowswhentypingatedges = 1,
        Growsonlywhentypingbefore = 2,
        Growsonlywhentypingafter = 3,
    }
}

int_enum! {
    pub enum ScrollType {
        Smooth = 0,
        Immediate = 1,
    }
}

int_enum! {
    pub enum CursorChangeReason {
        /// Unknown or not set.
        Notset = 0,
        /// A `model.setValue()` was called.
        Contentflush = 1,
        /// The `model` has been changed outside of this cursor and the cursor recovers its position from associated markers.
        Recoverfrommarkers = 2,
        /// There was an explicit user gesture.
        Explicit = 3,
        /// There was a Paste.
        Paste = 4,
        /// There was an Undo.
        Undo = 5,
        /// There was a Redo.
        Redo = 6,
    }
}

int_enum! {
    pub enum AccessibilitySupport {
        /// This should be the browser case where it is not known if a screen reader is attached or no.
        Unknown = 0,
        Disabled = 1,
        Enabled = 2,
    }
}

str_enum! {
    pub enum EditorAutoClosingStrategy {
        Always = "always",
        LanguageDefined = "languageDefined",
        BeforeWhitespace = "beforeWhitespace",
        Never = "never",
    }
}

str_enum! {
    pub enum EditorAutoSurroundStrategy {
        LanguageDefined = "languageDefined",
        Quotes = "quotes",
        Brackets = "brackets",
        Never = "never",
    }
}

str_enum! {
    pub enum EditorAutoClosingOvertypeStrategy {
        Always = "always",
        Auto = "auto",
        Never = "never",
    }
}

int_enum! {
    pub enum EditorAutoIndentStrategy {
        None = 0,
        Keep = 1,
        Brackets = 2,
        Advanced = 3,
        Full = 4,
    }
}

int_enum! {
    pub enum TextEditorCursorBlinkingStyle {
        /// Hidden
        Hidden = 0,
        /// Blinking
        Blink = 1,
        /// Blinking with smooth fading
        Smooth = 2,
        /// Blinking with prolonged filled state and smooth fading
        Phase = 3,
        /// Expand collapse animation on the y axis
        Expand = 4,
        /// No-Blinking
        Solid = 5,
    }
}

int_enum! {
    pub enum TextEditorCursorStyle {
        /// As a vertical line (sitting between two characters).
        Line = 1,
        /// As a block (sitting on top of a character).
        Block = 2,
        /// As a horizontal line (sitting under a character).
        Underline = 3,
        /// As a thin vertical line (sitting between two characters).
        Linethin = 4,
        /// As an outlined block (sitting on top of a character).
        Blockoutline = 5,
        /// As a thin horizontal line (sitting under a character).
        Underlinethin = 6,
    }
}

str_enum! {
    pub enum GoToLocationValues {
        Peek = "peek",
        GotoAndPeek = "gotoAndPeek",
        Goto = "goto",
    }
}

int_enum! {
    pub enum RenderMinimap {
        None = 0,
        Text = 1,
        Blocks = 2,
    }
}

pub enum LineNumbersType {
    On,
    Off,
    Relative,
    Interval,

    /// Custom function with signature `(lineNumber: number) => string`
    Custom(js_sys::Function),
}
impl LineNumbersType {
    /// Get the variant for the value.
    /// Returns `None` if the value isn't part of the enum.
    pub fn from_value(val: &str) -> Option<Self> {
        match val {
            "on" => Some(LineNumbersType::On),
            "off" => Some(LineNumbersType::Off),
            "relative" => Some(LineNumbersType::Relative),
            "interval" => Some(LineNumbersType::Interval),
            _ => None,
        }
    }
}
impl LineNumbersType {
    /// Get the value of the variant.
    pub fn to_value(&self) -> &'static str {
        match self {
            LineNumbersType::On => "on",
            LineNumbersType::Off => "off",
            LineNumbersType::Relative => "relative",
            LineNumbersType::Interval => "interval",
            LineNumbersType::Custom(_) => panic!("This variant is not representable by string"),
        }
    }
}
impl crate::macros::exports::WasmDescribe for LineNumbersType {
    fn describe() {
        <JsValue as crate::macros::exports::WasmDescribe>::describe()
    }
}
impl crate::macros::exports::OptionFromWasmAbi for LineNumbersType {
    fn is_none(abi: &Self::Abi) -> bool {
        // SAFETY: this isn't any more unsafe than the FromWasmAbi implementation is in
        // the first place.
        let js_value = unsafe { <JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(*abi) };
        js_value.is_null() || js_value.is_undefined()
    }
}
impl crate::macros::exports::OptionIntoWasmAbi for LineNumbersType {
    fn none() -> Self::Abi {
        let value = JsValue::undefined();
        <JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value)
    }
}
impl wasm_bindgen::convert::FromWasmAbi for LineNumbersType {
    type Abi = <crate::macros::exports::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;

    unsafe fn from_abi(abi: Self::Abi) -> Self {
        let js_value = <JsValue as crate::macros::exports::FromWasmAbi>::from_abi(abi);
        if let Some(value) = js_value.as_string() {
            Self::from_value(&value).expect("received value outside of enum")
        } else {
            Self::Custom(js_value.into())
        }
    }
}
impl wasm_bindgen::convert::IntoWasmAbi for LineNumbersType {
    type Abi = <JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        let value: JsValue = match self {
            Self::On | Self::Off | Self::Relative | Self::Interval => self.to_value().into(),
            Self::Custom(value) => value.into(),
        };
        <JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value)
    }
}

int_enum! {
    pub enum RenderLineNumbersType {
        Off = 0,
        On = 1,
        Relative = 2,
        Interval = 3,
        Custom = 4,
    }
}

int_enum! {
    pub enum WrappingIndent {
        /// No indentation => wrapped lines begin at column 1.
        None = 0,
        /// Same => wrapped lines get the same indentation as the parent.
        Same = 1,
        /// Indent => wrapped lines get +1 indentation toward the parent.
        Indent = 2,
        /// DeepIndent => wrapped lines get +2 indentation toward the parent.
        Deepindent = 3,
    }
}

int_enum! {
    pub enum EditorOption {
        Acceptsuggestiononcommitcharacter = 0,
        Acceptsuggestiononenter = 1,
        Accessibilitysupport = 2,
        Accessibilitypagesize = 3,
        Arialabel = 4,
        Autoclosingbrackets = 5,
        Autoclosingovertype = 6,
        Autoclosingquotes = 7,
        Autoindent = 8,
        Automaticlayout = 9,
        Autosurround = 10,
        Codelens = 11,
        Colordecorators = 12,
        Comments = 13,
        Contextmenu = 14,
        Copywithsyntaxhighlighting = 15,
        Cursorblinking = 16,
        Cursorsmoothcaretanimation = 17,
        Cursorstyle = 18,
        Cursorsurroundinglines = 19,
        Cursorsurroundinglinesstyle = 20,
        Cursorwidth = 21,
        Disablelayerhinting = 22,
        Disablemonospaceoptimizations = 23,
        Draganddrop = 24,
        Emptyselectionclipboard = 25,
        Extraeditorclassname = 26,
        Fastscrollsensitivity = 27,
        Find = 28,
        Fixedoverflowwidgets = 29,
        Folding = 30,
        Foldingstrategy = 31,
        Foldinghighlight = 32,
        Fontfamily = 33,
        Fontinfo = 34,
        Fontligatures = 35,
        Fontsize = 36,
        Fontweight = 37,
        Formatonpaste = 38,
        Formatontype = 39,
        Glyphmargin = 40,
        Gotolocation = 41,
        Hidecursorinoverviewruler = 42,
        Highlightactiveindentguide = 43,
        Hover = 44,
        Indiffeditor = 45,
        Letterspacing = 46,
        Lightbulb = 47,
        Linedecorationswidth = 48,
        Lineheight = 49,
        Linenumbers = 50,
        Linenumbersminchars = 51,
        Links = 52,
        Matchbrackets = 53,
        Minimap = 54,
        Mousestyle = 55,
        Mousewheelscrollsensitivity = 56,
        Mousewheelzoom = 57,
        Multicursormergeoverlapping = 58,
        Multicursormodifier = 59,
        Multicursorpaste = 60,
        Occurrenceshighlight = 61,
        Overviewrulerborder = 62,
        Overviewrulerlanes = 63,
        Parameterhints = 64,
        Peekwidgetdefaultfocus = 65,
        Quicksuggestions = 66,
        Quicksuggestionsdelay = 67,
        Readonly = 68,
        Rendercontrolcharacters = 69,
        Renderindentguides = 70,
        Renderfinalnewline = 71,
        Renderlinehighlight = 72,
        Rendervalidationdecorations = 73,
        Renderwhitespace = 74,
        Revealhorizontalrightpadding = 75,
        Roundedselection = 76,
        Rulers = 77,
        Scrollbar = 78,
        Scrollbeyondlastcolumn = 79,
        Scrollbeyondlastline = 80,
        Selectionclipboard = 81,
        Selectionhighlight = 82,
        Selectonlinenumbers = 83,
        Showfoldingcontrols = 84,
        Showunused = 85,
        Snippetsuggestions = 86,
        Smoothscrolling = 87,
        Stoprenderinglineafter = 88,
        Suggest = 89,
        Suggestfontsize = 90,
        Suggestlineheight = 91,
        Suggestontriggercharacters = 92,
        Suggestselection = 93,
        Tabcompletion = 94,
        Usetabstops = 95,
        Wordseparators = 96,
        Wordwrap = 97,
        Wordwrapbreakaftercharacters = 98,
        Wordwrapbreakbeforecharacters = 99,
        Wordwrapcolumn = 100,
        Wordwrapminified = 101,
        Wrappingindent = 102,
        Wrappingstrategy = 103,
        Editorclassname = 104,
        Pixelratio = 105,
        Tabfocusmode = 106,
        Layoutinfo = 107,
        Wrappinginfo = 108,
    }
}

int_enum! {
    pub enum ContentWidgetPositionPreference {
        /// Place the content widget exactly at a position
        Exact = 0,
        /// Place the content widget above a position
        Above = 1,
        /// Place the content widget below a position
        Below = 2,
    }
}

int_enum! {
    pub enum OverlayWidgetPositionPreference {
        /// Position the overlay widget in the top right corner
        TopRightCorner = 0,
        /// Position the overlay widget in the bottom right corner
        BottomRightCorner = 1,
        /// Position the overlay widget in the top center
        TopCenter = 2,
    }
}

int_enum! {
    pub enum MouseTargetType {
        /// Mouse is on top of an unknown element.
        Unknown = 0,
        /// Mouse is on top of the textarea used for input.
        Textarea = 1,
        /// Mouse is on top of the glyph margin
        GutterGlyphMargin = 2,
        /// Mouse is on top of the line numbers
        GutterLineNumbers = 3,
        /// Mouse is on top of the line decorations
        GutterLineDecorations = 4,
        /// Mouse is on top of the whitespace left in the gutter by a view zone.
        GutterViewZone = 5,
        /// Mouse is on top of text in the content.
        ContentText = 6,
        /// Mouse is on top of empty space in the content (e.g. after line text or below last line)
        ContentEmpty = 7,
        /// Mouse is on top of a view zone in the content.
        ContentViewZone = 8,
        /// Mouse is on top of a content widget.
        ContentWidget = 9,
        /// Mouse is on top of the decorations overview ruler.
        OverviewRuler = 10,
        /// Mouse is on top of a scrollbar.
        Scrollbar = 11,
        /// Mouse is on top of an overlay widget.
        OverlayWidget = 12,
        /// Mouse is outside of the editor.
        OutsideEditor = 13,
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IDiffNavigator;
    #[wasm_bindgen(method, js_class = "IDiffNavigator", js_name = "canNavigate")]
    pub fn can_navigate(this: &IDiffNavigator) -> bool;
    #[wasm_bindgen(method, js_class = "IDiffNavigator", js_name = "next")]
    pub fn next(this: &IDiffNavigator);
    #[wasm_bindgen(method, js_class = "IDiffNavigator", js_name = "previous")]
    pub fn previous(this: &IDiffNavigator);
    #[wasm_bindgen(method, js_class = "IDiffNavigator", js_name = "dispose")]
    pub fn dispose(this: &IDiffNavigator);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IDiffNavigatorOptions;
    #[wasm_bindgen(method, js_class = "IDiffNavigatorOptions", js_name = "followsCaret", getter = followsCaret)]
    pub fn follows_caret(this: &IDiffNavigatorOptions) -> Option<bool>;
    #[wasm_bindgen(method, js_class = "IDiffNavigatorOptions", js_name = "ignoreCharChanges", getter = ignoreCharChanges)]
    pub fn ignore_char_changes(this: &IDiffNavigatorOptions) -> Option<bool>;
    #[wasm_bindgen(method, js_class = "IDiffNavigatorOptions", js_name = "alwaysRevealFirst", getter = alwaysRevealFirst)]
    pub fn always_reveal_first(this: &IDiffNavigatorOptions) -> Option<bool>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IStandaloneThemeData;
    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "base", getter = base)]
    pub fn base(this: &IStandaloneThemeData) -> BuiltinTheme;
    /// Set the `base` property.
    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "base", setter = base)]
    pub fn set_base(this: &IStandaloneThemeData, val: BuiltinTheme);
    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "inherit", getter = inherit)]
    pub fn inherit(this: &IStandaloneThemeData) -> bool;
    /// Set the `inherit` property.
    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "inherit", setter = inherit)]
    pub fn set_inherit(this: &IStandaloneThemeData, val: bool);
    /// Type: `ITokenThemeRule[]`
    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "rules", getter = rules)]
    pub fn rules(this: &IStandaloneThemeData) -> Array;
    /// Set the `rules` property.
    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "rules", setter = rules)]
    pub fn set_rules(this: &IStandaloneThemeData, val: &Array);
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "encodedTokensColors", getter = encodedTokensColors)]
    pub fn encoded_tokens_colors(this: &IStandaloneThemeData) -> Option<Array>;
    /// Set the `encodedTokensColors` property.
    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "encodedTokensColors", setter = encodedTokensColors)]
    pub fn set_encoded_tokens_colors(this: &IStandaloneThemeData, val: Option<&Array>);
    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "colors", getter = colors)]
    pub fn colors(this: &IStandaloneThemeData) -> Object;
    /// Set the `colors` property.
    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "colors", setter = colors)]
    pub fn set_colors(this: &IStandaloneThemeData, val: &Object);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ITokenThemeRule;
    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "token", getter = token)]
    pub fn token(this: &ITokenThemeRule) -> String;
    /// Set the `token` property.
    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "token", setter = token)]
    pub fn set_token(this: &ITokenThemeRule, val: &str);
    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "foreground", getter = foreground)]
    pub fn foreground(this: &ITokenThemeRule) -> Option<String>;
    /// Set the `foreground` property.
    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "foreground", setter = foreground)]
    pub fn set_foreground(this: &ITokenThemeRule, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "background", getter = background)]
    pub fn background(this: &ITokenThemeRule) -> Option<String>;
    /// Set the `background` property.
    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "background", setter = background)]
    pub fn set_background(this: &ITokenThemeRule, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "fontStyle", getter = fontStyle)]
    pub fn font_style(this: &ITokenThemeRule) -> Option<String>;
    /// Set the `fontStyle` property.
    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "fontStyle", setter = fontStyle)]
    pub fn set_font_style(this: &ITokenThemeRule, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    /// A web worker that can provide a proxy to an arbitrary file.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type MonacoWebWorker;
    /// Terminate the web worker, thus invalidating the returned proxy.
    #[wasm_bindgen(method, js_class = "MonacoWebWorker", js_name = "dispose")]
    pub fn dispose(this: &MonacoWebWorker);
    /// Get a proxy to the arbitrary loaded code.
    #[wasm_bindgen(method, js_class = "MonacoWebWorker", js_name = "getProxy")]
    pub fn get_proxy(this: &MonacoWebWorker) -> Promise;
    /// Synchronize (send) the models at `resources` to the web worker,
    /// making them available in the monaco.worker.getMirrorModels().
    ///
    /// # Arguments
    ///
    /// * `resources` - `Uri[]`
    #[wasm_bindgen(method, js_class = "MonacoWebWorker", js_name = "withSyncedResources")]
    pub fn with_synced_resources(this: &MonacoWebWorker, resources: &Array) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IWebWorkerOptions;
    /// The AMD moduleId to load.
    /// It should export a function `create` that should return the exported
    /// proxy.
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "moduleId", getter = moduleId)]
    pub fn module_id(this: &IWebWorkerOptions) -> String;
    /// Set the `moduleId` property.
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "moduleId", setter = moduleId)]
    pub fn set_module_id(this: &IWebWorkerOptions, val: &str);
    /// The data to send over when calling create on the module.
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "createData", getter = createData)]
    pub fn create_data(this: &IWebWorkerOptions) -> JsValue;
    /// Set the `createData` property.
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "createData", setter = createData)]
    pub fn set_create_data(this: &IWebWorkerOptions, val: &JsValue);
    /// A label to be used to identify the web worker for debugging purposes.
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "label", getter = label)]
    pub fn label(this: &IWebWorkerOptions) -> Option<String>;
    /// Set the `label` property.
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "label", setter = label)]
    pub fn set_label(this: &IWebWorkerOptions, val: Option<&str>);
    /// An object that can be used by the web worker to make calls back to the
    /// main thread.
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "host", getter = host)]
    pub fn host(this: &IWebWorkerOptions) -> JsValue;
    /// Set the `host` property.
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "host", setter = host)]
    pub fn set_host(this: &IWebWorkerOptions, val: &JsValue);
    /// Keep idle models.
    /// Defaults to false, which means that idle models will stop syncing after
    /// a while.
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "keepIdleModels", getter = keepIdleModels)]
    pub fn keep_idle_models(this: &IWebWorkerOptions) -> Option<bool>;
    /// Set the `keepIdleModels` property.
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "keepIdleModels", setter = keepIdleModels)]
    pub fn set_keep_idle_models(this: &IWebWorkerOptions, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    /// Description of an action contribution
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IActionDescriptor;
    /// An unique identifier of the contributed action.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "id", getter = id)]
    pub fn id(this: &IActionDescriptor) -> String;
    /// Set the `id` property.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "id", setter = id)]
    pub fn set_id(this: &IActionDescriptor, val: &str);
    /// A label of the action that will be presented to the user.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "label", getter = label)]
    pub fn label(this: &IActionDescriptor) -> String;
    /// Set the `label` property.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "label", setter = label)]
    pub fn set_label(this: &IActionDescriptor, val: &str);
    /// Precondition rule.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "precondition", getter = precondition)]
    pub fn precondition(this: &IActionDescriptor) -> Option<String>;
    /// Set the `precondition` property.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "precondition", setter = precondition)]
    pub fn set_precondition(this: &IActionDescriptor, val: Option<&str>);
    /// An array of keybindings for the action.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "keybindings", getter = keybindings)]
    pub fn keybindings(this: &IActionDescriptor) -> Option<Vec<f64>>;
    /// Set the `keybindings` property.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "keybindings", setter = keybindings)]
    pub fn set_keybindings(this: &IActionDescriptor, val: Option<&[f64]>);
    /// The keybinding rule (condition on top of precondition).
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "keybindingContext", getter = keybindingContext)]
    pub fn keybinding_context(this: &IActionDescriptor) -> Option<String>;
    /// Set the `keybindingContext` property.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "keybindingContext", setter = keybindingContext)]
    pub fn set_keybinding_context(this: &IActionDescriptor, val: Option<&str>);
    /// Control if the action should show up in the context menu and where.
    /// The context menu of the editor has these default:
    ///   navigation - The navigation group comes first in all cases.
    ///   1_modification - This group comes next and contains commands that
    /// modify your code.   9_cutcopypaste - The last default group with the
    /// basic editing commands. You can also create your own group.
    /// Defaults to null (don't show in context menu).
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "contextMenuGroupId", getter = contextMenuGroupId)]
    pub fn context_menu_group_id(this: &IActionDescriptor) -> Option<String>;
    /// Set the `contextMenuGroupId` property.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "contextMenuGroupId", setter = contextMenuGroupId)]
    pub fn set_context_menu_group_id(this: &IActionDescriptor, val: Option<&str>);
    /// Control the order in the context menu group.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "contextMenuOrder", getter = contextMenuOrder)]
    pub fn context_menu_order(this: &IActionDescriptor) -> Option<f64>;
    /// Set the `contextMenuOrder` property.
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "contextMenuOrder", setter = contextMenuOrder)]
    pub fn set_context_menu_order(this: &IActionDescriptor, val: Option<f64>);
    /// Method that will be executed when the action is triggered.
    /// @param editor The editor instance is passed in as a convenience
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "run")]
    pub fn run(
        this: &IActionDescriptor,
        editor: &ICodeEditor,
        args: Vec<JsValue>,
    ) -> Option<Promise>;
}

#[wasm_bindgen]
extern "C" {
    /// Options which apply for all editors.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IGlobalEditorOptions;
    /// The number of spaces a tab is equal to.
    /// This setting is overridden based on the file contents when
    /// `detectIndentation` is on. Defaults to 4.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "tabSize", getter = tabSize)]
    pub fn tab_size(this: &IGlobalEditorOptions) -> Option<f64>;
    /// Set the `tabSize` property.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "tabSize", setter = tabSize)]
    pub fn set_tab_size(this: &IGlobalEditorOptions, val: Option<f64>);
    /// Insert spaces when pressing `Tab`.
    /// This setting is overridden based on the file contents when
    /// `detectIndentation` is on. Defaults to true.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "insertSpaces", getter = insertSpaces)]
    pub fn insert_spaces(this: &IGlobalEditorOptions) -> Option<bool>;
    /// Set the `insertSpaces` property.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "insertSpaces", setter = insertSpaces)]
    pub fn set_insert_spaces(this: &IGlobalEditorOptions, val: Option<bool>);
    /// Controls whether `tabSize` and `insertSpaces` will be automatically
    /// detected when a file is opened based on the file contents.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "detectIndentation", getter = detectIndentation)]
    pub fn detect_indentation(this: &IGlobalEditorOptions) -> Option<bool>;
    /// Set the `detectIndentation` property.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "detectIndentation", setter = detectIndentation)]
    pub fn set_detect_indentation(this: &IGlobalEditorOptions, val: Option<bool>);
    /// Remove trailing auto inserted whitespace.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "trimAutoWhitespace", getter = trimAutoWhitespace)]
    pub fn trim_auto_whitespace(this: &IGlobalEditorOptions) -> Option<bool>;
    /// Set the `trimAutoWhitespace` property.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "trimAutoWhitespace", setter = trimAutoWhitespace)]
    pub fn set_trim_auto_whitespace(this: &IGlobalEditorOptions, val: Option<bool>);
    /// Special handling for large files to disable certain memory intensive
    /// features. Defaults to true.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "largeFileOptimizations", getter = largeFileOptimizations)]
    pub fn large_file_optimizations(this: &IGlobalEditorOptions) -> Option<bool>;
    /// Set the `largeFileOptimizations` property.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "largeFileOptimizations", setter = largeFileOptimizations)]
    pub fn set_large_file_optimizations(this: &IGlobalEditorOptions, val: Option<bool>);
    /// Controls whether completions should be computed based on words in the
    /// document. Defaults to true.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "wordBasedSuggestions", getter = wordBasedSuggestions)]
    pub fn word_based_suggestions(this: &IGlobalEditorOptions) -> Option<bool>;
    /// Set the `wordBasedSuggestions` property.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "wordBasedSuggestions", setter = wordBasedSuggestions)]
    pub fn set_word_based_suggestions(this: &IGlobalEditorOptions, val: Option<bool>);
    /// Keep peek editors open even when double clicking their content or when
    /// hitting `Escape`. Defaults to false.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "stablePeek", getter = stablePeek)]
    pub fn stable_peek(this: &IGlobalEditorOptions) -> Option<bool>;
    /// Set the `stablePeek` property.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "stablePeek", setter = stablePeek)]
    pub fn set_stable_peek(this: &IGlobalEditorOptions, val: Option<bool>);
    /// Lines above this length will not be tokenized for performance reasons.
    /// Defaults to 20000.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "maxTokenizationLineLength", getter = maxTokenizationLineLength)]
    pub fn max_tokenization_line_length(this: &IGlobalEditorOptions) -> Option<f64>;
    /// Set the `maxTokenizationLineLength` property.
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "maxTokenizationLineLength", setter = maxTokenizationLineLength)]
    pub fn set_max_tokenization_line_length(this: &IGlobalEditorOptions, val: Option<f64>);
}

#[wasm_bindgen]
extern "C" {
    /// The options to create an editor.
    #[derive(Debug, PartialEq, Clone)]
    #[wasm_bindgen(extends = IEditorConstructionOptions, extends = IGlobalEditorOptions, extends = Object)]
    pub type IStandaloneEditorConstructionOptions;
    /// The initial model associated with this code editor.
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "model", getter = model)]
    pub fn model(this: &IStandaloneEditorConstructionOptions) -> Option<ITextModel>;
    /// Set the `model` property.
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "model", setter = model)]
    pub fn set_model(this: &IStandaloneEditorConstructionOptions, val: Option<&ITextModel>);
    /// The initial value of the auto created model in the editor.
    /// To not create automatically a model, use `model: null`.
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "value", getter = value)]
    pub fn value(this: &IStandaloneEditorConstructionOptions) -> Option<String>;
    /// Set the `value` property.
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "value", setter = value)]
    pub fn set_value(this: &IStandaloneEditorConstructionOptions, val: Option<&str>);
    /// The initial language of the auto created model in the editor.
    /// To not create automatically a model, use `model: null`.
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "language", getter = language)]
    pub fn language(this: &IStandaloneEditorConstructionOptions) -> Option<String>;
    /// Set the `language` property.
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "language", setter = language)]
    pub fn set_language(this: &IStandaloneEditorConstructionOptions, val: Option<&str>);
    /// Initial theme to be used for rendering.
    /// The current out-of-the-box available themes are: 'vs' (default),
    /// 'vs-dark', 'hc-black'. You can create custom themes via
    /// `monaco.editor.defineTheme`. To switch a theme, use
    /// `monaco.editor.setTheme`
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "theme", getter = theme)]
    pub fn theme(this: &IStandaloneEditorConstructionOptions) -> Option<String>;
    /// Set the `theme` property.
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "theme", setter = theme)]
    pub fn set_theme(this: &IStandaloneEditorConstructionOptions, val: Option<&str>);
    /// An URL to open when Ctrl+H (Windows and Linux) or Cmd+H (OSX) is pressed
    /// in the accessibility help dialog in the editor.
    ///
    /// Defaults to "https://go.microsoft.com/fwlink/?linkid=852450"
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "accessibilityHelpUrl", getter = accessibilityHelpUrl)]
    pub fn accessibility_help_url(this: &IStandaloneEditorConstructionOptions) -> Option<String>;
    /// Set the `accessibilityHelpUrl` property.
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "accessibilityHelpUrl", setter = accessibilityHelpUrl)]
    pub fn set_accessibility_help_url(
        this: &IStandaloneEditorConstructionOptions,
        val: Option<&str>,
    );
}

#[wasm_bindgen]
extern "C" {
    /// The options to create a diff editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = IDiffEditorOptions, extends = Object)]
    pub type IDiffEditorConstructionOptions;
    /// Initial theme to be used for rendering.
    /// The current out-of-the-box available themes are: 'vs' (default),
    /// 'vs-dark', 'hc-black'. You can create custom themes via
    /// `monaco.editor.defineTheme`. To switch a theme, use
    /// `monaco.editor.setTheme`
    #[wasm_bindgen(method, js_class = "IDiffEditorConstructionOptions", js_name = "theme", getter = theme)]
    pub fn theme(this: &IDiffEditorConstructionOptions) -> Option<String>;
    /// Set the `theme` property.
    #[wasm_bindgen(method, js_class = "IDiffEditorConstructionOptions", js_name = "theme", setter = theme)]
    pub fn set_theme(this: &IDiffEditorConstructionOptions, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = ICodeEditor, extends = Object)]
    pub type IStandaloneCodeEditor;
    #[wasm_bindgen(method, js_class = "IStandaloneCodeEditor", js_name = "updateOptions")]
    pub fn update_options_editor(this: &IStandaloneCodeEditor, new_options: &IEditorOptions);
    #[wasm_bindgen(method, js_class = "IStandaloneCodeEditor", js_name = "updateOptions")]
    pub fn update_options_global(this: &IStandaloneCodeEditor, new_options: &IGlobalEditorOptions);
    /// # Arguments
    ///
    /// * `handler` - `(...args: any[]) => void`
    #[wasm_bindgen(method, js_class = "IStandaloneCodeEditor", js_name = "addCommand")]
    pub fn add_command(
        this: &IStandaloneCodeEditor,
        keybinding: f64,
        handler: &Function,
        context: Option<&str>,
    ) -> Option<String>;
    #[wasm_bindgen(
        method,
        js_class = "IStandaloneCodeEditor",
        js_name = "createContextKey"
    )]
    pub fn create_context_key(
        this: &IStandaloneCodeEditor,
        key: &str,
        default_value: &JsValue,
    ) -> IContextKey;
    #[wasm_bindgen(method, js_class = "IStandaloneCodeEditor", js_name = "addAction")]
    pub fn add_action(this: &IStandaloneCodeEditor, descriptor: &IActionDescriptor) -> IDisposable;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = IDiffEditor, extends = Object)]
    pub type IStandaloneDiffEditor;
    /// # Arguments
    ///
    /// * `handler` - `(...args: any[]) => void`
    #[wasm_bindgen(method, js_class = "IStandaloneDiffEditor", js_name = "addCommand")]
    pub fn add_command(
        this: &IStandaloneDiffEditor,
        keybinding: f64,
        handler: &Function,
        context: Option<&str>,
    ) -> Option<String>;
    #[wasm_bindgen(
        method,
        js_class = "IStandaloneDiffEditor",
        js_name = "createContextKey"
    )]
    pub fn create_context_key(
        this: &IStandaloneDiffEditor,
        key: &str,
        default_value: &JsValue,
    ) -> IContextKey;
    #[wasm_bindgen(method, js_class = "IStandaloneDiffEditor", js_name = "addAction")]
    pub fn add_action(this: &IStandaloneDiffEditor, descriptor: &IActionDescriptor) -> IDisposable;
    #[wasm_bindgen(
        method,
        js_class = "IStandaloneDiffEditor",
        js_name = "getOriginalEditor"
    )]
    pub fn get_original_editor(this: &IStandaloneDiffEditor) -> IStandaloneCodeEditor;
    #[wasm_bindgen(
        method,
        js_class = "IStandaloneDiffEditor",
        js_name = "getModifiedEditor"
    )]
    pub fn get_modified_editor(this: &IStandaloneDiffEditor) -> IStandaloneCodeEditor;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IContextKey;
    #[wasm_bindgen(method, js_class = "IContextKey", js_name = "set")]
    pub fn set(this: &IContextKey, value: &JsValue);
    #[wasm_bindgen(method, js_class = "IContextKey", js_name = "reset")]
    pub fn reset(this: &IContextKey);
    #[wasm_bindgen(method, js_class = "IContextKey", js_name = "get")]
    pub fn get(this: &IContextKey) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IMarker;
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "owner", getter = owner)]
    pub fn owner(this: &IMarker) -> String;
    /// Set the `owner` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "owner", setter = owner)]
    pub fn set_owner(this: &IMarker, val: &str);
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "resource", getter = resource)]
    pub fn resource(this: &IMarker) -> Uri;
    /// Set the `resource` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "resource", setter = resource)]
    pub fn set_resource(this: &IMarker, val: &Uri);
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "severity", getter = severity)]
    pub fn severity(this: &IMarker) -> MarkerSeverity;
    /// Set the `severity` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "severity", setter = severity)]
    pub fn set_severity(this: &IMarker, val: MarkerSeverity);
    /// Type: `{ value: string; link: Uri }`
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "code", getter = code)]
    pub fn code(this: &IMarker) -> Option<Object>;
    /// Set the `code` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "code", setter = code)]
    pub fn set_code(this: &IMarker, val: Option<&Object>);
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "message", getter = message)]
    pub fn message(this: &IMarker) -> String;
    /// Set the `message` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "message", setter = message)]
    pub fn set_message(this: &IMarker, val: &str);
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "source", getter = source)]
    pub fn source(this: &IMarker) -> Option<String>;
    /// Set the `source` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "source", setter = source)]
    pub fn set_source(this: &IMarker, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "startLineNumber", getter = startLineNumber)]
    pub fn start_line_number(this: &IMarker) -> f64;
    /// Set the `startLineNumber` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "startLineNumber", setter = startLineNumber)]
    pub fn set_start_line_number(this: &IMarker, val: f64);
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "startColumn", getter = startColumn)]
    pub fn start_column(this: &IMarker) -> f64;
    /// Set the `startColumn` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "startColumn", setter = startColumn)]
    pub fn set_start_column(this: &IMarker, val: f64);
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "endLineNumber", getter = endLineNumber)]
    pub fn end_line_number(this: &IMarker) -> f64;
    /// Set the `endLineNumber` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "endLineNumber", setter = endLineNumber)]
    pub fn set_end_line_number(this: &IMarker, val: f64);
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "endColumn", getter = endColumn)]
    pub fn end_column(this: &IMarker) -> f64;
    /// Set the `endColumn` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "endColumn", setter = endColumn)]
    pub fn set_end_column(this: &IMarker, val: f64);
    /// Type: `IRelatedInformation[]`
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "relatedInformation", getter = relatedInformation)]
    pub fn related_information(this: &IMarker) -> Option<Array>;
    /// Set the `relatedInformation` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "relatedInformation", setter = relatedInformation)]
    pub fn set_related_information(this: &IMarker, val: Option<&Array>);
    /// Type: `MarkerTag[]`
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "tags", getter = tags)]
    pub fn tags(this: &IMarker) -> Option<Array>;
    /// Set the `tags` property.
    #[wasm_bindgen(method, js_class = "IMarker", js_name = "tags", setter = tags)]
    pub fn set_tags(this: &IMarker, val: Option<&Array>);
}

#[wasm_bindgen]
extern "C" {
    /// A structure defining a problem/warning/etc.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IMarkerData;
    /// Type: `{ value: string; link: Uri }`
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "code", getter = code)]
    pub fn code(this: &IMarkerData) -> Option<Object>;
    /// Set the `code` property.
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "code", setter = code)]
    pub fn set_code(this: &IMarkerData, val: Option<&Object>);
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "severity", getter = severity)]
    pub fn severity(this: &IMarkerData) -> MarkerSeverity;
    /// Set the `severity` property.
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "severity", setter = severity)]
    pub fn set_severity(this: &IMarkerData, val: MarkerSeverity);
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "message", getter = message)]
    pub fn message(this: &IMarkerData) -> String;
    /// Set the `message` property.
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "message", setter = message)]
    pub fn set_message(this: &IMarkerData, val: &str);
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "source", getter = source)]
    pub fn source(this: &IMarkerData) -> Option<String>;
    /// Set the `source` property.
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "source", setter = source)]
    pub fn set_source(this: &IMarkerData, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "startLineNumber", getter = startLineNumber)]
    pub fn start_line_number(this: &IMarkerData) -> f64;
    /// Set the `startLineNumber` property.
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "startLineNumber", setter = startLineNumber)]
    pub fn set_start_line_number(this: &IMarkerData, val: f64);
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "startColumn", getter = startColumn)]
    pub fn start_column(this: &IMarkerData) -> f64;
    /// Set the `startColumn` property.
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "startColumn", setter = startColumn)]
    pub fn set_start_column(this: &IMarkerData, val: f64);
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "endLineNumber", getter = endLineNumber)]
    pub fn end_line_number(this: &IMarkerData) -> f64;
    /// Set the `endLineNumber` property.
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "endLineNumber", setter = endLineNumber)]
    pub fn set_end_line_number(this: &IMarkerData, val: f64);
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "endColumn", getter = endColumn)]
    pub fn end_column(this: &IMarkerData) -> f64;
    /// Set the `endColumn` property.
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "endColumn", setter = endColumn)]
    pub fn set_end_column(this: &IMarkerData, val: f64);
    /// Type: `IRelatedInformation[]`
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "relatedInformation", getter = relatedInformation)]
    pub fn related_information(this: &IMarkerData) -> Option<Array>;
    /// Set the `relatedInformation` property.
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "relatedInformation", setter = relatedInformation)]
    pub fn set_related_information(this: &IMarkerData, val: Option<&Array>);
    /// Type: `MarkerTag[]`
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "tags", getter = tags)]
    pub fn tags(this: &IMarkerData) -> Option<Array>;
    /// Set the `tags` property.
    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "tags", setter = tags)]
    pub fn set_tags(this: &IMarkerData, val: Option<&Array>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IRelatedInformation;
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "resource", getter = resource)]
    pub fn resource(this: &IRelatedInformation) -> Uri;
    /// Set the `resource` property.
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "resource", setter = resource)]
    pub fn set_resource(this: &IRelatedInformation, val: &Uri);
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "message", getter = message)]
    pub fn message(this: &IRelatedInformation) -> String;
    /// Set the `message` property.
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "message", setter = message)]
    pub fn set_message(this: &IRelatedInformation, val: &str);
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "startLineNumber", getter = startLineNumber)]
    pub fn start_line_number(this: &IRelatedInformation) -> f64;
    /// Set the `startLineNumber` property.
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "startLineNumber", setter = startLineNumber)]
    pub fn set_start_line_number(this: &IRelatedInformation, val: f64);
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "startColumn", getter = startColumn)]
    pub fn start_column(this: &IRelatedInformation) -> f64;
    /// Set the `startColumn` property.
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "startColumn", setter = startColumn)]
    pub fn set_start_column(this: &IRelatedInformation, val: f64);
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "endLineNumber", getter = endLineNumber)]
    pub fn end_line_number(this: &IRelatedInformation) -> f64;
    /// Set the `endLineNumber` property.
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "endLineNumber", setter = endLineNumber)]
    pub fn set_end_line_number(this: &IRelatedInformation, val: f64);
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "endColumn", getter = endColumn)]
    pub fn end_column(this: &IRelatedInformation) -> f64;
    /// Set the `endColumn` property.
    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "endColumn", setter = endColumn)]
    pub fn set_end_column(this: &IRelatedInformation, val: f64);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IColorizerOptions;
    #[wasm_bindgen(method, js_class = "IColorizerOptions", js_name = "tabSize", getter = tabSize)]
    pub fn tab_size(this: &IColorizerOptions) -> Option<f64>;
    /// Set the `tabSize` property.
    #[wasm_bindgen(method, js_class = "IColorizerOptions", js_name = "tabSize", setter = tabSize)]
    pub fn set_tab_size(this: &IColorizerOptions, val: Option<f64>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = IColorizerOptions, extends = Object)]
    pub type IColorizerElementOptions;
    #[wasm_bindgen(method, js_class = "IColorizerElementOptions", js_name = "theme", getter = theme)]
    pub fn theme(this: &IColorizerElementOptions) -> Option<String>;
    /// Set the `theme` property.
    #[wasm_bindgen(method, js_class = "IColorizerElementOptions", js_name = "theme", setter = theme)]
    pub fn set_theme(this: &IColorizerElementOptions, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "IColorizerElementOptions", js_name = "mimeType", getter = mimeType)]
    pub fn mime_type(this: &IColorizerElementOptions) -> Option<String>;
    /// Set the `mimeType` property.
    #[wasm_bindgen(method, js_class = "IColorizerElementOptions", js_name = "mimeType", setter = mimeType)]
    pub fn set_mime_type(this: &IColorizerElementOptions, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ThemeColor;
    #[wasm_bindgen(method, js_class = "ThemeColor", js_name = "id", getter = id)]
    pub fn id(this: &ThemeColor) -> String;
    /// Set the `id` property.
    #[wasm_bindgen(method, js_class = "ThemeColor", js_name = "id", setter = id)]
    pub fn set_id(this: &ThemeColor, val: &str);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IDecorationOptions;
    /// CSS color to render.
    /// e.g.: rgba(100, 100, 100, 0.5) or a color from the color registry
    #[wasm_bindgen(method, js_class = "IDecorationOptions", js_name = "color", getter = color)]
    pub fn color(this: &IDecorationOptions) -> JsValue;
    /// Set the `color` property.
    #[wasm_bindgen(method, js_class = "IDecorationOptions", js_name = "color", setter = color)]
    pub fn set_color(this: &IDecorationOptions, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "IDecorationOptions", js_name = "color", setter = color)]
    pub fn set_color_theme(this: &IDecorationOptions, val: Option<&ThemeColor>);
    /// CSS color to render.
    /// e.g.: rgba(100, 100, 100, 0.5) or a color from the color registry
    #[wasm_bindgen(method, js_class = "IDecorationOptions", js_name = "darkColor", getter = darkColor)]
    pub fn dark_color(this: &IDecorationOptions) -> JsValue;
    /// Set the `darkColor` property.
    #[wasm_bindgen(method, js_class = "IDecorationOptions", js_name = "darkColor", setter = darkColor)]
    pub fn set_dark_color(this: &IDecorationOptions, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "IDecorationOptions", js_name = "darkColor", setter = darkColor)]
    pub fn set_dark_color_theme(this: &IDecorationOptions, val: Option<&ThemeColor>);
}

#[wasm_bindgen]
extern "C" {
    /// Options for rendering a model decoration in the overview ruler.
    #[derive(Debug)]
    #[wasm_bindgen(extends = IDecorationOptions, extends = Object)]
    pub type IModelDecorationOverviewRulerOptions;
    /// The position in the overview ruler.
    #[wasm_bindgen(method, js_class = "IModelDecorationOverviewRulerOptions", js_name = "position", getter = position)]
    pub fn position(this: &IModelDecorationOverviewRulerOptions) -> OverviewRulerLane;
    /// Set the `position` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOverviewRulerOptions", js_name = "position", setter = position)]
    pub fn set_position(this: &IModelDecorationOverviewRulerOptions, val: OverviewRulerLane);
}

#[wasm_bindgen]
extern "C" {
    /// Options for rendering a model decoration in the overview ruler.
    #[derive(Debug)]
    #[wasm_bindgen(extends = IDecorationOptions, extends = Object)]
    pub type IModelDecorationMinimapOptions;
    /// The position in the overview ruler.
    #[wasm_bindgen(method, js_class = "IModelDecorationMinimapOptions", js_name = "position", getter = position)]
    pub fn position(this: &IModelDecorationMinimapOptions) -> MinimapPosition;
    /// Set the `position` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationMinimapOptions", js_name = "position", setter = position)]
    pub fn set_position(this: &IModelDecorationMinimapOptions, val: MinimapPosition);
}

#[wasm_bindgen]
extern "C" {
    /// Options for a model decoration.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IModelDecorationOptions;
    /// Customize the growing behavior of the decoration when typing at the
    /// edges of the decoration. Defaults to
    /// TrackedRangeStickiness.AlwaysGrowsWhenTypingAtEdges
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "stickiness", getter = stickiness)]
    pub fn stickiness(this: &IModelDecorationOptions) -> Option<TrackedRangeStickiness>;
    /// Set the `stickiness` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "stickiness", setter = stickiness)]
    pub fn set_stickiness(this: &IModelDecorationOptions, val: Option<TrackedRangeStickiness>);
    /// CSS class name describing the decoration.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "className", getter = className)]
    pub fn class_name(this: &IModelDecorationOptions) -> Option<String>;
    /// Set the `className` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "className", setter = className)]
    pub fn set_class_name(this: &IModelDecorationOptions, val: Option<&str>);
    /// Message to be rendered when hovering over the glyph margin decoration.
    ///
    /// Type: `IMarkdownString | IMarkdownString[]`
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "glyphMarginHoverMessage", getter = glyphMarginHoverMessage)]
    pub fn glyph_margin_hover_message(this: &IModelDecorationOptions) -> JsValue;
    /// Set the `glyphMarginHoverMessage` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "glyphMarginHoverMessage", setter = glyphMarginHoverMessage)]
    pub fn set_glyph_margin_hover_message(this: &IModelDecorationOptions, val: &JsValue);
    /// Array of MarkdownString to render as the decoration message.
    ///
    /// Type: `IMarkdownString | IMarkdownString[]`
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "hoverMessage", getter = hoverMessage)]
    pub fn hover_message(this: &IModelDecorationOptions) -> JsValue;
    /// Set the `hoverMessage` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "hoverMessage", setter = hoverMessage)]
    pub fn set_hover_message(this: &IModelDecorationOptions, val: &JsValue);
    /// Should the decoration expand to encompass a whole line.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "isWholeLine", getter = isWholeLine)]
    pub fn is_whole_line(this: &IModelDecorationOptions) -> Option<bool>;
    /// Set the `isWholeLine` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "isWholeLine", setter = isWholeLine)]
    pub fn set_is_whole_line(this: &IModelDecorationOptions, val: Option<bool>);
    /// Specifies the stack order of a decoration.
    /// A decoration with greater stack order is always in front of a decoration
    /// with a lower stack order.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "zIndex", getter = zIndex)]
    pub fn z_index(this: &IModelDecorationOptions) -> Option<f64>;
    /// Set the `zIndex` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "zIndex", setter = zIndex)]
    pub fn set_z_index(this: &IModelDecorationOptions, val: Option<f64>);
    /// If set, render this decoration in the overview ruler.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "overviewRuler", getter = overviewRuler)]
    pub fn overview_ruler(
        this: &IModelDecorationOptions,
    ) -> Option<IModelDecorationOverviewRulerOptions>;
    /// Set the `overviewRuler` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "overviewRuler", setter = overviewRuler)]
    pub fn set_overview_ruler(
        this: &IModelDecorationOptions,
        val: Option<&IModelDecorationOverviewRulerOptions>,
    );
    /// If set, render this decoration in the minimap.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "minimap", getter = minimap)]
    pub fn minimap(this: &IModelDecorationOptions) -> Option<IModelDecorationMinimapOptions>;
    /// Set the `minimap` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "minimap", setter = minimap)]
    pub fn set_minimap(
        this: &IModelDecorationOptions,
        val: Option<&IModelDecorationMinimapOptions>,
    );
    /// If set, the decoration will be rendered in the glyph margin with this
    /// CSS class name.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "glyphMarginClassName", getter = glyphMarginClassName)]
    pub fn glyph_margin_class_name(this: &IModelDecorationOptions) -> Option<String>;
    /// Set the `glyphMarginClassName` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "glyphMarginClassName", setter = glyphMarginClassName)]
    pub fn set_glyph_margin_class_name(this: &IModelDecorationOptions, val: Option<&str>);
    /// If set, the decoration will be rendered in the lines decorations with
    /// this CSS class name.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "linesDecorationsClassName", getter = linesDecorationsClassName)]
    pub fn lines_decorations_class_name(this: &IModelDecorationOptions) -> Option<String>;
    /// Set the `linesDecorationsClassName` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "linesDecorationsClassName", setter = linesDecorationsClassName)]
    pub fn set_lines_decorations_class_name(this: &IModelDecorationOptions, val: Option<&str>);
    /// If set, the decoration will be rendered in the margin (covering its full
    /// width) with this CSS class name.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "marginClassName", getter = marginClassName)]
    pub fn margin_class_name(this: &IModelDecorationOptions) -> Option<String>;
    /// Set the `marginClassName` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "marginClassName", setter = marginClassName)]
    pub fn set_margin_class_name(this: &IModelDecorationOptions, val: Option<&str>);
    /// If set, the decoration will be rendered inline with the text with this
    /// CSS class name. Please use this only for CSS rules that must impact
    /// the text. For example, use `className` to have a background color
    /// decoration.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "inlineClassName", getter = inlineClassName)]
    pub fn inline_class_name(this: &IModelDecorationOptions) -> Option<String>;
    /// Set the `inlineClassName` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "inlineClassName", setter = inlineClassName)]
    pub fn set_inline_class_name(this: &IModelDecorationOptions, val: Option<&str>);
    /// If there is an `inlineClassName` which affects letter spacing.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "inlineClassNameAffectsLetterSpacing", getter = inlineClassNameAffectsLetterSpacing)]
    pub fn inline_class_name_affects_letter_spacing(this: &IModelDecorationOptions)
        -> Option<bool>;
    /// Set the `inlineClassNameAffectsLetterSpacing` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "inlineClassNameAffectsLetterSpacing", setter = inlineClassNameAffectsLetterSpacing)]
    pub fn set_inline_class_name_affects_letter_spacing(
        this: &IModelDecorationOptions,
        val: Option<bool>,
    );
    /// If set, the decoration will be rendered before the text with this CSS
    /// class name.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "beforeContentClassName", getter = beforeContentClassName)]
    pub fn before_content_class_name(this: &IModelDecorationOptions) -> Option<String>;
    /// Set the `beforeContentClassName` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "beforeContentClassName", setter = beforeContentClassName)]
    pub fn set_before_content_class_name(this: &IModelDecorationOptions, val: Option<&str>);
    /// If set, the decoration will be rendered after the text with this CSS
    /// class name.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "afterContentClassName", getter = afterContentClassName)]
    pub fn after_content_class_name(this: &IModelDecorationOptions) -> Option<String>;
    /// Set the `afterContentClassName` property.
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "afterContentClassName", setter = afterContentClassName)]
    pub fn set_after_content_class_name(this: &IModelDecorationOptions, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    /// New model decorations.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IModelDeltaDecoration;
    /// Range that this decoration covers.
    #[wasm_bindgen(method, js_class = "IModelDeltaDecoration", js_name = "range", getter = range)]
    pub fn range(this: &IModelDeltaDecoration) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "IModelDeltaDecoration", js_name = "range", setter = range)]
    pub fn set_range(this: &IModelDeltaDecoration, val: &IRange);
    /// Options associated with this decoration.
    #[wasm_bindgen(method, js_class = "IModelDeltaDecoration", js_name = "options", getter = options)]
    pub fn options(this: &IModelDeltaDecoration) -> IModelDecorationOptions;
    /// Set the `options` property.
    #[wasm_bindgen(method, js_class = "IModelDeltaDecoration", js_name = "options", setter = options)]
    pub fn set_options(this: &IModelDeltaDecoration, val: &IModelDecorationOptions);
}

#[wasm_bindgen]
extern "C" {
    /// A decoration in the model.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IModelDecoration;
    /// Identifier for a decoration.
    #[wasm_bindgen(method, js_class = "IModelDecoration", js_name = "id", getter = id)]
    pub fn id(this: &IModelDecoration) -> String;
    /// Identifier for a decoration's owner.
    #[wasm_bindgen(method, js_class = "IModelDecoration", js_name = "ownerId", getter = ownerId)]
    pub fn owner_id(this: &IModelDecoration) -> f64;
    /// Range that this decoration covers.
    #[wasm_bindgen(method, js_class = "IModelDecoration", js_name = "range", getter = range)]
    pub fn range(this: &IModelDecoration) -> Range;
    /// Options associated with this decoration.
    #[wasm_bindgen(method, js_class = "IModelDecoration", js_name = "options", getter = options)]
    pub fn options(this: &IModelDecoration) -> IModelDecorationOptions;
}

#[wasm_bindgen]
extern "C" {
    /// Word inside a model.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IWordAtPosition;
    /// The word.
    #[wasm_bindgen(method, js_class = "IWordAtPosition", js_name = "word", getter = word)]
    pub fn word(this: &IWordAtPosition) -> String;
    /// The column where the word starts.
    #[wasm_bindgen(method, js_class = "IWordAtPosition", js_name = "startColumn", getter = startColumn)]
    pub fn start_column(this: &IWordAtPosition) -> f64;
    /// The column where the word ends.
    #[wasm_bindgen(method, js_class = "IWordAtPosition", js_name = "endColumn", getter = endColumn)]
    pub fn end_column(this: &IWordAtPosition) -> f64;
}

#[wasm_bindgen]
extern "C" {
    /// A single edit operation, that acts as a simple replace.
    /// i.e. Replace text at `range` with `text` in model.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ISingleEditOperation;
    /// The range to replace. This can be empty to emulate a simple insert.
    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "range", getter = range)]
    pub fn range(this: &ISingleEditOperation) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "range", setter = range)]
    pub fn set_range(this: &ISingleEditOperation, val: &IRange);
    /// The text to replace with. This can be null to emulate a simple delete.
    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "text", getter = text)]
    pub fn text(this: &ISingleEditOperation) -> Option<String>;
    /// Set the `text` property.
    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "text", setter = text)]
    pub fn set_text(this: &ISingleEditOperation, val: Option<&str>);
    /// This indicates that this operation has "insert" semantics.
    /// i.e. forceMoveMarkers = true => if `range` is collapsed, all markers at
    /// the position will be moved.
    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "forceMoveMarkers", getter = forceMoveMarkers)]
    pub fn force_move_markers(this: &ISingleEditOperation) -> Option<bool>;
    /// Set the `forceMoveMarkers` property.
    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "forceMoveMarkers", setter = forceMoveMarkers)]
    pub fn set_force_move_markers(this: &ISingleEditOperation, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    /// A single edit operation, that has an identifier.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IIdentifiedSingleEditOperation;
    /// The range to replace. This can be empty to emulate a simple insert.
    #[wasm_bindgen(method, js_class = "IIdentifiedSingleEditOperation", js_name = "range", getter = range)]
    pub fn range(this: &IIdentifiedSingleEditOperation) -> Range;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "IIdentifiedSingleEditOperation", js_name = "range", setter = range)]
    pub fn set_range(this: &IIdentifiedSingleEditOperation, val: &Range);
    /// The text to replace with. This can be null to emulate a simple delete.
    #[wasm_bindgen(method, js_class = "IIdentifiedSingleEditOperation", js_name = "text", getter = text)]
    pub fn text(this: &IIdentifiedSingleEditOperation) -> Option<String>;
    /// Set the `text` property.
    #[wasm_bindgen(method, js_class = "IIdentifiedSingleEditOperation", js_name = "text", setter = text)]
    pub fn set_text(this: &IIdentifiedSingleEditOperation, val: Option<&str>);
    /// This indicates that this operation has "insert" semantics.
    /// i.e. forceMoveMarkers = true => if `range` is collapsed, all markers at
    /// the position will be moved.
    #[wasm_bindgen(method, js_class = "IIdentifiedSingleEditOperation", js_name = "forceMoveMarkers", getter = forceMoveMarkers)]
    pub fn force_move_markers(this: &IIdentifiedSingleEditOperation) -> Option<bool>;
    /// Set the `forceMoveMarkers` property.
    #[wasm_bindgen(method, js_class = "IIdentifiedSingleEditOperation", js_name = "forceMoveMarkers", setter = forceMoveMarkers)]
    pub fn set_force_move_markers(this: &IIdentifiedSingleEditOperation, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ITextModelUpdateOptions;
    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "tabSize", getter = tabSize)]
    pub fn tab_size(this: &ITextModelUpdateOptions) -> Option<f64>;
    /// Set the `tabSize` property.
    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "tabSize", setter = tabSize)]
    pub fn set_tab_size(this: &ITextModelUpdateOptions, val: Option<f64>);
    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "indentSize", getter = indentSize)]
    pub fn indent_size(this: &ITextModelUpdateOptions) -> Option<f64>;
    /// Set the `indentSize` property.
    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "indentSize", setter = indentSize)]
    pub fn set_indent_size(this: &ITextModelUpdateOptions, val: Option<f64>);
    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "insertSpaces", getter = insertSpaces)]
    pub fn insert_spaces(this: &ITextModelUpdateOptions) -> Option<bool>;
    /// Set the `insertSpaces` property.
    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "insertSpaces", setter = insertSpaces)]
    pub fn set_insert_spaces(this: &ITextModelUpdateOptions, val: Option<bool>);
    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "trimAutoWhitespace", getter = trimAutoWhitespace)]
    pub fn trim_auto_whitespace(this: &ITextModelUpdateOptions) -> Option<bool>;
    /// Set the `trimAutoWhitespace` property.
    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "trimAutoWhitespace", setter = trimAutoWhitespace)]
    pub fn set_trim_auto_whitespace(this: &ITextModelUpdateOptions, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    /// A model.
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = Object)]
    pub type ITextModel;
    /// Gets the resource associated with this editor model.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "uri", getter = uri)]
    pub fn uri(this: &ITextModel) -> Uri;
    /// A unique identifier associated with this model.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "id", getter = id)]
    pub fn id(this: &ITextModel) -> String;
    /// Get the resolved options for this model.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getOptions")]
    pub fn get_options(this: &ITextModel) -> TextModelResolvedOptions;
    /// Get the current version id of the model.
    /// Anytime a change happens to the model (even undo/redo),
    /// the version id is incremented.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getVersionId")]
    pub fn get_version_id(this: &ITextModel) -> f64;
    /// Get the alternative version id of the model.
    /// This alternative version id is not always incremented,
    /// it will return the same values in the case of undo-redo.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getAlternativeVersionId")]
    pub fn get_alternative_version_id(this: &ITextModel) -> f64;
    /// Replace the entire text buffer value contained in this model.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "setValue")]
    pub fn set_value(this: &ITextModel, new_value: &str);
    /// Get the text stored in this model.
    /// @param eol The end of line character preference. Defaults to
    /// `EndOfLinePreference.TextDefined`. @param preserverBOM Preserve a
    /// BOM character if it was detected when the model was constructed.
    /// @return The text.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getValue")]
    pub fn get_value(
        this: &ITextModel,
        eol: Option<EndOfLinePreference>,
        preserve_bom: Option<bool>,
    ) -> String;
    /// Get the length of the text stored in this model.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getValueLength")]
    pub fn get_value_length(
        this: &ITextModel,
        eol: Option<EndOfLinePreference>,
        preserve_bom: Option<bool>,
    ) -> f64;
    /// Get the text in a certain range.
    /// @param range The range describing what text to get.
    /// @param eol The end of line character preference. This will only be used
    /// for multiline ranges. Defaults to `EndOfLinePreference.TextDefined`.
    /// @return The text.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getValueInRange")]
    pub fn get_value_in_range(
        this: &ITextModel,
        range: &IRange,
        eol: Option<EndOfLinePreference>,
    ) -> String;
    /// Get the length of text in a certain range.
    /// @param range The range describing what text length to get.
    /// @return The text length.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getValueLengthInRange")]
    pub fn get_value_length_in_range(this: &ITextModel, range: &IRange) -> f64;
    /// Get the character count of text in a certain range.
    /// @param range The range describing what text length to get.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getCharacterCountInRange")]
    pub fn get_character_count_in_range(this: &ITextModel, range: &IRange) -> f64;
    /// Get the number of lines in the model.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineCount")]
    pub fn get_line_count(this: &ITextModel) -> f64;
    /// Get the text for a certain line.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineContent")]
    pub fn get_line_content(this: &ITextModel, line_number: f64) -> String;
    /// Get the text length for a certain line.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineLength")]
    pub fn get_line_length(this: &ITextModel, line_number: f64) -> f64;
    /// Get the text for all lines.
    ///
    /// # Returns
    ///
    /// `string[]`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLinesContent")]
    pub fn get_lines_content(this: &ITextModel) -> Array;
    /// Get the end of line sequence predominantly used in the text buffer.
    /// @return EOL char sequence (e.g.: '\n' or '\r\n').
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getEOL")]
    pub fn get_eol(this: &ITextModel) -> String;
    /// Get the minimum legal column for line at `lineNumber`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineMinColumn")]
    pub fn get_line_min_column(this: &ITextModel, line_number: f64) -> f64;
    /// Get the maximum legal column for line at `lineNumber`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineMaxColumn")]
    pub fn get_line_max_column(this: &ITextModel, line_number: f64) -> f64;
    /// Returns the column before the first non whitespace character for line at
    /// `lineNumber`. Returns 0 if line is empty or contains only
    /// whitespace.
    #[wasm_bindgen(
        method,
        js_class = "ITextModel",
        js_name = "getLineFirstNonWhitespaceColumn"
    )]
    pub fn get_line_first_non_whitespace_column(this: &ITextModel, line_number: f64) -> f64;
    /// Returns the column after the last non whitespace character for line at
    /// `lineNumber`. Returns 0 if line is empty or contains only
    /// whitespace.
    #[wasm_bindgen(
        method,
        js_class = "ITextModel",
        js_name = "getLineLastNonWhitespaceColumn"
    )]
    pub fn get_line_last_non_whitespace_column(this: &ITextModel, line_number: f64) -> f64;
    /// Create a valid position,
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "validatePosition")]
    pub fn validate_position(this: &ITextModel, position: &IPosition) -> Position;
    /// Advances the given position by the given offset (negative offsets are
    /// also accepted) and returns it as a new valid position.
    ///
    /// If the offset and position are such that their combination goes beyond
    /// the beginning or end of the model, throws an exception.
    ///
    /// If the offset is such that the new position would be in the middle of a
    /// multi-byte line terminator, throws an exception.
    #[wasm_bindgen(catch, method, js_class = "ITextModel", js_name = "modifyPosition")]
    pub fn modify_position(
        this: &ITextModel,
        position: &IPosition,
        offset: f64,
    ) -> Result<Position, JsValue>;
    /// Create a valid range.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "validateRange")]
    pub fn validate_range(this: &ITextModel, range: &IRange) -> Range;
    /// Converts the position to a zero-based offset.
    ///
    /// The position will be [adjusted](#TextDocument.validatePosition).
    ///
    /// @param position A position.
    /// @return A valid zero-based offset.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getOffsetAt")]
    pub fn get_offset_at(this: &ITextModel, position: &IPosition) -> f64;
    /// Converts a zero-based offset to a position.
    ///
    /// @param offset A zero-based offset.
    /// @return A valid [position](#Position).
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getPositionAt")]
    pub fn get_position_at(this: &ITextModel, offset: f64) -> Position;
    /// Get a range covering the entire model
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getFullModelRange")]
    pub fn get_full_model_range(this: &ITextModel) -> Range;
    /// Returns if the model was disposed or not.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "isDisposed")]
    pub fn is_disposed(this: &ITextModel) -> bool;
    /// Search the model.
    /// @param searchString The string used to search. If it is a regular
    /// expression, set `isRegex` to true. @param searchOnlyEditableRange
    /// Limit the searching to only search inside the editable range of the
    /// model. @param isRegex Used to indicate that `searchString` is a
    /// regular expression. @param matchCase Force the matching to match
    /// lower/upper case exactly. @param wordSeparators Force the matching
    /// to match entire words only. Pass null otherwise.
    /// @param captureMatches The result will contain the captured groups.
    /// @param limitResultCount Limit the number of results
    /// @return The ranges where the matches are. It is empty if not matches
    /// have been found.
    ///
    /// # Returns
    ///
    /// `FindMatch[]`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "findMatches")]
    pub fn find_matches(
        this: &ITextModel,
        search_string: &str,
        search_only_editable_range: bool,
        is_regex: bool,
        match_case: bool,
        word_separators: Option<&str>,
        capture_matches: bool,
        limit_result_count: Option<f64>,
    ) -> Array;
    /// Search the model.
    /// @param searchString The string used to search. If it is a regular
    /// expression, set `isRegex` to true. @param searchScope Limit the
    /// searching to only search inside this range. @param isRegex Used to
    /// indicate that `searchString` is a regular expression.
    /// @param matchCase Force the matching to match lower/upper case exactly.
    /// @param wordSeparators Force the matching to match entire words only.
    /// Pass null otherwise. @param captureMatches The result will contain
    /// the captured groups. @param limitResultCount Limit the number of
    /// results @return The ranges where the matches are. It is empty if no
    /// matches have been found.
    ///
    /// # Returns
    ///
    /// `FindMatch[]`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "findMatches")]
    pub fn find_matches_range(
        this: &ITextModel,
        search_string: &str,
        search_scope: &IRange,
        is_regex: bool,
        match_case: bool,
        word_separators: Option<&str>,
        capture_matches: bool,
        limit_result_count: Option<f64>,
    ) -> Array;
    /// Search the model for the next match. Loops to the beginning of the model
    /// if needed. @param searchString The string used to search. If it is a
    /// regular expression, set `isRegex` to true. @param searchStart Start
    /// the searching at the specified position. @param isRegex Used to
    /// indicate that `searchString` is a regular expression.
    /// @param matchCase Force the matching to match lower/upper case exactly.
    /// @param wordSeparators Force the matching to match entire words only.
    /// Pass null otherwise. @param captureMatches The result will contain
    /// the captured groups. @return The range where the next match is. It
    /// is null if no next match has been found.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "findNextMatch")]
    pub fn find_next_match(
        this: &ITextModel,
        search_string: &str,
        search_start: &IPosition,
        is_regex: bool,
        match_case: bool,
        word_separators: Option<&str>,
        capture_matches: bool,
    ) -> Option<FindMatch>;
    /// Search the model for the previous match. Loops to the end of the model
    /// if needed. @param searchString The string used to search. If it is a
    /// regular expression, set `isRegex` to true. @param searchStart Start
    /// the searching at the specified position. @param isRegex Used to
    /// indicate that `searchString` is a regular expression.
    /// @param matchCase Force the matching to match lower/upper case exactly.
    /// @param wordSeparators Force the matching to match entire words only.
    /// Pass null otherwise. @param captureMatches The result will contain
    /// the captured groups. @return The range where the previous match is.
    /// It is null if no previous match has been found.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "findPreviousMatch")]
    pub fn find_previous_match(
        this: &ITextModel,
        search_string: &str,
        search_start: &IPosition,
        is_regex: bool,
        match_case: bool,
        word_separators: Option<&str>,
        capture_matches: bool,
    ) -> Option<FindMatch>;
    /// Get the language associated with this model.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getModeId")]
    pub fn get_mode_id(this: &ITextModel) -> String;
    /// Get the word under or besides `position`.
    /// @param position The position to look for a word.
    /// @return The word under or besides `position`. Might be null.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getWordAtPosition")]
    pub fn get_word_at_position(this: &ITextModel, position: &IPosition)
        -> Option<IWordAtPosition>;
    /// Get the word under or besides `position` trimmed to `position`.column
    /// @param position The position to look for a word.
    /// @return The word under or besides `position`. Will never be null.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getWordUntilPosition")]
    pub fn get_word_until_position(this: &ITextModel, position: &IPosition) -> IWordAtPosition;
    /// Perform a minimum amount of operations, in order to transform the
    /// decorations identified by `oldDecorations` to the decorations
    /// described by `newDecorations` and returns the new identifiers
    /// associated with the resulting decorations.
    ///
    /// @param oldDecorations Array containing previous decorations identifiers.
    /// @param newDecorations Array describing what decorations should result
    /// after the call. @param ownerId Identifies the editor id in which
    /// these decorations should appear. If no `ownerId` is provided, the
    /// decorations will appear in all editors that attach this model.
    /// @return An array containing the new decorations identifiers.
    ///
    /// # Arguments
    ///
    /// * `old_decorations` - `string[]`
    /// * `new_decorations` - `IModelDeltaDecoration[]`
    ///
    /// # Returns
    ///
    /// `string[]`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "deltaDecorations")]
    pub fn delta_decorations(
        this: &ITextModel,
        old_decorations: &Array,
        new_decorations: &Array,
        owner_id: Option<f64>,
    ) -> Array;
    /// Get the options associated with a decoration.
    /// @param id The decoration id.
    /// @return The decoration options or null if the decoration was not found.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getDecorationOptions")]
    pub fn get_decoration_options(this: &ITextModel, id: &str) -> Option<IModelDecorationOptions>;
    /// Get the range associated with a decoration.
    /// @param id The decoration id.
    /// @return The decoration range or null if the decoration was not found.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getDecorationRange")]
    pub fn get_decoration_range(this: &ITextModel, id: &str) -> Option<Range>;
    /// Gets all the decorations for the line `lineNumber` as an array.
    /// @param lineNumber The line number
    /// @param ownerId If set, it will ignore decorations belonging to other
    /// owners. @param filterOutValidation If set, it will ignore
    /// decorations specific to validation (i.e. warnings, errors).
    /// @return An array with the decorations
    ///
    /// # Returns
    ///
    /// `IModelDecoration[]`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineDecorations")]
    pub fn get_line_decorations(
        this: &ITextModel,
        line_number: f64,
        owner_id: Option<f64>,
        filter_out_validation: Option<bool>,
    ) -> Array;
    /// Gets all the decorations for the lines between `startLineNumber` and
    /// `endLineNumber` as an array. @param startLineNumber The start line
    /// number @param endLineNumber The end line number
    /// @param ownerId If set, it will ignore decorations belonging to other
    /// owners. @param filterOutValidation If set, it will ignore
    /// decorations specific to validation (i.e. warnings, errors).
    /// @return An array with the decorations
    ///
    /// # Returns
    ///
    /// `IModelDecoration[]`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLinesDecorations")]
    pub fn get_lines_decorations(
        this: &ITextModel,
        start_line_number: f64,
        end_line_number: f64,
        owner_id: Option<f64>,
        filter_out_validation: Option<bool>,
    ) -> Array;
    /// Gets all the decorations in a range as an array. Only `startLineNumber`
    /// and `endLineNumber` from `range` are used for filtering. So for now
    /// it returns all the decorations on the same line as `range`.
    /// @param range The range to search in
    /// @param ownerId If set, it will ignore decorations belonging to other
    /// owners. @param filterOutValidation If set, it will ignore
    /// decorations specific to validation (i.e. warnings, errors).
    /// @return An array with the decorations
    ///
    /// # Returns
    ///
    /// `IModelDecoration[]`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getDecorationsInRange")]
    pub fn get_decorations_in_range(
        this: &ITextModel,
        range: &IRange,
        owner_id: Option<f64>,
        filter_out_validation: Option<bool>,
    ) -> Array;
    /// Gets all the decorations as an array.
    /// @param ownerId If set, it will ignore decorations belonging to other
    /// owners. @param filterOutValidation If set, it will ignore
    /// decorations specific to validation (i.e. warnings, errors).
    ///
    /// # Returns
    ///
    /// `IModelDecoration[]`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getAllDecorations")]
    pub fn get_all_decorations(
        this: &ITextModel,
        owner_id: Option<f64>,
        filter_out_validation: Option<bool>,
    ) -> Array;
    /// Gets all the decorations that should be rendered in the overview ruler
    /// as an array. @param ownerId If set, it will ignore decorations
    /// belonging to other owners. @param filterOutValidation If set, it
    /// will ignore decorations specific to validation (i.e. warnings, errors).
    ///
    /// # Returns
    ///
    /// `IModelDecoration[]`
    #[wasm_bindgen(
        method,
        js_class = "ITextModel",
        js_name = "getOverviewRulerDecorations"
    )]
    pub fn get_overview_ruler_decorations(
        this: &ITextModel,
        owner_id: Option<f64>,
        filter_out_validation: Option<bool>,
    ) -> Array;
    /// Normalize a string containing whitespace according to indentation rules
    /// (converts to spaces or to tabs).
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "normalizeIndentation")]
    pub fn normalize_indentation(this: &ITextModel, str: &str) -> String;
    /// Change the options of this model.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "updateOptions")]
    pub fn update_options(this: &ITextModel, new_opts: &ITextModelUpdateOptions);
    /// Detect the indentation options for this model from its content.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "detectIndentation")]
    pub fn detect_indentation(
        this: &ITextModel,
        default_insert_spaces: bool,
        default_tab_size: f64,
    );
    /// Push a stack element onto the undo stack. This acts as an undo/redo
    /// point. The idea is to use `pushEditOperations` to edit the model and
    /// then to `pushStackElement` to create an undo/redo stop point.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "pushStackElement")]
    pub fn push_stack_element(this: &ITextModel);
    /// Push edit operations, basically editing the model. This is the preferred
    /// way of editing the model. The edit operations will land on the undo
    /// stack. @param beforeCursorState The cursor state before the edit
    /// operations. This cursor state will be returned when `undo` or `redo` are
    /// invoked. @param editOperations The edit operations.
    /// @param cursorStateComputer A callback that can compute the resulting
    /// cursors state after the edit operations have been executed.
    /// @return The cursor state returned by the `cursorStateComputer`.
    ///
    /// # Arguments
    ///
    /// * `before_cursor_state` - `Selection[]`
    /// * `edit_operations` - `IIdentifiedSingleEditOperation[]`
    /// * `cursor_state_computer` - `( inverseEditOperations:
    ///   IIdentifiedSingleEditOperation[] ) => Selection[]`
    ///
    /// # Returns
    ///
    /// `Selection[]`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "pushEditOperations")]
    pub fn push_edit_operations(
        this: &ITextModel,
        before_cursor_state: &Array,
        edit_operations: &Array,
        cursor_state_computer: Option<&Array>,
    ) -> Option<Array>;
    /// Change the end of line sequence. This is the preferred way of
    /// changing the eol sequence. This will land on the undo stack.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "pushEOL")]
    pub fn push_eol(this: &ITextModel, eol: EndOfLineSequence);
    /// Edit the model without adding the edits to the undo stack.
    /// This can have dire consequences on the undo stack! See
    /// @pushEditOperations for the preferred way. @param operations The
    /// edit operations. @return The inverse edit operations, that, when
    /// applied, will bring the model back to the previous state.
    ///
    /// # Arguments
    ///
    /// * `operations` - `IIdentifiedSingleEditOperation[]`
    ///
    /// # Returns
    ///
    /// `IIdentifiedSingleEditOperation[]`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "applyEdits")]
    pub fn apply_edits(this: &ITextModel, operations: &Array) -> Array;
    /// Change the end of line sequence without recording in the undo stack.
    /// This can have dire consequences on the undo stack! See @pushEOL for the
    /// preferred way.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "setEOL")]
    pub fn set_eol(this: &ITextModel, eol: EndOfLineSequence);
    /// An event emitted when the contents of the model have changed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IModelContentChangedEvent) => void`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onDidChangeContent")]
    pub fn on_did_change_content(this: &ITextModel, listener: &Function) -> IDisposable;
    /// An event emitted when decorations of the model have changed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: {}) => void`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onDidChangeDecorations")]
    pub fn on_did_change_decorations(this: &ITextModel, listener: &Function) -> IDisposable;
    /// An event emitted when the model options have changed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IModelOptionsChangedEvent) => void`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onDidChangeOptions")]
    pub fn on_did_change_options(this: &ITextModel, listener: &Function) -> IDisposable;
    /// An event emitted when the language associated with the model has
    /// changed. @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IModelLanguageChangedEvent) => void`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onDidChangeLanguage")]
    pub fn on_did_change_language(this: &ITextModel, listener: &Function) -> IDisposable;
    /// An event emitted when the language configuration associated with the
    /// model has changed. @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: {}) => void`
    #[wasm_bindgen(
        method,
        js_class = "ITextModel",
        js_name = "onDidChangeLanguageConfiguration"
    )]
    pub fn on_did_change_language_configuration(
        this: &ITextModel,
        listener: &Function,
    ) -> IDisposable;
    /// An event emitted right before disposing the model.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `() => void`
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onWillDispose")]
    pub fn on_will_dispose(this: &ITextModel, listener: &Function) -> IDisposable;
    /// Destroy this model. This will unbind the model from the mode
    /// and make all necessary clean-up to release this object to the GC.
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "dispose")]
    pub fn dispose(this: &ITextModel);
}

#[wasm_bindgen]
extern "C" {
    /// A builder and helper for edit operations for a command.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditOperationBuilder;
    /// Add a new edit operation (a replace operation).
    /// @param range The range to replace (delete). May be empty to represent a
    /// simple insert. @param text The text to replace with. May be null to
    /// represent a simple delete.
    #[wasm_bindgen(
        method,
        js_class = "IEditOperationBuilder",
        js_name = "addEditOperation"
    )]
    pub fn add_edit_operation(
        this: &IEditOperationBuilder,
        range: &Range,
        text: Option<&str>,
        force_move_markers: Option<bool>,
    );
    /// Add a new edit operation (a replace operation).
    /// The inverse edits will be accessible in
    /// `ICursorStateComputerData.getInverseEditOperations()` @param range
    /// The range to replace (delete). May be empty to represent a simple
    /// insert. @param text The text to replace with. May be null to
    /// represent a simple delete.
    #[wasm_bindgen(
        method,
        js_class = "IEditOperationBuilder",
        js_name = "addTrackedEditOperation"
    )]
    pub fn add_tracked_edit_operation(
        this: &IEditOperationBuilder,
        range: &Range,
        text: Option<&str>,
        force_move_markers: Option<bool>,
    );
    /// Track `selection` when applying edit operations.
    /// A best effort will be made to not grow/expand the selection.
    /// An empty selection will clamp to a nearby character.
    /// @param selection The selection to track.
    /// @param trackPreviousOnEmpty If set, and the selection is empty,
    /// indicates whether the selection           should clamp to the
    /// previous or the next character. @return A unique identifier.
    #[wasm_bindgen(method, js_class = "IEditOperationBuilder", js_name = "trackSelection")]
    pub fn track_selection(
        this: &IEditOperationBuilder,
        selection: &Selection,
        track_previous_on_empty: Option<bool>,
    ) -> String;
}

#[wasm_bindgen]
extern "C" {
    /// A helper for computing cursor state after a command.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ICursorStateComputerData;
    /// Get the inverse edit operations of the added edit operations.
    ///
    /// # Returns
    ///
    /// `IIdentifiedSingleEditOperation[]`
    #[wasm_bindgen(
        method,
        js_class = "ICursorStateComputerData",
        js_name = "getInverseEditOperations"
    )]
    pub fn get_inverse_edit_operations(this: &ICursorStateComputerData) -> Array;
    /// Get a previously tracked selection.
    /// @param id The unique identifier returned by `trackSelection`.
    /// @return The selection.
    #[wasm_bindgen(
        method,
        js_class = "ICursorStateComputerData",
        js_name = "getTrackedSelection"
    )]
    pub fn get_tracked_selection(this: &ICursorStateComputerData, id: &str) -> Selection;
}

#[wasm_bindgen]
extern "C" {
    /// A command that modifies text / cursor state on a model.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ICommand;
    /// Get the edit operations needed to execute this command.
    /// @param model The model the command will execute on.
    /// @param builder A helper to collect the needed edit operations and to
    /// track selections.
    #[wasm_bindgen(method, js_class = "ICommand", js_name = "getEditOperations")]
    pub fn get_edit_operations(
        this: &ICommand,
        model: &ITextModel,
        builder: &IEditOperationBuilder,
    );
    /// Compute the cursor state after the edit operations were applied.
    /// @param model The model the command has executed on.
    /// @param helper A helper to get inverse edit operations and to get
    /// previously tracked selections. @return The cursor state after the
    /// command executed.
    #[wasm_bindgen(method, js_class = "ICommand", js_name = "computeCursorState")]
    pub fn compute_cursor_state(
        this: &ICommand,
        model: &ITextModel,
        helper: &ICursorStateComputerData,
    ) -> Selection;
}

#[wasm_bindgen]
extern "C" {
    /// A model for the diff editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IDiffEditorModel;
    /// Original model.
    #[wasm_bindgen(method, js_class = "IDiffEditorModel", js_name = "original", getter = original)]
    pub fn original(this: &IDiffEditorModel) -> ITextModel;
    /// Set the `original` property.
    #[wasm_bindgen(method, js_class = "IDiffEditorModel", js_name = "original", setter = original)]
    pub fn set_original(this: &IDiffEditorModel, val: &ITextModel);
    /// Modified model.
    #[wasm_bindgen(method, js_class = "IDiffEditorModel", js_name = "modified", getter = modified)]
    pub fn modified(this: &IDiffEditorModel) -> ITextModel;
    /// Set the `modified` property.
    #[wasm_bindgen(method, js_class = "IDiffEditorModel", js_name = "modified", setter = modified)]
    pub fn set_modified(this: &IDiffEditorModel, val: &ITextModel);
}

#[wasm_bindgen]
extern "C" {
    /// An event describing that an editor has had its model reset (i.e.
    /// `editor.setModel()`).
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IModelChangedEvent;
    /// The `uri` of the previous model or null.
    #[wasm_bindgen(method, js_class = "IModelChangedEvent", js_name = "oldModelUrl", getter = oldModelUrl)]
    pub fn old_model_url(this: &IModelChangedEvent) -> Option<Uri>;
    /// The `uri` of the new model or null.
    #[wasm_bindgen(method, js_class = "IModelChangedEvent", js_name = "newModelUrl", getter = newModelUrl)]
    pub fn new_model_url(this: &IModelChangedEvent) -> Option<Uri>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = Object)]
    pub type IDimension;
    #[wasm_bindgen(method, js_class = "IDimension", js_name = "width", getter = width)]
    pub fn width(this: &IDimension) -> f64;
    /// Set the `width` property.
    #[wasm_bindgen(method, js_class = "IDimension", js_name = "width", setter = width)]
    pub fn set_width(this: &IDimension, val: f64);
    #[wasm_bindgen(method, js_class = "IDimension", js_name = "height", getter = height)]
    pub fn height(this: &IDimension) -> f64;
    /// Set the `height` property.
    #[wasm_bindgen(method, js_class = "IDimension", js_name = "height", setter = height)]
    pub fn set_height(this: &IDimension, val: f64);
}

#[wasm_bindgen]
extern "C" {
    /// A change
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IChange;
    #[wasm_bindgen(method, js_class = "IChange", js_name = "originalStartLineNumber", getter = originalStartLineNumber)]
    pub fn original_start_line_number(this: &IChange) -> f64;
    #[wasm_bindgen(method, js_class = "IChange", js_name = "originalEndLineNumber", getter = originalEndLineNumber)]
    pub fn original_end_line_number(this: &IChange) -> f64;
    #[wasm_bindgen(method, js_class = "IChange", js_name = "modifiedStartLineNumber", getter = modifiedStartLineNumber)]
    pub fn modified_start_line_number(this: &IChange) -> f64;
    #[wasm_bindgen(method, js_class = "IChange", js_name = "modifiedEndLineNumber", getter = modifiedEndLineNumber)]
    pub fn modified_end_line_number(this: &IChange) -> f64;
}

#[wasm_bindgen]
extern "C" {
    /// A character level change.
    #[derive(Debug)]
    #[wasm_bindgen(extends = IChange, extends = Object)]
    pub type ICharChange;
    #[wasm_bindgen(method, js_class = "ICharChange", js_name = "originalStartColumn", getter = originalStartColumn)]
    pub fn original_start_column(this: &ICharChange) -> f64;
    #[wasm_bindgen(method, js_class = "ICharChange", js_name = "originalEndColumn", getter = originalEndColumn)]
    pub fn original_end_column(this: &ICharChange) -> f64;
    #[wasm_bindgen(method, js_class = "ICharChange", js_name = "modifiedStartColumn", getter = modifiedStartColumn)]
    pub fn modified_start_column(this: &ICharChange) -> f64;
    #[wasm_bindgen(method, js_class = "ICharChange", js_name = "modifiedEndColumn", getter = modifiedEndColumn)]
    pub fn modified_end_column(this: &ICharChange) -> f64;
}

#[wasm_bindgen]
extern "C" {
    /// A line change
    #[derive(Debug)]
    #[wasm_bindgen(extends = IChange, extends = Object)]
    pub type ILineChange;
    /// Type: `ICharChange[]`
    #[wasm_bindgen(method, js_class = "ILineChange", js_name = "charChanges", getter = charChanges)]
    pub fn char_changes(this: &ILineChange) -> Option<Array>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IContentSizeChangedEvent;
    #[wasm_bindgen(method, js_class = "IContentSizeChangedEvent", js_name = "contentWidth", getter = contentWidth)]
    pub fn content_width(this: &IContentSizeChangedEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IContentSizeChangedEvent", js_name = "contentHeight", getter = contentHeight)]
    pub fn content_height(this: &IContentSizeChangedEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IContentSizeChangedEvent", js_name = "contentWidthChanged", getter = contentWidthChanged)]
    pub fn content_width_changed(this: &IContentSizeChangedEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IContentSizeChangedEvent", js_name = "contentHeightChanged", getter = contentHeightChanged)]
    pub fn content_height_changed(this: &IContentSizeChangedEvent) -> bool;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type INewScrollPosition;
    #[wasm_bindgen(method, js_class = "INewScrollPosition", js_name = "scrollLeft", getter = scrollLeft)]
    pub fn scroll_left(this: &INewScrollPosition) -> Option<f64>;
    /// Set the `scrollLeft` property.
    #[wasm_bindgen(method, js_class = "INewScrollPosition", js_name = "scrollLeft", setter = scrollLeft)]
    pub fn set_scroll_left(this: &INewScrollPosition, val: Option<f64>);
    #[wasm_bindgen(method, js_class = "INewScrollPosition", js_name = "scrollTop", getter = scrollTop)]
    pub fn scroll_top(this: &INewScrollPosition) -> Option<f64>;
    /// Set the `scrollTop` property.
    #[wasm_bindgen(method, js_class = "INewScrollPosition", js_name = "scrollTop", setter = scrollTop)]
    pub fn set_scroll_top(this: &INewScrollPosition, val: Option<f64>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorAction;
    #[wasm_bindgen(method, js_class = "IEditorAction", js_name = "id", getter = id)]
    pub fn id(this: &IEditorAction) -> String;
    #[wasm_bindgen(method, js_class = "IEditorAction", js_name = "label", getter = label)]
    pub fn label(this: &IEditorAction) -> String;
    #[wasm_bindgen(method, js_class = "IEditorAction", js_name = "alias", getter = alias)]
    pub fn alias(this: &IEditorAction) -> String;
    #[wasm_bindgen(method, js_class = "IEditorAction", js_name = "isSupported")]
    pub fn is_supported(this: &IEditorAction) -> bool;
    #[wasm_bindgen(method, js_class = "IEditorAction", js_name = "run")]
    pub fn run(this: &IEditorAction) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    /// A (serializable) state of the cursors.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ICursorState;
    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "inSelectionMode", getter = inSelectionMode)]
    pub fn in_selection_mode(this: &ICursorState) -> bool;
    /// Set the `inSelectionMode` property.
    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "inSelectionMode", setter = inSelectionMode)]
    pub fn set_in_selection_mode(this: &ICursorState, val: bool);
    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "selectionStart", getter = selectionStart)]
    pub fn selection_start(this: &ICursorState) -> IPosition;
    /// Set the `selectionStart` property.
    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "selectionStart", setter = selectionStart)]
    pub fn set_selection_start(this: &ICursorState, val: &IPosition);
    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "position", getter = position)]
    pub fn position(this: &ICursorState) -> IPosition;
    /// Set the `position` property.
    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "position", setter = position)]
    pub fn set_position(this: &ICursorState, val: &IPosition);
}

#[wasm_bindgen]
extern "C" {
    /// A (serializable) state of the view.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IViewState;
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollTop", getter = scrollTop)]
    pub fn scroll_top(this: &IViewState) -> Option<f64>;
    /// Set the `scrollTop` property.
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollTop", setter = scrollTop)]
    pub fn set_scroll_top(this: &IViewState, val: Option<f64>);
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollTopWithoutViewZones", getter = scrollTopWithoutViewZones)]
    pub fn scroll_top_without_view_zones(this: &IViewState) -> Option<f64>;
    /// Set the `scrollTopWithoutViewZones` property.
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollTopWithoutViewZones", setter = scrollTopWithoutViewZones)]
    pub fn set_scroll_top_without_view_zones(this: &IViewState, val: Option<f64>);
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollLeft", getter = scrollLeft)]
    pub fn scroll_left(this: &IViewState) -> f64;
    /// Set the `scrollLeft` property.
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollLeft", setter = scrollLeft)]
    pub fn set_scroll_left(this: &IViewState, val: f64);
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "firstPosition", getter = firstPosition)]
    pub fn first_position(this: &IViewState) -> IPosition;
    /// Set the `firstPosition` property.
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "firstPosition", setter = firstPosition)]
    pub fn set_first_position(this: &IViewState, val: &IPosition);
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "firstPositionDeltaTop", getter = firstPositionDeltaTop)]
    pub fn first_position_delta_top(this: &IViewState) -> f64;
    /// Set the `firstPositionDeltaTop` property.
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "firstPositionDeltaTop", setter = firstPositionDeltaTop)]
    pub fn set_first_position_delta_top(this: &IViewState, val: f64);
}

#[wasm_bindgen]
extern "C" {
    /// A (serializable) state of the code editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ICodeEditorViewState;
    /// Type: `ICursorState[]`
    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "cursorState", getter = cursorState)]
    pub fn cursor_state(this: &ICodeEditorViewState) -> Array;
    /// Set the `cursorState` property.
    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "cursorState", setter = cursorState)]
    pub fn set_cursor_state(this: &ICodeEditorViewState, val: &Array);
    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "viewState", getter = viewState)]
    pub fn view_state(this: &ICodeEditorViewState) -> IViewState;
    /// Set the `viewState` property.
    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "viewState", setter = viewState)]
    pub fn set_view_state(this: &ICodeEditorViewState, val: &IViewState);
    /// Type: `{ [id: string]: any }`
    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "contributionsState", getter = contributionsState)]
    pub fn contributions_state(this: &ICodeEditorViewState) -> Object;
    /// Set the `contributionsState` property.
    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "contributionsState", setter = contributionsState)]
    pub fn set_contributions_state(this: &ICodeEditorViewState, val: &Object);
}

#[wasm_bindgen]
extern "C" {
    /// (Serializable) View state for the diff editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IDiffEditorViewState;
    #[wasm_bindgen(method, js_class = "IDiffEditorViewState", js_name = "original", getter = original)]
    pub fn original(this: &IDiffEditorViewState) -> Option<ICodeEditorViewState>;
    /// Set the `original` property.
    #[wasm_bindgen(method, js_class = "IDiffEditorViewState", js_name = "original", setter = original)]
    pub fn set_original(this: &IDiffEditorViewState, val: Option<&ICodeEditorViewState>);
    #[wasm_bindgen(method, js_class = "IDiffEditorViewState", js_name = "modified", getter = modified)]
    pub fn modified(this: &IDiffEditorViewState) -> Option<ICodeEditorViewState>;
    /// Set the `modified` property.
    #[wasm_bindgen(method, js_class = "IDiffEditorViewState", js_name = "modified", setter = modified)]
    pub fn set_modified(this: &IDiffEditorViewState, val: Option<&ICodeEditorViewState>);
}

#[wasm_bindgen]
extern "C" {
    /// An editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditor;
    /// An event emitted when the editor has been disposed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `() => void`
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "onDidDispose")]
    pub fn on_did_dispose(this: &IEditor, listener: &Function) -> IDisposable;
    /// Dispose the editor.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "dispose")]
    pub fn dispose(this: &IEditor);
    /// Get a unique id for this editor instance.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getId")]
    pub fn get_id(this: &IEditor) -> String;
    /// Get the editor type. Please see `EditorType`.
    /// This is to avoid an instanceof check
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getEditorType")]
    pub fn get_editor_type(this: &IEditor) -> String;
    /// Update the editor's options after the editor has been created.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "updateOptions")]
    pub fn update_options(this: &IEditor, new_options: &IEditorOptions);
    /// Instructs the editor to remeasure its container. This method should
    /// be called when the container of the editor gets resized.
    ///
    /// If a dimension is passed in, the passed in value will be used.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "layout")]
    pub fn layout(this: &IEditor, dimension: Option<&IDimension>);
    /// Brings browser focus to the editor text
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "focus")]
    pub fn focus(this: &IEditor);
    /// Returns true if the text inside this editor is focused (i.e. cursor is
    /// blinking).
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "hasTextFocus")]
    pub fn has_text_focus(this: &IEditor) -> bool;
    /// Returns all actions associated with this editor.
    ///
    /// # Returns
    ///
    /// `IEditorAction[]`
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getSupportedActions")]
    pub fn get_supported_actions(this: &IEditor) -> Array;
    /// Saves current view state of the editor in a serializable object.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "saveViewState")]
    pub fn save_view_state(this: &IEditor) -> JsValue;
    /// Restores the view state of the editor from a serializable object
    /// generated by `saveViewState`.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "restoreViewState")]
    pub fn restore_view_state(this: &IEditor, state: &JsValue);
    /// Given a position, returns a column number that takes tab-widths into
    /// account.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getVisibleColumnFromPosition")]
    pub fn get_visible_column_from_position(this: &IEditor, position: &IPosition) -> f64;
    /// Returns the primary position of the cursor.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getPosition")]
    pub fn get_position(this: &IEditor) -> Option<Position>;
    /// Set the primary position of the cursor. This will remove any secondary
    /// cursors. @param position New primary cursor's position
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setPosition")]
    pub fn set_position(this: &IEditor, position: &IPosition);
    /// Scroll vertically as necessary and reveal a line.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLine")]
    pub fn reveal_line(this: &IEditor, line_number: f64, scroll_type: Option<ScrollType>);
    /// Scroll vertically as necessary and reveal a line centered vertically.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLineInCenter")]
    pub fn reveal_line_in_center(this: &IEditor, line_number: f64, scroll_type: Option<ScrollType>);
    /// Scroll vertically as necessary and reveal a line centered vertically
    /// only if it lies outside the viewport.
    #[wasm_bindgen(
        method,
        js_class = "IEditor",
        js_name = "revealLineInCenterIfOutsideViewport"
    )]
    pub fn reveal_line_in_center_if_outside_viewport(
        this: &IEditor,
        line_number: f64,
        scroll_type: Option<ScrollType>,
    );
    /// Scroll vertically or horizontally as necessary and reveal a position.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealPosition")]
    pub fn reveal_position(this: &IEditor, position: &IPosition, scroll_type: Option<ScrollType>);
    /// Scroll vertically or horizontally as necessary and reveal a position
    /// centered vertically.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealPositionInCenter")]
    pub fn reveal_position_in_center(
        this: &IEditor,
        position: &IPosition,
        scroll_type: Option<ScrollType>,
    );
    /// Scroll vertically or horizontally as necessary and reveal a position
    /// centered vertically only if it lies outside the viewport.
    #[wasm_bindgen(
        method,
        js_class = "IEditor",
        js_name = "revealPositionInCenterIfOutsideViewport"
    )]
    pub fn reveal_position_in_center_if_outside_viewport(
        this: &IEditor,
        position: &IPosition,
        scroll_type: Option<ScrollType>,
    );
    /// Returns the primary selection of the editor.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getSelection")]
    pub fn get_selection(this: &IEditor) -> Option<Selection>;
    /// Returns all the selections of the editor.
    ///
    /// # Returns
    ///
    /// `Selection[]`
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getSelections")]
    pub fn get_selections(this: &IEditor) -> Option<Array>;
    /// Set the primary selection of the editor. This will remove any secondary
    /// cursors. @param selection The new selection
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setSelection")]
    pub fn set_selection_range(this: &IEditor, selection: &IRange);
    /// Set the primary selection of the editor. This will remove any secondary
    /// cursors. @param selection The new selection
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setSelection")]
    pub fn set_selection(this: &IEditor, selection: &ISelection);
    /// Set the selections for all the cursors of the editor.
    /// Cursors will be removed or added, as necessary.
    ///
    /// # Arguments
    ///
    /// * `selections` - `readonly ISelection[]`
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setSelections")]
    pub fn set_selections(this: &IEditor, selections: &Array);
    /// Scroll vertically as necessary and reveal lines.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLines")]
    pub fn reveal_lines(
        this: &IEditor,
        start_line_number: f64,
        end_line_number: f64,
        scroll_type: Option<ScrollType>,
    );
    /// Scroll vertically as necessary and reveal lines centered vertically.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLinesInCenter")]
    pub fn reveal_lines_in_center(
        this: &IEditor,
        line_number: f64,
        end_line_number: f64,
        scroll_type: Option<ScrollType>,
    );
    /// Scroll vertically as necessary and reveal lines centered vertically only
    /// if it lies outside the viewport.
    #[wasm_bindgen(
        method,
        js_class = "IEditor",
        js_name = "revealLinesInCenterIfOutsideViewport"
    )]
    pub fn reveal_lines_in_center_if_outside_viewport(
        this: &IEditor,
        line_number: f64,
        end_line_number: f64,
        scroll_type: Option<ScrollType>,
    );
    /// Scroll vertically or horizontally as necessary and reveal a range.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealRange")]
    pub fn reveal_range(this: &IEditor, range: &IRange, scroll_type: Option<ScrollType>);
    /// Scroll vertically or horizontally as necessary and reveal a range
    /// centered vertically.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealRangeInCenter")]
    pub fn reveal_range_in_center(this: &IEditor, range: &IRange, scroll_type: Option<ScrollType>);
    /// Scroll vertically or horizontally as necessary and reveal a range at the
    /// top of the viewport.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealRangeAtTop")]
    pub fn reveal_range_at_top(this: &IEditor, range: &IRange, scroll_type: Option<ScrollType>);
    /// Scroll vertically or horizontally as necessary and reveal a range
    /// centered vertically only if it lies outside the viewport.
    #[wasm_bindgen(
        method,
        js_class = "IEditor",
        js_name = "revealRangeInCenterIfOutsideViewport"
    )]
    pub fn reveal_range_in_center_if_outside_viewport(
        this: &IEditor,
        range: &IRange,
        scroll_type: Option<ScrollType>,
    );
    /// Directly trigger a handler or an editor action.
    /// @param source The source of the call.
    /// @param handlerId The id of the handler or the id of a contribution.
    /// @param payload Extra data to be sent to the handler.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "trigger")]
    pub fn trigger(this: &IEditor, source: &str, handler_id: &str, payload: &JsValue);
    /// Gets the current model attached to this editor.
    /// Type: `ITextModel | IDiffEditorModel`
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getModel")]
    pub fn get_model(this: &IEditor) -> Option<Object>;
    /// Sets the current model attached to this editor.
    /// If the previous model was created by the editor via the value key in the
    /// options literal object, it will be destroyed. Otherwise, if the
    /// previous model was set via setModel, or the model key in the options
    /// literal object, the previous model will not be destroyed.
    /// It is safe to call setModel(null) to simply detach the current model
    /// from the editor.
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setModel")]
    pub fn set_model(this: &IEditor, model: Option<&Object>);
}

#[wasm_bindgen]
extern "C" {
    /// An editor contribution that gets created every time a new editor gets
    /// created and gets disposed when the editor gets disposed.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorContribution;
    /// Dispose this contribution.
    #[wasm_bindgen(method, js_class = "IEditorContribution", js_name = "dispose")]
    pub fn dispose(this: &IEditorContribution);
    /// Store view state.
    ///
    /// Type: `() => any`
    #[wasm_bindgen(method, js_class = "IEditorContribution", js_name = "saveViewState", getter = saveViewState)]
    pub fn save_view_state(this: &IEditorContribution) -> Option<Function>;
    /// Set the `saveViewState` property.
    #[wasm_bindgen(method, js_class = "IEditorContribution", js_name = "saveViewState", setter = saveViewState)]
    pub fn set_save_view_state(this: &IEditorContribution, val: Option<&Function>);
    /// Restore view state.
    ///
    /// Type: `(state: any) => void`
    #[wasm_bindgen(method, js_class = "IEditorContribution", js_name = "restoreViewState", getter = restoreViewState)]
    pub fn restore_view_state(this: &IEditorContribution) -> Option<Function>;
    /// Set the `restoreViewState` property.
    #[wasm_bindgen(method, js_class = "IEditorContribution", js_name = "restoreViewState", setter = restoreViewState)]
    pub fn set_restore_view_state(this: &IEditorContribution, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    /// An event describing that the current mode associated with a model has
    /// changed.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IModelLanguageChangedEvent;
    /// Previous language
    #[wasm_bindgen(method, js_class = "IModelLanguageChangedEvent", js_name = "oldLanguage", getter = oldLanguage)]
    pub fn old_language(this: &IModelLanguageChangedEvent) -> String;
    /// New language
    #[wasm_bindgen(method, js_class = "IModelLanguageChangedEvent", js_name = "newLanguage", getter = newLanguage)]
    pub fn new_language(this: &IModelLanguageChangedEvent) -> String;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IModelContentChange;
    /// The range that got replaced.
    #[wasm_bindgen(method, js_class = "IModelContentChange", js_name = "range", getter = range)]
    pub fn range(this: &IModelContentChange) -> IRange;
    /// The offset of the range that got replaced.
    #[wasm_bindgen(method, js_class = "IModelContentChange", js_name = "rangeOffset", getter = rangeOffset)]
    pub fn range_offset(this: &IModelContentChange) -> f64;
    /// The length of the range that got replaced.
    #[wasm_bindgen(method, js_class = "IModelContentChange", js_name = "rangeLength", getter = rangeLength)]
    pub fn range_length(this: &IModelContentChange) -> f64;
    /// The new text for the range.
    #[wasm_bindgen(method, js_class = "IModelContentChange", js_name = "text", getter = text)]
    pub fn text(this: &IModelContentChange) -> String;
}

#[wasm_bindgen]
extern "C" {
    /// An event describing a change in the text of a model.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IModelContentChangedEvent;
    /// Type: `IModelContentChange[]`
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "changes", getter = changes)]
    pub fn changes(this: &IModelContentChangedEvent) -> Array;
    /// The (new) end-of-line character.
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "eol", getter = eol)]
    pub fn eol(this: &IModelContentChangedEvent) -> String;
    /// The new version id the model has transitioned to.
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "versionId", getter = versionId)]
    pub fn version_id(this: &IModelContentChangedEvent) -> f64;
    /// Flag that indicates that this event was generated while undoing.
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "isUndoing", getter = isUndoing)]
    pub fn is_undoing(this: &IModelContentChangedEvent) -> bool;
    /// Flag that indicates that this event was generated while redoing.
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "isRedoing", getter = isRedoing)]
    pub fn is_redoing(this: &IModelContentChangedEvent) -> bool;
    /// Flag that indicates that all decorations were lost with this edit.
    /// The model has been reset to a new value.
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "isFlush", getter = isFlush)]
    pub fn is_flush(this: &IModelContentChangedEvent) -> bool;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IModelOptionsChangedEvent;
    #[wasm_bindgen(method, js_class = "IModelOptionsChangedEvent", js_name = "tabSize", getter = tabSize)]
    pub fn tab_size(this: &IModelOptionsChangedEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IModelOptionsChangedEvent", js_name = "indentSize", getter = indentSize)]
    pub fn indent_size(this: &IModelOptionsChangedEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IModelOptionsChangedEvent", js_name = "insertSpaces", getter = insertSpaces)]
    pub fn insert_spaces(this: &IModelOptionsChangedEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IModelOptionsChangedEvent", js_name = "trimAutoWhitespace", getter = trimAutoWhitespace)]
    pub fn trim_auto_whitespace(this: &IModelOptionsChangedEvent) -> bool;
}

#[wasm_bindgen]
extern "C" {
    /// An event describing that the cursor position has changed.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ICursorPositionChangedEvent;
    /// Primary cursor's position.
    #[wasm_bindgen(method, js_class = "ICursorPositionChangedEvent", js_name = "position", getter = position)]
    pub fn position(this: &ICursorPositionChangedEvent) -> Position;
    /// Secondary cursors' position.
    ///
    /// Type: `Position[]`
    #[wasm_bindgen(method, js_class = "ICursorPositionChangedEvent", js_name = "secondaryPositions", getter = secondaryPositions)]
    pub fn secondary_positions(this: &ICursorPositionChangedEvent) -> Array;
    /// Reason.
    #[wasm_bindgen(method, js_class = "ICursorPositionChangedEvent", js_name = "reason", getter = reason)]
    pub fn reason(this: &ICursorPositionChangedEvent) -> CursorChangeReason;
    /// Source of the call that caused the event.
    #[wasm_bindgen(method, js_class = "ICursorPositionChangedEvent", js_name = "source", getter = source)]
    pub fn source(this: &ICursorPositionChangedEvent) -> String;
}

#[wasm_bindgen]
extern "C" {
    /// An event describing that the cursor selection has changed.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ICursorSelectionChangedEvent;
    /// The primary selection.
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "selection", getter = selection)]
    pub fn selection(this: &ICursorSelectionChangedEvent) -> Selection;
    /// The secondary selections.
    ///
    /// Type: `Selection[]`
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "secondarySelections", getter = secondarySelections)]
    pub fn secondary_selections(this: &ICursorSelectionChangedEvent) -> Array;
    /// The model version id.
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "modelVersionId", getter = modelVersionId)]
    pub fn model_version_id(this: &ICursorSelectionChangedEvent) -> f64;
    /// The old selections.
    ///
    /// Type: `Selection[]`
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "oldSelections", getter = oldSelections)]
    pub fn old_selections(this: &ICursorSelectionChangedEvent) -> Option<Array>;
    /// The model version id the that `oldSelections` refer to.
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "oldModelVersionId", getter = oldModelVersionId)]
    pub fn old_model_version_id(this: &ICursorSelectionChangedEvent) -> f64;
    /// Source of the call that caused the event.
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "source", getter = source)]
    pub fn source(this: &ICursorSelectionChangedEvent) -> String;
    /// Reason.
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "reason", getter = reason)]
    pub fn reason(this: &ICursorSelectionChangedEvent) -> CursorChangeReason;
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for the editor.
    #[derive(Debug, PartialEq, Clone)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorOptions;
    /// This editor is used inside a diff editor.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "inDiffEditor", getter = inDiffEditor)]
    pub fn in_diff_editor(this: &IEditorOptions) -> Option<bool>;
    /// Set the `inDiffEditor` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "inDiffEditor", setter = inDiffEditor)]
    pub fn set_in_diff_editor(this: &IEditorOptions, val: Option<bool>);
    /// The aria label for the editor's textarea (when it is focused).
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "ariaLabel", getter = ariaLabel)]
    pub fn aria_label(this: &IEditorOptions) -> Option<String>;
    /// Set the `ariaLabel` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "ariaLabel", setter = ariaLabel)]
    pub fn set_aria_label(this: &IEditorOptions, val: Option<&str>);
    /// Render vertical lines at the specified columns.
    /// Defaults to empty array.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "rulers", getter = rulers)]
    pub fn rulers(this: &IEditorOptions) -> Option<Vec<f64>>;
    /// Set the `rulers` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "rulers", setter = rulers)]
    pub fn set_rulers(this: &IEditorOptions, val: Option<&[f64]>);
    /// A string containing the word separators used when doing word navigation.
    /// Defaults to `~!@#$%^&*()-=+[{]}\\|;:\'",.<>/?
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordSeparators", getter = wordSeparators)]
    pub fn word_separators(this: &IEditorOptions) -> Option<String>;
    /// Set the `wordSeparators` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordSeparators", setter = wordSeparators)]
    pub fn set_word_separators(this: &IEditorOptions, val: Option<&str>);
    /// Enable Linux primary clipboard.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectionClipboard", getter = selectionClipboard)]
    pub fn selection_clipboard(this: &IEditorOptions) -> Option<bool>;
    /// Set the `selectionClipboard` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectionClipboard", setter = selectionClipboard)]
    pub fn set_selection_clipboard(this: &IEditorOptions, val: Option<bool>);
    /// Control the rendering of line numbers.
    /// If it is a function, it will be invoked when rendering a line number and
    /// the return value will be rendered. Otherwise, if it is a truey, line
    /// numbers will be rendered normally (equivalent of using an identity
    /// function). Otherwise, line numbers will not be rendered.
    /// Defaults to `on`.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineNumbers", getter = lineNumbers)]
    pub fn line_numbers(this: &IEditorOptions) -> Option<LineNumbersType>;
    /// Set the `lineNumbers` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineNumbers", setter = lineNumbers)]
    pub fn set_line_numbers(this: &IEditorOptions, val: Option<LineNumbersType>);
    /// Controls the minimal number of visible leading and trailing lines
    /// surrounding the cursor. Defaults to 0.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSurroundingLines", getter = cursorSurroundingLines)]
    pub fn cursor_surrounding_lines(this: &IEditorOptions) -> Option<f64>;
    /// Set the `cursorSurroundingLines` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSurroundingLines", setter = cursorSurroundingLines)]
    pub fn set_cursor_surrounding_lines(this: &IEditorOptions, val: Option<f64>);
    /// Controls when `cursorSurroundingLines` should be enforced
    /// Defaults to `default`, `cursorSurroundingLines` is not enforced when
    /// cursor position is changed by mouse.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSurroundingLinesStyle", getter = cursorSurroundingLinesStyle)]
    pub fn cursor_surrounding_lines_style(
        this: &IEditorOptions,
    ) -> Option<IEditorOptionsCursorSurroundingLinesStyle>;
    /// Set the `cursorSurroundingLinesStyle` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSurroundingLinesStyle", setter = cursorSurroundingLinesStyle)]
    pub fn set_cursor_surrounding_lines_style(
        this: &IEditorOptions,
        val: Option<IEditorOptionsCursorSurroundingLinesStyle>,
    );
    /// Render last line number when the file ends with a newline.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderFinalNewline", getter = renderFinalNewline)]
    pub fn render_final_newline(this: &IEditorOptions) -> Option<bool>;
    /// Set the `renderFinalNewline` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderFinalNewline", setter = renderFinalNewline)]
    pub fn set_render_final_newline(this: &IEditorOptions, val: Option<bool>);
    /// Should the corresponding line be selected when clicking on the line
    /// number? Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectOnLineNumbers", getter = selectOnLineNumbers)]
    pub fn select_on_line_numbers(this: &IEditorOptions) -> Option<bool>;
    /// Set the `selectOnLineNumbers` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectOnLineNumbers", setter = selectOnLineNumbers)]
    pub fn set_select_on_line_numbers(this: &IEditorOptions, val: Option<bool>);
    /// Control the width of line numbers, by reserving horizontal space for
    /// rendering at least an amount of digits. Defaults to 5.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineNumbersMinChars", getter = lineNumbersMinChars)]
    pub fn line_numbers_min_chars(this: &IEditorOptions) -> Option<f64>;
    /// Set the `lineNumbersMinChars` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineNumbersMinChars", setter = lineNumbersMinChars)]
    pub fn set_line_numbers_min_chars(this: &IEditorOptions, val: Option<f64>);
    /// Enable the rendering of the glyph margin.
    /// Defaults to true in vscode and to false in monaco-editor.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "glyphMargin", getter = glyphMargin)]
    pub fn glyph_margin(this: &IEditorOptions) -> Option<bool>;
    /// Set the `glyphMargin` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "glyphMargin", setter = glyphMargin)]
    pub fn set_glyph_margin(this: &IEditorOptions, val: Option<bool>);
    /// The width reserved for line decorations (in px).
    /// Line decorations are placed between line numbers and the editor content.
    /// You can pass in a string in the format floating point followed by "ch".
    /// e.g. 1.3ch. Defaults to 10.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineDecorationsWidth", getter = lineDecorationsWidth)]
    pub fn line_decorations_width(this: &IEditorOptions) -> Option<f64>;
    /// Set the `lineDecorationsWidth` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineDecorationsWidth", setter = lineDecorationsWidth)]
    pub fn set_line_decorations_width(this: &IEditorOptions, val: Option<f64>);
    /// When revealing the cursor, a virtual padding (px) is added to the
    /// cursor, turning it into a rectangle. This virtual padding ensures
    /// that the cursor gets revealed before hitting the edge of the viewport.
    /// Defaults to 30 (px).
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "revealHorizontalRightPadding", getter = revealHorizontalRightPadding)]
    pub fn reveal_horizontal_right_padding(this: &IEditorOptions) -> Option<f64>;
    /// Set the `revealHorizontalRightPadding` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "revealHorizontalRightPadding", setter = revealHorizontalRightPadding)]
    pub fn set_reveal_horizontal_right_padding(this: &IEditorOptions, val: Option<f64>);
    /// Render the editor selection with rounded borders.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "roundedSelection", getter = roundedSelection)]
    pub fn rounded_selection(this: &IEditorOptions) -> Option<bool>;
    /// Set the `roundedSelection` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "roundedSelection", setter = roundedSelection)]
    pub fn set_rounded_selection(this: &IEditorOptions, val: Option<bool>);
    /// Class name to be added to the editor.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "extraEditorClassName", getter = extraEditorClassName)]
    pub fn extra_editor_class_name(this: &IEditorOptions) -> Option<String>;
    /// Set the `extraEditorClassName` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "extraEditorClassName", setter = extraEditorClassName)]
    pub fn set_extra_editor_class_name(this: &IEditorOptions, val: Option<&str>);
    /// Should the editor be read only.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "readOnly", getter = readOnly)]
    pub fn read_only(this: &IEditorOptions) -> Option<bool>;
    /// Set the `readOnly` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "readOnly", setter = readOnly)]
    pub fn set_read_only(this: &IEditorOptions, val: Option<bool>);
    /// Should the editor render validation decorations.
    /// Defaults to editable.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderValidationDecorations", getter = renderValidationDecorations)]
    pub fn render_validation_decorations(
        this: &IEditorOptions,
    ) -> Option<IEditorOptionsRenderValidationDecorations>;
    /// Set the `renderValidationDecorations` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderValidationDecorations", setter = renderValidationDecorations)]
    pub fn set_render_validation_decorations(
        this: &IEditorOptions,
        val: Option<IEditorOptionsRenderValidationDecorations>,
    );
    /// Control the behavior and rendering of the scrollbars.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollbar", getter = scrollbar)]
    pub fn scrollbar(this: &IEditorOptions) -> Option<IEditorScrollbarOptions>;
    /// Set the `scrollbar` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollbar", setter = scrollbar)]
    pub fn set_scrollbar(this: &IEditorOptions, val: Option<&IEditorScrollbarOptions>);
    /// Control the behavior and rendering of the minimap.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "minimap", getter = minimap)]
    pub fn minimap(this: &IEditorOptions) -> Option<IEditorMinimapOptions>;
    /// Set the `minimap` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "minimap", setter = minimap)]
    pub fn set_minimap(this: &IEditorOptions, val: Option<&IEditorMinimapOptions>);
    /// Control the behavior of the find widget.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "find", getter = find)]
    pub fn find(this: &IEditorOptions) -> Option<IEditorFindOptions>;
    /// Set the `find` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "find", setter = find)]
    pub fn set_find(this: &IEditorOptions, val: Option<&IEditorFindOptions>);
    /// Display overflow widgets as `fixed`.
    /// Defaults to `false`.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fixedOverflowWidgets", getter = fixedOverflowWidgets)]
    pub fn fixed_overflow_widgets(this: &IEditorOptions) -> Option<bool>;
    /// Set the `fixedOverflowWidgets` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fixedOverflowWidgets", setter = fixedOverflowWidgets)]
    pub fn set_fixed_overflow_widgets(this: &IEditorOptions, val: Option<bool>);
    /// The number of vertical lanes the overview ruler should render.
    /// Defaults to 3.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "overviewRulerLanes", getter = overviewRulerLanes)]
    pub fn overview_ruler_lanes(this: &IEditorOptions) -> Option<f64>;
    /// Set the `overviewRulerLanes` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "overviewRulerLanes", setter = overviewRulerLanes)]
    pub fn set_overview_ruler_lanes(this: &IEditorOptions, val: Option<f64>);
    /// Controls if a border should be drawn around the overview ruler.
    /// Defaults to `true`.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "overviewRulerBorder", getter = overviewRulerBorder)]
    pub fn overview_ruler_border(this: &IEditorOptions) -> Option<bool>;
    /// Set the `overviewRulerBorder` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "overviewRulerBorder", setter = overviewRulerBorder)]
    pub fn set_overview_ruler_border(this: &IEditorOptions, val: Option<bool>);
    /// Control the cursor animation style, possible values are 'blink',
    /// 'smooth', 'phase', 'expand' and 'solid'. Defaults to 'blink'.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorBlinking", getter = cursorBlinking)]
    pub fn cursor_blinking(this: &IEditorOptions) -> Option<IEditorOptionsCursorBlinking>;
    /// Set the `cursorBlinking` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorBlinking", setter = cursorBlinking)]
    pub fn set_cursor_blinking(this: &IEditorOptions, val: Option<IEditorOptionsCursorBlinking>);
    /// Zoom the font in the editor when using the mouse wheel in combination
    /// with holding Ctrl. Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseWheelZoom", getter = mouseWheelZoom)]
    pub fn mouse_wheel_zoom(this: &IEditorOptions) -> Option<bool>;
    /// Set the `mouseWheelZoom` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseWheelZoom", setter = mouseWheelZoom)]
    pub fn set_mouse_wheel_zoom(this: &IEditorOptions, val: Option<bool>);
    /// Control the mouse pointer style, either 'text' or 'default' or 'copy'
    /// Defaults to 'text'
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseStyle", getter = mouseStyle)]
    pub fn mouse_style(this: &IEditorOptions) -> Option<IEditorOptionsMouseStyle>;
    /// Set the `mouseStyle` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseStyle", setter = mouseStyle)]
    pub fn set_mouse_style(this: &IEditorOptions, val: Option<IEditorOptionsMouseStyle>);
    /// Enable smooth caret animation.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSmoothCaretAnimation", getter = cursorSmoothCaretAnimation)]
    pub fn cursor_smooth_caret_animation(this: &IEditorOptions) -> Option<bool>;
    /// Set the `cursorSmoothCaretAnimation` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSmoothCaretAnimation", setter = cursorSmoothCaretAnimation)]
    pub fn set_cursor_smooth_caret_animation(this: &IEditorOptions, val: Option<bool>);
    /// Control the cursor style, either 'block' or 'line'.
    /// Defaults to 'line'.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorStyle", getter = cursorStyle)]
    pub fn cursor_style(this: &IEditorOptions) -> Option<IEditorOptionsCursorStyle>;
    /// Set the `cursorStyle` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorStyle", setter = cursorStyle)]
    pub fn set_cursor_style(this: &IEditorOptions, val: Option<IEditorOptionsCursorStyle>);
    /// Control the width of the cursor when cursorStyle is set to 'line'
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorWidth", getter = cursorWidth)]
    pub fn cursor_width(this: &IEditorOptions) -> Option<f64>;
    /// Set the `cursorWidth` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorWidth", setter = cursorWidth)]
    pub fn set_cursor_width(this: &IEditorOptions, val: Option<f64>);
    /// Enable font ligatures.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontLigatures", getter = fontLigatures)]
    pub fn font_ligatures(this: &IEditorOptions) -> Option<bool>;
    /// Set the `fontLigatures` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontLigatures", setter = fontLigatures)]
    pub fn set_font_ligatures(this: &IEditorOptions, val: Option<bool>);
    /// Disable the use of `transform: translate3d(0px, 0px, 0px)` for the
    /// editor margin and lines layers. The usage of `transform:
    /// translate3d(0px, 0px, 0px)` acts as a hint for browsers to create an
    /// extra layer. Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "disableLayerHinting", getter = disableLayerHinting)]
    pub fn disable_layer_hinting(this: &IEditorOptions) -> Option<bool>;
    /// Set the `disableLayerHinting` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "disableLayerHinting", setter = disableLayerHinting)]
    pub fn set_disable_layer_hinting(this: &IEditorOptions, val: Option<bool>);
    /// Disable the optimizations for monospace fonts.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "disableMonospaceOptimizations", getter = disableMonospaceOptimizations)]
    pub fn disable_monospace_optimizations(this: &IEditorOptions) -> Option<bool>;
    /// Set the `disableMonospaceOptimizations` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "disableMonospaceOptimizations", setter = disableMonospaceOptimizations)]
    pub fn set_disable_monospace_optimizations(this: &IEditorOptions, val: Option<bool>);
    /// Should the cursor be hidden in the overview ruler.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "hideCursorInOverviewRuler", getter = hideCursorInOverviewRuler)]
    pub fn hide_cursor_in_overview_ruler(this: &IEditorOptions) -> Option<bool>;
    /// Set the `hideCursorInOverviewRuler` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "hideCursorInOverviewRuler", setter = hideCursorInOverviewRuler)]
    pub fn set_hide_cursor_in_overview_ruler(this: &IEditorOptions, val: Option<bool>);
    /// Enable that scrolling can go one screen size after the last line.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollBeyondLastLine", getter = scrollBeyondLastLine)]
    pub fn scroll_beyond_last_line(this: &IEditorOptions) -> Option<bool>;
    /// Set the `scrollBeyondLastLine` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollBeyondLastLine", setter = scrollBeyondLastLine)]
    pub fn set_scroll_beyond_last_line(this: &IEditorOptions, val: Option<bool>);
    /// Enable that scrolling can go beyond the last column by a number of
    /// columns. Defaults to 5.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollBeyondLastColumn", getter = scrollBeyondLastColumn)]
    pub fn scroll_beyond_last_column(this: &IEditorOptions) -> Option<f64>;
    /// Set the `scrollBeyondLastColumn` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollBeyondLastColumn", setter = scrollBeyondLastColumn)]
    pub fn set_scroll_beyond_last_column(this: &IEditorOptions, val: Option<f64>);
    /// Enable that the editor animates scrolling to a position.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "smoothScrolling", getter = smoothScrolling)]
    pub fn smooth_scrolling(this: &IEditorOptions) -> Option<bool>;
    /// Set the `smoothScrolling` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "smoothScrolling", setter = smoothScrolling)]
    pub fn set_smooth_scrolling(this: &IEditorOptions, val: Option<bool>);
    /// Enable that the editor will install an interval to check if its
    /// container dom node size has changed. Enabling this might have a
    /// severe performance impact. Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "automaticLayout", getter = automaticLayout)]
    pub fn automatic_layout(this: &IEditorOptions) -> Option<bool>;
    /// Set the `automaticLayout` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "automaticLayout", setter = automaticLayout)]
    pub fn set_automatic_layout(this: &IEditorOptions, val: Option<bool>);
    /// Control the wrapping of the editor.
    /// When `wordWrap` = "off", the lines will never wrap.
    /// When `wordWrap` = "on", the lines will wrap at the viewport width.
    /// When `wordWrap` = "wordWrapColumn", the lines will wrap at
    /// `wordWrapColumn`. When `wordWrap` = "bounded", the lines will wrap
    /// at min(viewport width, wordWrapColumn). Defaults to "off".
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrap", getter = wordWrap)]
    pub fn word_wrap(this: &IEditorOptions) -> Option<IEditorOptionsWordWrap>;
    /// Set the `wordWrap` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrap", setter = wordWrap)]
    pub fn set_word_wrap(this: &IEditorOptions, val: Option<IEditorOptionsWordWrap>);
    /// Control the wrapping of the editor.
    /// When `wordWrap` = "off", the lines will never wrap.
    /// When `wordWrap` = "on", the lines will wrap at the viewport width.
    /// When `wordWrap` = "wordWrapColumn", the lines will wrap at
    /// `wordWrapColumn`. When `wordWrap` = "bounded", the lines will wrap
    /// at min(viewport width, wordWrapColumn). Defaults to 80.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapColumn", getter = wordWrapColumn)]
    pub fn word_wrap_column(this: &IEditorOptions) -> Option<f64>;
    /// Set the `wordWrapColumn` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapColumn", setter = wordWrapColumn)]
    pub fn set_word_wrap_column(this: &IEditorOptions, val: Option<f64>);
    /// Force word wrapping when the text appears to be of a minified/generated
    /// file. Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapMinified", getter = wordWrapMinified)]
    pub fn word_wrap_minified(this: &IEditorOptions) -> Option<bool>;
    /// Set the `wordWrapMinified` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapMinified", setter = wordWrapMinified)]
    pub fn set_word_wrap_minified(this: &IEditorOptions, val: Option<bool>);
    /// Control indentation of wrapped lines. Can be: 'none', 'same', 'indent'
    /// or 'deepIndent'. Defaults to 'same' in vscode and to 'none' in
    /// monaco-editor.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wrappingIndent", getter = wrappingIndent)]
    pub fn wrapping_indent(this: &IEditorOptions) -> Option<IEditorOptionsWrappingIndent>;
    /// Set the `wrappingIndent` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wrappingIndent", setter = wrappingIndent)]
    pub fn set_wrapping_indent(this: &IEditorOptions, val: Option<IEditorOptionsWrappingIndent>);
    /// Controls the wrapping strategy to use.
    /// Defaults to 'simple'.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wrappingStrategy", getter = wrappingStrategy)]
    pub fn wrapping_strategy(this: &IEditorOptions) -> Option<IEditorOptionsWrappingStrategy>;
    /// Set the `wrappingStrategy` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wrappingStrategy", setter = wrappingStrategy)]
    pub fn set_wrapping_strategy(
        this: &IEditorOptions,
        val: Option<IEditorOptionsWrappingStrategy>,
    );
    /// Configure word wrapping characters. A break will be introduced before
    /// these characters. Defaults to '([{‘“〈《「『【〔（［｛｢£¥＄￡￥+＋'.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapBreakBeforeCharacters", getter = wordWrapBreakBeforeCharacters)]
    pub fn word_wrap_break_before_characters(this: &IEditorOptions) -> Option<String>;
    /// Set the `wordWrapBreakBeforeCharacters` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapBreakBeforeCharacters", setter = wordWrapBreakBeforeCharacters)]
    pub fn set_word_wrap_break_before_characters(this: &IEditorOptions, val: Option<&str>);
    /// Configure word wrapping characters. A break will be introduced after
    /// these characters. Defaults to '
    /// \t})]?|/&.,;¢°′″‰℃、。｡､￠，．：；？！％・･
    /// ゝゞヽヾーァィゥェォッャュョヮヵヶぁぃぅぇぉっゃゅょゎゕゖㇰㇱㇲㇳㇴㇵㇶㇷㇸㇹㇺㇻㇼㇽㇾㇿ々〻ｧｨｩｪｫｬｭｮｯｰ”〉》」』】〕）］｝｣'
    /// .
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapBreakAfterCharacters", getter = wordWrapBreakAfterCharacters)]
    pub fn word_wrap_break_after_characters(this: &IEditorOptions) -> Option<String>;
    /// Set the `wordWrapBreakAfterCharacters` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapBreakAfterCharacters", setter = wordWrapBreakAfterCharacters)]
    pub fn set_word_wrap_break_after_characters(this: &IEditorOptions, val: Option<&str>);
    /// Performance guard: Stop rendering a line after x characters.
    /// Defaults to 10000.
    /// Use -1 to never stop rendering
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "stopRenderingLineAfter", getter = stopRenderingLineAfter)]
    pub fn stop_rendering_line_after(this: &IEditorOptions) -> Option<f64>;
    /// Set the `stopRenderingLineAfter` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "stopRenderingLineAfter", setter = stopRenderingLineAfter)]
    pub fn set_stop_rendering_line_after(this: &IEditorOptions, val: Option<f64>);
    /// Configure the editor's hover.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "hover", getter = hover)]
    pub fn hover(this: &IEditorOptions) -> Option<IEditorHoverOptions>;
    /// Set the `hover` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "hover", setter = hover)]
    pub fn set_hover(this: &IEditorOptions, val: Option<&IEditorHoverOptions>);
    /// Enable detecting links and making them clickable.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "links", getter = links)]
    pub fn links(this: &IEditorOptions) -> Option<bool>;
    /// Set the `links` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "links", setter = links)]
    pub fn set_links(this: &IEditorOptions, val: Option<bool>);
    /// Enable inline color decorators and color picker rendering.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "colorDecorators", getter = colorDecorators)]
    pub fn color_decorators(this: &IEditorOptions) -> Option<bool>;
    /// Set the `colorDecorators` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "colorDecorators", setter = colorDecorators)]
    pub fn set_color_decorators(this: &IEditorOptions, val: Option<bool>);
    /// Control the behaviour of comments in the editor.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "comments", getter = comments)]
    pub fn comments(this: &IEditorOptions) -> Option<IEditorCommentsOptions>;
    /// Set the `comments` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "comments", setter = comments)]
    pub fn set_comments(this: &IEditorOptions, val: Option<&IEditorCommentsOptions>);
    /// Enable custom contextmenu.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "contextmenu", getter = contextmenu)]
    pub fn contextmenu(this: &IEditorOptions) -> Option<bool>;
    /// Set the `contextmenu` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "contextmenu", setter = contextmenu)]
    pub fn set_contextmenu(this: &IEditorOptions, val: Option<bool>);
    /// A multiplier to be used on the `deltaX` and `deltaY` of mouse wheel
    /// scroll events. Defaults to 1.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseWheelScrollSensitivity", getter = mouseWheelScrollSensitivity)]
    pub fn mouse_wheel_scroll_sensitivity(this: &IEditorOptions) -> Option<f64>;
    /// Set the `mouseWheelScrollSensitivity` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseWheelScrollSensitivity", setter = mouseWheelScrollSensitivity)]
    pub fn set_mouse_wheel_scroll_sensitivity(this: &IEditorOptions, val: Option<f64>);
    /// FastScrolling mulitplier speed when pressing `Alt`
    /// Defaults to 5.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fastScrollSensitivity", getter = fastScrollSensitivity)]
    pub fn fast_scroll_sensitivity(this: &IEditorOptions) -> Option<f64>;
    /// Set the `fastScrollSensitivity` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fastScrollSensitivity", setter = fastScrollSensitivity)]
    pub fn set_fast_scroll_sensitivity(this: &IEditorOptions, val: Option<f64>);
    /// The modifier to be used to add multiple cursors with the mouse.
    /// Defaults to 'alt'
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorModifier", getter = multiCursorModifier)]
    pub fn multi_cursor_modifier(
        this: &IEditorOptions,
    ) -> Option<IEditorOptionsMultiCursorModifier>;
    /// Set the `multiCursorModifier` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorModifier", setter = multiCursorModifier)]
    pub fn set_multi_cursor_modifier(
        this: &IEditorOptions,
        val: Option<IEditorOptionsMultiCursorModifier>,
    );
    /// Merge overlapping selections.
    /// Defaults to true
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorMergeOverlapping", getter = multiCursorMergeOverlapping)]
    pub fn multi_cursor_merge_overlapping(this: &IEditorOptions) -> Option<bool>;
    /// Set the `multiCursorMergeOverlapping` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorMergeOverlapping", setter = multiCursorMergeOverlapping)]
    pub fn set_multi_cursor_merge_overlapping(this: &IEditorOptions, val: Option<bool>);
    /// Configure the behaviour when pasting a text with the line count equal to
    /// the cursor count. Defaults to 'spread'.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorPaste", getter = multiCursorPaste)]
    pub fn multi_cursor_paste(this: &IEditorOptions) -> Option<IEditorOptionsMultiCursorPaste>;
    /// Set the `multiCursorPaste` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorPaste", setter = multiCursorPaste)]
    pub fn set_multi_cursor_paste(
        this: &IEditorOptions,
        val: Option<IEditorOptionsMultiCursorPaste>,
    );
    /// Configure the editor's accessibility support.
    /// Defaults to 'auto'. It is best to leave this to 'auto'.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "accessibilitySupport", getter = accessibilitySupport)]
    pub fn accessibility_support(
        this: &IEditorOptions,
    ) -> Option<IEditorOptionsAccessibilitySupport>;
    /// Set the `accessibilitySupport` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "accessibilitySupport", setter = accessibilitySupport)]
    pub fn set_accessibility_support(
        this: &IEditorOptions,
        val: Option<IEditorOptionsAccessibilitySupport>,
    );
    /// Controls the number of lines in the editor that can be read out by a
    /// screen reader
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "accessibilityPageSize", getter = accessibilityPageSize)]
    pub fn accessibility_page_size(this: &IEditorOptions) -> Option<f64>;
    /// Set the `accessibilityPageSize` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "accessibilityPageSize", setter = accessibilityPageSize)]
    pub fn set_accessibility_page_size(this: &IEditorOptions, val: Option<f64>);
    /// Suggest options.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggest", getter = suggest)]
    pub fn suggest(this: &IEditorOptions) -> Option<ISuggestOptions>;
    /// Set the `suggest` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggest", setter = suggest)]
    pub fn set_suggest(this: &IEditorOptions, val: Option<&ISuggestOptions>);
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "gotoLocation", getter = gotoLocation)]
    pub fn goto_location(this: &IEditorOptions) -> Option<IGotoLocationOptions>;
    /// Set the `gotoLocation` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "gotoLocation", setter = gotoLocation)]
    pub fn set_goto_location(this: &IEditorOptions, val: Option<&IGotoLocationOptions>);
    /// Enable quick suggestions (shadow suggestions)
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "quickSuggestions", getter = quickSuggestions)]
    pub fn quick_suggestions(this: &IEditorOptions) -> Option<IQuickSuggestionsOptions>;
    /// Set the `quickSuggestions` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "quickSuggestions", setter = quickSuggestions)]
    pub fn set_quick_suggestions(this: &IEditorOptions, val: Option<&IQuickSuggestionsOptions>);
    /// Quick suggestions show delay (in ms)
    /// Defaults to 10 (ms)
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "quickSuggestionsDelay", getter = quickSuggestionsDelay)]
    pub fn quick_suggestions_delay(this: &IEditorOptions) -> Option<f64>;
    /// Set the `quickSuggestionsDelay` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "quickSuggestionsDelay", setter = quickSuggestionsDelay)]
    pub fn set_quick_suggestions_delay(this: &IEditorOptions, val: Option<f64>);
    /// Parameter hint options.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "parameterHints", getter = parameterHints)]
    pub fn parameter_hints(this: &IEditorOptions) -> Option<IEditorParameterHintOptions>;
    /// Set the `parameterHints` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "parameterHints", setter = parameterHints)]
    pub fn set_parameter_hints(this: &IEditorOptions, val: Option<&IEditorParameterHintOptions>);
    /// Options for auto closing brackets.
    /// Defaults to language defined behavior.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingBrackets", getter = autoClosingBrackets)]
    pub fn auto_closing_brackets(this: &IEditorOptions) -> Option<EditorAutoClosingStrategy>;
    /// Set the `autoClosingBrackets` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingBrackets", setter = autoClosingBrackets)]
    pub fn set_auto_closing_brackets(this: &IEditorOptions, val: Option<EditorAutoClosingStrategy>);
    /// Options for auto closing quotes.
    /// Defaults to language defined behavior.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingQuotes", getter = autoClosingQuotes)]
    pub fn auto_closing_quotes(this: &IEditorOptions) -> Option<EditorAutoClosingStrategy>;
    /// Set the `autoClosingQuotes` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingQuotes", setter = autoClosingQuotes)]
    pub fn set_auto_closing_quotes(this: &IEditorOptions, val: Option<EditorAutoClosingStrategy>);
    /// Options for typing over closing quotes or brackets.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingOvertype", getter = autoClosingOvertype)]
    pub fn auto_closing_overtype(
        this: &IEditorOptions,
    ) -> Option<EditorAutoClosingOvertypeStrategy>;
    /// Set the `autoClosingOvertype` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingOvertype", setter = autoClosingOvertype)]
    pub fn set_auto_closing_overtype(
        this: &IEditorOptions,
        val: Option<EditorAutoClosingOvertypeStrategy>,
    );
    /// Options for auto surrounding.
    /// Defaults to always allowing auto surrounding.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoSurround", getter = autoSurround)]
    pub fn auto_surround(this: &IEditorOptions) -> Option<EditorAutoSurroundStrategy>;
    /// Set the `autoSurround` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoSurround", setter = autoSurround)]
    pub fn set_auto_surround(this: &IEditorOptions, val: Option<EditorAutoSurroundStrategy>);
    /// Controls whether the editor should automatically adjust the indentation
    /// when users type, paste, move or indent lines. Defaults to advanced.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoIndent", getter = autoIndent)]
    pub fn auto_indent(this: &IEditorOptions) -> Option<IEditorOptionsAutoIndent>;
    /// Set the `autoIndent` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoIndent", setter = autoIndent)]
    pub fn set_auto_indent(this: &IEditorOptions, val: Option<IEditorOptionsAutoIndent>);
    /// Enable format on type.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "formatOnType", getter = formatOnType)]
    pub fn format_on_type(this: &IEditorOptions) -> Option<bool>;
    /// Set the `formatOnType` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "formatOnType", setter = formatOnType)]
    pub fn set_format_on_type(this: &IEditorOptions, val: Option<bool>);
    /// Enable format on paste.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "formatOnPaste", getter = formatOnPaste)]
    pub fn format_on_paste(this: &IEditorOptions) -> Option<bool>;
    /// Set the `formatOnPaste` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "formatOnPaste", setter = formatOnPaste)]
    pub fn set_format_on_paste(this: &IEditorOptions, val: Option<bool>);
    /// Controls if the editor should allow to move selections via drag and
    /// drop. Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "dragAndDrop", getter = dragAndDrop)]
    pub fn drag_and_drop(this: &IEditorOptions) -> Option<bool>;
    /// Set the `dragAndDrop` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "dragAndDrop", setter = dragAndDrop)]
    pub fn set_drag_and_drop(this: &IEditorOptions, val: Option<bool>);
    /// Enable the suggestion box to pop-up on trigger characters.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestOnTriggerCharacters", getter = suggestOnTriggerCharacters)]
    pub fn suggest_on_trigger_characters(this: &IEditorOptions) -> Option<bool>;
    /// Set the `suggestOnTriggerCharacters` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestOnTriggerCharacters", setter = suggestOnTriggerCharacters)]
    pub fn set_suggest_on_trigger_characters(this: &IEditorOptions, val: Option<bool>);
    /// Accept suggestions on ENTER.
    /// Defaults to 'on'.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "acceptSuggestionOnEnter", getter = acceptSuggestionOnEnter)]
    pub fn accept_suggestion_on_enter(
        this: &IEditorOptions,
    ) -> Option<IEditorOptionsAcceptSuggestionOnEnter>;
    /// Set the `acceptSuggestionOnEnter` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "acceptSuggestionOnEnter", setter = acceptSuggestionOnEnter)]
    pub fn set_accept_suggestion_on_enter(
        this: &IEditorOptions,
        val: Option<IEditorOptionsAcceptSuggestionOnEnter>,
    );
    /// Accept suggestions on provider defined characters.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "acceptSuggestionOnCommitCharacter", getter = acceptSuggestionOnCommitCharacter)]
    pub fn accept_suggestion_on_commit_character(this: &IEditorOptions) -> Option<bool>;
    /// Set the `acceptSuggestionOnCommitCharacter` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "acceptSuggestionOnCommitCharacter", setter = acceptSuggestionOnCommitCharacter)]
    pub fn set_accept_suggestion_on_commit_character(this: &IEditorOptions, val: Option<bool>);
    /// Enable snippet suggestions. Default to 'true'.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "snippetSuggestions", getter = snippetSuggestions)]
    pub fn snippet_suggestions(this: &IEditorOptions) -> Option<IEditorOptionsSnippetSuggestions>;
    /// Set the `snippetSuggestions` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "snippetSuggestions", setter = snippetSuggestions)]
    pub fn set_snippet_suggestions(
        this: &IEditorOptions,
        val: Option<IEditorOptionsSnippetSuggestions>,
    );
    /// Copying without a selection copies the current line.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "emptySelectionClipboard", getter = emptySelectionClipboard)]
    pub fn empty_selection_clipboard(this: &IEditorOptions) -> Option<bool>;
    /// Set the `emptySelectionClipboard` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "emptySelectionClipboard", setter = emptySelectionClipboard)]
    pub fn set_empty_selection_clipboard(this: &IEditorOptions, val: Option<bool>);
    /// Syntax highlighting is copied.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "copyWithSyntaxHighlighting", getter = copyWithSyntaxHighlighting)]
    pub fn copy_with_syntax_highlighting(this: &IEditorOptions) -> Option<bool>;
    /// Set the `copyWithSyntaxHighlighting` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "copyWithSyntaxHighlighting", setter = copyWithSyntaxHighlighting)]
    pub fn set_copy_with_syntax_highlighting(this: &IEditorOptions, val: Option<bool>);
    /// The history mode for suggestions.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestSelection", getter = suggestSelection)]
    pub fn suggest_selection(this: &IEditorOptions) -> Option<IEditorOptionsSuggestSelection>;
    /// Set the `suggestSelection` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestSelection", setter = suggestSelection)]
    pub fn set_suggest_selection(
        this: &IEditorOptions,
        val: Option<IEditorOptionsSuggestSelection>,
    );
    /// The font size for the suggest widget.
    /// Defaults to the editor font size.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestFontSize", getter = suggestFontSize)]
    pub fn suggest_font_size(this: &IEditorOptions) -> Option<f64>;
    /// Set the `suggestFontSize` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestFontSize", setter = suggestFontSize)]
    pub fn set_suggest_font_size(this: &IEditorOptions, val: Option<f64>);
    /// The line height for the suggest widget.
    /// Defaults to the editor line height.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestLineHeight", getter = suggestLineHeight)]
    pub fn suggest_line_height(this: &IEditorOptions) -> Option<f64>;
    /// Set the `suggestLineHeight` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestLineHeight", setter = suggestLineHeight)]
    pub fn set_suggest_line_height(this: &IEditorOptions, val: Option<f64>);
    /// Enable tab completion.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "tabCompletion", getter = tabCompletion)]
    pub fn tab_completion(this: &IEditorOptions) -> Option<IEditorOptionsTabCompletion>;
    /// Set the `tabCompletion` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "tabCompletion", setter = tabCompletion)]
    pub fn set_tab_completion(this: &IEditorOptions, val: Option<IEditorOptionsTabCompletion>);
    /// Enable selection highlight.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectionHighlight", getter = selectionHighlight)]
    pub fn selection_highlight(this: &IEditorOptions) -> Option<bool>;
    /// Set the `selectionHighlight` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectionHighlight", setter = selectionHighlight)]
    pub fn set_selection_highlight(this: &IEditorOptions, val: Option<bool>);
    /// Enable semantic occurrences highlight.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "occurrencesHighlight", getter = occurrencesHighlight)]
    pub fn occurrences_highlight(this: &IEditorOptions) -> Option<bool>;
    /// Set the `occurrencesHighlight` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "occurrencesHighlight", setter = occurrencesHighlight)]
    pub fn set_occurrences_highlight(this: &IEditorOptions, val: Option<bool>);
    /// Show code lens
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeLens", getter = codeLens)]
    pub fn code_lens(this: &IEditorOptions) -> Option<bool>;
    /// Set the `codeLens` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeLens", setter = codeLens)]
    pub fn set_code_lens(this: &IEditorOptions, val: Option<bool>);
    /// Control the behavior and rendering of the code action lightbulb.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lightbulb", getter = lightbulb)]
    pub fn lightbulb(this: &IEditorOptions) -> Option<IEditorLightbulbOptions>;
    /// Set the `lightbulb` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lightbulb", setter = lightbulb)]
    pub fn set_lightbulb(this: &IEditorOptions, val: Option<&IEditorLightbulbOptions>);
    /// Timeout for running code actions on save.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeActionsOnSaveTimeout", getter = codeActionsOnSaveTimeout)]
    pub fn code_actions_on_save_timeout(this: &IEditorOptions) -> Option<f64>;
    /// Set the `codeActionsOnSaveTimeout` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeActionsOnSaveTimeout", setter = codeActionsOnSaveTimeout)]
    pub fn set_code_actions_on_save_timeout(this: &IEditorOptions, val: Option<f64>);
    /// Enable code folding.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "folding", getter = folding)]
    pub fn folding(this: &IEditorOptions) -> Option<bool>;
    /// Set the `folding` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "folding", setter = folding)]
    pub fn set_folding(this: &IEditorOptions, val: Option<bool>);
    /// Selects the folding strategy. 'auto' uses the strategies contributed for
    /// the current document, 'indentation' uses the indentation based folding
    /// strategy. Defaults to 'auto'.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingStrategy", getter = foldingStrategy)]
    pub fn folding_strategy(this: &IEditorOptions) -> Option<IEditorOptionsFoldingStrategy>;
    /// Set the `foldingStrategy` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingStrategy", setter = foldingStrategy)]
    pub fn set_folding_strategy(this: &IEditorOptions, val: Option<IEditorOptionsFoldingStrategy>);
    /// Enable highlight for folded regions.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingHighlight", getter = foldingHighlight)]
    pub fn folding_highlight(this: &IEditorOptions) -> Option<bool>;
    /// Set the `foldingHighlight` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingHighlight", setter = foldingHighlight)]
    pub fn set_folding_highlight(this: &IEditorOptions, val: Option<bool>);
    /// Controls whether the fold actions in the gutter stay always visible or
    /// hide unless the mouse is over the gutter. Defaults to 'mouseover'.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "showFoldingControls", getter = showFoldingControls)]
    pub fn show_folding_controls(
        this: &IEditorOptions,
    ) -> Option<IEditorOptionsShowFoldingControls>;
    /// Set the `showFoldingControls` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "showFoldingControls", setter = showFoldingControls)]
    pub fn set_show_folding_controls(
        this: &IEditorOptions,
        val: Option<IEditorOptionsShowFoldingControls>,
    );
    /// Enable highlighting of matching brackets.
    /// Defaults to 'always'.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "matchBrackets", getter = matchBrackets)]
    pub fn match_brackets(this: &IEditorOptions) -> Option<IEditorOptionsMatchBrackets>;
    /// Set the `matchBrackets` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "matchBrackets", setter = matchBrackets)]
    pub fn set_match_brackets(this: &IEditorOptions, val: Option<IEditorOptionsMatchBrackets>);
    /// Enable rendering of whitespace.
    /// Defaults to none.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderWhitespace", getter = renderWhitespace)]
    pub fn render_whitespace(this: &IEditorOptions) -> Option<IEditorOptionsRenderWhitespace>;
    /// Set the `renderWhitespace` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderWhitespace", setter = renderWhitespace)]
    pub fn set_render_whitespace(
        this: &IEditorOptions,
        val: Option<IEditorOptionsRenderWhitespace>,
    );
    /// Enable rendering of control characters.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderControlCharacters", getter = renderControlCharacters)]
    pub fn render_control_characters(this: &IEditorOptions) -> Option<bool>;
    /// Set the `renderControlCharacters` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderControlCharacters", setter = renderControlCharacters)]
    pub fn set_render_control_characters(this: &IEditorOptions, val: Option<bool>);
    /// Enable rendering of indent guides.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderIndentGuides", getter = renderIndentGuides)]
    pub fn render_indent_guides(this: &IEditorOptions) -> Option<bool>;
    /// Set the `renderIndentGuides` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderIndentGuides", setter = renderIndentGuides)]
    pub fn set_render_indent_guides(this: &IEditorOptions, val: Option<bool>);
    /// Enable highlighting of the active indent guide.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "highlightActiveIndentGuide", getter = highlightActiveIndentGuide)]
    pub fn highlight_active_indent_guide(this: &IEditorOptions) -> Option<bool>;
    /// Set the `highlightActiveIndentGuide` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "highlightActiveIndentGuide", setter = highlightActiveIndentGuide)]
    pub fn set_highlight_active_indent_guide(this: &IEditorOptions, val: Option<bool>);
    /// Enable rendering of current line highlight.
    /// Defaults to all.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderLineHighlight", getter = renderLineHighlight)]
    pub fn render_line_highlight(
        this: &IEditorOptions,
    ) -> Option<IEditorOptionsRenderLineHighlight>;
    /// Set the `renderLineHighlight` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderLineHighlight", setter = renderLineHighlight)]
    pub fn set_render_line_highlight(
        this: &IEditorOptions,
        val: Option<IEditorOptionsRenderLineHighlight>,
    );
    /// Inserting and deleting whitespace follows tab stops.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "useTabStops", getter = useTabStops)]
    pub fn use_tab_stops(this: &IEditorOptions) -> Option<bool>;
    /// Set the `useTabStops` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "useTabStops", setter = useTabStops)]
    pub fn set_use_tab_stops(this: &IEditorOptions, val: Option<bool>);
    /// The font family
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontFamily", getter = fontFamily)]
    pub fn font_family(this: &IEditorOptions) -> Option<String>;
    /// Set the `fontFamily` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontFamily", setter = fontFamily)]
    pub fn set_font_family(this: &IEditorOptions, val: Option<&str>);
    /// The font weight
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontWeight", getter = fontWeight)]
    pub fn font_weight(this: &IEditorOptions) -> Option<String>;
    /// Set the `fontWeight` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontWeight", setter = fontWeight)]
    pub fn set_font_weight(this: &IEditorOptions, val: Option<&str>);
    /// The font size
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontSize", getter = fontSize)]
    pub fn font_size(this: &IEditorOptions) -> Option<f64>;
    /// Set the `fontSize` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontSize", setter = fontSize)]
    pub fn set_font_size(this: &IEditorOptions, val: Option<f64>);
    /// The line height
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineHeight", getter = lineHeight)]
    pub fn line_height(this: &IEditorOptions) -> Option<f64>;
    /// Set the `lineHeight` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineHeight", setter = lineHeight)]
    pub fn set_line_height(this: &IEditorOptions, val: Option<f64>);
    /// The letter spacing
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "letterSpacing", getter = letterSpacing)]
    pub fn letter_spacing(this: &IEditorOptions) -> Option<f64>;
    /// Set the `letterSpacing` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "letterSpacing", setter = letterSpacing)]
    pub fn set_letter_spacing(this: &IEditorOptions, val: Option<f64>);
    /// Controls fading out of unused variables.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "showUnused", getter = showUnused)]
    pub fn show_unused(this: &IEditorOptions) -> Option<bool>;
    /// Set the `showUnused` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "showUnused", setter = showUnused)]
    pub fn set_show_unused(this: &IEditorOptions, val: Option<bool>);
    /// Controls whether to focus the inline editor in the peek widget by
    /// default. Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "peekWidgetDefaultFocus", getter = peekWidgetDefaultFocus)]
    pub fn peek_widget_default_focus(
        this: &IEditorOptions,
    ) -> Option<IEditorOptionsPeekWidgetDefaultFocus>;
    /// Set the `peekWidgetDefaultFocus` property.
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "peekWidgetDefaultFocus", setter = peekWidgetDefaultFocus)]
    pub fn set_peek_widget_default_focus(
        this: &IEditorOptions,
        val: Option<IEditorOptionsPeekWidgetDefaultFocus>,
    );
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, PartialEq, Clone)]
    #[wasm_bindgen(extends = IEditorOptions, extends = Object)]
    pub type IEditorConstructionOptions;
    /// The initial editor dimension (to avoid measuring the container).
    #[wasm_bindgen(method, js_class = "IEditorConstructionOptions", js_name = "dimension", getter = dimension)]
    pub fn dimension(this: &IEditorConstructionOptions) -> Option<IDimension>;
    /// Set the `dimension` property.
    #[wasm_bindgen(method, js_class = "IEditorConstructionOptions", js_name = "dimension", setter = dimension)]
    pub fn set_dimension(this: &IEditorConstructionOptions, val: Option<&IDimension>);
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for the diff editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = IEditorOptions, extends = Object)]
    pub type IDiffEditorOptions;
    /// Allow the user to resize the diff editor split view.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "enableSplitViewResizing", getter = enableSplitViewResizing)]
    pub fn enable_split_view_resizing(this: &IDiffEditorOptions) -> Option<bool>;
    /// Set the `enableSplitViewResizing` property.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "enableSplitViewResizing", setter = enableSplitViewResizing)]
    pub fn set_enable_split_view_resizing(this: &IDiffEditorOptions, val: Option<bool>);
    /// Render the differences in two side-by-side editors.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "renderSideBySide", getter = renderSideBySide)]
    pub fn render_side_by_side(this: &IDiffEditorOptions) -> Option<bool>;
    /// Set the `renderSideBySide` property.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "renderSideBySide", setter = renderSideBySide)]
    pub fn set_render_side_by_side(this: &IDiffEditorOptions, val: Option<bool>);
    /// Timeout in milliseconds after which diff computation is cancelled.
    /// Defaults to 5000.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "maxComputationTime", getter = maxComputationTime)]
    pub fn max_computation_time(this: &IDiffEditorOptions) -> Option<f64>;
    /// Set the `maxComputationTime` property.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "maxComputationTime", setter = maxComputationTime)]
    pub fn set_max_computation_time(this: &IDiffEditorOptions, val: Option<f64>);
    /// Compute the diff by ignoring leading/trailing whitespace
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "ignoreTrimWhitespace", getter = ignoreTrimWhitespace)]
    pub fn ignore_trim_whitespace(this: &IDiffEditorOptions) -> Option<bool>;
    /// Set the `ignoreTrimWhitespace` property.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "ignoreTrimWhitespace", setter = ignoreTrimWhitespace)]
    pub fn set_ignore_trim_whitespace(this: &IDiffEditorOptions, val: Option<bool>);
    /// Render +/- indicators for added/deleted changes.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "renderIndicators", getter = renderIndicators)]
    pub fn render_indicators(this: &IDiffEditorOptions) -> Option<bool>;
    /// Set the `renderIndicators` property.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "renderIndicators", setter = renderIndicators)]
    pub fn set_render_indicators(this: &IDiffEditorOptions, val: Option<bool>);
    /// Original model should be editable?
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "originalEditable", getter = originalEditable)]
    pub fn original_editable(this: &IDiffEditorOptions) -> Option<bool>;
    /// Set the `originalEditable` property.
    #[wasm_bindgen(method, js_class = "IDiffEditorOptions", js_name = "originalEditable", setter = originalEditable)]
    pub fn set_original_editable(this: &IDiffEditorOptions, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    /// All computed editor options.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IComputedEditorOptions;
    #[wasm_bindgen(method, js_class = "IComputedEditorOptions", js_name = "get")]
    pub fn get(this: &IComputedEditorOptions, id: &JsValue) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorOption;
    #[wasm_bindgen(method, js_class = "IEditorOption", js_name = "id", getter = id)]
    pub fn id(this: &IEditorOption) -> String;
    #[wasm_bindgen(method, js_class = "IEditorOption", js_name = "name", getter = name)]
    pub fn name(this: &IEditorOption) -> String;
    #[wasm_bindgen(method, js_class = "IEditorOption", js_name = "defaultValue", getter = defaultValue)]
    pub fn default_value(this: &IEditorOption) -> JsValue;
    /// Set the `defaultValue` property.
    #[wasm_bindgen(method, js_class = "IEditorOption", js_name = "defaultValue", setter = defaultValue)]
    pub fn set_default_value(this: &IEditorOption, val: &JsValue);
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for editor comments
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorCommentsOptions;
    /// Insert a space after the line comment token and inside the block
    /// comments tokens. Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorCommentsOptions", js_name = "insertSpace", getter = insertSpace)]
    pub fn insert_space(this: &IEditorCommentsOptions) -> Option<bool>;
    /// Set the `insertSpace` property.
    #[wasm_bindgen(method, js_class = "IEditorCommentsOptions", js_name = "insertSpace", setter = insertSpace)]
    pub fn set_insert_space(this: &IEditorCommentsOptions, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for editor find widget
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorFindOptions;
    /// Controls if we seed search string in the Find Widget with editor
    /// selection.
    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "seedSearchStringFromSelection", getter = seedSearchStringFromSelection)]
    pub fn seed_search_string_from_selection(this: &IEditorFindOptions) -> Option<bool>;
    /// Set the `seedSearchStringFromSelection` property.
    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "seedSearchStringFromSelection", setter = seedSearchStringFromSelection)]
    pub fn set_seed_search_string_from_selection(this: &IEditorFindOptions, val: Option<bool>);
    /// Controls if Find in Selection flag is turned on in the editor.
    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "autoFindInSelection", getter = autoFindInSelection)]
    pub fn auto_find_in_selection(
        this: &IEditorFindOptions,
    ) -> Option<IEditorFindOptionsAutoFindInSelection>;
    /// Set the `autoFindInSelection` property.
    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "autoFindInSelection", setter = autoFindInSelection)]
    pub fn set_auto_find_in_selection(
        this: &IEditorFindOptions,
        val: Option<IEditorFindOptionsAutoFindInSelection>,
    );
    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "addExtraSpaceOnTop", getter = addExtraSpaceOnTop)]
    pub fn add_extra_space_on_top(this: &IEditorFindOptions) -> Option<bool>;
    /// Set the `addExtraSpaceOnTop` property.
    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "addExtraSpaceOnTop", setter = addExtraSpaceOnTop)]
    pub fn set_add_extra_space_on_top(this: &IEditorFindOptions, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for go to location
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IGotoLocationOptions;
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multiple", getter = multiple)]
    pub fn multiple(this: &IGotoLocationOptions) -> Option<GoToLocationValues>;
    /// Set the `multiple` property.
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multiple", setter = multiple)]
    pub fn set_multiple(this: &IGotoLocationOptions, val: Option<GoToLocationValues>);
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleDefinitions", getter = multipleDefinitions)]
    pub fn multiple_definitions(this: &IGotoLocationOptions) -> Option<GoToLocationValues>;
    /// Set the `multipleDefinitions` property.
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleDefinitions", setter = multipleDefinitions)]
    pub fn set_multiple_definitions(this: &IGotoLocationOptions, val: Option<GoToLocationValues>);
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleTypeDefinitions", getter = multipleTypeDefinitions)]
    pub fn multiple_type_definitions(this: &IGotoLocationOptions) -> Option<GoToLocationValues>;
    /// Set the `multipleTypeDefinitions` property.
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleTypeDefinitions", setter = multipleTypeDefinitions)]
    pub fn set_multiple_type_definitions(
        this: &IGotoLocationOptions,
        val: Option<GoToLocationValues>,
    );
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleDeclarations", getter = multipleDeclarations)]
    pub fn multiple_declarations(this: &IGotoLocationOptions) -> Option<GoToLocationValues>;
    /// Set the `multipleDeclarations` property.
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleDeclarations", setter = multipleDeclarations)]
    pub fn set_multiple_declarations(this: &IGotoLocationOptions, val: Option<GoToLocationValues>);
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleImplementations", getter = multipleImplementations)]
    pub fn multiple_implementations(this: &IGotoLocationOptions) -> Option<GoToLocationValues>;
    /// Set the `multipleImplementations` property.
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleImplementations", setter = multipleImplementations)]
    pub fn set_multiple_implementations(
        this: &IGotoLocationOptions,
        val: Option<GoToLocationValues>,
    );
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleReferences", getter = multipleReferences)]
    pub fn multiple_references(this: &IGotoLocationOptions) -> Option<GoToLocationValues>;
    /// Set the `multipleReferences` property.
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleReferences", setter = multipleReferences)]
    pub fn set_multiple_references(this: &IGotoLocationOptions, val: Option<GoToLocationValues>);
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeDefinitionCommand", getter = alternativeDefinitionCommand)]
    pub fn alternative_definition_command(this: &IGotoLocationOptions) -> Option<String>;
    /// Set the `alternativeDefinitionCommand` property.
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeDefinitionCommand", setter = alternativeDefinitionCommand)]
    pub fn set_alternative_definition_command(this: &IGotoLocationOptions, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeTypeDefinitionCommand", getter = alternativeTypeDefinitionCommand)]
    pub fn alternative_type_definition_command(this: &IGotoLocationOptions) -> Option<String>;
    /// Set the `alternativeTypeDefinitionCommand` property.
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeTypeDefinitionCommand", setter = alternativeTypeDefinitionCommand)]
    pub fn set_alternative_type_definition_command(this: &IGotoLocationOptions, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeDeclarationCommand", getter = alternativeDeclarationCommand)]
    pub fn alternative_declaration_command(this: &IGotoLocationOptions) -> Option<String>;
    /// Set the `alternativeDeclarationCommand` property.
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeDeclarationCommand", setter = alternativeDeclarationCommand)]
    pub fn set_alternative_declaration_command(this: &IGotoLocationOptions, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeImplementationCommand", getter = alternativeImplementationCommand)]
    pub fn alternative_implementation_command(this: &IGotoLocationOptions) -> Option<String>;
    /// Set the `alternativeImplementationCommand` property.
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeImplementationCommand", setter = alternativeImplementationCommand)]
    pub fn set_alternative_implementation_command(this: &IGotoLocationOptions, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeReferenceCommand", getter = alternativeReferenceCommand)]
    pub fn alternative_reference_command(this: &IGotoLocationOptions) -> Option<String>;
    /// Set the `alternativeReferenceCommand` property.
    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeReferenceCommand", setter = alternativeReferenceCommand)]
    pub fn set_alternative_reference_command(this: &IGotoLocationOptions, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for editor hover
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorHoverOptions;
    /// Enable the hover.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "enabled", getter = enabled)]
    pub fn enabled(this: &IEditorHoverOptions) -> Option<bool>;
    /// Set the `enabled` property.
    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "enabled", setter = enabled)]
    pub fn set_enabled(this: &IEditorHoverOptions, val: Option<bool>);
    /// Delay for showing the hover.
    /// Defaults to 300.
    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "delay", getter = delay)]
    pub fn delay(this: &IEditorHoverOptions) -> Option<f64>;
    /// Set the `delay` property.
    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "delay", setter = delay)]
    pub fn set_delay(this: &IEditorHoverOptions, val: Option<f64>);
    /// Is the hover sticky such that it can be clicked and its contents
    /// selected? Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "sticky", getter = sticky)]
    pub fn sticky(this: &IEditorHoverOptions) -> Option<bool>;
    /// Set the `sticky` property.
    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "sticky", setter = sticky)]
    pub fn set_sticky(this: &IEditorHoverOptions, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    /// A description for the overview ruler position.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type OverviewRulerPosition;
    /// Width of the overview ruler
    #[wasm_bindgen(method, js_class = "OverviewRulerPosition", js_name = "width", getter = width)]
    pub fn width(this: &OverviewRulerPosition) -> f64;
    /// Height of the overview ruler
    #[wasm_bindgen(method, js_class = "OverviewRulerPosition", js_name = "height", getter = height)]
    pub fn height(this: &OverviewRulerPosition) -> f64;
    /// Top position for the overview ruler
    #[wasm_bindgen(method, js_class = "OverviewRulerPosition", js_name = "top", getter = top)]
    pub fn top(this: &OverviewRulerPosition) -> f64;
    /// Right position for the overview ruler
    #[wasm_bindgen(method, js_class = "OverviewRulerPosition", js_name = "right", getter = right)]
    pub fn right(this: &OverviewRulerPosition) -> f64;
}

#[wasm_bindgen]
extern "C" {
    /// The internal layout details of the editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type EditorLayoutInfo;
    /// Full editor width.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "width", getter = width)]
    pub fn width(this: &EditorLayoutInfo) -> f64;
    /// Full editor height.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "height", getter = height)]
    pub fn height(this: &EditorLayoutInfo) -> f64;
    /// Left position for the glyph margin.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "glyphMarginLeft", getter = glyphMarginLeft)]
    pub fn glyph_margin_left(this: &EditorLayoutInfo) -> f64;
    /// The width of the glyph margin.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "glyphMarginWidth", getter = glyphMarginWidth)]
    pub fn glyph_margin_width(this: &EditorLayoutInfo) -> f64;
    /// Left position for the line numbers.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "lineNumbersLeft", getter = lineNumbersLeft)]
    pub fn line_numbers_left(this: &EditorLayoutInfo) -> f64;
    /// The width of the line numbers.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "lineNumbersWidth", getter = lineNumbersWidth)]
    pub fn line_numbers_width(this: &EditorLayoutInfo) -> f64;
    /// Left position for the line decorations.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "decorationsLeft", getter = decorationsLeft)]
    pub fn decorations_left(this: &EditorLayoutInfo) -> f64;
    /// The width of the line decorations.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "decorationsWidth", getter = decorationsWidth)]
    pub fn decorations_width(this: &EditorLayoutInfo) -> f64;
    /// Left position for the content (actual text)
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "contentLeft", getter = contentLeft)]
    pub fn content_left(this: &EditorLayoutInfo) -> f64;
    /// The width of the content (actual text)
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "contentWidth", getter = contentWidth)]
    pub fn content_width(this: &EditorLayoutInfo) -> f64;
    /// The position for the minimap
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "minimapLeft", getter = minimapLeft)]
    pub fn minimap_left(this: &EditorLayoutInfo) -> f64;
    /// The width of the minimap
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "minimapWidth", getter = minimapWidth)]
    pub fn minimap_width(this: &EditorLayoutInfo) -> f64;
    /// Minimap render type
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "renderMinimap", getter = renderMinimap)]
    pub fn render_minimap(this: &EditorLayoutInfo) -> RenderMinimap;
    /// The number of columns (of typical characters) fitting on a viewport
    /// line.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "viewportColumn", getter = viewportColumn)]
    pub fn viewport_column(this: &EditorLayoutInfo) -> f64;
    /// The width of the vertical scrollbar.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "verticalScrollbarWidth", getter = verticalScrollbarWidth)]
    pub fn vertical_scrollbar_width(this: &EditorLayoutInfo) -> f64;
    /// The height of the horizontal scrollbar.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "horizontalScrollbarHeight", getter = horizontalScrollbarHeight)]
    pub fn horizontal_scrollbar_height(this: &EditorLayoutInfo) -> f64;
    /// The position of the overview ruler.
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "overviewRuler", getter = overviewRuler)]
    pub fn overview_ruler(this: &EditorLayoutInfo) -> OverviewRulerPosition;
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for editor lightbulb
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorLightbulbOptions;
    /// Enable the lightbulb code action.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorLightbulbOptions", js_name = "enabled", getter = enabled)]
    pub fn enabled(this: &IEditorLightbulbOptions) -> Option<bool>;
    /// Set the `enabled` property.
    #[wasm_bindgen(method, js_class = "IEditorLightbulbOptions", js_name = "enabled", setter = enabled)]
    pub fn set_enabled(this: &IEditorLightbulbOptions, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for editor minimap
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorMinimapOptions;
    /// Enable the rendering of the minimap.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "enabled", getter = enabled)]
    pub fn enabled(this: &IEditorMinimapOptions) -> Option<bool>;
    /// Set the `enabled` property.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "enabled", setter = enabled)]
    pub fn set_enabled(this: &IEditorMinimapOptions, val: Option<bool>);
    /// Control the side of the minimap in editor.
    /// Defaults to 'right'.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "side", getter = side)]
    pub fn side(this: &IEditorMinimapOptions) -> Option<IEditorMinimapOptionsSide>;
    /// Set the `side` property.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "side", setter = side)]
    pub fn set_side(this: &IEditorMinimapOptions, val: Option<IEditorMinimapOptionsSide>);
    /// Control the rendering of the minimap slider.
    /// Defaults to 'mouseover'.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "showSlider", getter = showSlider)]
    pub fn show_slider(this: &IEditorMinimapOptions) -> Option<IEditorMinimapOptionsShowSlider>;
    /// Set the `showSlider` property.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "showSlider", setter = showSlider)]
    pub fn set_show_slider(
        this: &IEditorMinimapOptions,
        val: Option<IEditorMinimapOptionsShowSlider>,
    );
    /// Render the actual text on a line (as opposed to color blocks).
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "renderCharacters", getter = renderCharacters)]
    pub fn render_characters(this: &IEditorMinimapOptions) -> Option<bool>;
    /// Set the `renderCharacters` property.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "renderCharacters", setter = renderCharacters)]
    pub fn set_render_characters(this: &IEditorMinimapOptions, val: Option<bool>);
    /// Limit the width of the minimap to render at most a certain number of
    /// columns. Defaults to 120.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "maxColumn", getter = maxColumn)]
    pub fn max_column(this: &IEditorMinimapOptions) -> Option<f64>;
    /// Set the `maxColumn` property.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "maxColumn", setter = maxColumn)]
    pub fn set_max_column(this: &IEditorMinimapOptions, val: Option<f64>);
    /// Relative size of the font in the minimap. Defaults to 1.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "scale", getter = scale)]
    pub fn scale(this: &IEditorMinimapOptions) -> Option<f64>;
    /// Set the `scale` property.
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "scale", setter = scale)]
    pub fn set_scale(this: &IEditorMinimapOptions, val: Option<f64>);
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for parameter hints
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorParameterHintOptions;
    /// Enable parameter hints.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorParameterHintOptions", js_name = "enabled", getter = enabled)]
    pub fn enabled(this: &IEditorParameterHintOptions) -> Option<bool>;
    /// Set the `enabled` property.
    #[wasm_bindgen(method, js_class = "IEditorParameterHintOptions", js_name = "enabled", setter = enabled)]
    pub fn set_enabled(this: &IEditorParameterHintOptions, val: Option<bool>);
    /// Enable cycling of parameter hints.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorParameterHintOptions", js_name = "cycle", getter = cycle)]
    pub fn cycle(this: &IEditorParameterHintOptions) -> Option<bool>;
    /// Set the `cycle` property.
    #[wasm_bindgen(method, js_class = "IEditorParameterHintOptions", js_name = "cycle", setter = cycle)]
    pub fn set_cycle(this: &IEditorParameterHintOptions, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for quick suggestions
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IQuickSuggestionsOptions;
    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "other", getter = other)]
    pub fn other(this: &IQuickSuggestionsOptions) -> bool;
    /// Set the `other` property.
    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "other", setter = other)]
    pub fn set_other(this: &IQuickSuggestionsOptions, val: bool);
    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "comments", getter = comments)]
    pub fn comments(this: &IQuickSuggestionsOptions) -> bool;
    /// Set the `comments` property.
    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "comments", setter = comments)]
    pub fn set_comments(this: &IQuickSuggestionsOptions, val: bool);
    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "strings", getter = strings)]
    pub fn strings(this: &IQuickSuggestionsOptions) -> bool;
    /// Set the `strings` property.
    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "strings", setter = strings)]
    pub fn set_strings(this: &IQuickSuggestionsOptions, val: bool);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type InternalEditorRenderLineNumbersOptions;
    #[wasm_bindgen(method, js_class = "InternalEditorRenderLineNumbersOptions", js_name = "renderType", getter = renderType)]
    pub fn render_type(this: &InternalEditorRenderLineNumbersOptions) -> RenderLineNumbersType;
    /// Type: `((lineNumber: number) => string)`
    #[wasm_bindgen(method, js_class = "InternalEditorRenderLineNumbersOptions", js_name = "renderFn", getter = renderFn)]
    pub fn render_fn(this: &InternalEditorRenderLineNumbersOptions) -> Option<Function>;
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for editor scrollbars
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorScrollbarOptions;
    /// The size of arrows (if displayed).
    /// Defaults to 11.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "arrowSize", getter = arrowSize)]
    pub fn arrow_size(this: &IEditorScrollbarOptions) -> Option<f64>;
    /// Set the `arrowSize` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "arrowSize", setter = arrowSize)]
    pub fn set_arrow_size(this: &IEditorScrollbarOptions, val: Option<f64>);
    /// Render vertical scrollbar.
    /// Defaults to 'auto'.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "vertical", getter = vertical)]
    pub fn vertical(this: &IEditorScrollbarOptions) -> Option<IEditorScrollbarOptionsVertical>;
    /// Set the `vertical` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "vertical", setter = vertical)]
    pub fn set_vertical(
        this: &IEditorScrollbarOptions,
        val: Option<IEditorScrollbarOptionsVertical>,
    );
    /// Render horizontal scrollbar.
    /// Defaults to 'auto'.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontal", getter = horizontal)]
    pub fn horizontal(this: &IEditorScrollbarOptions) -> Option<IEditorScrollbarOptionsHorizontal>;
    /// Set the `horizontal` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontal", setter = horizontal)]
    pub fn set_horizontal(
        this: &IEditorScrollbarOptions,
        val: Option<IEditorScrollbarOptionsHorizontal>,
    );
    /// Cast horizontal and vertical shadows when the content is scrolled.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "useShadows", getter = useShadows)]
    pub fn use_shadows(this: &IEditorScrollbarOptions) -> Option<bool>;
    /// Set the `useShadows` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "useShadows", setter = useShadows)]
    pub fn set_use_shadows(this: &IEditorScrollbarOptions, val: Option<bool>);
    /// Render arrows at the top and bottom of the vertical scrollbar.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalHasArrows", getter = verticalHasArrows)]
    pub fn vertical_has_arrows(this: &IEditorScrollbarOptions) -> Option<bool>;
    /// Set the `verticalHasArrows` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalHasArrows", setter = verticalHasArrows)]
    pub fn set_vertical_has_arrows(this: &IEditorScrollbarOptions, val: Option<bool>);
    /// Render arrows at the left and right of the horizontal scrollbar.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalHasArrows", getter = horizontalHasArrows)]
    pub fn horizontal_has_arrows(this: &IEditorScrollbarOptions) -> Option<bool>;
    /// Set the `horizontalHasArrows` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalHasArrows", setter = horizontalHasArrows)]
    pub fn set_horizontal_has_arrows(this: &IEditorScrollbarOptions, val: Option<bool>);
    /// Listen to mouse wheel events and react to them by scrolling.
    /// Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "handleMouseWheel", getter = handleMouseWheel)]
    pub fn handle_mouse_wheel(this: &IEditorScrollbarOptions) -> Option<bool>;
    /// Set the `handleMouseWheel` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "handleMouseWheel", setter = handleMouseWheel)]
    pub fn set_handle_mouse_wheel(this: &IEditorScrollbarOptions, val: Option<bool>);
    /// Always consume mouse wheel events (always call preventDefault() and
    /// stopPropagation() on the browser events). Defaults to true.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "alwaysConsumeMouseWheel", getter = alwaysConsumeMouseWheel)]
    pub fn always_consume_mouse_wheel(this: &IEditorScrollbarOptions) -> Option<bool>;
    /// Set the `alwaysConsumeMouseWheel` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "alwaysConsumeMouseWheel", setter = alwaysConsumeMouseWheel)]
    pub fn set_always_consume_mouse_wheel(this: &IEditorScrollbarOptions, val: Option<bool>);
    /// Height in pixels for the horizontal scrollbar.
    /// Defaults to 10 (px).
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalScrollbarSize", getter = horizontalScrollbarSize)]
    pub fn horizontal_scrollbar_size(this: &IEditorScrollbarOptions) -> Option<f64>;
    /// Set the `horizontalScrollbarSize` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalScrollbarSize", setter = horizontalScrollbarSize)]
    pub fn set_horizontal_scrollbar_size(this: &IEditorScrollbarOptions, val: Option<f64>);
    /// Width in pixels for the vertical scrollbar.
    /// Defaults to 10 (px).
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalScrollbarSize", getter = verticalScrollbarSize)]
    pub fn vertical_scrollbar_size(this: &IEditorScrollbarOptions) -> Option<f64>;
    /// Set the `verticalScrollbarSize` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalScrollbarSize", setter = verticalScrollbarSize)]
    pub fn set_vertical_scrollbar_size(this: &IEditorScrollbarOptions, val: Option<f64>);
    /// Width in pixels for the vertical slider.
    /// Defaults to `verticalScrollbarSize`.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalSliderSize", getter = verticalSliderSize)]
    pub fn vertical_slider_size(this: &IEditorScrollbarOptions) -> Option<f64>;
    /// Set the `verticalSliderSize` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalSliderSize", setter = verticalSliderSize)]
    pub fn set_vertical_slider_size(this: &IEditorScrollbarOptions, val: Option<f64>);
    /// Height in pixels for the horizontal slider.
    /// Defaults to `horizontalScrollbarSize`.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalSliderSize", getter = horizontalSliderSize)]
    pub fn horizontal_slider_size(this: &IEditorScrollbarOptions) -> Option<f64>;
    /// Set the `horizontalSliderSize` property.
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalSliderSize", setter = horizontalSliderSize)]
    pub fn set_horizontal_slider_size(this: &IEditorScrollbarOptions, val: Option<f64>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type InternalEditorScrollbarOptions;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "arrowSize", getter = arrowSize)]
    pub fn arrow_size(this: &InternalEditorScrollbarOptions) -> f64;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "vertical", getter = vertical)]
    pub fn vertical(this: &InternalEditorScrollbarOptions) -> ScrollbarVisibility;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "horizontal", getter = horizontal)]
    pub fn horizontal(this: &InternalEditorScrollbarOptions) -> ScrollbarVisibility;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "useShadows", getter = useShadows)]
    pub fn use_shadows(this: &InternalEditorScrollbarOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "verticalHasArrows", getter = verticalHasArrows)]
    pub fn vertical_has_arrows(this: &InternalEditorScrollbarOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "horizontalHasArrows", getter = horizontalHasArrows)]
    pub fn horizontal_has_arrows(this: &InternalEditorScrollbarOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "handleMouseWheel", getter = handleMouseWheel)]
    pub fn handle_mouse_wheel(this: &InternalEditorScrollbarOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "alwaysConsumeMouseWheel", getter = alwaysConsumeMouseWheel)]
    pub fn always_consume_mouse_wheel(this: &InternalEditorScrollbarOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "horizontalScrollbarSize", getter = horizontalScrollbarSize)]
    pub fn horizontal_scrollbar_size(this: &InternalEditorScrollbarOptions) -> f64;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "horizontalSliderSize", getter = horizontalSliderSize)]
    pub fn horizontal_slider_size(this: &InternalEditorScrollbarOptions) -> f64;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "verticalScrollbarSize", getter = verticalScrollbarSize)]
    pub fn vertical_scrollbar_size(this: &InternalEditorScrollbarOptions) -> f64;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "verticalSliderSize", getter = verticalSliderSize)]
    pub fn vertical_slider_size(this: &InternalEditorScrollbarOptions) -> f64;
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for editor suggest widget
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ISuggestOptions;
    /// Overwrite word ends on accept. Default to false.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "insertMode", getter = insertMode)]
    pub fn insert_mode(this: &ISuggestOptions) -> Option<ISuggestOptionsInsertMode>;
    /// Set the `insertMode` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "insertMode", setter = insertMode)]
    pub fn set_insert_mode(this: &ISuggestOptions, val: Option<ISuggestOptionsInsertMode>);
    /// Show a highlight when suggestion replaces or keep text after the cursor.
    /// Defaults to false.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "insertHighlight", getter = insertHighlight)]
    pub fn insert_highlight(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `insertHighlight` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "insertHighlight", setter = insertHighlight)]
    pub fn set_insert_highlight(this: &ISuggestOptions, val: Option<bool>);
    /// Enable graceful matching. Defaults to true.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "filterGraceful", getter = filterGraceful)]
    pub fn filter_graceful(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `filterGraceful` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "filterGraceful", setter = filterGraceful)]
    pub fn set_filter_graceful(this: &ISuggestOptions, val: Option<bool>);
    /// Prevent quick suggestions when a snippet is active. Defaults to true.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "snippetsPreventQuickSuggestions", getter = snippetsPreventQuickSuggestions)]
    pub fn snippets_prevent_quick_suggestions(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `snippetsPreventQuickSuggestions` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "snippetsPreventQuickSuggestions", setter = snippetsPreventQuickSuggestions)]
    pub fn set_snippets_prevent_quick_suggestions(this: &ISuggestOptions, val: Option<bool>);
    /// Favours words that appear close to the cursor.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "localityBonus", getter = localityBonus)]
    pub fn locality_bonus(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `localityBonus` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "localityBonus", setter = localityBonus)]
    pub fn set_locality_bonus(this: &ISuggestOptions, val: Option<bool>);
    /// Enable using global storage for remembering suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "shareSuggestSelections", getter = shareSuggestSelections)]
    pub fn share_suggest_selections(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `shareSuggestSelections` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "shareSuggestSelections", setter = shareSuggestSelections)]
    pub fn set_share_suggest_selections(this: &ISuggestOptions, val: Option<bool>);
    /// Enable or disable icons in suggestions. Defaults to true.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showIcons", getter = showIcons)]
    pub fn show_icons(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showIcons` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showIcons", setter = showIcons)]
    pub fn set_show_icons(this: &ISuggestOptions, val: Option<bool>);
    /// Max suggestions to show in suggestions. Defaults to 12.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "maxVisibleSuggestions", getter = maxVisibleSuggestions)]
    pub fn max_visible_suggestions(this: &ISuggestOptions) -> Option<f64>;
    /// Set the `maxVisibleSuggestions` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "maxVisibleSuggestions", setter = maxVisibleSuggestions)]
    pub fn set_max_visible_suggestions(this: &ISuggestOptions, val: Option<f64>);
    /// Show method-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showMethods", getter = showMethods)]
    pub fn show_methods(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showMethods` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showMethods", setter = showMethods)]
    pub fn set_show_methods(this: &ISuggestOptions, val: Option<bool>);
    /// Show function-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFunctions", getter = showFunctions)]
    pub fn show_functions(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showFunctions` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFunctions", setter = showFunctions)]
    pub fn set_show_functions(this: &ISuggestOptions, val: Option<bool>);
    /// Show constructor-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showConstructors", getter = showConstructors)]
    pub fn show_constructors(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showConstructors` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showConstructors", setter = showConstructors)]
    pub fn set_show_constructors(this: &ISuggestOptions, val: Option<bool>);
    /// Show field-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFields", getter = showFields)]
    pub fn show_fields(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showFields` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFields", setter = showFields)]
    pub fn set_show_fields(this: &ISuggestOptions, val: Option<bool>);
    /// Show variable-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showVariables", getter = showVariables)]
    pub fn show_variables(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showVariables` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showVariables", setter = showVariables)]
    pub fn set_show_variables(this: &ISuggestOptions, val: Option<bool>);
    /// Show class-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showClasses", getter = showClasses)]
    pub fn show_classes(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showClasses` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showClasses", setter = showClasses)]
    pub fn set_show_classes(this: &ISuggestOptions, val: Option<bool>);
    /// Show struct-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showStructs", getter = showStructs)]
    pub fn show_structs(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showStructs` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showStructs", setter = showStructs)]
    pub fn set_show_structs(this: &ISuggestOptions, val: Option<bool>);
    /// Show interface-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showInterfaces", getter = showInterfaces)]
    pub fn show_interfaces(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showInterfaces` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showInterfaces", setter = showInterfaces)]
    pub fn set_show_interfaces(this: &ISuggestOptions, val: Option<bool>);
    /// Show module-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showModules", getter = showModules)]
    pub fn show_modules(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showModules` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showModules", setter = showModules)]
    pub fn set_show_modules(this: &ISuggestOptions, val: Option<bool>);
    /// Show property-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showProperties", getter = showProperties)]
    pub fn show_properties(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showProperties` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showProperties", setter = showProperties)]
    pub fn set_show_properties(this: &ISuggestOptions, val: Option<bool>);
    /// Show event-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEvents", getter = showEvents)]
    pub fn show_events(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showEvents` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEvents", setter = showEvents)]
    pub fn set_show_events(this: &ISuggestOptions, val: Option<bool>);
    /// Show operator-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showOperators", getter = showOperators)]
    pub fn show_operators(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showOperators` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showOperators", setter = showOperators)]
    pub fn set_show_operators(this: &ISuggestOptions, val: Option<bool>);
    /// Show unit-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showUnits", getter = showUnits)]
    pub fn show_units(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showUnits` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showUnits", setter = showUnits)]
    pub fn set_show_units(this: &ISuggestOptions, val: Option<bool>);
    /// Show value-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showValues", getter = showValues)]
    pub fn show_values(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showValues` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showValues", setter = showValues)]
    pub fn set_show_values(this: &ISuggestOptions, val: Option<bool>);
    /// Show constant-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showConstants", getter = showConstants)]
    pub fn show_constants(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showConstants` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showConstants", setter = showConstants)]
    pub fn set_show_constants(this: &ISuggestOptions, val: Option<bool>);
    /// Show enum-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEnums", getter = showEnums)]
    pub fn show_enums(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showEnums` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEnums", setter = showEnums)]
    pub fn set_show_enums(this: &ISuggestOptions, val: Option<bool>);
    /// Show enumMember-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEnumMembers", getter = showEnumMembers)]
    pub fn show_enum_members(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showEnumMembers` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEnumMembers", setter = showEnumMembers)]
    pub fn set_show_enum_members(this: &ISuggestOptions, val: Option<bool>);
    /// Show keyword-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showKeywords", getter = showKeywords)]
    pub fn show_keywords(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showKeywords` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showKeywords", setter = showKeywords)]
    pub fn set_show_keywords(this: &ISuggestOptions, val: Option<bool>);
    /// Show text-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showWords", getter = showWords)]
    pub fn show_words(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showWords` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showWords", setter = showWords)]
    pub fn set_show_words(this: &ISuggestOptions, val: Option<bool>);
    /// Show color-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showColors", getter = showColors)]
    pub fn show_colors(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showColors` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showColors", setter = showColors)]
    pub fn set_show_colors(this: &ISuggestOptions, val: Option<bool>);
    /// Show file-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFiles", getter = showFiles)]
    pub fn show_files(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showFiles` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFiles", setter = showFiles)]
    pub fn set_show_files(this: &ISuggestOptions, val: Option<bool>);
    /// Show reference-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showReferences", getter = showReferences)]
    pub fn show_references(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showReferences` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showReferences", setter = showReferences)]
    pub fn set_show_references(this: &ISuggestOptions, val: Option<bool>);
    /// Show folder-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFolders", getter = showFolders)]
    pub fn show_folders(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showFolders` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFolders", setter = showFolders)]
    pub fn set_show_folders(this: &ISuggestOptions, val: Option<bool>);
    /// Show typeParameter-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showTypeParameters", getter = showTypeParameters)]
    pub fn show_type_parameters(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showTypeParameters` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showTypeParameters", setter = showTypeParameters)]
    pub fn set_show_type_parameters(this: &ISuggestOptions, val: Option<bool>);
    /// Show snippet-suggestions.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showSnippets", getter = showSnippets)]
    pub fn show_snippets(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `showSnippets` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showSnippets", setter = showSnippets)]
    pub fn set_show_snippets(this: &ISuggestOptions, val: Option<bool>);
    /// Controls the visibility of the status bar at the bottom of the suggest
    /// widget.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "hideStatusBar", getter = hideStatusBar)]
    pub fn hide_status_bar(this: &ISuggestOptions) -> Option<bool>;
    /// Set the `hideStatusBar` property.
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "hideStatusBar", setter = hideStatusBar)]
    pub fn set_hide_status_bar(this: &ISuggestOptions, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type EditorWrappingInfo;
    #[wasm_bindgen(method, js_class = "EditorWrappingInfo", js_name = "isDominatedByLongLines", getter = isDominatedByLongLines)]
    pub fn is_dominated_by_long_lines(this: &EditorWrappingInfo) -> bool;
    #[wasm_bindgen(method, js_class = "EditorWrappingInfo", js_name = "isWordWrapMinified", getter = isWordWrapMinified)]
    pub fn is_word_wrap_minified(this: &EditorWrappingInfo) -> bool;
    #[wasm_bindgen(method, js_class = "EditorWrappingInfo", js_name = "isViewportWrapping", getter = isViewportWrapping)]
    pub fn is_viewport_wrapping(this: &EditorWrappingInfo) -> bool;
    #[wasm_bindgen(method, js_class = "EditorWrappingInfo", js_name = "wrappingColumn", getter = wrappingColumn)]
    pub fn wrapping_column(this: &EditorWrappingInfo) -> f64;
}

#[wasm_bindgen]
extern "C" {
    /// A view zone is a full horizontal rectangle that 'pushes' text down.
    /// The editor reserves space for view zones when rendering.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IViewZone;
    /// The line number after which this zone should appear.
    /// Use 0 to place a view zone before the first line number.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "afterLineNumber", getter = afterLineNumber)]
    pub fn after_line_number(this: &IViewZone) -> f64;
    /// Set the `afterLineNumber` property.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "afterLineNumber", setter = afterLineNumber)]
    pub fn set_after_line_number(this: &IViewZone, val: f64);
    /// The column after which this zone should appear.
    /// If not set, the maxLineColumn of `afterLineNumber` will be used.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "afterColumn", getter = afterColumn)]
    pub fn after_column(this: &IViewZone) -> Option<f64>;
    /// Set the `afterColumn` property.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "afterColumn", setter = afterColumn)]
    pub fn set_after_column(this: &IViewZone, val: Option<f64>);
    /// Suppress mouse down events.
    /// If set, the editor will attach a mouse down listener to the view zone
    /// and .preventDefault on it. Defaults to false
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "suppressMouseDown", getter = suppressMouseDown)]
    pub fn suppress_mouse_down(this: &IViewZone) -> Option<bool>;
    /// Set the `suppressMouseDown` property.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "suppressMouseDown", setter = suppressMouseDown)]
    pub fn set_suppress_mouse_down(this: &IViewZone, val: Option<bool>);
    /// The height in lines of the view zone.
    /// If specified, `heightInPx` will be used instead of this.
    /// If neither `heightInPx` nor `heightInLines` is specified, a default of
    /// `heightInLines` = 1 will be chosen.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "heightInLines", getter = heightInLines)]
    pub fn height_in_lines(this: &IViewZone) -> Option<f64>;
    /// Set the `heightInLines` property.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "heightInLines", setter = heightInLines)]
    pub fn set_height_in_lines(this: &IViewZone, val: Option<f64>);
    /// The height in px of the view zone.
    /// If this is set, the editor will give preference to it rather than
    /// `heightInLines` above. If neither `heightInPx` nor `heightInLines`
    /// is specified, a default of `heightInLines` = 1 will be chosen.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "heightInPx", getter = heightInPx)]
    pub fn height_in_px(this: &IViewZone) -> Option<f64>;
    /// Set the `heightInPx` property.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "heightInPx", setter = heightInPx)]
    pub fn set_height_in_px(this: &IViewZone, val: Option<f64>);
    /// The minimum width in px of the view zone.
    /// If this is set, the editor will ensure that the scroll width is >= than
    /// this value.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "minWidthInPx", getter = minWidthInPx)]
    pub fn min_width_in_px(this: &IViewZone) -> Option<f64>;
    /// Set the `minWidthInPx` property.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "minWidthInPx", setter = minWidthInPx)]
    pub fn set_min_width_in_px(this: &IViewZone, val: Option<f64>);
    /// The dom node of the view zone
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "domNode", getter = domNode)]
    pub fn dom_node(this: &IViewZone) -> HtmlElement;
    /// Set the `domNode` property.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "domNode", setter = domNode)]
    pub fn set_dom_node(this: &IViewZone, val: &HtmlElement);
    /// An optional dom node for the view zone that will be placed in the margin
    /// area.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "marginDomNode", getter = marginDomNode)]
    pub fn margin_dom_node(this: &IViewZone) -> Option<HtmlElement>;
    /// Set the `marginDomNode` property.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "marginDomNode", setter = marginDomNode)]
    pub fn set_margin_dom_node(this: &IViewZone, val: Option<&HtmlElement>);
    /// Callback which gives the relative top of the view zone as it appears
    /// (taking scrolling into account).
    ///
    /// Type: `(top: number) => void`
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "onDomNodeTop", getter = onDomNodeTop)]
    pub fn on_dom_node_top(this: &IViewZone) -> Option<Function>;
    /// Set the `onDomNodeTop` property.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "onDomNodeTop", setter = onDomNodeTop)]
    pub fn set_on_dom_node_top(this: &IViewZone, val: Option<&Function>);
    /// Callback which gives the height in pixels of the view zone.
    ///
    /// Type: `(height: number) => void`
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "onComputedHeight", getter = onComputedHeight)]
    pub fn on_computed_height(this: &IViewZone) -> Option<Function>;
    /// Set the `onComputedHeight` property.
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "onComputedHeight", setter = onComputedHeight)]
    pub fn set_on_computed_height(this: &IViewZone, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    /// An accessor that allows for zones to be added or removed.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IViewZoneChangeAccessor;
    /// Create a new view zone.
    /// @param zone Zone to create
    /// @return A unique identifier to the view zone.
    #[wasm_bindgen(method, js_class = "IViewZoneChangeAccessor", js_name = "addZone")]
    pub fn add_zone(this: &IViewZoneChangeAccessor, zone: &IViewZone) -> String;
    /// Remove a zone
    /// @param id A unique identifier to the view zone, as returned by the
    /// `addZone` call.
    #[wasm_bindgen(method, js_class = "IViewZoneChangeAccessor", js_name = "removeZone")]
    pub fn remove_zone(this: &IViewZoneChangeAccessor, id: &str);
    /// Change a zone's position.
    /// The editor will rescan the `afterLineNumber` and `afterColumn`
    /// properties of a view zone.
    #[wasm_bindgen(method, js_class = "IViewZoneChangeAccessor", js_name = "layoutZone")]
    pub fn layout_zone(this: &IViewZoneChangeAccessor, id: &str);
}

#[wasm_bindgen]
extern "C" {
    /// A position for rendering content widgets.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IContentWidgetPosition;
    /// Desired position for the content widget.
    /// `preference` will also affect the placement.
    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "position", getter = position)]
    pub fn position(this: &IContentWidgetPosition) -> Option<IPosition>;
    /// Set the `position` property.
    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "position", setter = position)]
    pub fn set_position(this: &IContentWidgetPosition, val: Option<&IPosition>);
    /// Optionally, a range can be provided to further
    /// define the position of the content widget.
    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "range", getter = range)]
    pub fn range(this: &IContentWidgetPosition) -> Option<IRange>;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "range", setter = range)]
    pub fn set_range(this: &IContentWidgetPosition, val: Option<&IRange>);
    /// Placement preference for position, in order of preference.
    ///
    /// Type: `ContentWidgetPositionPreference[]`
    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "preference", getter = preference)]
    pub fn preference(this: &IContentWidgetPosition) -> Array;
    /// Set the `preference` property.
    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "preference", setter = preference)]
    pub fn set_preference(this: &IContentWidgetPosition, val: &Array);
}

#[wasm_bindgen]
extern "C" {
    /// A content widget renders inline with the text and can be easily placed
    /// 'near' an editor position.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IContentWidget;
    /// Render this content widget in a location where it could overflow the
    /// editor's view dom node.
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "allowEditorOverflow", getter = allowEditorOverflow)]
    pub fn allow_editor_overflow(this: &IContentWidget) -> Option<bool>;
    /// Set the `allowEditorOverflow` property.
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "allowEditorOverflow", setter = allowEditorOverflow)]
    pub fn set_allow_editor_overflow(this: &IContentWidget, val: Option<bool>);
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "suppressMouseDown", getter = suppressMouseDown)]
    pub fn suppress_mouse_down(this: &IContentWidget) -> Option<bool>;
    /// Set the `suppressMouseDown` property.
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "suppressMouseDown", setter = suppressMouseDown)]
    pub fn set_suppress_mouse_down(this: &IContentWidget, val: Option<bool>);
    /// Get a unique identifier of the content widget.
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "getId")]
    pub fn get_id(this: &IContentWidget) -> String;
    /// Get the dom node of the content widget.
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "getDomNode")]
    pub fn get_dom_node(this: &IContentWidget) -> HtmlElement;
    /// Get the placement of the content widget.
    /// If null is returned, the content widget will be placed off screen.
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "getPosition")]
    pub fn get_position(this: &IContentWidget) -> Option<IContentWidgetPosition>;
}

#[wasm_bindgen]
extern "C" {
    /// A position for rendering overlay widgets.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IOverlayWidgetPosition;
    /// The position preference for the overlay widget.
    #[wasm_bindgen(method, js_class = "IOverlayWidgetPosition", js_name = "preference", getter = preference)]
    pub fn preference(this: &IOverlayWidgetPosition) -> Option<OverlayWidgetPositionPreference>;
    /// Set the `preference` property.
    #[wasm_bindgen(method, js_class = "IOverlayWidgetPosition", js_name = "preference", setter = preference)]
    pub fn set_preference(
        this: &IOverlayWidgetPosition,
        val: Option<OverlayWidgetPositionPreference>,
    );
}

#[wasm_bindgen]
extern "C" {
    /// An overlay widgets renders on top of the text.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IOverlayWidget;
    /// Get a unique identifier of the overlay widget.
    #[wasm_bindgen(method, js_class = "IOverlayWidget", js_name = "getId")]
    pub fn get_id(this: &IOverlayWidget) -> String;
    /// Get the dom node of the overlay widget.
    #[wasm_bindgen(method, js_class = "IOverlayWidget", js_name = "getDomNode")]
    pub fn get_dom_node(this: &IOverlayWidget) -> HtmlElement;
    /// Get the placement of the overlay widget.
    /// If null is returned, the overlay widget is responsible to place itself.
    #[wasm_bindgen(method, js_class = "IOverlayWidget", js_name = "getPosition")]
    pub fn get_position(this: &IOverlayWidget) -> Option<IOverlayWidgetPosition>;
}

#[wasm_bindgen]
extern "C" {
    /// Target hit with the mouse in the editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IMouseTarget;
    /// The target element
    #[wasm_bindgen(method, js_class = "IMouseTarget", js_name = "element", getter = element)]
    pub fn element(this: &IMouseTarget) -> Option<HtmlElement>;
    /// The target type
    #[wasm_bindgen(method, js_class = "IMouseTarget", js_name = "type", getter = type)]
    pub fn type_(this: &IMouseTarget) -> MouseTargetType;
    /// The 'approximate' editor position
    #[wasm_bindgen(method, js_class = "IMouseTarget", js_name = "position", getter = position)]
    pub fn position(this: &IMouseTarget) -> Option<Position>;
    /// Desired mouse column (e.g. when position.column gets clamped to text
    /// length -- clicking after text on a line).
    #[wasm_bindgen(method, js_class = "IMouseTarget", js_name = "mouseColumn", getter = mouseColumn)]
    pub fn mouse_column(this: &IMouseTarget) -> f64;
    /// The 'approximate' editor range
    #[wasm_bindgen(method, js_class = "IMouseTarget", js_name = "range", getter = range)]
    pub fn range(this: &IMouseTarget) -> Option<Range>;
    /// Some extra detail.
    #[wasm_bindgen(method, js_class = "IMouseTarget", js_name = "detail", getter = detail)]
    pub fn detail(this: &IMouseTarget) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// A mouse event originating from the editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorMouseEvent;
    #[wasm_bindgen(method, js_class = "IEditorMouseEvent", js_name = "event", getter = event)]
    pub fn event(this: &IEditorMouseEvent) -> IMouseEvent;
    #[wasm_bindgen(method, js_class = "IEditorMouseEvent", js_name = "target", getter = target)]
    pub fn target(this: &IEditorMouseEvent) -> IMouseTarget;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IPartialEditorMouseEvent;
    #[wasm_bindgen(method, js_class = "IPartialEditorMouseEvent", js_name = "event", getter = event)]
    pub fn event(this: &IPartialEditorMouseEvent) -> IMouseEvent;
    #[wasm_bindgen(method, js_class = "IPartialEditorMouseEvent", js_name = "target", getter = target)]
    pub fn target(this: &IPartialEditorMouseEvent) -> Option<IMouseTarget>;
}

#[wasm_bindgen]
extern "C" {
    /// A paste event originating from the editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IPasteEvent;
    #[wasm_bindgen(method, js_class = "IPasteEvent", js_name = "range", getter = range)]
    pub fn range(this: &IPasteEvent) -> Range;
    #[wasm_bindgen(method, js_class = "IPasteEvent", js_name = "mode", getter = mode)]
    pub fn mode(this: &IPasteEvent) -> Option<String>;
}

#[wasm_bindgen]
extern "C" {
    /// A rich code editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = IEditor, extends = Object)]
    pub type ICodeEditor;
    /// An event emitted when the content of the current model has changed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IModelContentChangedEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeModelContent")]
    pub fn on_did_change_model_content(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the language of the current model has changed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IModelLanguageChangedEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeModelLanguage")]
    pub fn on_did_change_model_language(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the language configuration of the current model
    /// has changed. @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: {}) => void`
    #[wasm_bindgen(
        method,
        js_class = "ICodeEditor",
        js_name = "onDidChangeModelLanguageConfiguration"
    )]
    pub fn on_did_change_model_language_configuration(
        this: &ICodeEditor,
        listener: &Function,
    ) -> IDisposable;
    /// An event emitted when the options of the current model has changed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IModelOptionsChangedEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeModelOptions")]
    pub fn on_did_change_model_options(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the configuration of the editor has changed. (e.g.
    /// `editor.updateOptions()`) @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: ConfigurationChangedEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeConfiguration")]
    pub fn on_did_change_configuration(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the cursor position has changed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: ICursorPositionChangedEvent) => void`
    #[wasm_bindgen(
        method,
        js_class = "ICodeEditor",
        js_name = "onDidChangeCursorPosition"
    )]
    pub fn on_did_change_cursor_position(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the cursor selection has changed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: ICursorSelectionChangedEvent) => void`
    #[wasm_bindgen(
        method,
        js_class = "ICodeEditor",
        js_name = "onDidChangeCursorSelection"
    )]
    pub fn on_did_change_cursor_selection(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the model of this editor has changed (e.g.
    /// `editor.setModel()`). @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IModelChangedEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeModel")]
    pub fn on_did_change_model(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the decorations of the current model have changed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: {}) => void`
    #[wasm_bindgen(
        method,
        js_class = "ICodeEditor",
        js_name = "onDidChangeModelDecorations"
    )]
    pub fn on_did_change_model_decorations(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the text inside this editor gained focus (i.e.
    /// cursor starts blinking). @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `() => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidFocusEditorText")]
    pub fn on_did_focus_editor_text(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the text inside this editor lost focus (i.e.
    /// cursor stops blinking). @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `() => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidBlurEditorText")]
    pub fn on_did_blur_editor_text(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the text inside this editor or an editor widget
    /// gained focus. @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `() => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidFocusEditorWidget")]
    pub fn on_did_focus_editor_widget(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the text inside this editor or an editor widget
    /// lost focus. @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `() => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidBlurEditorWidget")]
    pub fn on_did_blur_editor_widget(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted after composition has started.
    ///
    /// # Arguments
    ///
    /// * `listener` - `() => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidCompositionStart")]
    pub fn on_did_composition_start(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted after composition has ended.
    ///
    /// # Arguments
    ///
    /// * `listener` - `() => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidCompositionEnd")]
    pub fn on_did_composition_end(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when users paste text in the editor.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IPasteEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidPaste")]
    pub fn on_did_paste(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted on a "mouseup".
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IEditorMouseEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onMouseUp")]
    pub fn on_mouse_up(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted on a "mousedown".
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IEditorMouseEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onMouseDown")]
    pub fn on_mouse_down(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted on a "contextmenu".
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IEditorMouseEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onContextMenu")]
    pub fn on_context_menu(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted on a "mousemove".
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IEditorMouseEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onMouseMove")]
    pub fn on_mouse_move(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted on a "mouseleave".
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IPartialEditorMouseEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onMouseLeave")]
    pub fn on_mouse_leave(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted on a "keyup".
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IKeyboardEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onKeyUp")]
    pub fn on_key_up(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted on a "keydown".
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IKeyboardEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onKeyDown")]
    pub fn on_key_down(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the layout of the editor has changed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: EditorLayoutInfo) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidLayoutChange")]
    pub fn on_did_layout_change(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the content width or content height in the editor
    /// has changed. @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IContentSizeChangedEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidContentSizeChange")]
    pub fn on_did_content_size_change(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// An event emitted when the scroll in the editor has changed.
    /// @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `(e: IScrollEvent) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidScrollChange")]
    pub fn on_did_scroll_change(this: &ICodeEditor, listener: &Function) -> IDisposable;
    /// Saves current view state of the editor in a serializable object.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "saveViewState")]
    pub fn save_view_state(this: &ICodeEditor) -> Option<ICodeEditorViewState>;
    /// Restores the view state of the editor from a serializable object
    /// generated by `saveViewState`.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "restoreViewState")]
    pub fn restore_view_state(this: &ICodeEditor, state: &ICodeEditorViewState);
    /// Returns true if the text inside this editor or an editor widget has
    /// focus.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "hasWidgetFocus")]
    pub fn has_widget_focus(this: &ICodeEditor) -> bool;
    /// Get a contribution of this editor.
    /// @id Unique identifier of the contribution.
    /// @return The contribution or null if contribution not found.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getContribution")]
    pub fn get_contribution(this: &ICodeEditor, id: &str) -> JsValue;
    /// Type the getModel() of IEditor.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getModel")]
    pub fn get_model(this: &ICodeEditor) -> Option<ITextModel>;
    /// Sets the current model attached to this editor.
    /// If the previous model was created by the editor via the value key in the
    /// options literal object, it will be destroyed. Otherwise, if the
    /// previous model was set via setModel, or the model key in the options
    /// literal object, the previous model will not be destroyed.
    /// It is safe to call setModel(null) to simply detach the current model
    /// from the editor.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "setModel")]
    pub fn set_model(this: &ICodeEditor, model: Option<&ITextModel>);
    /// Gets all the editor computed options.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getOptions")]
    pub fn get_options(this: &ICodeEditor) -> IComputedEditorOptions;
    /// Gets a specific editor option.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getOption")]
    pub fn get_option(this: &ICodeEditor, id: &JsValue) -> JsValue;
    /// Returns the editor's configuration (without any validation or defaults).
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getRawOptions")]
    pub fn get_raw_options(this: &ICodeEditor) -> IEditorOptions;
    /// Get value of the current model attached to this editor.
    /// @see `ITextModel.getValue`
    ///
    /// # Arguments
    ///
    /// * `options` - `{ preserveBOM: boolean; lineEnding: string }`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getValue")]
    pub fn get_value(this: &ICodeEditor, options: Option<&Object>) -> String;
    /// Set the value of the current model attached to this editor.
    /// @see `ITextModel.setValue`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "setValue")]
    pub fn set_value(this: &ICodeEditor, new_value: &str);
    /// Get the width of the editor's content.
    /// This is information that is "erased" when computing `scrollWidth =
    /// Math.max(contentWidth, width)`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getContentWidth")]
    pub fn get_content_width(this: &ICodeEditor) -> f64;
    /// Get the scrollWidth of the editor's viewport.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getScrollWidth")]
    pub fn get_scroll_width(this: &ICodeEditor) -> f64;
    /// Get the scrollLeft of the editor's viewport.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getScrollLeft")]
    pub fn get_scroll_left(this: &ICodeEditor) -> f64;
    /// Get the height of the editor's content.
    /// This is information that is "erased" when computing `scrollHeight =
    /// Math.max(contentHeight, height)`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getContentHeight")]
    pub fn get_content_height(this: &ICodeEditor) -> f64;
    /// Get the scrollHeight of the editor's viewport.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getScrollHeight")]
    pub fn get_scroll_height(this: &ICodeEditor) -> f64;
    /// Get the scrollTop of the editor's viewport.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getScrollTop")]
    pub fn get_scroll_top(this: &ICodeEditor) -> f64;
    /// Change the scrollLeft of the editor's viewport.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "setScrollLeft")]
    pub fn set_scroll_left(this: &ICodeEditor, new_scroll_left: f64);
    /// Change the scrollTop of the editor's viewport.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "setScrollTop")]
    pub fn set_scroll_top(this: &ICodeEditor, new_scroll_top: f64);
    /// Change the scroll position of the editor's viewport.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "setScrollPosition")]
    pub fn set_scroll_position(this: &ICodeEditor, position: &INewScrollPosition);
    /// Get an action that is a contribution to this editor.
    /// @id Unique identifier of the contribution.
    /// @return The action or null if action not found.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getAction")]
    pub fn get_action(this: &ICodeEditor, id: &str) -> IEditorAction;
    /// Execute a command on the editor.
    /// The edits will land on the undo-redo stack, but no "undo stop" will be
    /// pushed. @param source The source of the call.
    /// @param command The command to execute
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "executeCommand")]
    pub fn execute_command(this: &ICodeEditor, source: &str, command: &ICommand);
    /// Push an "undo stop" in the undo-redo stack.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "pushUndoStop")]
    pub fn push_undo_stop(this: &ICodeEditor) -> bool;
    /// Execute edits on the editor.
    /// The edits will land on the undo-redo stack, but no "undo stop" will be
    /// pushed. @param source The source of the call.
    /// @param edits The edits to execute.
    /// @param endCursorState Cursor state after the edits were applied.
    ///
    /// # Arguments
    ///
    /// * `edits` - `IIdentifiedSingleEditOperation[]`
    /// * `end_cursor_state` - `( inverseEditOperations:
    ///   IIdentifiedSingleEditOperation[] ) => Selection[] | Selection[]`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "executeEdits")]
    pub fn execute_edits(
        this: &ICodeEditor,
        source: &str,
        edits: &Array,
        end_cursor_state: Option<Vec<JsValue>>,
    ) -> bool;
    /// Execute multiple (concomitant) commands on the editor.
    /// @param source The source of the call.
    /// @param command The commands to execute
    ///
    /// # Arguments
    ///
    /// * `commands` - `(ICommand | null)[]`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "executeCommands")]
    pub fn execute_commands(this: &ICodeEditor, source: &str, commands: Vec<JsValue>);
    /// Get all the decorations on a line (filtering out decorations from other
    /// editors).
    ///
    /// # Returns
    ///
    /// `IModelDecoration[]`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getLineDecorations")]
    pub fn get_line_decorations(this: &ICodeEditor, line_number: f64) -> Option<Array>;
    /// All decorations added through this call will get the ownerId of this
    /// editor. @see `ITextModel.deltaDecorations`
    ///
    /// # Arguments
    ///
    /// * `old_decorations` - `string[]`
    /// * `new_decorations` - `IModelDeltaDecoration[]`
    ///
    /// # Returns
    ///
    /// `string[]`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "deltaDecorations")]
    pub fn delta_decorations(
        this: &ICodeEditor,
        old_decorations: &Array,
        new_decorations: &Array,
    ) -> Array;
    /// Get the layout info for the editor.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getLayoutInfo")]
    pub fn get_layout_info(this: &ICodeEditor) -> EditorLayoutInfo;
    /// Returns the ranges that are currently visible.
    /// Does not account for horizontal scrolling.
    ///
    /// # Returns
    ///
    /// `Range[]`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getVisibleRanges")]
    pub fn get_visible_ranges(this: &ICodeEditor) -> Array;
    /// Get the vertical position (top offset) for the line w.r.t. to the first
    /// line.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getTopForLineNumber")]
    pub fn get_top_for_line_number(this: &ICodeEditor, line_number: f64) -> f64;
    /// Get the vertical position (top offset) for the position w.r.t. to the
    /// first line.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getTopForPosition")]
    pub fn get_top_for_position(this: &ICodeEditor, line_number: f64, column: f64) -> f64;
    /// Returns the editor's container dom node
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getContainerDomNode")]
    pub fn get_container_dom_node(this: &ICodeEditor) -> HtmlElement;
    /// Returns the editor's dom node
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getDomNode")]
    pub fn get_dom_node(this: &ICodeEditor) -> Option<HtmlElement>;
    /// Add a content widget. Widgets must have unique ids, otherwise they will
    /// be overwritten.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "addContentWidget")]
    pub fn add_content_widget(this: &ICodeEditor, widget: &IContentWidget);
    /// Layout/Reposition a content widget. This is a ping to the editor to call
    /// widget.getPosition() and update appropriately.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "layoutContentWidget")]
    pub fn layout_content_widget(this: &ICodeEditor, widget: &IContentWidget);
    /// Remove a content widget.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "removeContentWidget")]
    pub fn remove_content_widget(this: &ICodeEditor, widget: &IContentWidget);
    /// Add an overlay widget. Widgets must have unique ids, otherwise they will
    /// be overwritten.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "addOverlayWidget")]
    pub fn add_overlay_widget(this: &ICodeEditor, widget: &IOverlayWidget);
    /// Layout/Reposition an overlay widget. This is a ping to the editor to
    /// call widget.getPosition() and update appropriately.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "layoutOverlayWidget")]
    pub fn layout_overlay_widget(this: &ICodeEditor, widget: &IOverlayWidget);
    /// Remove an overlay widget.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "removeOverlayWidget")]
    pub fn remove_overlay_widget(this: &ICodeEditor, widget: &IOverlayWidget);
    /// Change the view zones. View zones are lost when a new model is attached
    /// to the editor.
    ///
    /// # Arguments
    ///
    /// * `callback` - `(accessor: IViewZoneChangeAccessor) => void`
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "changeViewZones")]
    pub fn change_view_zones(this: &ICodeEditor, callback: &Function);
    /// Get the horizontal position (left offset) for the column w.r.t to the
    /// beginning of the line. This method works only if the line
    /// `lineNumber` is currently rendered (in the editor's viewport).
    /// Use this method with caution.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getOffsetForColumn")]
    pub fn get_offset_for_column(this: &ICodeEditor, line_number: f64, column: f64) -> f64;
    /// Force an editor render now.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "render")]
    pub fn render(this: &ICodeEditor, force_redraw: Option<bool>);
    /// Get the hit test target at coordinates `clientX` and `clientY`.
    /// The coordinates are relative to the top-left of the viewport.
    ///
    /// @returns Hit test target or null if the coordinates fall outside the
    /// editor or the editor has no model.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getTargetAtClientPoint")]
    pub fn get_target_at_client_point(
        this: &ICodeEditor,
        client_x: f64,
        client_y: f64,
    ) -> Option<IMouseTarget>;
    /// Get the visible position for `position`.
    /// The result position takes scrolling into account and is relative to the
    /// top left corner of the editor. Explanation 1: the results of this
    /// method will change for the same `position` if the user scrolls the
    /// editor. Explanation 2: the results of this method will not change if
    /// the container of the editor gets repositioned. Warning: the results
    /// of this method are inaccurate for positions that are outside the current
    /// editor viewport.
    ///
    /// # Returns
    ///
    /// `{ top: number; left: number; height: number }`
    #[wasm_bindgen(
        method,
        js_class = "ICodeEditor",
        js_name = "getScrolledVisiblePosition"
    )]
    pub fn get_scrolled_visible_position(
        this: &ICodeEditor,
        position: &IPosition,
    ) -> Option<Object>;
    /// Apply the same font settings as the editor to `target`.
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "applyFontInfo")]
    pub fn apply_font_info(this: &ICodeEditor, target: &HtmlElement);
}

#[wasm_bindgen]
extern "C" {
    /// Information about a line in the diff editor
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IDiffLineInformation;
    #[wasm_bindgen(method, js_class = "IDiffLineInformation", js_name = "equivalentLineNumber", getter = equivalentLineNumber)]
    pub fn equivalent_line_number(this: &IDiffLineInformation) -> f64;
}

#[wasm_bindgen]
extern "C" {
    /// A rich diff editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = IEditor, extends = Object)]
    pub type IDiffEditor;
    /// @see ICodeEditor.getDomNode
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "getDomNode")]
    pub fn get_dom_node(this: &IDiffEditor) -> HtmlElement;
    /// An event emitted when the diff information computed by this diff editor
    /// has been updated. @event
    ///
    /// # Arguments
    ///
    /// * `listener` - `() => void`
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "onDidUpdateDiff")]
    pub fn on_did_update_diff(this: &IDiffEditor, listener: &Function) -> IDisposable;
    /// Saves current view state of the editor in a serializable object.
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "saveViewState")]
    pub fn save_view_state(this: &IDiffEditor) -> Option<IDiffEditorViewState>;
    /// Restores the view state of the editor from a serializable object
    /// generated by `saveViewState`.
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "restoreViewState")]
    pub fn restore_view_state(this: &IDiffEditor, state: &IDiffEditorViewState);
    /// Type the getModel() of IEditor.
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "getModel")]
    pub fn get_model(this: &IDiffEditor) -> Option<IDiffEditorModel>;
    /// Sets the current model attached to this editor.
    /// If the previous model was created by the editor via the value key in the
    /// options literal object, it will be destroyed. Otherwise, if the
    /// previous model was set via setModel, or the model key in the options
    /// literal object, the previous model will not be destroyed.
    /// It is safe to call setModel(null) to simply detach the current model
    /// from the editor.
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "setModel")]
    pub fn set_model(this: &IDiffEditor, model: Option<&IDiffEditorModel>);
    /// Get the `original` editor.
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "getOriginalEditor")]
    pub fn get_original_editor(this: &IDiffEditor) -> ICodeEditor;
    /// Get the `modified` editor.
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "getModifiedEditor")]
    pub fn get_modified_editor(this: &IDiffEditor) -> ICodeEditor;
    /// Get the computed diff information.
    ///
    /// # Returns
    ///
    /// `ILineChange[]`
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "getLineChanges")]
    pub fn get_line_changes(this: &IDiffEditor) -> Option<Array>;
    /// Get information based on computed diff about a line number from the
    /// original model. If the diff computation is not finished or the model
    /// is missing, will return null.
    #[wasm_bindgen(
        method,
        js_class = "IDiffEditor",
        js_name = "getDiffLineInformationForOriginal"
    )]
    pub fn get_diff_line_information_for_original(
        this: &IDiffEditor,
        line_number: f64,
    ) -> Option<IDiffLineInformation>;
    /// Get information based on computed diff about a line number from the
    /// modified model. If the diff computation is not finished or the model
    /// is missing, will return null.
    #[wasm_bindgen(
        method,
        js_class = "IDiffEditor",
        js_name = "getDiffLineInformationForModified"
    )]
    pub fn get_diff_line_information_for_modified(
        this: &IDiffEditor,
        line_number: f64,
    ) -> Option<IDiffLineInformation>;
    /// Update the editor's options after the editor has been created.
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "updateOptions")]
    pub fn update_options(this: &IDiffEditor, new_options: &IDiffEditorOptions);
}

str_enum! {
    pub enum IEditorOptionsCursorSurroundingLinesStyle {
        Default = "default",
        All = "all",
    }
}

str_enum! {
    pub enum IEditorOptionsRenderValidationDecorations {
        Editable = "editable",
        On = "on",
        Off = "off",
    }
}

str_enum! {
    pub enum IEditorOptionsCursorBlinking {
        Blink = "blink",
        Smooth = "smooth",
        Phase = "phase",
        Expand = "expand",
        Solid = "solid",
    }
}

str_enum! {
    pub enum IEditorOptionsMouseStyle {
        Text = "text",
        Default = "default",
        Copy = "copy",
    }
}

str_enum! {
    pub enum IEditorOptionsCursorStyle {
        Line = "line",
        Block = "block",
        Underline = "underline",
        LineThin = "line-thin",
        BlockOutline = "block-outline",
        UnderlineThin = "underline-thin",
    }
}

str_enum! {
    pub enum IEditorOptionsWordWrap {
        Off = "off",
        On = "on",
        WordWrapColumn = "wordWrapColumn",
        Bounded = "bounded",
    }
}

str_enum! {
    pub enum IEditorOptionsWrappingIndent {
        None = "none",
        Same = "same",
        Indent = "indent",
        DeepIndent = "deepIndent",
    }
}

str_enum! {
    pub enum IEditorOptionsWrappingStrategy {
        Simple = "simple",
        Advanced = "advanced",
    }
}

str_enum! {
    pub enum IEditorOptionsMultiCursorModifier {
        CtrlCmd = "ctrlCmd",
        Alt = "alt",
    }
}

str_enum! {
    pub enum IEditorOptionsMultiCursorPaste {
        Spread = "spread",
        Full = "full",
    }
}

str_enum! {
    pub enum IEditorOptionsAccessibilitySupport {
        Auto = "auto",
        Off = "off",
        On = "on",
    }
}

str_enum! {
    pub enum IEditorOptionsAutoIndent {
        None = "none",
        Keep = "keep",
        Brackets = "brackets",
        Advanced = "advanced",
        Full = "full",
    }
}

str_enum! {
    pub enum IEditorOptionsAcceptSuggestionOnEnter {
        On = "on",
        Smart = "smart",
        Off = "off",
    }
}

str_enum! {
    pub enum IEditorOptionsSnippetSuggestions {
        Top = "top",
        Bottom = "bottom",
        Inline = "inline",
        None = "none",
    }
}

str_enum! {
    pub enum IEditorOptionsSuggestSelection {
        First = "first",
        RecentlyUsed = "recentlyUsed",
        RecentlyUsedByPrefix = "recentlyUsedByPrefix",
    }
}

str_enum! {
    pub enum IEditorOptionsTabCompletion {
        On = "on",
        Off = "off",
        OnlySnippets = "onlySnippets",
    }
}

str_enum! {
    pub enum IEditorOptionsFoldingStrategy {
        Auto = "auto",
        Indentation = "indentation",
    }
}

str_enum! {
    pub enum IEditorOptionsShowFoldingControls {
        Always = "always",
        Mouseover = "mouseover",
    }
}

str_enum! {
    pub enum IEditorOptionsMatchBrackets {
        Never = "never",
        Near = "near",
        Always = "always",
    }
}

str_enum! {
    pub enum IEditorOptionsRenderWhitespace {
        None = "none",
        Boundary = "boundary",
        Selection = "selection",
        All = "all",
    }
}

str_enum! {
    pub enum IEditorOptionsRenderLineHighlight {
        None = "none",
        Gutter = "gutter",
        Line = "line",
        All = "all",
    }
}

str_enum! {
    pub enum IEditorOptionsPeekWidgetDefaultFocus {
        Tree = "tree",
        Editor = "editor",
    }
}

str_enum! {
    pub enum IEditorFindOptionsAutoFindInSelection {
        Never = "never",
        Always = "always",
        Multiline = "multiline",
    }
}

str_enum! {
    pub enum IEditorMinimapOptionsSide {
        Right = "right",
        Left = "left",
    }
}

str_enum! {
    pub enum IEditorMinimapOptionsShowSlider {
        Always = "always",
        Mouseover = "mouseover",
    }
}

str_enum! {
    pub enum IEditorScrollbarOptionsVertical {
        Auto = "auto",
        Visible = "visible",
        Hidden = "hidden",
    }
}

str_enum! {
    pub enum IEditorScrollbarOptionsHorizontal {
        Auto = "auto",
        Visible = "visible",
        Hidden = "hidden",
    }
}

str_enum! {
    pub enum ISuggestOptionsInsertMode {
        Insert = "insert",
        Replace = "replace",
    }
}
