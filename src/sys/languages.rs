//! Bindings for the `monaco.languages` namespace.
use super::{
    editor::ITextModel,
    CancellationToken,
    IDisposable,
    IPosition,
    IRange,
    Position,
    Range,
    Uri,
};
use js_sys::{Array, Function, Object, RegExp, Uint32Array};
use wasm_bindgen::prelude::*;

#[cfg_attr(debug_assertions, wasm_bindgen(module = "/js/debug/editor.js"))]
#[cfg_attr(not(debug_assertions), wasm_bindgen(module = "/js/release/editor.js"))]
extern "C" {
    /// Register information about a new language.
    #[wasm_bindgen(js_name = "register", js_namespace = languages)]
    pub fn register(language: &ILanguageExtensionPoint);

    /// Get the information of all the registered languages.
    ///
    /// # Returns
    ///
    /// `ILanguageExtensionPoint[]`
    #[wasm_bindgen(js_name = "getLanguages", js_namespace = languages)]
    pub fn get_languages() -> Array;

    #[wasm_bindgen(js_name = "getEncodedLanguageId", js_namespace = languages)]
    pub fn get_encoded_language_id(language_id: &str) -> f64;

    /// An event emitted when a language is needed for the first time (e.g. a
    /// model has it set). @event
    ///
    /// # Arguments
    ///
    /// * `callback` - `() => void`
    #[wasm_bindgen(js_name = "onLanguage", js_namespace = languages)]
    pub fn on_language(language_id: &str, callback: &Function) -> IDisposable;

    /// Set the editing configuration for a language.
    #[wasm_bindgen(js_name = "setLanguageConfiguration", js_namespace = languages)]
    pub fn set_language_configuration(
        language_id: &str,
        configuration: &LanguageConfiguration,
    ) -> IDisposable;

    /// Change the color map that is used for token colors.
    /// Supported formats (hex): #RRGGBB, $RRGGBBAA, #RGB, #RGBA
    ///
    /// # Arguments
    ///
    /// * `color_map` - `string[]`
    #[wasm_bindgen(js_name = "setColorMap", js_namespace = languages)]
    pub fn set_color_map(color_map: Option<&Array>);

    /// Register a tokens provider factory for a language. This tokenizer will
    /// be exclusive with a tokenizer set using `setTokensProvider` or one
    /// created using `setMonarchTokensProvider`, but will work together
    /// with a tokens provider set using
    /// `registerDocumentSemanticTokensProvider` or
    /// `registerDocumentRangeSemanticTokensProvider`.
    #[wasm_bindgen(js_name = "registerTokensProviderFactory", js_namespace = languages)]
    pub fn register_tokens_provider_factory(
        language_id: &str,
        factory: &TokensProviderFactory,
    ) -> IDisposable;

    /// Set the tokens provider for a language (manual implementation). This
    /// tokenizer will be exclusive with a tokenizer created using
    /// `setMonarchTokensProvider`, or with `registerTokensProviderFactory`,
    /// but will work together with a tokens provider set using
    /// `registerDocumentSemanticTokensProvider`
    /// or `registerDocumentRangeSemanticTokensProvider`.
    #[wasm_bindgen(js_name = "setTokensProvider", js_namespace = languages)]
    pub fn set_tokens_provider(language_id: &str, provider: &JsValue) -> IDisposable;

    /// Set the tokens provider for a language (monarch implementation). This
    /// tokenizer will be exclusive with a tokenizer set using
    /// `setTokensProvider`, or with `registerTokensProviderFactory`, but will
    /// work together with a tokens provider set using
    /// `registerDocumentSemanticTokensProvider` or
    /// `registerDocumentRangeSemanticTokensProvider`.
    #[wasm_bindgen(js_name = "setMonarchTokensProvider", js_namespace = languages)]
    pub fn set_monarch_tokens_provider(language_id: &str, language_def: &JsValue) -> IDisposable;

    /// Register a reference provider (used by e.g. reference search).
    #[wasm_bindgen(js_name = "registerReferenceProvider", js_namespace = languages)]
    pub fn register_reference_provider(
        language_id: &str,
        provider: &ReferenceProvider,
    ) -> IDisposable;

    /// Register a rename provider (used by e.g. rename symbol).
    #[wasm_bindgen(js_name = "registerRenameProvider", js_namespace = languages)]
    pub fn register_rename_provider(language_id: &str, provider: &RenameProvider) -> IDisposable;

    /// Register a signature help provider (used by e.g. parameter hints).
    #[wasm_bindgen(js_name = "registerSignatureHelpProvider", js_namespace = languages)]
    pub fn register_signature_help_provider(
        language_id: &str,
        provider: &SignatureHelpProvider,
    ) -> IDisposable;

    /// Register a hover provider (used by e.g. editor hover).
    #[wasm_bindgen(js_name = "registerHoverProvider", js_namespace = languages)]
    pub fn register_hover_provider(language_id: &str, provider: &HoverProvider) -> IDisposable;

    /// Register a document symbol provider (used by e.g. outline).
    #[wasm_bindgen(js_name = "registerDocumentSymbolProvider", js_namespace = languages)]
    pub fn register_document_symbol_provider(
        language_id: &str,
        provider: &DocumentSymbolProvider,
    ) -> IDisposable;

    /// Register a document highlight provider (used by e.g. highlight
    /// occurrences).
    #[wasm_bindgen(js_name = "registerDocumentHighlightProvider", js_namespace = languages)]
    pub fn register_document_highlight_provider(
        language_id: &str,
        provider: &DocumentHighlightProvider,
    ) -> IDisposable;

    /// Register an linked editing range provider.
    #[wasm_bindgen(js_name = "registerLinkedEditingRangeProvider", js_namespace = languages)]
    pub fn register_linked_editing_range_provider(
        language_id: &str,
        provider: &LinkedEditingRangeProvider,
    ) -> IDisposable;

    /// Register a definition provider (used by e.g. go to definition).
    #[wasm_bindgen(js_name = "registerDefinitionProvider", js_namespace = languages)]
    pub fn register_definition_provider(
        language_id: &str,
        provider: &DefinitionProvider,
    ) -> IDisposable;

    /// Register a implementation provider (used by e.g. go to implementation).
    #[wasm_bindgen(js_name = "registerImplementationProvider", js_namespace = languages)]
    pub fn register_implementation_provider(
        language_id: &str,
        provider: &ImplementationProvider,
    ) -> IDisposable;

    /// Register a type definition provider (used by e.g. go to type
    /// definition).
    #[wasm_bindgen(js_name = "registerTypeDefinitionProvider", js_namespace = languages)]
    pub fn register_type_definition_provider(
        language_id: &str,
        provider: &TypeDefinitionProvider,
    ) -> IDisposable;

    /// Register a code lens provider (used by e.g. inline code lenses).
    #[wasm_bindgen(js_name = "registerCodeLensProvider", js_namespace = languages)]
    pub fn register_code_lens_provider(
        language_id: &str,
        provider: &CodeLensProvider,
    ) -> IDisposable;

    /// Register a code action provider (used by e.g. quick fix).
    #[wasm_bindgen(js_name = "registerCodeActionProvider", js_namespace = languages)]
    pub fn register_code_action_provider(
        language_id: &str,
        provider: &CodeActionProvider,
        metadata: Option<&CodeActionProviderMetadata>,
    ) -> IDisposable;

    /// Register a formatter that can handle only entire models.
    #[wasm_bindgen(js_name = "registerDocumentFormattingEditProvider", js_namespace = languages)]
    pub fn register_document_formatting_edit_provider(
        language_id: &str,
        provider: &DocumentFormattingEditProvider,
    ) -> IDisposable;

    /// Register a formatter that can handle a range inside a model.
    #[wasm_bindgen(js_name = "registerDocumentRangeFormattingEditProvider", js_namespace = languages)]
    pub fn register_document_range_formatting_edit_provider(
        language_id: &str,
        provider: &DocumentRangeFormattingEditProvider,
    ) -> IDisposable;

    /// Register a formatter than can do formatting as the user types.
    #[wasm_bindgen(js_name = "registerOnTypeFormattingEditProvider", js_namespace = languages)]
    pub fn register_on_type_formatting_edit_provider(
        language_id: &str,
        provider: &OnTypeFormattingEditProvider,
    ) -> IDisposable;

    /// Register a link provider that can find links in text.
    #[wasm_bindgen(js_name = "registerLinkProvider", js_namespace = languages)]
    pub fn register_link_provider(language_id: &str, provider: &LinkProvider) -> IDisposable;

    /// Register a completion item provider (use by e.g. suggestions).
    #[wasm_bindgen(js_name = "registerCompletionItemProvider", js_namespace = languages)]
    pub fn register_completion_item_provider(
        language_id: &str,
        provider: &CompletionItemProvider,
    ) -> IDisposable;

    /// Register a document color provider (used by Color Picker, Color
    /// Decorator).
    #[wasm_bindgen(js_name = "registerColorProvider", js_namespace = languages)]
    pub fn register_color_provider(
        language_id: &str,
        provider: &DocumentColorProvider,
    ) -> IDisposable;

    /// Register a folding range provider
    #[wasm_bindgen(js_name = "registerFoldingRangeProvider", js_namespace = languages)]
    pub fn register_folding_range_provider(
        language_id: &str,
        provider: &FoldingRangeProvider,
    ) -> IDisposable;

    /// Register a declaration provider
    #[wasm_bindgen(js_name = "registerDeclarationProvider", js_namespace = languages)]
    pub fn register_declaration_provider(
        language_id: &str,
        provider: &DeclarationProvider,
    ) -> IDisposable;

    /// Register a selection range provider
    #[wasm_bindgen(js_name = "registerSelectionRangeProvider", js_namespace = languages)]
    pub fn register_selection_range_provider(
        language_id: &str,
        provider: &SelectionRangeProvider,
    ) -> IDisposable;

    /// Register a document semantic tokens provider. A semantic tokens provider
    /// will complement and enhance a simple top-down tokenizer. Simple
    /// top-down tokenizers can be set either via `setMonarchTokensProvider`
    /// or `setTokensProvider`.
    ///
    /// For the best user experience, register both a semantic tokens provider
    /// and a top-down tokenizer.
    #[wasm_bindgen(js_name = "registerDocumentSemanticTokensProvider", js_namespace = languages)]
    pub fn register_document_semantic_tokens_provider(
        language_id: &str,
        provider: &DocumentSemanticTokensProvider,
    ) -> IDisposable;

    /// Register a document range semantic tokens provider. A semantic tokens
    /// provider will complement and enhance a simple top-down tokenizer.
    /// Simple top-down tokenizers can be set either via
    /// `setMonarchTokensProvider` or `setTokensProvider`.
    ///
    /// For the best user experience, register both a semantic tokens provider
    /// and a top-down tokenizer.
    #[wasm_bindgen(js_name = "registerDocumentRangeSemanticTokensProvider", js_namespace = languages)]
    pub fn register_document_range_semantic_tokens_provider(
        language_id: &str,
        provider: &DocumentRangeSemanticTokensProvider,
    ) -> IDisposable;

    /// Register an inline completions provider.
    #[wasm_bindgen(js_name = "registerInlineCompletionsProvider", js_namespace = languages)]
    pub fn register_inline_completions_provider(
        language_id: &str,
        provider: &InlineCompletionsProvider,
    ) -> IDisposable;

    /// Register an inlay hints provider.
    #[wasm_bindgen(js_name = "registerInlayHintsProvider", js_namespace = languages)]
    pub fn register_inlay_hints_provider(
        language_id: &str,
        provider: &InlayHintsProvider,
    ) -> IDisposable;

    #[derive(Debug)]
    pub type FoldingRangeKind;
    #[wasm_bindgen(method, js_class = "FoldingRangeKind", js_name = "value", js_namespace = languages, getter = value)]
    pub fn value(this: &FoldingRangeKind) -> String;
    /// Set the `value` property.
    #[wasm_bindgen(method, js_class = "FoldingRangeKind", js_name = "value", js_namespace = languages, setter = value)]
    pub fn set_value(this: &FoldingRangeKind, val: &str);
    /// Kind for folding range representing a comment. The value of the kind is
    /// 'comment'.
    #[wasm_bindgen(static_method_of = FoldingRangeKind, js_class = "FoldingRangeKind", js_name = "Comment", js_namespace = languages, getter = Comment)]
    pub fn comment() -> FoldingRangeKind;
    /// Kind for folding range representing a import. The value of the kind is
    /// 'imports'.
    #[wasm_bindgen(static_method_of = FoldingRangeKind, js_class = "FoldingRangeKind", js_name = "Imports", js_namespace = languages, getter = Imports)]
    pub fn imports() -> FoldingRangeKind;
    /// Kind for folding range representing regions (for example marked by
    /// `#region`, `#endregion`). The value of the kind is 'region'.
    #[wasm_bindgen(static_method_of = FoldingRangeKind, js_class = "FoldingRangeKind", js_name = "Region", js_namespace = languages, getter = Region)]
    pub fn region() -> FoldingRangeKind;
    /// Creates a new {@link FoldingRangeKind}.
    ///
    /// @param value of the kind.
    #[wasm_bindgen(method, js_class = "FoldingRangeKind", js_name = "constructor", js_namespace = languages)]
    pub fn constructor(this: &FoldingRangeKind, value: &str);
}
int_enum! {
    pub enum IndentAction {
        /// Insert new line and copy the previous line's indentation.
        None = 0,
        /// Insert new line and indent once (relative to the previous line's indentation).
        Indent = 1,
        /// Insert two new lines:
        ///  - the first one indented which will hold the cursor
        ///  - the second one at the same indentation level
        Indentoutdent = 2,
        /// Insert new line and outdent once (relative to the previous line's indentation).
        Outdent = 3,
    }
}

int_enum! {
    pub enum CompletionItemKind {
        Method = 0,
        Function = 1,
        Constructor = 2,
        Field = 3,
        Variable = 4,
        Class = 5,
        Struct = 6,
        Interface = 7,
        Module = 8,
        Property = 9,
        Event = 10,
        Operator = 11,
        Unit = 12,
        Value = 13,
        Constant = 14,
        Enum = 15,
        Enummember = 16,
        Keyword = 17,
        Text = 18,
        Color = 19,
        File = 20,
        Reference = 21,
        Customcolor = 22,
        Folder = 23,
        Typeparameter = 24,
        User = 25,
        Issue = 26,
        Snippet = 27,
    }
}

int_enum! {
    pub enum CompletionItemTag {
        Deprecated = 1,
    }
}

int_enum! {
    pub enum CompletionItemInsertTextRule {
        /// Adjust whitespace/indentation of multiline insert texts to
        /// match the current line indentation.
        Keepwhitespace = 1,
        /// `insertText` is a snippet.
        Insertassnippet = 4,
    }
}

int_enum! {
    pub enum CompletionTriggerKind {
        Invoke = 0,
        Triggercharacter = 1,
        Triggerforincompletecompletions = 2,
    }
}

int_enum! {
    pub enum InlineCompletionTriggerKind {
        /// Completion was triggered automatically while editing.
        /// It is sufficient to return a single completion item in this case.
        Automatic = 0,
        /// Completion was triggered explicitly by a user gesture.
        /// Return multiple completion items to enable cycling through them.
        Explicit = 1,
    }
}

int_enum! {
    pub enum SignatureHelpTriggerKind {
        Invoke = 1,
        Triggercharacter = 2,
        Contentchange = 3,
    }
}

int_enum! {
    pub enum DocumentHighlightKind {
        /// A textual occurrence.
        Text = 0,
        /// Read-access of a symbol, like reading a variable.
        Read = 1,
        /// Write-access of a symbol, like writing to a variable.
        Write = 2,
    }
}

int_enum! {
    pub enum SymbolKind {
        File = 0,
        Module = 1,
        Namespace = 2,
        Package = 3,
        Class = 4,
        Method = 5,
        Property = 6,
        Field = 7,
        Constructor = 8,
        Enum = 9,
        Interface = 10,
        Function = 11,
        Variable = 12,
        Constant = 13,
        String = 14,
        Number = 15,
        Boolean = 16,
        Array = 17,
        Object = 18,
        Key = 19,
        Null = 20,
        Enummember = 21,
        Struct = 22,
        Event = 23,
        Operator = 24,
        Typeparameter = 25,
    }
}

int_enum! {
    pub enum SymbolTag {
        Deprecated = 1,
    }
}

int_enum! {
    pub enum InlayHintKind {
        Other = 0,
        Type = 1,
        Parameter = 2,
    }
}

