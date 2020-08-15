use super::enums::*;
use crate::sys::Uri;
use js_sys::{Object, Uint32Array};
use wasm_bindgen::{prelude::*, JsCast};

define_object_interface! {
    /// Dimension with width and height
    type IDimension;
}
impl IDimension {
    define_property! {
        /// Height in pixels
        height: Option<f64>
    }

    define_property! {
        /// Width in pixels
        width: Option<f64>
    }
}

define_object_interface! {
    /// Configuration options for editor comments
    type IEditorCommentsOptions;
}
impl IEditorCommentsOptions {
    define_property! {
        /// Insert a space after the line comment token and inside the block comments tokens. Defaults to true.
        insertSpace: Option<bool>
    }
}

define_object_interface! {
    /// Options to create an editor
    type IEditorConstructionOptions extends IEditorOptions;
}
impl IEditorConstructionOptions {
    define_property! {
        /// The initial editor dimension (to avoid measuring the container).
        ref dimension: Option<IDimension>
    }

    /// Add the given `IEditorOptions`.
    pub fn with_editor_options(self, val: &IEditorOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }
}

define_object_interface! {
    /// Configuration options for editor find widget
    type IEditorFindOptions;
}
impl IEditorFindOptions {
    define_property! {
        /// Add extra space on top.
        addExtraSpaceOnTop: Option<bool>
    }

    define_property! {
        /// Controls if Find in Selection flag is turned on in the editor.
        enum autoFindInSelection: Option<AutoFindInSelection>
    }

    define_property! {
        /// Controls if we seed search string in the Find Widget with editor selection.
        seedSearchStringFromSelection: Option<bool>
    }
}

define_object_interface! {
    /// Configuration options for the editor.
    type IEditorOptions;
}
impl IEditorOptions {
    define_property! {
        /// Accept suggestions on provider defined characters. Defaults to true.
        acceptSuggestionOnCommitCharacter: Option<bool>
    }

    define_property! {
        /// Accept suggestions on ENTER. Defaults to 'on'.
        enum acceptSuggestionOnEnter: Option<AcceptSuggestionOnEnter>
    }

    define_property! {
        /// Controls the number of lines in the editor that can be read out by a
        /// screen reader
        accessibilityPageSize: Option<f64>
    }

    define_property! {
        /// Configure the editor's accessibility support. Defaults to 'auto'. It is
        /// best to leave this to 'auto'.
        enum accessibilitySupport: Option<AccessibilitySupport>
    }

    define_property! {
        /// The aria label for the editor's textarea (when it is focused).
        ariaLabel: Option<String>
    }

    define_property! {
        /// Options for auto closing brackets. Defaults to language defined
        /// behavior.
        enum autoClosingBrackets: Option<AutoClosingStrategy>
    }

    define_property! {
        /// Options for typing over closing quotes or brackets.
        enum autoClosingOvertype: Option<AutoClosingOvertype>
    }

    define_property! {
        /// Options for auto closing quotes. Defaults to language defined behavior.
        enum autoClosingQuotes: Option<AutoClosingStrategy>
    }

    define_property! {
        /// Controls whether the editor should automatically adjust the indentation
        /// when users type, paste, move or indent lines. Defaults to advanced.
        enum autoIndent: Option<AutoIdent>
    }

    define_property! {
        /// Options for auto surrounding. Defaults to always allowing auto
        /// surrounding.
        enum autoSurround: Option<AutoSurroundStrategy>
    }

    define_property! {
        /// Enable that the editor will install an interval to check if its
        /// container dom node size has changed. Enabling this might have a severe
        /// performance impact. Defaults to false.
        automaticLayout: Option<bool>
    }

    define_property! {
        /// Timeout for running code actions on save.
        codeActionsOnSaveTimeout: Option<f64>
    }

    define_property! {
        /// Show code lens Defaults to true.
        codeLens: Option<bool>
    }

    define_property! {
        /// Enable inline color decorators and color picker rendering.
        colorDecorators: Option<bool>
    }

