use crate::sys::Uri;
use js_sys::{Object, Uint32Array};
use wasm_bindgen::{prelude::*, JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
    /// Options which apply for all editors.
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = Object)]
    pub type IGlobalEditorOptions;
}
impl IGlobalEditorOptions {
    /// Controls whether tabSize and insertSpaces will be automatically detected
    /// when a file is opened based on the file contents. Defaults to true.
    pub fn detect_indentation(self, val: bool) -> Self {
        object_set!(self.detectIndentation = val);
        self
    }

    /// Insert spaces when pressing Tab. This setting is overridden based on the
    /// file contents when detectIndentation is on. Defaults to true.
    pub fn insert_spaces(self, val: bool) -> Self {
        object_set!(self.insertSpaces = val);
        self
    }

    /// Special handling for large files to disable certain memory intensive
    /// features. Defaults to true.
    pub fn large_file_optimizations(self, val: bool) -> Self {
        object_set!(self.largeFileOptimizations = val);
        self
    }

    /// Lines above this length will not be tokenized for performance reasons.
    /// Defaults to 20000.
    pub fn max_tokenization_line_length(self, val: u32) -> Self {
        object_set!(self.maxTokenizationLineLength = val);
        self
    }

    /// Keep peek editors open even when double clicking their content or when
    /// hitting Escape. Defaults to false.
    pub fn stable_peek(self, val: bool) -> Self {
        object_set!(self.stablePeek = val);
        self
    }

    /// The number of spaces a tab is equal to. This setting is overridden based
    /// on the file contents when detectIndentation is on. Defaults to 4.
    pub fn tab_size(self, val: u32) -> Self {
        object_set!(self.tabSize = val);
        self
    }

    /// Remove trailing auto inserted whitespace. Defaults to true.
    pub fn trim_auto_whitespace(self, val: bool) -> Self {
        object_set!(self.trimAutoWhitespace = val);
        self
    }

    /// Controls whether completions should be computed based on words in the
    /// document. Defaults to true.
    pub fn word_based_suggestions(self, val: bool) -> Self {
        object_set!(self.wordBasedSuggestions = val);
        self
    }
}
impl Default for IGlobalEditorOptions {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

#[wasm_bindgen]
extern "C" {
    /// Configuration options for the editor.
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = Object)]
    pub type IEditorOptions;
}
impl IEditorOptions {
    /// Accept suggestions on provider defined characters. Defaults to true.
    pub fn accept_suggestion_on_commit_character(self, val: bool) -> Self {
        object_set!(self.acceptSuggestionOnCommitCharacter = val);
        self
    }

    /// Accept suggestions on ENTER. Defaults to 'on'.
    /// "on" | "smart" | "off"
    pub fn accept_suggestion_on_enter(self, val: &str) -> Self {
        object_set!(self.acceptSuggestionOnEnter = val);
        self
    }

    /// Controls the number of lines in the editor that can be read out by a
    /// screen reader
    pub fn accessibility_page_size(self, val: u32) -> Self {
        object_set!(self.accessibilityPageSize = val);
        self
    }

    /// Configure the editor's accessibility support. Defaults to 'auto'. It is
    /// best to leave this to 'auto'. "auto" | "off" | "on"
    pub fn accessibility_support(self, val: &str) -> Self {
        object_set!(self.accessibilitySupport = val);
        self
    }

    /// The aria label for the editor's textarea (when it is focused).
    pub fn aria_label(self, val: &str) -> Self {
        object_set!(self.ariaLabel = val);
        self
    }

    /// Options for auto closing brackets. Defaults to language defined
    /// behavior.
    pub fn auto_closing_brackets(self, val: &EditorAutoClosingStrategy) -> Self {
        object_set!(self.autoClosingBrackets = val);
        self
    }

    /// Options for typing over closing quotes or brackets.
    pub fn auto_closing_overtype(self, val: &EditorAutoClosingOvertypeStrategy) -> Self {
        object_set!(self.autoClosingOvertype = val);
        self
    }

    /// Options for auto closing quotes. Defaults to language defined behavior.
    pub fn auto_closing_quotes(self, val: &EditorAutoClosingStrategy) -> Self {
        object_set!(self.autoClosingQuotes = val);
        self
    }