#[wasm_bindgen]
extern "C" {
    /// A token.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IToken;
    #[wasm_bindgen(method, js_class = "IToken", js_name = "startIndex", js_namespace = languages, getter = startIndex)]
    pub fn start_index(this: &IToken) -> f64;
    /// Set the `startIndex` property.
    #[wasm_bindgen(method, js_class = "IToken", js_name = "startIndex", js_namespace = languages, setter = startIndex)]
    pub fn set_start_index(this: &IToken, val: f64);
    #[wasm_bindgen(method, js_class = "IToken", js_name = "scopes", js_namespace = languages, getter = scopes)]
    pub fn scopes(this: &IToken) -> String;
    /// Set the `scopes` property.
    #[wasm_bindgen(method, js_class = "IToken", js_name = "scopes", js_namespace = languages, setter = scopes)]
    pub fn set_scopes(this: &IToken, val: &str);
}

#[wasm_bindgen]
extern "C" {
    /// The result of a line tokenization.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ILineTokens;
    /// The list of tokens on the line.
    ///
    /// Type: `IToken[]`
    #[wasm_bindgen(method, js_class = "ILineTokens", js_name = "tokens", js_namespace = languages, getter = tokens)]
    pub fn tokens(this: &ILineTokens) -> Array;
    /// Set the `tokens` property.
    #[wasm_bindgen(method, js_class = "ILineTokens", js_name = "tokens", js_namespace = languages, setter = tokens)]
    pub fn set_tokens(this: &ILineTokens, val: &Array);
    /// The tokenization end state.
    /// A pointer will be held to this and the object should not be modified by
    /// the tokenizer after the pointer is returned.
    #[wasm_bindgen(method, js_class = "ILineTokens", js_name = "endState", js_namespace = languages, getter = endState)]
    pub fn end_state(this: &ILineTokens) -> IState;
    /// Set the `endState` property.
    #[wasm_bindgen(method, js_class = "ILineTokens", js_name = "endState", js_namespace = languages, setter = endState)]
    pub fn set_end_state(this: &ILineTokens, val: &IState);
}

#[wasm_bindgen]
extern "C" {
    /// The result of a line tokenization.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IEncodedLineTokens;
    /// The tokens on the line in a binary, encoded format. Each token occupies
    /// two array indices. For token i:
    ///  - at offset 2*i => startIndex
    ///  - at offset 2*i + 1 => metadata
    /// Meta data is in binary format:
    /// - ------------------------------------------- 3322 2222 2222 1111 1111
    ///   1100 0000 0000 1098 7654 3210 9876 5432 1098 7654 3210
    /// - ------------------------------------------- bbbb bbbb bfff ffff ffFF
    ///   FFTT LLLL LLLL
    /// - -------------------------------------------
    ///  - L = EncodedLanguageId (8 bits): Use `getEncodedLanguageId` to get the
    ///    encoded ID of a language.
    ///  - T = StandardTokenType (2 bits): Other = 0, Comment = 1, String = 2,
    ///    RegEx = 3.
    ///  - F = FontStyle (4 bits): None = 0, Italic = 1, Bold = 2, Underline =
    ///    4, Strikethrough = 8.
    ///  - f = foreground ColorId (9 bits)
    ///  - b = background ColorId (9 bits)
    ///  - The color value for each colorId is defined in
    ///    IStandaloneThemeData.customTokenColors:
    /// e.g. colorId = 1 is stored in IStandaloneThemeData.customTokenColors[1].
    /// Color id = 0 means no color, id = 1 is for the default foreground
    /// color, id = 2 for the default background.
    #[wasm_bindgen(method, js_class = "IEncodedLineTokens", js_name = "tokens", js_namespace = languages, getter = tokens)]
    pub fn tokens(this: &IEncodedLineTokens) -> Uint32Array;
    /// Set the `tokens` property.
    #[wasm_bindgen(method, js_class = "IEncodedLineTokens", js_name = "tokens", js_namespace = languages, setter = tokens)]
    pub fn set_tokens(this: &IEncodedLineTokens, val: &Uint32Array);
    /// The tokenization end state.
    /// A pointer will be held to this and the object should not be modified by
    /// the tokenizer after the pointer is returned.
    #[wasm_bindgen(method, js_class = "IEncodedLineTokens", js_name = "endState", js_namespace = languages, getter = endState)]
    pub fn end_state(this: &IEncodedLineTokens) -> IState;
    /// Set the `endState` property.
    #[wasm_bindgen(method, js_class = "IEncodedLineTokens", js_name = "endState", js_namespace = languages, setter = endState)]
    pub fn set_end_state(this: &IEncodedLineTokens, val: &IState);
}

#[wasm_bindgen]
extern "C" {
    /// A factory for token providers.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type TokensProviderFactory;
    #[wasm_bindgen(method, js_class = "TokensProviderFactory", js_name = "create", js_namespace = languages)]
    pub fn create(this: &TokensProviderFactory) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// A "manual" provider of tokens.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type TokensProvider;
    /// The initial state of a language. Will be the state passed in to tokenize
    /// the first line.
    #[wasm_bindgen(method, js_class = "TokensProvider", js_name = "getInitialState", js_namespace = languages)]
    pub fn get_initial_state(this: &TokensProvider) -> IState;
    /// Tokenize a line given the state at the beginning of the line.
    #[wasm_bindgen(method, js_class = "TokensProvider", js_name = "tokenize", js_namespace = languages)]
    pub fn tokenize(this: &TokensProvider, line: &str, state: &IState) -> ILineTokens;
}

#[wasm_bindgen]
extern "C" {
    /// A "manual" provider of tokens, returning tokens in a binary form.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type EncodedTokensProvider;
    /// The initial state of a language. Will be the state passed in to tokenize
    /// the first line.
    #[wasm_bindgen(
        method,
        js_class = "EncodedTokensProvider",
        js_name = "getInitialState", js_namespace = languages
    )]
    pub fn get_initial_state(this: &EncodedTokensProvider) -> IState;
    /// Tokenize a line given the state at the beginning of the line.
    #[wasm_bindgen(
        method,
        js_class = "EncodedTokensProvider",
        js_name = "tokenizeEncoded", js_namespace = languages
    )]
    pub fn tokenize_encoded(
        this: &EncodedTokensProvider,
        line: &str,
        state: &IState,
    ) -> IEncodedLineTokens;
    /// Tokenize a line given the state at the beginning of the line.
    ///
    /// Type: `((line: string, state: IState) => ILineTokens)`
    #[wasm_bindgen(method, js_class = "EncodedTokensProvider", js_name = "tokenize", js_namespace = languages, getter = tokenize)]
    pub fn tokenize(this: &EncodedTokensProvider) -> Option<Function>;
    /// Set the `tokenize` property.
    #[wasm_bindgen(method, js_class = "EncodedTokensProvider", js_name = "tokenize", js_namespace = languages, setter = tokenize)]
    pub fn set_tokenize(this: &EncodedTokensProvider, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    /// Contains additional diagnostic information about the context in which
    /// a [code action](#CodeActionProvider.provideCodeActions) is run.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CodeActionContext;
    /// An array of diagnostics.
    ///
    /// Type: `editor.IMarkerData[]`
    #[wasm_bindgen(method, js_class = "CodeActionContext", js_name = "markers", js_namespace = languages, getter = markers)]
    pub fn markers(this: &CodeActionContext) -> Array;
    /// Requested kind of actions to return.
    #[wasm_bindgen(method, js_class = "CodeActionContext", js_name = "only", js_namespace = languages, getter = only)]
    pub fn only(this: &CodeActionContext) -> Option<String>;
}

#[wasm_bindgen]
extern "C" {
    /// The code action interface defines the contract between extensions and
    /// the [light bulb](https://code.visualstudio.com/docs/editor/editingevolved#_code-action) feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CodeActionProvider;
    /// Provide commands for the given document and range.
    #[wasm_bindgen(
        method,
        js_class = "CodeActionProvider",
        js_name = "provideCodeActions", js_namespace = languages
    )]
    pub fn provide_code_actions(
        this: &CodeActionProvider,
        model: &ITextModel,
        range: &Range,
        context: &CodeActionContext,
        token: &CancellationToken,
    ) -> JsValue;
    /// Given a code action fill in the edit. Will only invoked when missing.
    ///
    /// Type: `((codeAction: CodeAction, token: CancellationToken) => JsValue)`
    #[wasm_bindgen(method, js_class = "CodeActionProvider", js_name = "resolveCodeAction", js_namespace = languages, getter = resolveCodeAction)]
    pub fn resolve_code_action(this: &CodeActionProvider) -> Option<Function>;
    /// Set the `resolveCodeAction` property.
    #[wasm_bindgen(method, js_class = "CodeActionProvider", js_name = "resolveCodeAction", js_namespace = languages, setter = resolveCodeAction)]
    pub fn set_resolve_code_action(this: &CodeActionProvider, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    /// Metadata about the type of code actions that a {@link
    /// CodeActionProvider} provides.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CodeActionProviderMetadata;
    /// List of code action kinds that a {@link CodeActionProvider} may return.
    ///
    /// This list is used to determine if a given `CodeActionProvider` should be
    /// invoked or not. To avoid unnecessary computation, every
    /// `CodeActionProvider` should list use `providedCodeActionKinds`. The
    /// list of kinds may either be generic, such as `["quickfix", "refactor",
    /// "source"]`, or list out every kind provided, such as `["quickfix.
    /// removeLine", "source.fixAll" ...]`.
    ///
    /// Type: `readonly string[]`
    #[wasm_bindgen(method, js_class = "CodeActionProviderMetadata", js_name = "providedCodeActionKinds", js_namespace = languages, getter = providedCodeActionKinds)]
    pub fn provided_code_action_kinds(this: &CodeActionProviderMetadata) -> Option<Array>;
}

#[wasm_bindgen]
extern "C" {
    /// Describes how comments for a language work.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CommentRule;
    /// The line comment token, like `// this is a comment`
    #[wasm_bindgen(method, js_class = "CommentRule", js_name = "lineComment", js_namespace = languages, getter = lineComment)]
    pub fn line_comment(this: &CommentRule) -> Option<String>;
    /// Set the `lineComment` property.
    #[wasm_bindgen(method, js_class = "CommentRule", js_name = "lineComment", js_namespace = languages, setter = lineComment)]
    pub fn set_line_comment(this: &CommentRule, val: Option<&str>);
    /// The block comment character pair, like `/* block comment *&#47;`
    #[wasm_bindgen(method, js_class = "CommentRule", js_name = "blockComment", js_namespace = languages, getter = blockComment)]
    pub fn block_comment(this: &CommentRule) -> Option<Array>;
    /// Set the `blockComment` property.
    #[wasm_bindgen(method, js_class = "CommentRule", js_name = "blockComment", js_namespace = languages, setter = blockComment)]
    pub fn set_block_comment(this: &CommentRule, val: Option<&Array>);
}

#[wasm_bindgen]
extern "C" {
    /// The language configuration interface defines the contract between
    /// extensions and various editor features, like automatic bracket
    /// insertion, automatic indentation etc.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type LanguageConfiguration;
    /// The language's comment settings.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "comments", js_namespace = languages, getter = comments)]
    pub fn comments(this: &LanguageConfiguration) -> Option<CommentRule>;
    /// Set the `comments` property.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "comments", js_namespace = languages, setter = comments)]
    pub fn set_comments(this: &LanguageConfiguration, val: Option<&CommentRule>);
    /// The language's brackets.
    /// This configuration implicitly affects pressing Enter around these
    /// brackets.
    ///
    /// Type: `CharacterPair[]`
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "brackets", js_namespace = languages, getter = brackets)]
    pub fn brackets(this: &LanguageConfiguration) -> Option<Array>;
    /// Set the `brackets` property.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "brackets", js_namespace = languages, setter = brackets)]
    pub fn set_brackets(this: &LanguageConfiguration, val: Option<&Array>);
    /// The language's word definition.
    /// If the language supports Unicode identifiers (e.g. JavaScript), it is
    /// preferable to provide a word definition that uses exclusion of known
    /// separators. e.g.: A regex that matches anything except known
    /// separators (and dot is allowed to occur in a floating point number):
    ///   /(-?\d*\.\d\w*)|([^\`\~\!\@\#\%\^\&\*\(\)\-\=\+\[\{\]\}\\\|\;\:\'\"\,\
    /// .\<\>\/\?\s]+)/g
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "wordPattern", js_namespace = languages, getter = wordPattern)]
    pub fn word_pattern(this: &LanguageConfiguration) -> Option<RegExp>;
    /// Set the `wordPattern` property.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "wordPattern", js_namespace = languages, setter = wordPattern)]
    pub fn set_word_pattern(this: &LanguageConfiguration, val: Option<&RegExp>);
    /// The language's indentation settings.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "indentationRules", js_namespace = languages, getter = indentationRules)]
    pub fn indentation_rules(this: &LanguageConfiguration) -> Option<IndentationRule>;
    /// Set the `indentationRules` property.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "indentationRules", js_namespace = languages, setter = indentationRules)]
    pub fn set_indentation_rules(this: &LanguageConfiguration, val: Option<&IndentationRule>);
    /// The language's rules to be evaluated when pressing Enter.
    ///
    /// Type: `OnEnterRule[]`
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "onEnterRules", js_namespace = languages, getter = onEnterRules)]
    pub fn on_enter_rules(this: &LanguageConfiguration) -> Option<Array>;
    /// Set the `onEnterRules` property.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "onEnterRules", js_namespace = languages, setter = onEnterRules)]
    pub fn set_on_enter_rules(this: &LanguageConfiguration, val: Option<&Array>);
    /// The language's auto closing pairs. The 'close' character is
    /// automatically inserted with the 'open' character is typed. If not
    /// set, the configured brackets will be used.
    ///
    /// Type: `IAutoClosingPairConditional[]`
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "autoClosingPairs", js_namespace = languages, getter = autoClosingPairs)]
    pub fn auto_closing_pairs(this: &LanguageConfiguration) -> Option<Array>;
    /// Set the `autoClosingPairs` property.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "autoClosingPairs", js_namespace = languages, setter = autoClosingPairs)]
    pub fn set_auto_closing_pairs(this: &LanguageConfiguration, val: Option<&Array>);
    /// The language's surrounding pairs. When the 'open' character is typed on
    /// a selection, the selected string is surrounded by the open and close
    /// characters. If not set, the autoclosing pairs settings will be used.
    ///
    /// Type: `IAutoClosingPair[]`
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "surroundingPairs", js_namespace = languages, getter = surroundingPairs)]
    pub fn surrounding_pairs(this: &LanguageConfiguration) -> Option<Array>;
    /// Set the `surroundingPairs` property.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "surroundingPairs", js_namespace = languages, setter = surroundingPairs)]
    pub fn set_surrounding_pairs(this: &LanguageConfiguration, val: Option<&Array>);
    /// Defines a list of bracket pairs that are colorized depending on their
    /// nesting level. If not set, the configured brackets will be used.
    ///
    /// Type: `CharacterPair[]`
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "colorizedBracketPairs", js_namespace = languages, getter = colorizedBracketPairs)]
    pub fn colorized_bracket_pairs(this: &LanguageConfiguration) -> Option<Array>;
    /// Set the `colorizedBracketPairs` property.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "colorizedBracketPairs", js_namespace = languages, setter = colorizedBracketPairs)]
    pub fn set_colorized_bracket_pairs(this: &LanguageConfiguration, val: Option<&Array>);
    /// Defines what characters must be after the cursor for bracket or quote
    /// autoclosing to occur when using the \'languageDefined\' autoclosing
    /// setting.
    ///
    /// This is typically the set of characters which can not start an
    /// expression, such as whitespace, closing brackets, non-unary operators,
    /// etc.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "autoCloseBefore", js_namespace = languages, getter = autoCloseBefore)]
    pub fn auto_close_before(this: &LanguageConfiguration) -> Option<String>;
    /// Set the `autoCloseBefore` property.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "autoCloseBefore", js_namespace = languages, setter = autoCloseBefore)]
    pub fn set_auto_close_before(this: &LanguageConfiguration, val: Option<&str>);
    /// The language's folding rules.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "folding", js_namespace = languages, getter = folding)]
    pub fn folding(this: &LanguageConfiguration) -> Option<FoldingRules>;
    /// Set the `folding` property.
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "folding", js_namespace = languages, setter = folding)]
    pub fn set_folding(this: &LanguageConfiguration, val: Option<&FoldingRules>);
}