    define_property! {
        /// Control the behaviour of comments in the editor.
        ref comments: Option<IEditorCommentsOptions>
    }

    define_property! {
        /// Enable custom contextmenu. Defaults to true.
        contextmenu: Option<bool>
    }

    define_property! {
        /// Syntax highlighting is copied.
        copyWithSyntaxHighlighting: Option<bool>
    }

    define_property! {
        /// Control the cursor animation style, possible values are 'blink',
        /// 'smooth', 'phase', 'expand' and 'solid'. Defaults to 'blink'.
        enum cursorBlinking: Option<CursorBlinkingStyle>
    }

    define_property! {
        /// Enable smooth caret animation. Defaults to false.
        cursorSmoothCaretAnimation: Option<bool>
    }

    define_property! {
        /// Control the cursor style, either 'block' or 'line'. Defaults to 'line'.
        enum cursorStyle: Option<CursorStyle>
    }

    define_property! {
        /// Controls the minimal number of visible leading and trailing lines
        /// surrounding the cursor. Defaults to 0.
        cursorSurroundingLines: Option<f64>
    }

    define_property! {
        /// Controls when cursorSurroundingLines should be enforced Defaults to
        /// default, cursorSurroundingLines is not enforced when cursor position is
        /// changed by mouse.
        enum cursorSurroundingLinesStyle: Option<CursorSurroundingLinesStyle>
    }

    define_property! {
        /// Control the width of the cursor when cursorStyle is set to 'line'
        cursorWidth: Option<f64>
    }

    define_property! {
        /// Disable the use of transform: translate3d(0px, 0px, 0px) for the editor
        /// margin and lines layers. The usage of transform: translate3d(0px, 0px,
        /// 0px) acts as a hint for browsers to create an extra layer. Defaults to
        /// false.
        disableLayerHinting: Option<bool>
    }

    define_property! {
        /// Disable the optimizations for monospace fonts. Defaults to false.
        disableMonospaceOptimizations: Option<bool>
    }

    define_property! {
        /// Controls if the editor should allow to move selections via drag and
        /// drop. Defaults to false.
        dragAndDrop: Option<bool>
    }

    define_property! {
        /// Copying without a selection copies the current line.
        emptySelectionClipboard: Option<bool>
    }

    define_property! {
        /// Class name to be added to the editor.
        extraEditorClassName: Option<String>
    }

    define_property! {
        /// FastScrolling mulitplier speed when pressing Alt Defaults to 5.
        fastScrollSensitivity: Option<f64>
    }

    define_property! {
        /// Control the behavior of the find widget.
        ref find: Option<IEditorFindOptions>
    }

    define_property! {
        /// Display overflow widgets as fixed. Defaults to false.
        fixedOverflowWidgets: Option<bool>
    }

    define_property! {
        /// Enable code folding. Defaults to true.
        folding: Option<bool>
    }

    define_property! {
        /// Enable highlight for folded regions. Defaults to true.
        foldingHighlight: Option<bool>
    }

    define_property! {
        /// Selects the folding strategy. 'auto' uses the strategies contributed for
        /// the current document, 'indentation' uses the indentation based folding
        /// strategy. Defaults to 'auto'.
        enum foldingStrategy: Option<FoldingStrategy>
    }

    define_property! {
        /// The font family
        fontFamily: Option<String>
    }

    define_property! {
        /// Enable font ligatures. Defaults to false.
        // TODO this one is special: bool | string
        fontLigatures: Option<bool>
    }

    define_property! {
        /// The font size
        fontSize: Option<f64>
    }

    define_property! {
        /// The font weight
        fontWeight: Option<String>
    }

    define_property! {
        /// Enable format on paste. Defaults to false.
        formatOnPaste: Option<bool>
    }

    define_property! {
        /// Enable format on type. Defaults to false.
        formatOnType: Option<bool>
    }

    define_property! {
        /// Enable the rendering of the glyph margin. Defaults to true in vscode and
        /// to false in monaco-editor.
        glyphMargin: Option<bool>
    }