    /// Controls whether the editor should automatically adjust the indentation
    /// when users type, paste, move or indent lines. Defaults to advanced.
    /// "none" | "keep" | "brackets" | "advanced" | "full"
    pub fn auto_indent(self, val: &str) -> Self {
        object_set!(self.autoIndent = val);
        self
    }

    /// Options for auto surrounding. Defaults to always allowing auto
    /// surrounding.
    pub fn auto_surround(self, val: &EditorAutoSurroundStrategy) -> Self {
        object_set!(self.autoSurround = val);
        self
    }

    /// Enable that the editor will install an interval to check if its
    /// container dom node size has changed. Enabling this might have a severe
    /// performance impact. Defaults to false.
    pub fn automatic_layout(self, val: bool) -> Self {
        object_set!(self.automaticLayout = val);
        self
    }

    /// Timeout for running code actions on save.
    pub fn code_actions_on_save_timeout(self, val: u32) -> Self {
        object_set!(self.codeActionsOnSaveTimeout = val);
        self
    }

    /// Show code lens Defaults to true.
    pub fn code_lens(self, val: bool) -> Self {
        object_set!(self.codeLens = val);
        self
    }

    /// Enable inline color decorators and color picker rendering.
    pub fn color_decorators(self, val: bool) -> Self {
        object_set!(self.colorDecorators = val);
        self
    }

    /// Control the behaviour of comments in the editor.
    pub fn comments(self, val: &IEditorCommentsOptions) -> Self {
        object_set!(self.comments = val);
        self
    }

    /// Enable custom contextmenu. Defaults to true.
    pub fn contextmenu(self, val: bool) -> Self {
        object_set!(self.contextmenu = val);
        self
    }

    /// Syntax highlighting is copied.
    pub fn copy_with_syntax_highlighting(self, val: bool) -> Self {
        object_set!(self.copyWithSyntaxHighlighting = val);
        self
    }

    /// Control the cursor animation style, possible values are 'blink',
    /// 'smooth', 'phase', 'expand' and 'solid'. Defaults to 'blink'.
    /// "blink" | "smooth" | "phase" | "expand" | "solid"
    pub fn cursor_blinking(self, val: &str) -> Self {
        object_set!(self.cursorBlinking = val);
        self
    }

    /// Enable smooth caret animation. Defaults to false.
    pub fn cursor_smooth_caret_animation(self, val: bool) -> Self {
        object_set!(self.cursorSmoothCaretAnimation = val);
        self
    }

    /// Control the cursor style, either 'block' or 'line'. Defaults to 'line'.
    /// "line" | "block" | "underline" | "line-thin" | "block-outline" |
    /// "underline-thin"
    pub fn cursor_style(self, val: &str) -> Self {
        object_set!(self.cursorStyle = val);
        self
    }

    /// Controls the minimal number of visible leading and trailing lines
    /// surrounding the cursor. Defaults to 0.
    pub fn cursor_surrounding_lines(self, val: u32) -> Self {
        object_set!(self.cursorSurroundingLines = val);
        self
    }

    /// Controls when cursorSurroundingLines should be enforced Defaults to
    /// default, cursorSurroundingLines is not enforced when cursor position is
    /// changed by mouse. "default" | "all"
    pub fn cursor_surrounding_lines_style(self, val: &str) -> Self {
        object_set!(self.cursorSurroundingLinesStyle = val);
        self
    }

    /// Control the width of the cursor when cursorStyle is set to 'line'
    pub fn cursor_width(self, val: u32) -> Self {
        object_set!(self.cursorWidth = val);
        self
    }

    /// Disable the use of transform: translate3d(0px, 0px, 0px) for the editor
    /// margin and lines layers. The usage of transform: translate3d(0px, 0px,
    /// 0px) acts as a hint for browsers to create an extra layer. Defaults to
    /// false.
    pub fn disable_layer_hinting(self, val: bool) -> Self {
        object_set!(self.disableLayerHinting = val);
        self
    }

    /// Disable the optimizations for monospace fonts. Defaults to false.
    pub fn disable_monospace_optimizations(self, val: bool) -> Self {
        object_set!(self.disableMonospaceOptimizations = val);
        self
    }

    /// Controls if the editor should allow to move selections via drag and
    /// drop. Defaults to false.
    pub fn drag_and_drop(self, val: bool) -> Self {
        object_set!(self.dragAndDrop = val);
        self
    }

