use super::enums::*;
use crate::sys::Uri;
use js_sys::{Object, Uint32Array};
use wasm_bindgen::{prelude::*, JsCast};

define_interface_builder! {
    /// Options which apply for all editors.
    type IGlobalEditorOptions extends Object {
        /// Controls whether tabSize and insertSpaces will be automatically detected
        /// when a file is opened based on the file contents. Defaults to true.
        detect_indentation: bool => detectIndentation;
        /// Insert spaces when pressing Tab. This setting is overridden based on the
        /// file contents when detectIndentation is on. Defaults to true.
        insert_spaces: bool => insertSpaces;
        /// Special handling for large files to disable certain memory intensive
        /// features. Defaults to true.
        large_file_optimizations: bool => largeFileOptimizations;
        /// Lines above this length will not be tokenized for performance reasons.
        /// Defaults to 20000.
        max_tokenization_line_length: u32 => maxTokenizationLineLength;
        /// Keep peek editors open even when double clicking their content or when
        /// hitting Escape. Defaults to false.
        stable_peek: bool => stablePeek;
        /// The number of spaces a tab is equal to. This setting is overridden based
        /// on the file contents when detectIndentation is on. Defaults to 4.
        tab_size: u32 => tabSize;
        /// Remove trailing auto inserted whitespace. Defaults to true.
        trim_auto_whitespace: bool => trimAutoWhitespace;
        /// Controls whether completions should be computed based on words in the
        /// document. Defaults to true.
        word_based_suggestions: bool => wordBasedSuggestions;
    }
}