    define_property! {
        /// Optional hideCursorInOverviewRuler
        ref gotoLocation: Option<IGotoLocationOptions>
    }

    define_property! {
        /// Should the cursor be hidden in the overview ruler. Defaults to false.
        hideCursorInOverviewRuler: Option<bool>
    }

    define_property! {
        /// Enable highlighting of the active indent guide. Defaults to true.
        highlightActiveIndentGuide: Option<bool>
    }

    define_property! {
        /// Configure the editor's hover.
        ref hover: Option<IEditorHoverOptions>
    }

    define_property! {
        /// This editor is used inside a diff editor.
        inDiffEditor: Option<bool>
    }

    define_property! {
        /// The letter spacing
        letterSpacing: Option<f64>
    }

    define_property! {
        /// Control the behavior and rendering of the code action lightbulb.
        ref lightbulb: Option<IEditorLightbulbOptions>
    }

    define_property! {
        /// The width reserved for line decorations (in px). Line decorations are
        /// placed between line numbers and the editor content. You can pass in a
        /// string in the format floating point followed by "ch". e.g. 1.3ch.
        /// Defaults to 10.
        // TODO special value: f64 | string
        lineDecorationsWidth: Option<f64>
    }

    define_property! {
        /// The line height
        lineHeight: Option<f64>
    }

    define_property! {
        /// Control the rendering of line numbers. If it is a function, it will be
        /// invoked when rendering a line number and the return value will be
        /// rendered. Otherwise, if it is a truey, line numbers will be rendered
        /// normally (equivalent of using an identity function). Otherwise, line
        /// numbers will not be rendered. Defaults to on.
        enum lineNumbers: Option<LineNumbersType>
    }

    define_property! {
        /// Control the width of line numbers, by reserving horizontal space for
        /// rendering at least an amount of digits. Defaults to 5.
        lineNumbersMinChars: Option<f64>
    }

    define_property! {
        /// Enable detecting links and making them clickable. Defaults to true.
        links: Option<bool>
    }

    define_property! {
        /// Enable highlighting of matching brackets. Defaults to 'always'.
        enum matchBrackets: Option<MatchBrackets>
    }

    define_property! {
        /// Control the behavior and rendering of the minimap.
        ref minimap: Option<IEditorMinimapOptions>
    }

    define_property! {
        /// Control the mouse pointer style, either 'text' or 'default' or 'copy'
        /// Defaults to 'text'.
        enum mouseStyle: Option<MouseStyle>
    }

    define_property! {
        /// A multiplier to be used on the deltaX and deltaY of mouse wheel scroll
        /// events. Defaults to 1.
        mouseWheelScrollSensitivity: Option<f64>
    }

    define_property! {
        /// Zoom the font in the editor when using the mouse wheel in combination
        /// with holding Ctrl. Defaults to false.
        mouseWheelZoom: Option<bool>
    }

    define_property! {
        /// Merge overlapping selections. Defaults to true
        multiCursorMergeOverlapping: Option<bool>
    }

    define_property! {
        /// The modifier to be used to add multiple cursors with the mouse. Defaults
        /// to 'alt'.
        enum multiCursorModifier: Option<MultiCursorModifier>
    }

    define_property! {
        /// Configure the behaviour when pasting a text with the line count equal to
        /// the cursor count. Defaults to 'spread'.
        enum multiCursorPaste: Option<MultiCursorPaste>
    }

    define_property! {
        /// Enable semantic occurrences highlight. Defaults to true.
        occurrencesHighlight: Option<bool>
    }

    define_property! {
        /// Controls if a border should be drawn around the overview ruler. Defaults
        /// to true.
        overviewRulerBorder: Option<bool>
    }

    define_property! {
        /// The number of vertical lanes the overview ruler should render. Defaults
        /// to 3.
        overviewRulerLanes: Option<f64>
    }