    /// Copying without a selection copies the current line.
    pub fn empty_selection_clipboard(self, val: bool) -> Self {
        object_set!(self.emptySelectionClipboard = val);
        self
    }

    /// Class name to be added to the editor.
    pub fn extra_editor_class_name(self, val: &str) -> Self {
        object_set!(self.extraEditorClassName = val);
        self
    }

    /// FastScrolling mulitplier speed when pressing Alt Defaults to 5.
    pub fn fast_scroll_sensitivity(self, val: u32) -> Self {
        object_set!(self.fastScrollSensitivity = val);
        self
    }

    /// Control the behavior of the find widget.
    pub fn find(self, val: &IEditorFindOptions) -> Self {
        object_set!(self.find = val);
        self
    }

    /// Display overflow widgets as fixed. Defaults to false.
    pub fn fixed_overflow_widgets(self, val: bool) -> Self {
        object_set!(self.fixedOverflowWidgets = val);
        self
    }

    /// Enable code folding. Defaults to true.
    pub fn folding(self, val: bool) -> Self {
        object_set!(self.folding = val);
        self
    }

    /// Enable highlight for folded regions. Defaults to true.
    pub fn folding_highlight(self, val: bool) -> Self {
        object_set!(self.foldingHighlight = val);
        self
    }

    /// Selects the folding strategy. 'auto' uses the strategies contributed for
    /// the current document, 'indentation' uses the indentation based folding
    /// strategy. Defaults to 'auto'. "auto" | "indentation"
    pub fn folding_strategy(self, val: &str) -> Self {
        object_set!(self.foldingStrategy = val);
        self
    }

    /// The font family
    pub fn font_family(self, val: &str) -> Self {
        object_set!(self.fontFamily = val);
        self
    }

    /// Enable font ligatures. Defaults to false.
    // TODO this one is special: bool | string
    pub fn font_ligatures(self, val: &str) -> Self {
        object_set!(self.fontLigatures = val);
        self
    }

    /// The font size
    pub fn font_size(self, val: u32) -> Self {
        object_set!(self.fontSize = val);
        self
    }

    /// The font weight
    pub fn font_weight(self, val: &str) -> Self {
        object_set!(self.fontWeight = val);
        self
    }

    /// Enable format on paste. Defaults to false.
    pub fn format_on_paste(self, val: bool) -> Self {
        object_set!(self.formatOnPaste = val);
        self
    }

    /// Enable format on type. Defaults to false.
    pub fn format_on_type(self, val: bool) -> Self {
        object_set!(self.formatOnType = val);
        self
    }

    /// Enable the rendering of the glyph margin. Defaults to true in vscode and
    /// to false in monaco-editor.
    pub fn glyph_margin(self, val: bool) -> Self {
        object_set!(self.glyphMargin = val);
        self
    }

    /// Optional hideCursorInOverviewRuler
    pub fn goto_location(self, val: &IGotoLocationOptions) -> Self {
        object_set!(self.gotoLocation = val);
        self
    }

    /// Should the cursor be hidden in the overview ruler. Defaults to false.
    pub fn hide_cursor_in_overview_ruler(self, val: bool) -> Self {
        object_set!(self.hideCursorInOverviewRuler = val);
        self
    }

    /// Enable highlighting of the active indent guide. Defaults to true.
    pub fn highlightActiveIndentGuide(self, val: bool) -> Self {
        object_set!(self.highlightActiveIndentGuide = val);
        self
    }

    /// Configure the editor's hover.
    pub fn hover(self, val: &IEditorHoverOptions) -> Self {
        object_set!(self.hover = val);
        self
    }

    /// This editor is used inside a diff editor.
    pub fn in_diff_editor(self, val: bool) -> Self {
        object_set!(self.inDiffEditor = val);
        self
    }

    /// The letter spacing
    pub fn letter_spacing(self, val: u32) -> Self {
        object_set!(self.letterSpacing = val);
        self
    }

    /// Control the behavior and rendering of the code action lightbulb.
    pub fn lightbulb(self, val: &IEditorLightbulbOptions) -> Self {
        object_set!(self.lightbulb = val);
        self
    }

    /// The width reserved for line decorations (in px). Line decorations are
    /// placed between line numbers and the editor content. You can pass in a
    /// string in the format floating point followed by "ch". e.g. 1.3ch.
    /// Defaults to 10.
    // TODO special value: u32 | string
    pub fn line_decorations_width(self, val: u32) -> Self {
        object_set!(self.lineDecorationsWidth = val);
        self
    }