#[wasm_bindgen]
extern "C" {
    /// Describes indentation rules for a language.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IndentationRule;
    /// If a line matches this pattern, then all the lines after it should be
    /// unindented once (until another rule matches).
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "decreaseIndentPattern", js_namespace = languages, getter = decreaseIndentPattern)]
    pub fn decrease_indent_pattern(this: &IndentationRule) -> RegExp;
    /// Set the `decreaseIndentPattern` property.
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "decreaseIndentPattern", js_namespace = languages, setter = decreaseIndentPattern)]
    pub fn set_decrease_indent_pattern(this: &IndentationRule, val: &RegExp);
    /// If a line matches this pattern, then all the lines after it should be
    /// indented once (until another rule matches).
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "increaseIndentPattern", js_namespace = languages, getter = increaseIndentPattern)]
    pub fn increase_indent_pattern(this: &IndentationRule) -> RegExp;
    /// Set the `increaseIndentPattern` property.
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "increaseIndentPattern", js_namespace = languages, setter = increaseIndentPattern)]
    pub fn set_increase_indent_pattern(this: &IndentationRule, val: &RegExp);
    /// If a line matches this pattern, then **only the next line** after it
    /// should be indented once.
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "indentNextLinePattern", js_namespace = languages, getter = indentNextLinePattern)]
    pub fn indent_next_line_pattern(this: &IndentationRule) -> Option<RegExp>;
    /// Set the `indentNextLinePattern` property.
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "indentNextLinePattern", js_namespace = languages, setter = indentNextLinePattern)]
    pub fn set_indent_next_line_pattern(this: &IndentationRule, val: Option<&RegExp>);
    /// If a line matches this pattern, then its indentation should not be
    /// changed and it should not be evaluated against the other rules.
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "unIndentedLinePattern", js_namespace = languages, getter = unIndentedLinePattern)]
    pub fn un_indented_line_pattern(this: &IndentationRule) -> Option<RegExp>;
    /// Set the `unIndentedLinePattern` property.
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "unIndentedLinePattern", js_namespace = languages, setter = unIndentedLinePattern)]
    pub fn set_un_indented_line_pattern(this: &IndentationRule, val: Option<&RegExp>);
}

#[wasm_bindgen]
extern "C" {
    /// Describes language specific folding markers such as '#region' and
    /// '#endregion'. The start and end regexes will be tested against the
    /// contents of all lines and must be designed efficiently:
    /// - the regex should start with '^'
    /// - regexp flags (i, g) are ignored
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type FoldingMarkers;
    #[wasm_bindgen(method, js_class = "FoldingMarkers", js_name = "start", js_namespace = languages, getter = start)]
    pub fn start(this: &FoldingMarkers) -> RegExp;
    /// Set the `start` property.
    #[wasm_bindgen(method, js_class = "FoldingMarkers", js_name = "start", js_namespace = languages, setter = start)]
    pub fn set_start(this: &FoldingMarkers, val: &RegExp);
    #[wasm_bindgen(method, js_class = "FoldingMarkers", js_name = "end", js_namespace = languages, getter = end)]
    pub fn end(this: &FoldingMarkers) -> RegExp;
    /// Set the `end` property.
    #[wasm_bindgen(method, js_class = "FoldingMarkers", js_name = "end", js_namespace = languages, setter = end)]
    pub fn set_end(this: &FoldingMarkers, val: &RegExp);
}

#[wasm_bindgen]
extern "C" {
    /// Describes folding rules for a language.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type FoldingRules;
    /// Used by the indentation based strategy to decide whether empty lines
    /// belong to the previous or the next block. A language adheres to the
    /// off-side rule if blocks in that language are expressed by their
    /// indentation. See [wikipedia](https://en.wikipedia.org/wiki/Off-side_rule) for more information.
    /// If not set, `false` is used and empty lines belong to the previous
    /// block.
    #[wasm_bindgen(method, js_class = "FoldingRules", js_name = "offSide", js_namespace = languages, getter = offSide)]
    pub fn off_side(this: &FoldingRules) -> Option<bool>;
    /// Set the `offSide` property.
    #[wasm_bindgen(method, js_class = "FoldingRules", js_name = "offSide", js_namespace = languages, setter = offSide)]
    pub fn set_off_side(this: &FoldingRules, val: Option<bool>);
    /// Region markers used by the language.
    #[wasm_bindgen(method, js_class = "FoldingRules", js_name = "markers", js_namespace = languages, getter = markers)]
    pub fn markers(this: &FoldingRules) -> Option<FoldingMarkers>;
    /// Set the `markers` property.
    #[wasm_bindgen(method, js_class = "FoldingRules", js_name = "markers", js_namespace = languages, setter = markers)]
    pub fn set_markers(this: &FoldingRules, val: Option<&FoldingMarkers>);
}

#[wasm_bindgen]
extern "C" {
    /// Describes a rule to be evaluated when pressing Enter.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type OnEnterRule;
    /// This rule will only execute if the text before the cursor matches this
    /// regular expression.
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "beforeText", js_namespace = languages, getter = beforeText)]
    pub fn before_text(this: &OnEnterRule) -> RegExp;
    /// Set the `beforeText` property.
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "beforeText", js_namespace = languages, setter = beforeText)]
    pub fn set_before_text(this: &OnEnterRule, val: &RegExp);
    /// This rule will only execute if the text after the cursor matches this
    /// regular expression.
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "afterText", js_namespace = languages, getter = afterText)]
    pub fn after_text(this: &OnEnterRule) -> Option<RegExp>;
    /// Set the `afterText` property.
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "afterText", js_namespace = languages, setter = afterText)]
    pub fn set_after_text(this: &OnEnterRule, val: Option<&RegExp>);
    /// This rule will only execute if the text above the this line matches this
    /// regular expression.
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "previousLineText", js_namespace = languages, getter = previousLineText)]
    pub fn previous_line_text(this: &OnEnterRule) -> Option<RegExp>;
    /// Set the `previousLineText` property.
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "previousLineText", js_namespace = languages, setter = previousLineText)]
    pub fn set_previous_line_text(this: &OnEnterRule, val: Option<&RegExp>);
    /// The action to execute.
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "action", js_namespace = languages, getter = action)]
    pub fn action(this: &OnEnterRule) -> EnterAction;
    /// Set the `action` property.
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "action", js_namespace = languages, setter = action)]
    pub fn set_action(this: &OnEnterRule, val: &EnterAction);
}

#[wasm_bindgen]
extern "C" {
    /// Definition of documentation comments (e.g. Javadoc/JSdoc)
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IDocComment;
    /// The string that starts a doc comment (e.g. '/**')
    #[wasm_bindgen(method, js_class = "IDocComment", js_name = "open", js_namespace = languages, getter = open)]
    pub fn open(this: &IDocComment) -> String;
    /// Set the `open` property.
    #[wasm_bindgen(method, js_class = "IDocComment", js_name = "open", js_namespace = languages, setter = open)]
    pub fn set_open(this: &IDocComment, val: &str);
    /// The string that appears on the last line and closes the doc comment
    /// (e.g. ' * /').
    #[wasm_bindgen(method, js_class = "IDocComment", js_name = "close", js_namespace = languages, getter = close)]
    pub fn close(this: &IDocComment) -> Option<String>;
    /// Set the `close` property.
    #[wasm_bindgen(method, js_class = "IDocComment", js_name = "close", js_namespace = languages, setter = close)]
    pub fn set_close(this: &IDocComment, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IAutoClosingPair;
    #[wasm_bindgen(method, js_class = "IAutoClosingPair", js_name = "open", js_namespace = languages, getter = open)]
    pub fn open(this: &IAutoClosingPair) -> String;
    /// Set the `open` property.
    #[wasm_bindgen(method, js_class = "IAutoClosingPair", js_name = "open", js_namespace = languages, setter = open)]
    pub fn set_open(this: &IAutoClosingPair, val: &str);
    #[wasm_bindgen(method, js_class = "IAutoClosingPair", js_name = "close", js_namespace = languages, getter = close)]
    pub fn close(this: &IAutoClosingPair) -> String;
    /// Set the `close` property.
    #[wasm_bindgen(method, js_class = "IAutoClosingPair", js_name = "close", js_namespace = languages, setter = close)]
    pub fn set_close(this: &IAutoClosingPair, val: &str);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object, extends = IAutoClosingPair)]
    pub type IAutoClosingPairConditional;
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "IAutoClosingPairConditional", js_name = "notIn", js_namespace = languages, getter = notIn)]
    pub fn not_in(this: &IAutoClosingPairConditional) -> Option<Array>;
    /// Set the `notIn` property.
    #[wasm_bindgen(method, js_class = "IAutoClosingPairConditional", js_name = "notIn", js_namespace = languages, setter = notIn)]
    pub fn set_not_in(this: &IAutoClosingPairConditional, val: Option<&Array>);
}

#[wasm_bindgen]
extern "C" {
    /// Describes what to do when pressing Enter.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type EnterAction;
    /// Describe what to do with the indentation.
    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "indentAction", js_namespace = languages, getter = indentAction)]
    pub fn indent_action(this: &EnterAction) -> IndentAction;
    /// Set the `indentAction` property.
    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "indentAction", js_namespace = languages, setter = indentAction)]
    pub fn set_indent_action(this: &EnterAction, val: IndentAction);
    /// Describes text to be appended after the new line and after the
    /// indentation.
    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "appendText", js_namespace = languages, getter = appendText)]
    pub fn append_text(this: &EnterAction) -> Option<String>;
    /// Set the `appendText` property.
    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "appendText", js_namespace = languages, setter = appendText)]
    pub fn set_append_text(this: &EnterAction, val: Option<&str>);
    /// Describes the number of characters to remove from the new line's
    /// indentation.
    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "removeText", js_namespace = languages, getter = removeText)]
    pub fn remove_text(this: &EnterAction) -> Option<f64>;
    /// Set the `removeText` property.
    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "removeText", js_namespace = languages, setter = removeText)]
    pub fn set_remove_text(this: &EnterAction, val: Option<f64>);
}

#[wasm_bindgen]
extern "C" {
    /// The state of the tokenizer between two lines.
    /// It is useful to store flags such as in multiline comment, etc.
    /// The model will clone the previous line's state and pass it in to
    /// tokenize the next line.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IState;
    #[wasm_bindgen(method, js_class = "IState", js_name = "clone", js_namespace = languages)]
    pub fn clone(this: &IState) -> IState;
    #[wasm_bindgen(method, js_class = "IState", js_name = "equals", js_namespace = languages)]
    pub fn equals(this: &IState, other: &IState) -> bool;
}

#[wasm_bindgen]
extern "C" {
    /// A hover represents additional information for a symbol or word. Hovers
    /// are rendered in a tooltip-like widget.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type Hover;
    /// The contents of this hover.
    ///
    /// Type: `IMarkdownString[]`
    #[wasm_bindgen(method, js_class = "Hover", js_name = "contents", js_namespace = languages, getter = contents)]
    pub fn contents(this: &Hover) -> Array;
    /// Set the `contents` property.
    #[wasm_bindgen(method, js_class = "Hover", js_name = "contents", js_namespace = languages, setter = contents)]
    pub fn set_contents(this: &Hover, val: &Array);
    /// The range to which this hover applies. When missing, the
    /// editor will use the range at the current position or the
    /// current position itself.
    #[wasm_bindgen(method, js_class = "Hover", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &Hover) -> Option<IRange>;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "Hover", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &Hover, val: Option<&IRange>);
}

#[wasm_bindgen]
extern "C" {
    /// The hover provider interface defines the contract between extensions and
    /// the [hover](https://code.visualstudio.com/docs/editor/intellisense)-feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type HoverProvider;
    /// Provide a hover for the given position and document. Multiple hovers at
    /// the same position will be merged by the editor. A hover can have a
    /// range which defaults to the word range at the position when omitted.
    #[wasm_bindgen(method, js_class = "HoverProvider", js_name = "provideHover", js_namespace = languages)]
    pub fn provide_hover(
        this: &HoverProvider,
        model: &ITextModel,
        position: &Position,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CompletionItemLabel;
    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "label", js_namespace = languages, getter = label)]
    pub fn label(this: &CompletionItemLabel) -> String;
    /// Set the `label` property.
    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "label", js_namespace = languages, setter = label)]
    pub fn set_label(this: &CompletionItemLabel, val: &str);
    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "detail", js_namespace = languages, getter = detail)]
    pub fn detail(this: &CompletionItemLabel) -> Option<String>;
    /// Set the `detail` property.
    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "detail", js_namespace = languages, setter = detail)]
    pub fn set_detail(this: &CompletionItemLabel, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "description", js_namespace = languages, getter = description)]
    pub fn description(this: &CompletionItemLabel) -> Option<String>;
    /// Set the `description` property.
    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "description", js_namespace = languages, setter = description)]
    pub fn set_description(this: &CompletionItemLabel, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CompletionItemRanges;
    #[wasm_bindgen(method, js_class = "CompletionItemRanges", js_name = "insert", js_namespace = languages, getter = insert)]
    pub fn insert(this: &CompletionItemRanges) -> IRange;
    /// Set the `insert` property.
    #[wasm_bindgen(method, js_class = "CompletionItemRanges", js_name = "insert", js_namespace = languages, setter = insert)]
    pub fn set_insert(this: &CompletionItemRanges, val: &IRange);
    #[wasm_bindgen(method, js_class = "CompletionItemRanges", js_name = "replace", js_namespace = languages, getter = replace)]
    pub fn replace(this: &CompletionItemRanges) -> IRange;
    /// Set the `replace` property.
    #[wasm_bindgen(method, js_class = "CompletionItemRanges", js_name = "replace", js_namespace = languages, setter = replace)]
    pub fn set_replace(this: &CompletionItemRanges, val: &IRange);
}

