str_enum! {
    /// Accept suggestion on enter.
    pub enum AcceptSuggestionOnEnter = On: "on" | Smart: "smart" | Off: "off";
}

str_enum! {
    /// Accessibility support.
    pub enum AccessibilitySupport = Auto: "auto" | Off: "off" | On: "on";
}

str_enum! {
    /// Configuration options for auto closing quotes and brackets
    pub enum AutoClosingStrategy = Always: "always" | LanguageDefined: "languageDefined" | BeforeWhitespace: "beforeWhitespace" | Never: "never";
}

str_enum! {
    /// Configuration options for typing over closing quotes or brackets
    pub enum AutoClosingOvertype = Always: "always" | Auto: "auto" | Never: "never";
}

str_enum! {
    /// Auto find in selection
    pub enum AutoFindInSelection = Never: "never" | Always: "always" | Multiline: "multiline";
}

str_enum! {
    /// Automatically indent mode
    pub enum AutoIdent = None: "none" | Keep: "keep" | Brackets: "brackets" | Advanced: "advanced" | Full: "full";
}

str_enum! {
    /// Configuration options for auto wrapping quotes and brackets
    pub enum AutoSurroundStrategy = LanguageDefined: "languageDefined" | Quotes: "quotes" | Brackets: "brackets" | Never: "never";
}

str_enum! {
    /// Builtin theme
    pub enum BuiltinTheme = Vs: "vs" | VsDark: "vs-dark" | HcBlack: "hc-black";
}

str_enum! {
    /// Cursor blinking style
    pub enum CursorBlinkingStyle = Blink: "blink" | Smooth: "smooth" | Phase: "phase" | Expand: "expand" | Solid: "solid";
}

str_enum! {
    /// Cursor style
    pub enum CursorStyle = Line: "line" | Block: "block" | Underline: "underline" |
        LineThin: "line-thin" | BlockOutline: "block-outline" | UnderlineThin: "underline-thin";
}

str_enum! {
    /// Cursor surrounding lines style
    pub enum CursorSurroundingLinesStyle = Default: "default" | All: "all";
}

str_enum! {
    /// Folding behaviour
    pub enum FoldingStrategy = Auto: "auto" | Indentation: "indentation";
}

str_enum! {
    /// Go to location behaviour
    pub enum GoToLocationValues = Peek: "peek" | GotoAndPeek: "gotoAndPeek" | Goto: "goto";
}

// TODO: this could also accept a function apparently.
str_enum! {
    /// Line numbers type
    pub enum LineNumbersType = On: "on" | Off: "off" | Relative: "relative" | Interval: "interval";
}

str_enum! {
    /// Match brackets
    pub enum MatchBrackets = Never: "never" | Near: "near" | Always: "always";
}

str_enum! {
    /// Minimap show slider
    pub enum MinimapShowSlider = Always: "always" | Mouseover: "mouseover";
}

str_enum! {
    /// Minimap page side.
    pub enum MinimapSide = Right: "right" | Left: "left";
}

str_enum! {
    /// Mouse style
    pub enum MouseStyle = Text: "text" | Default: "default" | Copy: "copy";
}

str_enum! {
    /// Multi cursor modifier
    pub enum MultiCursorModifier = CtrlCmd: "ctrlCmd" | Alt: "alt";
}

str_enum! {
    /// Multi cursor paste behaviour.
    pub enum MultiCursorPaste = Spread: "spread" | Full: "full";
}

str_enum! {
    /// Peek widget default focus
    pub enum PeekWidgetDefaultFocus = Tree: "tree" | Editor: "editor";
}

str_enum! {
    /// Render line highlight.
    pub enum RenderLineHighlight = None: "none" | Gutter: "gutter" | Line: "line" | All: "all";
}

str_enum! {
    /// Render validation decorations.
    pub enum RenderValidationDecorations = Editable: "editable" | One: "on" | Off: "off";
}

str_enum! {
    /// Whitespace rendering
    pub enum RenderWhitespace = None: "none" | Boundary: "boundary" | Selection: "selection" | All: "all";
}

str_enum! {
    /// Scrollbar visibility
    pub enum ScrollbarVisible = Auto: "auto" | Visible: "visible" | Hidden: "hidden";
}

str_enum! {
    /// Show folding controls
    pub enum ShowFoldingControls = Always: "always" | Mouseover: "mouseover";
}

str_enum! {
    /// Snippet suggestion location.
    pub enum SnippetSuggestions = Top: "top" | Bottom: "bottom" | Inline: "inline" | None: "none";
}

str_enum! {
    /// Suggestion insert mode.
    pub enum SuggestInsertMode = Insert: "insert" | Replace: "replace";
}

str_enum! {
    /// Suggestion selection order.
    pub enum SuggestSelection = First: "first" | RecentlyUsed: "recentlyUsed" | RecentlyUsedByPrefix: "recentlyUsedByPrefix";
}

str_enum! {
    /// Tab completion strategy.
    pub enum TabCompletion = On: "on" | Off: "off" | OnlySnippets: "onlySnippets";
}

str_enum! {
    /// Word wrapping strategy.
    pub enum WordWrap = Off: "off" | On: "on" | WordWrapColumn: "wordWrapColumn" | Bounded: "bounded";
}

str_enum! {
    /// Text wrapping indent
    pub enum WrappingIndent = None: "none" | Same: "same" | Indent: "indent" | DeepIndent: "deepIndent";
}

str_enum! {
    /// Text wrapping strategy
    pub enum WrappingStrategy = Simple: "simple" | Advanced: "advanced";
}