    /// The line height
    pub fn line_height(self, val: u32) -> Self {
        object_set!(self.lineHeight = val);
        self
    }

    /// Control the rendering of line numbers. If it is a function, it will be
    /// invoked when rendering a line number and the return value will be
    /// rendered. Otherwise, if it is a truey, line numbers will be rendered
    /// normally (equivalent of using an identity function). Otherwise, line
    /// numbers will not be rendered. Defaults to on.
    pub fn line_numbers(self, val: &LineNumbersType) -> Self {
        object_set!(self.lineNumbers = val);
        self
    }

    /// Control the width of line numbers, by reserving horizontal space for
    /// rendering at least an amount of digits. Defaults to 5.
    pub fn line_numbers_min_chars(self, val: u32) -> Self {
        object_set!(self.lineNumbersMinChars = val);
        self
    }

    /// Enable detecting links and making them clickable. Defaults to true.
    pub fn links(self, val: bool) -> Self {
        object_set!(self.links = val);
        self
    }

    /// Enable highlighting of matching brackets. Defaults to 'always'.
    /// "never" | "near" | "always"
    pub fn match_brackets(self, val: &str) -> Self {
        object_set!(self.matchBrackets = val);
        self
    }

    /// Control the behavior and rendering of the minimap.
    pub fn minimap(self, val: &IEditorMinimapOptions) -> Self {
        object_set!(self.minimap = val);
        self
    }

    /// Control the mouse pointer style, either 'text' or 'default' or 'copy'
    /// Defaults to 'text' "text" | "default" | "copy"
    pub fn mouse_style(self, val: &str) -> Self {
        object_set!(self.mouseStyle = val);
        self
    }

    /// A multiplier to be used on the deltaX and deltaY of mouse wheel scroll
    /// events. Defaults to 1.
    pub fn mouse_wheel_scroll_sensitivity(self, val: u32) -> Self {
        object_set!(self.mouseWheelScrollSensitivity = val);
        self
    }

    /// Zoom the font in the editor when using the mouse wheel in combination
    /// with holding Ctrl. Defaults to false.
    pub fn mouse_wheel_zoom(self, val: bool) -> Self {
        object_set!(self.mouseWheelZoom = val);
        self
    }

    /// Merge overlapping selections. Defaults to true
    pub fn multi_cursor_merge_overlapping(self, val: bool) -> Self {
        object_set!(self.multiCursorMergeOverlapping = val);
        self
    }

    /// The modifier to be used to add multiple cursors with the mouse. Defaults
    /// to 'alt' "ctrlCmd" | "alt"
    pub fn multi_cursor_modifier(self, val: &str) -> Self {
        object_set!(self.multiCursorModifier = val);
        self
    }

    /// Configure the behaviour when pasting a text with the line count equal to
    /// the cursor count. Defaults to 'spread'. "spread" | "full"
    pub fn multi_cursor_paste(self, val: &str) -> Self {
        object_set!(self.multiCursorPaste = val);
        self
    }

    /// Enable semantic occurrences highlight. Defaults to true.
    pub fn occurrences_highlight(self, val: bool) -> Self {
        object_set!(self.occurrencesHighlight = val);
        self
    }

    /// Controls if a border should be drawn around the overview ruler. Defaults
    /// to true.
    pub fn overview_ruler_border(self, val: bool) -> Self {
        object_set!(self.overviewRulerBorder = val);
        self
    }

    /// The number of vertical lanes the overview ruler should render. Defaults
    /// to 3.
    pub fn overview_ruler_lanes(self, val: u32) -> Self {
        object_set!(self.overviewRulerLanes = val);
        self
    }

    /// Parameter hint options.
    pub fn parameter_hints(self, val: &IEditorParameterHintOptions) -> Self {
        object_set!(self.parameterHints = val);
        self
    }

    /// Controls whether to focus the inline editor in the peek widget by
    /// default. Defaults to false. "tree" | "editor"
    pub fn peek_widget_default_focus(self, val: &str) -> Self {
        object_set!(self.peekWidgetDefaultFocus = val);
        self
    }