#[wasm_bindgen]
extern "C" {
    /// A completion item represents a text snippet that is
    /// proposed to complete text that is being typed.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CompletionItem;
    /// The label of this completion item. By default
    /// this is also the text that is inserted when selecting
    /// this completion.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "label", js_namespace = languages, getter = label)]
    pub fn label(this: &CompletionItem) -> JsValue;
    /// Set the `label` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "label", js_namespace = languages, setter = label)]
    pub fn set_label(this: &CompletionItem, val: &JsValue);
    /// The kind of this completion item. Based on the kind
    /// an icon is chosen by the editor.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "kind", js_namespace = languages, getter = kind)]
    pub fn kind(this: &CompletionItem) -> CompletionItemKind;
    /// Set the `kind` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "kind", js_namespace = languages, setter = kind)]
    pub fn set_kind(this: &CompletionItem, val: CompletionItemKind);
    /// A modifier to the `kind` which affect how the item
    /// is rendered, e.g. Deprecated is rendered with a strikeout
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "tags", js_namespace = languages, getter = tags)]
    pub fn tags(this: &CompletionItem) -> Option<Array>;
    /// Set the `tags` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "tags", js_namespace = languages, setter = tags)]
    pub fn set_tags(this: &CompletionItem, val: Option<&Array>);
    /// A human-readable string with additional information
    /// about this item, like type or symbol information.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "detail", js_namespace = languages, getter = detail)]
    pub fn detail(this: &CompletionItem) -> Option<String>;
    /// Set the `detail` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "detail", js_namespace = languages, setter = detail)]
    pub fn set_detail(this: &CompletionItem, val: Option<&str>);
    /// A human-readable string that represents a doc-comment.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "documentation", js_namespace = languages, getter = documentation)]
    pub fn documentation(this: &CompletionItem) -> JsValue;
    /// Set the `documentation` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "documentation", js_namespace = languages, setter = documentation)]
    pub fn set_documentation(this: &CompletionItem, val: &JsValue);
    /// A string that should be used when comparing this item
    /// with other items. When `falsy` the {@link CompletionItem.label label}
    /// is used.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "sortText", js_namespace = languages, getter = sortText)]
    pub fn sort_text(this: &CompletionItem) -> Option<String>;
    /// Set the `sortText` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "sortText", js_namespace = languages, setter = sortText)]
    pub fn set_sort_text(this: &CompletionItem, val: Option<&str>);
    /// A string that should be used when filtering a set of
    /// completion items. When `falsy` the {@link CompletionItem.label label}
    /// is used.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "filterText", js_namespace = languages, getter = filterText)]
    pub fn filter_text(this: &CompletionItem) -> Option<String>;
    /// Set the `filterText` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "filterText", js_namespace = languages, setter = filterText)]
    pub fn set_filter_text(this: &CompletionItem, val: Option<&str>);
    /// Select this item when showing. *Note* that only one completion item can
    /// be selected and that the editor decides which item that is. The rule
    /// is that the *first* item of those that match best is selected.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "preselect", js_namespace = languages, getter = preselect)]
    pub fn preselect(this: &CompletionItem) -> Option<bool>;
    /// Set the `preselect` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "preselect", js_namespace = languages, setter = preselect)]
    pub fn set_preselect(this: &CompletionItem, val: Option<bool>);
    /// A string or snippet that should be inserted in a document when selecting
    /// this completion.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "insertText", js_namespace = languages, getter = insertText)]
    pub fn insert_text(this: &CompletionItem) -> String;
    /// Set the `insertText` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "insertText", js_namespace = languages, setter = insertText)]
    pub fn set_insert_text(this: &CompletionItem, val: &str);
    /// Additional rules (as bitmask) that should be applied when inserting
    /// this completion.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "insertTextRules", js_namespace = languages, getter = insertTextRules)]
    pub fn insert_text_rules(this: &CompletionItem) -> Option<CompletionItemInsertTextRule>;
    /// Set the `insertTextRules` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "insertTextRules", js_namespace = languages, setter = insertTextRules)]
    pub fn set_insert_text_rules(this: &CompletionItem, val: Option<CompletionItemInsertTextRule>);
    /// A range of text that should be replaced by this completion item.
    ///
    /// Defaults to a range from the start of the {@link
    /// TextDocument.getWordRangeAtPosition current word} to the
    /// current position.
    ///
    /// *Note:* The range must be a {@link Range.isSingleLine single line} and
    /// it must {@link Range.contains contain} the position at which
    /// completion has been {@link CompletionItemProvider.provideCompletionItems
    /// requested}.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &CompletionItem) -> JsValue;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &CompletionItem, val: &JsValue);
    /// An optional set of characters that when pressed while this completion is
    /// active will accept it first and then type that character. *Note*
    /// that all commit characters should have `length=1` and that superfluous
    /// characters will be ignored.
    ///
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "commitCharacters", js_namespace = languages, getter = commitCharacters)]
    pub fn commit_characters(this: &CompletionItem) -> Option<Array>;
    /// Set the `commitCharacters` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "commitCharacters", js_namespace = languages, setter = commitCharacters)]
    pub fn set_commit_characters(this: &CompletionItem, val: Option<&Array>);
    /// An optional array of additional text edits that are applied when
    /// selecting this completion. Edits must not overlap with the main edit
    /// nor with themselves.
    ///
    /// Type: `editor.ISingleEditOperation[]`
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "additionalTextEdits", js_namespace = languages, getter = additionalTextEdits)]
    pub fn additional_text_edits(this: &CompletionItem) -> Option<Array>;
    /// Set the `additionalTextEdits` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "additionalTextEdits", js_namespace = languages, setter = additionalTextEdits)]
    pub fn set_additional_text_edits(this: &CompletionItem, val: Option<&Array>);
    /// A command that should be run upon acceptance of this item.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "command", js_namespace = languages, getter = command)]
    pub fn command(this: &CompletionItem) -> Option<Command>;
    /// Set the `command` property.
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "command", js_namespace = languages, setter = command)]
    pub fn set_command(this: &CompletionItem, val: Option<&Command>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CompletionList;
    /// Type: `CompletionItem[]`
    #[wasm_bindgen(method, js_class = "CompletionList", js_name = "suggestions", js_namespace = languages, getter = suggestions)]
    pub fn suggestions(this: &CompletionList) -> Array;
    /// Set the `suggestions` property.
    #[wasm_bindgen(method, js_class = "CompletionList", js_name = "suggestions", js_namespace = languages, setter = suggestions)]
    pub fn set_suggestions(this: &CompletionList, val: &Array);
    #[wasm_bindgen(method, js_class = "CompletionList", js_name = "incomplete", js_namespace = languages, getter = incomplete)]
    pub fn incomplete(this: &CompletionList) -> Option<bool>;
    /// Set the `incomplete` property.
    #[wasm_bindgen(method, js_class = "CompletionList", js_name = "incomplete", js_namespace = languages, setter = incomplete)]
    pub fn set_incomplete(this: &CompletionList, val: Option<bool>);
    /// Type: `(() => void)`
    #[wasm_bindgen(method, js_class = "CompletionList", js_name = "dispose", js_namespace = languages, getter = dispose)]
    pub fn dispose(this: &CompletionList) -> Option<Function>;
    /// Set the `dispose` property.
    #[wasm_bindgen(method, js_class = "CompletionList", js_name = "dispose", js_namespace = languages, setter = dispose)]
    pub fn set_dispose(this: &CompletionList, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    /// Contains additional information about the context in which
    /// {@link CompletionItemProvider.provideCompletionItems completion
    /// provider} is triggered.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CompletionContext;
    /// How the completion was triggered.
    #[wasm_bindgen(method, js_class = "CompletionContext", js_name = "triggerKind", js_namespace = languages, getter = triggerKind)]
    pub fn trigger_kind(this: &CompletionContext) -> CompletionTriggerKind;
    /// Set the `triggerKind` property.
    #[wasm_bindgen(method, js_class = "CompletionContext", js_name = "triggerKind", js_namespace = languages, setter = triggerKind)]
    pub fn set_trigger_kind(this: &CompletionContext, val: CompletionTriggerKind);
    /// Character that triggered the completion item provider.
    ///
    /// `undefined` if provider was not triggered by a character.
    #[wasm_bindgen(method, js_class = "CompletionContext", js_name = "triggerCharacter", js_namespace = languages, getter = triggerCharacter)]
    pub fn trigger_character(this: &CompletionContext) -> Option<String>;
    /// Set the `triggerCharacter` property.
    #[wasm_bindgen(method, js_class = "CompletionContext", js_name = "triggerCharacter", js_namespace = languages, setter = triggerCharacter)]
    pub fn set_trigger_character(this: &CompletionContext, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    /// The completion item provider interface defines the contract between
    /// extensions and the [IntelliSense](https://code.visualstudio.com/docs/editor/intellisense).
    ///
    /// When computing *complete* completion items is expensive, providers can
    /// optionally implement the `resolveCompletionItem`-function. In that
    /// case it is enough to return completion items with a {@link
    /// CompletionItem.label label} from the {@link CompletionItemProvider.
    /// provideCompletionItems provideCompletionItems}-function. Subsequently,
    /// when a completion item is shown in the UI and gains focus this provider
    /// is asked to resolve the item, like adding {@link
    /// CompletionItem.documentation doc-comment} or {@link
    /// CompletionItem.detail details}.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CompletionItemProvider;
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "CompletionItemProvider", js_name = "triggerCharacters", js_namespace = languages, getter = triggerCharacters)]
    pub fn trigger_characters(this: &CompletionItemProvider) -> Option<Array>;
    /// Set the `triggerCharacters` property.
    #[wasm_bindgen(method, js_class = "CompletionItemProvider", js_name = "triggerCharacters", js_namespace = languages, setter = triggerCharacters)]
    pub fn set_trigger_characters(this: &CompletionItemProvider, val: Option<&Array>);
    /// Provide completion items for the given position and document.
    #[wasm_bindgen(
        method,
        js_class = "CompletionItemProvider",
        js_name = "provideCompletionItems", js_namespace = languages
    )]
    pub fn provide_completion_items(
        this: &CompletionItemProvider,
        model: &ITextModel,
        position: &Position,
        context: &CompletionContext,
        token: &CancellationToken,
    ) -> JsValue;
    /// Given a completion item fill in more data, like {@link
    /// CompletionItem.documentation doc-comment} or {@link CompletionItem.
    /// detail details}.
    ///
    /// The editor will only resolve a completion item once.
    ///
    /// Type: `((item: CompletionItem, token: CancellationToken) => JsValue)`
    #[wasm_bindgen(method, js_class = "CompletionItemProvider", js_name = "resolveCompletionItem", js_namespace = languages, getter = resolveCompletionItem)]
    pub fn resolve_completion_item(this: &CompletionItemProvider) -> Option<Function>;
    /// Set the `resolveCompletionItem` property.
    #[wasm_bindgen(method, js_class = "CompletionItemProvider", js_name = "resolveCompletionItem", js_namespace = languages, setter = resolveCompletionItem)]
    pub fn set_resolve_completion_item(this: &CompletionItemProvider, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type InlineCompletionContext;
    /// How the completion was triggered.
    #[wasm_bindgen(method, js_class = "InlineCompletionContext", js_name = "triggerKind", js_namespace = languages, getter = triggerKind)]
    pub fn trigger_kind(this: &InlineCompletionContext) -> InlineCompletionTriggerKind;
    #[wasm_bindgen(method, js_class = "InlineCompletionContext", js_name = "selectedSuggestionInfo", js_namespace = languages, getter = selectedSuggestionInfo)]
    pub fn selected_suggestion_info(
        this: &InlineCompletionContext,
    ) -> Option<SelectedSuggestionInfo>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type SelectedSuggestionInfo;
    #[wasm_bindgen(method, js_class = "SelectedSuggestionInfo", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &SelectedSuggestionInfo) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "SelectedSuggestionInfo", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &SelectedSuggestionInfo, val: &IRange);
    #[wasm_bindgen(method, js_class = "SelectedSuggestionInfo", js_name = "text", js_namespace = languages, getter = text)]
    pub fn text(this: &SelectedSuggestionInfo) -> String;
    /// Set the `text` property.
    #[wasm_bindgen(method, js_class = "SelectedSuggestionInfo", js_name = "text", js_namespace = languages, setter = text)]
    pub fn set_text(this: &SelectedSuggestionInfo, val: &str);
    #[wasm_bindgen(method, js_class = "SelectedSuggestionInfo", js_name = "isSnippetText", js_namespace = languages, getter = isSnippetText)]
    pub fn is_snippet_text(this: &SelectedSuggestionInfo) -> bool;
    /// Set the `isSnippetText` property.
    #[wasm_bindgen(method, js_class = "SelectedSuggestionInfo", js_name = "isSnippetText", js_namespace = languages, setter = isSnippetText)]
    pub fn set_is_snippet_text(this: &SelectedSuggestionInfo, val: bool);
    #[wasm_bindgen(method, js_class = "SelectedSuggestionInfo", js_name = "completionKind", js_namespace = languages, getter = completionKind)]
    pub fn completion_kind(this: &SelectedSuggestionInfo) -> CompletionItemKind;
    /// Set the `completionKind` property.
    #[wasm_bindgen(method, js_class = "SelectedSuggestionInfo", js_name = "completionKind", js_namespace = languages, setter = completionKind)]
    pub fn set_completion_kind(this: &SelectedSuggestionInfo, val: CompletionItemKind);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type InlineCompletion;
    /// The text to insert.
    /// If the text contains a line break, the range must end at the end of a
    /// line. If existing text should be replaced, the existing text must be
    /// a prefix of the text to insert.
    #[wasm_bindgen(method, js_class = "InlineCompletion", js_name = "text", js_namespace = languages, getter = text)]
    pub fn text(this: &InlineCompletion) -> String;
    /// The range to replace.
    /// Must begin and end on the same line.
    #[wasm_bindgen(method, js_class = "InlineCompletion", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &InlineCompletion) -> Option<IRange>;
    #[wasm_bindgen(method, js_class = "InlineCompletion", js_name = "command", js_namespace = languages, getter = command)]
    pub fn command(this: &InlineCompletion) -> Option<Command>;
    /// If set to `true`, unopened closing brackets are removed and unclosed
    /// opening brackets are closed. Defaults to `false`.
    #[wasm_bindgen(method, js_class = "InlineCompletion", js_name = "completeBracketPairs", js_namespace = languages, getter = completeBracketPairs)]
    pub fn complete_bracket_pairs(this: &InlineCompletion) -> Option<bool>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type InlineCompletions;
    /// Type: `readonly TItem[]`
    #[wasm_bindgen(method, js_class = "InlineCompletions", js_name = "items", js_namespace = languages, getter = items)]
    pub fn items(this: &InlineCompletions) -> Array;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type InlineCompletionsProvider;
    #[wasm_bindgen(
        method,
        js_class = "InlineCompletionsProvider",
        js_name = "provideInlineCompletions", js_namespace = languages
    )]
    pub fn provide_inline_completions(
        this: &InlineCompletionsProvider,
        model: &ITextModel,
        position: &Position,
        context: &InlineCompletionContext,
        token: &CancellationToken,
    ) -> JsValue;
    /// Will be called when an item is shown.
    ///
    /// Type: `((completions: T, item: T['items'][number]) => void)`
    #[wasm_bindgen(method, js_class = "InlineCompletionsProvider", js_name = "handleItemDidShow", js_namespace = languages, getter = handleItemDidShow)]
    pub fn handle_item_did_show(this: &InlineCompletionsProvider) -> Option<Function>;
    /// Set the `handleItemDidShow` property.
    #[wasm_bindgen(method, js_class = "InlineCompletionsProvider", js_name = "handleItemDidShow", js_namespace = languages, setter = handleItemDidShow)]
    pub fn set_handle_item_did_show(this: &InlineCompletionsProvider, val: Option<&Function>);
    /// Will be called when a completions list is no longer in use and can be
    /// garbage-collected.
    #[wasm_bindgen(
        method,
        js_class = "InlineCompletionsProvider",
        js_name = "freeInlineCompletions", js_namespace = languages
    )]
    pub fn free_inline_completions(this: &InlineCompletionsProvider, completions: &JsValue);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CodeAction;
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "title", js_namespace = languages, getter = title)]
    pub fn title(this: &CodeAction) -> String;
    /// Set the `title` property.
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "title", js_namespace = languages, setter = title)]
    pub fn set_title(this: &CodeAction, val: &str);
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "command", js_namespace = languages, getter = command)]
    pub fn command(this: &CodeAction) -> Option<Command>;
    /// Set the `command` property.
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "command", js_namespace = languages, setter = command)]
    pub fn set_command(this: &CodeAction, val: Option<&Command>);
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "edit", js_namespace = languages, getter = edit)]
    pub fn edit(this: &CodeAction) -> Option<WorkspaceEdit>;
    /// Set the `edit` property.
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "edit", js_namespace = languages, setter = edit)]
    pub fn set_edit(this: &CodeAction, val: Option<&WorkspaceEdit>);
    /// Type: `editor.IMarkerData[]`
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "diagnostics", js_namespace = languages, getter = diagnostics)]
    pub fn diagnostics(this: &CodeAction) -> Option<Array>;
    /// Set the `diagnostics` property.
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "diagnostics", js_namespace = languages, setter = diagnostics)]
    pub fn set_diagnostics(this: &CodeAction, val: Option<&Array>);
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "kind", js_namespace = languages, getter = kind)]
    pub fn kind(this: &CodeAction) -> Option<String>;
    /// Set the `kind` property.
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "kind", js_namespace = languages, setter = kind)]
    pub fn set_kind(this: &CodeAction, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "isPreferred", js_namespace = languages, getter = isPreferred)]
    pub fn is_preferred(this: &CodeAction) -> Option<bool>;
    /// Set the `isPreferred` property.
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "isPreferred", js_namespace = languages, setter = isPreferred)]
    pub fn set_is_preferred(this: &CodeAction, val: Option<bool>);
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "disabled", js_namespace = languages, getter = disabled)]
    pub fn disabled(this: &CodeAction) -> Option<String>;
    /// Set the `disabled` property.
    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "disabled", js_namespace = languages, setter = disabled)]
    pub fn set_disabled(this: &CodeAction, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object, extends = IDisposable)]
    pub type CodeActionList;
    #[wasm_bindgen(method, js_class = "CodeActionList", js_name = "actions", js_namespace = languages, getter = actions)]
    pub fn actions(this: &CodeActionList) -> Array;
}

#[wasm_bindgen]
extern "C" {
    /// Represents a parameter of a callable-signature. A parameter can
    /// have a label and a doc-comment.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ParameterInformation;
    /// The label of this signature. Will be shown in
    /// the UI.
    #[wasm_bindgen(method, js_class = "ParameterInformation", js_name = "label", js_namespace = languages, getter = label)]
    pub fn label(this: &ParameterInformation) -> JsValue;
    /// Set the `label` property.
    #[wasm_bindgen(method, js_class = "ParameterInformation", js_name = "label", js_namespace = languages, setter = label)]
    pub fn set_label(this: &ParameterInformation, val: &JsValue);
    /// The human-readable doc-comment of this signature. Will be shown
    /// in the UI but can be omitted.
    #[wasm_bindgen(method, js_class = "ParameterInformation", js_name = "documentation", js_namespace = languages, getter = documentation)]
    pub fn documentation(this: &ParameterInformation) -> JsValue;
    /// Set the `documentation` property.
    #[wasm_bindgen(method, js_class = "ParameterInformation", js_name = "documentation", js_namespace = languages, setter = documentation)]
    pub fn set_documentation(this: &ParameterInformation, val: &JsValue);
}