define_interface_builder! {
    /// Configuration options for the editor.
    type IEditorOptions extends Object {
        /// Accept suggestions on provider defined characters. Defaults to true.
        accept_suggestion_on_commit_character: bool => acceptSuggestionOnCommitCharacter;
        /// Accept suggestions on ENTER. Defaults to 'on'.
        accept_suggestion_on_enter: AcceptSuggestionOnEnter => acceptSuggestionOnEnter;
        /// Controls the number of lines in the editor that can be read out by a
        /// screen reader
        accessibility_page_size: u32 => accessibilityPageSize;
        /// Configure the editor's accessibility support. Defaults to 'auto'. It is
        /// best to leave this to 'auto'.
        accessibility_support: AccessibilitySupport => accessibilitySupport;
        /// The aria label for the editor's textarea (when it is focused).
        aria_label: &str => ariaLabel;
        /// Options for auto closing brackets. Defaults to language defined
        /// behavior.
        auto_closing_brackets: AutoClosingStrategy => autoClosingBrackets;
        /// Options for typing over closing quotes or brackets.
        auto_closing_overtype: AutoClosingOvertype => autoClosingOvertype;
        /// Options for auto closing quotes. Defaults to language defined behavior.
        auto_closing_quotes: AutoClosingStrategy => autoClosingQuotes;
        /// Controls whether the editor should automatically adjust the indentation
        /// when users type, paste, move or indent lines. Defaults to advanced.
        auto_indent: AutoIdent => autoIndent;
        /// Options for auto surrounding. Defaults to always allowing auto
        /// surrounding.
        auto_surround: AutoSurroundStrategy => autoSurround;
        /// Enable that the editor will install an interval to check if its
        /// container dom node size has changed. Enabling this might have a severe
        /// performance impact. Defaults to false.
        automatic_layout: bool => automaticLayout;
        /// Timeout for running code actions on save.
        code_actions_on_save_timeout: u32 => codeActionsOnSaveTimeout;
        /// Show code lens Defaults to true.
        code_lens: bool => codeLens;
        /// Enable inline color decorators and color picker rendering.
        color_decorators: bool => colorDecorators;
        /// Control the behaviour of comments in the editor.
        comments: &IEditorCommentsOptions => comments;
        /// Enable custom contextmenu. Defaults to true.
        contextmenu: bool => contextmenu;
        /// Syntax highlighting is copied.
        copy_with_syntax_highlighting: bool => copyWithSyntaxHighlighting;
        /// Control the cursor animation style, possible values are 'blink',
        /// 'smooth', 'phase', 'expand' and 'solid'. Defaults to 'blink'.
        cursor_blinking: CursorBlinkingStyle => cursorBlinking;
        /// Enable smooth caret animation. Defaults to false.
        cursor_smooth_caret_animation: bool => cursorSmoothCaretAnimation;
        /// Control the cursor style, either 'block' or 'line'. Defaults to 'line'.
        cursor_style: CursorStyle => cursorStyle;
        /// Controls the minimal number of visible leading and trailing lines
        /// surrounding the cursor. Defaults to 0.
        cursor_surrounding_lines: u32 => cursorSurroundingLines;
        /// Controls when cursorSurroundingLines should be enforced Defaults to
        /// default, cursorSurroundingLines is not enforced when cursor position is
        /// changed by mouse.
        cursor_surrounding_lines_style: CursorSurroundingLinesStyle => cursorSurroundingLinesStyle;
        /// Control the width of the cursor when cursorStyle is set to 'line'
        cursor_width: u32 => cursorWidth;
        /// Disable the use of transform: translate3d(0px, 0px, 0px) for the editor
        /// margin and lines layers. The usage of transform: translate3d(0px, 0px,
        /// 0px) acts as a hint for browsers to create an extra layer. Defaults to
        /// false.
        disable_layer_hinting: bool => disableLayerHinting;
        /// Disable the optimizations for monospace fonts. Defaults to false.
        disable_monospace_optimizations: bool => disableMonospaceOptimizations;
        /// Controls if the editor should allow to move selections via drag and
        /// drop. Defaults to false.
        drag_and_drop: bool => dragAndDrop;
        /// Copying without a selection copies the current line.
        empty_selection_clipboard: bool => emptySelectionClipboard;
        /// Class name to be added to the editor.
        extra_editor_class_name: &str => extraEditorClassName;
        /// FastScrolling mulitplier speed when pressing Alt Defaults to 5.
        fast_scroll_sensitivity: u32 => fastScrollSensitivity;
        /// Control the behavior of the find widget.
        find: &IEditorFindOptions => find;
        /// Display overflow widgets as fixed. Defaults to false.
        fixed_overflow_widgets: bool => fixedOverflowWidgets;
        /// Enable code folding. Defaults to true.
        folding: bool => folding;
        /// Enable highlight for folded regions. Defaults to true.
        folding_highlight: bool => foldingHighlight;
        /// Selects the folding strategy. 'auto' uses the strategies contributed for
        /// the current document, 'indentation' uses the indentation based folding
        /// strategy. Defaults to 'auto'.
        folding_strategy: FoldingStrategy => foldingStrategy;
        /// The font family
        font_family: &str => fontFamily;
        /// Enable font ligatures. Defaults to false.
        // TODO this one is special: bool | string
        font_ligatures: bool => fontLigatures;
        /// The font size
        font_size: u32 => fontSize;
        /// The font weight
        font_weight: &str => fontWeight;
        /// Enable format on paste. Defaults to false.
        format_on_paste: bool => formatOnPaste;
        /// Enable format on type. Defaults to false.
        format_on_type: bool => formatOnType;
        /// Enable the rendering of the glyph margin. Defaults to true in vscode and
        /// to false in monaco-editor.
        glyph_margin: bool => glyphMargin;
        /// Optional hideCursorInOverviewRuler
        goto_location: &IGotoLocationOptions => gotoLocation;
        /// Should the cursor be hidden in the overview ruler. Defaults to false.
        hide_cursor_in_overview_ruler: bool => hideCursorInOverviewRuler;
        /// Enable highlighting of the active indent guide. Defaults to true.
        highlight_active_indent_guide: bool => highlightActiveIndentGuide;
        /// Configure the editor's hover.
        hover: &IEditorHoverOptions => hover;
        /// This editor is used inside a diff editor.
        in_diff_editor: bool => inDiffEditor;
        /// The letter spacing
        letter_spacing: u32 => letterSpacing;
        /// Control the behavior and rendering of the code action lightbulb.
        lightbulb: &IEditorLightbulbOptions => lightbulb;
        /// The width reserved for line decorations (in px). Line decorations are
        /// placed between line numbers and the editor content. You can pass in a
        /// string in the format floating point followed by "ch". e.g. 1.3ch.
        /// Defaults to 10.
        // TODO special value: u32 | string
        line_decorations_width: u32 => lineDecorationsWidth;
        /// The line height
        line_height: u32 => lineHeight;
        /// Control the rendering of line numbers. If it is a function, it will be
        /// invoked when rendering a line number and the return value will be
        /// rendered. Otherwise, if it is a truey, line numbers will be rendered
        /// normally (equivalent of using an identity function). Otherwise, line
        /// numbers will not be rendered. Defaults to on.
        line_numbers: LineNumbersType => lineNumbers;
        /// Control the width of line numbers, by reserving horizontal space for
        /// rendering at least an amount of digits. Defaults to 5.
        line_numbers_min_chars: u32 => lineNumbersMinChars;
        /// Enable detecting links and making them clickable. Defaults to true.
        links: bool => links;
        /// Enable highlighting of matching brackets. Defaults to 'always'.
        match_brackets: MatchBrackets => matchBrackets;
        /// Control the behavior and rendering of the minimap.
        minimap: &IEditorMinimapOptions => minimap;
        /// Control the mouse pointer style, either 'text' or 'default' or 'copy'
        /// Defaults to 'text'.
        mouse_style: MouseStyle => mouseStyle;
        /// A multiplier to be used on the deltaX and deltaY of mouse wheel scroll
        /// events. Defaults to 1.
        mouse_wheel_scroll_sensitivity: u32 => mouseWheelScrollSensitivity;
        /// Zoom the font in the editor when using the mouse wheel in combination
        /// with holding Ctrl. Defaults to false.
        mouse_wheel_zoom: bool => mouseWheelZoom;
        /// Merge overlapping selections. Defaults to true
        multi_cursor_merge_overlapping: bool => multiCursorMergeOverlapping;
        /// The modifier to be used to add multiple cursors with the mouse. Defaults
        /// to 'alt'.
        multi_cursor_modifier: MultiCursorModifier => multiCursorModifier;
        /// Configure the behaviour when pasting a text with the line count equal to
        /// the cursor count. Defaults to 'spread'.
        multi_cursor_paste: MultiCursorPaste => multiCursorPaste;
        /// Enable semantic occurrences highlight. Defaults to true.
        occurrences_highlight: bool => occurrencesHighlight;
        /// Controls if a border should be drawn around the overview ruler. Defaults
        /// to true.
        overview_ruler_border: bool => overviewRulerBorder;
        /// The number of vertical lanes the overview ruler should render. Defaults
        /// to 3.
        overview_ruler_lanes: u32 => overviewRulerLanes;
        /// Parameter hint options.
        parameter_hints: &IEditorParameterHintOptions => parameterHints;
        /// Controls whether to focus the inline editor in the peek widget by
        /// default. Defaults to false.
        peek_widget_default_focus: PeekWidgetDefaultFocus => peekWidgetDefaultFocus;
        /// Enable quick suggestions (shadow suggestions) Defaults to true.
        // TODO: special value: bool | IQuickSuggestionsOptions
        quick_suggestions: bool => quickSuggestions;
        /// Quick suggestions show delay (in ms) Defaults to 10 (ms)
        quick_suggestions_delay: u32 => quickSuggestionsDelay;
        /// Should the editor be read only. Defaults to false.
        read_only: bool => readOnly;
        /// Enable rendering of control characters. Defaults to false.
        render_control_characters: bool => renderControlCharacters;
        /// Render last line number when the file ends with a newline. Defaults to
        /// true.
        render_final_newline: bool => renderFinalNewline;
        /// Enable rendering of indent guides. Defaults to true.
        render_indent_guides: bool => renderIndentGuides;
        /// Enable rendering of current line highlight. Defaults to all.
        render_line_highlight: RenderLineHighlight => renderLineHighlight;
        /// Should the editor render validation decorations. Defaults to editable.
        render_validation_decorations: RenderValidationDecorations => renderValidationDecorations;
        /// Enable rendering of whitespace. Defaults to none.
        render_whitespace: RenderWhitespace => renderWhitespace;
        /// When revealing the cursor, a virtual padding (px) is added to the
        /// cursor, turning it into a rectangle. This virtual padding ensures that
        /// the cursor gets revealed before hitting the edge of the viewport.
        /// Defaults to 30 (px).
        reveal_horizontal_right_padding: u32 => revealHorizontalRightPadding;
        /// Render the editor selection with rounded borders. Defaults to true.
        rounded_selection: bool => roundedSelection;
        /// Render vertical lines at the specified columns. Defaults to empty array.
        rulers: &Uint32Array => rulers;
        /// Enable that scrolling can go beyond the last column by a number of
        /// columns. Defaults to 5.
        scroll_beyond_last_column: u32 => scrollBeyondLastColumn;
        /// Enable that scrolling can go one screen size after the last line.
        /// Defaults to true.
        scroll_beyond_last_line: bool => scrollBeyondLastLine;
        /// Control the behavior and rendering of the scrollbars.
        scrollbar: &IEditorScrollbarOptions => scrollbar;
        /// Should the corresponding line be selected when clicking on the line
        /// number? Defaults to true.
        select_on_line_numbers: bool => selectOnLineNumbers;
        /// Enable Linux primary clipboard. Defaults to true.
        selection_clipboard: bool => selectionClipboard;
        /// Enable selection highlight. Defaults to true.
        selection_highlight: bool => selectionHighlight;
        /// Controls whether the fold actions in the gutter stay always visible or
        /// hide unless the mouse is over the gutter. Defaults to 'mouseover'.
        show_folding_controls: ShowFoldingControls => showFoldingControls;
        /// Controls fading out of unused variables.
        show_unused: bool => showUnused;
        /// Enable that the editor animates scrolling to a position. Defaults to
        /// false.
        smooth_scrolling: bool => smoothScrolling;
        /// Enable snippet suggestions. Default to 'true'.
        snippet_suggestions: SnippetSuggestions => snippetSuggestions;
        /// Performance guard: Stop rendering a line after x characters. Defaults to
        /// 10000. Use -1 to never stop rendering
        stop_rendering_line_after: u32 => stopRenderingLineAfter;
        /// Suggest options.
        suggest: &ISuggestOptions => suggest;
        /// The font size for the suggest widget. Defaults to the editor font size.
        suggest_font_size: u32 => suggestFontSize;
        /// The line height for the suggest widget. Defaults to the editor line
        /// height.
        suggest_line_height: u32 => suggestLineHeight;
        /// Enable the suggestion box to pop-up on trigger characters. Defaults to
        /// true.
        suggest_on_trigger_characters: bool => suggestOnTriggerCharacters;
        /// The history mode for suggestions.
        suggest_selection: SuggestSelection => suggestSelection;
        /// Enable tab completion.
        tab_completion: TabCompletion => tabCompletion;
        /// Inserting and deleting whitespace follows tab stops.
        use_tab_stops: bool => useTabStops;
        /// A string containing the word separators used when doing word navigation.
        /// Defaults to `~!@#$%^&*()-=+[{]}\|;:'",.<>/?
        word_separators: &str => wordSeparators;
        /// Control the wrapping of the editor. When wordWrap = "off", the lines
        /// will never wrap. When wordWrap = "on", the lines will wrap at the
        /// viewport width. When wordWrap = "wordWrapColumn", the lines will wrap at
        /// wordWrapColumn. When wordWrap = "bounded", the lines will wrap at
        /// min(viewport width, wordWrapColumn). Defaults to "off".
        word_wrap: WordWrap => wordWrap;
        /// Configure word wrapping characters. A break will be introduced after
        /// these characters. Defaults to '
        /// \t})]?|/&.,;¢°′″‰℃、。｡､￠，．：；？！％・･
        /// ゝゞヽヾーァィゥェォッャュョヮヵヶぁぃぅぇぉっゃゅょゎゕゖㇰㇱㇲㇳㇴㇵㇶㇷㇸㇹㇺㇻㇼㇽㇾㇿ々〻ｧｨｩｪｫｬｭｮｯｰ”〉》」』】〕）］｝｣'
        /// .
        word_wrap_break_after_characters: &str => wordWrapBreakAfterCharacters;
        /// Configure word wrapping characters. A break will be introduced before
        /// these characters. Defaults to '([{‘“〈《「『【〔（［｛｢£¥＄￡￥+＋'.
        word_wrap_break_before_characters: &str => wordWrapBreakBeforeCharacters;
        /// Control the wrapping of the editor. When wordWrap = "off", the lines
        /// will never wrap. When wordWrap = "on", the lines will wrap at the
        /// viewport width. When wordWrap = "wordWrapColumn", the lines will wrap at
        /// wordWrapColumn. When wordWrap = "bounded", the lines will wrap at
        /// min(viewport width, wordWrapColumn). Defaults to 80.
        word_wrap_column: u32 => wordWrapColumn;
        /// Force word wrapping when the text appears to be of a minified/generated
        /// file. Defaults to true.
        word_wrap_minified: bool => wordWrapMinified;
        /// Control indentation of wrapped lines. Can be: 'none', 'same', 'indent'
        /// or 'deepIndent'. Defaults to 'same' in vscode and to 'none' in
        /// monaco-editor.
        wrapping_indent: WrappingIndent => wrappingIndent;
        /// Controls the wrapping strategy to use. Defaults to 'simple'.
        wrapping_strategy: WrappingStrategy => wrappingStrategy;
    }
}