    /// Enable quick suggestions (shadow suggestions) Defaults to true.
    // TODO: special value: bool | IQuickSuggestionsOptions
    pub fn quick_suggestions(self, val: bool) -> Self {
        object_set!(self.quickSuggestions = val);
        self
    }

    /// Quick suggestions show delay (in ms) Defaults to 10 (ms)
    pub fn quick_suggestions_delay(self, val: u32) -> Self {
        object_set!(self.quickSuggestionsDelay = val);
        self
    }

    /// Should the editor be read only. Defaults to false.
    pub fn read_only(self, val: bool) -> Self {
        object_set!(self.readOnly = val);
        self
    }

    /// Enable rendering of control characters. Defaults to false.
    pub fn render_control_characters(self, val: bool) -> Self {
        object_set!(self.renderControlCharacters = val);
        self
    }

    /// Render last line number when the file ends with a newline. Defaults to
    /// true.
    pub fn render_final_newline(self, val: bool) -> Self {
        object_set!(self.renderFinalNewline = val);
        self
    }

    /// Enable rendering of indent guides. Defaults to true.
    pub fn render_indent_guides(self, val: bool) -> Self {
        object_set!(self.renderIndentGuides = val);
        self
    }

    /// Enable rendering of current line highlight. Defaults to all.
    /// "none" | "gutter" | "line" | "all"
    pub fn render_line_highlight(self, val: &str) -> Self {
        object_set!(self.renderLineHighlight = val);
        self
    }

    /// Should the editor render validation decorations. Defaults to editable.
    /// "editable" | "on" | "off"
    pub fn render_validation_decorations(self, val: &str) -> Self {
        object_set!(self.renderValidationDecorations = val);
        self
    }

    /// Enable rendering of whitespace. Defaults to none.
    /// "none" | "boundary" | "selection" | "all"
    pub fn render_whitespace(self, val: &str) -> Self {
        object_set!(self.renderWhitespace = val);
        self
    }

    /// When revealing the cursor, a virtual padding (px) is added to the
    /// cursor, turning it into a rectangle. This virtual padding ensures that
    /// the cursor gets revealed before hitting the edge of the viewport.
    /// Defaults to 30 (px).
    pub fn reveal_horizontal_right_padding(self, val: u32) -> Self {
        object_set!(self.revealHorizontalRightPadding = val);
        self
    }

    /// Render the editor selection with rounded borders. Defaults to true.
    pub fn rounded_selection(self, val: bool) -> Self {
        object_set!(self.roundedSelection = val);
        self
    }

    /// Render vertical lines at the specified columns. Defaults to empty array.
    pub fn rulers(self, val: &[u32]) -> Self {
        object_set!(self.rulers = Uint32Array::from(val));
        self
    }

    /// Enable that scrolling can go beyond the last column by a number of
    /// columns. Defaults to 5.
    pub fn scroll_beyond_last_column(self, val: u32) -> Self {
        object_set!(self.scrollBeyondLastColumn = val);
        self
    }

    /// Enable that scrolling can go one screen size after the last line.
    /// Defaults to true.
    pub fn scroll_beyond_last_line(self, val: bool) -> Self {
        object_set!(self.scrollBeyondLastLine = val);
        self
    }

    /// Control the behavior and rendering of the scrollbars.
    pub fn scrollbar(self, val: &IEditorScrollbarOptions) -> Self {
        object_set!(self.scrollbar = val);
        self
    }

    /// Should the corresponding line be selected when clicking on the line
    /// number? Defaults to true.
    pub fn select_on_line_numbers(self, val: bool) -> Self {
        object_set!(self.selectOnLineNumbers = val);
        self
    }

    /// Enable Linux primary clipboard. Defaults to true.
    pub fn selection_clipboard(self, val: bool) -> Self {
        object_set!(self.selectionClipboard = val);
        self
    }

    /// Enable selection highlight. Defaults to true.
    pub fn selection_highlight(self, val: bool) -> Self {
        object_set!(self.selectionHighlight = val);
        self
    }

    /// Controls whether the fold actions in the gutter stay always visible or
    /// hide unless the mouse is over the gutter. Defaults to 'mouseover'.
    /// "always" | "mouseover"
    pub fn show_folding_controls(self, val: &str) -> Self {
        object_set!(self.showFoldingControls = val);
        self
    }

    /// Controls fading out of unused variables.
    pub fn show_unused(self, val: bool) -> Self {
        object_set!(self.showUnused = val);
        self
    }