#[wasm_bindgen]
extern "C" {
    /// Represents the signature of something callable. A signature
    /// can have a label, like a function-name, a doc-comment, and
    /// a set of parameters.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type SignatureInformation;
    /// The label of this signature. Will be shown in
    /// the UI.
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "label", js_namespace = languages, getter = label)]
    pub fn label(this: &SignatureInformation) -> String;
    /// Set the `label` property.
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "label", js_namespace = languages, setter = label)]
    pub fn set_label(this: &SignatureInformation, val: &str);
    /// The human-readable doc-comment of this signature. Will be shown
    /// in the UI but can be omitted.
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "documentation", js_namespace = languages, getter = documentation)]
    pub fn documentation(this: &SignatureInformation) -> JsValue;
    /// Set the `documentation` property.
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "documentation", js_namespace = languages, setter = documentation)]
    pub fn set_documentation(this: &SignatureInformation, val: &JsValue);
    /// The parameters of this signature.
    ///
    /// Type: `ParameterInformation[]`
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "parameters", js_namespace = languages, getter = parameters)]
    pub fn parameters(this: &SignatureInformation) -> Array;
    /// Set the `parameters` property.
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "parameters", js_namespace = languages, setter = parameters)]
    pub fn set_parameters(this: &SignatureInformation, val: &Array);
    /// Index of the active parameter.
    ///
    /// If provided, this is used in place of `SignatureHelp.activeSignature`.
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "activeParameter", js_namespace = languages, getter = activeParameter)]
    pub fn active_parameter(this: &SignatureInformation) -> Option<f64>;
    /// Set the `activeParameter` property.
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "activeParameter", js_namespace = languages, setter = activeParameter)]
    pub fn set_active_parameter(this: &SignatureInformation, val: Option<f64>);
}

#[wasm_bindgen]
extern "C" {
    /// Signature help represents the signature of something
    /// callable. There can be multiple signatures but only one
    /// active and only one active parameter.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type SignatureHelp;
    /// One or more signatures.
    ///
    /// Type: `SignatureInformation[]`
    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "signatures", js_namespace = languages, getter = signatures)]
    pub fn signatures(this: &SignatureHelp) -> Array;
    /// Set the `signatures` property.
    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "signatures", js_namespace = languages, setter = signatures)]
    pub fn set_signatures(this: &SignatureHelp, val: &Array);
    /// The active signature.
    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "activeSignature", js_namespace = languages, getter = activeSignature)]
    pub fn active_signature(this: &SignatureHelp) -> f64;
    /// Set the `activeSignature` property.
    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "activeSignature", js_namespace = languages, setter = activeSignature)]
    pub fn set_active_signature(this: &SignatureHelp, val: f64);
    /// The active parameter of the active signature.
    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "activeParameter", js_namespace = languages, getter = activeParameter)]
    pub fn active_parameter(this: &SignatureHelp) -> f64;
    /// Set the `activeParameter` property.
    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "activeParameter", js_namespace = languages, setter = activeParameter)]
    pub fn set_active_parameter(this: &SignatureHelp, val: f64);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object, extends = IDisposable)]
    pub type SignatureHelpResult;
    #[wasm_bindgen(method, js_class = "SignatureHelpResult", js_name = "value", js_namespace = languages, getter = value)]
    pub fn value(this: &SignatureHelpResult) -> SignatureHelp;
    /// Set the `value` property.
    #[wasm_bindgen(method, js_class = "SignatureHelpResult", js_name = "value", js_namespace = languages, setter = value)]
    pub fn set_value(this: &SignatureHelpResult, val: &SignatureHelp);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type SignatureHelpContext;
    #[wasm_bindgen(method, js_class = "SignatureHelpContext", js_name = "triggerKind", js_namespace = languages, getter = triggerKind)]
    pub fn trigger_kind(this: &SignatureHelpContext) -> SignatureHelpTriggerKind;
    #[wasm_bindgen(method, js_class = "SignatureHelpContext", js_name = "triggerCharacter", js_namespace = languages, getter = triggerCharacter)]
    pub fn trigger_character(this: &SignatureHelpContext) -> Option<String>;
    #[wasm_bindgen(method, js_class = "SignatureHelpContext", js_name = "isRetrigger", js_namespace = languages, getter = isRetrigger)]
    pub fn is_retrigger(this: &SignatureHelpContext) -> bool;
    #[wasm_bindgen(method, js_class = "SignatureHelpContext", js_name = "activeSignatureHelp", js_namespace = languages, getter = activeSignatureHelp)]
    pub fn active_signature_help(this: &SignatureHelpContext) -> Option<SignatureHelp>;
}

#[wasm_bindgen]
extern "C" {
    /// The signature help provider interface defines the contract between
    /// extensions and the [parameter hints](https://code.visualstudio.com/docs/editor/intellisense)-feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type SignatureHelpProvider;
    #[wasm_bindgen(method, js_class = "SignatureHelpProvider", js_name = "signatureHelpTriggerCharacters", js_namespace = languages, getter = signatureHelpTriggerCharacters)]
    pub fn signature_help_trigger_characters(this: &SignatureHelpProvider) -> Option<Array>;
    #[wasm_bindgen(method, js_class = "SignatureHelpProvider", js_name = "signatureHelpRetriggerCharacters", js_namespace = languages, getter = signatureHelpRetriggerCharacters)]
    pub fn signature_help_retrigger_characters(this: &SignatureHelpProvider) -> Option<Array>;
    /// Provide help for the signature at the given position and document.
    #[wasm_bindgen(
        method,
        js_class = "SignatureHelpProvider",
        js_name = "provideSignatureHelp", js_namespace = languages
    )]
    pub fn provide_signature_help(
        this: &SignatureHelpProvider,
        model: &ITextModel,
        position: &Position,
        token: &CancellationToken,
        context: &SignatureHelpContext,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// A document highlight is a range inside a text document which deserves
    /// special attention. Usually a document highlight is visualized by
    /// changing the background color of its range.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DocumentHighlight;
    /// The range this highlight applies to.
    #[wasm_bindgen(method, js_class = "DocumentHighlight", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &DocumentHighlight) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "DocumentHighlight", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &DocumentHighlight, val: &IRange);
    /// The highlight kind, default is {@link DocumentHighlightKind.Text text}.
    #[wasm_bindgen(method, js_class = "DocumentHighlight", js_name = "kind", js_namespace = languages, getter = kind)]
    pub fn kind(this: &DocumentHighlight) -> Option<DocumentHighlightKind>;
    /// Set the `kind` property.
    #[wasm_bindgen(method, js_class = "DocumentHighlight", js_name = "kind", js_namespace = languages, setter = kind)]
    pub fn set_kind(this: &DocumentHighlight, val: Option<DocumentHighlightKind>);
}

#[wasm_bindgen]
extern "C" {
    /// The document highlight provider interface defines the contract between
    /// extensions and the word-highlight-feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DocumentHighlightProvider;
    /// Provide a set of document highlights, like all occurrences of a variable
    /// or all exit-points of a function.
    #[wasm_bindgen(
        method,
        js_class = "DocumentHighlightProvider",
        js_name = "provideDocumentHighlights", js_namespace = languages
    )]
    pub fn provide_document_highlights(
        this: &DocumentHighlightProvider,
        model: &ITextModel,
        position: &Position,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// The linked editing range provider interface defines the contract between
    /// extensions and the linked editing feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type LinkedEditingRangeProvider;
    /// Provide a list of ranges that can be edited together.
    #[wasm_bindgen(
        method,
        js_class = "LinkedEditingRangeProvider",
        js_name = "provideLinkedEditingRanges", js_namespace = languages
    )]
    pub fn provide_linked_editing_ranges(
        this: &LinkedEditingRangeProvider,
        model: &ITextModel,
        position: &Position,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// Represents a list of ranges that can be edited together along with a
    /// word pattern to describe valid contents.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type LinkedEditingRanges;
    /// A list of ranges that can be edited together. The ranges must have
    /// identical length and text content. The ranges cannot overlap
    ///
    /// Type: `IRange[]`
    #[wasm_bindgen(method, js_class = "LinkedEditingRanges", js_name = "ranges", js_namespace = languages, getter = ranges)]
    pub fn ranges(this: &LinkedEditingRanges) -> Array;
    /// Set the `ranges` property.
    #[wasm_bindgen(method, js_class = "LinkedEditingRanges", js_name = "ranges", js_namespace = languages, setter = ranges)]
    pub fn set_ranges(this: &LinkedEditingRanges, val: &Array);
    /// An optional word pattern that describes valid contents for the given
    /// ranges. If no pattern is provided, the language configuration's word
    /// pattern will be used.
    #[wasm_bindgen(method, js_class = "LinkedEditingRanges", js_name = "wordPattern", js_namespace = languages, getter = wordPattern)]
    pub fn word_pattern(this: &LinkedEditingRanges) -> Option<RegExp>;
    /// Set the `wordPattern` property.
    #[wasm_bindgen(method, js_class = "LinkedEditingRanges", js_name = "wordPattern", js_namespace = languages, setter = wordPattern)]
    pub fn set_word_pattern(this: &LinkedEditingRanges, val: Option<&RegExp>);
}

#[wasm_bindgen]
extern "C" {
    /// Value-object that contains additional information when
    /// requesting references.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ReferenceContext;
    /// Include the declaration of the current symbol.
    #[wasm_bindgen(method, js_class = "ReferenceContext", js_name = "includeDeclaration", js_namespace = languages, getter = includeDeclaration)]
    pub fn include_declaration(this: &ReferenceContext) -> bool;
    /// Set the `includeDeclaration` property.
    #[wasm_bindgen(method, js_class = "ReferenceContext", js_name = "includeDeclaration", js_namespace = languages, setter = includeDeclaration)]
    pub fn set_include_declaration(this: &ReferenceContext, val: bool);
}

#[wasm_bindgen]
extern "C" {
    /// The reference provider interface defines the contract between extensions
    /// and the [find references](https://code.visualstudio.com/docs/editor/editingevolved#_peek)-feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ReferenceProvider;
    /// Provide a set of project-wide references for the given position and
    /// document.
    #[wasm_bindgen(method, js_class = "ReferenceProvider", js_name = "provideReferences", js_namespace = languages)]
    pub fn provide_references(
        this: &ReferenceProvider,
        model: &ITextModel,
        position: &Position,
        context: &ReferenceContext,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// Represents a location inside a resource, such as a line
    /// inside a text file.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type Location;
    /// The resource identifier of this location.
    #[wasm_bindgen(method, js_class = "Location", js_name = "uri", js_namespace = languages, getter = uri)]
    pub fn uri(this: &Location) -> Uri;
    /// Set the `uri` property.
    #[wasm_bindgen(method, js_class = "Location", js_name = "uri", js_namespace = languages, setter = uri)]
    pub fn set_uri(this: &Location, val: &Uri);
    /// The document range of this locations.
    #[wasm_bindgen(method, js_class = "Location", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &Location) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "Location", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &Location, val: &IRange);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type LocationLink;
    /// A range to select where this link originates from.
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "originSelectionRange", js_namespace = languages, getter = originSelectionRange)]
    pub fn origin_selection_range(this: &LocationLink) -> Option<IRange>;
    /// Set the `originSelectionRange` property.
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "originSelectionRange", js_namespace = languages, setter = originSelectionRange)]
    pub fn set_origin_selection_range(this: &LocationLink, val: Option<&IRange>);
    /// The target uri this link points to.
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "uri", js_namespace = languages, getter = uri)]
    pub fn uri(this: &LocationLink) -> Uri;
    /// Set the `uri` property.
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "uri", js_namespace = languages, setter = uri)]
    pub fn set_uri(this: &LocationLink, val: &Uri);
    /// The full range this link points to.
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &LocationLink) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &LocationLink, val: &IRange);
    /// A range to select this link points to. Must be contained
    /// in `LocationLink.range`.
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "targetSelectionRange", js_namespace = languages, getter = targetSelectionRange)]
    pub fn target_selection_range(this: &LocationLink) -> Option<IRange>;
    /// Set the `targetSelectionRange` property.
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "targetSelectionRange", js_namespace = languages, setter = targetSelectionRange)]
    pub fn set_target_selection_range(this: &LocationLink, val: Option<&IRange>);
}

#[wasm_bindgen]
extern "C" {
    /// The definition provider interface defines the contract between
    /// extensions and the [go to definition](https://code.visualstudio.com/docs/editor/editingevolved#_go-to-definition)
    /// and peek definition features.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DefinitionProvider;
    /// Provide the definition of the symbol at the given position and document.
    #[wasm_bindgen(method, js_class = "DefinitionProvider", js_name = "provideDefinition", js_namespace = languages)]
    pub fn provide_definition(
        this: &DefinitionProvider,
        model: &ITextModel,
        position: &Position,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// The definition provider interface defines the contract between
    /// extensions and the [go to definition](https://code.visualstudio.com/docs/editor/editingevolved#_go-to-definition)
    /// and peek definition features.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DeclarationProvider;
    /// Provide the declaration of the symbol at the given position and
    /// document.
    #[wasm_bindgen(
        method,
        js_class = "DeclarationProvider",
        js_name = "provideDeclaration", js_namespace = languages
    )]
    pub fn provide_declaration(
        this: &DeclarationProvider,
        model: &ITextModel,
        position: &Position,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// The implementation provider interface defines the contract between
    /// extensions and the go to implementation feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ImplementationProvider;
    /// Provide the implementation of the symbol at the given position and
    /// document.
    #[wasm_bindgen(
        method,
        js_class = "ImplementationProvider",
        js_name = "provideImplementation", js_namespace = languages
    )]
    pub fn provide_implementation(
        this: &ImplementationProvider,
        model: &ITextModel,
        position: &Position,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// The type definition provider interface defines the contract between
    /// extensions and the go to type definition feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type TypeDefinitionProvider;
    /// Provide the type definition of the symbol at the given position and
    /// document.
    #[wasm_bindgen(
        method,
        js_class = "TypeDefinitionProvider",
        js_name = "provideTypeDefinition", js_namespace = languages
    )]
    pub fn provide_type_definition(
        this: &TypeDefinitionProvider,
        model: &ITextModel,
        position: &Position,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DocumentSymbol;
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "name", js_namespace = languages, getter = name)]
    pub fn name(this: &DocumentSymbol) -> String;
    /// Set the `name` property.
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "name", js_namespace = languages, setter = name)]
    pub fn set_name(this: &DocumentSymbol, val: &str);
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "detail", js_namespace = languages, getter = detail)]
    pub fn detail(this: &DocumentSymbol) -> String;
    /// Set the `detail` property.
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "detail", js_namespace = languages, setter = detail)]
    pub fn set_detail(this: &DocumentSymbol, val: &str);
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "kind", js_namespace = languages, getter = kind)]
    pub fn kind(this: &DocumentSymbol) -> SymbolKind;
    /// Set the `kind` property.
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "kind", js_namespace = languages, setter = kind)]
    pub fn set_kind(this: &DocumentSymbol, val: SymbolKind);
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "tags", js_namespace = languages, getter = tags)]
    pub fn tags(this: &DocumentSymbol) -> Array;
    /// Set the `tags` property.
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "tags", js_namespace = languages, setter = tags)]
    pub fn set_tags(this: &DocumentSymbol, val: &Array);
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "containerName", js_namespace = languages, getter = containerName)]
    pub fn container_name(this: &DocumentSymbol) -> Option<String>;
    /// Set the `containerName` property.
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "containerName", js_namespace = languages, setter = containerName)]
    pub fn set_container_name(this: &DocumentSymbol, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &DocumentSymbol) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &DocumentSymbol, val: &IRange);
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "selectionRange", js_namespace = languages, getter = selectionRange)]
    pub fn selection_range(this: &DocumentSymbol) -> IRange;
    /// Set the `selectionRange` property.
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "selectionRange", js_namespace = languages, setter = selectionRange)]
    pub fn set_selection_range(this: &DocumentSymbol, val: &IRange);
    /// Type: `DocumentSymbol[]`
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "children", js_namespace = languages, getter = children)]
    pub fn children(this: &DocumentSymbol) -> Option<Array>;
    /// Set the `children` property.
    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "children", js_namespace = languages, setter = children)]
    pub fn set_children(this: &DocumentSymbol, val: Option<&Array>);
}