// TODO: IDiffEditorOptions

define_interface_builder! {
    /// Options to create an editor
    type IEditorConstructionOptions extends IEditorOptions {
        /// The initial editor dimension (to avoid measuring the container).
        dimension: &IDimension => dimension;
    }
}
impl IEditorConstructionOptions {
    /// Add the given `IEditorOptions`.
    pub fn with_editor_options(self, val: &IEditorOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }
}

define_interface_builder! {
    /// Dimension with width and height
    type IDimension extends Object {
        /// Height in pixels
        height: u32 => height;
        /// Width in pixels
        width: u32 => width;
    }
}

define_interface_builder! {
    /// The options to create an editor
    type IStandaloneEditorConstructionOptions extends IEditorConstructionOptions, IGlobalEditorOptions {
        /// An URL to open when Ctrl+H (Windows and Linux) or Cmd+H (OSX) is pressed
        /// in the accessibility help dialog in the editor. Defaults to "https://go.microsoft.com/fwlink/?linkid=852450"
        accessibility_help_url: &str => accessibilityHelpUrl;
        /// The initial language of the auto created model in the editor. To not
        /// create automatically a model, use model: null.
        language: &str => language;
        /// The initial model associated with this code editor.
        model: ITextModel => model;
        /// Initial theme to be used for rendering. The current out-of-the-box
        /// available themes are: 'vs' (default), 'vs-dark', 'hc-black'. You can
        /// create custom themes via monaco.editor.defineTheme. To switch a theme,
        /// use monaco.editor.setTheme
        // TODO allow setting custom theme
        theme: BuiltinTheme => theme;
        /// The initial value of the auto created model in the editor. To not create
        /// automatically a model, use model: null.
        value: &str => value;
    }
}
impl IStandaloneEditorConstructionOptions {
    /// Add the given `IEditorConstructionOptions`.
    pub fn with_editor_construction_options(self, val: &IEditorConstructionOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }

    /// Add the options from `IGlobalEditorOptions`.
    pub fn with_global_editor_options(self, val: &IGlobalEditorOptions) -> Self {
        JsCast::unchecked_into(Object::assign(&self, val))
    }
}

define_interface_builder! {
    /// A model.
    type ITextModel extends Object {
        /// Model id.
        id: &str => id;
        /// Gets the resource associated with this editor model.
        uri: &Uri => uri;
    }
}

define_interface_builder! {
    /// Configuration options for editor comments
    type IEditorCommentsOptions extends Object {
        /// Insert a space after the line comment token and inside the block comments tokens. Defaults to true.
        insert_space: bool => insertSpace;
    }
}

define_interface_builder! {
    /// Configuration options for editor find widget
    type IEditorFindOptions extends Object {
        /// Add extra space on top.
        add_extra_space_on_top: bool => addExtraSpaceOnTop;
        /// Controls if Find in Selection flag is turned on in the editor.
        auto_find_in_selection: AutoFindInSelection => autoFindInSelection;
        /// Controls if we seed search string in the Find Widget with editor selection.
        seed_search_string_from_selection: bool => seedSearchStringFromSelection;
    }
}

define_interface_builder! {
    /// Configuration options for editor hover
    type IEditorHoverOptions extends Object {
        /// Delay for showing the hover. Defaults to 300.
        delay: u32 => delay;
        /// Enable the hover. Defaults to true.
        enabled: bool => enabled;
        /// Is the hover sticky such that it can be clicked and its contents selected? Defaults to true.
        sticky: bool => sticky;
    }
}