    /// Enable that the editor animates scrolling to a position. Defaults to
    /// false.
    pub fn smooth_scrolling(self, val: bool) -> Self {
        object_set!(self.smoothScrolling = val);
        self
    }

    /// Enable snippet suggestions. Default to 'true'.
    /// "top" | "bottom" | "inline" | "none"
    pub fn snippet_suggestions(self, val: &str) -> Self {
        object_set!(self.snippetSuggestions = val);
        self
    }

    /// Performance guard: Stop rendering a line after x characters. Defaults to
    /// 10000. Use -1 to never stop rendering
    pub fn stop_rendering_line_after(self, val: u32) -> Self {
        object_set!(self.stopRenderingLineAfter = val);
        self
    }

    /// Suggest options.
    pub fn suggest(self, val: &ISuggestOptions) -> Self {
        object_set!(self.suggest = val);
        self
    }

    /// The font size for the suggest widget. Defaults to the editor font size.
    pub fn suggest_font_size(self, val: u32) -> Self {
        object_set!(self.suggestFontSize = val);
        self
    }

    /// The line height for the suggest widget. Defaults to the editor line
    /// height.
    pub fn suggest_line_height(self, val: u32) -> Self {
        object_set!(self.suggestLineHeight = val);
        self
    }

    /// Enable the suggestion box to pop-up on trigger characters. Defaults to
    /// true.
    pub fn suggest_on_trigger_characters(self, val: bool) -> Self {
        object_set!(self.suggestOnTriggerCharacters = val);
        self
    }

    /// The history mode for suggestions.
    /// "first" | "recentlyUsed" | "recentlyUsedByPrefix"
    pub fn suggest_selection(self, val: &str) -> Self {
        object_set!(self.suggestSelection = val);
        self
    }

    /// Enable tab completion.
    /// "on" | "off" | "onlySnippets"
    pub fn tab_completion(self, val: &str) -> Self {
        object_set!(self.tabCompletion = val);
        self
    }

    /// Inserting and deleting whitespace follows tab stops.
    pub fn use_tab_stops(self, val: bool) -> Self {
        object_set!(self.useTabStops = val);
        self
    }

    /// A string containing the word separators used when doing word navigation.
    /// Defaults to `~!@#$%^&*()-=+[{]}\|;:'",.<>/?
    pub fn word_separators(self, val: &str) -> Self {
        object_set!(self.wordSeparators = val);
        self
    }

    /// Control the wrapping of the editor. When wordWrap = "off", the lines
    /// will never wrap. When wordWrap = "on", the lines will wrap at the
    /// viewport width. When wordWrap = "wordWrapColumn", the lines will wrap at
    /// wordWrapColumn. When wordWrap = "bounded", the lines will wrap at
    /// min(viewport width, wordWrapColumn). Defaults to "off". "off" | "on"
    /// | "wordWrapColumn" | "bounded"
    pub fn word_wrap(self, val: &str) -> Self {
        object_set!(self.wordWrap = val);
        self
    }

    /// Configure word wrapping characters. A break will be introduced after
    /// these characters. Defaults to '
    /// \t})]?|/&.,;¢°′″‰℃、。｡､￠，．：；？！％・･
    /// ゝゞヽヾーァィゥェォッャュョヮヵヶぁぃぅぇぉっゃゅょゎゕゖㇰㇱㇲㇳㇴㇵㇶㇷㇸㇹㇺㇻㇼㇽㇾㇿ々〻ｧｨｩｪｫｬｭｮｯｰ”〉》」』】〕）］｝｣'
    /// .
    pub fn word_wrap_break_after_characters(self, val: &str) -> Self {
        object_set!(self.wordWrapBreakAfterCharacters = val);
        self
    }

    /// Configure word wrapping characters. A break will be introduced before
    /// these characters. Defaults to '([{‘“〈《「『【〔（［｛｢£¥＄￡￥+＋'.
    pub fn word_wrap_break_before_characters(self, val: &str) -> Self {
        object_set!(self.wordWrapBreakBeforeCharacters = val);
        self
    }