#[wasm_bindgen]
extern "C" {
    /// The document symbol provider interface defines the contract between
    /// extensions and the [go to symbol](https://code.visualstudio.com/docs/editor/editingevolved#_go-to-symbol)-feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DocumentSymbolProvider;
    #[wasm_bindgen(method, js_class = "DocumentSymbolProvider", js_name = "displayName", js_namespace = languages, getter = displayName)]
    pub fn display_name(this: &DocumentSymbolProvider) -> Option<String>;
    /// Set the `displayName` property.
    #[wasm_bindgen(method, js_class = "DocumentSymbolProvider", js_name = "displayName", js_namespace = languages, setter = displayName)]
    pub fn set_display_name(this: &DocumentSymbolProvider, val: Option<&str>);
    /// Provide symbol information for the given document.
    #[wasm_bindgen(
        method,
        js_class = "DocumentSymbolProvider",
        js_name = "provideDocumentSymbols", js_namespace = languages
    )]
    pub fn provide_document_symbols(
        this: &DocumentSymbolProvider,
        model: &ITextModel,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// Interface used to format a model
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type FormattingOptions;
    /// Size of a tab in spaces.
    #[wasm_bindgen(method, js_class = "FormattingOptions", js_name = "tabSize", js_namespace = languages, getter = tabSize)]
    pub fn tab_size(this: &FormattingOptions) -> f64;
    /// Set the `tabSize` property.
    #[wasm_bindgen(method, js_class = "FormattingOptions", js_name = "tabSize", js_namespace = languages, setter = tabSize)]
    pub fn set_tab_size(this: &FormattingOptions, val: f64);
    /// Prefer spaces over tabs.
    #[wasm_bindgen(method, js_class = "FormattingOptions", js_name = "insertSpaces", js_namespace = languages, getter = insertSpaces)]
    pub fn insert_spaces(this: &FormattingOptions) -> bool;
    /// Set the `insertSpaces` property.
    #[wasm_bindgen(method, js_class = "FormattingOptions", js_name = "insertSpaces", js_namespace = languages, setter = insertSpaces)]
    pub fn set_insert_spaces(this: &FormattingOptions, val: bool);
}

#[wasm_bindgen]
extern "C" {
    /// The document formatting provider interface defines the contract between
    /// extensions and the formatting-feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DocumentFormattingEditProvider;
    #[wasm_bindgen(method, js_class = "DocumentFormattingEditProvider", js_name = "displayName", js_namespace = languages, getter = displayName)]
    pub fn display_name(this: &DocumentFormattingEditProvider) -> Option<String>;
    /// Provide formatting edits for a whole document.
    #[wasm_bindgen(
        method,
        js_class = "DocumentFormattingEditProvider",
        js_name = "provideDocumentFormattingEdits", js_namespace = languages
    )]
    pub fn provide_document_formatting_edits(
        this: &DocumentFormattingEditProvider,
        model: &ITextModel,
        options: &FormattingOptions,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// The document formatting provider interface defines the contract between
    /// extensions and the formatting-feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DocumentRangeFormattingEditProvider;
    #[wasm_bindgen(method, js_class = "DocumentRangeFormattingEditProvider", js_name = "displayName", js_namespace = languages, getter = displayName)]
    pub fn display_name(this: &DocumentRangeFormattingEditProvider) -> Option<String>;
    /// Provide formatting edits for a range in a document.
    ///
    /// The given range is a hint and providers can decide to format a smaller
    /// or larger range. Often this is done by adjusting the start and end
    /// of the range to full syntax nodes.
    #[wasm_bindgen(
        method,
        js_class = "DocumentRangeFormattingEditProvider",
        js_name = "provideDocumentRangeFormattingEdits", js_namespace = languages
    )]
    pub fn provide_document_range_formatting_edits(
        this: &DocumentRangeFormattingEditProvider,
        model: &ITextModel,
        range: &Range,
        options: &FormattingOptions,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// The document formatting provider interface defines the contract between
    /// extensions and the formatting-feature.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type OnTypeFormattingEditProvider;
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "OnTypeFormattingEditProvider", js_name = "autoFormatTriggerCharacters", js_namespace = languages, getter = autoFormatTriggerCharacters)]
    pub fn auto_format_trigger_characters(this: &OnTypeFormattingEditProvider) -> Array;
    /// Set the `autoFormatTriggerCharacters` property.
    #[wasm_bindgen(method, js_class = "OnTypeFormattingEditProvider", js_name = "autoFormatTriggerCharacters", js_namespace = languages, setter = autoFormatTriggerCharacters)]
    pub fn set_auto_format_trigger_characters(this: &OnTypeFormattingEditProvider, val: &Array);
    /// Provide formatting edits after a character has been typed.
    ///
    /// The given position and character should hint to the provider
    /// what range the position to expand to, like find the matching `{`
    /// when `}` has been entered.
    #[wasm_bindgen(
        method,
        js_class = "OnTypeFormattingEditProvider",
        js_name = "provideOnTypeFormattingEdits", js_namespace = languages
    )]
    pub fn provide_on_type_formatting_edits(
        this: &OnTypeFormattingEditProvider,
        model: &ITextModel,
        position: &Position,
        ch: &str,
        options: &FormattingOptions,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// A link inside the editor.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ILink;
    #[wasm_bindgen(method, js_class = "ILink", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &ILink) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "ILink", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &ILink, val: &IRange);
    #[wasm_bindgen(method, js_class = "ILink", js_name = "url", js_namespace = languages, getter = url)]
    pub fn url(this: &ILink) -> JsValue;
    /// Set the `url` property.
    #[wasm_bindgen(method, js_class = "ILink", js_name = "url", js_namespace = languages, setter = url)]
    pub fn set_url(this: &ILink, val: &JsValue);
    #[wasm_bindgen(method, js_class = "ILink", js_name = "tooltip", js_namespace = languages, getter = tooltip)]
    pub fn tooltip(this: &ILink) -> Option<String>;
    /// Set the `tooltip` property.
    #[wasm_bindgen(method, js_class = "ILink", js_name = "tooltip", js_namespace = languages, setter = tooltip)]
    pub fn set_tooltip(this: &ILink, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ILinksList;
    /// Type: `ILink[]`
    #[wasm_bindgen(method, js_class = "ILinksList", js_name = "links", js_namespace = languages, getter = links)]
    pub fn links(this: &ILinksList) -> Array;
    /// Set the `links` property.
    #[wasm_bindgen(method, js_class = "ILinksList", js_name = "links", js_namespace = languages, setter = links)]
    pub fn set_links(this: &ILinksList, val: &Array);
    /// Type: `(() => void)`
    #[wasm_bindgen(method, js_class = "ILinksList", js_name = "dispose", js_namespace = languages, getter = dispose)]
    pub fn dispose(this: &ILinksList) -> Option<Function>;
    /// Set the `dispose` property.
    #[wasm_bindgen(method, js_class = "ILinksList", js_name = "dispose", js_namespace = languages, setter = dispose)]
    pub fn set_dispose(this: &ILinksList, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    /// A provider of links.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type LinkProvider;
    #[wasm_bindgen(method, js_class = "LinkProvider", js_name = "provideLinks", js_namespace = languages)]
    pub fn provide_links(
        this: &LinkProvider,
        model: &ITextModel,
        token: &CancellationToken,
    ) -> JsValue;
    /// Type: `(link: ILink, token: CancellationToken) => JsValue`
    #[wasm_bindgen(method, js_class = "LinkProvider", js_name = "resolveLink", js_namespace = languages, getter = resolveLink)]
    pub fn resolve_link(this: &LinkProvider) -> Option<Function>;
    /// Set the `resolveLink` property.
    #[wasm_bindgen(method, js_class = "LinkProvider", js_name = "resolveLink", js_namespace = languages, setter = resolveLink)]
    pub fn set_resolve_link(this: &LinkProvider, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    /// A color in RGBA format.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IColor;
    /// The red component in the range [0-1].
    #[wasm_bindgen(method, js_class = "IColor", js_name = "red", js_namespace = languages, getter = red)]
    pub fn red(this: &IColor) -> f64;
    /// The green component in the range [0-1].
    #[wasm_bindgen(method, js_class = "IColor", js_name = "green", js_namespace = languages, getter = green)]
    pub fn green(this: &IColor) -> f64;
    /// The blue component in the range [0-1].
    #[wasm_bindgen(method, js_class = "IColor", js_name = "blue", js_namespace = languages, getter = blue)]
    pub fn blue(this: &IColor) -> f64;
    /// The alpha component in the range [0-1].
    #[wasm_bindgen(method, js_class = "IColor", js_name = "alpha", js_namespace = languages, getter = alpha)]
    pub fn alpha(this: &IColor) -> f64;
}

#[wasm_bindgen]
extern "C" {
    /// String representations for a color
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IColorPresentation;
    /// The label of this color presentation. It will be shown on the color
    /// picker header. By default this is also the text that is inserted when
    /// selecting this color presentation.
    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "label", js_namespace = languages, getter = label)]
    pub fn label(this: &IColorPresentation) -> String;
    /// Set the `label` property.
    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "label", js_namespace = languages, setter = label)]
    pub fn set_label(this: &IColorPresentation, val: &str);
    /// An {@link TextEdit edit} which is applied to a document when selecting
    /// this presentation for the color.
    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "textEdit", js_namespace = languages, getter = textEdit)]
    pub fn text_edit(this: &IColorPresentation) -> JsValue;
    /// Set the `textEdit` property.
    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "textEdit", js_namespace = languages, setter = textEdit)]
    pub fn set_text_edit(this: &IColorPresentation, val: &JsValue);
    /// An optional array of additional {@link TextEdit text edits} that are
    /// applied when selecting this color presentation.
    ///
    /// Type: `TextEdit[]`
    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "additionalTextEdits", js_namespace = languages, getter = additionalTextEdits)]
    pub fn additional_text_edits(this: &IColorPresentation) -> Option<Array>;
    /// Set the `additionalTextEdits` property.
    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "additionalTextEdits", js_namespace = languages, setter = additionalTextEdits)]
    pub fn set_additional_text_edits(this: &IColorPresentation, val: Option<&Array>);
}

#[wasm_bindgen]
extern "C" {
    /// A color range is a range in a text model which represents a color.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IColorInformation;
    /// The range within the model.
    #[wasm_bindgen(method, js_class = "IColorInformation", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &IColorInformation) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "IColorInformation", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &IColorInformation, val: &IRange);
    /// The color represented in this range.
    #[wasm_bindgen(method, js_class = "IColorInformation", js_name = "color", js_namespace = languages, getter = color)]
    pub fn color(this: &IColorInformation) -> IColor;
    /// Set the `color` property.
    #[wasm_bindgen(method, js_class = "IColorInformation", js_name = "color", js_namespace = languages, setter = color)]
    pub fn set_color(this: &IColorInformation, val: &IColor);
}