    define_property! {
        /// Parameter hint options.
        ref parameterHints: Option<IEditorParameterHintOptions>
    }

    define_property! {
        /// Controls whether to focus the inline editor in the peek widget by
        /// default. Defaults to false.
        enum peekWidgetDefaultFocus: Option<PeekWidgetDefaultFocus>
    }

    define_property! {
        /// Enable quick suggestions (shadow suggestions) Defaults to true.
        // TODO: special value: bool | IQuickSuggestionsOptions
        quickSuggestions: Option<bool>
    }

    define_property! {
        /// Quick suggestions show delay (in ms) Defaults to 10 (ms)
        quickSuggestionsDelay: Option<f64>
    }

    define_property! {
        /// Should the editor be read only. Defaults to false.
        readOnly: Option<bool>
    }

    define_property! {
        /// Enable rendering of control characters. Defaults to false.
        renderControlCharacters: Option<bool>
    }

    define_property! {
        /// Render last line number when the file ends with a newline. Defaults to
        /// true.
        renderFinalNewline: Option<bool>
    }

    define_property! {
        /// Enable rendering of indent guides. Defaults to true.
        renderIndentGuides: Option<bool>
    }

    define_property! {
        /// Enable rendering of current line highlight. Defaults to all.
        enum renderLineHighlight: Option<RenderLineHighlight>
    }

    define_property! {
        /// Should the editor render validation decorations. Defaults to editable.
        enum renderValidationDecorations: Option<RenderValidationDecorations>
    }

    define_property! {
        /// Enable rendering of whitespace. Defaults to none.
        enum renderWhitespace: Option<RenderWhitespace>
    }

    define_property! {
        /// When revealing the cursor, a virtual padding (px) is added to the
        /// cursor, turning it into a rectangle. This virtual padding ensures that
        /// the cursor gets revealed before hitting the edge of the viewport.
        /// Defaults to 30 (px).
        revealHorizontalRightPadding: Option<f64>
    }

    define_property! {
        /// Render the editor selection with rounded borders. Defaults to true.
        roundedSelection: Option<bool>
    }

    define_property! {
        /// Render vertical lines at the specified columns. Defaults to empty array.
        ref rulers: Option<Uint32Array>
    }

    define_property! {
        /// Enable that scrolling can go beyond the last column by a number of
        /// columns. Defaults to 5.
        scrollBeyondLastColumn: Option<f64>
    }

    define_property! {
        /// Enable that scrolling can go one screen size after the last line.
        /// Defaults to true.
        scrollBeyondLastLine: Option<bool>
    }

    define_property! {
        /// Control the behavior and rendering of the scrollbars.
        ref scrollbar: Option<IEditorScrollbarOptions>
    }

    define_property! {
        /// Should the corresponding line be selected when clicking on the line
        /// number? Defaults to true.
        selectOnLineNumbers: Option<bool>
    }

    define_property! {
        /// Enable Linux primary clipboard. Defaults to true.
        selectionClipboard: Option<bool>
    }

    define_property! {
        /// Enable selection highlight. Defaults to true.
        selectionHighlight: Option<bool>
    }

    define_property! {
        /// Controls whether the fold actions in the gutter stay always visible or
        /// hide unless the mouse is over the gutter. Defaults to 'mouseover'.
        enum showFoldingControls: Option<ShowFoldingControls>
    }

    define_property! {
        /// Controls fading out of unused variables.
        showUnused: Option<bool>
    }

    define_property! {
        /// Enable that the editor animates scrolling to a position. Defaults to
        /// false.
        smoothScrolling: Option<bool>
    }

    define_property! {
        /// Enable snippet suggestions. Default to 'true'.
        enum snippetSuggestions: Option<SnippetSuggestions>
    }

    define_property! {
        /// Performance guard: Stop rendering a line after x characters. Defaults to
        /// 10000. Use -1 to never stop rendering
        stopRenderingLineAfter: Option<f64>
    }

    define_property! {
        /// Suggest options.
        ref suggest: Option<ISuggestOptions>
    }

