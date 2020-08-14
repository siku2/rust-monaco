/// Raw bindings for the monaco editor API.
pub use environment::*;
use js_sys::Object;
use wasm_bindgen::prelude::*;

pub mod editor;
mod environment;

#[wasm_bindgen(module = "/js/editor.js")]
extern "C" {
    /// The `editor` namespace.
    /// [API docs](https://microsoft.github.io/monaco-editor/api/modules/monaco.editor.html)
    pub static editor: editor::Editor;

    /// [API docs](https://microsoft.github.io/monaco-editor/api/classes/monaco.cancellationtokensource.html)
    #[derive(Debug)]
    pub type CancellationTokenSource;

    /// [API docs](https://microsoft.github.io/monaco-editor/api/classes/monaco.emitter.html)
    #[derive(Debug)]
    pub type Emitter;

    /// [API docs](https://microsoft.github.io/monaco-editor/api/classes/monaco.keymod.html)
    #[derive(Debug)]
    pub type KeyMod;

    /// [API docs](https://microsoft.github.io/monaco-editor/api/classes/monaco.position.html)
    #[derive(Debug)]
    pub type Position;

    /// [API docs](https://microsoft.github.io/monaco-editor/api/classes/monaco.range.html)
    #[derive(Debug)]
    pub type Range;

    /// [API docs](https://microsoft.github.io/monaco-editor/api/classes/monaco.selection.html)
    #[derive(Debug)]
    pub type Selection;

    /// [API docs](https://microsoft.github.io/monaco-editor/api/classes/monaco.token.html)
    #[derive(Debug)]
    pub type Token;

    /// Uniform Resource Identifier (Uri) http://tools.ietf.org/html/rfc3986.
    /// This class is a simple parser which creates the basic component parts (http://tools.ietf.org/html/rfc3986#section-3) with minimal validation and encoding.
    /// [API docs](https://microsoft.github.io/monaco-editor/api/classes/monaco.uri.html)
    #[derive(Debug)]
    pub type Uri;

    /// authority is the 'www.msft.com' part of 'http://www.msft.com/some/path?query#fragment'. The part between the first double slashes and the next slash.
    #[wasm_bindgen(method, getter)]
    pub fn authority(this: &Uri) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_authority(this: &Uri, value: &str) -> String;

    /// fragment is the 'fragment' part of 'http://www.msft.com/some/path?query#fragment'.
    #[wasm_bindgen(method, getter)]
    pub fn fragment(this: &Uri) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_fragment(this: &Uri, value: &str) -> String;

    /// path is the '/some/path' part of 'http://www.msft.com/some/path?query#fragment'.
    #[wasm_bindgen(method, getter)]
    pub fn path(this: &Uri) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_path(this: &Uri, value: &str) -> String;

    /// query is the 'query' part of 'http://www.msft.com/some/path?query#fragment'.
    #[wasm_bindgen(method, getter)]
    pub fn query(this: &Uri) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_query(this: &Uri, value: &str) -> String;

    /// scheme is the 'http' part of 'http://www.msft.com/some/path?query#fragment'. The part before the first colon.
    #[wasm_bindgen(method, getter)]
    pub fn scheme(this: &Uri) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_scheme(this: &Uri, value: &str) -> String;

    /// Returns a string representing the corresponding file system path of this
    /// Uri. Will handle UNC paths, normalizes windows drive letters to
    /// lower-case, and uses the platform specific path separator.
    ///
    /// Will not validate the path for invalid characters and semantics.
    /// Will not look at the scheme of this Uri.
    /// The result shall not be used for display purposes but for accessing a
    /// file on disk.
    ///
    /// The difference to Uri#path is the use of the platform specific separator
    /// and the handling of UNC paths. See the below sample of a file-uri with
    /// an authority (UNC path).
    ///
    /// Using Uri#path to read a file (using fs-apis) would not be enough
    /// because parts of the path, namely the server name, would be missing.
    /// Therefore Uri#fsPath exists - it's sugar to ease working with URIs that
    /// represent files on disk (file scheme).
    #[wasm_bindgen(method, getter, js_name = "fsPath")]
    pub fn fs_path(this: &Uri) -> String;

    // TODO: UriComponents
    /// Convert to a raw object.
    #[wasm_bindgen(method, js_name = "toJSON")]
    pub fn to_json(this: &Uri) -> JsValue;

    /// Creates a string representation for this Uri. It's guaranteed that
    /// calling Uri.parse with the result of this function creates an Uri which
    /// is equal to this Uri.
    ///
    ///   - The result shall not be used for display purposes but for
    ///     externalization or transport.
    ///   - The result will be encoded using the percentage encoding and
    ///     encoding happens mostly ignore the scheme-specific encoding rules.
    #[wasm_bindgen(method, js_name = "toString")]
    pub fn to_string(this: &Uri, skip_encoding: bool) -> String;

    /// Create from components
    #[wasm_bindgen(static_method_of = Uri)]
    pub fn from_(components: &Object) -> Uri;

// TODO: complete Uri implementation
}