#[wasm_bindgen]
extern "C" {
    /// A provider of colors for editor models.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DocumentColorProvider;
    /// Provides the color ranges for a specific model.
    #[wasm_bindgen(
        method,
        js_class = "DocumentColorProvider",
        js_name = "provideDocumentColors", js_namespace = languages
    )]
    pub fn provide_document_colors(
        this: &DocumentColorProvider,
        model: &ITextModel,
        token: &CancellationToken,
    ) -> JsValue;
    /// Provide the string representations for a color.
    #[wasm_bindgen(
        method,
        js_class = "DocumentColorProvider",
        js_name = "provideColorPresentations", js_namespace = languages
    )]
    pub fn provide_color_presentations(
        this: &DocumentColorProvider,
        model: &ITextModel,
        color_info: &IColorInformation,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type SelectionRange;
    #[wasm_bindgen(method, js_class = "SelectionRange", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &SelectionRange) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "SelectionRange", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &SelectionRange, val: &IRange);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type SelectionRangeProvider;
    /// Provide ranges that should be selected from the given position.
    ///
    /// # Arguments
    ///
    /// * `positions` - `Position[]`
    #[wasm_bindgen(
        method,
        js_class = "SelectionRangeProvider",
        js_name = "provideSelectionRanges", js_namespace = languages
    )]
    pub fn provide_selection_ranges(
        this: &SelectionRangeProvider,
        model: &ITextModel,
        positions: &Array,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    /// A provider of folding ranges for editor models.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type FoldingRangeProvider;
    /// An optional event to signal that the folding ranges from this provider
    /// have changed.
    #[wasm_bindgen(method, js_class = "FoldingRangeProvider", js_name = "onDidChange", js_namespace = languages, getter = onDidChange)]
    pub fn on_did_change(this: &FoldingRangeProvider) -> Option<Function>;
    /// Set the `onDidChange` property.
    #[wasm_bindgen(method, js_class = "FoldingRangeProvider", js_name = "onDidChange", js_namespace = languages, setter = onDidChange)]
    pub fn set_on_did_change(this: &FoldingRangeProvider, val: Option<&Function>);
    /// Provides the folding ranges for a specific model.
    ///
    /// Type: `(model: editor.ITextModel, context: FoldingContext, token:
    /// CancellationToken) => JsValue`
    #[wasm_bindgen(method, js_class = "FoldingRangeProvider", js_name = "provideFoldingRanges", js_namespace = languages, getter = provideFoldingRanges)]
    pub fn provide_folding_ranges(this: &FoldingRangeProvider) -> Function;
    /// Set the `provideFoldingRanges` property.
    #[wasm_bindgen(method, js_class = "FoldingRangeProvider", js_name = "provideFoldingRanges", js_namespace = languages, setter = provideFoldingRanges)]
    pub fn set_provide_folding_ranges(this: &FoldingRangeProvider, val: &Function);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type FoldingRange;
    /// The one-based start line of the range to fold. The folded area starts
    /// after the line's last character.
    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "start", js_namespace = languages, getter = start)]
    pub fn start(this: &FoldingRange) -> f64;
    /// Set the `start` property.
    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "start", js_namespace = languages, setter = start)]
    pub fn set_start(this: &FoldingRange, val: f64);
    /// The one-based end line of the range to fold. The folded area ends with
    /// the line's last character.
    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "end", js_namespace = languages, getter = end)]
    pub fn end(this: &FoldingRange) -> f64;
    /// Set the `end` property.
    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "end", js_namespace = languages, setter = end)]
    pub fn set_end(this: &FoldingRange, val: f64);
    /// Describes the {@link FoldingRangeKind Kind} of the folding range such as
    /// {@link FoldingRangeKind.Comment Comment} or {@link FoldingRangeKind.
    /// Region Region}. The kind is used to categorize folding ranges and used
    /// by commands like 'Fold all comments'. See
    /// {@link FoldingRangeKind} for an enumeration of standardized kinds.
    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "kind", js_namespace = languages, getter = kind)]
    pub fn kind(this: &FoldingRange) -> Option<FoldingRangeKind>;
    /// Set the `kind` property.
    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "kind", js_namespace = languages, setter = kind)]
    pub fn set_kind(this: &FoldingRange, val: Option<&FoldingRangeKind>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceEditMetadata;
    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "needsConfirmation", js_namespace = languages, getter = needsConfirmation)]
    pub fn needs_confirmation(this: &WorkspaceEditMetadata) -> bool;
    /// Set the `needsConfirmation` property.
    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "needsConfirmation", js_namespace = languages, setter = needsConfirmation)]
    pub fn set_needs_confirmation(this: &WorkspaceEditMetadata, val: bool);
    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "label", js_namespace = languages, getter = label)]
    pub fn label(this: &WorkspaceEditMetadata) -> String;
    /// Set the `label` property.
    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "label", js_namespace = languages, setter = label)]
    pub fn set_label(this: &WorkspaceEditMetadata, val: &str);
    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "description", js_namespace = languages, getter = description)]
    pub fn description(this: &WorkspaceEditMetadata) -> Option<String>;
    /// Set the `description` property.
    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "description", js_namespace = languages, setter = description)]
    pub fn set_description(this: &WorkspaceEditMetadata, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceFileEditOptions;
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "overwrite", js_namespace = languages, getter = overwrite)]
    pub fn overwrite(this: &WorkspaceFileEditOptions) -> Option<bool>;
    /// Set the `overwrite` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "overwrite", js_namespace = languages, setter = overwrite)]
    pub fn set_overwrite(this: &WorkspaceFileEditOptions, val: Option<bool>);
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "ignoreIfNotExists", js_namespace = languages, getter = ignoreIfNotExists)]
    pub fn ignore_if_not_exists(this: &WorkspaceFileEditOptions) -> Option<bool>;
    /// Set the `ignoreIfNotExists` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "ignoreIfNotExists", js_namespace = languages, setter = ignoreIfNotExists)]
    pub fn set_ignore_if_not_exists(this: &WorkspaceFileEditOptions, val: Option<bool>);
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "ignoreIfExists", js_namespace = languages, getter = ignoreIfExists)]
    pub fn ignore_if_exists(this: &WorkspaceFileEditOptions) -> Option<bool>;
    /// Set the `ignoreIfExists` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "ignoreIfExists", js_namespace = languages, setter = ignoreIfExists)]
    pub fn set_ignore_if_exists(this: &WorkspaceFileEditOptions, val: Option<bool>);
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "recursive", js_namespace = languages, getter = recursive)]
    pub fn recursive(this: &WorkspaceFileEditOptions) -> Option<bool>;
    /// Set the `recursive` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "recursive", js_namespace = languages, setter = recursive)]
    pub fn set_recursive(this: &WorkspaceFileEditOptions, val: Option<bool>);
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "copy", js_namespace = languages, getter = copy)]
    pub fn copy(this: &WorkspaceFileEditOptions) -> Option<bool>;
    /// Set the `copy` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "copy", js_namespace = languages, setter = copy)]
    pub fn set_copy(this: &WorkspaceFileEditOptions, val: Option<bool>);
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "folder", js_namespace = languages, getter = folder)]
    pub fn folder(this: &WorkspaceFileEditOptions) -> Option<bool>;
    /// Set the `folder` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "folder", js_namespace = languages, setter = folder)]
    pub fn set_folder(this: &WorkspaceFileEditOptions, val: Option<bool>);
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "skipTrashBin", js_namespace = languages, getter = skipTrashBin)]
    pub fn skip_trash_bin(this: &WorkspaceFileEditOptions) -> Option<bool>;
    /// Set the `skipTrashBin` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "skipTrashBin", js_namespace = languages, setter = skipTrashBin)]
    pub fn set_skip_trash_bin(this: &WorkspaceFileEditOptions, val: Option<bool>);
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "maxSize", js_namespace = languages, getter = maxSize)]
    pub fn max_size(this: &WorkspaceFileEditOptions) -> Option<f64>;
    /// Set the `maxSize` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "maxSize", js_namespace = languages, setter = maxSize)]
    pub fn set_max_size(this: &WorkspaceFileEditOptions, val: Option<f64>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceFileEdit;
    #[wasm_bindgen(method, js_class = "WorkspaceFileEdit", js_name = "oldUri", js_namespace = languages, getter = oldUri)]
    pub fn old_uri(this: &WorkspaceFileEdit) -> Option<Uri>;
    /// Set the `oldUri` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEdit", js_name = "oldUri", js_namespace = languages, setter = oldUri)]
    pub fn set_old_uri(this: &WorkspaceFileEdit, val: Option<&Uri>);
    #[wasm_bindgen(method, js_class = "WorkspaceFileEdit", js_name = "newUri", js_namespace = languages, getter = newUri)]
    pub fn new_uri(this: &WorkspaceFileEdit) -> Option<Uri>;
    /// Set the `newUri` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEdit", js_name = "newUri", js_namespace = languages, setter = newUri)]
    pub fn set_new_uri(this: &WorkspaceFileEdit, val: Option<&Uri>);
    #[wasm_bindgen(method, js_class = "WorkspaceFileEdit", js_name = "options", js_namespace = languages, getter = options)]
    pub fn options(this: &WorkspaceFileEdit) -> Option<WorkspaceFileEditOptions>;
    /// Set the `options` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEdit", js_name = "options", js_namespace = languages, setter = options)]
    pub fn set_options(this: &WorkspaceFileEdit, val: Option<&WorkspaceFileEditOptions>);
    #[wasm_bindgen(method, js_class = "WorkspaceFileEdit", js_name = "metadata", js_namespace = languages, getter = metadata)]
    pub fn metadata(this: &WorkspaceFileEdit) -> Option<WorkspaceEditMetadata>;
    /// Set the `metadata` property.
    #[wasm_bindgen(method, js_class = "WorkspaceFileEdit", js_name = "metadata", js_namespace = languages, setter = metadata)]
    pub fn set_metadata(this: &WorkspaceFileEdit, val: Option<&WorkspaceEditMetadata>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceTextEdit;
    #[wasm_bindgen(method, js_class = "WorkspaceTextEdit", js_name = "resource", js_namespace = languages, getter = resource)]
    pub fn resource(this: &WorkspaceTextEdit) -> Uri;
    /// Set the `resource` property.
    #[wasm_bindgen(method, js_class = "WorkspaceTextEdit", js_name = "resource", js_namespace = languages, setter = resource)]
    pub fn set_resource(this: &WorkspaceTextEdit, val: &Uri);
    #[wasm_bindgen(method, js_class = "WorkspaceTextEdit", js_name = "edit", js_namespace = languages, getter = edit)]
    pub fn edit(this: &WorkspaceTextEdit) -> JsValue;
    /// Set the `edit` property.
    #[wasm_bindgen(method, js_class = "WorkspaceTextEdit", js_name = "edit", js_namespace = languages, setter = edit)]
    pub fn set_edit(this: &WorkspaceTextEdit, val: &JsValue);
    #[wasm_bindgen(method, js_class = "WorkspaceTextEdit", js_name = "modelVersionId", js_namespace = languages, getter = modelVersionId)]
    pub fn model_version_id(this: &WorkspaceTextEdit) -> Option<f64>;
    /// Set the `modelVersionId` property.
    #[wasm_bindgen(method, js_class = "WorkspaceTextEdit", js_name = "modelVersionId", js_namespace = languages, setter = modelVersionId)]
    pub fn set_model_version_id(this: &WorkspaceTextEdit, val: Option<f64>);
    #[wasm_bindgen(method, js_class = "WorkspaceTextEdit", js_name = "metadata", js_namespace = languages, getter = metadata)]
    pub fn metadata(this: &WorkspaceTextEdit) -> Option<WorkspaceEditMetadata>;
    /// Set the `metadata` property.
    #[wasm_bindgen(method, js_class = "WorkspaceTextEdit", js_name = "metadata", js_namespace = languages, setter = metadata)]
    pub fn set_metadata(this: &WorkspaceTextEdit, val: Option<&WorkspaceEditMetadata>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceEdit;
    #[wasm_bindgen(method, js_class = "WorkspaceEdit", js_name = "edits", js_namespace = languages, getter = edits)]
    pub fn edits(this: &WorkspaceEdit) -> Array;
    /// Set the `edits` property.
    #[wasm_bindgen(method, js_class = "WorkspaceEdit", js_name = "edits", js_namespace = languages, setter = edits)]
    pub fn set_edits(this: &WorkspaceEdit, val: &Array);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type Rejection;
    #[wasm_bindgen(method, js_class = "Rejection", js_name = "rejectReason", js_namespace = languages, getter = rejectReason)]
    pub fn reject_reason(this: &Rejection) -> Option<String>;
    /// Set the `rejectReason` property.
    #[wasm_bindgen(method, js_class = "Rejection", js_name = "rejectReason", js_namespace = languages, setter = rejectReason)]
    pub fn set_reject_reason(this: &Rejection, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type RenameLocation;
    #[wasm_bindgen(method, js_class = "RenameLocation", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &RenameLocation) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "RenameLocation", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &RenameLocation, val: &IRange);
    #[wasm_bindgen(method, js_class = "RenameLocation", js_name = "text", js_namespace = languages, getter = text)]
    pub fn text(this: &RenameLocation) -> String;
    /// Set the `text` property.
    #[wasm_bindgen(method, js_class = "RenameLocation", js_name = "text", js_namespace = languages, setter = text)]
    pub fn set_text(this: &RenameLocation, val: &str);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type RenameProvider;
    #[wasm_bindgen(method, js_class = "RenameProvider", js_name = "provideRenameEdits", js_namespace = languages)]
    pub fn provide_rename_edits(
        this: &RenameProvider,
        model: &ITextModel,
        position: &Position,
        new_name: &str,
        token: &CancellationToken,
    ) -> JsValue;
    /// Type: `((model: editor.ITextModel, position: Position, token:
    /// CancellationToken) => JsValue)`
    #[wasm_bindgen(method, js_class = "RenameProvider", js_name = "resolveRenameLocation", js_namespace = languages, getter = resolveRenameLocation)]
    pub fn resolve_rename_location(this: &RenameProvider) -> Option<Function>;
    /// Set the `resolveRenameLocation` property.
    #[wasm_bindgen(method, js_class = "RenameProvider", js_name = "resolveRenameLocation", js_namespace = languages, setter = resolveRenameLocation)]
    pub fn set_resolve_rename_location(this: &RenameProvider, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type Command;
    #[wasm_bindgen(method, js_class = "Command", js_name = "id", js_namespace = languages, getter = id)]
    pub fn id(this: &Command) -> String;
    /// Set the `id` property.
    #[wasm_bindgen(method, js_class = "Command", js_name = "id", js_namespace = languages, setter = id)]
    pub fn set_id(this: &Command, val: &str);
    #[wasm_bindgen(method, js_class = "Command", js_name = "title", js_namespace = languages, getter = title)]
    pub fn title(this: &Command) -> String;
    /// Set the `title` property.
    #[wasm_bindgen(method, js_class = "Command", js_name = "title", js_namespace = languages, setter = title)]
    pub fn set_title(this: &Command, val: &str);
    #[wasm_bindgen(method, js_class = "Command", js_name = "tooltip", js_namespace = languages, getter = tooltip)]
    pub fn tooltip(this: &Command) -> Option<String>;
    /// Set the `tooltip` property.
    #[wasm_bindgen(method, js_class = "Command", js_name = "tooltip", js_namespace = languages, setter = tooltip)]
    pub fn set_tooltip(this: &Command, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "Command", js_name = "arguments", js_namespace = languages, getter = arguments)]
    pub fn arguments(this: &Command) -> Option<Array>;
    /// Set the `arguments` property.
    #[wasm_bindgen(method, js_class = "Command", js_name = "arguments", js_namespace = languages, setter = arguments)]
    pub fn set_arguments(this: &Command, val: &Array);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CodeLens;
    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "range", js_namespace = languages, getter = range)]
    pub fn range(this: &CodeLens) -> IRange;
    /// Set the `range` property.
    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "range", js_namespace = languages, setter = range)]
    pub fn set_range(this: &CodeLens, val: &IRange);
    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "id", js_namespace = languages, getter = id)]
    pub fn id(this: &CodeLens) -> Option<String>;
    /// Set the `id` property.
    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "id", js_namespace = languages, setter = id)]
    pub fn set_id(this: &CodeLens, val: Option<&str>);
    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "command", js_namespace = languages, getter = command)]
    pub fn command(this: &CodeLens) -> Option<Command>;
    /// Set the `command` property.
    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "command", js_namespace = languages, setter = command)]
    pub fn set_command(this: &CodeLens, val: Option<&Command>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CodeLensList;
    /// Type: `CodeLens[]`
    #[wasm_bindgen(method, js_class = "CodeLensList", js_name = "lenses", js_namespace = languages, getter = lenses)]
    pub fn lenses(this: &CodeLensList) -> Array;
    /// Set the `lenses` property.
    #[wasm_bindgen(method, js_class = "CodeLensList", js_name = "lenses", js_namespace = languages, setter = lenses)]
    pub fn set_lenses(this: &CodeLensList, val: &Array);
    #[wasm_bindgen(method, js_class = "CodeLensList", js_name = "dispose", js_namespace = languages)]
    pub fn dispose(this: &CodeLensList);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CodeLensProvider;
    #[wasm_bindgen(method, js_class = "CodeLensProvider", js_name = "onDidChange", js_namespace = languages, getter = onDidChange)]
    pub fn on_did_change(this: &CodeLensProvider) -> Option<Function>;
    /// Set the `onDidChange` property.
    #[wasm_bindgen(method, js_class = "CodeLensProvider", js_name = "onDidChange", js_namespace = languages, setter = onDidChange)]
    pub fn set_on_did_change(this: &CodeLensProvider, val: Option<&Function>);
    #[wasm_bindgen(method, js_class = "CodeLensProvider", js_name = "provideCodeLenses", js_namespace = languages)]
    pub fn provide_code_lenses(
        this: &CodeLensProvider,
        model: &ITextModel,
        token: &CancellationToken,
    ) -> JsValue;
    /// Type: `((model: editor.ITextModel, codeLens: CodeLens, token:
    /// CancellationToken) => JsValue)`
    #[wasm_bindgen(method, js_class = "CodeLensProvider", js_name = "resolveCodeLens", js_namespace = languages, getter = resolveCodeLens)]
    pub fn resolve_code_lens(this: &CodeLensProvider) -> Option<Function>;
    /// Set the `resolveCodeLens` property.
    #[wasm_bindgen(method, js_class = "CodeLensProvider", js_name = "resolveCodeLens", js_namespace = languages, setter = resolveCodeLens)]
    pub fn set_resolve_code_lens(this: &CodeLensProvider, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type InlayHintLabelPart;
    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "label", js_namespace = languages, getter = label)]
    pub fn label(this: &InlayHintLabelPart) -> String;
    /// Set the `label` property.
    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "label", js_namespace = languages, setter = label)]
    pub fn set_label(this: &InlayHintLabelPart, val: &str);
    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "tooltip", js_namespace = languages, getter = tooltip)]
    pub fn tooltip(this: &InlayHintLabelPart) -> JsValue;
    /// Set the `tooltip` property.
    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "tooltip", js_namespace = languages, setter = tooltip)]
    pub fn set_tooltip(this: &InlayHintLabelPart, val: &JsValue);
    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "command", js_namespace = languages, getter = command)]
    pub fn command(this: &InlayHintLabelPart) -> Option<Command>;
    /// Set the `command` property.
    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "command", js_namespace = languages, setter = command)]
    pub fn set_command(this: &InlayHintLabelPart, val: Option<&Command>);
    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "location", js_namespace = languages, getter = location)]
    pub fn location(this: &InlayHintLabelPart) -> Option<Location>;
    /// Set the `location` property.
    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "location", js_namespace = languages, setter = location)]
    pub fn set_location(this: &InlayHintLabelPart, val: Option<&Location>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type InlayHint;
    /// Type: `string | InlayHintLabelPart[]`
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "label", js_namespace = languages, getter = label)]
    pub fn label(this: &InlayHint) -> JsValue;
    /// Set the `label` property.
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "label", js_namespace = languages, setter = label)]
    pub fn set_label(this: &InlayHint, val: &JsValue);
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "tooltip", js_namespace = languages, getter = tooltip)]
    pub fn tooltip(this: &InlayHint) -> JsValue;
    /// Set the `tooltip` property.
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "tooltip", js_namespace = languages, setter = tooltip)]
    pub fn set_tooltip(this: &InlayHint, val: &JsValue);
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "position", js_namespace = languages, getter = position)]
    pub fn position(this: &InlayHint) -> IPosition;
    /// Set the `position` property.
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "position", js_namespace = languages, setter = position)]
    pub fn set_position(this: &InlayHint, val: &IPosition);
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "kind", js_namespace = languages, getter = kind)]
    pub fn kind(this: &InlayHint) -> InlayHintKind;
    /// Set the `kind` property.
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "kind", js_namespace = languages, setter = kind)]
    pub fn set_kind(this: &InlayHint, val: InlayHintKind);
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "paddingLeft", js_namespace = languages, getter = paddingLeft)]
    pub fn padding_left(this: &InlayHint) -> Option<bool>;
    /// Set the `paddingLeft` property.
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "paddingLeft", js_namespace = languages, setter = paddingLeft)]
    pub fn set_padding_left(this: &InlayHint, val: Option<bool>);
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "paddingRight", js_namespace = languages, getter = paddingRight)]
    pub fn padding_right(this: &InlayHint) -> Option<bool>;
    /// Set the `paddingRight` property.
    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "paddingRight", js_namespace = languages, setter = paddingRight)]
    pub fn set_padding_right(this: &InlayHint, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type InlayHintList;
    /// Type: `InlayHint[]`
    #[wasm_bindgen(method, js_class = "InlayHintList", js_name = "hints", js_namespace = languages, getter = hints)]
    pub fn hints(this: &InlayHintList) -> Array;
    /// Set the `hints` property.
    #[wasm_bindgen(method, js_class = "InlayHintList", js_name = "hints", js_namespace = languages, setter = hints)]
    pub fn set_hints(this: &InlayHintList, val: &Array);
    #[wasm_bindgen(method, js_class = "InlayHintList", js_name = "dispose", js_namespace = languages)]
    pub fn dispose(this: &InlayHintList);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type InlayHintsProvider;
    #[wasm_bindgen(method, js_class = "InlayHintsProvider", js_name = "onDidChangeInlayHints", js_namespace = languages, getter = onDidChangeInlayHints)]
    pub fn on_did_change_inlay_hints(this: &InlayHintsProvider) -> Option<Function>;
    /// Set the `onDidChangeInlayHints` property.
    #[wasm_bindgen(method, js_class = "InlayHintsProvider", js_name = "onDidChangeInlayHints", js_namespace = languages, setter = onDidChangeInlayHints)]
    pub fn set_on_did_change_inlay_hints(this: &InlayHintsProvider, val: Option<&Function>);
    #[wasm_bindgen(method, js_class = "InlayHintsProvider", js_name = "provideInlayHints", js_namespace = languages)]
    pub fn provide_inlay_hints(
        this: &InlayHintsProvider,
        model: &ITextModel,
        range: &Range,
        token: &CancellationToken,
    ) -> JsValue;
    /// Type: `((hint: InlayHint, token: CancellationToken) => JsValue)`
    #[wasm_bindgen(method, js_class = "InlayHintsProvider", js_name = "resolveInlayHintL", js_namespace = languages, getter = resolveInlayHintL)]
    pub fn resolve_inlay_hint_l(this: &InlayHintsProvider) -> Option<Function>;
    /// Set the `resolveInlayHintL` property.
    #[wasm_bindgen(method, js_class = "InlayHintsProvider", js_name = "resolveInlayHintL", js_namespace = languages, setter = resolveInlayHintL)]
    pub fn set_resolve_inlay_hint_l(this: &InlayHintsProvider, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type SemanticTokensLegend;
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "SemanticTokensLegend", js_name = "tokenTypes", js_namespace = languages, getter = tokenTypes)]
    pub fn token_types(this: &SemanticTokensLegend) -> Array;
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "SemanticTokensLegend", js_name = "tokenModifiers", js_namespace = languages, getter = tokenModifiers)]
    pub fn token_modifiers(this: &SemanticTokensLegend) -> Array;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type SemanticTokens;
    #[wasm_bindgen(method, js_class = "SemanticTokens", js_name = "resultId", js_namespace = languages, getter = resultId)]
    pub fn result_id(this: &SemanticTokens) -> Option<String>;
    #[wasm_bindgen(method, js_class = "SemanticTokens", js_name = "data", js_namespace = languages, getter = data)]
    pub fn data(this: &SemanticTokens) -> Uint32Array;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type SemanticTokensEdit;
    #[wasm_bindgen(method, js_class = "SemanticTokensEdit", js_name = "start", js_namespace = languages, getter = start)]
    pub fn start(this: &SemanticTokensEdit) -> f64;
    #[wasm_bindgen(method, js_class = "SemanticTokensEdit", js_name = "deleteCount", js_namespace = languages, getter = deleteCount)]
    pub fn delete_count(this: &SemanticTokensEdit) -> f64;
    #[wasm_bindgen(method, js_class = "SemanticTokensEdit", js_name = "data", js_namespace = languages, getter = data)]
    pub fn data(this: &SemanticTokensEdit) -> Option<Uint32Array>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type SemanticTokensEdits;
    #[wasm_bindgen(method, js_class = "SemanticTokensEdits", js_name = "resultId", js_namespace = languages, getter = resultId)]
    pub fn result_id(this: &SemanticTokensEdits) -> Option<String>;
    /// Type: `SemanticTokensEdit[]`
    #[wasm_bindgen(method, js_class = "SemanticTokensEdits", js_name = "edits", js_namespace = languages, getter = edits)]
    pub fn edits(this: &SemanticTokensEdits) -> Array;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DocumentSemanticTokensProvider;
    #[wasm_bindgen(method, js_class = "DocumentSemanticTokensProvider", js_name = "onDidChange", js_namespace = languages, getter = onDidChange)]
    pub fn on_did_change(this: &DocumentSemanticTokensProvider) -> Option<Function>;
    /// Set the `onDidChange` property.
    #[wasm_bindgen(method, js_class = "DocumentSemanticTokensProvider", js_name = "onDidChange", js_namespace = languages, setter = onDidChange)]
    pub fn set_on_did_change(this: &DocumentSemanticTokensProvider, val: Option<&Function>);
    #[wasm_bindgen(
        method,
        js_class = "DocumentSemanticTokensProvider",
        js_name = "getLegend", js_namespace = languages
    )]
    pub fn get_legend(this: &DocumentSemanticTokensProvider) -> SemanticTokensLegend;
    #[wasm_bindgen(
        method,
        js_class = "DocumentSemanticTokensProvider",
        js_name = "provideDocumentSemanticTokens", js_namespace = languages
    )]
    pub fn provide_document_semantic_tokens(
        this: &DocumentSemanticTokensProvider,
        model: &ITextModel,
        last_result_id: Option<&str>,
        token: &CancellationToken,
    ) -> JsValue;
    #[wasm_bindgen(
        method,
        js_class = "DocumentSemanticTokensProvider",
        js_name = "releaseDocumentSemanticTokens", js_namespace = languages
    )]
    pub fn release_document_semantic_tokens(
        this: &DocumentSemanticTokensProvider,
        result_id: Option<&str>,
    );
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DocumentRangeSemanticTokensProvider;
    #[wasm_bindgen(
        method,
        js_class = "DocumentRangeSemanticTokensProvider",
        js_name = "getLegend", js_namespace = languages
    )]
    pub fn get_legend(this: &DocumentRangeSemanticTokensProvider) -> SemanticTokensLegend;
    #[wasm_bindgen(
        method,
        js_class = "DocumentRangeSemanticTokensProvider",
        js_name = "provideDocumentRangeSemanticTokens", js_namespace = languages
    )]
    pub fn provide_document_range_semantic_tokens(
        this: &DocumentRangeSemanticTokensProvider,
        model: &ITextModel,
        range: &Range,
        token: &CancellationToken,
    ) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ILanguageExtensionPoint;
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "id", js_namespace = languages, getter = id)]
    pub fn id(this: &ILanguageExtensionPoint) -> String;
    /// Set the `id` property.
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "id", js_namespace = languages, setter = id)]
    pub fn set_id(this: &ILanguageExtensionPoint, val: &str);
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "extensions", js_namespace = languages, getter = extensions)]
    pub fn extensions(this: &ILanguageExtensionPoint) -> Option<Array>;
    /// Set the `extensions` property.
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "extensions", js_namespace = languages, setter = extensions)]
    pub fn set_extensions(this: &ILanguageExtensionPoint, val: Option<&Array>);
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "filenames", js_namespace = languages, getter = filenames)]
    pub fn filenames(this: &ILanguageExtensionPoint) -> Option<Array>;
    /// Set the `filenames` property.
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "filenames", js_namespace = languages, setter = filenames)]
    pub fn set_filenames(this: &ILanguageExtensionPoint, val: Option<&Array>);
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "filenamePatterns", js_namespace = languages, getter = filenamePatterns)]
    pub fn filename_patterns(this: &ILanguageExtensionPoint) -> Option<Array>;
    /// Set the `filenamePatterns` property.
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "filenamePatterns", js_namespace = languages, setter = filenamePatterns)]
    pub fn set_filename_patterns(this: &ILanguageExtensionPoint, val: Option<&Array>);
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "firstLine", js_namespace = languages, getter = firstLine)]
    pub fn first_line(this: &ILanguageExtensionPoint) -> Option<String>;
    /// Set the `firstLine` property.
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "firstLine", js_namespace = languages, setter = firstLine)]
    pub fn set_first_line(this: &ILanguageExtensionPoint, val: Option<&str>);
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "aliases", js_namespace = languages, getter = aliases)]
    pub fn aliases(this: &ILanguageExtensionPoint) -> Option<Array>;
    /// Set the `aliases` property.
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "aliases", js_namespace = languages, setter = aliases)]
    pub fn set_aliases(this: &ILanguageExtensionPoint, val: Option<&Array>);
    /// Type: `string[]`
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "mimetypes", js_namespace = languages, getter = mimetypes)]
    pub fn mimetypes(this: &ILanguageExtensionPoint) -> Option<Array>;
    /// Set the `mimetypes` property.
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "mimetypes", js_namespace = languages, setter = mimetypes)]
    pub fn set_mimetypes(this: &ILanguageExtensionPoint, val: Option<&Array>);
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "configuration", js_namespace = languages, getter = configuration)]
    pub fn configuration(this: &ILanguageExtensionPoint) -> Option<Uri>;
    /// Set the `configuration` property.
    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "configuration", js_namespace = languages, setter = configuration)]
    pub fn set_configuration(this: &ILanguageExtensionPoint, val: Option<&Uri>);
}