    define_property! {
        /// The font size for the suggest widget. Defaults to the editor font size.
        suggestFontSize: Option<f64>
    }

    define_property! {
        /// The line height for the suggest widget. Defaults to the editor line
        /// height.
        suggestLineHeight: Option<f64>
    }

    define_property! {
        /// Enable the suggestion box to pop-up on trigger characters. Defaults to
        /// true.
        suggestOnTriggerCharacters: Option<bool>
    }

    define_property! {
        /// The history mode for suggestions.
        enum suggestSelection: Option<SuggestSelection>
    }

    define_property! {
        /// Enable tab completion.
        enum tabCompletion: Option<TabCompletion>
    }

    define_property! {
        /// Inserting and deleting whitespace follows tab stops.
        useTabStops: Option<bool>
    }

    define_property! {
        /// A string containing the word separators used when doing word navigation.
        /// Defaults to `~!@#$%^&*()-=+[{]}\|;:'",.<>/?
        wordSeparators: Option<String>
    }

    define_property! {
        /// Control the wrapping of the editor. When wordWrap = "off", the lines
        /// will never wrap. When wordWrap = "on", the lines will wrap at the
        /// viewport width. When wordWrap = "wordWrapColumn", the lines will wrap at
        /// wordWrapColumn. When wordWrap = "bounded", the lines will wrap at
        /// min(viewport width, wordWrapColumn). Defaults to "off".
        enum wordWrap: Option<WordWrap>
    }

    define_property! {
        /// Configure word wrapping characters. A break will be introduced after
        /// these characters. Defaults to '
        /// \t})]?|/&.,;¢°′″‰℃、。｡､￠，．：；？！％・･
        /// ゝゞヽヾーァィゥェォッャュョヮヵヶぁぃぅぇぉっゃゅょゎゕゖㇰㇱㇲㇳㇴㇵㇶㇷㇸㇹㇺㇻㇼㇽㇾㇿ々〻ｧｨｩｪｫｬｭｮｯｰ”〉》」』】〕）］｝｣'
        /// .
        wordWrapBreakAfterCharacters: Option<String>
    }

    define_property! {
        /// Configure word wrapping characters. A break will be introduced before
        /// these characters. Defaults to '([{‘“〈《「『【〔（［｛｢£¥＄￡￥+＋'.
        wordWrapBreakBeforeCharacters: Option<String>
    }

    define_property! {
        /// Control the wrapping of the editor. When wordWrap = "off", the lines
        /// will never wrap. When wordWrap = "on", the lines will wrap at the
        /// viewport width. When wordWrap = "wordWrapColumn", the lines will wrap at
        /// wordWrapColumn. When wordWrap = "bounded", the lines will wrap at
        /// min(viewport width, wordWrapColumn). Defaults to 80.
        wordWrapColumn: Option<f64>
    }

    define_property! {
        /// Force word wrapping when the text appears to be of a minified/generated
        /// file. Defaults to true.
        wordWrapMinified: Option<bool>
    }

    define_property! {
        /// Control indentation of wrapped lines. Can be: 'none', 'same', 'indent'
        /// or 'deepIndent'. Defaults to 'same' in vscode and to 'none' in
        /// monaco-editor.
        enum wrappingIndent: Option<WrappingIndent>
    }

    define_property! {
        /// Controls the wrapping strategy to use. Defaults to 'simple'.
        enum wrappingStrategy: Option<WrappingStrategy>
    }
}

define_object_interface! {
    /// Configuration options for editor hover
    type IEditorHoverOptions;
}
impl IEditorHoverOptions {
    define_property! {
        /// Delay for showing the hover. Defaults to 300.
        delay: Option<f64>
    }

    define_property! {
        /// Enable the hover. Defaults to true.
        enabled: Option<bool>
    }

    define_property! {
        /// Is the hover sticky such that it can be clicked and its contents selected? Defaults to true.
        sticky: Option<bool>
    }
}

