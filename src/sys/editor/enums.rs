str_enum! {
    pub enum AcceptSuggestionOnEnter = On: "on" | Smart: "smart" | Off: "off";
}

str_enum! {
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
    pub enum AutoFindInSelection = Never: "never" | Always: "always" | Multiline: "multiline";
}

str_enum! {
    pub enum AutoIdent = None: "none" | Keep: "keep" | Brackets: "brackets" | Advanced: "advanced" | Full: "full";
}

str_enum! {
    /// Configuration options for auto wrapping quotes and brackets
    pub enum AutoSurroundStrategy = LanguageDefined: "languageDefined" | Quotes: "quotes" | Brackets: "brackets" | Never: "never";
}

str_enum! {
    pub enum BuiltinTheme = Vs: "vs" | VsDark: "vs-dark" | HcBlack: "hc-black";
}

str_enum! {
    pub enum CursorBlinkingStyle = Blink: "blink" | Smooth: "smooth" | Phase: "phase" | Expand: "expand" | Solid: "solid";
}

str_enum! {
    pub enum CursorStyle = Line: "line" | Block: "block" | Underline: "underline" |
        LineThin: "line-thin" | BlockOutline: "block-outline" | UnderlineThin: "underline-thin";
}

str_enum! {
    pub enum CursorSurroundingLinesStyle = Default: "default" | All: "all";
}

str_enum! {
    pub enum FoldingStrategy = Auto: "auto" | Indentation: "indentation";
}

str_enum! {
    pub enum GoToLocationValues = Peek: "peek" | GotoAndPeek: "gotoAndPeek" | Goto: "goto";
}

// TODO: this could also accept a function apparently.
str_enum! {
    pub enum LineNumbersType = On: "on" | Off: "off" | Relative: "relative" | Interval: "interval";
}

str_enum! {
    pub enum MatchBrackets = Never: "never" | Near: "near" | Always: "always";
}

str_enum! {
    pub enum MinimapShowSlider = Always: "always" | Mouseover: "mouseover";
}

str_enum! {
    pub enum MinimapSide = Right: "right" | Left: "left";
}

str_enum! {
    pub enum MouseStyle = Text: "text" | Default: "default" | Copy: "copy";
}

str_enum! {
    pub enum MultiCursorModifier = CtrlCmd: "ctrlCmd" | Alt: "alt";
}

str_enum! {
    pub enum MultiCursorPaste = Spread: "spread" | Full: "full";
}

str_enum! {
    pub enum PeekWidgetDefaultFocus = Tree: "tree" | Editor: "editor";
}

str_enum! {
    pub enum RenderLineHighlight = None: "none" | Gutter: "gutter" | Line: "line" | All: "all";
}

str_enum! {
    pub enum RenderValidationDecorations = Editable: "editable" | One: "on" | Off: "off";
}

str_enum! {
    pub enum RenderWhitespace = None: "none" | Boundary: "boundary" | Selection: "selection" | All: "all";
}

str_enum! {
    pub enum ScrollbarVisible = Auto: "auto" | Visible: "visible" | Hidden: "hidden";
}

str_enum! {
    pub enum ShowFoldingControls = Always: "always" | Mouseover: "mouseover";
}

str_enum! {
    pub enum SnippetSuggestions = Top: "top" | Bottom: "bottom" | Inline: "inline" | None: "none";
}

str_enum! {
    pub enum SuggestInsertMode = Insert: "insert" | Replace: "replace";
}

str_enum! {
    pub enum SuggestSelection = First: "first" | RecentlyUsed: "recentlyUsed" | RecentlyUsedByPrefix: "recentlyUsedByPrefix";
}

str_enum! {
    pub enum TabCompletion = On: "on" | Off: "off" | OnlySnippets: "onlySnippets";
}

str_enum! {
    pub enum WordWrap = Off: "off" | On: "on" | WordWrapColumn: "wordWrapColumn" | Bounded: "bounded";
}

str_enum! {
    pub enum WrappingIndent = None: "none" | Same: "same" | Indent: "indent" | DeepIndent: "deepIndent";
}

str_enum! {
    pub enum WrappingStrategy = Simple: "simple" | Advanced: "advanced";
}