define_interface_builder! {
    /// Configuration options for editor lightbulb
    type IEditorLightbulbOptions extends Object {
        /// Enable the lightbulb code action. Defaults to true.
        enabled: bool => enabled;
    }
}

define_interface_builder! {
    /// Configuration options for editor minimap
    type IEditorMinimapOptions extends Object {
        /// Enable the rendering of the minimap. Defaults to true.
        enabled: bool => enabled;
        /// Limit the width of the minimap to render at most a certain number of columns. Defaults to 120.
        max_column: u32 => maxColumn;
        /// Render the actual text on a line (as opposed to color blocks). Defaults to true.
        render_characters: bool => renderCharacters;
        /// Relative size of the font in the minimap. Defaults to 1.
        scale: f32 => scale;
        /// Control the rendering of the minimap slider. Defaults to 'mouseover'.
        show_slider: MinimapShowSlider => showSlider;
        /// Control the side of the minimap in editor. Defaults to 'right'.
        side: MinimapSide => side;
    }
}

define_interface_builder! {
    /// Configuration options for parameter hints
    type IEditorParameterHintOptions extends Object {
        /// Enable cycling of parameter hints. Defaults to false.
        cycle: bool => cycle;
        /// Enable parameter hints. Defaults to true.
        enabled: bool => enabled;
    }
}