define_object_interface! {
    /// Configuration options for editor lightbulb
    type IEditorLightbulbOptions;
}
impl IEditorLightbulbOptions {
    define_property! {
        /// Enable the lightbulb code action. Defaults to true.
        enabled: Option<bool>
    }
}

define_object_interface! {
    /// Configuration options for editor minimap
    type IEditorMinimapOptions;
}
impl IEditorMinimapOptions {
    define_property! {
        /// Enable the rendering of the minimap. Defaults to true.
        enabled: Option<bool>
    }

    define_property! {
        /// Limit the width of the minimap to render at most a certain number of columns. Defaults to 120.
        maxColumn: Option<f64>
    }

    define_property! {
        /// Render the actual text on a line (as opposed to color blocks). Defaults to true.
        renderCharacters: Option<bool>
    }

    define_property! {
        /// Relative size of the font in the minimap. Defaults to 1.
        scale: Option<f64>
    }

    define_property! {
        /// Control the rendering of the minimap slider. Defaults to 'mouseover'.
        enum showSlider: Option<MinimapShowSlider>
    }

    define_property! {
        /// Control the side of the minimap in editor. Defaults to 'right'.
        enum side: Option<MinimapSide>
    }
}

define_object_interface! {
    /// Configuration options for parameter hints
    type IEditorParameterHintOptions;
}
impl IEditorParameterHintOptions {
    define_property! {
        /// Enable cycling of parameter hints. Defaults to false.
        cycle: Option<bool>
    }

    define_property! {
        /// Enable parameter hints. Defaults to true.
        enabled: Option<bool>
    }
}

define_object_interface! {
    /// Configuration options for editor scrollbars
    type IEditorScrollbarOptions;
}
impl IEditorScrollbarOptions {
    define_property! {
        /// Always consume mouse wheel events (always call preventDefault() and stopPropagation() on the browser events). Defaults to true.
        alwaysConsumeMouseWheel: Option<bool>
    }

    define_property! {
        /// The size of arrows (if displayed). Defaults to 11.
        arrowSize: Option<f64>
    }

    define_property! {
        /// Listen to mouse wheel events and react to them by scrolling. Defaults to true.
        handleMouseWheel: Option<bool>
    }

    define_property! {
        /// Render horizontal scrollbar. Defaults to 'auto'.
        enum horizontal: Option<ScrollbarVisible>
    }

    define_property! {
        /// Render arrows at the left and right of the horizontal scrollbar. Defaults to false.
        horizontalHasArrows: Option<bool>
    }

    define_property! {
        /// Height in pixels for the horizontal scrollbar. Defaults to 10 (px).
        horizontalScrollbarSize: Option<f64>
    }

    define_property! {
        /// Height in pixels for the horizontal slider. Defaults to horizontalScrollbarSize.
        horizontalSliderSize: Option<f64>
    }

    define_property! {
        /// Cast horizontal and vertical shadows when the content is scrolled. Defaults to true.
        useShadows: Option<bool>
    }

    define_property! {
        /// Render vertical scrollbar. Defaults to 'auto'.
        enum vertical: Option<ScrollbarVisible>
    }

    define_property! {
        /// Render arrows at the top and bottom of the vertical scrollbar. Defaults to false.
        verticalHasArrows: Option<bool>
    }

    define_property! {
        /// Width in pixels for the vertical scrollbar. Defaults to 10 (px).
        verticalScrollbarSize: Option<f64>
    }

    define_property! {
        /// Width in pixels for the vertical slider. Defaults to verticalScrollbarSize.
        verticalSliderSize: Option<f64>
    }
}

define_object_interface! {
    /// Options which apply for all editors.
    type IGlobalEditorOptions;
}
impl IGlobalEditorOptions {
    define_property! {
        /// Controls whether tabSize and insertSpaces will be automatically detected
        /// when a file is opened based on the file contents. Defaults to true.
        detectIndentation: Option<bool>
    }