    /// Control the wrapping of the editor. When wordWrap = "off", the lines
    /// will never wrap. When wordWrap = "on", the lines will wrap at the
    /// viewport width. When wordWrap = "wordWrapColumn", the lines will wrap at
    /// wordWrapColumn. When wordWrap = "bounded", the lines will wrap at
    /// min(viewport width, wordWrapColumn). Defaults to 80.
    pub fn word_wrap_column(self, val: u32) -> Self {
        object_set!(self.wordWrapColumn = val);
        self
    }

    /// Force word wrapping when the text appears to be of a minified/generated
    /// file. Defaults to true.
    pub fn word_wrap_minified(self, val: bool) -> Self {
        object_set!(self.wordWrapMinified = val);
        self
    }

    /// Control indentation of wrapped lines. Can be: 'none', 'same', 'indent'
    /// or 'deepIndent'. Defaults to 'same' in vscode and to 'none' in
    /// monaco-editor. "none" | "same" | "indent" | "deepIndent"
    pub fn wrapping_indent(self, val: &str) -> Self {
        object_set!(self.wrappingIndent = val);
        self
    }

    /// Controls the wrapping strategy to use. Defaults to 'simple'.
    /// "simple" | "advanced"
    pub fn wrapping_strategy(self, val: &str) -> Self {
        object_set!(self.wrappingStrategy = val);
        self
    }
}
impl Default for IEditorOptions {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

// TODO: IDiffEditorOptions

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = IEditorOptions)]
    pub type IEditorConstructionOptions;
}
impl IEditorConstructionOptions {
    pub fn with_editor_options(self, val: &IEditorOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }

    /// The initial editor dimension (to avoid measuring the container).
    pub fn dimension(self, dimension: IDimension) -> Self {
        object_set!(self.dimension = dimension);
        self
    }
}
impl Default for IEditorConstructionOptions {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = Object)]
    pub type IDimension;
}
impl IDimension {
    pub fn height(self, val: u32) -> Self {
        object_set!(self.height = val);
        self
    }

    pub fn width(self, val: u32) -> Self {
        object_set!(self.width = val);
        self
    }
}
impl Default for IDimension {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

#[wasm_bindgen]
extern "C" {
    /// The options to create an editor.
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = IEditorConstructionOptions, extends = IGlobalEditorOptions)]
    pub type IStandaloneEditorConstructionOptions;
}
impl IStandaloneEditorConstructionOptions {
    pub fn with_editor_construction_options(self, val: &IEditorConstructionOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }

    pub fn with_global_editor_options(self, val: &IGlobalEditorOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }

    /// An URL to open when Ctrl+H (Windows and Linux) or Cmd+H (OSX) is pressed
    /// in the accessibility help dialog in the editor. Defaults to "https://go.microsoft.com/fwlink/?linkid=852450"
    pub fn accessibility_help_url(self, val: &str) -> Self {
        object_set!(self.accessibilityHelpUrl = val);
        self
    }

    /// The initial language of the auto created model in the editor. To not
    /// create automatically a model, use model: null.
    pub fn language(self, val: bool) -> Self {
        object_set!(self.language = val);
        self
    }

    /// The initial model associated with this code editor.
    pub fn model(self, val: ITextModel) -> Self {
        object_set!(self.model = val);
        self
    }

    /// Initial theme to be used for rendering. The current out-of-the-box
    /// available themes are: 'vs' (default), 'vs-dark', 'hc-black'. You can
    /// create custom themes via monaco.editor.defineTheme. To switch a theme,
    /// use monaco.editor.setTheme
    pub fn theme(self, val: &str) -> Self {
        object_set!(self.theme = val);
        self
    }

    /// The initial value of the auto created model in the editor. To not create
    /// automatically a model, use model: null.
    pub fn value(self, val: &str) -> Self {
        object_set!(self.value = val);
        self
    }
}
impl Default for IStandaloneEditorConstructionOptions {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[wasm_bindgen(extends = Object)]
    pub type ITextModel;
}
impl ITextModel {
    /// A unique identifier associated with this model.
    pub fn id(&self) -> Option<String> {
        object_get!(self.id)
            .ok()
            .as_ref()
            .and_then(JsValue::as_string)
    }

    pub fn set_id(&self, val: &str) {
        object_set!(self.id = val);
    }

    /// Gets the resource associated with this editor model.
    pub fn set_uri(&self, val: Uri) {
        object_set!(self.uri = val);
    }
}
impl Default for ITextModel {
    fn default() -> Self {
        JsCast::unchecked_into(Object::new())
    }
}

// TODO: Uri