define_interface_builder! {
    /// Configuration options for editor scrollbars
    type IEditorScrollbarOptions extends Object {
        /// Always consume mouse wheel events (always call preventDefault() and stopPropagation() on the browser events). Defaults to true.
        always_consume_mouse_wheel: bool => alwaysConsumeMouseWheel;
        /// The size of arrows (if displayed). Defaults to 11.
        arrow_size: u32 => arrowSize;
        /// Listen to mouse wheel events and react to them by scrolling. Defaults to true.
        handle_mouse_wheel: bool => handleMouseWheel;
        /// Render horizontal scrollbar. Defaults to 'auto'.
        horizontal: ScrollbarVisible => horizontal;
        /// Render arrows at the left and right of the horizontal scrollbar. Defaults to false.
        horizontal_has_arrows: bool => horizontalHasArrows;
        /// Height in pixels for the horizontal scrollbar. Defaults to 10 (px).
        horizontal_scrollbar_size: u32 => horizontalScrollbarSize;
        /// Height in pixels for the horizontal slider. Defaults to horizontalScrollbarSize.
        horizontal_slider_size: u32 => horizontalSliderSize;
        /// Cast horizontal and vertical shadows when the content is scrolled. Defaults to true.
        use_shadows: bool => useShadows;
        /// Render vertical scrollbar. Defaults to 'auto'.
        vertical: ScrollbarVisible => vertical;
        /// Render arrows at the top and bottom of the vertical scrollbar. Defaults to false.
        vertical_has_arrows: bool => verticalHasArrows;
        /// Width in pixels for the vertical scrollbar. Defaults to 10 (px).
        vertical_scrollbar_size: u32 => verticalScrollbarSize;
        /// Width in pixels for the vertical slider. Defaults to verticalScrollbarSize.
        vertical_slider_size: u32 => verticalSliderSize;
    }
}