    define_property! {
        /// Insert spaces when pressing Tab. This setting is overridden based on the
        /// file contents when detectIndentation is on. Defaults to true.
        insertSpaces: Option<bool>
    }

    define_property! {
        /// Special handling for large files to disable certain memory intensive
        /// features. Defaults to true.
        largeFileOptimizations: Option<bool>
    }

    define_property! {
        /// Lines above this length will not be tokenized for performance reasons.
        /// Defaults to 20000.
        maxTokenizationLineLength: Option<bool>
    }

    define_property! {
        /// Keep peek editors open even when double clicking their content or when
        /// hitting Escape. Defaults to false.
        stablePeek: Option<bool>
    }

    define_property! {
        /// The number of spaces a tab is equal to. This setting is overridden based
        /// on the file contents when detectIndentation is on. Defaults to 4.
        tabSize: Option<f64>
    }

    define_property! {
        /// Remove trailing auto inserted whitespace. Defaults to true.
        trimAutoWhitespace: Option<bool>
    }

    define_property! {
        /// Controls whether completions should be computed based on words in the
        /// document. Defaults to true.
        wordBasedSuggestions: Option<bool>
    }
}

define_object_interface! {
    /// Configuration options for go to location
    type IGotoLocationOptions;
}
impl IGotoLocationOptions {
    define_property! {
        /// Alternative declaration command
        alternativeDeclarationCommand: Option<String>
    }

    define_property! {
        /// Alternative definition command
        alternativeDefinitionCommand: Option<String>
    }

    define_property! {
        /// Alternative implementation command
        alternativeImplementationCommand: Option<String>
    }

    define_property! {
        /// Alternative reference command
        alternativeReferenceCommand: Option<String>
    }

    define_property! {
        /// Alternative type definition command
        alternativeTypeDefinitionCommand: Option<String>
    }

    define_property! {
        /// Multiple
        enum multiple: Option<GoToLocationValues>
    }

    define_property! {
        /// Multiple declarations
        enum multipleDeclarations: Option<GoToLocationValues>
    }

    define_property! {
        /// Multiple definitions
        enum multipleDefinitions: Option<GoToLocationValues>
    }

    define_property! {
        /// Multiple implementations
        enum multipleImplementations: Option<GoToLocationValues>
    }

    define_property! {
        /// Multiple references
        enum multipleReferences: Option<GoToLocationValues>
    }

    define_property! {
        /// Multiple type definitions
        enum multipleTypeDefinitions: Option<GoToLocationValues>
    }
}

define_object_interface! {
    /// The options to create an editor
    type IStandaloneEditorConstructionOptions extends IEditorConstructionOptions, IGlobalEditorOptions;
}
impl IStandaloneEditorConstructionOptions {
    define_property! {
        /// An URL to open when Ctrl+H (Windows and Linux) or Cmd+H (OSX) is pressed
        /// in the accessibility help dialog in the editor. Defaults to "https://go.microsoft.com/fwlink/?linkid=852450"
        accessibilityHelpUrl: Option<String>
    }

    define_property! {
        /// The initial language of the auto created model in the editor. To not
        /// create automatically a model, use model: null.
        language: Option<String>
    }

    define_property! {
        /// The initial model associated with this code editor.
        ref model: Option<ITextModel>
    }

    define_property! {
        /// Initial theme to be used for rendering. The current out-of-the-box
        /// available themes are: 'vs' (default), 'vs-dark', 'hc-black'. You can
        /// create custom themes via monaco.editor.defineTheme. To switch a theme,
        /// use monaco.editor.setTheme
        // TODO allow setting custom theme
        enum theme: Option<BuiltinTheme>
    }

    define_property! {
        /// The initial value of the auto created model in the editor. To not create
        /// automatically a model, use model: null.
        value: Option<String>
    }

    /// Add the given `IEditorConstructionOptions`.
    pub fn with_editor_construction_options(self, val: &IEditorConstructionOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }

    /// Add the options from `IGlobalEditorOptions`.
    pub fn with_global_editor_options(self, val: &IGlobalEditorOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }
}