#[wasm_bindgen]
extern "C" {
    /// A Monarch language definition
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IMonarchLanguage;
    /// map from string to ILanguageRule[]
    ///
    /// Type: `{}`
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "tokenizer", js_namespace = languages, getter = tokenizer)]
    pub fn tokenizer(this: &IMonarchLanguage) -> Object;
    /// Set the `tokenizer` property.
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "tokenizer", js_namespace = languages, setter = tokenizer)]
    pub fn set_tokenizer(this: &IMonarchLanguage, val: &Object);
    /// is the language case insensitive?
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "ignoreCase", js_namespace = languages, getter = ignoreCase)]
    pub fn ignore_case(this: &IMonarchLanguage) -> Option<bool>;
    /// Set the `ignoreCase` property.
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "ignoreCase", js_namespace = languages, setter = ignoreCase)]
    pub fn set_ignore_case(this: &IMonarchLanguage, val: Option<bool>);
    /// is the language unicode-aware? (i.e., /\u{1D306}/)
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "unicode", js_namespace = languages, getter = unicode)]
    pub fn unicode(this: &IMonarchLanguage) -> Option<bool>;
    /// Set the `unicode` property.
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "unicode", js_namespace = languages, setter = unicode)]
    pub fn set_unicode(this: &IMonarchLanguage, val: Option<bool>);
    /// if no match in the tokenizer assign this token class (default 'source')
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "defaultToken", js_namespace = languages, getter = defaultToken)]
    pub fn default_token(this: &IMonarchLanguage) -> Option<String>;
    /// Set the `defaultToken` property.
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "defaultToken", js_namespace = languages, setter = defaultToken)]
    pub fn set_default_token(this: &IMonarchLanguage, val: Option<&str>);
    /// for example [['{','}','delimiter.curly']]
    ///
    /// Type: `IMonarchLanguageBracket[]`
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "brackets", js_namespace = languages, getter = brackets)]
    pub fn brackets(this: &IMonarchLanguage) -> Option<Array>;
    /// Set the `brackets` property.
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "brackets", js_namespace = languages, setter = brackets)]
    pub fn set_brackets(this: &IMonarchLanguage, val: Option<&Array>);
    /// start symbol in the tokenizer (by default the first entry is used)
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "start", js_namespace = languages, getter = start)]
    pub fn start(this: &IMonarchLanguage) -> Option<String>;
    /// Set the `start` property.
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "start", js_namespace = languages, setter = start)]
    pub fn set_start(this: &IMonarchLanguage, val: Option<&str>);
    /// attach this to every token class (by default '.' + name)
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "tokenPostfix", js_namespace = languages, getter = tokenPostfix)]
    pub fn token_postfix(this: &IMonarchLanguage) -> Option<String>;
    /// Set the `tokenPostfix` property.
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "tokenPostfix", js_namespace = languages, setter = tokenPostfix)]
    pub fn set_token_postfix(this: &IMonarchLanguage, val: Option<&str>);
    /// include line feeds (in the form of a \n character) at the end of lines
    /// Defaults to false
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "includeLF", js_namespace = languages, getter = includeLF)]
    pub fn include_lf(this: &IMonarchLanguage) -> Option<bool>;
    /// Set the `includeLF` property.
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "includeLF", js_namespace = languages, setter = includeLF)]
    pub fn set_include_lf(this: &IMonarchLanguage, val: Option<bool>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IExpandedMonarchLanguageRule;
    /// match tokens
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "regex", js_namespace = languages, getter = regex)]
    pub fn regex(this: &IExpandedMonarchLanguageRule) -> JsValue;
    /// Set the `regex` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "regex", js_namespace = languages, setter = regex)]
    pub fn set_regex(this: &IExpandedMonarchLanguageRule, val: &JsValue);
    /// action to take on match
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "action", js_namespace = languages, getter = action)]
    pub fn action(this: &IExpandedMonarchLanguageRule) -> JsValue;
    /// Set the `action` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "action", js_namespace = languages, setter = action)]
    pub fn set_action(this: &IExpandedMonarchLanguageRule, val: &JsValue);
    /// or an include rule. include all rules from the included state
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "include", js_namespace = languages, getter = include)]
    pub fn include(this: &IExpandedMonarchLanguageRule) -> Option<String>;
    /// Set the `include` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "include", js_namespace = languages, setter = include)]
    pub fn set_include(this: &IExpandedMonarchLanguageRule, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IExpandedMonarchLanguageAction;
    /// array of actions for each parenthesized match group
    ///
    /// Type: `IMonarchLanguageAction[]`
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "group", js_namespace = languages, getter = group)]
    pub fn group(this: &IExpandedMonarchLanguageAction) -> Option<Array>;
    /// Set the `group` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "group", js_namespace = languages, setter = group)]
    pub fn set_group(this: &IExpandedMonarchLanguageAction, val: Option<&Array>);
    /// map from string to ILanguageAction
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "cases", js_namespace = languages, getter = cases)]
    pub fn cases(this: &IExpandedMonarchLanguageAction) -> Option<Object>;
    /// Set the `cases` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "cases", js_namespace = languages, setter = cases)]
    pub fn set_cases(this: &IExpandedMonarchLanguageAction, val: Option<&Object>);
    /// token class (ie. css class) (or "@brackets" or "@rematch")
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "token", js_namespace = languages, getter = token)]
    pub fn token(this: &IExpandedMonarchLanguageAction) -> Option<String>;
    /// Set the `token` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "token", js_namespace = languages, setter = token)]
    pub fn set_token(this: &IExpandedMonarchLanguageAction, val: Option<&str>);
    /// the next state to push, or "@push", "@pop", "@popall"
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "next", js_namespace = languages, getter = next)]
    pub fn next(this: &IExpandedMonarchLanguageAction) -> Option<String>;
    /// Set the `next` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "next", js_namespace = languages, setter = next)]
    pub fn set_next(this: &IExpandedMonarchLanguageAction, val: Option<&str>);
    /// switch to this state
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "switchTo", js_namespace = languages, getter = switchTo)]
    pub fn switch_to(this: &IExpandedMonarchLanguageAction) -> Option<String>;
    /// Set the `switchTo` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "switchTo", js_namespace = languages, setter = switchTo)]
    pub fn set_switch_to(this: &IExpandedMonarchLanguageAction, val: Option<&str>);
    /// go back n characters in the stream
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "goBack", js_namespace = languages, getter = goBack)]
    pub fn go_back(this: &IExpandedMonarchLanguageAction) -> Option<f64>;
    /// Set the `goBack` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "goBack", js_namespace = languages, setter = goBack)]
    pub fn set_go_back(this: &IExpandedMonarchLanguageAction, val: Option<f64>);
    /// @open or @close
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "bracket", js_namespace = languages, getter = bracket)]
    pub fn bracket(this: &IExpandedMonarchLanguageAction) -> Option<String>;
    /// Set the `bracket` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "bracket", js_namespace = languages, setter = bracket)]
    pub fn set_bracket(this: &IExpandedMonarchLanguageAction, val: Option<&str>);
    /// switch to embedded language (using the mimetype) or get out using "@pop"
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "nextEmbedded", js_namespace = languages, getter = nextEmbedded)]
    pub fn next_embedded(this: &IExpandedMonarchLanguageAction) -> Option<String>;
    /// Set the `nextEmbedded` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "nextEmbedded", js_namespace = languages, setter = nextEmbedded)]
    pub fn set_next_embedded(this: &IExpandedMonarchLanguageAction, val: Option<&str>);
    /// log a message to the browser console window
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "log", js_namespace = languages, getter = log)]
    pub fn log(this: &IExpandedMonarchLanguageAction) -> Option<String>;
    /// Set the `log` property.
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "log", js_namespace = languages, setter = log)]
    pub fn set_log(this: &IExpandedMonarchLanguageAction, val: Option<&str>);
}

#[wasm_bindgen]
extern "C" {
    /// This interface can be shortened as an array, ie.
    /// ['{','}','delimiter.curly']
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IMonarchLanguageBracket;
    /// open bracket
    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "open", js_namespace = languages, getter = open)]
    pub fn open(this: &IMonarchLanguageBracket) -> String;
    /// Set the `open` property.
    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "open", js_namespace = languages, setter = open)]
    pub fn set_open(this: &IMonarchLanguageBracket, val: &str);
    /// closing bracket
    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "close", js_namespace = languages, getter = close)]
    pub fn close(this: &IMonarchLanguageBracket) -> String;
    /// Set the `close` property.
    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "close", js_namespace = languages, setter = close)]
    pub fn set_close(this: &IMonarchLanguageBracket, val: &str);
    /// token class
    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "token", js_namespace = languages, getter = token)]
    pub fn token(this: &IMonarchLanguageBracket) -> String;
    /// Set the `token` property.
    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "token", js_namespace = languages, setter = token)]
    pub fn set_token(this: &IMonarchLanguageBracket, val: &str);
}