define_interface_builder! {
    /// Configuration options for go to location
    type IGotoLocationOptions extends Object {
        /// Alternative declaration command
        alternative_declaration_command: &str => alternativeDeclarationCommand;
        /// Alternative definition command
        alternative_definition_command: &str => alternativeDefinitionCommand;
        /// Alternative implementation command
        alternative_implementation_command: &str => alternativeImplementationCommand;
        /// Alternative reference command
        alternative_reference_command: &str => alternativeReferenceCommand;
        /// Alternative type definition command
        alternative_type_definition_command: &str => alternativeTypeDefinitionCommand;
        /// Multiple
        multiple: GoToLocationValues => multiple;
        /// Multiple declarations
        multiple_declarations: GoToLocationValues => multipleDeclarations;
        /// Multiple definitions
        multiple_definitions: GoToLocationValues => multipleDefinitions;
        /// Multiple implementations
        multiple_implementations: GoToLocationValues => multipleImplementations;
        /// Multiple references
        multiple_references: GoToLocationValues => multipleReferences;
        /// Multiple type definitions
        multiple_type_definitions: GoToLocationValues => multipleTypeDefinitions;
    }
}

define_interface_builder! {
    /// Configuration options for editor suggest widget
    type ISuggestOptions extends Object {
        /// Enable graceful matching. Defaults to true.
        filter_graceful: bool => filterGraceful;
        /// Controls the visibility of the status bar at the bottom of the suggest widget.
        hide_status_bar: bool => hideStatusBar;
        /// Show a highlight when suggestion replaces or keep text after the cursor. Defaults to false.
        insert_highlight: bool => insertHighlight;
        /// Overwrite word ends on accept. Default to false.
        insert_mode: SuggestInsertMode => insertMode;
        /// Favours words that appear close to the cursor.
        locality_bonus: bool => localityBonus;
        /// Max suggestions to show in suggestions. Defaults to 12.
        max_visible_suggestions: u32 => maxVisibleSuggestions;
        /// Enable using global storage for remembering suggestions.
        share_suggest_selections: bool => shareSuggestSelections;
        /// Show class-suggestions.
        show_classes: bool => showClasses;
        /// Show color-suggestions.
        show_colors: bool => showColors;
        /// Show constant-suggestions.
        show_constants: bool => showConstants;
        /// Show constructor-suggestions.
        show_constructors: bool => showConstructors;
        /// Show enumMember-suggestions.
        show_enum_members: bool => showEnumMembers;
        /// Show enum-suggestions.
        show_enums: bool => showEnums;
        /// Show event-suggestions.
        show_events: bool => showEvents;
        /// Show field-suggestions.
        show_fields: bool => showFields;
        /// Show file-suggestions.
        show_files: bool => showFiles;
        /// Show folder-suggestions.
        show_folders: bool => showFolders;
        /// Show function-suggestions.
        show_functions: bool => showFunctions;
        /// Enable or disable icons in suggestions. Defaults to true.
        show_icons: bool => showIcons;
        /// Show interface-suggestions.
        show_interfaces: bool => showInterfaces;
        /// Show keyword-suggestions.
        show_keywords: bool => showKeywords;
        /// Show method-suggestions.
        show_methods: bool => showMethods;
        /// Show module-suggestions.
        show_modules: bool => showModules;
        /// Show operator-suggestions.
        show_operators: bool => showOperators;
        /// Show property-suggestions.
        show_properties: bool => showProperties;
        /// Show reference-suggestions.
        show_references: bool => showReferences;
        /// Show snippet-suggestions.
        show_snippets: bool => showSnippets;
        /// Show struct-suggestions.
        show_structs: bool => showStructs;
        /// Show typeParameter-suggestions.
        show_type_parameters: bool => showTypeParameters;
        /// Show unit-suggestions.
        show_units: bool => showUnits;
        /// Show value-suggestions.
        show_values: bool => showValues;
        /// Show variable-suggestions.
        show_variables: bool => showVariables;
        /// Show text-suggestions.
        show_words: bool => showWords;
        /// Prevent quick suggestions when a snippet is active. Defaults to true.
        snippets_prevent_quick_suggestions: bool => snippetsPreventQuickSuggestions;
    }
}