define_object_interface! {
    /// Configuration options for editor suggest widget
    type ISuggestOptions;
}
impl ISuggestOptions {
    define_property! {
        /// Enable graceful matching. Defaults to true.
        filterGraceful: Option<bool>
    }

    define_property! {
        /// Controls the visibility of the status bar at the bottom of the suggest widget.
        hideStatusBar: Option<bool>
    }

    define_property! {
        /// Show a highlight when suggestion replaces or keep text after the cursor. Defaults to false.
        insertHighlight: Option<bool>
    }

    define_property! {
        /// Overwrite word ends on accept. Default to false.
        enum insertMode: Option<SuggestInsertMode>
    }

    define_property! {
        /// Favours words that appear close to the cursor.
        localityBonus: Option<bool>
    }

    define_property! {
        /// Max suggestions to show in suggestions. Defaults to 12.
        maxVisibleSuggestions: Option<f64>
    }

    define_property! {
        /// Enable using global storage for remembering suggestions.
        shareSuggestSelections: Option<bool>
    }

    define_property! {
        /// Show class-suggestions.
        showClasses: Option<bool>
    }

    define_property! {
        /// Show color-suggestions.
        showColors: Option<bool>
    }

    define_property! {
        /// Show constant-suggestions.
        showConstants: Option<bool>
    }

    define_property! {
        /// Show constructor-suggestions.
        showConstructors: Option<bool>
    }

    define_property! {
        /// Show enumMember-suggestions.
        showEnumMembers: Option<bool>
    }

    define_property! {
        /// Show enum-suggestions.
        showEnums: Option<bool>
    }

    define_property! {
        /// Show event-suggestions.
        showEvents: Option<bool>
    }

    define_property! {
        /// Show field-suggestions.
        showFields: Option<bool>
    }

    define_property! {
        /// Show file-suggestions.
        showFiles: Option<bool>
    }

    define_property! {
        /// Show folder-suggestions.
        showFolders: Option<bool>
    }

    define_property! {
        /// Show function-suggestions.
        showFunctions: Option<bool>
    }

    define_property! {
        /// Enable or disable icons in suggestions. Defaults to true.
        showIcons: Option<bool>
    }

    define_property! {
        /// Show interface-suggestions.
        showInterfaces: Option<bool>
    }

    define_property! {
        /// Show keyword-suggestions.
        showKeywords: Option<bool>
    }

    define_property! {
        /// Show method-suggestions.
        showMethods: Option<bool>
    }

    define_property! {
        /// Show module-suggestions.
        showModules: Option<bool>
    }

    define_property! {
        /// Show operator-suggestions.
        showOperators: Option<bool>
    }

    define_property! {
        /// Show property-suggestions.
        showProperties: Option<bool>
    }

    define_property! {
        /// Show reference-suggestions.
        showReferences: Option<bool>
    }

    define_property! {
        /// Show snippet-suggestions.
        showSnippets: Option<bool>
    }

    define_property! {
        /// Show struct-suggestions.
        showStructs: Option<bool>
    }

    define_property! {
        /// Show typeParameter-suggestions.
        showTypeParameters: Option<bool>
    }

    define_property! {
        /// Show unit-suggestions.
        showUnits: Option<bool>
    }

    define_property! {
        /// Show value-suggestions.
        showValues: Option<bool>
    }

    define_property! {
        /// Show variable-suggestions.
        showVariables: Option<bool>
    }

    define_property! {
        /// Show text-suggestions.
        showWords: Option<bool>
    }

    define_property! {
        /// Prevent quick suggestions when a snippet is active. Defaults to true.
        snippetsPreventQuickSuggestions: Option<bool>
    }
}

define_object_interface! {
    /// A model.
    type ITextModel;
}
impl ITextModel {
    define_property! {
        /// Model id.
        id: Option<String>
    }

    define_property! {
        /// Gets the resource associated with this editor model.
        ref uri: Option<Uri>
    }
}
