pub mod monaco {

#[allow(unused)]
use wasm_bindgen::prelude::*;
#[allow(unused)]
use web_sys::*;
#[allow(unused)]
use js_sys::*;
#[allow(unused)]
use super::*;

#[allow(unused)]
pub type HTMLElement = HtmlElement;
#[allow(unused)]
pub type ReadonlyArray = Array;
#[allow(unused)]
pub type NonNullable = JsValue;
#[allow(unused)]
pub type Record = JsValue;
#[allow(unused)]
pub type PromiseLike = JsValue;


pub type Thenable = PromiseLike
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Environment;

    /**
         * Define a global `monaco` symbol.
         * This is true by default in AMD and false by default in ESM.
         */ 
    #[wasm_bindgen(method, js_class = "Environment", js_name = "globalAPI", getter)]
    pub fn global_api(this: &Environment) -> bool;

    #[wasm_bindgen(method, js_class = "Environment", js_name = "globalAPI", setter)]
    pub fn set_global_api(this: &Environment, global_api: bool);

    /**
         * The base url where the editor sources are found (which contains the vs folder)
         */ 
    #[wasm_bindgen(method, js_class = "Environment", js_name = "baseUrl", getter)]
    pub fn base_url(this: &Environment) -> String;

    #[wasm_bindgen(method, js_class = "Environment", js_name = "baseUrl", setter)]
    pub fn set_base_url(this: &Environment, base_url: String);

    /**
         * A web worker factory.
         * NOTE: If `getWorker` is defined, `getWorkerUrl` is not invoked.
         */ 
    #[wasm_bindgen(method, js_class = "Environment", js_name = "getWorker")]
    pub fn get_worker(this: &Environment, worker_id: String, label: String, ) -> JsValue;

    /**
         * Return the location for web worker scripts.
         * NOTE: If `getWorker` is defined, `getWorkerUrl` is not invoked.
         */ 
    #[wasm_bindgen(method, js_class = "Environment", js_name = "getWorkerUrl")]
    pub fn get_worker_url(this: &Environment, worker_id: String, label: String, ) -> Option<String>;

    /**
         * Create a trusted types policy (same API as window.trustedTypes.createPolicy)
         */ 
    #[wasm_bindgen(method, js_class = "Environment", js_name = "createTrustedTypesPolicy")]
    pub fn create_trusted_types_policy(this: &Environment, policy_name: String, policy_options: Option<ITrustedTypePolicyOptions>, ) -> JsValue;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ITrustedTypePolicyOptions;

    #[wasm_bindgen(method, js_class = "ITrustedTypePolicyOptions", js_name = "createHTML", getter)]
    pub fn create_html(this: &ITrustedTypePolicyOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "ITrustedTypePolicyOptions", js_name = "createHTML", setter)]
    pub fn set_create_html(this: &ITrustedTypePolicyOptions, create_html: JsValue);

    #[wasm_bindgen(method, js_class = "ITrustedTypePolicyOptions", js_name = "createScript", getter)]
    pub fn create_script(this: &ITrustedTypePolicyOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "ITrustedTypePolicyOptions", js_name = "createScript", setter)]
    pub fn set_create_script(this: &ITrustedTypePolicyOptions, create_script: JsValue);

    #[wasm_bindgen(method, js_class = "ITrustedTypePolicyOptions", js_name = "createScriptURL", getter)]
    pub fn create_script_url(this: &ITrustedTypePolicyOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "ITrustedTypePolicyOptions", js_name = "createScriptURL", setter)]
    pub fn set_create_script_url(this: &ITrustedTypePolicyOptions, create_script_url: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ITrustedTypePolicy;

    #[wasm_bindgen(method, js_class = "ITrustedTypePolicy", js_name = "name", getter)]
    pub fn name(this: &ITrustedTypePolicy) -> String;
    #[wasm_bindgen(method, js_class = "ITrustedTypePolicy", js_name = "createHTML")]
    pub fn create_html(this: &ITrustedTypePolicy, input: String, ) -> JsValue;

    #[wasm_bindgen(method, js_class = "ITrustedTypePolicy", js_name = "createScript")]
    pub fn create_script(this: &ITrustedTypePolicy, input: String, ) -> JsValue;

    #[wasm_bindgen(method, js_class = "ITrustedTypePolicy", js_name = "createScriptURL")]
    pub fn create_script_url(this: &ITrustedTypePolicy, input: String, ) -> JsValue;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDisposable;

    #[wasm_bindgen(method, js_class = "IDisposable", js_name = "dispose")]
    pub fn dispose(this: &IDisposable, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEvent;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Emitter;

  #[wasm_bindgen(constructor, js_class = "Emitter")]
  pub fn new() -> Emitter;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn fire(this: &Emitter, event: JsValue, );


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn dispose(this: &Emitter, );


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum MarkerTag {
    Unnecessary = 1,
    Deprecated = 2,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum MarkerSeverity {
    Hint = 1,
    Info = 2,
    Warning = 4,
    Error = 8,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CancellationTokenSource;

  #[wasm_bindgen(constructor, js_class = "CancellationTokenSource")]
  pub fn new() -> CancellationTokenSource;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", getter = token, method)]
    pub fn token(this: &CancellationTokenSource, ) -> CancellationToken;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn cancel(this: &CancellationTokenSource, );


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn dispose(this: &CancellationTokenSource, cancel: Option<bool>, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CancellationToken;

    /**
         * A flag signalling is cancellation has been requested.
         */ 
    #[wasm_bindgen(method, js_class = "CancellationToken", js_name = "isCancellationRequested", getter)]
    pub fn is_cancellation_requested(this: &CancellationToken) -> bool;
    /**
         * An event which fires when cancellation is requested. This event
         * only ever fires `once` as cancellation can only happen once. Listeners
         * that are registered after cancellation will be called (next event loop run),
         * but also only once.
         *
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "CancellationToken", js_name = "onCancellationRequested", getter)]
    pub fn on_cancellation_requested(this: &CancellationToken) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Uri;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Uri)]
    pub fn is_uri(thing: JsValue, ) -> JsValue;

    /**
         * Returns a string representing the corresponding file system path of this Uri.
         * Will handle UNC paths, normalizes windows drive letters to lower-case, and uses the
         * platform specific path separator.
         *
         * * Will *not* validate the path for invalid characters and semantics.
         * * Will *not* look at the scheme of this Uri.
         * * The result shall *not* be used for display purposes but for accessing a file on disk.
         *
         *
         * The *difference* to `Uri#path` is the use of the platform specific separator and the handling
         * of UNC paths. See the below sample of a file-uri with an authority (UNC path).
         *
         * ```ts
            const u = Uri.parse('file://server/c$/folder/file.txt')
            u.authority === 'server'
            u.path === '/shares/c$/file.txt'
            u.fsPath === '\\server\c$\folder\file.txt'
        ```
         *
         * Using `Uri#path` to read a file (using fs-apis) would not be enough because parts of the path,
         * namely the server name, would be missing. Therefore `Uri#fsPath` exists - it's sugar to ease working
         * with URIs that represent files on disk (`file` scheme).
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", getter = fsPath, method)]
    pub fn fs_path(this: &Uri, ) -> String;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn with(this: &Uri, change: js_sys::Object, ) -> Uri;

    /**
         * Creates a new Uri from a string, e.g. `http://www.example.com/some/path`,
         * `file:///usr/home`, or `scheme:with/path`.
         *
         * @param value A string which represents an Uri (see `Uri#toString`).
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Uri)]
    pub fn parse(value: String, strict: Option<bool>, ) -> Uri;

    /**
         * Creates a new Uri from a file system path, e.g. `c:\my\files`,
         * `/usr/home`, or `\\server\share\some\path`.
         *
         * The *difference* between `Uri#parse` and `Uri#file` is that the latter treats the argument
         * as path, not as stringified-uri. E.g. `Uri.file(path)` is **not the same as**
         * `Uri.parse('file://' + path)` because the path might contain characters that are
         * interpreted (# and ?). See the following sample:
         * ```ts
        const good = Uri.file('/coding/c#/project1');
        good.scheme === 'file';
        good.path === '/coding/c#/project1';
        good.fragment === '';
        const bad = Uri.parse('file://' + '/coding/c#/project1');
        bad.scheme === 'file';
        bad.path === '/coding/c'; // path is now broken
        bad.fragment === '/project1';
        ```
         *
         * @param path A file system path (see `Uri#fsPath`)
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Uri)]
    pub fn file(path: String, ) -> Uri;

    /**
         * Creates new Uri from uri components.
         *
         * Unless `strict` is `true` the scheme is defaults to be `file`. This function performs
         * validation and should be used for untrusted uri components retrieved from storage,
         * user input, command arguments etc
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Uri)]
    pub fn from(components: UriComponents, strict: Option<bool>, ) -> Uri;

    /**
         * Join a Uri path with path fragments and normalizes the resulting path.
         *
         * @param uri The input Uri.
         * @param pathFragment The path fragment to add to the Uri path.
         * @returns The resulting Uri.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Uri)]
    pub fn join_path(uri: Uri, ) -> Uri;

    /**
         * Creates a string representation for this Uri. It's guaranteed that calling
         * `Uri.parse` with the result of this function creates an Uri which is equal
         * to this Uri.
         *
         * * The result shall *not* be used for display purposes but for externalization or transport.
         * * The result will be encoded using the percentage encoding and encoding happens mostly
         * ignore the scheme-specific encoding rules.
         *
         * @param skipEncoding Do not encode the result, default is `false`
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_string(this: &Uri, skip_encoding: Option<bool>, ) -> String;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_json(this: &Uri, ) -> UriComponents;

    /**
         * A helper function to revive URIs.
         *
         * **Note** that this function should only be used when receiving Uri#toJSON generated data
         * and that it doesn't do any validation. Use {@link Uri.from} when received "untrusted"
         * uri components such as command arguments or data from storage.
         *
         * @param data The Uri components or Uri to revive.
         * @returns The revived Uri or undefined or null.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Uri)]
    pub fn revive(data: JsValue, ) -> Uri;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Uri)]
    pub fn revive_1(data: JsValue, ) -> JsValue;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Uri)]
    pub fn revive_2(data: JsValue, ) -> JsValue;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Uri)]
    pub fn revive_3(data: JsValue, ) -> JsValue;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type UriComponents;

    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "scheme", getter)]
    pub fn scheme(this: &UriComponents) -> String;

    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "scheme", setter)]
    pub fn set_scheme(this: &UriComponents, scheme: String);

    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "authority", getter)]
    pub fn authority(this: &UriComponents) -> String;

    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "authority", setter)]
    pub fn set_authority(this: &UriComponents, authority: String);

    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "path", getter)]
    pub fn path(this: &UriComponents) -> String;

    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "path", setter)]
    pub fn set_path(this: &UriComponents, path: String);

    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "query", getter)]
    pub fn query(this: &UriComponents) -> String;

    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "query", setter)]
    pub fn set_query(this: &UriComponents, query: String);

    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "fragment", getter)]
    pub fn fragment(this: &UriComponents) -> String;

    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "fragment", setter)]
    pub fn set_fragment(this: &UriComponents, fragment: String);


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum KeyCode {
    DependsOnKbLayout = -1,
    /**
         * Placed first to cover the 0 value of the enum.
         */ 
    Unknown = 0,
    Backspace = 1,
    Tab = 2,
    Enter = 3,
    Shift = 4,
    Ctrl = 5,
    Alt = 6,
    PauseBreak = 7,
    CapsLock = 8,
    Escape = 9,
    Space = 10,
    PageUp = 11,
    PageDown = 12,
    End = 13,
    Home = 14,
    LeftArrow = 15,
    UpArrow = 16,
    RightArrow = 17,
    DownArrow = 18,
    Insert = 19,
    Delete = 20,
    Digit0 = 21,
    Digit1 = 22,
    Digit2 = 23,
    Digit3 = 24,
    Digit4 = 25,
    Digit5 = 26,
    Digit6 = 27,
    Digit7 = 28,
    Digit8 = 29,
    Digit9 = 30,
    KeyA = 31,
    KeyB = 32,
    KeyC = 33,
    KeyD = 34,
    KeyE = 35,
    KeyF = 36,
    KeyG = 37,
    KeyH = 38,
    KeyI = 39,
    KeyJ = 40,
    KeyK = 41,
    KeyL = 42,
    KeyM = 43,
    KeyN = 44,
    KeyO = 45,
    KeyP = 46,
    KeyQ = 47,
    KeyR = 48,
    KeyS = 49,
    KeyT = 50,
    KeyU = 51,
    KeyV = 52,
    KeyW = 53,
    KeyX = 54,
    KeyY = 55,
    KeyZ = 56,
    Meta = 57,
    ContextMenu = 58,
    F1 = 59,
    F2 = 60,
    F3 = 61,
    F4 = 62,
    F5 = 63,
    F6 = 64,
    F7 = 65,
    F8 = 66,
    F9 = 67,
    F10 = 68,
    F11 = 69,
    F12 = 70,
    F13 = 71,
    F14 = 72,
    F15 = 73,
    F16 = 74,
    F17 = 75,
    F18 = 76,
    F19 = 77,
    F20 = 78,
    F21 = 79,
    F22 = 80,
    F23 = 81,
    F24 = 82,
    NumLock = 83,
    ScrollLock = 84,
    /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the ';:' key
         */ 
    Semicolon = 85,
    /**
         * For any country/region, the '+' key
         * For the US standard keyboard, the '=+' key
         */ 
    Equal = 86,
    /**
         * For any country/region, the ',' key
         * For the US standard keyboard, the ',<' key
         */ 
    Comma = 87,
    /**
         * For any country/region, the '-' key
         * For the US standard keyboard, the '-_' key
         */ 
    Minus = 88,
    /**
         * For any country/region, the '.' key
         * For the US standard keyboard, the '.>' key
         */ 
    Period = 89,
    /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the '/?' key
         */ 
    Slash = 90,
    /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the '`~' key
         */ 
    Backquote = 91,
    /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the '[{' key
         */ 
    BracketLeft = 92,
    /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the '\|' key
         */ 
    Backslash = 93,
    /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the ']}' key
         */ 
    BracketRight = 94,
    /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the ''"' key
         */ 
    Quote = 95,
    /**
         * Used for miscellaneous characters; it can vary by keyboard.
         */ 
    OEM_8 = 96,
    /**
         * Either the angle bracket key or the backslash key on the RT 102-key keyboard.
         */ 
    IntlBackslash = 97,
    Numpad0 = 98,
    Numpad1 = 99,
    Numpad2 = 100,
    Numpad3 = 101,
    Numpad4 = 102,
    Numpad5 = 103,
    Numpad6 = 104,
    Numpad7 = 105,
    Numpad8 = 106,
    Numpad9 = 107,
    NumpadMultiply = 108,
    NumpadAdd = 109,
    NUMPAD_SEPARATOR = 110,
    NumpadSubtract = 111,
    NumpadDecimal = 112,
    NumpadDivide = 113,
    /**
         * Cover all key codes when IME is processing input.
         */ 
    KEY_IN_COMPOSITION = 114,
    ABNT_C1 = 115,
    ABNT_C2 = 116,
    AudioVolumeMute = 117,
    AudioVolumeUp = 118,
    AudioVolumeDown = 119,
    BrowserSearch = 120,
    BrowserHome = 121,
    BrowserBack = 122,
    BrowserForward = 123,
    MediaTrackNext = 124,
    MediaTrackPrevious = 125,
    MediaStop = 126,
    MediaPlayPause = 127,
    LaunchMediaPlayer = 128,
    LaunchMail = 129,
    LaunchApp2 = 130,
    /**
         * VK_CLEAR, 0x0C, CLEAR key
         */ 
    Clear = 131,
    /**
         * Placed last to cover the length of the enum.
         * Please do not depend on this value!
         */ 
    MAX_VALUE = 132,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type KeyMod;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = KeyMod)]
    pub fn chord(first_part: f64, second_part: f64, ) -> f64;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMarkdownString;

    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "value", getter)]
    pub fn value(this: &IMarkdownString) -> String;
    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "isTrusted", getter)]
    pub fn is_trusted(this: &IMarkdownString) -> JsValue;
    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "supportThemeIcons", getter)]
    pub fn support_theme_icons(this: &IMarkdownString) -> bool;
    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "supportHtml", getter)]
    pub fn support_html(this: &IMarkdownString) -> bool;
    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "baseUri", getter)]
    pub fn base_uri(this: &IMarkdownString) -> UriComponents;
    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "uris", getter)]
    pub fn uris(this: &IMarkdownString) -> js_sys::Object;

    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "uris", setter)]
    pub fn set_uris(this: &IMarkdownString, uris: js_sys::Object);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type MarkdownStringTrustedOptions;

    #[wasm_bindgen(method, js_class = "MarkdownStringTrustedOptions", js_name = "enabledCommands", getter)]
    pub fn enabled_commands(this: &MarkdownStringTrustedOptions) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IKeyboardEvent;

    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "_standardKeyboardEventBrand", getter)]
    pub fn standard_keyboard_event_brand(this: &IKeyboardEvent) -> JsValue;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "browserEvent", getter)]
    pub fn browser_event(this: &IKeyboardEvent) -> KeyboardEvent;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "target", getter)]
    pub fn target(this: &IKeyboardEvent) -> HTMLElement;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "ctrlKey", getter)]
    pub fn ctrl_key(this: &IKeyboardEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "shiftKey", getter)]
    pub fn shift_key(this: &IKeyboardEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "altKey", getter)]
    pub fn alt_key(this: &IKeyboardEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "metaKey", getter)]
    pub fn meta_key(this: &IKeyboardEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "altGraphKey", getter)]
    pub fn alt_graph_key(this: &IKeyboardEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "keyCode", getter)]
    pub fn key_code(this: &IKeyboardEvent) -> KeyCode;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "code", getter)]
    pub fn code(this: &IKeyboardEvent) -> String;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "equals")]
    pub fn equals(this: &IKeyboardEvent, keybinding: f64, ) -> bool;

    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "preventDefault")]
    pub fn prevent_default(this: &IKeyboardEvent, );

    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "stopPropagation")]
    pub fn stop_propagation(this: &IKeyboardEvent, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseEvent;

    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "browserEvent", getter)]
    pub fn browser_event(this: &IMouseEvent) -> MouseEvent;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "leftButton", getter)]
    pub fn left_button(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "middleButton", getter)]
    pub fn middle_button(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "rightButton", getter)]
    pub fn right_button(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "buttons", getter)]
    pub fn buttons(this: &IMouseEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "target", getter)]
    pub fn target(this: &IMouseEvent) -> HTMLElement;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "detail", getter)]
    pub fn detail(this: &IMouseEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "posx", getter)]
    pub fn posx(this: &IMouseEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "posy", getter)]
    pub fn posy(this: &IMouseEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "ctrlKey", getter)]
    pub fn ctrl_key(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "shiftKey", getter)]
    pub fn shift_key(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "altKey", getter)]
    pub fn alt_key(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "metaKey", getter)]
    pub fn meta_key(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "timestamp", getter)]
    pub fn timestamp(this: &IMouseEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "preventDefault")]
    pub fn prevent_default(this: &IMouseEvent, );

    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "stopPropagation")]
    pub fn stop_propagation(this: &IMouseEvent, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IScrollEvent;

    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollTop", getter)]
    pub fn scroll_top(this: &IScrollEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollLeft", getter)]
    pub fn scroll_left(this: &IScrollEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollWidth", getter)]
    pub fn scroll_width(this: &IScrollEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollHeight", getter)]
    pub fn scroll_height(this: &IScrollEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollTopChanged", getter)]
    pub fn scroll_top_changed(this: &IScrollEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollLeftChanged", getter)]
    pub fn scroll_left_changed(this: &IScrollEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollWidthChanged", getter)]
    pub fn scroll_width_changed(this: &IScrollEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollHeightChanged", getter)]
    pub fn scroll_height_changed(this: &IScrollEvent) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IPosition;

    /**
         * line number (starts at 1)
         */ 
    #[wasm_bindgen(method, js_class = "IPosition", js_name = "lineNumber", getter)]
    pub fn line_number(this: &IPosition) -> f64;
    /**
         * column (the first character in a line is between column 1 and column 2)
         */ 
    #[wasm_bindgen(method, js_class = "IPosition", js_name = "column", getter)]
    pub fn column(this: &IPosition) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Position;

  #[wasm_bindgen(constructor, js_class = "Position")]
  pub fn new() -> Position;

    /**
         * Create a new position from this position.
         *
         * @param newLineNumber new line number
         * @param newColumn new column
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn with(this: &Position, new_line_number: Option<f64>, new_column: Option<f64>, ) -> Position;

    /**
         * Derive a new position from this position.
         *
         * @param deltaLineNumber line number delta
         * @param deltaColumn column delta
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn delta(this: &Position, delta_line_number: Option<f64>, delta_column: Option<f64>, ) -> Position;

    /**
         * Test if this position equals other position
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn equals(this: &Position, other: IPosition, ) -> bool;

    /**
         * Test if position `a` equals position `b`
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Position)]
    pub fn equals_1(a: JsValue, b: JsValue, ) -> bool;

    /**
         * Test if this position is before other position.
         * If the two positions are equal, the result will be false.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn is_before(this: &Position, other: IPosition, ) -> bool;

    /**
         * Test if position `a` is before position `b`.
         * If the two positions are equal, the result will be false.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Position)]
    pub fn is_before_1(a: IPosition, b: IPosition, ) -> bool;

    /**
         * Test if this position is before other position.
         * If the two positions are equal, the result will be true.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn is_before_or_equal(this: &Position, other: IPosition, ) -> bool;

    /**
         * Test if position `a` is before position `b`.
         * If the two positions are equal, the result will be true.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Position)]
    pub fn is_before_or_equal_1(a: IPosition, b: IPosition, ) -> bool;

    /**
         * A function that compares positions, useful for sorting
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Position)]
    pub fn compare(a: IPosition, b: IPosition, ) -> f64;

    /**
         * Clone this position.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn clone(this: &Position, ) -> Position;

    /**
         * Convert to a human-readable representation.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_string(this: &Position, ) -> String;

    /**
         * Create a `Position` from an `IPosition`.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Position)]
    pub fn lift(pos: IPosition, ) -> Position;

    /**
         * Test if `obj` is an `IPosition`.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Position)]
    pub fn is_i_position(obj: JsValue, ) -> JsValue;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IRange;

    /**
         * Line number on which the range starts (starts at 1).
         */ 
    #[wasm_bindgen(method, js_class = "IRange", js_name = "startLineNumber", getter)]
    pub fn start_line_number(this: &IRange) -> f64;
    /**
         * Column on which the range starts in line `startLineNumber` (starts at 1).
         */ 
    #[wasm_bindgen(method, js_class = "IRange", js_name = "startColumn", getter)]
    pub fn start_column(this: &IRange) -> f64;
    /**
         * Line number on which the range ends.
         */ 
    #[wasm_bindgen(method, js_class = "IRange", js_name = "endLineNumber", getter)]
    pub fn end_line_number(this: &IRange) -> f64;
    /**
         * Column on which the range ends in line `endLineNumber`.
         */ 
    #[wasm_bindgen(method, js_class = "IRange", js_name = "endColumn", getter)]
    pub fn end_column(this: &IRange) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Range;

  #[wasm_bindgen(constructor, js_class = "Range")]
  pub fn new() -> Range;

    /**
         * Test if this range is empty.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn is_empty(this: &Range, ) -> bool;

    /**
         * Test if `range` is empty.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn is_empty_1(range: IRange, ) -> bool;

    /**
         * Test if position is in this range. If the position is at the edges, will return true.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn contains_position(this: &Range, position: IPosition, ) -> bool;

    /**
         * Test if `position` is in `range`. If the position is at the edges, will return true.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn contains_position_1(range: IRange, position: IPosition, ) -> bool;

    /**
         * Test if range is in this range. If the range is equal to this range, will return true.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn contains_range(this: &Range, range: IRange, ) -> bool;

    /**
         * Test if `otherRange` is in `range`. If the ranges are equal, will return true.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn contains_range_1(range: IRange, other_range: IRange, ) -> bool;

    /**
         * Test if `range` is strictly in this range. `range` must start after and end before this range for the result to be true.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn strict_contains_range(this: &Range, range: IRange, ) -> bool;

    /**
         * Test if `otherRange` is strictly in `range` (must start after, and end before). If the ranges are equal, will return false.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn strict_contains_range_1(range: IRange, other_range: IRange, ) -> bool;

    /**
         * A reunion of the two ranges.
         * The smallest position will be used as the start point, and the largest one as the end point.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn plus_range(this: &Range, range: IRange, ) -> Range;

    /**
         * A reunion of the two ranges.
         * The smallest position will be used as the start point, and the largest one as the end point.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn plus_range_1(a: IRange, b: IRange, ) -> Range;

    /**
         * A intersection of the two ranges.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn intersect_ranges(this: &Range, range: IRange, ) -> JsValue;

    /**
         * A intersection of the two ranges.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn intersect_ranges_1(a: IRange, b: IRange, ) -> JsValue;

    /**
         * Test if this range equals other.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn equals_range(this: &Range, other: JsValue, ) -> bool;

    /**
         * Test if range `a` equals `b`.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn equals_range_1(a: JsValue, b: JsValue, ) -> bool;

    /**
         * Return the end position (which will be after or equal to the start position)
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn get_end_position(this: &Range, ) -> Position;

    /**
         * Return the end position (which will be after or equal to the start position)
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn get_end_position_1(range: IRange, ) -> Position;

    /**
         * Return the start position (which will be before or equal to the end position)
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn get_start_position(this: &Range, ) -> Position;

    /**
         * Return the start position (which will be before or equal to the end position)
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn get_start_position_1(range: IRange, ) -> Position;

    /**
         * Transform to a user presentable string representation.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_string(this: &Range, ) -> String;

    /**
         * Create a new range using this range's start position, and using endLineNumber and endColumn as the end position.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn set_end_position(this: &Range, end_line_number: f64, end_column: f64, ) -> Range;

    /**
         * Create a new range using this range's end position, and using startLineNumber and startColumn as the start position.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn set_start_position(this: &Range, start_line_number: f64, start_column: f64, ) -> Range;

    /**
         * Create a new empty range using this range's start position.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn collapse_to_start(this: &Range, ) -> Range;

    /**
         * Create a new empty range using this range's start position.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn collapse_to_start_1(range: IRange, ) -> Range;

    /**
         * Create a new empty range using this range's end position.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn collapse_to_end(this: &Range, ) -> Range;

    /**
         * Create a new empty range using this range's end position.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn collapse_to_end_1(range: IRange, ) -> Range;

    /**
         * Moves the range by the given amount of lines.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn delta(this: &Range, line_count: f64, ) -> Range;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn from_positions(start: IPosition, end: Option<IPosition>, ) -> Range;

    /**
         * Create a `Range` from an `IRange`.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn lift(range: JsValue, ) -> JsValue;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn lift_1(range: IRange, ) -> Range;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn lift_2(range: JsValue, ) -> JsValue;

    /**
         * Test if `obj` is an `IRange`.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn is_i_range(obj: JsValue, ) -> JsValue;

    /**
         * Test if the two ranges are touching in any way.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn are_intersecting_or_touching(a: IRange, b: IRange, ) -> bool;

    /**
         * Test if the two ranges are intersecting. If the ranges are touching it returns true.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn are_intersecting(a: IRange, b: IRange, ) -> bool;

    /**
         * A function that compares ranges, useful for sorting ranges
         * It will first compare ranges on the startPosition and then on the endPosition
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn compare_ranges_using_starts(a: JsValue, b: JsValue, ) -> f64;

    /**
         * A function that compares ranges, useful for sorting ranges
         * It will first compare ranges on the endPosition and then on the startPosition
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn compare_ranges_using_ends(a: IRange, b: IRange, ) -> f64;

    /**
         * Test if the range spans multiple lines.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Range)]
    pub fn spans_multiple_lines(range: IRange, ) -> bool;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_json(this: &Range, ) -> IRange;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ISelection;

    /**
         * The line number on which the selection has started.
         */ 
    #[wasm_bindgen(method, js_class = "ISelection", js_name = "selectionStartLineNumber", getter)]
    pub fn selection_start_line_number(this: &ISelection) -> f64;
    /**
         * The column on `selectionStartLineNumber` where the selection has started.
         */ 
    #[wasm_bindgen(method, js_class = "ISelection", js_name = "selectionStartColumn", getter)]
    pub fn selection_start_column(this: &ISelection) -> f64;
    /**
         * The line number on which the selection has ended.
         */ 
    #[wasm_bindgen(method, js_class = "ISelection", js_name = "positionLineNumber", getter)]
    pub fn position_line_number(this: &ISelection) -> f64;
    /**
         * The column on `positionLineNumber` where the selection has ended.
         */ 
    #[wasm_bindgen(method, js_class = "ISelection", js_name = "positionColumn", getter)]
    pub fn position_column(this: &ISelection) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object, extends = Range)]
    pub type Selection;

  #[wasm_bindgen(constructor, js_class = "Selection")]
  pub fn new() -> Selection;

    /**
         * Transform to a human-readable representation.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_string(this: &Selection, ) -> String;

    /**
         * Test if equals other selection.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn equals_selection(this: &Selection, other: ISelection, ) -> bool;

    /**
         * Test if the two selections are equal.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Selection)]
    pub fn selections_equal(a: ISelection, b: ISelection, ) -> bool;

    /**
         * Get directions (LTR or RTL).
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn get_direction(this: &Selection, ) -> SelectionDirection;

    /**
         * Create a new selection with a different `positionLineNumber` and `positionColumn`.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn set_end_position(this: &Selection, end_line_number: f64, end_column: f64, ) -> Selection;

    /**
         * Get the position at `positionLineNumber` and `positionColumn`.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn get_position(this: &Selection, ) -> Position;

    /**
         * Get the position at the start of the selection.
        */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn get_selection_start(this: &Selection, ) -> Position;

    /**
         * Create a new selection with a different `selectionStartLineNumber` and `selectionStartColumn`.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn set_start_position(this: &Selection, start_line_number: f64, start_column: f64, ) -> Selection;

    /**
         * Create a `Selection` from one or two positions
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Selection)]
    pub fn from_positions(start: IPosition, end: Option<IPosition>, ) -> Selection;

    /**
         * Creates a `Selection` from a range, given a direction.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Selection)]
    pub fn from_range(range: Range, direction: SelectionDirection, ) -> Selection;

    /**
         * Create a `Selection` from an `ISelection`.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Selection)]
    pub fn lift_selection(sel: ISelection, ) -> Selection;

    /**
         * `a` equals `b`.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Selection)]
    pub fn selections_arr_equal(a: js_sys::Array, b: js_sys::Array, ) -> bool;

    /**
         * Test if `obj` is an `ISelection`.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Selection)]
    pub fn is_i_selection(obj: JsValue, ) -> JsValue;

    /**
         * Create with a direction.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = Selection)]
    pub fn create_with_direction(start_line_number: f64, start_column: f64, end_line_number: f64, end_column: f64, direction: SelectionDirection, ) -> Selection;


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum SelectionDirection {
    /**
         * The selection starts above where it ends.
         */ 
    LTR = 0,
    /**
         * The selection starts below where it ends.
         */ 
    RTL = 1,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Token;

  #[wasm_bindgen(constructor, js_class = "Token")]
  pub fn new() -> Token;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_string(this: &Token, ) -> String;


}

pub mod editor {

#[allow(unused)]
use wasm_bindgen::prelude::*;
#[allow(unused)]
use web_sys::*;
#[allow(unused)]
use js_sys::*;
#[allow(unused)]
use super::*;

#[allow(unused)]
pub type HTMLElement = HtmlElement;
#[allow(unused)]
pub type ReadonlyArray = Array;
#[allow(unused)]
pub type NonNullable = JsValue;
#[allow(unused)]
pub type Record = JsValue;
#[allow(unused)]
pub type PromiseLike = JsValue;



#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDiffNavigator;

    #[wasm_bindgen(method, js_class = "IDiffNavigator", js_name = "canNavigate")]
    pub fn can_navigate(this: &IDiffNavigator, ) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffNavigator", js_name = "next")]
    pub fn next(this: &IDiffNavigator, );

    #[wasm_bindgen(method, js_class = "IDiffNavigator", js_name = "previous")]
    pub fn previous(this: &IDiffNavigator, );

    #[wasm_bindgen(method, js_class = "IDiffNavigator", js_name = "dispose")]
    pub fn dispose(this: &IDiffNavigator, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDiffNavigatorOptions;

    #[wasm_bindgen(method, js_class = "IDiffNavigatorOptions", js_name = "followsCaret", getter)]
    pub fn follows_caret(this: &IDiffNavigatorOptions) -> bool;
    #[wasm_bindgen(method, js_class = "IDiffNavigatorOptions", js_name = "ignoreCharChanges", getter)]
    pub fn ignore_char_changes(this: &IDiffNavigatorOptions) -> bool;
    #[wasm_bindgen(method, js_class = "IDiffNavigatorOptions", js_name = "alwaysRevealFirst", getter)]
    pub fn always_reveal_first(this: &IDiffNavigatorOptions) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICommandDescriptor;

    /**
         * An unique identifier of the contributed command.
         */ 
    #[wasm_bindgen(method, js_class = "ICommandDescriptor", js_name = "id", getter)]
    pub fn id(this: &ICommandDescriptor) -> String;

    #[wasm_bindgen(method, js_class = "ICommandDescriptor", js_name = "id", setter)]
    pub fn set_id(this: &ICommandDescriptor, id: String);

    /**
         * Callback that will be executed when the command is triggered.
         */ 
    #[wasm_bindgen(method, js_class = "ICommandDescriptor", js_name = "run", getter)]
    pub fn run(this: &ICommandDescriptor) -> ICommandHandler;

    #[wasm_bindgen(method, js_class = "ICommandDescriptor", js_name = "run", setter)]
    pub fn set_run(this: &ICommandDescriptor, run: ICommandHandler);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IKeybindingRule;

    #[wasm_bindgen(method, js_class = "IKeybindingRule", js_name = "keybinding", getter)]
    pub fn keybinding(this: &IKeybindingRule) -> f64;

    #[wasm_bindgen(method, js_class = "IKeybindingRule", js_name = "keybinding", setter)]
    pub fn set_keybinding(this: &IKeybindingRule, keybinding: f64);

    #[wasm_bindgen(method, js_class = "IKeybindingRule", js_name = "command", getter)]
    pub fn command(this: &IKeybindingRule) -> JsValue;

    #[wasm_bindgen(method, js_class = "IKeybindingRule", js_name = "command", setter)]
    pub fn set_command(this: &IKeybindingRule, command: JsValue);

    #[wasm_bindgen(method, js_class = "IKeybindingRule", js_name = "commandArgs", getter)]
    pub fn command_args(this: &IKeybindingRule) -> JsValue;

    #[wasm_bindgen(method, js_class = "IKeybindingRule", js_name = "commandArgs", setter)]
    pub fn set_command_args(this: &IKeybindingRule, command_args: JsValue);

    #[wasm_bindgen(method, js_class = "IKeybindingRule", js_name = "when", getter)]
    pub fn when(this: &IKeybindingRule) -> JsValue;

    #[wasm_bindgen(method, js_class = "IKeybindingRule", js_name = "when", setter)]
    pub fn set_when(this: &IKeybindingRule, when: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ILinkOpener;

    #[wasm_bindgen(method, js_class = "ILinkOpener", js_name = "open")]
    pub fn open(this: &ILinkOpener, resource: Uri, ) -> JsValue;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICodeEditorOpener;

    /**
         * Callback that is invoked when a resource other than the current model should be opened (e.g. when "go to definition" is called).
         * The callback should return `true` if the request was handled and `false` otherwise.
         * @param source The code editor instance that initiated the request.
         * @param resource The Uri of the resource that should be opened.
         * @param selectionOrPosition An optional position or selection inside the model corresponding to `resource` that can be used to set the cursor.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditorOpener", js_name = "openCodeEditor")]
    pub fn open_code_editor(this: &ICodeEditorOpener, source: ICodeEditor, resource: Uri, selection_or_position: JsValue, ) -> JsValue;


}

pub type BuiltinTheme = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IStandaloneThemeData;

    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "base", getter)]
    pub fn base(this: &IStandaloneThemeData) -> BuiltinTheme;

    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "base", setter)]
    pub fn set_base(this: &IStandaloneThemeData, base: BuiltinTheme);

    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "inherit", getter)]
    pub fn inherit(this: &IStandaloneThemeData) -> bool;

    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "inherit", setter)]
    pub fn set_inherit(this: &IStandaloneThemeData, inherit: bool);

    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "rules", getter)]
    pub fn rules(this: &IStandaloneThemeData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "rules", setter)]
    pub fn set_rules(this: &IStandaloneThemeData, rules: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "encodedTokensColors", getter)]
    pub fn encoded_tokens_colors(this: &IStandaloneThemeData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "encodedTokensColors", setter)]
    pub fn set_encoded_tokens_colors(this: &IStandaloneThemeData, encoded_tokens_colors: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "colors", getter)]
    pub fn colors(this: &IStandaloneThemeData) -> IColors;

    #[wasm_bindgen(method, js_class = "IStandaloneThemeData", js_name = "colors", setter)]
    pub fn set_colors(this: &IStandaloneThemeData, colors: IColors);


}

pub type IColors = js_sys::Object
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ITokenThemeRule;

    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "token", getter)]
    pub fn token(this: &ITokenThemeRule) -> String;

    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "token", setter)]
    pub fn set_token(this: &ITokenThemeRule, token: String);

    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "foreground", getter)]
    pub fn foreground(this: &ITokenThemeRule) -> String;

    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "foreground", setter)]
    pub fn set_foreground(this: &ITokenThemeRule, foreground: String);

    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "background", getter)]
    pub fn background(this: &ITokenThemeRule) -> String;

    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "background", setter)]
    pub fn set_background(this: &ITokenThemeRule, background: String);

    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "fontStyle", getter)]
    pub fn font_style(this: &ITokenThemeRule) -> String;

    #[wasm_bindgen(method, js_class = "ITokenThemeRule", js_name = "fontStyle", setter)]
    pub fn set_font_style(this: &ITokenThemeRule, font_style: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type MonacoWebWorker;

    /**
         * Terminate the web worker, thus invalidating the returned proxy.
         */ 
    #[wasm_bindgen(method, js_class = "MonacoWebWorker", js_name = "dispose")]
    pub fn dispose(this: &MonacoWebWorker, );

    /**
         * Get a proxy to the arbitrary loaded code.
         */ 
    #[wasm_bindgen(method, js_class = "MonacoWebWorker", js_name = "getProxy")]
    pub fn get_proxy(this: &MonacoWebWorker, ) -> Promise;

    /**
         * Synchronize (send) the models at `resources` to the web worker,
         * making them available in the monaco.worker.getMirrorModels().
         */ 
    #[wasm_bindgen(method, js_class = "MonacoWebWorker", js_name = "withSyncedResources")]
    pub fn with_synced_resources(this: &MonacoWebWorker, resources: js_sys::Array, ) -> Promise;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IWebWorkerOptions;

    /**
         * The AMD moduleId to load.
         * It should export a function `create` that should return the exported proxy.
         */ 
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "moduleId", getter)]
    pub fn module_id(this: &IWebWorkerOptions) -> String;

    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "moduleId", setter)]
    pub fn set_module_id(this: &IWebWorkerOptions, module_id: String);

    /**
         * The data to send over when calling create on the module.
         */ 
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "createData", getter)]
    pub fn create_data(this: &IWebWorkerOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "createData", setter)]
    pub fn set_create_data(this: &IWebWorkerOptions, create_data: JsValue);

    /**
         * A label to be used to identify the web worker for debugging purposes.
         */ 
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "label", getter)]
    pub fn label(this: &IWebWorkerOptions) -> String;

    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "label", setter)]
    pub fn set_label(this: &IWebWorkerOptions, label: String);

    /**
         * An object that can be used by the web worker to make calls back to the main thread.
         */ 
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "host", getter)]
    pub fn host(this: &IWebWorkerOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "host", setter)]
    pub fn set_host(this: &IWebWorkerOptions, host: JsValue);

    /**
         * Keep idle models.
         * Defaults to false, which means that idle models will stop syncing after a while.
         */ 
    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "keepIdleModels", getter)]
    pub fn keep_idle_models(this: &IWebWorkerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IWebWorkerOptions", js_name = "keepIdleModels", setter)]
    pub fn set_keep_idle_models(this: &IWebWorkerOptions, keep_idle_models: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IActionDescriptor;

    /**
         * An unique identifier of the contributed action.
         */ 
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "id", getter)]
    pub fn id(this: &IActionDescriptor) -> String;

    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "id", setter)]
    pub fn set_id(this: &IActionDescriptor, id: String);

    /**
         * A label of the action that will be presented to the user.
         */ 
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "label", getter)]
    pub fn label(this: &IActionDescriptor) -> String;

    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "label", setter)]
    pub fn set_label(this: &IActionDescriptor, label: String);

    /**
         * Precondition rule.
         */ 
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "precondition", getter)]
    pub fn precondition(this: &IActionDescriptor) -> String;

    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "precondition", setter)]
    pub fn set_precondition(this: &IActionDescriptor, precondition: String);

    /**
         * An array of keybindings for the action.
         */ 
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "keybindings", getter)]
    pub fn keybindings(this: &IActionDescriptor) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "keybindings", setter)]
    pub fn set_keybindings(this: &IActionDescriptor, keybindings: js_sys::Array);

    /**
         * The keybinding rule (condition on top of precondition).
         */ 
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "keybindingContext", getter)]
    pub fn keybinding_context(this: &IActionDescriptor) -> String;

    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "keybindingContext", setter)]
    pub fn set_keybinding_context(this: &IActionDescriptor, keybinding_context: String);

    /**
         * Control if the action should show up in the context menu and where.
         * The context menu of the editor has these default:
         *   navigation - The navigation group comes first in all cases.
         *   1_modification - This group comes next and contains commands that modify your code.
         *   9_cutcopypaste - The last default group with the basic editing commands.
         * You can also create your own group.
         * Defaults to null (don't show in context menu).
         */ 
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "contextMenuGroupId", getter)]
    pub fn context_menu_group_id(this: &IActionDescriptor) -> String;

    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "contextMenuGroupId", setter)]
    pub fn set_context_menu_group_id(this: &IActionDescriptor, context_menu_group_id: String);

    /**
         * Control the order in the context menu group.
         */ 
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "contextMenuOrder", getter)]
    pub fn context_menu_order(this: &IActionDescriptor) -> f64;

    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "contextMenuOrder", setter)]
    pub fn set_context_menu_order(this: &IActionDescriptor, context_menu_order: f64);

    /**
         * Method that will be executed when the action is triggered.
         * @param editor The editor instance is passed in as a convenience
         */ 
    #[wasm_bindgen(method, js_class = "IActionDescriptor", js_name = "run")]
    pub fn run(this: &IActionDescriptor, editor: ICodeEditor, ) -> JsValue;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IGlobalEditorOptions;

    /**
         * The number of spaces a tab is equal to.
         * This setting is overridden based on the file contents when `detectIndentation` is on.
         * Defaults to 4.
         */ 
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "tabSize", getter)]
    pub fn tab_size(this: &IGlobalEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "tabSize", setter)]
    pub fn set_tab_size(this: &IGlobalEditorOptions, tab_size: f64);

    /**
         * Insert spaces when pressing `Tab`.
         * This setting is overridden based on the file contents when `detectIndentation` is on.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "insertSpaces", getter)]
    pub fn insert_spaces(this: &IGlobalEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "insertSpaces", setter)]
    pub fn set_insert_spaces(this: &IGlobalEditorOptions, insert_spaces: bool);

    /**
         * Controls whether `tabSize` and `insertSpaces` will be automatically detected when a file is opened based on the file contents.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "detectIndentation", getter)]
    pub fn detect_indentation(this: &IGlobalEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "detectIndentation", setter)]
    pub fn set_detect_indentation(this: &IGlobalEditorOptions, detect_indentation: bool);

    /**
         * Remove trailing auto inserted whitespace.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "trimAutoWhitespace", getter)]
    pub fn trim_auto_whitespace(this: &IGlobalEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "trimAutoWhitespace", setter)]
    pub fn set_trim_auto_whitespace(this: &IGlobalEditorOptions, trim_auto_whitespace: bool);

    /**
         * Special handling for large files to disable certain memory intensive features.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "largeFileOptimizations", getter)]
    pub fn large_file_optimizations(this: &IGlobalEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "largeFileOptimizations", setter)]
    pub fn set_large_file_optimizations(this: &IGlobalEditorOptions, large_file_optimizations: bool);

    /**
         * Controls whether completions should be computed based on words in the document.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "wordBasedSuggestions", getter)]
    pub fn word_based_suggestions(this: &IGlobalEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "wordBasedSuggestions", setter)]
    pub fn set_word_based_suggestions(this: &IGlobalEditorOptions, word_based_suggestions: bool);

    /**
         * Controls whether word based completions should be included from opened documents of the same language or any language.
         */ 
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "wordBasedSuggestionsOnlySameLanguage", getter)]
    pub fn word_based_suggestions_only_same_language(this: &IGlobalEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "wordBasedSuggestionsOnlySameLanguage", setter)]
    pub fn set_word_based_suggestions_only_same_language(this: &IGlobalEditorOptions, word_based_suggestions_only_same_language: bool);

    /**
         * Keep peek editors open even when double-clicking their content or when hitting `Escape`.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "stablePeek", getter)]
    pub fn stable_peek(this: &IGlobalEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "stablePeek", setter)]
    pub fn set_stable_peek(this: &IGlobalEditorOptions, stable_peek: bool);

    /**
         * Lines above this length will not be tokenized for performance reasons.
         * Defaults to 20000.
         */ 
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "maxTokenizationLineLength", getter)]
    pub fn max_tokenization_line_length(this: &IGlobalEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "maxTokenizationLineLength", setter)]
    pub fn set_max_tokenization_line_length(this: &IGlobalEditorOptions, max_tokenization_line_length: f64);

    /**
         * Theme to be used for rendering.
         * The current out-of-the-box available themes are: 'vs' (default), 'vs-dark', 'hc-black', 'hc-light'.
         * You can create custom themes via `monaco.editor.defineTheme`.
         * To switch a theme, use `monaco.editor.setTheme`.
         * **NOTE**: The theme might be overwritten if the OS is in high contrast mode, unless `autoDetectHighContrast` is set to false.
         */ 
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "theme", getter)]
    pub fn theme(this: &IGlobalEditorOptions) -> String;

    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "theme", setter)]
    pub fn set_theme(this: &IGlobalEditorOptions, theme: String);

    /**
         * If enabled, will automatically change to high contrast theme if the OS is using a high contrast theme.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "autoDetectHighContrast", getter)]
    pub fn auto_detect_high_contrast(this: &IGlobalEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IGlobalEditorOptions", js_name = "autoDetectHighContrast", setter)]
    pub fn set_auto_detect_high_contrast(this: &IGlobalEditorOptions, auto_detect_high_contrast: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IStandaloneEditorConstructionOptions;

    /**
         * The initial model associated with this code editor.
         */ 
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "model", getter)]
    pub fn model(this: &IStandaloneEditorConstructionOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "model", setter)]
    pub fn set_model(this: &IStandaloneEditorConstructionOptions, model: JsValue);

    /**
         * The initial value of the auto created model in the editor.
         * To not automatically create a model, use `model: null`.
         */ 
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "value", getter)]
    pub fn value(this: &IStandaloneEditorConstructionOptions) -> String;

    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "value", setter)]
    pub fn set_value(this: &IStandaloneEditorConstructionOptions, value: String);

    /**
         * The initial language of the auto created model in the editor.
         * To not automatically create a model, use `model: null`.
         */ 
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "language", getter)]
    pub fn language(this: &IStandaloneEditorConstructionOptions) -> String;

    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "language", setter)]
    pub fn set_language(this: &IStandaloneEditorConstructionOptions, language: String);

    /**
         * Initial theme to be used for rendering.
         * The current out-of-the-box available themes are: 'vs' (default), 'vs-dark', 'hc-black', 'hc-light.
         * You can create custom themes via `monaco.editor.defineTheme`.
         * To switch a theme, use `monaco.editor.setTheme`.
         * **NOTE**: The theme might be overwritten if the OS is in high contrast mode, unless `autoDetectHighContrast` is set to false.
         */ 
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "theme", getter)]
    pub fn theme(this: &IStandaloneEditorConstructionOptions) -> String;

    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "theme", setter)]
    pub fn set_theme(this: &IStandaloneEditorConstructionOptions, theme: String);

    /**
         * If enabled, will automatically change to high contrast theme if the OS is using a high contrast theme.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "autoDetectHighContrast", getter)]
    pub fn auto_detect_high_contrast(this: &IStandaloneEditorConstructionOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "autoDetectHighContrast", setter)]
    pub fn set_auto_detect_high_contrast(this: &IStandaloneEditorConstructionOptions, auto_detect_high_contrast: bool);

    /**
         * An URL to open when Ctrl+H (Windows and Linux) or Cmd+H (OSX) is pressed in
         * the accessibility help dialog in the editor.
         *
         * Defaults to "https://go.microsoft.com/fwlink/?linkid=852450"
         */ 
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "accessibilityHelpUrl", getter)]
    pub fn accessibility_help_url(this: &IStandaloneEditorConstructionOptions) -> String;

    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "accessibilityHelpUrl", setter)]
    pub fn set_accessibility_help_url(this: &IStandaloneEditorConstructionOptions, accessibility_help_url: String);

    /**
         * Container element to use for ARIA messages.
         * Defaults to document.body.
         */ 
    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "ariaContainerElement", getter)]
    pub fn aria_container_element(this: &IStandaloneEditorConstructionOptions) -> HTMLElement;

    #[wasm_bindgen(method, js_class = "IStandaloneEditorConstructionOptions", js_name = "ariaContainerElement", setter)]
    pub fn set_aria_container_element(this: &IStandaloneEditorConstructionOptions, aria_container_element: HTMLElement);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IStandaloneDiffEditorConstructionOptions;

    /**
         * Initial theme to be used for rendering.
         * The current out-of-the-box available themes are: 'vs' (default), 'vs-dark', 'hc-black', 'hc-light.
         * You can create custom themes via `monaco.editor.defineTheme`.
         * To switch a theme, use `monaco.editor.setTheme`.
         * **NOTE**: The theme might be overwritten if the OS is in high contrast mode, unless `autoDetectHighContrast` is set to false.
         */ 
    #[wasm_bindgen(method, js_class = "IStandaloneDiffEditorConstructionOptions", js_name = "theme", getter)]
    pub fn theme(this: &IStandaloneDiffEditorConstructionOptions) -> String;

    #[wasm_bindgen(method, js_class = "IStandaloneDiffEditorConstructionOptions", js_name = "theme", setter)]
    pub fn set_theme(this: &IStandaloneDiffEditorConstructionOptions, theme: String);

    /**
         * If enabled, will automatically change to high contrast theme if the OS is using a high contrast theme.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IStandaloneDiffEditorConstructionOptions", js_name = "autoDetectHighContrast", getter)]
    pub fn auto_detect_high_contrast(this: &IStandaloneDiffEditorConstructionOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IStandaloneDiffEditorConstructionOptions", js_name = "autoDetectHighContrast", setter)]
    pub fn set_auto_detect_high_contrast(this: &IStandaloneDiffEditorConstructionOptions, auto_detect_high_contrast: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IStandaloneCodeEditor;

    #[wasm_bindgen(method, js_class = "IStandaloneCodeEditor", js_name = "updateOptions")]
    pub fn update_options(this: &IStandaloneCodeEditor, new_options: JsValue, );

    #[wasm_bindgen(method, js_class = "IStandaloneCodeEditor", js_name = "addCommand")]
    pub fn add_command(this: &IStandaloneCodeEditor, keybinding: f64, handler: ICommandHandler, context: Option<String>, ) -> JsValue;

    #[wasm_bindgen(method, js_class = "IStandaloneCodeEditor", js_name = "createContextKey")]
    pub fn create_context_key(this: &IStandaloneCodeEditor, key: String, default_value: JsValue, ) -> IContextKey;

    #[wasm_bindgen(method, js_class = "IStandaloneCodeEditor", js_name = "addAction")]
    pub fn add_action(this: &IStandaloneCodeEditor, descriptor: IActionDescriptor, ) -> IDisposable;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IStandaloneDiffEditor;

    #[wasm_bindgen(method, js_class = "IStandaloneDiffEditor", js_name = "addCommand")]
    pub fn add_command(this: &IStandaloneDiffEditor, keybinding: f64, handler: ICommandHandler, context: Option<String>, ) -> JsValue;

    #[wasm_bindgen(method, js_class = "IStandaloneDiffEditor", js_name = "createContextKey")]
    pub fn create_context_key(this: &IStandaloneDiffEditor, key: String, default_value: JsValue, ) -> IContextKey;

    #[wasm_bindgen(method, js_class = "IStandaloneDiffEditor", js_name = "addAction")]
    pub fn add_action(this: &IStandaloneDiffEditor, descriptor: IActionDescriptor, ) -> IDisposable;

    #[wasm_bindgen(method, js_class = "IStandaloneDiffEditor", js_name = "getOriginalEditor")]
    pub fn get_original_editor(this: &IStandaloneDiffEditor, ) -> IStandaloneCodeEditor;

    #[wasm_bindgen(method, js_class = "IStandaloneDiffEditor", js_name = "getModifiedEditor")]
    pub fn get_modified_editor(this: &IStandaloneDiffEditor, ) -> IStandaloneCodeEditor;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICommandHandler;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IContextKey;

    #[wasm_bindgen(method, js_class = "IContextKey", js_name = "set")]
    pub fn set(this: &IContextKey, value: JsValue, );

    #[wasm_bindgen(method, js_class = "IContextKey", js_name = "reset")]
    pub fn reset(this: &IContextKey, );

    #[wasm_bindgen(method, js_class = "IContextKey", js_name = "get")]
    pub fn get(this: &IContextKey, ) -> JsValue;


}

pub type ContextKeyValue = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorOverrideServices;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMarker;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "owner", getter)]
    pub fn owner(this: &IMarker) -> String;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "owner", setter)]
    pub fn set_owner(this: &IMarker, owner: String);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "resource", getter)]
    pub fn resource(this: &IMarker) -> Uri;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "resource", setter)]
    pub fn set_resource(this: &IMarker, resource: Uri);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "severity", getter)]
    pub fn severity(this: &IMarker) -> MarkerSeverity;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "severity", setter)]
    pub fn set_severity(this: &IMarker, severity: MarkerSeverity);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "code", getter)]
    pub fn code(this: &IMarker) -> JsValue;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "code", setter)]
    pub fn set_code(this: &IMarker, code: JsValue);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "message", getter)]
    pub fn message(this: &IMarker) -> String;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "message", setter)]
    pub fn set_message(this: &IMarker, message: String);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "source", getter)]
    pub fn source(this: &IMarker) -> String;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "source", setter)]
    pub fn set_source(this: &IMarker, source: String);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "startLineNumber", getter)]
    pub fn start_line_number(this: &IMarker) -> f64;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "startLineNumber", setter)]
    pub fn set_start_line_number(this: &IMarker, start_line_number: f64);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "startColumn", getter)]
    pub fn start_column(this: &IMarker) -> f64;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "startColumn", setter)]
    pub fn set_start_column(this: &IMarker, start_column: f64);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "endLineNumber", getter)]
    pub fn end_line_number(this: &IMarker) -> f64;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "endLineNumber", setter)]
    pub fn set_end_line_number(this: &IMarker, end_line_number: f64);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "endColumn", getter)]
    pub fn end_column(this: &IMarker) -> f64;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "endColumn", setter)]
    pub fn set_end_column(this: &IMarker, end_column: f64);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "modelVersionId", getter)]
    pub fn model_version_id(this: &IMarker) -> f64;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "modelVersionId", setter)]
    pub fn set_model_version_id(this: &IMarker, model_version_id: f64);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "relatedInformation", getter)]
    pub fn related_information(this: &IMarker) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "relatedInformation", setter)]
    pub fn set_related_information(this: &IMarker, related_information: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "tags", getter)]
    pub fn tags(this: &IMarker) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IMarker", js_name = "tags", setter)]
    pub fn set_tags(this: &IMarker, tags: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMarkerData;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "code", getter)]
    pub fn code(this: &IMarkerData) -> JsValue;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "code", setter)]
    pub fn set_code(this: &IMarkerData, code: JsValue);

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "severity", getter)]
    pub fn severity(this: &IMarkerData) -> MarkerSeverity;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "severity", setter)]
    pub fn set_severity(this: &IMarkerData, severity: MarkerSeverity);

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "message", getter)]
    pub fn message(this: &IMarkerData) -> String;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "message", setter)]
    pub fn set_message(this: &IMarkerData, message: String);

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "source", getter)]
    pub fn source(this: &IMarkerData) -> String;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "source", setter)]
    pub fn set_source(this: &IMarkerData, source: String);

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "startLineNumber", getter)]
    pub fn start_line_number(this: &IMarkerData) -> f64;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "startLineNumber", setter)]
    pub fn set_start_line_number(this: &IMarkerData, start_line_number: f64);

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "startColumn", getter)]
    pub fn start_column(this: &IMarkerData) -> f64;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "startColumn", setter)]
    pub fn set_start_column(this: &IMarkerData, start_column: f64);

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "endLineNumber", getter)]
    pub fn end_line_number(this: &IMarkerData) -> f64;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "endLineNumber", setter)]
    pub fn set_end_line_number(this: &IMarkerData, end_line_number: f64);

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "endColumn", getter)]
    pub fn end_column(this: &IMarkerData) -> f64;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "endColumn", setter)]
    pub fn set_end_column(this: &IMarkerData, end_column: f64);

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "modelVersionId", getter)]
    pub fn model_version_id(this: &IMarkerData) -> f64;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "modelVersionId", setter)]
    pub fn set_model_version_id(this: &IMarkerData, model_version_id: f64);

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "relatedInformation", getter)]
    pub fn related_information(this: &IMarkerData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "relatedInformation", setter)]
    pub fn set_related_information(this: &IMarkerData, related_information: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "tags", getter)]
    pub fn tags(this: &IMarkerData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IMarkerData", js_name = "tags", setter)]
    pub fn set_tags(this: &IMarkerData, tags: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IRelatedInformation;

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "resource", getter)]
    pub fn resource(this: &IRelatedInformation) -> Uri;

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "resource", setter)]
    pub fn set_resource(this: &IRelatedInformation, resource: Uri);

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "message", getter)]
    pub fn message(this: &IRelatedInformation) -> String;

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "message", setter)]
    pub fn set_message(this: &IRelatedInformation, message: String);

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "startLineNumber", getter)]
    pub fn start_line_number(this: &IRelatedInformation) -> f64;

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "startLineNumber", setter)]
    pub fn set_start_line_number(this: &IRelatedInformation, start_line_number: f64);

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "startColumn", getter)]
    pub fn start_column(this: &IRelatedInformation) -> f64;

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "startColumn", setter)]
    pub fn set_start_column(this: &IRelatedInformation, start_column: f64);

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "endLineNumber", getter)]
    pub fn end_line_number(this: &IRelatedInformation) -> f64;

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "endLineNumber", setter)]
    pub fn set_end_line_number(this: &IRelatedInformation, end_line_number: f64);

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "endColumn", getter)]
    pub fn end_column(this: &IRelatedInformation) -> f64;

    #[wasm_bindgen(method, js_class = "IRelatedInformation", js_name = "endColumn", setter)]
    pub fn set_end_column(this: &IRelatedInformation, end_column: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IColorizerOptions;

    #[wasm_bindgen(method, js_class = "IColorizerOptions", js_name = "tabSize", getter)]
    pub fn tab_size(this: &IColorizerOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IColorizerOptions", js_name = "tabSize", setter)]
    pub fn set_tab_size(this: &IColorizerOptions, tab_size: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IColorizerElementOptions;

    #[wasm_bindgen(method, js_class = "IColorizerElementOptions", js_name = "theme", getter)]
    pub fn theme(this: &IColorizerElementOptions) -> String;

    #[wasm_bindgen(method, js_class = "IColorizerElementOptions", js_name = "theme", setter)]
    pub fn set_theme(this: &IColorizerElementOptions, theme: String);

    #[wasm_bindgen(method, js_class = "IColorizerElementOptions", js_name = "mimeType", getter)]
    pub fn mime_type(this: &IColorizerElementOptions) -> String;

    #[wasm_bindgen(method, js_class = "IColorizerElementOptions", js_name = "mimeType", setter)]
    pub fn set_mime_type(this: &IColorizerElementOptions, mime_type: String);


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum ScrollbarVisibility {
    Auto = 1,
    Hidden = 2,
    Visible = 3,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ThemeColor;

    #[wasm_bindgen(method, js_class = "ThemeColor", js_name = "id", getter)]
    pub fn id(this: &ThemeColor) -> String;

    #[wasm_bindgen(method, js_class = "ThemeColor", js_name = "id", setter)]
    pub fn set_id(this: &ThemeColor, id: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ISingleEditOperation;

    /**
         * The range to replace. This can be empty to emulate a simple insert.
         */ 
    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "range", getter)]
    pub fn range(this: &ISingleEditOperation) -> IRange;

    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "range", setter)]
    pub fn set_range(this: &ISingleEditOperation, range: IRange);

    /**
         * The text to replace with. This can be null to emulate a simple delete.
         */ 
    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "text", getter)]
    pub fn text(this: &ISingleEditOperation) -> JsValue;

    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "text", setter)]
    pub fn set_text(this: &ISingleEditOperation, text: JsValue);

    /**
         * This indicates that this operation has "insert" semantics.
         * i.e. forceMoveMarkers = true => if `range` is collapsed, all markers at the position will be moved.
         */ 
    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "forceMoveMarkers", getter)]
    pub fn force_move_markers(this: &ISingleEditOperation) -> bool;

    #[wasm_bindgen(method, js_class = "ISingleEditOperation", js_name = "forceMoveMarkers", setter)]
    pub fn set_force_move_markers(this: &ISingleEditOperation, force_move_markers: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IWordAtPosition;

    /**
         * The word.
         */ 
    #[wasm_bindgen(method, js_class = "IWordAtPosition", js_name = "word", getter)]
    pub fn word(this: &IWordAtPosition) -> String;
    /**
         * The column where the word starts.
         */ 
    #[wasm_bindgen(method, js_class = "IWordAtPosition", js_name = "startColumn", getter)]
    pub fn start_column(this: &IWordAtPosition) -> f64;
    /**
         * The column where the word ends.
         */ 
    #[wasm_bindgen(method, js_class = "IWordAtPosition", js_name = "endColumn", getter)]
    pub fn end_column(this: &IWordAtPosition) -> f64;

}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum OverviewRulerLane {
    Left = 1,
    Center = 2,
    Right = 4,
    Full = 7,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum GlyphMarginLane {
    Left = 1,
    Right = 2,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum MinimapPosition {
    Inline = 1,
    Gutter = 2,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDecorationOptions;

    /**
         * CSS color to render.
         * e.g.: rgba(100, 100, 100, 0.5) or a color from the color registry
         */ 
    #[wasm_bindgen(method, js_class = "IDecorationOptions", js_name = "color", getter)]
    pub fn color(this: &IDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IDecorationOptions", js_name = "color", setter)]
    pub fn set_color(this: &IDecorationOptions, color: JsValue);

    /**
         * CSS color to render.
         * e.g.: rgba(100, 100, 100, 0.5) or a color from the color registry
         */ 
    #[wasm_bindgen(method, js_class = "IDecorationOptions", js_name = "darkColor", getter)]
    pub fn dark_color(this: &IDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IDecorationOptions", js_name = "darkColor", setter)]
    pub fn set_dark_color(this: &IDecorationOptions, dark_color: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelDecorationGlyphMarginOptions;

    /**
         * The position in the glyph margin.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationGlyphMarginOptions", js_name = "position", getter)]
    pub fn position(this: &IModelDecorationGlyphMarginOptions) -> GlyphMarginLane;

    #[wasm_bindgen(method, js_class = "IModelDecorationGlyphMarginOptions", js_name = "position", setter)]
    pub fn set_position(this: &IModelDecorationGlyphMarginOptions, position: GlyphMarginLane);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelDecorationOverviewRulerOptions;

    /**
         * The position in the overview ruler.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOverviewRulerOptions", js_name = "position", getter)]
    pub fn position(this: &IModelDecorationOverviewRulerOptions) -> OverviewRulerLane;

    #[wasm_bindgen(method, js_class = "IModelDecorationOverviewRulerOptions", js_name = "position", setter)]
    pub fn set_position(this: &IModelDecorationOverviewRulerOptions, position: OverviewRulerLane);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelDecorationMinimapOptions;

    /**
         * The position in the minimap.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationMinimapOptions", js_name = "position", getter)]
    pub fn position(this: &IModelDecorationMinimapOptions) -> MinimapPosition;

    #[wasm_bindgen(method, js_class = "IModelDecorationMinimapOptions", js_name = "position", setter)]
    pub fn set_position(this: &IModelDecorationMinimapOptions, position: MinimapPosition);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelDecorationOptions;

    /**
         * Customize the growing behavior of the decoration when typing at the edges of the decoration.
         * Defaults to TrackedRangeStickiness.AlwaysGrowsWhenTypingAtEdges
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "stickiness", getter)]
    pub fn stickiness(this: &IModelDecorationOptions) -> TrackedRangeStickiness;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "stickiness", setter)]
    pub fn set_stickiness(this: &IModelDecorationOptions, stickiness: TrackedRangeStickiness);

    /**
         * CSS class name describing the decoration.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "className", getter)]
    pub fn class_name(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "className", setter)]
    pub fn set_class_name(this: &IModelDecorationOptions, class_name: JsValue);

    /**
         * Indicates whether the decoration should span across the entire line when it continues onto the next line.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "shouldFillLineOnLineBreak", getter)]
    pub fn should_fill_line_on_line_break(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "shouldFillLineOnLineBreak", setter)]
    pub fn set_should_fill_line_on_line_break(this: &IModelDecorationOptions, should_fill_line_on_line_break: JsValue);

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "blockClassName", getter)]
    pub fn block_class_name(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "blockClassName", setter)]
    pub fn set_block_class_name(this: &IModelDecorationOptions, block_class_name: JsValue);

    /**
         * Indicates if this block should be rendered after the last line.
         * In this case, the range must be empty and set to the last line.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "blockIsAfterEnd", getter)]
    pub fn block_is_after_end(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "blockIsAfterEnd", setter)]
    pub fn set_block_is_after_end(this: &IModelDecorationOptions, block_is_after_end: JsValue);

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "blockDoesNotCollapse", getter)]
    pub fn block_does_not_collapse(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "blockDoesNotCollapse", setter)]
    pub fn set_block_does_not_collapse(this: &IModelDecorationOptions, block_does_not_collapse: JsValue);

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "blockPadding", getter)]
    pub fn block_padding(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "blockPadding", setter)]
    pub fn set_block_padding(this: &IModelDecorationOptions, block_padding: JsValue);

    /**
         * Message to be rendered when hovering over the glyph margin decoration.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "glyphMarginHoverMessage", getter)]
    pub fn glyph_margin_hover_message(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "glyphMarginHoverMessage", setter)]
    pub fn set_glyph_margin_hover_message(this: &IModelDecorationOptions, glyph_margin_hover_message: JsValue);

    /**
         * Array of MarkdownString to render as the decoration message.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "hoverMessage", getter)]
    pub fn hover_message(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "hoverMessage", setter)]
    pub fn set_hover_message(this: &IModelDecorationOptions, hover_message: JsValue);

    /**
         * Should the decoration expand to encompass a whole line.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "isWholeLine", getter)]
    pub fn is_whole_line(this: &IModelDecorationOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "isWholeLine", setter)]
    pub fn set_is_whole_line(this: &IModelDecorationOptions, is_whole_line: bool);

    /**
         * Always render the decoration (even when the range it encompasses is collapsed).
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "showIfCollapsed", getter)]
    pub fn show_if_collapsed(this: &IModelDecorationOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "showIfCollapsed", setter)]
    pub fn set_show_if_collapsed(this: &IModelDecorationOptions, show_if_collapsed: bool);

    /**
         * Specifies the stack order of a decoration.
         * A decoration with greater stack order is always in front of a decoration with
         * a lower stack order when the decorations are on the same line.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "zIndex", getter)]
    pub fn z_index(this: &IModelDecorationOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "zIndex", setter)]
    pub fn set_z_index(this: &IModelDecorationOptions, z_index: f64);

    /**
         * If set, render this decoration in the overview ruler.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "overviewRuler", getter)]
    pub fn overview_ruler(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "overviewRuler", setter)]
    pub fn set_overview_ruler(this: &IModelDecorationOptions, overview_ruler: JsValue);

    /**
         * If set, render this decoration in the minimap.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "minimap", getter)]
    pub fn minimap(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "minimap", setter)]
    pub fn set_minimap(this: &IModelDecorationOptions, minimap: JsValue);

    /**
         * If set, the decoration will be rendered in the glyph margin with this CSS class name.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "glyphMarginClassName", getter)]
    pub fn glyph_margin_class_name(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "glyphMarginClassName", setter)]
    pub fn set_glyph_margin_class_name(this: &IModelDecorationOptions, glyph_margin_class_name: JsValue);

    /**
         * If set and the decoration has {@link glyphMarginClassName} set, render this decoration
         * with the specified {@link IModelDecorationGlyphMarginOptions} in the glyph margin.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "glyphMargin", getter)]
    pub fn glyph_margin(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "glyphMargin", setter)]
    pub fn set_glyph_margin(this: &IModelDecorationOptions, glyph_margin: JsValue);

    /**
         * If set, the decoration will be rendered in the lines decorations with this CSS class name.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "linesDecorationsClassName", getter)]
    pub fn lines_decorations_class_name(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "linesDecorationsClassName", setter)]
    pub fn set_lines_decorations_class_name(this: &IModelDecorationOptions, lines_decorations_class_name: JsValue);

    /**
         * If set, the decoration will be rendered in the lines decorations with this CSS class name, but only for the first line in case of line wrapping.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "firstLineDecorationClassName", getter)]
    pub fn first_line_decoration_class_name(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "firstLineDecorationClassName", setter)]
    pub fn set_first_line_decoration_class_name(this: &IModelDecorationOptions, first_line_decoration_class_name: JsValue);

    /**
         * If set, the decoration will be rendered in the margin (covering its full width) with this CSS class name.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "marginClassName", getter)]
    pub fn margin_class_name(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "marginClassName", setter)]
    pub fn set_margin_class_name(this: &IModelDecorationOptions, margin_class_name: JsValue);

    /**
         * If set, the decoration will be rendered inline with the text with this CSS class name.
         * Please use this only for CSS rules that must impact the text. For example, use `className`
         * to have a background color decoration.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "inlineClassName", getter)]
    pub fn inline_class_name(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "inlineClassName", setter)]
    pub fn set_inline_class_name(this: &IModelDecorationOptions, inline_class_name: JsValue);

    /**
         * If there is an `inlineClassName` which affects letter spacing.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "inlineClassNameAffectsLetterSpacing", getter)]
    pub fn inline_class_name_affects_letter_spacing(this: &IModelDecorationOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "inlineClassNameAffectsLetterSpacing", setter)]
    pub fn set_inline_class_name_affects_letter_spacing(this: &IModelDecorationOptions, inline_class_name_affects_letter_spacing: bool);

    /**
         * If set, the decoration will be rendered before the text with this CSS class name.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "beforeContentClassName", getter)]
    pub fn before_content_class_name(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "beforeContentClassName", setter)]
    pub fn set_before_content_class_name(this: &IModelDecorationOptions, before_content_class_name: JsValue);

    /**
         * If set, the decoration will be rendered after the text with this CSS class name.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "afterContentClassName", getter)]
    pub fn after_content_class_name(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "afterContentClassName", setter)]
    pub fn set_after_content_class_name(this: &IModelDecorationOptions, after_content_class_name: JsValue);

    /**
         * If set, text will be injected in the view after the range.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "after", getter)]
    pub fn after(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "after", setter)]
    pub fn set_after(this: &IModelDecorationOptions, after: JsValue);

    /**
         * If set, text will be injected in the view before the range.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "before", getter)]
    pub fn before(this: &IModelDecorationOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IModelDecorationOptions", js_name = "before", setter)]
    pub fn set_before(this: &IModelDecorationOptions, before: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InjectedTextOptions;

    /**
         * Sets the text to inject. Must be a single line.
         */ 
    #[wasm_bindgen(method, js_class = "InjectedTextOptions", js_name = "content", getter)]
    pub fn content(this: &InjectedTextOptions) -> String;
    /**
         * If set, the decoration will be rendered inline with the text with this CSS class name.
         */ 
    #[wasm_bindgen(method, js_class = "InjectedTextOptions", js_name = "inlineClassName", getter)]
    pub fn inline_class_name(this: &InjectedTextOptions) -> JsValue;
    /**
         * If there is an `inlineClassName` which affects letter spacing.
         */ 
    #[wasm_bindgen(method, js_class = "InjectedTextOptions", js_name = "inlineClassNameAffectsLetterSpacing", getter)]
    pub fn inline_class_name_affects_letter_spacing(this: &InjectedTextOptions) -> bool;
    /**
         * This field allows to attach data to this injected text.
         * The data can be read when injected texts at a given position are queried.
         */ 
    #[wasm_bindgen(method, js_class = "InjectedTextOptions", js_name = "attachedData", getter)]
    pub fn attached_data(this: &InjectedTextOptions) -> JsValue;
    /**
         * Configures cursor stops around injected text.
         * Defaults to {@link InjectedTextCursorStops.Both}.
        */ 
    #[wasm_bindgen(method, js_class = "InjectedTextOptions", js_name = "cursorStops", getter)]
    pub fn cursor_stops(this: &InjectedTextOptions) -> JsValue;

}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum InjectedTextCursorStops {
    Both = 0,
    Right = 1,
    Left = 2,
    None = 3,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelDeltaDecoration;

    /**
         * Range that this decoration covers.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDeltaDecoration", js_name = "range", getter)]
    pub fn range(this: &IModelDeltaDecoration) -> IRange;

    #[wasm_bindgen(method, js_class = "IModelDeltaDecoration", js_name = "range", setter)]
    pub fn set_range(this: &IModelDeltaDecoration, range: IRange);

    /**
         * Options associated with this decoration.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDeltaDecoration", js_name = "options", getter)]
    pub fn options(this: &IModelDeltaDecoration) -> IModelDecorationOptions;

    #[wasm_bindgen(method, js_class = "IModelDeltaDecoration", js_name = "options", setter)]
    pub fn set_options(this: &IModelDeltaDecoration, options: IModelDecorationOptions);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelDecoration;

    /**
         * Identifier for a decoration.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecoration", js_name = "id", getter)]
    pub fn id(this: &IModelDecoration) -> String;
    /**
         * Identifier for a decoration's owner.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecoration", js_name = "ownerId", getter)]
    pub fn owner_id(this: &IModelDecoration) -> f64;
    /**
         * Range that this decoration covers.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecoration", js_name = "range", getter)]
    pub fn range(this: &IModelDecoration) -> Range;
    /**
         * Options associated with this decoration.
         */ 
    #[wasm_bindgen(method, js_class = "IModelDecoration", js_name = "options", getter)]
    pub fn options(this: &IModelDecoration) -> IModelDecorationOptions;

}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum EndOfLinePreference {
    /**
         * Use the end of line character identified in the text buffer.
         */ 
    TextDefined = 0,
    /**
         * Use line feed (\n) as the end of line character.
         */ 
    LF = 1,
    /**
         * Use carriage return and line feed (\r\n) as the end of line character.
         */ 
    CRLF = 2,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum DefaultEndOfLine {
    /**
         * Use line feed (\n) as the end of line character.
         */ 
    LF = 1,
    /**
         * Use carriage return and line feed (\r\n) as the end of line character.
         */ 
    CRLF = 2,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum EndOfLineSequence {
    /**
         * Use line feed (\n) as the end of line character.
         */ 
    LF = 0,
    /**
         * Use carriage return and line feed (\r\n) as the end of line character.
         */ 
    CRLF = 1,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IIdentifiedSingleEditOperation;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IValidEditOperation;

    /**
         * The range to replace. This can be empty to emulate a simple insert.
         */ 
    #[wasm_bindgen(method, js_class = "IValidEditOperation", js_name = "range", getter)]
    pub fn range(this: &IValidEditOperation) -> Range;

    #[wasm_bindgen(method, js_class = "IValidEditOperation", js_name = "range", setter)]
    pub fn set_range(this: &IValidEditOperation, range: Range);

    /**
         * The text to replace with. This can be empty to emulate a simple delete.
         */ 
    #[wasm_bindgen(method, js_class = "IValidEditOperation", js_name = "text", getter)]
    pub fn text(this: &IValidEditOperation) -> String;

    #[wasm_bindgen(method, js_class = "IValidEditOperation", js_name = "text", setter)]
    pub fn set_text(this: &IValidEditOperation, text: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICursorStateComputer;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type TextModelResolvedOptions;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", getter = originalIndentSize, method)]
    pub fn original_indent_size(this: &TextModelResolvedOptions, ) -> JsValue;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type BracketPairColorizationOptions;

    #[wasm_bindgen(method, js_class = "BracketPairColorizationOptions", js_name = "enabled", getter)]
    pub fn enabled(this: &BracketPairColorizationOptions) -> bool;

    #[wasm_bindgen(method, js_class = "BracketPairColorizationOptions", js_name = "enabled", setter)]
    pub fn set_enabled(this: &BracketPairColorizationOptions, enabled: bool);

    #[wasm_bindgen(method, js_class = "BracketPairColorizationOptions", js_name = "independentColorPoolPerBracketType", getter)]
    pub fn independent_color_pool_per_bracket_type(this: &BracketPairColorizationOptions) -> bool;

    #[wasm_bindgen(method, js_class = "BracketPairColorizationOptions", js_name = "independentColorPoolPerBracketType", setter)]
    pub fn set_independent_color_pool_per_bracket_type(this: &BracketPairColorizationOptions, independent_color_pool_per_bracket_type: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ITextModelUpdateOptions;

    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "tabSize", getter)]
    pub fn tab_size(this: &ITextModelUpdateOptions) -> f64;

    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "tabSize", setter)]
    pub fn set_tab_size(this: &ITextModelUpdateOptions, tab_size: f64);

    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "indentSize", getter)]
    pub fn indent_size(this: &ITextModelUpdateOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "indentSize", setter)]
    pub fn set_indent_size(this: &ITextModelUpdateOptions, indent_size: JsValue);

    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "insertSpaces", getter)]
    pub fn insert_spaces(this: &ITextModelUpdateOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "insertSpaces", setter)]
    pub fn set_insert_spaces(this: &ITextModelUpdateOptions, insert_spaces: bool);

    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "trimAutoWhitespace", getter)]
    pub fn trim_auto_whitespace(this: &ITextModelUpdateOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "trimAutoWhitespace", setter)]
    pub fn set_trim_auto_whitespace(this: &ITextModelUpdateOptions, trim_auto_whitespace: bool);

    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "bracketColorizationOptions", getter)]
    pub fn bracket_colorization_options(this: &ITextModelUpdateOptions) -> BracketPairColorizationOptions;

    #[wasm_bindgen(method, js_class = "ITextModelUpdateOptions", js_name = "bracketColorizationOptions", setter)]
    pub fn set_bracket_colorization_options(this: &ITextModelUpdateOptions, bracket_colorization_options: BracketPairColorizationOptions);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type FindMatch;


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum TrackedRangeStickiness {
    AlwaysGrowsWhenTypingAtEdges = 0,
    NeverGrowsWhenTypingAtEdges = 1,
    GrowsOnlyWhenTypingBefore = 2,
    GrowsOnlyWhenTypingAfter = 3,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ITextSnapshot;

    #[wasm_bindgen(method, js_class = "ITextSnapshot", js_name = "read")]
    pub fn read(this: &ITextSnapshot, ) -> JsValue;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ITextModel;

    /**
         * Gets the resource associated with this editor model.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "uri", getter)]
    pub fn uri(this: &ITextModel) -> Uri;
    /**
         * A unique identifier associated with this model.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "id", getter)]
    pub fn id(this: &ITextModel) -> String;
    /**
         * Get the resolved options for this model.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getOptions")]
    pub fn get_options(this: &ITextModel, ) -> TextModelResolvedOptions;

    /**
         * Get the current version id of the model.
         * Anytime a change happens to the model (even undo/redo),
         * the version id is incremented.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getVersionId")]
    pub fn get_version_id(this: &ITextModel, ) -> f64;

    /**
         * Get the alternative version id of the model.
         * This alternative version id is not always incremented,
         * it will return the same values in the case of undo-redo.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getAlternativeVersionId")]
    pub fn get_alternative_version_id(this: &ITextModel, ) -> f64;

    /**
         * Replace the entire text buffer value contained in this model.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "setValue")]
    pub fn set_value(this: &ITextModel, new_value: JsValue, );

    /**
         * Get the text stored in this model.
         * @param eol The end of line character preference. Defaults to `EndOfLinePreference.TextDefined`.
         * @param preserverBOM Preserve a BOM character if it was detected when the model was constructed.
         * @return The text.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getValue")]
    pub fn get_value(this: &ITextModel, eol: Option<EndOfLinePreference>, preserve_bom: Option<bool>, ) -> String;

    /**
         * Get the text stored in this model.
         * @param preserverBOM Preserve a BOM character if it was detected when the model was constructed.
         * @return The text snapshot (it is safe to consume it asynchronously).
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "createSnapshot")]
    pub fn create_snapshot(this: &ITextModel, preserve_bom: Option<bool>, ) -> ITextSnapshot;

    /**
         * Get the length of the text stored in this model.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getValueLength")]
    pub fn get_value_length(this: &ITextModel, eol: Option<EndOfLinePreference>, preserve_bom: Option<bool>, ) -> f64;

    /**
         * Get the text in a certain range.
         * @param range The range describing what text to get.
         * @param eol The end of line character preference. This will only be used for multiline ranges. Defaults to `EndOfLinePreference.TextDefined`.
         * @return The text.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getValueInRange")]
    pub fn get_value_in_range(this: &ITextModel, range: IRange, eol: Option<EndOfLinePreference>, ) -> String;

    /**
         * Get the length of text in a certain range.
         * @param range The range describing what text length to get.
         * @return The text length.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getValueLengthInRange")]
    pub fn get_value_length_in_range(this: &ITextModel, range: IRange, eol: Option<EndOfLinePreference>, ) -> f64;

    /**
         * Get the character count of text in a certain range.
         * @param range The range describing what text length to get.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getCharacterCountInRange")]
    pub fn get_character_count_in_range(this: &ITextModel, range: IRange, eol: Option<EndOfLinePreference>, ) -> f64;

    /**
         * Get the number of lines in the model.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineCount")]
    pub fn get_line_count(this: &ITextModel, ) -> f64;

    /**
         * Get the text for a certain line.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineContent")]
    pub fn get_line_content(this: &ITextModel, line_number: f64, ) -> String;

    /**
         * Get the text length for a certain line.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineLength")]
    pub fn get_line_length(this: &ITextModel, line_number: f64, ) -> f64;

    /**
         * Get the text for all lines.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLinesContent")]
    pub fn get_lines_content(this: &ITextModel, ) -> js_sys::Array;

    /**
         * Get the end of line sequence predominantly used in the text buffer.
         * @return EOL char sequence (e.g.: '\n' or '\r\n').
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getEOL")]
    pub fn get_eol(this: &ITextModel, ) -> String;

    /**
         * Get the end of line sequence predominantly used in the text buffer.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getEndOfLineSequence")]
    pub fn get_end_of_line_sequence(this: &ITextModel, ) -> EndOfLineSequence;

    /**
         * Get the minimum legal column for line at `lineNumber`
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineMinColumn")]
    pub fn get_line_min_column(this: &ITextModel, line_number: f64, ) -> f64;

    /**
         * Get the maximum legal column for line at `lineNumber`
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineMaxColumn")]
    pub fn get_line_max_column(this: &ITextModel, line_number: f64, ) -> f64;

    /**
         * Returns the column before the first non whitespace character for line at `lineNumber`.
         * Returns 0 if line is empty or contains only whitespace.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineFirstNonWhitespaceColumn")]
    pub fn get_line_first_non_whitespace_column(this: &ITextModel, line_number: f64, ) -> f64;

    /**
         * Returns the column after the last non whitespace character for line at `lineNumber`.
         * Returns 0 if line is empty or contains only whitespace.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineLastNonWhitespaceColumn")]
    pub fn get_line_last_non_whitespace_column(this: &ITextModel, line_number: f64, ) -> f64;

    /**
         * Create a valid position.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "validatePosition")]
    pub fn validate_position(this: &ITextModel, position: IPosition, ) -> Position;

    /**
         * Advances the given position by the given offset (negative offsets are also accepted)
         * and returns it as a new valid position.
         *
         * If the offset and position are such that their combination goes beyond the beginning or
         * end of the model, throws an exception.
         *
         * If the offset is such that the new position would be in the middle of a multi-byte
         * line terminator, throws an exception.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "modifyPosition")]
    pub fn modify_position(this: &ITextModel, position: IPosition, offset: f64, ) -> Position;

    /**
         * Create a valid range.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "validateRange")]
    pub fn validate_range(this: &ITextModel, range: IRange, ) -> Range;

    /**
         * Converts the position to a zero-based offset.
         *
         * The position will be [adjusted](#TextDocument.validatePosition).
         *
         * @param position A position.
         * @return A valid zero-based offset.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getOffsetAt")]
    pub fn get_offset_at(this: &ITextModel, position: IPosition, ) -> f64;

    /**
         * Converts a zero-based offset to a position.
         *
         * @param offset A zero-based offset.
         * @return A valid [position](#Position).
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getPositionAt")]
    pub fn get_position_at(this: &ITextModel, offset: f64, ) -> Position;

    /**
         * Get a range covering the entire model.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getFullModelRange")]
    pub fn get_full_model_range(this: &ITextModel, ) -> Range;

    /**
         * Returns if the model was disposed or not.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "isDisposed")]
    pub fn is_disposed(this: &ITextModel, ) -> bool;

    /**
         * Search the model.
         * @param searchString The string used to search. If it is a regular expression, set `isRegex` to true.
         * @param searchOnlyEditableRange Limit the searching to only search inside the editable range of the model.
         * @param isRegex Used to indicate that `searchString` is a regular expression.
         * @param matchCase Force the matching to match lower/upper case exactly.
         * @param wordSeparators Force the matching to match entire words only. Pass null otherwise.
         * @param captureMatches The result will contain the captured groups.
         * @param limitResultCount Limit the number of results
         * @return The ranges where the matches are. It is empty if not matches have been found.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "findMatches")]
    pub fn find_matches(this: &ITextModel, search_string: String, search_only_editable_range: bool, is_regex: bool, match_case: bool, word_separators: JsValue, capture_matches: bool, limit_result_count: Option<f64>, ) -> js_sys::Array;

    /**
         * Search the model.
         * @param searchString The string used to search. If it is a regular expression, set `isRegex` to true.
         * @param searchScope Limit the searching to only search inside these ranges.
         * @param isRegex Used to indicate that `searchString` is a regular expression.
         * @param matchCase Force the matching to match lower/upper case exactly.
         * @param wordSeparators Force the matching to match entire words only. Pass null otherwise.
         * @param captureMatches The result will contain the captured groups.
         * @param limitResultCount Limit the number of results
         * @return The ranges where the matches are. It is empty if no matches have been found.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "findMatches")]
    pub fn find_matches_1(this: &ITextModel, search_string: String, search_scope: JsValue, is_regex: bool, match_case: bool, word_separators: JsValue, capture_matches: bool, limit_result_count: Option<f64>, ) -> js_sys::Array;

    /**
         * Search the model for the next match. Loops to the beginning of the model if needed.
         * @param searchString The string used to search. If it is a regular expression, set `isRegex` to true.
         * @param searchStart Start the searching at the specified position.
         * @param isRegex Used to indicate that `searchString` is a regular expression.
         * @param matchCase Force the matching to match lower/upper case exactly.
         * @param wordSeparators Force the matching to match entire words only. Pass null otherwise.
         * @param captureMatches The result will contain the captured groups.
         * @return The range where the next match is. It is null if no next match has been found.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "findNextMatch")]
    pub fn find_next_match(this: &ITextModel, search_string: String, search_start: IPosition, is_regex: bool, match_case: bool, word_separators: JsValue, capture_matches: bool, ) -> JsValue;

    /**
         * Search the model for the previous match. Loops to the end of the model if needed.
         * @param searchString The string used to search. If it is a regular expression, set `isRegex` to true.
         * @param searchStart Start the searching at the specified position.
         * @param isRegex Used to indicate that `searchString` is a regular expression.
         * @param matchCase Force the matching to match lower/upper case exactly.
         * @param wordSeparators Force the matching to match entire words only. Pass null otherwise.
         * @param captureMatches The result will contain the captured groups.
         * @return The range where the previous match is. It is null if no previous match has been found.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "findPreviousMatch")]
    pub fn find_previous_match(this: &ITextModel, search_string: String, search_start: IPosition, is_regex: bool, match_case: bool, word_separators: JsValue, capture_matches: bool, ) -> JsValue;

    /**
         * Get the language associated with this model.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLanguageId")]
    pub fn get_language_id(this: &ITextModel, ) -> String;

    /**
         * Get the word under or besides `position`.
         * @param position The position to look for a word.
         * @return The word under or besides `position`. Might be null.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getWordAtPosition")]
    pub fn get_word_at_position(this: &ITextModel, position: IPosition, ) -> JsValue;

    /**
         * Get the word under or besides `position` trimmed to `position`.column
         * @param position The position to look for a word.
         * @return The word under or besides `position`. Will never be null.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getWordUntilPosition")]
    pub fn get_word_until_position(this: &ITextModel, position: IPosition, ) -> IWordAtPosition;

    /**
         * Perform a minimum amount of operations, in order to transform the decorations
         * identified by `oldDecorations` to the decorations described by `newDecorations`
         * and returns the new identifiers associated with the resulting decorations.
         *
         * @param oldDecorations Array containing previous decorations identifiers.
         * @param newDecorations Array describing what decorations should result after the call.
         * @param ownerId Identifies the editor id in which these decorations should appear. If no `ownerId` is provided, the decorations will appear in all editors that attach this model.
         * @return An array containing the new decorations identifiers.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "deltaDecorations")]
    pub fn delta_decorations(this: &ITextModel, old_decorations: js_sys::Array, new_decorations: js_sys::Array, owner_id: Option<f64>, ) -> js_sys::Array;

    /**
         * Get the options associated with a decoration.
         * @param id The decoration id.
         * @return The decoration options or null if the decoration was not found.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getDecorationOptions")]
    pub fn get_decoration_options(this: &ITextModel, id: String, ) -> JsValue;

    /**
         * Get the range associated with a decoration.
         * @param id The decoration id.
         * @return The decoration range or null if the decoration was not found.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getDecorationRange")]
    pub fn get_decoration_range(this: &ITextModel, id: String, ) -> JsValue;

    /**
         * Gets all the decorations for the line `lineNumber` as an array.
         * @param lineNumber The line number
         * @param ownerId If set, it will ignore decorations belonging to other owners.
         * @param filterOutValidation If set, it will ignore decorations specific to validation (i.e. warnings, errors).
         * @return An array with the decorations
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLineDecorations")]
    pub fn get_line_decorations(this: &ITextModel, line_number: f64, owner_id: Option<f64>, filter_out_validation: Option<bool>, ) -> js_sys::Array;

    /**
         * Gets all the decorations for the lines between `startLineNumber` and `endLineNumber` as an array.
         * @param startLineNumber The start line number
         * @param endLineNumber The end line number
         * @param ownerId If set, it will ignore decorations belonging to other owners.
         * @param filterOutValidation If set, it will ignore decorations specific to validation (i.e. warnings, errors).
         * @return An array with the decorations
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getLinesDecorations")]
    pub fn get_lines_decorations(this: &ITextModel, start_line_number: f64, end_line_number: f64, owner_id: Option<f64>, filter_out_validation: Option<bool>, ) -> js_sys::Array;

    /**
         * Gets all the decorations in a range as an array. Only `startLineNumber` and `endLineNumber` from `range` are used for filtering.
         * So for now it returns all the decorations on the same line as `range`.
         * @param range The range to search in
         * @param ownerId If set, it will ignore decorations belonging to other owners.
         * @param filterOutValidation If set, it will ignore decorations specific to validation (i.e. warnings, errors).
         * @param onlyMinimapDecorations If set, it will return only decorations that render in the minimap.
         * @param onlyMarginDecorations If set, it will return only decorations that render in the glyph margin.
         * @return An array with the decorations
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getDecorationsInRange")]
    pub fn get_decorations_in_range(this: &ITextModel, range: IRange, owner_id: Option<f64>, filter_out_validation: Option<bool>, only_minimap_decorations: Option<bool>, only_margin_decorations: Option<bool>, ) -> js_sys::Array;

    /**
         * Gets all the decorations as an array.
         * @param ownerId If set, it will ignore decorations belonging to other owners.
         * @param filterOutValidation If set, it will ignore decorations specific to validation (i.e. warnings, errors).
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getAllDecorations")]
    pub fn get_all_decorations(this: &ITextModel, owner_id: Option<f64>, filter_out_validation: Option<bool>, ) -> js_sys::Array;

    /**
         * Gets all decorations that render in the glyph margin as an array.
         * @param ownerId If set, it will ignore decorations belonging to other owners.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getAllMarginDecorations")]
    pub fn get_all_margin_decorations(this: &ITextModel, owner_id: Option<f64>, ) -> js_sys::Array;

    /**
         * Gets all the decorations that should be rendered in the overview ruler as an array.
         * @param ownerId If set, it will ignore decorations belonging to other owners.
         * @param filterOutValidation If set, it will ignore decorations specific to validation (i.e. warnings, errors).
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getOverviewRulerDecorations")]
    pub fn get_overview_ruler_decorations(this: &ITextModel, owner_id: Option<f64>, filter_out_validation: Option<bool>, ) -> js_sys::Array;

    /**
         * Gets all the decorations that contain injected text.
         * @param ownerId If set, it will ignore decorations belonging to other owners.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "getInjectedTextDecorations")]
    pub fn get_injected_text_decorations(this: &ITextModel, owner_id: Option<f64>, ) -> js_sys::Array;

    /**
         * Normalize a string containing whitespace according to indentation rules (converts to spaces or to tabs).
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "normalizeIndentation")]
    pub fn normalize_indentation(this: &ITextModel, str: String, ) -> String;

    /**
         * Change the options of this model.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "updateOptions")]
    pub fn update_options(this: &ITextModel, new_opts: ITextModelUpdateOptions, );

    /**
         * Detect the indentation options for this model from its content.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "detectIndentation")]
    pub fn detect_indentation(this: &ITextModel, default_insert_spaces: bool, default_tab_size: f64, );

    /**
         * Close the current undo-redo element.
         * This offers a way to create an undo/redo stop point.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "pushStackElement")]
    pub fn push_stack_element(this: &ITextModel, );

    /**
         * Open the current undo-redo element.
         * This offers a way to remove the current undo/redo stop point.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "popStackElement")]
    pub fn pop_stack_element(this: &ITextModel, );

    /**
         * Push edit operations, basically editing the model. This is the preferred way
         * of editing the model. The edit operations will land on the undo stack.
         * @param beforeCursorState The cursor state before the edit operations. This cursor state will be returned when `undo` or `redo` are invoked.
         * @param editOperations The edit operations.
         * @param cursorStateComputer A callback that can compute the resulting cursors state after the edit operations have been executed.
         * @return The cursor state returned by the `cursorStateComputer`.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "pushEditOperations")]
    pub fn push_edit_operations(this: &ITextModel, before_cursor_state: JsValue, edit_operations: js_sys::Array, cursor_state_computer: ICursorStateComputer, ) -> JsValue;

    /**
         * Change the end of line sequence. This is the preferred way of
         * changing the eol sequence. This will land on the undo stack.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "pushEOL")]
    pub fn push_eol(this: &ITextModel, eol: EndOfLineSequence, );

    /**
         * Edit the model without adding the edits to the undo stack.
         * This can have dire consequences on the undo stack! See @pushEditOperations for the preferred way.
         * @param operations The edit operations.
         * @return If desired, the inverse edit operations, that, when applied, will bring the model back to the previous state.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "applyEdits")]
    pub fn apply_edits(this: &ITextModel, operations: js_sys::Array, );

    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "applyEdits")]
    pub fn apply_edits_1(this: &ITextModel, operations: js_sys::Array, compute_undo_edits: JsValue, );

    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "applyEdits")]
    pub fn apply_edits_2(this: &ITextModel, operations: js_sys::Array, compute_undo_edits: JsValue, ) -> js_sys::Array;

    /**
         * Change the end of line sequence without recording in the undo stack.
         * This can have dire consequences on the undo stack! See @pushEOL for the preferred way.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "setEOL")]
    pub fn set_eol(this: &ITextModel, eol: EndOfLineSequence, );

    /**
         * An event emitted when the contents of the model have changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onDidChangeContent")]
    pub fn on_did_change_content(this: &ITextModel, listener: JsValue, ) -> IDisposable;

    /**
         * An event emitted when decorations of the model have changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onDidChangeDecorations", getter)]
    pub fn on_did_change_decorations(this: &ITextModel) -> IEvent;
    /**
         * An event emitted when the model options have changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onDidChangeOptions", getter)]
    pub fn on_did_change_options(this: &ITextModel) -> IEvent;
    /**
         * An event emitted when the language associated with the model has changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onDidChangeLanguage", getter)]
    pub fn on_did_change_language(this: &ITextModel) -> IEvent;
    /**
         * An event emitted when the language configuration associated with the model has changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onDidChangeLanguageConfiguration", getter)]
    pub fn on_did_change_language_configuration(this: &ITextModel) -> IEvent;
    /**
         * An event emitted when the model has been attached to the first editor or detached from the last editor.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onDidChangeAttached", getter)]
    pub fn on_did_change_attached(this: &ITextModel) -> IEvent;
    /**
         * An event emitted right before disposing the model.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "onWillDispose", getter)]
    pub fn on_will_dispose(this: &ITextModel) -> IEvent;
    /**
         * Destroy this model.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "dispose")]
    pub fn dispose(this: &ITextModel, );

    /**
         * Returns if this model is attached to an editor or not.
         */ 
    #[wasm_bindgen(method, js_class = "ITextModel", js_name = "isAttachedToEditor")]
    pub fn is_attached_to_editor(this: &ITextModel, ) -> bool;


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum PositionAffinity {
    /**
         * Prefers the left most position.
        */ 
    Left = 0,
    /**
         * Prefers the right most position.
        */ 
    Right = 1,
    /**
         * No preference.
        */ 
    None = 2,
    /**
         * If the given position is on injected text, prefers the position left of it.
        */ 
    LeftOfInjectedText = 3,
    /**
         * If the given position is on injected text, prefers the position right of it.
        */ 
    RightOfInjectedText = 4,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IChange;

    #[wasm_bindgen(method, js_class = "IChange", js_name = "originalStartLineNumber", getter)]
    pub fn original_start_line_number(this: &IChange) -> f64;
    #[wasm_bindgen(method, js_class = "IChange", js_name = "originalEndLineNumber", getter)]
    pub fn original_end_line_number(this: &IChange) -> f64;
    #[wasm_bindgen(method, js_class = "IChange", js_name = "modifiedStartLineNumber", getter)]
    pub fn modified_start_line_number(this: &IChange) -> f64;
    #[wasm_bindgen(method, js_class = "IChange", js_name = "modifiedEndLineNumber", getter)]
    pub fn modified_end_line_number(this: &IChange) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICharChange;

    #[wasm_bindgen(method, js_class = "ICharChange", js_name = "originalStartColumn", getter)]
    pub fn original_start_column(this: &ICharChange) -> f64;
    #[wasm_bindgen(method, js_class = "ICharChange", js_name = "originalEndColumn", getter)]
    pub fn original_end_column(this: &ICharChange) -> f64;
    #[wasm_bindgen(method, js_class = "ICharChange", js_name = "modifiedStartColumn", getter)]
    pub fn modified_start_column(this: &ICharChange) -> f64;
    #[wasm_bindgen(method, js_class = "ICharChange", js_name = "modifiedEndColumn", getter)]
    pub fn modified_end_column(this: &ICharChange) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ILineChange;

    #[wasm_bindgen(method, js_class = "ILineChange", js_name = "charChanges", getter)]
    pub fn char_changes(this: &ILineChange) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDocumentDiffProvider;

    /**
         * Computes the diff between the text models `original` and `modified`.
         */ 
    #[wasm_bindgen(method, js_class = "IDocumentDiffProvider", js_name = "computeDiff")]
    pub fn compute_diff(this: &IDocumentDiffProvider, original: ITextModel, modified: ITextModel, options: IDocumentDiffProviderOptions, ) -> Promise;

    /**
         * Is fired when settings of the diff algorithm change that could alter the result of the diffing computation.
         * Any user of this provider should recompute the diff when this event is fired.
         */ 
    #[wasm_bindgen(method, js_class = "IDocumentDiffProvider", js_name = "onDidChange", getter)]
    pub fn on_did_change(this: &IDocumentDiffProvider) -> IEvent;

    #[wasm_bindgen(method, js_class = "IDocumentDiffProvider", js_name = "onDidChange", setter)]
    pub fn set_on_did_change(this: &IDocumentDiffProvider, on_did_change: IEvent);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDocumentDiffProviderOptions;

    /**
         * When set to true, the diff should ignore whitespace changes.
         */ 
    #[wasm_bindgen(method, js_class = "IDocumentDiffProviderOptions", js_name = "ignoreTrimWhitespace", getter)]
    pub fn ignore_trim_whitespace(this: &IDocumentDiffProviderOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDocumentDiffProviderOptions", js_name = "ignoreTrimWhitespace", setter)]
    pub fn set_ignore_trim_whitespace(this: &IDocumentDiffProviderOptions, ignore_trim_whitespace: bool);

    /**
         * A diff computation should throw if it takes longer than this value.
         */ 
    #[wasm_bindgen(method, js_class = "IDocumentDiffProviderOptions", js_name = "maxComputationTimeMs", getter)]
    pub fn max_computation_time_ms(this: &IDocumentDiffProviderOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IDocumentDiffProviderOptions", js_name = "maxComputationTimeMs", setter)]
    pub fn set_max_computation_time_ms(this: &IDocumentDiffProviderOptions, max_computation_time_ms: f64);

    /**
         * If set, the diff computation should compute moves in addition to insertions and deletions.
         */ 
    #[wasm_bindgen(method, js_class = "IDocumentDiffProviderOptions", js_name = "computeMoves", getter)]
    pub fn compute_moves(this: &IDocumentDiffProviderOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDocumentDiffProviderOptions", js_name = "computeMoves", setter)]
    pub fn set_compute_moves(this: &IDocumentDiffProviderOptions, compute_moves: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDocumentDiff;

    /**
         * If true, both text models are identical (byte-wise).
         */ 
    #[wasm_bindgen(method, js_class = "IDocumentDiff", js_name = "identical", getter)]
    pub fn identical(this: &IDocumentDiff) -> bool;
    /**
         * If true, the diff computation timed out and the diff might not be accurate.
         */ 
    #[wasm_bindgen(method, js_class = "IDocumentDiff", js_name = "quitEarly", getter)]
    pub fn quit_early(this: &IDocumentDiff) -> bool;
    /**
         * Maps all modified line ranges in the original to the corresponding line ranges in the modified text model.
         */ 
    #[wasm_bindgen(method, js_class = "IDocumentDiff", js_name = "changes", getter)]
    pub fn changes(this: &IDocumentDiff) -> JsValue;
    /**
         * Sorted by original line ranges.
         * The original line ranges and the modified line ranges must be disjoint (but can be touching).
         */ 
    #[wasm_bindgen(method, js_class = "IDocumentDiff", js_name = "moves", getter)]
    pub fn moves(this: &IDocumentDiff) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LineRange;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = LineRange)]
    pub fn from_range(range: Range, ) -> LineRange;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = LineRange)]
    pub fn subtract(a: LineRange, b: JsValue, ) -> js_sys::Array;

    /**
         * @param lineRanges An array of sorted line ranges.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = LineRange)]
    pub fn join_many(line_ranges: JsValue, ) -> JsValue;

    /**
         * @param lineRanges1 Must be sorted.
         * @param lineRanges2 Must be sorted.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = LineRange)]
    pub fn join(line_ranges_1: JsValue, line_ranges_2: JsValue, ) -> JsValue;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = LineRange)]
    pub fn of_length(start_line_number: f64, length: f64, ) -> LineRange;

  #[wasm_bindgen(constructor, js_class = "LineRange")]
  pub fn new() -> LineRange;

    /**
         * Indicates if this line range contains the given line number.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn contains(this: &LineRange, line_number: f64, ) -> bool;

    /**
         * Indicates if this line range is empty.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", getter = isEmpty, method)]
    pub fn is_empty(this: &LineRange, ) -> bool;

    /**
         * Moves this line range by the given offset of line numbers.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn delta(this: &LineRange, offset: f64, ) -> LineRange;

    /**
         * The number of lines this line range spans.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", getter = length, method)]
    pub fn length(this: &LineRange, ) -> f64;

    /**
         * Creates a line range that combines this and the given line range.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn join_1(this: &LineRange, other: LineRange, ) -> LineRange;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_string(this: &LineRange, ) -> String;

    /**
         * The resulting range is empty if the ranges do not intersect, but touch.
         * If the ranges don't even touch, the result is undefined.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn intersect(this: &LineRange, other: LineRange, ) -> JsValue;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn intersects_strict(this: &LineRange, other: LineRange, ) -> bool;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn overlap_or_touch(this: &LineRange, other: LineRange, ) -> bool;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn equals(this: &LineRange, b: LineRange, ) -> bool;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_inclusive_range(this: &LineRange, ) -> JsValue;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_exclusive_range(this: &LineRange, ) -> Range;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn map_to_line_array(this: &LineRange, f: JsValue, ) -> js_sys::Array;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn for_each(this: &LineRange, f: JsValue, );


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn includes(this: &LineRange, line_number: f64, ) -> bool;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LineRangeMapping;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = LineRangeMapping)]
    pub fn inverse(mapping: JsValue, original_line_count: f64, modified_line_count: f64, ) -> js_sys::Array;

  #[wasm_bindgen(constructor, js_class = "LineRangeMapping")]
  pub fn new() -> LineRangeMapping;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_string(this: &LineRangeMapping, ) -> String;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", getter = changedLineCount, method)]
    pub fn changed_line_count(this: &LineRangeMapping, ) -> JsValue;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn flip(this: &LineRangeMapping, ) -> LineRangeMapping;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type RangeMapping;

  #[wasm_bindgen(constructor, js_class = "RangeMapping")]
  pub fn new() -> RangeMapping;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_string(this: &RangeMapping, ) -> String;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn flip(this: &RangeMapping, ) -> RangeMapping;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type MovedText;

  #[wasm_bindgen(constructor, js_class = "MovedText")]
  pub fn new() -> MovedText;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn flip(this: &MovedText, ) -> MovedText;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SimpleLineRangeMapping;

  #[wasm_bindgen(constructor, js_class = "SimpleLineRangeMapping")]
  pub fn new() -> SimpleLineRangeMapping;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn to_string(this: &SimpleLineRangeMapping, ) -> String;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn flip(this: &SimpleLineRangeMapping, ) -> SimpleLineRangeMapping;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDimension;

    #[wasm_bindgen(method, js_class = "IDimension", js_name = "width", getter)]
    pub fn width(this: &IDimension) -> f64;

    #[wasm_bindgen(method, js_class = "IDimension", js_name = "width", setter)]
    pub fn set_width(this: &IDimension, width: f64);

    #[wasm_bindgen(method, js_class = "IDimension", js_name = "height", getter)]
    pub fn height(this: &IDimension) -> f64;

    #[wasm_bindgen(method, js_class = "IDimension", js_name = "height", setter)]
    pub fn set_height(this: &IDimension, height: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditOperationBuilder;

    /**
         * Add a new edit operation (a replace operation).
         * @param range The range to replace (delete). May be empty to represent a simple insert.
         * @param text The text to replace with. May be null to represent a simple delete.
         */ 
    #[wasm_bindgen(method, js_class = "IEditOperationBuilder", js_name = "addEditOperation")]
    pub fn add_edit_operation(this: &IEditOperationBuilder, range: IRange, text: JsValue, force_move_markers: Option<bool>, );

    /**
         * Add a new edit operation (a replace operation).
         * The inverse edits will be accessible in `ICursorStateComputerData.getInverseEditOperations()`
         * @param range The range to replace (delete). May be empty to represent a simple insert.
         * @param text The text to replace with. May be null to represent a simple delete.
         */ 
    #[wasm_bindgen(method, js_class = "IEditOperationBuilder", js_name = "addTrackedEditOperation")]
    pub fn add_tracked_edit_operation(this: &IEditOperationBuilder, range: IRange, text: JsValue, force_move_markers: Option<bool>, );

    /**
         * Track `selection` when applying edit operations.
         * A best effort will be made to not grow/expand the selection.
         * An empty selection will clamp to a nearby character.
         * @param selection The selection to track.
         * @param trackPreviousOnEmpty If set, and the selection is empty, indicates whether the selection
         *           should clamp to the previous or the next character.
         * @return A unique identifier.
         */ 
    #[wasm_bindgen(method, js_class = "IEditOperationBuilder", js_name = "trackSelection")]
    pub fn track_selection(this: &IEditOperationBuilder, selection: Selection, track_previous_on_empty: Option<bool>, ) -> String;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICursorStateComputerData;

    /**
         * Get the inverse edit operations of the added edit operations.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorStateComputerData", js_name = "getInverseEditOperations")]
    pub fn get_inverse_edit_operations(this: &ICursorStateComputerData, ) -> js_sys::Array;

    /**
         * Get a previously tracked selection.
         * @param id The unique identifier returned by `trackSelection`.
         * @return The selection.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorStateComputerData", js_name = "getTrackedSelection")]
    pub fn get_tracked_selection(this: &ICursorStateComputerData, id: String, ) -> Selection;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICommand;

    /**
         * Get the edit operations needed to execute this command.
         * @param model The model the command will execute on.
         * @param builder A helper to collect the needed edit operations and to track selections.
         */ 
    #[wasm_bindgen(method, js_class = "ICommand", js_name = "getEditOperations")]
    pub fn get_edit_operations(this: &ICommand, model: ITextModel, builder: IEditOperationBuilder, );

    /**
         * Compute the cursor state after the edit operations were applied.
         * @param model The model the command has executed on.
         * @param helper A helper to get inverse edit operations and to get previously tracked selections.
         * @return The cursor state after the command executed.
         */ 
    #[wasm_bindgen(method, js_class = "ICommand", js_name = "computeCursorState")]
    pub fn compute_cursor_state(this: &ICommand, model: ITextModel, helper: ICursorStateComputerData, ) -> Selection;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDiffEditorModel;

    /**
         * Original model.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorModel", js_name = "original", getter)]
    pub fn original(this: &IDiffEditorModel) -> ITextModel;

    #[wasm_bindgen(method, js_class = "IDiffEditorModel", js_name = "original", setter)]
    pub fn set_original(this: &IDiffEditorModel, original: ITextModel);

    /**
         * Modified model.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorModel", js_name = "modified", getter)]
    pub fn modified(this: &IDiffEditorModel) -> ITextModel;

    #[wasm_bindgen(method, js_class = "IDiffEditorModel", js_name = "modified", setter)]
    pub fn set_modified(this: &IDiffEditorModel, modified: ITextModel);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDiffEditorViewModel;

    #[wasm_bindgen(method, js_class = "IDiffEditorViewModel", js_name = "model", getter)]
    pub fn model(this: &IDiffEditorViewModel) -> IDiffEditorModel;
    #[wasm_bindgen(method, js_class = "IDiffEditorViewModel", js_name = "waitForDiff")]
    pub fn wait_for_diff(this: &IDiffEditorViewModel, ) -> Promise;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelChangedEvent;

    /**
         * The `uri` of the previous model or null.
         */ 
    #[wasm_bindgen(method, js_class = "IModelChangedEvent", js_name = "oldModelUrl", getter)]
    pub fn old_model_url(this: &IModelChangedEvent) -> JsValue;
    /**
         * The `uri` of the new model or null.
         */ 
    #[wasm_bindgen(method, js_class = "IModelChangedEvent", js_name = "newModelUrl", getter)]
    pub fn new_model_url(this: &IModelChangedEvent) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IContentSizeChangedEvent;

    #[wasm_bindgen(method, js_class = "IContentSizeChangedEvent", js_name = "contentWidth", getter)]
    pub fn content_width(this: &IContentSizeChangedEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IContentSizeChangedEvent", js_name = "contentHeight", getter)]
    pub fn content_height(this: &IContentSizeChangedEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IContentSizeChangedEvent", js_name = "contentWidthChanged", getter)]
    pub fn content_width_changed(this: &IContentSizeChangedEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IContentSizeChangedEvent", js_name = "contentHeightChanged", getter)]
    pub fn content_height_changed(this: &IContentSizeChangedEvent) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type INewScrollPosition;

    #[wasm_bindgen(method, js_class = "INewScrollPosition", js_name = "scrollLeft", getter)]
    pub fn scroll_left(this: &INewScrollPosition) -> f64;

    #[wasm_bindgen(method, js_class = "INewScrollPosition", js_name = "scrollLeft", setter)]
    pub fn set_scroll_left(this: &INewScrollPosition, scroll_left: f64);

    #[wasm_bindgen(method, js_class = "INewScrollPosition", js_name = "scrollTop", getter)]
    pub fn scroll_top(this: &INewScrollPosition) -> f64;

    #[wasm_bindgen(method, js_class = "INewScrollPosition", js_name = "scrollTop", setter)]
    pub fn set_scroll_top(this: &INewScrollPosition, scroll_top: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorAction;

    #[wasm_bindgen(method, js_class = "IEditorAction", js_name = "id", getter)]
    pub fn id(this: &IEditorAction) -> String;
    #[wasm_bindgen(method, js_class = "IEditorAction", js_name = "label", getter)]
    pub fn label(this: &IEditorAction) -> String;
    #[wasm_bindgen(method, js_class = "IEditorAction", js_name = "alias", getter)]
    pub fn alias(this: &IEditorAction) -> String;
    #[wasm_bindgen(method, js_class = "IEditorAction", js_name = "isSupported")]
    pub fn is_supported(this: &IEditorAction, ) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorAction", js_name = "run")]
    pub fn run(this: &IEditorAction, args: JsValue, ) -> Promise;


}

pub type IEditorModel = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICursorState;

    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "inSelectionMode", getter)]
    pub fn in_selection_mode(this: &ICursorState) -> bool;

    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "inSelectionMode", setter)]
    pub fn set_in_selection_mode(this: &ICursorState, in_selection_mode: bool);

    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "selectionStart", getter)]
    pub fn selection_start(this: &ICursorState) -> IPosition;

    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "selectionStart", setter)]
    pub fn set_selection_start(this: &ICursorState, selection_start: IPosition);

    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "position", getter)]
    pub fn position(this: &ICursorState) -> IPosition;

    #[wasm_bindgen(method, js_class = "ICursorState", js_name = "position", setter)]
    pub fn set_position(this: &ICursorState, position: IPosition);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IViewState;

    /** written by previous versions */ 
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollTop", getter)]
    pub fn scroll_top(this: &IViewState) -> f64;

    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollTop", setter)]
    pub fn set_scroll_top(this: &IViewState, scroll_top: f64);

    /** written by previous versions */ 
    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollTopWithoutViewZones", getter)]
    pub fn scroll_top_without_view_zones(this: &IViewState) -> f64;

    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollTopWithoutViewZones", setter)]
    pub fn set_scroll_top_without_view_zones(this: &IViewState, scroll_top_without_view_zones: f64);

    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollLeft", getter)]
    pub fn scroll_left(this: &IViewState) -> f64;

    #[wasm_bindgen(method, js_class = "IViewState", js_name = "scrollLeft", setter)]
    pub fn set_scroll_left(this: &IViewState, scroll_left: f64);

    #[wasm_bindgen(method, js_class = "IViewState", js_name = "firstPosition", getter)]
    pub fn first_position(this: &IViewState) -> IPosition;

    #[wasm_bindgen(method, js_class = "IViewState", js_name = "firstPosition", setter)]
    pub fn set_first_position(this: &IViewState, first_position: IPosition);

    #[wasm_bindgen(method, js_class = "IViewState", js_name = "firstPositionDeltaTop", getter)]
    pub fn first_position_delta_top(this: &IViewState) -> f64;

    #[wasm_bindgen(method, js_class = "IViewState", js_name = "firstPositionDeltaTop", setter)]
    pub fn set_first_position_delta_top(this: &IViewState, first_position_delta_top: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICodeEditorViewState;

    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "cursorState", getter)]
    pub fn cursor_state(this: &ICodeEditorViewState) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "cursorState", setter)]
    pub fn set_cursor_state(this: &ICodeEditorViewState, cursor_state: js_sys::Array);

    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "viewState", getter)]
    pub fn view_state(this: &ICodeEditorViewState) -> IViewState;

    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "viewState", setter)]
    pub fn set_view_state(this: &ICodeEditorViewState, view_state: IViewState);

    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "contributionsState", getter)]
    pub fn contributions_state(this: &ICodeEditorViewState) -> js_sys::Object;

    #[wasm_bindgen(method, js_class = "ICodeEditorViewState", js_name = "contributionsState", setter)]
    pub fn set_contributions_state(this: &ICodeEditorViewState, contributions_state: js_sys::Object);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDiffEditorViewState;

    #[wasm_bindgen(method, js_class = "IDiffEditorViewState", js_name = "original", getter)]
    pub fn original(this: &IDiffEditorViewState) -> JsValue;

    #[wasm_bindgen(method, js_class = "IDiffEditorViewState", js_name = "original", setter)]
    pub fn set_original(this: &IDiffEditorViewState, original: JsValue);

    #[wasm_bindgen(method, js_class = "IDiffEditorViewState", js_name = "modified", getter)]
    pub fn modified(this: &IDiffEditorViewState) -> JsValue;

    #[wasm_bindgen(method, js_class = "IDiffEditorViewState", js_name = "modified", setter)]
    pub fn set_modified(this: &IDiffEditorViewState, modified: JsValue);

    #[wasm_bindgen(method, js_class = "IDiffEditorViewState", js_name = "modelState", getter)]
    pub fn model_state(this: &IDiffEditorViewState) -> JsValue;

    #[wasm_bindgen(method, js_class = "IDiffEditorViewState", js_name = "modelState", setter)]
    pub fn set_model_state(this: &IDiffEditorViewState, model_state: JsValue);


}

pub type IEditorViewState = JsValue
;

            int_enum! {
#[allow(non_camel_case_types)]
pub enum ScrollType {
    Smooth = 0,
    Immediate = 1,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditor;

    /**
         * An event emitted when the editor has been disposed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "onDidDispose")]
    pub fn on_did_dispose(this: &IEditor, listener: JsValue, ) -> IDisposable;

    /**
         * Dispose the editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "dispose")]
    pub fn dispose(this: &IEditor, );

    /**
         * Get a unique id for this editor instance.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getId")]
    pub fn get_id(this: &IEditor, ) -> String;

    /**
         * Get the editor type. Please see `EditorType`.
         * This is to avoid an instanceof check
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getEditorType")]
    pub fn get_editor_type(this: &IEditor, ) -> String;

    /**
         * Update the editor's options after the editor has been created.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "updateOptions")]
    pub fn update_options(this: &IEditor, new_options: IEditorOptions, );

    /**
         * Instructs the editor to remeasure its container. This method should
         * be called when the container of the editor gets resized.
         *
         * If a dimension is passed in, the passed in value will be used.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "layout")]
    pub fn layout(this: &IEditor, dimension: Option<IDimension>, );

    /**
         * Brings browser focus to the editor text
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "focus")]
    pub fn focus(this: &IEditor, );

    /**
         * Returns true if the text inside this editor is focused (i.e. cursor is blinking).
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "hasTextFocus")]
    pub fn has_text_focus(this: &IEditor, ) -> bool;

    /**
         * Returns all actions associated with this editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getSupportedActions")]
    pub fn get_supported_actions(this: &IEditor, ) -> js_sys::Array;

    /**
         * Saves current view state of the editor in a serializable object.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "saveViewState")]
    pub fn save_view_state(this: &IEditor, ) -> JsValue;

    /**
         * Restores the view state of the editor from a serializable object generated by `saveViewState`.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "restoreViewState")]
    pub fn restore_view_state(this: &IEditor, state: JsValue, );

    /**
         * Given a position, returns a column number that takes tab-widths into account.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getVisibleColumnFromPosition")]
    pub fn get_visible_column_from_position(this: &IEditor, position: IPosition, ) -> f64;

    /**
         * Returns the primary position of the cursor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getPosition")]
    pub fn get_position(this: &IEditor, ) -> JsValue;

    /**
         * Set the primary position of the cursor. This will remove any secondary cursors.
         * @param position New primary cursor's position
         * @param source Source of the call that caused the position
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setPosition")]
    pub fn set_position(this: &IEditor, position: IPosition, source: Option<String>, );

    /**
         * Scroll vertically as necessary and reveal a line.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLine")]
    pub fn reveal_line(this: &IEditor, line_number: f64, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically as necessary and reveal a line centered vertically.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLineInCenter")]
    pub fn reveal_line_in_center(this: &IEditor, line_number: f64, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically as necessary and reveal a line centered vertically only if it lies outside the viewport.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLineInCenterIfOutsideViewport")]
    pub fn reveal_line_in_center_if_outside_viewport(this: &IEditor, line_number: f64, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically as necessary and reveal a line close to the top of the viewport,
         * optimized for viewing a code definition.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLineNearTop")]
    pub fn reveal_line_near_top(this: &IEditor, line_number: f64, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically or horizontally as necessary and reveal a position.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealPosition")]
    pub fn reveal_position(this: &IEditor, position: IPosition, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically or horizontally as necessary and reveal a position centered vertically.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealPositionInCenter")]
    pub fn reveal_position_in_center(this: &IEditor, position: IPosition, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically or horizontally as necessary and reveal a position centered vertically only if it lies outside the viewport.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealPositionInCenterIfOutsideViewport")]
    pub fn reveal_position_in_center_if_outside_viewport(this: &IEditor, position: IPosition, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically or horizontally as necessary and reveal a position close to the top of the viewport,
         * optimized for viewing a code definition.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealPositionNearTop")]
    pub fn reveal_position_near_top(this: &IEditor, position: IPosition, scroll_type: Option<ScrollType>, );

    /**
         * Returns the primary selection of the editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getSelection")]
    pub fn get_selection(this: &IEditor, ) -> JsValue;

    /**
         * Returns all the selections of the editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getSelections")]
    pub fn get_selections(this: &IEditor, ) -> JsValue;

    /**
         * Set the primary selection of the editor. This will remove any secondary cursors.
         * @param selection The new selection
         * @param source Source of the call that caused the selection
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setSelection")]
    pub fn set_selection(this: &IEditor, selection: IRange, source: Option<String>, );

    /**
         * Set the primary selection of the editor. This will remove any secondary cursors.
         * @param selection The new selection
         * @param source Source of the call that caused the selection
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setSelection")]
    pub fn set_selection_1(this: &IEditor, selection: Range, source: Option<String>, );

    /**
         * Set the primary selection of the editor. This will remove any secondary cursors.
         * @param selection The new selection
         * @param source Source of the call that caused the selection
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setSelection")]
    pub fn set_selection_2(this: &IEditor, selection: ISelection, source: Option<String>, );

    /**
         * Set the primary selection of the editor. This will remove any secondary cursors.
         * @param selection The new selection
         * @param source Source of the call that caused the selection
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setSelection")]
    pub fn set_selection_3(this: &IEditor, selection: Selection, source: Option<String>, );

    /**
         * Set the selections for all the cursors of the editor.
         * Cursors will be removed or added, as necessary.
         * @param selections The new selection
         * @param source Source of the call that caused the selection
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setSelections")]
    pub fn set_selections(this: &IEditor, selections: JsValue, source: Option<String>, );

    /**
         * Scroll vertically as necessary and reveal lines.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLines")]
    pub fn reveal_lines(this: &IEditor, start_line_number: f64, end_line_number: f64, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically as necessary and reveal lines centered vertically.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLinesInCenter")]
    pub fn reveal_lines_in_center(this: &IEditor, line_number: f64, end_line_number: f64, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically as necessary and reveal lines centered vertically only if it lies outside the viewport.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLinesInCenterIfOutsideViewport")]
    pub fn reveal_lines_in_center_if_outside_viewport(this: &IEditor, line_number: f64, end_line_number: f64, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically as necessary and reveal lines close to the top of the viewport,
         * optimized for viewing a code definition.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealLinesNearTop")]
    pub fn reveal_lines_near_top(this: &IEditor, line_number: f64, end_line_number: f64, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically or horizontally as necessary and reveal a range.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealRange")]
    pub fn reveal_range(this: &IEditor, range: IRange, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically or horizontally as necessary and reveal a range centered vertically.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealRangeInCenter")]
    pub fn reveal_range_in_center(this: &IEditor, range: IRange, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically or horizontally as necessary and reveal a range at the top of the viewport.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealRangeAtTop")]
    pub fn reveal_range_at_top(this: &IEditor, range: IRange, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically or horizontally as necessary and reveal a range centered vertically only if it lies outside the viewport.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealRangeInCenterIfOutsideViewport")]
    pub fn reveal_range_in_center_if_outside_viewport(this: &IEditor, range: IRange, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically or horizontally as necessary and reveal a range close to the top of the viewport,
         * optimized for viewing a code definition.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealRangeNearTop")]
    pub fn reveal_range_near_top(this: &IEditor, range: IRange, scroll_type: Option<ScrollType>, );

    /**
         * Scroll vertically or horizontally as necessary and reveal a range close to the top of the viewport,
         * optimized for viewing a code definition. Only if it lies outside the viewport.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "revealRangeNearTopIfOutsideViewport")]
    pub fn reveal_range_near_top_if_outside_viewport(this: &IEditor, range: IRange, scroll_type: Option<ScrollType>, );

    /**
         * Directly trigger a handler or an editor action.
         * @param source The source of the call.
         * @param handlerId The id of the handler or the id of a contribution.
         * @param payload Extra data to be sent to the handler.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "trigger")]
    pub fn trigger(this: &IEditor, source: JsValue, handler_id: String, payload: JsValue, );

    /**
         * Gets the current model attached to this editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "getModel")]
    pub fn get_model(this: &IEditor, ) -> JsValue;

    /**
         * Sets the current model attached to this editor.
         * If the previous model was created by the editor via the value key in the options
         * literal object, it will be destroyed. Otherwise, if the previous model was set
         * via setModel, or the model key in the options literal object, the previous model
         * will not be destroyed.
         * It is safe to call setModel(null) to simply detach the current model from the editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "setModel")]
    pub fn set_model(this: &IEditor, model: JsValue, );

    /**
         * Create a collection of decorations. All decorations added through this collection
         * will get the ownerId of the editor (meaning they will not show up in other editors).
         * These decorations will be automatically cleared when the editor's model changes.
         */ 
    #[wasm_bindgen(method, js_class = "IEditor", js_name = "createDecorationsCollection")]
    pub fn create_decorations_collection(this: &IEditor, decorations: Option<js_sys::Array>, ) -> IEditorDecorationsCollection;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorDecorationsCollection;

    /**
         * An event emitted when decorations change in the editor,
         * but the change is not caused by us setting or clearing the collection.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorDecorationsCollection", js_name = "onDidChange", getter)]
    pub fn on_did_change(this: &IEditorDecorationsCollection) -> IEvent;

    #[wasm_bindgen(method, js_class = "IEditorDecorationsCollection", js_name = "onDidChange", setter)]
    pub fn set_on_did_change(this: &IEditorDecorationsCollection, on_did_change: IEvent);

    /**
         * Get the decorations count.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorDecorationsCollection", js_name = "length", getter)]
    pub fn length(this: &IEditorDecorationsCollection) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorDecorationsCollection", js_name = "length", setter)]
    pub fn set_length(this: &IEditorDecorationsCollection, length: f64);

    /**
         * Get the range for a decoration.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorDecorationsCollection", js_name = "getRange")]
    pub fn get_range(this: &IEditorDecorationsCollection, index: f64, ) -> JsValue;

    /**
         * Get all ranges for decorations.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorDecorationsCollection", js_name = "getRanges")]
    pub fn get_ranges(this: &IEditorDecorationsCollection, ) -> js_sys::Array;

    /**
         * Determine if a decoration is in this collection.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorDecorationsCollection", js_name = "has")]
    pub fn has(this: &IEditorDecorationsCollection, decoration: IModelDecoration, ) -> bool;

    /**
         * Replace all previous decorations with `newDecorations`.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorDecorationsCollection", js_name = "set")]
    pub fn set(this: &IEditorDecorationsCollection, new_decorations: JsValue, ) -> js_sys::Array;

    /**
         * Remove all previous decorations.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorDecorationsCollection", js_name = "clear")]
    pub fn clear(this: &IEditorDecorationsCollection, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorContribution;

    /**
         * Dispose this contribution.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorContribution", js_name = "dispose")]
    pub fn dispose(this: &IEditorContribution, );

    /**
         * Store view state.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorContribution", js_name = "saveViewState")]
    pub fn save_view_state(this: &IEditorContribution, ) -> JsValue;

    /**
         * Restore view state.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorContribution", js_name = "restoreViewState")]
    pub fn restore_view_state(this: &IEditorContribution, state: JsValue, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelLanguageChangedEvent;

    /**
         * Previous language
         */ 
    #[wasm_bindgen(method, js_class = "IModelLanguageChangedEvent", js_name = "oldLanguage", getter)]
    pub fn old_language(this: &IModelLanguageChangedEvent) -> String;
    /**
         * New language
         */ 
    #[wasm_bindgen(method, js_class = "IModelLanguageChangedEvent", js_name = "newLanguage", getter)]
    pub fn new_language(this: &IModelLanguageChangedEvent) -> String;
    /**
         * Source of the call that caused the event.
         */ 
    #[wasm_bindgen(method, js_class = "IModelLanguageChangedEvent", js_name = "source", getter)]
    pub fn source(this: &IModelLanguageChangedEvent) -> String;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelLanguageConfigurationChangedEvent;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelContentChange;

    /**
         * The range that got replaced.
         */ 
    #[wasm_bindgen(method, js_class = "IModelContentChange", js_name = "range", getter)]
    pub fn range(this: &IModelContentChange) -> IRange;
    /**
         * The offset of the range that got replaced.
         */ 
    #[wasm_bindgen(method, js_class = "IModelContentChange", js_name = "rangeOffset", getter)]
    pub fn range_offset(this: &IModelContentChange) -> f64;
    /**
         * The length of the range that got replaced.
         */ 
    #[wasm_bindgen(method, js_class = "IModelContentChange", js_name = "rangeLength", getter)]
    pub fn range_length(this: &IModelContentChange) -> f64;
    /**
         * The new text for the range.
         */ 
    #[wasm_bindgen(method, js_class = "IModelContentChange", js_name = "text", getter)]
    pub fn text(this: &IModelContentChange) -> String;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelContentChangedEvent;

    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "changes", getter)]
    pub fn changes(this: &IModelContentChangedEvent) -> js_sys::Array;
    /**
         * The (new) end-of-line character.
         */ 
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "eol", getter)]
    pub fn eol(this: &IModelContentChangedEvent) -> String;
    /**
         * The new version id the model has transitioned to.
         */ 
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "versionId", getter)]
    pub fn version_id(this: &IModelContentChangedEvent) -> f64;
    /**
         * Flag that indicates that this event was generated while undoing.
         */ 
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "isUndoing", getter)]
    pub fn is_undoing(this: &IModelContentChangedEvent) -> bool;
    /**
         * Flag that indicates that this event was generated while redoing.
         */ 
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "isRedoing", getter)]
    pub fn is_redoing(this: &IModelContentChangedEvent) -> bool;
    /**
         * Flag that indicates that all decorations were lost with this edit.
         * The model has been reset to a new value.
         */ 
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "isFlush", getter)]
    pub fn is_flush(this: &IModelContentChangedEvent) -> bool;
    /**
         * Flag that indicates that this event describes an eol change.
         */ 
    #[wasm_bindgen(method, js_class = "IModelContentChangedEvent", js_name = "isEolChange", getter)]
    pub fn is_eol_change(this: &IModelContentChangedEvent) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelDecorationsChangedEvent;

    #[wasm_bindgen(method, js_class = "IModelDecorationsChangedEvent", js_name = "affectsMinimap", getter)]
    pub fn affects_minimap(this: &IModelDecorationsChangedEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IModelDecorationsChangedEvent", js_name = "affectsOverviewRuler", getter)]
    pub fn affects_overview_ruler(this: &IModelDecorationsChangedEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IModelDecorationsChangedEvent", js_name = "affectsGlyphMargin", getter)]
    pub fn affects_glyph_margin(this: &IModelDecorationsChangedEvent) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IModelOptionsChangedEvent;

    #[wasm_bindgen(method, js_class = "IModelOptionsChangedEvent", js_name = "tabSize", getter)]
    pub fn tab_size(this: &IModelOptionsChangedEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IModelOptionsChangedEvent", js_name = "indentSize", getter)]
    pub fn indent_size(this: &IModelOptionsChangedEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IModelOptionsChangedEvent", js_name = "insertSpaces", getter)]
    pub fn insert_spaces(this: &IModelOptionsChangedEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IModelOptionsChangedEvent", js_name = "trimAutoWhitespace", getter)]
    pub fn trim_auto_whitespace(this: &IModelOptionsChangedEvent) -> bool;

}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum CursorChangeReason {
    /**
         * Unknown or not set.
         */ 
    NotSet = 0,
    /**
         * A `model.setValue()` was called.
         */ 
    ContentFlush = 1,
    /**
         * The `model` has been changed outside of this cursor and the cursor recovers its position from associated markers.
         */ 
    RecoverFromMarkers = 2,
    /**
         * There was an explicit user gesture.
         */ 
    Explicit = 3,
    /**
         * There was a Paste.
         */ 
    Paste = 4,
    /**
         * There was an Undo.
         */ 
    Undo = 5,
    /**
         * There was a Redo.
         */ 
    Redo = 6,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICursorPositionChangedEvent;

    /**
         * Primary cursor's position.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorPositionChangedEvent", js_name = "position", getter)]
    pub fn position(this: &ICursorPositionChangedEvent) -> Position;
    /**
         * Secondary cursors' position.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorPositionChangedEvent", js_name = "secondaryPositions", getter)]
    pub fn secondary_positions(this: &ICursorPositionChangedEvent) -> js_sys::Array;
    /**
         * Reason.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorPositionChangedEvent", js_name = "reason", getter)]
    pub fn reason(this: &ICursorPositionChangedEvent) -> CursorChangeReason;
    /**
         * Source of the call that caused the event.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorPositionChangedEvent", js_name = "source", getter)]
    pub fn source(this: &ICursorPositionChangedEvent) -> String;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICursorSelectionChangedEvent;

    /**
         * The primary selection.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "selection", getter)]
    pub fn selection(this: &ICursorSelectionChangedEvent) -> Selection;
    /**
         * The secondary selections.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "secondarySelections", getter)]
    pub fn secondary_selections(this: &ICursorSelectionChangedEvent) -> js_sys::Array;
    /**
         * The model version id.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "modelVersionId", getter)]
    pub fn model_version_id(this: &ICursorSelectionChangedEvent) -> f64;
    /**
         * The old selections.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "oldSelections", getter)]
    pub fn old_selections(this: &ICursorSelectionChangedEvent) -> JsValue;
    /**
         * The model version id the that `oldSelections` refer to.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "oldModelVersionId", getter)]
    pub fn old_model_version_id(this: &ICursorSelectionChangedEvent) -> f64;
    /**
         * Source of the call that caused the event.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "source", getter)]
    pub fn source(this: &ICursorSelectionChangedEvent) -> String;
    /**
         * Reason.
         */ 
    #[wasm_bindgen(method, js_class = "ICursorSelectionChangedEvent", js_name = "reason", getter)]
    pub fn reason(this: &ICursorSelectionChangedEvent) -> CursorChangeReason;

}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum AccessibilitySupport {
    /**
         * This should be the browser case where it is not known if a screen reader is attached or no.
         */ 
    Unknown = 0,
    Disabled = 1,
    Enabled = 2,

  }
}

pub type EditorAutoClosingStrategy = JsValue
;
pub type EditorAutoSurroundStrategy = JsValue
;
pub type EditorAutoClosingEditStrategy = JsValue
;

            int_enum! {
#[allow(non_camel_case_types)]
pub enum EditorAutoIndentStrategy {
    None = 0,
    Keep = 1,
    Brackets = 2,
    Advanced = 3,
    Full = 4,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorOptions;

    /**
         * This editor is used inside a diff editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "inDiffEditor", getter)]
    pub fn in_diff_editor(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "inDiffEditor", setter)]
    pub fn set_in_diff_editor(this: &IEditorOptions, in_diff_editor: bool);

    /**
         * The aria label for the editor's textarea (when it is focused).
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "ariaLabel", getter)]
    pub fn aria_label(this: &IEditorOptions) -> String;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "ariaLabel", setter)]
    pub fn set_aria_label(this: &IEditorOptions, aria_label: String);

    /**
         * Whether the aria-required attribute should be set on the editors textarea.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "ariaRequired", getter)]
    pub fn aria_required(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "ariaRequired", setter)]
    pub fn set_aria_required(this: &IEditorOptions, aria_required: bool);

    /**
         * Control whether a screen reader announces inline suggestion content immediately.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "screenReaderAnnounceInlineSuggestion", getter)]
    pub fn screen_reader_announce_inline_suggestion(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "screenReaderAnnounceInlineSuggestion", setter)]
    pub fn set_screen_reader_announce_inline_suggestion(this: &IEditorOptions, screen_reader_announce_inline_suggestion: bool);

    /**
         * The `tabindex` property of the editor's textarea
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "tabIndex", getter)]
    pub fn tab_index(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "tabIndex", setter)]
    pub fn set_tab_index(this: &IEditorOptions, tab_index: f64);

    /**
         * Render vertical lines at the specified columns.
         * Defaults to empty array.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "rulers", getter)]
    pub fn rulers(this: &IEditorOptions) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "rulers", setter)]
    pub fn set_rulers(this: &IEditorOptions, rulers: js_sys::Array);

    /**
         * A string containing the word separators used when doing word navigation.
         * Defaults to `~!@#$%^&*()-=+[{]}\\|;:\'",.<>/?
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordSeparators", getter)]
    pub fn word_separators(this: &IEditorOptions) -> String;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordSeparators", setter)]
    pub fn set_word_separators(this: &IEditorOptions, word_separators: String);

    /**
         * Enable Linux primary clipboard.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectionClipboard", getter)]
    pub fn selection_clipboard(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectionClipboard", setter)]
    pub fn set_selection_clipboard(this: &IEditorOptions, selection_clipboard: bool);

    /**
         * Control the rendering of line numbers.
         * If it is a function, it will be invoked when rendering a line number and the return value will be rendered.
         * Otherwise, if it is a truthy, line numbers will be rendered normally (equivalent of using an identity function).
         * Otherwise, line numbers will not be rendered.
         * Defaults to `on`.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineNumbers", getter)]
    pub fn line_numbers(this: &IEditorOptions) -> LineNumbersType;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineNumbers", setter)]
    pub fn set_line_numbers(this: &IEditorOptions, line_numbers: LineNumbersType);

    /**
         * Controls the minimal number of visible leading and trailing lines surrounding the cursor.
         * Defaults to 0.
        */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSurroundingLines", getter)]
    pub fn cursor_surrounding_lines(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSurroundingLines", setter)]
    pub fn set_cursor_surrounding_lines(this: &IEditorOptions, cursor_surrounding_lines: f64);

    /**
         * Controls when `cursorSurroundingLines` should be enforced
         * Defaults to `default`, `cursorSurroundingLines` is not enforced when cursor position is changed
         * by mouse.
        */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSurroundingLinesStyle", getter)]
    pub fn cursor_surrounding_lines_style(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSurroundingLinesStyle", setter)]
    pub fn set_cursor_surrounding_lines_style(this: &IEditorOptions, cursor_surrounding_lines_style: JsValue);

    /**
         * Render last line number when the file ends with a newline.
         * Defaults to 'on' for Windows and macOS and 'dimmed' for Linux.
        */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderFinalNewline", getter)]
    pub fn render_final_newline(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderFinalNewline", setter)]
    pub fn set_render_final_newline(this: &IEditorOptions, render_final_newline: JsValue);

    /**
         * Remove unusual line terminators like LINE SEPARATOR (LS), PARAGRAPH SEPARATOR (PS).
         * Defaults to 'prompt'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "unusualLineTerminators", getter)]
    pub fn unusual_line_terminators(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "unusualLineTerminators", setter)]
    pub fn set_unusual_line_terminators(this: &IEditorOptions, unusual_line_terminators: JsValue);

    /**
         * Should the corresponding line be selected when clicking on the line number?
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectOnLineNumbers", getter)]
    pub fn select_on_line_numbers(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectOnLineNumbers", setter)]
    pub fn set_select_on_line_numbers(this: &IEditorOptions, select_on_line_numbers: bool);

    /**
         * Control the width of line numbers, by reserving horizontal space for rendering at least an amount of digits.
         * Defaults to 5.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineNumbersMinChars", getter)]
    pub fn line_numbers_min_chars(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineNumbersMinChars", setter)]
    pub fn set_line_numbers_min_chars(this: &IEditorOptions, line_numbers_min_chars: f64);

    /**
         * Enable the rendering of the glyph margin.
         * Defaults to true in vscode and to false in monaco-editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "glyphMargin", getter)]
    pub fn glyph_margin(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "glyphMargin", setter)]
    pub fn set_glyph_margin(this: &IEditorOptions, glyph_margin: bool);

    /**
         * The width reserved for line decorations (in px).
         * Line decorations are placed between line numbers and the editor content.
         * You can pass in a string in the format floating point followed by "ch". e.g. 1.3ch.
         * Defaults to 10.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineDecorationsWidth", getter)]
    pub fn line_decorations_width(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineDecorationsWidth", setter)]
    pub fn set_line_decorations_width(this: &IEditorOptions, line_decorations_width: JsValue);

    /**
         * When revealing the cursor, a virtual padding (px) is added to the cursor, turning it into a rectangle.
         * This virtual padding ensures that the cursor gets revealed before hitting the edge of the viewport.
         * Defaults to 30 (px).
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "revealHorizontalRightPadding", getter)]
    pub fn reveal_horizontal_right_padding(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "revealHorizontalRightPadding", setter)]
    pub fn set_reveal_horizontal_right_padding(this: &IEditorOptions, reveal_horizontal_right_padding: f64);

    /**
         * Render the editor selection with rounded borders.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "roundedSelection", getter)]
    pub fn rounded_selection(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "roundedSelection", setter)]
    pub fn set_rounded_selection(this: &IEditorOptions, rounded_selection: bool);

    /**
         * Class name to be added to the editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "extraEditorClassName", getter)]
    pub fn extra_editor_class_name(this: &IEditorOptions) -> String;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "extraEditorClassName", setter)]
    pub fn set_extra_editor_class_name(this: &IEditorOptions, extra_editor_class_name: String);

    /**
         * Should the editor be read only. See also `domReadOnly`.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "readOnly", getter)]
    pub fn read_only(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "readOnly", setter)]
    pub fn set_read_only(this: &IEditorOptions, read_only: bool);

    /**
         * The message to display when the editor is readonly.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "readOnlyMessage", getter)]
    pub fn read_only_message(this: &IEditorOptions) -> IMarkdownString;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "readOnlyMessage", setter)]
    pub fn set_read_only_message(this: &IEditorOptions, read_only_message: IMarkdownString);

    /**
         * Should the textarea used for input use the DOM `readonly` attribute.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "domReadOnly", getter)]
    pub fn dom_read_only(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "domReadOnly", setter)]
    pub fn set_dom_read_only(this: &IEditorOptions, dom_read_only: bool);

    /**
         * Enable linked editing.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "linkedEditing", getter)]
    pub fn linked_editing(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "linkedEditing", setter)]
    pub fn set_linked_editing(this: &IEditorOptions, linked_editing: bool);

    /**
         * deprecated, use linkedEditing instead
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renameOnType", getter)]
    pub fn rename_on_type(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renameOnType", setter)]
    pub fn set_rename_on_type(this: &IEditorOptions, rename_on_type: bool);

    /**
         * Should the editor render validation decorations.
         * Defaults to editable.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderValidationDecorations", getter)]
    pub fn render_validation_decorations(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderValidationDecorations", setter)]
    pub fn set_render_validation_decorations(this: &IEditorOptions, render_validation_decorations: JsValue);

    /**
         * Control the behavior and rendering of the scrollbars.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollbar", getter)]
    pub fn scrollbar(this: &IEditorOptions) -> IEditorScrollbarOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollbar", setter)]
    pub fn set_scrollbar(this: &IEditorOptions, scrollbar: IEditorScrollbarOptions);

    /**
         * Control the behavior of sticky scroll options
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "stickyScroll", getter)]
    pub fn sticky_scroll(this: &IEditorOptions) -> IEditorStickyScrollOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "stickyScroll", setter)]
    pub fn set_sticky_scroll(this: &IEditorOptions, sticky_scroll: IEditorStickyScrollOptions);

    /**
         * Control the behavior and rendering of the minimap.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "minimap", getter)]
    pub fn minimap(this: &IEditorOptions) -> IEditorMinimapOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "minimap", setter)]
    pub fn set_minimap(this: &IEditorOptions, minimap: IEditorMinimapOptions);

    /**
         * Control the behavior of the find widget.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "find", getter)]
    pub fn find(this: &IEditorOptions) -> IEditorFindOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "find", setter)]
    pub fn set_find(this: &IEditorOptions, find: IEditorFindOptions);

    /**
         * Display overflow widgets as `fixed`.
         * Defaults to `false`.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fixedOverflowWidgets", getter)]
    pub fn fixed_overflow_widgets(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fixedOverflowWidgets", setter)]
    pub fn set_fixed_overflow_widgets(this: &IEditorOptions, fixed_overflow_widgets: bool);

    /**
         * The number of vertical lanes the overview ruler should render.
         * Defaults to 3.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "overviewRulerLanes", getter)]
    pub fn overview_ruler_lanes(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "overviewRulerLanes", setter)]
    pub fn set_overview_ruler_lanes(this: &IEditorOptions, overview_ruler_lanes: f64);

    /**
         * Controls if a border should be drawn around the overview ruler.
         * Defaults to `true`.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "overviewRulerBorder", getter)]
    pub fn overview_ruler_border(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "overviewRulerBorder", setter)]
    pub fn set_overview_ruler_border(this: &IEditorOptions, overview_ruler_border: bool);

    /**
         * Control the cursor animation style, possible values are 'blink', 'smooth', 'phase', 'expand' and 'solid'.
         * Defaults to 'blink'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorBlinking", getter)]
    pub fn cursor_blinking(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorBlinking", setter)]
    pub fn set_cursor_blinking(this: &IEditorOptions, cursor_blinking: JsValue);

    /**
         * Zoom the font in the editor when using the mouse wheel in combination with holding Ctrl.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseWheelZoom", getter)]
    pub fn mouse_wheel_zoom(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseWheelZoom", setter)]
    pub fn set_mouse_wheel_zoom(this: &IEditorOptions, mouse_wheel_zoom: bool);

    /**
         * Control the mouse pointer style, either 'text' or 'default' or 'copy'
         * Defaults to 'text'
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseStyle", getter)]
    pub fn mouse_style(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseStyle", setter)]
    pub fn set_mouse_style(this: &IEditorOptions, mouse_style: JsValue);

    /**
         * Enable smooth caret animation.
         * Defaults to 'off'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSmoothCaretAnimation", getter)]
    pub fn cursor_smooth_caret_animation(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorSmoothCaretAnimation", setter)]
    pub fn set_cursor_smooth_caret_animation(this: &IEditorOptions, cursor_smooth_caret_animation: JsValue);

    /**
         * Control the cursor style, either 'block' or 'line'.
         * Defaults to 'line'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorStyle", getter)]
    pub fn cursor_style(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorStyle", setter)]
    pub fn set_cursor_style(this: &IEditorOptions, cursor_style: JsValue);

    /**
         * Control the width of the cursor when cursorStyle is set to 'line'
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorWidth", getter)]
    pub fn cursor_width(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "cursorWidth", setter)]
    pub fn set_cursor_width(this: &IEditorOptions, cursor_width: f64);

    /**
         * Enable font ligatures.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontLigatures", getter)]
    pub fn font_ligatures(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontLigatures", setter)]
    pub fn set_font_ligatures(this: &IEditorOptions, font_ligatures: JsValue);

    /**
         * Enable font variations.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontVariations", getter)]
    pub fn font_variations(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontVariations", setter)]
    pub fn set_font_variations(this: &IEditorOptions, font_variations: JsValue);

    /**
         * Controls whether to use default color decorations or not using the default document color provider
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "defaultColorDecorators", getter)]
    pub fn default_color_decorators(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "defaultColorDecorators", setter)]
    pub fn set_default_color_decorators(this: &IEditorOptions, default_color_decorators: bool);

    /**
         * Disable the use of `transform: translate3d(0px, 0px, 0px)` for the editor margin and lines layers.
         * The usage of `transform: translate3d(0px, 0px, 0px)` acts as a hint for browsers to create an extra layer.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "disableLayerHinting", getter)]
    pub fn disable_layer_hinting(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "disableLayerHinting", setter)]
    pub fn set_disable_layer_hinting(this: &IEditorOptions, disable_layer_hinting: bool);

    /**
         * Disable the optimizations for monospace fonts.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "disableMonospaceOptimizations", getter)]
    pub fn disable_monospace_optimizations(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "disableMonospaceOptimizations", setter)]
    pub fn set_disable_monospace_optimizations(this: &IEditorOptions, disable_monospace_optimizations: bool);

    /**
         * Should the cursor be hidden in the overview ruler.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "hideCursorInOverviewRuler", getter)]
    pub fn hide_cursor_in_overview_ruler(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "hideCursorInOverviewRuler", setter)]
    pub fn set_hide_cursor_in_overview_ruler(this: &IEditorOptions, hide_cursor_in_overview_ruler: bool);

    /**
         * Enable that scrolling can go one screen size after the last line.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollBeyondLastLine", getter)]
    pub fn scroll_beyond_last_line(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollBeyondLastLine", setter)]
    pub fn set_scroll_beyond_last_line(this: &IEditorOptions, scroll_beyond_last_line: bool);

    /**
         * Enable that scrolling can go beyond the last column by a number of columns.
         * Defaults to 5.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollBeyondLastColumn", getter)]
    pub fn scroll_beyond_last_column(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollBeyondLastColumn", setter)]
    pub fn set_scroll_beyond_last_column(this: &IEditorOptions, scroll_beyond_last_column: f64);

    /**
         * Enable that the editor animates scrolling to a position.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "smoothScrolling", getter)]
    pub fn smooth_scrolling(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "smoothScrolling", setter)]
    pub fn set_smooth_scrolling(this: &IEditorOptions, smooth_scrolling: bool);

    /**
         * Enable that the editor will install a ResizeObserver to check if its container dom node size has changed.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "automaticLayout", getter)]
    pub fn automatic_layout(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "automaticLayout", setter)]
    pub fn set_automatic_layout(this: &IEditorOptions, automatic_layout: bool);

    /**
         * Control the wrapping of the editor.
         * When `wordWrap` = "off", the lines will never wrap.
         * When `wordWrap` = "on", the lines will wrap at the viewport width.
         * When `wordWrap` = "wordWrapColumn", the lines will wrap at `wordWrapColumn`.
         * When `wordWrap` = "bounded", the lines will wrap at min(viewport width, wordWrapColumn).
         * Defaults to "off".
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrap", getter)]
    pub fn word_wrap(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrap", setter)]
    pub fn set_word_wrap(this: &IEditorOptions, word_wrap: JsValue);

    /**
         * Override the `wordWrap` setting.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapOverride1", getter)]
    pub fn word_wrap_override_1(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapOverride1", setter)]
    pub fn set_word_wrap_override_1(this: &IEditorOptions, word_wrap_override_1: JsValue);

    /**
         * Override the `wordWrapOverride1` setting.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapOverride2", getter)]
    pub fn word_wrap_override_2(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapOverride2", setter)]
    pub fn set_word_wrap_override_2(this: &IEditorOptions, word_wrap_override_2: JsValue);

    /**
         * Control the wrapping of the editor.
         * When `wordWrap` = "off", the lines will never wrap.
         * When `wordWrap` = "on", the lines will wrap at the viewport width.
         * When `wordWrap` = "wordWrapColumn", the lines will wrap at `wordWrapColumn`.
         * When `wordWrap` = "bounded", the lines will wrap at min(viewport width, wordWrapColumn).
         * Defaults to 80.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapColumn", getter)]
    pub fn word_wrap_column(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapColumn", setter)]
    pub fn set_word_wrap_column(this: &IEditorOptions, word_wrap_column: f64);

    /**
         * Control indentation of wrapped lines. Can be: 'none', 'same', 'indent' or 'deepIndent'.
         * Defaults to 'same' in vscode and to 'none' in monaco-editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wrappingIndent", getter)]
    pub fn wrapping_indent(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wrappingIndent", setter)]
    pub fn set_wrapping_indent(this: &IEditorOptions, wrapping_indent: JsValue);

    /**
         * Controls the wrapping strategy to use.
         * Defaults to 'simple'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wrappingStrategy", getter)]
    pub fn wrapping_strategy(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wrappingStrategy", setter)]
    pub fn set_wrapping_strategy(this: &IEditorOptions, wrapping_strategy: JsValue);

    /**
         * Configure word wrapping characters. A break will be introduced before these characters.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapBreakBeforeCharacters", getter)]
    pub fn word_wrap_break_before_characters(this: &IEditorOptions) -> String;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapBreakBeforeCharacters", setter)]
    pub fn set_word_wrap_break_before_characters(this: &IEditorOptions, word_wrap_break_before_characters: String);

    /**
         * Configure word wrapping characters. A break will be introduced after these characters.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapBreakAfterCharacters", getter)]
    pub fn word_wrap_break_after_characters(this: &IEditorOptions) -> String;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordWrapBreakAfterCharacters", setter)]
    pub fn set_word_wrap_break_after_characters(this: &IEditorOptions, word_wrap_break_after_characters: String);

    /**
         * Sets whether line breaks appear wherever the text would otherwise overflow its content box.
         * When wordBreak = 'normal', Use the default line break rule.
         * When wordBreak = 'keepAll', Word breaks should not be used for Chinese/Japanese/Korean (CJK) text. Non-CJK text behavior is the same as for normal.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordBreak", getter)]
    pub fn word_break(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "wordBreak", setter)]
    pub fn set_word_break(this: &IEditorOptions, word_break: JsValue);

    /**
         * Performance guard: Stop rendering a line after x characters.
         * Defaults to 10000.
         * Use -1 to never stop rendering
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "stopRenderingLineAfter", getter)]
    pub fn stop_rendering_line_after(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "stopRenderingLineAfter", setter)]
    pub fn set_stop_rendering_line_after(this: &IEditorOptions, stop_rendering_line_after: f64);

    /**
         * Configure the editor's hover.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "hover", getter)]
    pub fn hover(this: &IEditorOptions) -> IEditorHoverOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "hover", setter)]
    pub fn set_hover(this: &IEditorOptions, hover: IEditorHoverOptions);

    /**
         * Enable detecting links and making them clickable.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "links", getter)]
    pub fn links(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "links", setter)]
    pub fn set_links(this: &IEditorOptions, links: bool);

    /**
         * Enable inline color decorators and color picker rendering.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "colorDecorators", getter)]
    pub fn color_decorators(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "colorDecorators", setter)]
    pub fn set_color_decorators(this: &IEditorOptions, color_decorators: bool);

    /**
         * Controls what is the condition to spawn a color picker from a color dectorator
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "colorDecoratorsActivatedOn", getter)]
    pub fn color_decorators_activated_on(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "colorDecoratorsActivatedOn", setter)]
    pub fn set_color_decorators_activated_on(this: &IEditorOptions, color_decorators_activated_on: JsValue);

    /**
         * Controls the max number of color decorators that can be rendered in an editor at once.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "colorDecoratorsLimit", getter)]
    pub fn color_decorators_limit(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "colorDecoratorsLimit", setter)]
    pub fn set_color_decorators_limit(this: &IEditorOptions, color_decorators_limit: f64);

    /**
         * Control the behaviour of comments in the editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "comments", getter)]
    pub fn comments(this: &IEditorOptions) -> IEditorCommentsOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "comments", setter)]
    pub fn set_comments(this: &IEditorOptions, comments: IEditorCommentsOptions);

    /**
         * Enable custom contextmenu.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "contextmenu", getter)]
    pub fn contextmenu(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "contextmenu", setter)]
    pub fn set_contextmenu(this: &IEditorOptions, contextmenu: bool);

    /**
         * A multiplier to be used on the `deltaX` and `deltaY` of mouse wheel scroll events.
         * Defaults to 1.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseWheelScrollSensitivity", getter)]
    pub fn mouse_wheel_scroll_sensitivity(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "mouseWheelScrollSensitivity", setter)]
    pub fn set_mouse_wheel_scroll_sensitivity(this: &IEditorOptions, mouse_wheel_scroll_sensitivity: f64);

    /**
         * FastScrolling mulitplier speed when pressing `Alt`
         * Defaults to 5.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fastScrollSensitivity", getter)]
    pub fn fast_scroll_sensitivity(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fastScrollSensitivity", setter)]
    pub fn set_fast_scroll_sensitivity(this: &IEditorOptions, fast_scroll_sensitivity: f64);

    /**
         * Enable that the editor scrolls only the predominant axis. Prevents horizontal drift when scrolling vertically on a trackpad.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollPredominantAxis", getter)]
    pub fn scroll_predominant_axis(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "scrollPredominantAxis", setter)]
    pub fn set_scroll_predominant_axis(this: &IEditorOptions, scroll_predominant_axis: bool);

    /**
         * Enable that the selection with the mouse and keys is doing column selection.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "columnSelection", getter)]
    pub fn column_selection(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "columnSelection", setter)]
    pub fn set_column_selection(this: &IEditorOptions, column_selection: bool);

    /**
         * The modifier to be used to add multiple cursors with the mouse.
         * Defaults to 'alt'
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorModifier", getter)]
    pub fn multi_cursor_modifier(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorModifier", setter)]
    pub fn set_multi_cursor_modifier(this: &IEditorOptions, multi_cursor_modifier: JsValue);

    /**
         * Merge overlapping selections.
         * Defaults to true
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorMergeOverlapping", getter)]
    pub fn multi_cursor_merge_overlapping(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorMergeOverlapping", setter)]
    pub fn set_multi_cursor_merge_overlapping(this: &IEditorOptions, multi_cursor_merge_overlapping: bool);

    /**
         * Configure the behaviour when pasting a text with the line count equal to the cursor count.
         * Defaults to 'spread'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorPaste", getter)]
    pub fn multi_cursor_paste(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorPaste", setter)]
    pub fn set_multi_cursor_paste(this: &IEditorOptions, multi_cursor_paste: JsValue);

    /**
         * Controls the max number of text cursors that can be in an active editor at once.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorLimit", getter)]
    pub fn multi_cursor_limit(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "multiCursorLimit", setter)]
    pub fn set_multi_cursor_limit(this: &IEditorOptions, multi_cursor_limit: f64);

    /**
         * Configure the editor's accessibility support.
         * Defaults to 'auto'. It is best to leave this to 'auto'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "accessibilitySupport", getter)]
    pub fn accessibility_support(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "accessibilitySupport", setter)]
    pub fn set_accessibility_support(this: &IEditorOptions, accessibility_support: JsValue);

    /**
         * Controls the number of lines in the editor that can be read out by a screen reader
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "accessibilityPageSize", getter)]
    pub fn accessibility_page_size(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "accessibilityPageSize", setter)]
    pub fn set_accessibility_page_size(this: &IEditorOptions, accessibility_page_size: f64);

    /**
         * Suggest options.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggest", getter)]
    pub fn suggest(this: &IEditorOptions) -> ISuggestOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggest", setter)]
    pub fn set_suggest(this: &IEditorOptions, suggest: ISuggestOptions);

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "inlineSuggest", getter)]
    pub fn inline_suggest(this: &IEditorOptions) -> IInlineSuggestOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "inlineSuggest", setter)]
    pub fn set_inline_suggest(this: &IEditorOptions, inline_suggest: IInlineSuggestOptions);

    /**
         * Smart select options.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "smartSelect", getter)]
    pub fn smart_select(this: &IEditorOptions) -> ISmartSelectOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "smartSelect", setter)]
    pub fn set_smart_select(this: &IEditorOptions, smart_select: ISmartSelectOptions);

    /**
         *
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "gotoLocation", getter)]
    pub fn goto_location(this: &IEditorOptions) -> IGotoLocationOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "gotoLocation", setter)]
    pub fn set_goto_location(this: &IEditorOptions, goto_location: IGotoLocationOptions);

    /**
         * Enable quick suggestions (shadow suggestions)
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "quickSuggestions", getter)]
    pub fn quick_suggestions(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "quickSuggestions", setter)]
    pub fn set_quick_suggestions(this: &IEditorOptions, quick_suggestions: JsValue);

    /**
         * Quick suggestions show delay (in ms)
         * Defaults to 10 (ms)
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "quickSuggestionsDelay", getter)]
    pub fn quick_suggestions_delay(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "quickSuggestionsDelay", setter)]
    pub fn set_quick_suggestions_delay(this: &IEditorOptions, quick_suggestions_delay: f64);

    /**
         * Controls the spacing around the editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "padding", getter)]
    pub fn padding(this: &IEditorOptions) -> IEditorPaddingOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "padding", setter)]
    pub fn set_padding(this: &IEditorOptions, padding: IEditorPaddingOptions);

    /**
         * Parameter hint options.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "parameterHints", getter)]
    pub fn parameter_hints(this: &IEditorOptions) -> IEditorParameterHintOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "parameterHints", setter)]
    pub fn set_parameter_hints(this: &IEditorOptions, parameter_hints: IEditorParameterHintOptions);

    /**
         * Options for auto closing brackets.
         * Defaults to language defined behavior.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingBrackets", getter)]
    pub fn auto_closing_brackets(this: &IEditorOptions) -> EditorAutoClosingStrategy;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingBrackets", setter)]
    pub fn set_auto_closing_brackets(this: &IEditorOptions, auto_closing_brackets: EditorAutoClosingStrategy);

    /**
         * Options for auto closing quotes.
         * Defaults to language defined behavior.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingQuotes", getter)]
    pub fn auto_closing_quotes(this: &IEditorOptions) -> EditorAutoClosingStrategy;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingQuotes", setter)]
    pub fn set_auto_closing_quotes(this: &IEditorOptions, auto_closing_quotes: EditorAutoClosingStrategy);

    /**
         * Options for pressing backspace near quotes or bracket pairs.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingDelete", getter)]
    pub fn auto_closing_delete(this: &IEditorOptions) -> EditorAutoClosingEditStrategy;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingDelete", setter)]
    pub fn set_auto_closing_delete(this: &IEditorOptions, auto_closing_delete: EditorAutoClosingEditStrategy);

    /**
         * Options for typing over closing quotes or brackets.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingOvertype", getter)]
    pub fn auto_closing_overtype(this: &IEditorOptions) -> EditorAutoClosingEditStrategy;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoClosingOvertype", setter)]
    pub fn set_auto_closing_overtype(this: &IEditorOptions, auto_closing_overtype: EditorAutoClosingEditStrategy);

    /**
         * Options for auto surrounding.
         * Defaults to always allowing auto surrounding.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoSurround", getter)]
    pub fn auto_surround(this: &IEditorOptions) -> EditorAutoSurroundStrategy;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoSurround", setter)]
    pub fn set_auto_surround(this: &IEditorOptions, auto_surround: EditorAutoSurroundStrategy);

    /**
         * Controls whether the editor should automatically adjust the indentation when users type, paste, move or indent lines.
         * Defaults to advanced.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoIndent", getter)]
    pub fn auto_indent(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "autoIndent", setter)]
    pub fn set_auto_indent(this: &IEditorOptions, auto_indent: JsValue);

    /**
         * Emulate selection behaviour of tab characters when using spaces for indentation.
         * This means selection will stick to tab stops.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "stickyTabStops", getter)]
    pub fn sticky_tab_stops(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "stickyTabStops", setter)]
    pub fn set_sticky_tab_stops(this: &IEditorOptions, sticky_tab_stops: bool);

    /**
         * Enable format on type.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "formatOnType", getter)]
    pub fn format_on_type(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "formatOnType", setter)]
    pub fn set_format_on_type(this: &IEditorOptions, format_on_type: bool);

    /**
         * Enable format on paste.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "formatOnPaste", getter)]
    pub fn format_on_paste(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "formatOnPaste", setter)]
    pub fn set_format_on_paste(this: &IEditorOptions, format_on_paste: bool);

    /**
         * Controls if the editor should allow to move selections via drag and drop.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "dragAndDrop", getter)]
    pub fn drag_and_drop(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "dragAndDrop", setter)]
    pub fn set_drag_and_drop(this: &IEditorOptions, drag_and_drop: bool);

    /**
         * Enable the suggestion box to pop-up on trigger characters.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestOnTriggerCharacters", getter)]
    pub fn suggest_on_trigger_characters(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestOnTriggerCharacters", setter)]
    pub fn set_suggest_on_trigger_characters(this: &IEditorOptions, suggest_on_trigger_characters: bool);

    /**
         * Accept suggestions on ENTER.
         * Defaults to 'on'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "acceptSuggestionOnEnter", getter)]
    pub fn accept_suggestion_on_enter(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "acceptSuggestionOnEnter", setter)]
    pub fn set_accept_suggestion_on_enter(this: &IEditorOptions, accept_suggestion_on_enter: JsValue);

    /**
         * Accept suggestions on provider defined characters.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "acceptSuggestionOnCommitCharacter", getter)]
    pub fn accept_suggestion_on_commit_character(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "acceptSuggestionOnCommitCharacter", setter)]
    pub fn set_accept_suggestion_on_commit_character(this: &IEditorOptions, accept_suggestion_on_commit_character: bool);

    /**
         * Enable snippet suggestions. Default to 'true'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "snippetSuggestions", getter)]
    pub fn snippet_suggestions(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "snippetSuggestions", setter)]
    pub fn set_snippet_suggestions(this: &IEditorOptions, snippet_suggestions: JsValue);

    /**
         * Copying without a selection copies the current line.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "emptySelectionClipboard", getter)]
    pub fn empty_selection_clipboard(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "emptySelectionClipboard", setter)]
    pub fn set_empty_selection_clipboard(this: &IEditorOptions, empty_selection_clipboard: bool);

    /**
         * Syntax highlighting is copied.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "copyWithSyntaxHighlighting", getter)]
    pub fn copy_with_syntax_highlighting(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "copyWithSyntaxHighlighting", setter)]
    pub fn set_copy_with_syntax_highlighting(this: &IEditorOptions, copy_with_syntax_highlighting: bool);

    /**
         * The history mode for suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestSelection", getter)]
    pub fn suggest_selection(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestSelection", setter)]
    pub fn set_suggest_selection(this: &IEditorOptions, suggest_selection: JsValue);

    /**
         * The font size for the suggest widget.
         * Defaults to the editor font size.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestFontSize", getter)]
    pub fn suggest_font_size(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestFontSize", setter)]
    pub fn set_suggest_font_size(this: &IEditorOptions, suggest_font_size: f64);

    /**
         * The line height for the suggest widget.
         * Defaults to the editor line height.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestLineHeight", getter)]
    pub fn suggest_line_height(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "suggestLineHeight", setter)]
    pub fn set_suggest_line_height(this: &IEditorOptions, suggest_line_height: f64);

    /**
         * Enable tab completion.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "tabCompletion", getter)]
    pub fn tab_completion(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "tabCompletion", setter)]
    pub fn set_tab_completion(this: &IEditorOptions, tab_completion: JsValue);

    /**
         * Enable selection highlight.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectionHighlight", getter)]
    pub fn selection_highlight(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "selectionHighlight", setter)]
    pub fn set_selection_highlight(this: &IEditorOptions, selection_highlight: bool);

    /**
         * Enable semantic occurrences highlight.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "occurrencesHighlight", getter)]
    pub fn occurrences_highlight(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "occurrencesHighlight", setter)]
    pub fn set_occurrences_highlight(this: &IEditorOptions, occurrences_highlight: bool);

    /**
         * Show code lens
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeLens", getter)]
    pub fn code_lens(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeLens", setter)]
    pub fn set_code_lens(this: &IEditorOptions, code_lens: bool);

    /**
         * Code lens font family. Defaults to editor font family.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeLensFontFamily", getter)]
    pub fn code_lens_font_family(this: &IEditorOptions) -> String;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeLensFontFamily", setter)]
    pub fn set_code_lens_font_family(this: &IEditorOptions, code_lens_font_family: String);

    /**
         * Code lens font size. Default to 90% of the editor font size
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeLensFontSize", getter)]
    pub fn code_lens_font_size(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeLensFontSize", setter)]
    pub fn set_code_lens_font_size(this: &IEditorOptions, code_lens_font_size: f64);

    /**
         * Control the behavior and rendering of the code action lightbulb.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lightbulb", getter)]
    pub fn lightbulb(this: &IEditorOptions) -> IEditorLightbulbOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lightbulb", setter)]
    pub fn set_lightbulb(this: &IEditorOptions, lightbulb: IEditorLightbulbOptions);

    /**
         * Timeout for running code actions on save.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeActionsOnSaveTimeout", getter)]
    pub fn code_actions_on_save_timeout(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "codeActionsOnSaveTimeout", setter)]
    pub fn set_code_actions_on_save_timeout(this: &IEditorOptions, code_actions_on_save_timeout: f64);

    /**
         * Enable code folding.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "folding", getter)]
    pub fn folding(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "folding", setter)]
    pub fn set_folding(this: &IEditorOptions, folding: bool);

    /**
         * Selects the folding strategy. 'auto' uses the strategies contributed for the current document, 'indentation' uses the indentation based folding strategy.
         * Defaults to 'auto'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingStrategy", getter)]
    pub fn folding_strategy(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingStrategy", setter)]
    pub fn set_folding_strategy(this: &IEditorOptions, folding_strategy: JsValue);

    /**
         * Enable highlight for folded regions.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingHighlight", getter)]
    pub fn folding_highlight(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingHighlight", setter)]
    pub fn set_folding_highlight(this: &IEditorOptions, folding_highlight: bool);

    /**
         * Auto fold imports folding regions.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingImportsByDefault", getter)]
    pub fn folding_imports_by_default(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingImportsByDefault", setter)]
    pub fn set_folding_imports_by_default(this: &IEditorOptions, folding_imports_by_default: bool);

    /**
         * Maximum number of foldable regions.
         * Defaults to 5000.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingMaximumRegions", getter)]
    pub fn folding_maximum_regions(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "foldingMaximumRegions", setter)]
    pub fn set_folding_maximum_regions(this: &IEditorOptions, folding_maximum_regions: f64);

    /**
         * Controls whether the fold actions in the gutter stay always visible or hide unless the mouse is over the gutter.
         * Defaults to 'mouseover'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "showFoldingControls", getter)]
    pub fn show_folding_controls(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "showFoldingControls", setter)]
    pub fn set_show_folding_controls(this: &IEditorOptions, show_folding_controls: JsValue);

    /**
         * Controls whether clicking on the empty content after a folded line will unfold the line.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "unfoldOnClickAfterEndOfLine", getter)]
    pub fn unfold_on_click_after_end_of_line(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "unfoldOnClickAfterEndOfLine", setter)]
    pub fn set_unfold_on_click_after_end_of_line(this: &IEditorOptions, unfold_on_click_after_end_of_line: bool);

    /**
         * Enable highlighting of matching brackets.
         * Defaults to 'always'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "matchBrackets", getter)]
    pub fn match_brackets(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "matchBrackets", setter)]
    pub fn set_match_brackets(this: &IEditorOptions, match_brackets: JsValue);

    /**
         * Enable experimental whitespace rendering.
         * Defaults to 'svg'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "experimentalWhitespaceRendering", getter)]
    pub fn experimental_whitespace_rendering(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "experimentalWhitespaceRendering", setter)]
    pub fn set_experimental_whitespace_rendering(this: &IEditorOptions, experimental_whitespace_rendering: JsValue);

    /**
         * Enable rendering of whitespace.
         * Defaults to 'selection'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderWhitespace", getter)]
    pub fn render_whitespace(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderWhitespace", setter)]
    pub fn set_render_whitespace(this: &IEditorOptions, render_whitespace: JsValue);

    /**
         * Enable rendering of control characters.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderControlCharacters", getter)]
    pub fn render_control_characters(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderControlCharacters", setter)]
    pub fn set_render_control_characters(this: &IEditorOptions, render_control_characters: bool);

    /**
         * Enable rendering of current line highlight.
         * Defaults to all.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderLineHighlight", getter)]
    pub fn render_line_highlight(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderLineHighlight", setter)]
    pub fn set_render_line_highlight(this: &IEditorOptions, render_line_highlight: JsValue);

    /**
         * Control if the current line highlight should be rendered only the editor is focused.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderLineHighlightOnlyWhenFocus", getter)]
    pub fn render_line_highlight_only_when_focus(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "renderLineHighlightOnlyWhenFocus", setter)]
    pub fn set_render_line_highlight_only_when_focus(this: &IEditorOptions, render_line_highlight_only_when_focus: bool);

    /**
         * Inserting and deleting whitespace follows tab stops.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "useTabStops", getter)]
    pub fn use_tab_stops(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "useTabStops", setter)]
    pub fn set_use_tab_stops(this: &IEditorOptions, use_tab_stops: bool);

    /**
         * The font family
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontFamily", getter)]
    pub fn font_family(this: &IEditorOptions) -> String;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontFamily", setter)]
    pub fn set_font_family(this: &IEditorOptions, font_family: String);

    /**
         * The font weight
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontWeight", getter)]
    pub fn font_weight(this: &IEditorOptions) -> String;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontWeight", setter)]
    pub fn set_font_weight(this: &IEditorOptions, font_weight: String);

    /**
         * The font size
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontSize", getter)]
    pub fn font_size(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "fontSize", setter)]
    pub fn set_font_size(this: &IEditorOptions, font_size: f64);

    /**
         * The line height
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineHeight", getter)]
    pub fn line_height(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "lineHeight", setter)]
    pub fn set_line_height(this: &IEditorOptions, line_height: f64);

    /**
         * The letter spacing
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "letterSpacing", getter)]
    pub fn letter_spacing(this: &IEditorOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "letterSpacing", setter)]
    pub fn set_letter_spacing(this: &IEditorOptions, letter_spacing: f64);

    /**
         * Controls fading out of unused variables.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "showUnused", getter)]
    pub fn show_unused(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "showUnused", setter)]
    pub fn set_show_unused(this: &IEditorOptions, show_unused: bool);

    /**
         * Controls whether to focus the inline editor in the peek widget by default.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "peekWidgetDefaultFocus", getter)]
    pub fn peek_widget_default_focus(this: &IEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "peekWidgetDefaultFocus", setter)]
    pub fn set_peek_widget_default_focus(this: &IEditorOptions, peek_widget_default_focus: JsValue);

    /**
         * Controls whether the definition link opens element in the peek widget.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "definitionLinkOpensInPeek", getter)]
    pub fn definition_link_opens_in_peek(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "definitionLinkOpensInPeek", setter)]
    pub fn set_definition_link_opens_in_peek(this: &IEditorOptions, definition_link_opens_in_peek: bool);

    /**
         * Controls strikethrough deprecated variables.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "showDeprecated", getter)]
    pub fn show_deprecated(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "showDeprecated", setter)]
    pub fn set_show_deprecated(this: &IEditorOptions, show_deprecated: bool);

    /**
         * Controls whether suggestions allow matches in the middle of the word instead of only at the beginning
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "matchOnWordStartOnly", getter)]
    pub fn match_on_word_start_only(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "matchOnWordStartOnly", setter)]
    pub fn set_match_on_word_start_only(this: &IEditorOptions, match_on_word_start_only: bool);

    /**
         * Control the behavior and rendering of the inline hints.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "inlayHints", getter)]
    pub fn inlay_hints(this: &IEditorOptions) -> IEditorInlayHintsOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "inlayHints", setter)]
    pub fn set_inlay_hints(this: &IEditorOptions, inlay_hints: IEditorInlayHintsOptions);

    /**
         * Control if the editor should use shadow DOM.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "useShadowDOM", getter)]
    pub fn use_shadow_dom(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "useShadowDOM", setter)]
    pub fn set_use_shadow_dom(this: &IEditorOptions, use_shadow_dom: bool);

    /**
         * Controls the behavior of editor guides.
        */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "guides", getter)]
    pub fn guides(this: &IEditorOptions) -> IGuidesOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "guides", setter)]
    pub fn set_guides(this: &IEditorOptions, guides: IGuidesOptions);

    /**
         * Controls the behavior of the unicode highlight feature
         * (by default, ambiguous and invisible characters are highlighted).
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "unicodeHighlight", getter)]
    pub fn unicode_highlight(this: &IEditorOptions) -> IUnicodeHighlightOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "unicodeHighlight", setter)]
    pub fn set_unicode_highlight(this: &IEditorOptions, unicode_highlight: IUnicodeHighlightOptions);

    /**
         * Configures bracket pair colorization (disabled by default).
        */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "bracketPairColorization", getter)]
    pub fn bracket_pair_colorization(this: &IEditorOptions) -> IBracketPairColorizationOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "bracketPairColorization", setter)]
    pub fn set_bracket_pair_colorization(this: &IEditorOptions, bracket_pair_colorization: IBracketPairColorizationOptions);

    /**
         * Controls dropping into the editor from an external source.
         *
         * When enabled, this shows a preview of the drop location and triggers an `onDropIntoEditor` event.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "dropIntoEditor", getter)]
    pub fn drop_into_editor(this: &IEditorOptions) -> IDropIntoEditorOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "dropIntoEditor", setter)]
    pub fn set_drop_into_editor(this: &IEditorOptions, drop_into_editor: IDropIntoEditorOptions);

    /**
         * Controls support for changing how content is pasted into the editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "pasteAs", getter)]
    pub fn paste_as(this: &IEditorOptions) -> IPasteAsOptions;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "pasteAs", setter)]
    pub fn set_paste_as(this: &IEditorOptions, paste_as: IPasteAsOptions);

    /**
         * Controls whether the editor receives tabs or defers them to the workbench for navigation.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "tabFocusMode", getter)]
    pub fn tab_focus_mode(this: &IEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorOptions", js_name = "tabFocusMode", setter)]
    pub fn set_tab_focus_mode(this: &IEditorOptions, tab_focus_mode: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDiffEditorBaseOptions;

    /**
         * Allow the user to resize the diff editor split view.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "enableSplitViewResizing", getter)]
    pub fn enable_split_view_resizing(this: &IDiffEditorBaseOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "enableSplitViewResizing", setter)]
    pub fn set_enable_split_view_resizing(this: &IDiffEditorBaseOptions, enable_split_view_resizing: bool);

    /**
         * The default ratio when rendering side-by-side editors.
         * Must be a number between 0 and 1, min sizes apply.
         * Defaults to 0.5
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "splitViewDefaultRatio", getter)]
    pub fn split_view_default_ratio(this: &IDiffEditorBaseOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "splitViewDefaultRatio", setter)]
    pub fn set_split_view_default_ratio(this: &IDiffEditorBaseOptions, split_view_default_ratio: f64);

    /**
         * Render the differences in two side-by-side editors.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "renderSideBySide", getter)]
    pub fn render_side_by_side(this: &IDiffEditorBaseOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "renderSideBySide", setter)]
    pub fn set_render_side_by_side(this: &IDiffEditorBaseOptions, render_side_by_side: bool);

    /**
         * Timeout in milliseconds after which diff computation is cancelled.
         * Defaults to 5000.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "maxComputationTime", getter)]
    pub fn max_computation_time(this: &IDiffEditorBaseOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "maxComputationTime", setter)]
    pub fn set_max_computation_time(this: &IDiffEditorBaseOptions, max_computation_time: f64);

    /**
         * Maximum supported file size in MB.
         * Defaults to 50.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "maxFileSize", getter)]
    pub fn max_file_size(this: &IDiffEditorBaseOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "maxFileSize", setter)]
    pub fn set_max_file_size(this: &IDiffEditorBaseOptions, max_file_size: f64);

    /**
         * Compute the diff by ignoring leading/trailing whitespace
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "ignoreTrimWhitespace", getter)]
    pub fn ignore_trim_whitespace(this: &IDiffEditorBaseOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "ignoreTrimWhitespace", setter)]
    pub fn set_ignore_trim_whitespace(this: &IDiffEditorBaseOptions, ignore_trim_whitespace: bool);

    /**
         * Render +/- indicators for added/deleted changes.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "renderIndicators", getter)]
    pub fn render_indicators(this: &IDiffEditorBaseOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "renderIndicators", setter)]
    pub fn set_render_indicators(this: &IDiffEditorBaseOptions, render_indicators: bool);

    /**
         * Shows icons in the glyph margin to revert changes.
         * Default to true.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "renderMarginRevertIcon", getter)]
    pub fn render_margin_revert_icon(this: &IDiffEditorBaseOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "renderMarginRevertIcon", setter)]
    pub fn set_render_margin_revert_icon(this: &IDiffEditorBaseOptions, render_margin_revert_icon: bool);

    /**
         * Original model should be editable?
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "originalEditable", getter)]
    pub fn original_editable(this: &IDiffEditorBaseOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "originalEditable", setter)]
    pub fn set_original_editable(this: &IDiffEditorBaseOptions, original_editable: bool);

    /**
         * Should the diff editor enable code lens?
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "diffCodeLens", getter)]
    pub fn diff_code_lens(this: &IDiffEditorBaseOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "diffCodeLens", setter)]
    pub fn set_diff_code_lens(this: &IDiffEditorBaseOptions, diff_code_lens: bool);

    /**
         * Is the diff editor should render overview ruler
         * Defaults to true
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "renderOverviewRuler", getter)]
    pub fn render_overview_ruler(this: &IDiffEditorBaseOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "renderOverviewRuler", setter)]
    pub fn set_render_overview_ruler(this: &IDiffEditorBaseOptions, render_overview_ruler: bool);

    /**
         * Control the wrapping of the diff editor.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "diffWordWrap", getter)]
    pub fn diff_word_wrap(this: &IDiffEditorBaseOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "diffWordWrap", setter)]
    pub fn set_diff_word_wrap(this: &IDiffEditorBaseOptions, diff_word_wrap: JsValue);

    /**
         * Diff Algorithm
        */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "diffAlgorithm", getter)]
    pub fn diff_algorithm(this: &IDiffEditorBaseOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "diffAlgorithm", setter)]
    pub fn set_diff_algorithm(this: &IDiffEditorBaseOptions, diff_algorithm: JsValue);

    /**
         * Whether the diff editor aria label should be verbose.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "accessibilityVerbose", getter)]
    pub fn accessibility_verbose(this: &IDiffEditorBaseOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "accessibilityVerbose", setter)]
    pub fn set_accessibility_verbose(this: &IDiffEditorBaseOptions, accessibility_verbose: bool);

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "experimental", getter)]
    pub fn experimental(this: &IDiffEditorBaseOptions) -> js_sys::Object;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "experimental", setter)]
    pub fn set_experimental(this: &IDiffEditorBaseOptions, experimental: js_sys::Object);

    /**
         * Is the diff editor inside another editor
         * Defaults to false
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "isInEmbeddedEditor", getter)]
    pub fn is_in_embedded_editor(this: &IDiffEditorBaseOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "isInEmbeddedEditor", setter)]
    pub fn set_is_in_embedded_editor(this: &IDiffEditorBaseOptions, is_in_embedded_editor: bool);

    /**
         * If the diff editor should only show the difference review mode.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "onlyShowAccessibleDiffViewer", getter)]
    pub fn only_show_accessible_diff_viewer(this: &IDiffEditorBaseOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDiffEditorBaseOptions", js_name = "onlyShowAccessibleDiffViewer", setter)]
    pub fn set_only_show_accessible_diff_viewer(this: &IDiffEditorBaseOptions, only_show_accessible_diff_viewer: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDiffEditorOptions;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ConfigurationChangedEvent;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn has_changed(this: &ConfigurationChangedEvent, id: EditorOption, ) -> bool;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IComputedEditorOptions;

    #[wasm_bindgen(method, js_class = "IComputedEditorOptions", js_name = "get")]
    pub fn get(this: &IComputedEditorOptions, id: JsValue, ) -> FindComputedEditorOptionValueById;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorOption;

    #[wasm_bindgen(method, js_class = "IEditorOption", js_name = "id", getter)]
    pub fn id(this: &IEditorOption) -> JsValue;
    #[wasm_bindgen(method, js_class = "IEditorOption", js_name = "name", getter)]
    pub fn name(this: &IEditorOption) -> String;
    #[wasm_bindgen(method, js_class = "IEditorOption", js_name = "defaultValue", getter)]
    pub fn default_value(this: &IEditorOption) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorOption", js_name = "defaultValue", setter)]
    pub fn set_default_value(this: &IEditorOption, default_value: JsValue);

    /**
         * Might modify `value`.
        */ 
    #[wasm_bindgen(method, js_class = "IEditorOption", js_name = "applyUpdate")]
    pub fn apply_update(this: &IEditorOption, value: JsValue, update: JsValue, ) -> ApplyUpdateResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ApplyUpdateResult;

  #[wasm_bindgen(constructor, js_class = "ApplyUpdateResult")]
  pub fn new() -> ApplyUpdateResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorCommentsOptions;

    /**
         * Insert a space after the line comment token and inside the block comments tokens.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorCommentsOptions", js_name = "insertSpace", getter)]
    pub fn insert_space(this: &IEditorCommentsOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorCommentsOptions", js_name = "insertSpace", setter)]
    pub fn set_insert_space(this: &IEditorCommentsOptions, insert_space: bool);

    /**
         * Ignore empty lines when inserting line comments.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorCommentsOptions", js_name = "ignoreEmptyLines", getter)]
    pub fn ignore_empty_lines(this: &IEditorCommentsOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorCommentsOptions", js_name = "ignoreEmptyLines", setter)]
    pub fn set_ignore_empty_lines(this: &IEditorCommentsOptions, ignore_empty_lines: bool);


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum TextEditorCursorBlinkingStyle {
    /**
         * Hidden
         */ 
    Hidden = 0,
    /**
         * Blinking
         */ 
    Blink = 1,
    /**
         * Blinking with smooth fading
         */ 
    Smooth = 2,
    /**
         * Blinking with prolonged filled state and smooth fading
         */ 
    Phase = 3,
    /**
         * Expand collapse animation on the y axis
         */ 
    Expand = 4,
    /**
         * No-Blinking
         */ 
    Solid = 5,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum TextEditorCursorStyle {
    /**
         * As a vertical line (sitting between two characters).
         */ 
    Line = 1,
    /**
         * As a block (sitting on top of a character).
         */ 
    Block = 2,
    /**
         * As a horizontal line (sitting under a character).
         */ 
    Underline = 3,
    /**
         * As a thin vertical line (sitting between two characters).
         */ 
    LineThin = 4,
    /**
         * As an outlined block (sitting on top of a character).
         */ 
    BlockOutline = 5,
    /**
         * As a thin horizontal line (sitting under a character).
         */ 
    UnderlineThin = 6,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorFindOptions;

    /**
        * Controls whether the cursor should move to find matches while typing.
        */ 
    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "cursorMoveOnType", getter)]
    pub fn cursor_move_on_type(this: &IEditorFindOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "cursorMoveOnType", setter)]
    pub fn set_cursor_move_on_type(this: &IEditorFindOptions, cursor_move_on_type: bool);

    /**
         * Controls if we seed search string in the Find Widget with editor selection.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "seedSearchStringFromSelection", getter)]
    pub fn seed_search_string_from_selection(this: &IEditorFindOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "seedSearchStringFromSelection", setter)]
    pub fn set_seed_search_string_from_selection(this: &IEditorFindOptions, seed_search_string_from_selection: JsValue);

    /**
         * Controls if Find in Selection flag is turned on in the editor.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "autoFindInSelection", getter)]
    pub fn auto_find_in_selection(this: &IEditorFindOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "autoFindInSelection", setter)]
    pub fn set_auto_find_in_selection(this: &IEditorFindOptions, auto_find_in_selection: JsValue);

    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "addExtraSpaceOnTop", getter)]
    pub fn add_extra_space_on_top(this: &IEditorFindOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "addExtraSpaceOnTop", setter)]
    pub fn set_add_extra_space_on_top(this: &IEditorFindOptions, add_extra_space_on_top: bool);

    /**
         * Controls whether the search result and diff result automatically restarts from the beginning (or the end) when no further matches can be found
         */ 
    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "loop", getter)]
    pub fn r#loop(this: &IEditorFindOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorFindOptions", js_name = "loop", setter)]
    pub fn set_loop(this: &IEditorFindOptions, r#loop: bool);


}

pub type GoToLocationValues = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IGotoLocationOptions;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multiple", getter)]
    pub fn multiple(this: &IGotoLocationOptions) -> GoToLocationValues;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multiple", setter)]
    pub fn set_multiple(this: &IGotoLocationOptions, multiple: GoToLocationValues);

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleDefinitions", getter)]
    pub fn multiple_definitions(this: &IGotoLocationOptions) -> GoToLocationValues;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleDefinitions", setter)]
    pub fn set_multiple_definitions(this: &IGotoLocationOptions, multiple_definitions: GoToLocationValues);

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleTypeDefinitions", getter)]
    pub fn multiple_type_definitions(this: &IGotoLocationOptions) -> GoToLocationValues;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleTypeDefinitions", setter)]
    pub fn set_multiple_type_definitions(this: &IGotoLocationOptions, multiple_type_definitions: GoToLocationValues);

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleDeclarations", getter)]
    pub fn multiple_declarations(this: &IGotoLocationOptions) -> GoToLocationValues;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleDeclarations", setter)]
    pub fn set_multiple_declarations(this: &IGotoLocationOptions, multiple_declarations: GoToLocationValues);

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleImplementations", getter)]
    pub fn multiple_implementations(this: &IGotoLocationOptions) -> GoToLocationValues;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleImplementations", setter)]
    pub fn set_multiple_implementations(this: &IGotoLocationOptions, multiple_implementations: GoToLocationValues);

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleReferences", getter)]
    pub fn multiple_references(this: &IGotoLocationOptions) -> GoToLocationValues;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "multipleReferences", setter)]
    pub fn set_multiple_references(this: &IGotoLocationOptions, multiple_references: GoToLocationValues);

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeDefinitionCommand", getter)]
    pub fn alternative_definition_command(this: &IGotoLocationOptions) -> String;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeDefinitionCommand", setter)]
    pub fn set_alternative_definition_command(this: &IGotoLocationOptions, alternative_definition_command: String);

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeTypeDefinitionCommand", getter)]
    pub fn alternative_type_definition_command(this: &IGotoLocationOptions) -> String;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeTypeDefinitionCommand", setter)]
    pub fn set_alternative_type_definition_command(this: &IGotoLocationOptions, alternative_type_definition_command: String);

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeDeclarationCommand", getter)]
    pub fn alternative_declaration_command(this: &IGotoLocationOptions) -> String;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeDeclarationCommand", setter)]
    pub fn set_alternative_declaration_command(this: &IGotoLocationOptions, alternative_declaration_command: String);

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeImplementationCommand", getter)]
    pub fn alternative_implementation_command(this: &IGotoLocationOptions) -> String;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeImplementationCommand", setter)]
    pub fn set_alternative_implementation_command(this: &IGotoLocationOptions, alternative_implementation_command: String);

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeReferenceCommand", getter)]
    pub fn alternative_reference_command(this: &IGotoLocationOptions) -> String;

    #[wasm_bindgen(method, js_class = "IGotoLocationOptions", js_name = "alternativeReferenceCommand", setter)]
    pub fn set_alternative_reference_command(this: &IGotoLocationOptions, alternative_reference_command: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorHoverOptions;

    /**
         * Enable the hover.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "enabled", getter)]
    pub fn enabled(this: &IEditorHoverOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "enabled", setter)]
    pub fn set_enabled(this: &IEditorHoverOptions, enabled: bool);

    /**
         * Delay for showing the hover.
         * Defaults to 300.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "delay", getter)]
    pub fn delay(this: &IEditorHoverOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "delay", setter)]
    pub fn set_delay(this: &IEditorHoverOptions, delay: f64);

    /**
         * Is the hover sticky such that it can be clicked and its contents selected?
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "sticky", getter)]
    pub fn sticky(this: &IEditorHoverOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "sticky", setter)]
    pub fn set_sticky(this: &IEditorHoverOptions, sticky: bool);

    /**
         * Should the hover be shown above the line if possible?
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "above", getter)]
    pub fn above(this: &IEditorHoverOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorHoverOptions", js_name = "above", setter)]
    pub fn set_above(this: &IEditorHoverOptions, above: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type OverviewRulerPosition;

    /**
         * Width of the overview ruler
         */ 
    #[wasm_bindgen(method, js_class = "OverviewRulerPosition", js_name = "width", getter)]
    pub fn width(this: &OverviewRulerPosition) -> f64;
    /**
         * Height of the overview ruler
         */ 
    #[wasm_bindgen(method, js_class = "OverviewRulerPosition", js_name = "height", getter)]
    pub fn height(this: &OverviewRulerPosition) -> f64;
    /**
         * Top position for the overview ruler
         */ 
    #[wasm_bindgen(method, js_class = "OverviewRulerPosition", js_name = "top", getter)]
    pub fn top(this: &OverviewRulerPosition) -> f64;
    /**
         * Right position for the overview ruler
         */ 
    #[wasm_bindgen(method, js_class = "OverviewRulerPosition", js_name = "right", getter)]
    pub fn right(this: &OverviewRulerPosition) -> f64;

}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum RenderMinimap {
    None = 0,
    Text = 1,
    Blocks = 2,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type EditorLayoutInfo;

    /**
         * Full editor width.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "width", getter)]
    pub fn width(this: &EditorLayoutInfo) -> f64;
    /**
         * Full editor height.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "height", getter)]
    pub fn height(this: &EditorLayoutInfo) -> f64;
    /**
         * Left position for the glyph margin.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "glyphMarginLeft", getter)]
    pub fn glyph_margin_left(this: &EditorLayoutInfo) -> f64;
    /**
         * The width of the glyph margin.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "glyphMarginWidth", getter)]
    pub fn glyph_margin_width(this: &EditorLayoutInfo) -> f64;
    /**
         * The number of decoration lanes to render in the glyph margin.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "glyphMarginDecorationLaneCount", getter)]
    pub fn glyph_margin_decoration_lane_count(this: &EditorLayoutInfo) -> f64;
    /**
         * Left position for the line numbers.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "lineNumbersLeft", getter)]
    pub fn line_numbers_left(this: &EditorLayoutInfo) -> f64;
    /**
         * The width of the line numbers.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "lineNumbersWidth", getter)]
    pub fn line_numbers_width(this: &EditorLayoutInfo) -> f64;
    /**
         * Left position for the line decorations.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "decorationsLeft", getter)]
    pub fn decorations_left(this: &EditorLayoutInfo) -> f64;
    /**
         * The width of the line decorations.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "decorationsWidth", getter)]
    pub fn decorations_width(this: &EditorLayoutInfo) -> f64;
    /**
         * Left position for the content (actual text)
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "contentLeft", getter)]
    pub fn content_left(this: &EditorLayoutInfo) -> f64;
    /**
         * The width of the content (actual text)
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "contentWidth", getter)]
    pub fn content_width(this: &EditorLayoutInfo) -> f64;
    /**
         * Layout information for the minimap
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "minimap", getter)]
    pub fn minimap(this: &EditorLayoutInfo) -> EditorMinimapLayoutInfo;
    /**
         * The number of columns (of typical characters) fitting on a viewport line.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "viewportColumn", getter)]
    pub fn viewport_column(this: &EditorLayoutInfo) -> f64;
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "isWordWrapMinified", getter)]
    pub fn is_word_wrap_minified(this: &EditorLayoutInfo) -> bool;
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "isViewportWrapping", getter)]
    pub fn is_viewport_wrapping(this: &EditorLayoutInfo) -> bool;
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "wrappingColumn", getter)]
    pub fn wrapping_column(this: &EditorLayoutInfo) -> f64;
    /**
         * The width of the vertical scrollbar.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "verticalScrollbarWidth", getter)]
    pub fn vertical_scrollbar_width(this: &EditorLayoutInfo) -> f64;
    /**
         * The height of the horizontal scrollbar.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "horizontalScrollbarHeight", getter)]
    pub fn horizontal_scrollbar_height(this: &EditorLayoutInfo) -> f64;
    /**
         * The position of the overview ruler.
         */ 
    #[wasm_bindgen(method, js_class = "EditorLayoutInfo", js_name = "overviewRuler", getter)]
    pub fn overview_ruler(this: &EditorLayoutInfo) -> OverviewRulerPosition;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type EditorMinimapLayoutInfo;

    #[wasm_bindgen(method, js_class = "EditorMinimapLayoutInfo", js_name = "renderMinimap", getter)]
    pub fn render_minimap(this: &EditorMinimapLayoutInfo) -> RenderMinimap;
    #[wasm_bindgen(method, js_class = "EditorMinimapLayoutInfo", js_name = "minimapLeft", getter)]
    pub fn minimap_left(this: &EditorMinimapLayoutInfo) -> f64;
    #[wasm_bindgen(method, js_class = "EditorMinimapLayoutInfo", js_name = "minimapWidth", getter)]
    pub fn minimap_width(this: &EditorMinimapLayoutInfo) -> f64;
    #[wasm_bindgen(method, js_class = "EditorMinimapLayoutInfo", js_name = "minimapHeightIsEditorHeight", getter)]
    pub fn minimap_height_is_editor_height(this: &EditorMinimapLayoutInfo) -> bool;
    #[wasm_bindgen(method, js_class = "EditorMinimapLayoutInfo", js_name = "minimapIsSampling", getter)]
    pub fn minimap_is_sampling(this: &EditorMinimapLayoutInfo) -> bool;
    #[wasm_bindgen(method, js_class = "EditorMinimapLayoutInfo", js_name = "minimapScale", getter)]
    pub fn minimap_scale(this: &EditorMinimapLayoutInfo) -> f64;
    #[wasm_bindgen(method, js_class = "EditorMinimapLayoutInfo", js_name = "minimapLineHeight", getter)]
    pub fn minimap_line_height(this: &EditorMinimapLayoutInfo) -> f64;
    #[wasm_bindgen(method, js_class = "EditorMinimapLayoutInfo", js_name = "minimapCanvasInnerWidth", getter)]
    pub fn minimap_canvas_inner_width(this: &EditorMinimapLayoutInfo) -> f64;
    #[wasm_bindgen(method, js_class = "EditorMinimapLayoutInfo", js_name = "minimapCanvasInnerHeight", getter)]
    pub fn minimap_canvas_inner_height(this: &EditorMinimapLayoutInfo) -> f64;
    #[wasm_bindgen(method, js_class = "EditorMinimapLayoutInfo", js_name = "minimapCanvasOuterWidth", getter)]
    pub fn minimap_canvas_outer_width(this: &EditorMinimapLayoutInfo) -> f64;
    #[wasm_bindgen(method, js_class = "EditorMinimapLayoutInfo", js_name = "minimapCanvasOuterHeight", getter)]
    pub fn minimap_canvas_outer_height(this: &EditorMinimapLayoutInfo) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorLightbulbOptions;

    /**
         * Enable the lightbulb code action.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorLightbulbOptions", js_name = "enabled", getter)]
    pub fn enabled(this: &IEditorLightbulbOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorLightbulbOptions", js_name = "enabled", setter)]
    pub fn set_enabled(this: &IEditorLightbulbOptions, enabled: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorStickyScrollOptions;

    /**
         * Enable the sticky scroll
         */ 
    #[wasm_bindgen(method, js_class = "IEditorStickyScrollOptions", js_name = "enabled", getter)]
    pub fn enabled(this: &IEditorStickyScrollOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorStickyScrollOptions", js_name = "enabled", setter)]
    pub fn set_enabled(this: &IEditorStickyScrollOptions, enabled: bool);

    /**
         * Maximum number of sticky lines to show
         */ 
    #[wasm_bindgen(method, js_class = "IEditorStickyScrollOptions", js_name = "maxLineCount", getter)]
    pub fn max_line_count(this: &IEditorStickyScrollOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorStickyScrollOptions", js_name = "maxLineCount", setter)]
    pub fn set_max_line_count(this: &IEditorStickyScrollOptions, max_line_count: f64);

    /**
         * Model to choose for sticky scroll by default
         */ 
    #[wasm_bindgen(method, js_class = "IEditorStickyScrollOptions", js_name = "defaultModel", getter)]
    pub fn default_model(this: &IEditorStickyScrollOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorStickyScrollOptions", js_name = "defaultModel", setter)]
    pub fn set_default_model(this: &IEditorStickyScrollOptions, default_model: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorInlayHintsOptions;

    /**
         * Enable the inline hints.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorInlayHintsOptions", js_name = "enabled", getter)]
    pub fn enabled(this: &IEditorInlayHintsOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorInlayHintsOptions", js_name = "enabled", setter)]
    pub fn set_enabled(this: &IEditorInlayHintsOptions, enabled: JsValue);

    /**
         * Font size of inline hints.
         * Default to 90% of the editor font size.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorInlayHintsOptions", js_name = "fontSize", getter)]
    pub fn font_size(this: &IEditorInlayHintsOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorInlayHintsOptions", js_name = "fontSize", setter)]
    pub fn set_font_size(this: &IEditorInlayHintsOptions, font_size: f64);

    /**
         * Font family of inline hints.
         * Defaults to editor font family.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorInlayHintsOptions", js_name = "fontFamily", getter)]
    pub fn font_family(this: &IEditorInlayHintsOptions) -> String;

    #[wasm_bindgen(method, js_class = "IEditorInlayHintsOptions", js_name = "fontFamily", setter)]
    pub fn set_font_family(this: &IEditorInlayHintsOptions, font_family: String);

    /**
         * Enables the padding around the inlay hint.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorInlayHintsOptions", js_name = "padding", getter)]
    pub fn padding(this: &IEditorInlayHintsOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorInlayHintsOptions", js_name = "padding", setter)]
    pub fn set_padding(this: &IEditorInlayHintsOptions, padding: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorMinimapOptions;

    /**
         * Enable the rendering of the minimap.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "enabled", getter)]
    pub fn enabled(this: &IEditorMinimapOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "enabled", setter)]
    pub fn set_enabled(this: &IEditorMinimapOptions, enabled: bool);

    /**
         * Control the rendering of minimap.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "autohide", getter)]
    pub fn autohide(this: &IEditorMinimapOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "autohide", setter)]
    pub fn set_autohide(this: &IEditorMinimapOptions, autohide: bool);

    /**
         * Control the side of the minimap in editor.
         * Defaults to 'right'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "side", getter)]
    pub fn side(this: &IEditorMinimapOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "side", setter)]
    pub fn set_side(this: &IEditorMinimapOptions, side: JsValue);

    /**
         * Control the minimap rendering mode.
         * Defaults to 'actual'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "size", getter)]
    pub fn size(this: &IEditorMinimapOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "size", setter)]
    pub fn set_size(this: &IEditorMinimapOptions, size: JsValue);

    /**
         * Control the rendering of the minimap slider.
         * Defaults to 'mouseover'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "showSlider", getter)]
    pub fn show_slider(this: &IEditorMinimapOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "showSlider", setter)]
    pub fn set_show_slider(this: &IEditorMinimapOptions, show_slider: JsValue);

    /**
         * Render the actual text on a line (as opposed to color blocks).
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "renderCharacters", getter)]
    pub fn render_characters(this: &IEditorMinimapOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "renderCharacters", setter)]
    pub fn set_render_characters(this: &IEditorMinimapOptions, render_characters: bool);

    /**
         * Limit the width of the minimap to render at most a certain number of columns.
         * Defaults to 120.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "maxColumn", getter)]
    pub fn max_column(this: &IEditorMinimapOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "maxColumn", setter)]
    pub fn set_max_column(this: &IEditorMinimapOptions, max_column: f64);

    /**
         * Relative size of the font in the minimap. Defaults to 1.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "scale", getter)]
    pub fn scale(this: &IEditorMinimapOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorMinimapOptions", js_name = "scale", setter)]
    pub fn set_scale(this: &IEditorMinimapOptions, scale: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorPaddingOptions;

    /**
         * Spacing between top edge of editor and first line.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorPaddingOptions", js_name = "top", getter)]
    pub fn top(this: &IEditorPaddingOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorPaddingOptions", js_name = "top", setter)]
    pub fn set_top(this: &IEditorPaddingOptions, top: f64);

    /**
         * Spacing between bottom edge of editor and last line.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorPaddingOptions", js_name = "bottom", getter)]
    pub fn bottom(this: &IEditorPaddingOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorPaddingOptions", js_name = "bottom", setter)]
    pub fn set_bottom(this: &IEditorPaddingOptions, bottom: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorParameterHintOptions;

    /**
         * Enable parameter hints.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorParameterHintOptions", js_name = "enabled", getter)]
    pub fn enabled(this: &IEditorParameterHintOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorParameterHintOptions", js_name = "enabled", setter)]
    pub fn set_enabled(this: &IEditorParameterHintOptions, enabled: bool);

    /**
         * Enable cycling of parameter hints.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorParameterHintOptions", js_name = "cycle", getter)]
    pub fn cycle(this: &IEditorParameterHintOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorParameterHintOptions", js_name = "cycle", setter)]
    pub fn set_cycle(this: &IEditorParameterHintOptions, cycle: bool);


}

pub type QuickSuggestionsValue = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IQuickSuggestionsOptions;

    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "other", getter)]
    pub fn other(this: &IQuickSuggestionsOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "other", setter)]
    pub fn set_other(this: &IQuickSuggestionsOptions, other: JsValue);

    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "comments", getter)]
    pub fn comments(this: &IQuickSuggestionsOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "comments", setter)]
    pub fn set_comments(this: &IQuickSuggestionsOptions, comments: JsValue);

    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "strings", getter)]
    pub fn strings(this: &IQuickSuggestionsOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IQuickSuggestionsOptions", js_name = "strings", setter)]
    pub fn set_strings(this: &IQuickSuggestionsOptions, strings: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InternalQuickSuggestionsOptions;

    #[wasm_bindgen(method, js_class = "InternalQuickSuggestionsOptions", js_name = "other", getter)]
    pub fn other(this: &InternalQuickSuggestionsOptions) -> QuickSuggestionsValue;
    #[wasm_bindgen(method, js_class = "InternalQuickSuggestionsOptions", js_name = "comments", getter)]
    pub fn comments(this: &InternalQuickSuggestionsOptions) -> QuickSuggestionsValue;
    #[wasm_bindgen(method, js_class = "InternalQuickSuggestionsOptions", js_name = "strings", getter)]
    pub fn strings(this: &InternalQuickSuggestionsOptions) -> QuickSuggestionsValue;

}

pub type LineNumbersType = JsValue
;

            int_enum! {
#[allow(non_camel_case_types)]
pub enum RenderLineNumbersType {
    Off = 0,
    On = 1,
    Relative = 2,
    Interval = 3,
    Custom = 4,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InternalEditorRenderLineNumbersOptions;

    #[wasm_bindgen(method, js_class = "InternalEditorRenderLineNumbersOptions", js_name = "renderType", getter)]
    pub fn render_type(this: &InternalEditorRenderLineNumbersOptions) -> RenderLineNumbersType;
    #[wasm_bindgen(method, js_class = "InternalEditorRenderLineNumbersOptions", js_name = "renderFn", getter)]
    pub fn render_fn(this: &InternalEditorRenderLineNumbersOptions) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IRulerOption;

    #[wasm_bindgen(method, js_class = "IRulerOption", js_name = "column", getter)]
    pub fn column(this: &IRulerOption) -> f64;
    #[wasm_bindgen(method, js_class = "IRulerOption", js_name = "color", getter)]
    pub fn color(this: &IRulerOption) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorScrollbarOptions;

    /**
         * The size of arrows (if displayed).
         * Defaults to 11.
         * **NOTE**: This option cannot be updated using `updateOptions()`
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "arrowSize", getter)]
    pub fn arrow_size(this: &IEditorScrollbarOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "arrowSize", setter)]
    pub fn set_arrow_size(this: &IEditorScrollbarOptions, arrow_size: f64);

    /**
         * Render vertical scrollbar.
         * Defaults to 'auto'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "vertical", getter)]
    pub fn vertical(this: &IEditorScrollbarOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "vertical", setter)]
    pub fn set_vertical(this: &IEditorScrollbarOptions, vertical: JsValue);

    /**
         * Render horizontal scrollbar.
         * Defaults to 'auto'.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontal", getter)]
    pub fn horizontal(this: &IEditorScrollbarOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontal", setter)]
    pub fn set_horizontal(this: &IEditorScrollbarOptions, horizontal: JsValue);

    /**
         * Cast horizontal and vertical shadows when the content is scrolled.
         * Defaults to true.
         * **NOTE**: This option cannot be updated using `updateOptions()`
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "useShadows", getter)]
    pub fn use_shadows(this: &IEditorScrollbarOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "useShadows", setter)]
    pub fn set_use_shadows(this: &IEditorScrollbarOptions, use_shadows: bool);

    /**
         * Render arrows at the top and bottom of the vertical scrollbar.
         * Defaults to false.
         * **NOTE**: This option cannot be updated using `updateOptions()`
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalHasArrows", getter)]
    pub fn vertical_has_arrows(this: &IEditorScrollbarOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalHasArrows", setter)]
    pub fn set_vertical_has_arrows(this: &IEditorScrollbarOptions, vertical_has_arrows: bool);

    /**
         * Render arrows at the left and right of the horizontal scrollbar.
         * Defaults to false.
         * **NOTE**: This option cannot be updated using `updateOptions()`
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalHasArrows", getter)]
    pub fn horizontal_has_arrows(this: &IEditorScrollbarOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalHasArrows", setter)]
    pub fn set_horizontal_has_arrows(this: &IEditorScrollbarOptions, horizontal_has_arrows: bool);

    /**
         * Listen to mouse wheel events and react to them by scrolling.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "handleMouseWheel", getter)]
    pub fn handle_mouse_wheel(this: &IEditorScrollbarOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "handleMouseWheel", setter)]
    pub fn set_handle_mouse_wheel(this: &IEditorScrollbarOptions, handle_mouse_wheel: bool);

    /**
         * Always consume mouse wheel events (always call preventDefault() and stopPropagation() on the browser events).
         * Defaults to true.
         * **NOTE**: This option cannot be updated using `updateOptions()`
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "alwaysConsumeMouseWheel", getter)]
    pub fn always_consume_mouse_wheel(this: &IEditorScrollbarOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "alwaysConsumeMouseWheel", setter)]
    pub fn set_always_consume_mouse_wheel(this: &IEditorScrollbarOptions, always_consume_mouse_wheel: bool);

    /**
         * Height in pixels for the horizontal scrollbar.
         * Defaults to 10 (px).
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalScrollbarSize", getter)]
    pub fn horizontal_scrollbar_size(this: &IEditorScrollbarOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalScrollbarSize", setter)]
    pub fn set_horizontal_scrollbar_size(this: &IEditorScrollbarOptions, horizontal_scrollbar_size: f64);

    /**
         * Width in pixels for the vertical scrollbar.
         * Defaults to 10 (px).
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalScrollbarSize", getter)]
    pub fn vertical_scrollbar_size(this: &IEditorScrollbarOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalScrollbarSize", setter)]
    pub fn set_vertical_scrollbar_size(this: &IEditorScrollbarOptions, vertical_scrollbar_size: f64);

    /**
         * Width in pixels for the vertical slider.
         * Defaults to `verticalScrollbarSize`.
         * **NOTE**: This option cannot be updated using `updateOptions()`
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalSliderSize", getter)]
    pub fn vertical_slider_size(this: &IEditorScrollbarOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "verticalSliderSize", setter)]
    pub fn set_vertical_slider_size(this: &IEditorScrollbarOptions, vertical_slider_size: f64);

    /**
         * Height in pixels for the horizontal slider.
         * Defaults to `horizontalScrollbarSize`.
         * **NOTE**: This option cannot be updated using `updateOptions()`
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalSliderSize", getter)]
    pub fn horizontal_slider_size(this: &IEditorScrollbarOptions) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "horizontalSliderSize", setter)]
    pub fn set_horizontal_slider_size(this: &IEditorScrollbarOptions, horizontal_slider_size: f64);

    /**
         * Scroll gutter clicks move by page vs jump to position.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "scrollByPage", getter)]
    pub fn scroll_by_page(this: &IEditorScrollbarOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IEditorScrollbarOptions", js_name = "scrollByPage", setter)]
    pub fn set_scroll_by_page(this: &IEditorScrollbarOptions, scroll_by_page: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InternalEditorScrollbarOptions;

    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "arrowSize", getter)]
    pub fn arrow_size(this: &InternalEditorScrollbarOptions) -> f64;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "vertical", getter)]
    pub fn vertical(this: &InternalEditorScrollbarOptions) -> ScrollbarVisibility;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "horizontal", getter)]
    pub fn horizontal(this: &InternalEditorScrollbarOptions) -> ScrollbarVisibility;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "useShadows", getter)]
    pub fn use_shadows(this: &InternalEditorScrollbarOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "verticalHasArrows", getter)]
    pub fn vertical_has_arrows(this: &InternalEditorScrollbarOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "horizontalHasArrows", getter)]
    pub fn horizontal_has_arrows(this: &InternalEditorScrollbarOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "handleMouseWheel", getter)]
    pub fn handle_mouse_wheel(this: &InternalEditorScrollbarOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "alwaysConsumeMouseWheel", getter)]
    pub fn always_consume_mouse_wheel(this: &InternalEditorScrollbarOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "horizontalScrollbarSize", getter)]
    pub fn horizontal_scrollbar_size(this: &InternalEditorScrollbarOptions) -> f64;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "horizontalSliderSize", getter)]
    pub fn horizontal_slider_size(this: &InternalEditorScrollbarOptions) -> f64;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "verticalScrollbarSize", getter)]
    pub fn vertical_scrollbar_size(this: &InternalEditorScrollbarOptions) -> f64;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "verticalSliderSize", getter)]
    pub fn vertical_slider_size(this: &InternalEditorScrollbarOptions) -> f64;
    #[wasm_bindgen(method, js_class = "InternalEditorScrollbarOptions", js_name = "scrollByPage", getter)]
    pub fn scroll_by_page(this: &InternalEditorScrollbarOptions) -> bool;

}

pub type InUntrustedWorkspace = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IUnicodeHighlightOptions;

    /**
         * Controls whether all non-basic ASCII characters are highlighted. Only characters between U+0020 and U+007E, tab, line-feed and carriage-return are considered basic ASCII.
         */ 
    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "nonBasicASCII", getter)]
    pub fn non_basic_ascii(this: &IUnicodeHighlightOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "nonBasicASCII", setter)]
    pub fn set_non_basic_ascii(this: &IUnicodeHighlightOptions, non_basic_ascii: JsValue);

    /**
         * Controls whether characters that just reserve space or have no width at all are highlighted.
         */ 
    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "invisibleCharacters", getter)]
    pub fn invisible_characters(this: &IUnicodeHighlightOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "invisibleCharacters", setter)]
    pub fn set_invisible_characters(this: &IUnicodeHighlightOptions, invisible_characters: bool);

    /**
         * Controls whether characters are highlighted that can be confused with basic ASCII characters, except those that are common in the current user locale.
         */ 
    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "ambiguousCharacters", getter)]
    pub fn ambiguous_characters(this: &IUnicodeHighlightOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "ambiguousCharacters", setter)]
    pub fn set_ambiguous_characters(this: &IUnicodeHighlightOptions, ambiguous_characters: bool);

    /**
         * Controls whether characters in comments should also be subject to unicode highlighting.
         */ 
    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "includeComments", getter)]
    pub fn include_comments(this: &IUnicodeHighlightOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "includeComments", setter)]
    pub fn set_include_comments(this: &IUnicodeHighlightOptions, include_comments: JsValue);

    /**
         * Controls whether characters in strings should also be subject to unicode highlighting.
         */ 
    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "includeStrings", getter)]
    pub fn include_strings(this: &IUnicodeHighlightOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "includeStrings", setter)]
    pub fn set_include_strings(this: &IUnicodeHighlightOptions, include_strings: JsValue);

    /**
         * Defines allowed characters that are not being highlighted.
         */ 
    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "allowedCharacters", getter)]
    pub fn allowed_characters(this: &IUnicodeHighlightOptions) -> Record;

    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "allowedCharacters", setter)]
    pub fn set_allowed_characters(this: &IUnicodeHighlightOptions, allowed_characters: Record);

    /**
         * Unicode characters that are common in allowed locales are not being highlighted.
         */ 
    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "allowedLocales", getter)]
    pub fn allowed_locales(this: &IUnicodeHighlightOptions) -> Record;

    #[wasm_bindgen(method, js_class = "IUnicodeHighlightOptions", js_name = "allowedLocales", setter)]
    pub fn set_allowed_locales(this: &IUnicodeHighlightOptions, allowed_locales: Record);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IInlineSuggestOptions;

    /**
         * Enable or disable the rendering of automatic inline completions.
        */ 
    #[wasm_bindgen(method, js_class = "IInlineSuggestOptions", js_name = "enabled", getter)]
    pub fn enabled(this: &IInlineSuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IInlineSuggestOptions", js_name = "enabled", setter)]
    pub fn set_enabled(this: &IInlineSuggestOptions, enabled: bool);

    /**
         * Configures the mode.
         * Use `prefix` to only show ghost text if the text to replace is a prefix of the suggestion text.
         * Use `subword` to only show ghost text if the replace text is a subword of the suggestion text.
         * Use `subwordSmart` to only show ghost text if the replace text is a subword of the suggestion text, but the subword must start after the cursor position.
         * Defaults to `prefix`.
        */ 
    #[wasm_bindgen(method, js_class = "IInlineSuggestOptions", js_name = "mode", getter)]
    pub fn mode(this: &IInlineSuggestOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IInlineSuggestOptions", js_name = "mode", setter)]
    pub fn set_mode(this: &IInlineSuggestOptions, mode: JsValue);

    #[wasm_bindgen(method, js_class = "IInlineSuggestOptions", js_name = "showToolbar", getter)]
    pub fn show_toolbar(this: &IInlineSuggestOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IInlineSuggestOptions", js_name = "showToolbar", setter)]
    pub fn set_show_toolbar(this: &IInlineSuggestOptions, show_toolbar: JsValue);

    #[wasm_bindgen(method, js_class = "IInlineSuggestOptions", js_name = "suppressSuggestions", getter)]
    pub fn suppress_suggestions(this: &IInlineSuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IInlineSuggestOptions", js_name = "suppressSuggestions", setter)]
    pub fn set_suppress_suggestions(this: &IInlineSuggestOptions, suppress_suggestions: bool);

    /**
         * Does not clear active inline suggestions when the editor loses focus.
         */ 
    #[wasm_bindgen(method, js_class = "IInlineSuggestOptions", js_name = "keepOnBlur", getter)]
    pub fn keep_on_blur(this: &IInlineSuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IInlineSuggestOptions", js_name = "keepOnBlur", setter)]
    pub fn set_keep_on_blur(this: &IInlineSuggestOptions, keep_on_blur: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IBracketPairColorizationOptions;

    /**
         * Enable or disable bracket pair colorization.
        */ 
    #[wasm_bindgen(method, js_class = "IBracketPairColorizationOptions", js_name = "enabled", getter)]
    pub fn enabled(this: &IBracketPairColorizationOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IBracketPairColorizationOptions", js_name = "enabled", setter)]
    pub fn set_enabled(this: &IBracketPairColorizationOptions, enabled: bool);

    /**
         * Use independent color pool per bracket type.
        */ 
    #[wasm_bindgen(method, js_class = "IBracketPairColorizationOptions", js_name = "independentColorPoolPerBracketType", getter)]
    pub fn independent_color_pool_per_bracket_type(this: &IBracketPairColorizationOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IBracketPairColorizationOptions", js_name = "independentColorPoolPerBracketType", setter)]
    pub fn set_independent_color_pool_per_bracket_type(this: &IBracketPairColorizationOptions, independent_color_pool_per_bracket_type: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IGuidesOptions;

    /**
         * Enable rendering of bracket pair guides.
         * Defaults to false.
        */ 
    #[wasm_bindgen(method, js_class = "IGuidesOptions", js_name = "bracketPairs", getter)]
    pub fn bracket_pairs(this: &IGuidesOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IGuidesOptions", js_name = "bracketPairs", setter)]
    pub fn set_bracket_pairs(this: &IGuidesOptions, bracket_pairs: JsValue);

    /**
         * Enable rendering of vertical bracket pair guides.
         * Defaults to 'active'.
         */ 
    #[wasm_bindgen(method, js_class = "IGuidesOptions", js_name = "bracketPairsHorizontal", getter)]
    pub fn bracket_pairs_horizontal(this: &IGuidesOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IGuidesOptions", js_name = "bracketPairsHorizontal", setter)]
    pub fn set_bracket_pairs_horizontal(this: &IGuidesOptions, bracket_pairs_horizontal: JsValue);

    /**
         * Enable highlighting of the active bracket pair.
         * Defaults to true.
        */ 
    #[wasm_bindgen(method, js_class = "IGuidesOptions", js_name = "highlightActiveBracketPair", getter)]
    pub fn highlight_active_bracket_pair(this: &IGuidesOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IGuidesOptions", js_name = "highlightActiveBracketPair", setter)]
    pub fn set_highlight_active_bracket_pair(this: &IGuidesOptions, highlight_active_bracket_pair: bool);

    /**
         * Enable rendering of indent guides.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IGuidesOptions", js_name = "indentation", getter)]
    pub fn indentation(this: &IGuidesOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IGuidesOptions", js_name = "indentation", setter)]
    pub fn set_indentation(this: &IGuidesOptions, indentation: bool);

    /**
         * Enable highlighting of the active indent guide.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IGuidesOptions", js_name = "highlightActiveIndentation", getter)]
    pub fn highlight_active_indentation(this: &IGuidesOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IGuidesOptions", js_name = "highlightActiveIndentation", setter)]
    pub fn set_highlight_active_indentation(this: &IGuidesOptions, highlight_active_indentation: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ISuggestOptions;

    /**
         * Overwrite word ends on accept. Default to false.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "insertMode", getter)]
    pub fn insert_mode(this: &ISuggestOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "insertMode", setter)]
    pub fn set_insert_mode(this: &ISuggestOptions, insert_mode: JsValue);

    /**
         * Enable graceful matching. Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "filterGraceful", getter)]
    pub fn filter_graceful(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "filterGraceful", setter)]
    pub fn set_filter_graceful(this: &ISuggestOptions, filter_graceful: bool);

    /**
         * Prevent quick suggestions when a snippet is active. Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "snippetsPreventQuickSuggestions", getter)]
    pub fn snippets_prevent_quick_suggestions(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "snippetsPreventQuickSuggestions", setter)]
    pub fn set_snippets_prevent_quick_suggestions(this: &ISuggestOptions, snippets_prevent_quick_suggestions: bool);

    /**
         * Favors words that appear close to the cursor.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "localityBonus", getter)]
    pub fn locality_bonus(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "localityBonus", setter)]
    pub fn set_locality_bonus(this: &ISuggestOptions, locality_bonus: bool);

    /**
         * Enable using global storage for remembering suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "shareSuggestSelections", getter)]
    pub fn share_suggest_selections(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "shareSuggestSelections", setter)]
    pub fn set_share_suggest_selections(this: &ISuggestOptions, share_suggest_selections: bool);

    /**
         * Select suggestions when triggered via quick suggest or trigger characters
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "selectionMode", getter)]
    pub fn selection_mode(this: &ISuggestOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "selectionMode", setter)]
    pub fn set_selection_mode(this: &ISuggestOptions, selection_mode: JsValue);

    /**
         * Enable or disable icons in suggestions. Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showIcons", getter)]
    pub fn show_icons(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showIcons", setter)]
    pub fn set_show_icons(this: &ISuggestOptions, show_icons: bool);

    /**
         * Enable or disable the suggest status bar.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showStatusBar", getter)]
    pub fn show_status_bar(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showStatusBar", setter)]
    pub fn set_show_status_bar(this: &ISuggestOptions, show_status_bar: bool);

    /**
         * Enable or disable the rendering of the suggestion preview.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "preview", getter)]
    pub fn preview(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "preview", setter)]
    pub fn set_preview(this: &ISuggestOptions, preview: bool);

    /**
         * Configures the mode of the preview.
        */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "previewMode", getter)]
    pub fn preview_mode(this: &ISuggestOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "previewMode", setter)]
    pub fn set_preview_mode(this: &ISuggestOptions, preview_mode: JsValue);

    /**
         * Show details inline with the label. Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showInlineDetails", getter)]
    pub fn show_inline_details(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showInlineDetails", setter)]
    pub fn set_show_inline_details(this: &ISuggestOptions, show_inline_details: bool);

    /**
         * Show method-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showMethods", getter)]
    pub fn show_methods(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showMethods", setter)]
    pub fn set_show_methods(this: &ISuggestOptions, show_methods: bool);

    /**
         * Show function-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFunctions", getter)]
    pub fn show_functions(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFunctions", setter)]
    pub fn set_show_functions(this: &ISuggestOptions, show_functions: bool);

    /**
         * Show constructor-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showConstructors", getter)]
    pub fn show_constructors(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showConstructors", setter)]
    pub fn set_show_constructors(this: &ISuggestOptions, show_constructors: bool);

    /**
         * Show deprecated-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showDeprecated", getter)]
    pub fn show_deprecated(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showDeprecated", setter)]
    pub fn set_show_deprecated(this: &ISuggestOptions, show_deprecated: bool);

    /**
         * Controls whether suggestions allow matches in the middle of the word instead of only at the beginning
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "matchOnWordStartOnly", getter)]
    pub fn match_on_word_start_only(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "matchOnWordStartOnly", setter)]
    pub fn set_match_on_word_start_only(this: &ISuggestOptions, match_on_word_start_only: bool);

    /**
         * Show field-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFields", getter)]
    pub fn show_fields(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFields", setter)]
    pub fn set_show_fields(this: &ISuggestOptions, show_fields: bool);

    /**
         * Show variable-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showVariables", getter)]
    pub fn show_variables(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showVariables", setter)]
    pub fn set_show_variables(this: &ISuggestOptions, show_variables: bool);

    /**
         * Show class-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showClasses", getter)]
    pub fn show_classes(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showClasses", setter)]
    pub fn set_show_classes(this: &ISuggestOptions, show_classes: bool);

    /**
         * Show struct-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showStructs", getter)]
    pub fn show_structs(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showStructs", setter)]
    pub fn set_show_structs(this: &ISuggestOptions, show_structs: bool);

    /**
         * Show interface-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showInterfaces", getter)]
    pub fn show_interfaces(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showInterfaces", setter)]
    pub fn set_show_interfaces(this: &ISuggestOptions, show_interfaces: bool);

    /**
         * Show module-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showModules", getter)]
    pub fn show_modules(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showModules", setter)]
    pub fn set_show_modules(this: &ISuggestOptions, show_modules: bool);

    /**
         * Show property-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showProperties", getter)]
    pub fn show_properties(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showProperties", setter)]
    pub fn set_show_properties(this: &ISuggestOptions, show_properties: bool);

    /**
         * Show event-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEvents", getter)]
    pub fn show_events(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEvents", setter)]
    pub fn set_show_events(this: &ISuggestOptions, show_events: bool);

    /**
         * Show operator-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showOperators", getter)]
    pub fn show_operators(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showOperators", setter)]
    pub fn set_show_operators(this: &ISuggestOptions, show_operators: bool);

    /**
         * Show unit-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showUnits", getter)]
    pub fn show_units(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showUnits", setter)]
    pub fn set_show_units(this: &ISuggestOptions, show_units: bool);

    /**
         * Show value-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showValues", getter)]
    pub fn show_values(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showValues", setter)]
    pub fn set_show_values(this: &ISuggestOptions, show_values: bool);

    /**
         * Show constant-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showConstants", getter)]
    pub fn show_constants(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showConstants", setter)]
    pub fn set_show_constants(this: &ISuggestOptions, show_constants: bool);

    /**
         * Show enum-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEnums", getter)]
    pub fn show_enums(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEnums", setter)]
    pub fn set_show_enums(this: &ISuggestOptions, show_enums: bool);

    /**
         * Show enumMember-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEnumMembers", getter)]
    pub fn show_enum_members(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showEnumMembers", setter)]
    pub fn set_show_enum_members(this: &ISuggestOptions, show_enum_members: bool);

    /**
         * Show keyword-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showKeywords", getter)]
    pub fn show_keywords(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showKeywords", setter)]
    pub fn set_show_keywords(this: &ISuggestOptions, show_keywords: bool);

    /**
         * Show text-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showWords", getter)]
    pub fn show_words(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showWords", setter)]
    pub fn set_show_words(this: &ISuggestOptions, show_words: bool);

    /**
         * Show color-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showColors", getter)]
    pub fn show_colors(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showColors", setter)]
    pub fn set_show_colors(this: &ISuggestOptions, show_colors: bool);

    /**
         * Show file-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFiles", getter)]
    pub fn show_files(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFiles", setter)]
    pub fn set_show_files(this: &ISuggestOptions, show_files: bool);

    /**
         * Show reference-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showReferences", getter)]
    pub fn show_references(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showReferences", setter)]
    pub fn set_show_references(this: &ISuggestOptions, show_references: bool);

    /**
         * Show folder-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFolders", getter)]
    pub fn show_folders(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showFolders", setter)]
    pub fn set_show_folders(this: &ISuggestOptions, show_folders: bool);

    /**
         * Show typeParameter-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showTypeParameters", getter)]
    pub fn show_type_parameters(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showTypeParameters", setter)]
    pub fn set_show_type_parameters(this: &ISuggestOptions, show_type_parameters: bool);

    /**
         * Show issue-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showIssues", getter)]
    pub fn show_issues(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showIssues", setter)]
    pub fn set_show_issues(this: &ISuggestOptions, show_issues: bool);

    /**
         * Show user-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showUsers", getter)]
    pub fn show_users(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showUsers", setter)]
    pub fn set_show_users(this: &ISuggestOptions, show_users: bool);

    /**
         * Show snippet-suggestions.
         */ 
    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showSnippets", getter)]
    pub fn show_snippets(this: &ISuggestOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISuggestOptions", js_name = "showSnippets", setter)]
    pub fn set_show_snippets(this: &ISuggestOptions, show_snippets: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ISmartSelectOptions;

    #[wasm_bindgen(method, js_class = "ISmartSelectOptions", js_name = "selectLeadingAndTrailingWhitespace", getter)]
    pub fn select_leading_and_trailing_whitespace(this: &ISmartSelectOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISmartSelectOptions", js_name = "selectLeadingAndTrailingWhitespace", setter)]
    pub fn set_select_leading_and_trailing_whitespace(this: &ISmartSelectOptions, select_leading_and_trailing_whitespace: bool);

    #[wasm_bindgen(method, js_class = "ISmartSelectOptions", js_name = "selectSubwords", getter)]
    pub fn select_subwords(this: &ISmartSelectOptions) -> bool;

    #[wasm_bindgen(method, js_class = "ISmartSelectOptions", js_name = "selectSubwords", setter)]
    pub fn set_select_subwords(this: &ISmartSelectOptions, select_subwords: bool);


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum WrappingIndent {
    /**
         * No indentation => wrapped lines begin at column 1.
         */ 
    None = 0,
    /**
         * Same => wrapped lines get the same indentation as the parent.
         */ 
    Same = 1,
    /**
         * Indent => wrapped lines get +1 indentation toward the parent.
         */ 
    Indent = 2,
    /**
         * DeepIndent => wrapped lines get +2 indentation toward the parent.
         */ 
    DeepIndent = 3,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type EditorWrappingInfo;

    #[wasm_bindgen(method, js_class = "EditorWrappingInfo", js_name = "isDominatedByLongLines", getter)]
    pub fn is_dominated_by_long_lines(this: &EditorWrappingInfo) -> bool;
    #[wasm_bindgen(method, js_class = "EditorWrappingInfo", js_name = "isWordWrapMinified", getter)]
    pub fn is_word_wrap_minified(this: &EditorWrappingInfo) -> bool;
    #[wasm_bindgen(method, js_class = "EditorWrappingInfo", js_name = "isViewportWrapping", getter)]
    pub fn is_viewport_wrapping(this: &EditorWrappingInfo) -> bool;
    #[wasm_bindgen(method, js_class = "EditorWrappingInfo", js_name = "wrappingColumn", getter)]
    pub fn wrapping_column(this: &EditorWrappingInfo) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDropIntoEditorOptions;

    /**
         * Enable dropping into editor.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IDropIntoEditorOptions", js_name = "enabled", getter)]
    pub fn enabled(this: &IDropIntoEditorOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IDropIntoEditorOptions", js_name = "enabled", setter)]
    pub fn set_enabled(this: &IDropIntoEditorOptions, enabled: bool);

    /**
         * Controls if a widget is shown after a drop.
         * Defaults to 'afterDrop'.
         */ 
    #[wasm_bindgen(method, js_class = "IDropIntoEditorOptions", js_name = "showDropSelector", getter)]
    pub fn show_drop_selector(this: &IDropIntoEditorOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IDropIntoEditorOptions", js_name = "showDropSelector", setter)]
    pub fn set_show_drop_selector(this: &IDropIntoEditorOptions, show_drop_selector: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IPasteAsOptions;

    /**
         * Enable paste as functionality in editors.
         * Defaults to true.
         */ 
    #[wasm_bindgen(method, js_class = "IPasteAsOptions", js_name = "enabled", getter)]
    pub fn enabled(this: &IPasteAsOptions) -> bool;

    #[wasm_bindgen(method, js_class = "IPasteAsOptions", js_name = "enabled", setter)]
    pub fn set_enabled(this: &IPasteAsOptions, enabled: bool);

    /**
         * Controls if a widget is shown after a drop.
         * Defaults to 'afterPaste'.
         */ 
    #[wasm_bindgen(method, js_class = "IPasteAsOptions", js_name = "showPasteSelector", getter)]
    pub fn show_paste_selector(this: &IPasteAsOptions) -> JsValue;

    #[wasm_bindgen(method, js_class = "IPasteAsOptions", js_name = "showPasteSelector", setter)]
    pub fn set_show_paste_selector(this: &IPasteAsOptions, show_paste_selector: JsValue);


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum EditorOption {
    acceptSuggestionOnCommitCharacter = 0,
    acceptSuggestionOnEnter = 1,
    accessibilitySupport = 2,
    accessibilityPageSize = 3,
    ariaLabel = 4,
    ariaRequired = 5,
    autoClosingBrackets = 6,
    screenReaderAnnounceInlineSuggestion = 7,
    autoClosingDelete = 8,
    autoClosingOvertype = 9,
    autoClosingQuotes = 10,
    autoIndent = 11,
    automaticLayout = 12,
    autoSurround = 13,
    bracketPairColorization = 14,
    guides = 15,
    codeLens = 16,
    codeLensFontFamily = 17,
    codeLensFontSize = 18,
    colorDecorators = 19,
    colorDecoratorsLimit = 20,
    columnSelection = 21,
    comments = 22,
    contextmenu = 23,
    copyWithSyntaxHighlighting = 24,
    cursorBlinking = 25,
    cursorSmoothCaretAnimation = 26,
    cursorStyle = 27,
    cursorSurroundingLines = 28,
    cursorSurroundingLinesStyle = 29,
    cursorWidth = 30,
    disableLayerHinting = 31,
    disableMonospaceOptimizations = 32,
    domReadOnly = 33,
    dragAndDrop = 34,
    dropIntoEditor = 35,
    emptySelectionClipboard = 36,
    experimentalWhitespaceRendering = 37,
    extraEditorClassName = 38,
    fastScrollSensitivity = 39,
    find = 40,
    fixedOverflowWidgets = 41,
    folding = 42,
    foldingStrategy = 43,
    foldingHighlight = 44,
    foldingImportsByDefault = 45,
    foldingMaximumRegions = 46,
    unfoldOnClickAfterEndOfLine = 47,
    fontFamily = 48,
    fontInfo = 49,
    fontLigatures = 50,
    fontSize = 51,
    fontWeight = 52,
    fontVariations = 53,
    formatOnPaste = 54,
    formatOnType = 55,
    glyphMargin = 56,
    gotoLocation = 57,
    hideCursorInOverviewRuler = 58,
    hover = 59,
    inDiffEditor = 60,
    inlineSuggest = 61,
    letterSpacing = 62,
    lightbulb = 63,
    lineDecorationsWidth = 64,
    lineHeight = 65,
    lineNumbers = 66,
    lineNumbersMinChars = 67,
    linkedEditing = 68,
    links = 69,
    matchBrackets = 70,
    minimap = 71,
    mouseStyle = 72,
    mouseWheelScrollSensitivity = 73,
    mouseWheelZoom = 74,
    multiCursorMergeOverlapping = 75,
    multiCursorModifier = 76,
    multiCursorPaste = 77,
    multiCursorLimit = 78,
    occurrencesHighlight = 79,
    overviewRulerBorder = 80,
    overviewRulerLanes = 81,
    padding = 82,
    pasteAs = 83,
    parameterHints = 84,
    peekWidgetDefaultFocus = 85,
    definitionLinkOpensInPeek = 86,
    quickSuggestions = 87,
    quickSuggestionsDelay = 88,
    readOnly = 89,
    readOnlyMessage = 90,
    renameOnType = 91,
    renderControlCharacters = 92,
    renderFinalNewline = 93,
    renderLineHighlight = 94,
    renderLineHighlightOnlyWhenFocus = 95,
    renderValidationDecorations = 96,
    renderWhitespace = 97,
    revealHorizontalRightPadding = 98,
    roundedSelection = 99,
    rulers = 100,
    scrollbar = 101,
    scrollBeyondLastColumn = 102,
    scrollBeyondLastLine = 103,
    scrollPredominantAxis = 104,
    selectionClipboard = 105,
    selectionHighlight = 106,
    selectOnLineNumbers = 107,
    showFoldingControls = 108,
    showUnused = 109,
    snippetSuggestions = 110,
    smartSelect = 111,
    smoothScrolling = 112,
    stickyScroll = 113,
    stickyTabStops = 114,
    stopRenderingLineAfter = 115,
    suggest = 116,
    suggestFontSize = 117,
    suggestLineHeight = 118,
    suggestOnTriggerCharacters = 119,
    suggestSelection = 120,
    tabCompletion = 121,
    tabIndex = 122,
    unicodeHighlighting = 123,
    unusualLineTerminators = 124,
    useShadowDOM = 125,
    useTabStops = 126,
    wordBreak = 127,
    wordSeparators = 128,
    wordWrap = 129,
    wordWrapBreakAfterCharacters = 130,
    wordWrapBreakBeforeCharacters = 131,
    wordWrapColumn = 132,
    wordWrapOverride1 = 133,
    wordWrapOverride2 = 134,
    wrappingIndent = 135,
    wrappingStrategy = 136,
    showDeprecated = 137,
    inlayHints = 138,
    editorClassName = 139,
    pixelRatio = 140,
    tabFocusMode = 141,
    layoutInfo = 142,
    wrappingInfo = 143,
    defaultColorDecorators = 144,
    colorDecoratorsActivatedOn = 145,

  }
}

pub type EditorOptionsType = JsValue
;
pub type FindEditorOptionsKeyById = JsValue
;
pub type ComputedEditorOptionValue = JsValue
;
pub type FindComputedEditorOptionValueById = NonNullable
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorConstructionOptions;

    /**
         * The initial editor dimension (to avoid measuring the container).
         */ 
    #[wasm_bindgen(method, js_class = "IEditorConstructionOptions", js_name = "dimension", getter)]
    pub fn dimension(this: &IEditorConstructionOptions) -> IDimension;

    #[wasm_bindgen(method, js_class = "IEditorConstructionOptions", js_name = "dimension", setter)]
    pub fn set_dimension(this: &IEditorConstructionOptions, dimension: IDimension);

    /**
         * Place overflow widgets inside an external DOM node.
         * Defaults to an internal DOM node.
         */ 
    #[wasm_bindgen(method, js_class = "IEditorConstructionOptions", js_name = "overflowWidgetsDomNode", getter)]
    pub fn overflow_widgets_dom_node(this: &IEditorConstructionOptions) -> HTMLElement;

    #[wasm_bindgen(method, js_class = "IEditorConstructionOptions", js_name = "overflowWidgetsDomNode", setter)]
    pub fn set_overflow_widgets_dom_node(this: &IEditorConstructionOptions, overflow_widgets_dom_node: HTMLElement);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IViewZone;

    /**
         * The line number after which this zone should appear.
         * Use 0 to place a view zone before the first line number.
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "afterLineNumber", getter)]
    pub fn after_line_number(this: &IViewZone) -> f64;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "afterLineNumber", setter)]
    pub fn set_after_line_number(this: &IViewZone, after_line_number: f64);

    /**
         * The column after which this zone should appear.
         * If not set, the maxLineColumn of `afterLineNumber` will be used.
         * This is relevant for wrapped lines.
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "afterColumn", getter)]
    pub fn after_column(this: &IViewZone) -> f64;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "afterColumn", setter)]
    pub fn set_after_column(this: &IViewZone, after_column: f64);

    /**
         * If the `afterColumn` has multiple view columns, the affinity specifies which one to use. Defaults to `none`.
        */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "afterColumnAffinity", getter)]
    pub fn after_column_affinity(this: &IViewZone) -> PositionAffinity;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "afterColumnAffinity", setter)]
    pub fn set_after_column_affinity(this: &IViewZone, after_column_affinity: PositionAffinity);

    /**
         * Render the zone even when its line is hidden.
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "showInHiddenAreas", getter)]
    pub fn show_in_hidden_areas(this: &IViewZone) -> bool;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "showInHiddenAreas", setter)]
    pub fn set_show_in_hidden_areas(this: &IViewZone, show_in_hidden_areas: bool);

    /**
         * Tiebreaker that is used when multiple view zones want to be after the same line.
         * Defaults to `afterColumn` otherwise 10000;
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "ordinal", getter)]
    pub fn ordinal(this: &IViewZone) -> f64;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "ordinal", setter)]
    pub fn set_ordinal(this: &IViewZone, ordinal: f64);

    /**
         * Suppress mouse down events.
         * If set, the editor will attach a mouse down listener to the view zone and .preventDefault on it.
         * Defaults to false
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "suppressMouseDown", getter)]
    pub fn suppress_mouse_down(this: &IViewZone) -> bool;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "suppressMouseDown", setter)]
    pub fn set_suppress_mouse_down(this: &IViewZone, suppress_mouse_down: bool);

    /**
         * The height in lines of the view zone.
         * If specified, `heightInPx` will be used instead of this.
         * If neither `heightInPx` nor `heightInLines` is specified, a default of `heightInLines` = 1 will be chosen.
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "heightInLines", getter)]
    pub fn height_in_lines(this: &IViewZone) -> f64;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "heightInLines", setter)]
    pub fn set_height_in_lines(this: &IViewZone, height_in_lines: f64);

    /**
         * The height in px of the view zone.
         * If this is set, the editor will give preference to it rather than `heightInLines` above.
         * If neither `heightInPx` nor `heightInLines` is specified, a default of `heightInLines` = 1 will be chosen.
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "heightInPx", getter)]
    pub fn height_in_px(this: &IViewZone) -> f64;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "heightInPx", setter)]
    pub fn set_height_in_px(this: &IViewZone, height_in_px: f64);

    /**
         * The minimum width in px of the view zone.
         * If this is set, the editor will ensure that the scroll width is >= than this value.
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "minWidthInPx", getter)]
    pub fn min_width_in_px(this: &IViewZone) -> f64;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "minWidthInPx", setter)]
    pub fn set_min_width_in_px(this: &IViewZone, min_width_in_px: f64);

    /**
         * The dom node of the view zone
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "domNode", getter)]
    pub fn dom_node(this: &IViewZone) -> HTMLElement;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "domNode", setter)]
    pub fn set_dom_node(this: &IViewZone, dom_node: HTMLElement);

    /**
         * An optional dom node for the view zone that will be placed in the margin area.
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "marginDomNode", getter)]
    pub fn margin_dom_node(this: &IViewZone) -> JsValue;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "marginDomNode", setter)]
    pub fn set_margin_dom_node(this: &IViewZone, margin_dom_node: JsValue);

    /**
         * Callback which gives the relative top of the view zone as it appears (taking scrolling into account).
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "onDomNodeTop", getter)]
    pub fn on_dom_node_top(this: &IViewZone) -> JsValue;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "onDomNodeTop", setter)]
    pub fn set_on_dom_node_top(this: &IViewZone, on_dom_node_top: JsValue);

    /**
         * Callback which gives the height in pixels of the view zone.
         */ 
    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "onComputedHeight", getter)]
    pub fn on_computed_height(this: &IViewZone) -> JsValue;

    #[wasm_bindgen(method, js_class = "IViewZone", js_name = "onComputedHeight", setter)]
    pub fn set_on_computed_height(this: &IViewZone, on_computed_height: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IViewZoneChangeAccessor;

    /**
         * Create a new view zone.
         * @param zone Zone to create
         * @return A unique identifier to the view zone.
         */ 
    #[wasm_bindgen(method, js_class = "IViewZoneChangeAccessor", js_name = "addZone")]
    pub fn add_zone(this: &IViewZoneChangeAccessor, zone: IViewZone, ) -> String;

    /**
         * Remove a zone
         * @param id A unique identifier to the view zone, as returned by the `addZone` call.
         */ 
    #[wasm_bindgen(method, js_class = "IViewZoneChangeAccessor", js_name = "removeZone")]
    pub fn remove_zone(this: &IViewZoneChangeAccessor, id: String, );

    /**
         * Change a zone's position.
         * The editor will rescan the `afterLineNumber` and `afterColumn` properties of a view zone.
         */ 
    #[wasm_bindgen(method, js_class = "IViewZoneChangeAccessor", js_name = "layoutZone")]
    pub fn layout_zone(this: &IViewZoneChangeAccessor, id: String, );


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum ContentWidgetPositionPreference {
    /**
         * Place the content widget exactly at a position
         */ 
    EXACT = 0,
    /**
         * Place the content widget above a position
         */ 
    ABOVE = 1,
    /**
         * Place the content widget below a position
         */ 
    BELOW = 2,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IContentWidgetPosition;

    /**
         * Desired position which serves as an anchor for placing the content widget.
         * The widget will be placed above, at, or below the specified position, based on the
         * provided preference. The widget will always touch this position.
         *
         * Given sufficient horizontal space, the widget will be placed to the right of the
         * passed in position. This can be tweaked by providing a `secondaryPosition`.
         *
         * @see preference
         * @see secondaryPosition
         */ 
    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "position", getter)]
    pub fn position(this: &IContentWidgetPosition) -> JsValue;

    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "position", setter)]
    pub fn set_position(this: &IContentWidgetPosition, position: JsValue);

    /**
         * Optionally, a secondary position can be provided to further define the placing of
         * the content widget. The secondary position must have the same line number as the
         * primary position. If possible, the widget will be placed such that it also touches
         * the secondary position.
         */ 
    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "secondaryPosition", getter)]
    pub fn secondary_position(this: &IContentWidgetPosition) -> JsValue;

    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "secondaryPosition", setter)]
    pub fn set_secondary_position(this: &IContentWidgetPosition, secondary_position: JsValue);

    /**
         * Placement preference for position, in order of preference.
         */ 
    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "preference", getter)]
    pub fn preference(this: &IContentWidgetPosition) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "preference", setter)]
    pub fn set_preference(this: &IContentWidgetPosition, preference: js_sys::Array);

    /**
         * Placement preference when multiple view positions refer to the same (model) position.
         * This plays a role when injected text is involved.
        */ 
    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "positionAffinity", getter)]
    pub fn position_affinity(this: &IContentWidgetPosition) -> PositionAffinity;

    #[wasm_bindgen(method, js_class = "IContentWidgetPosition", js_name = "positionAffinity", setter)]
    pub fn set_position_affinity(this: &IContentWidgetPosition, position_affinity: PositionAffinity);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IContentWidget;

    /**
         * Render this content widget in a location where it could overflow the editor's view dom node.
         */ 
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "allowEditorOverflow", getter)]
    pub fn allow_editor_overflow(this: &IContentWidget) -> bool;

    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "allowEditorOverflow", setter)]
    pub fn set_allow_editor_overflow(this: &IContentWidget, allow_editor_overflow: bool);

    /**
         * Call preventDefault() on mousedown events that target the content widget.
         */ 
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "suppressMouseDown", getter)]
    pub fn suppress_mouse_down(this: &IContentWidget) -> bool;

    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "suppressMouseDown", setter)]
    pub fn set_suppress_mouse_down(this: &IContentWidget, suppress_mouse_down: bool);

    /**
         * Get a unique identifier of the content widget.
         */ 
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "getId")]
    pub fn get_id(this: &IContentWidget, ) -> String;

    /**
         * Get the dom node of the content widget.
         */ 
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "getDomNode")]
    pub fn get_dom_node(this: &IContentWidget, ) -> HTMLElement;

    /**
         * Get the placement of the content widget.
         * If null is returned, the content widget will be placed off screen.
         */ 
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "getPosition")]
    pub fn get_position(this: &IContentWidget, ) -> JsValue;

    /**
         * Optional function that is invoked before rendering
         * the content widget. If a dimension is returned the editor will
         * attempt to use it.
         */ 
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "beforeRender")]
    pub fn before_render(this: &IContentWidget, ) -> JsValue;

    /**
         * Optional function that is invoked after rendering the content
         * widget. Is being invoked with the selected position preference
         * or `null` if not rendered.
         */ 
    #[wasm_bindgen(method, js_class = "IContentWidget", js_name = "afterRender")]
    pub fn after_render(this: &IContentWidget, position: JsValue, );


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum OverlayWidgetPositionPreference {
    /**
         * Position the overlay widget in the top right corner
         */ 
    TOP_RIGHT_CORNER = 0,
    /**
         * Position the overlay widget in the bottom right corner
         */ 
    BOTTOM_RIGHT_CORNER = 1,
    /**
         * Position the overlay widget in the top center
         */ 
    TOP_CENTER = 2,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IOverlayWidgetPosition;

    /**
         * The position preference for the overlay widget.
         */ 
    #[wasm_bindgen(method, js_class = "IOverlayWidgetPosition", js_name = "preference", getter)]
    pub fn preference(this: &IOverlayWidgetPosition) -> JsValue;

    #[wasm_bindgen(method, js_class = "IOverlayWidgetPosition", js_name = "preference", setter)]
    pub fn set_preference(this: &IOverlayWidgetPosition, preference: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IOverlayWidget;

    /**
         * Get a unique identifier of the overlay widget.
         */ 
    #[wasm_bindgen(method, js_class = "IOverlayWidget", js_name = "getId")]
    pub fn get_id(this: &IOverlayWidget, ) -> String;

    /**
         * Get the dom node of the overlay widget.
         */ 
    #[wasm_bindgen(method, js_class = "IOverlayWidget", js_name = "getDomNode")]
    pub fn get_dom_node(this: &IOverlayWidget, ) -> HTMLElement;

    /**
         * Get the placement of the overlay widget.
         * If null is returned, the overlay widget is responsible to place itself.
         */ 
    #[wasm_bindgen(method, js_class = "IOverlayWidget", js_name = "getPosition")]
    pub fn get_position(this: &IOverlayWidget, ) -> JsValue;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IGlyphMarginWidget;

    /**
         * Get a unique identifier of the glyph widget.
         */ 
    #[wasm_bindgen(method, js_class = "IGlyphMarginWidget", js_name = "getId")]
    pub fn get_id(this: &IGlyphMarginWidget, ) -> String;

    /**
         * Get the dom node of the glyph widget.
         */ 
    #[wasm_bindgen(method, js_class = "IGlyphMarginWidget", js_name = "getDomNode")]
    pub fn get_dom_node(this: &IGlyphMarginWidget, ) -> HTMLElement;

    /**
         * Get the placement of the glyph widget.
         */ 
    #[wasm_bindgen(method, js_class = "IGlyphMarginWidget", js_name = "getPosition")]
    pub fn get_position(this: &IGlyphMarginWidget, ) -> IGlyphMarginWidgetPosition;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IGlyphMarginWidgetPosition;

    /**
         * The glyph margin lane where the widget should be shown.
         */ 
    #[wasm_bindgen(method, js_class = "IGlyphMarginWidgetPosition", js_name = "lane", getter)]
    pub fn lane(this: &IGlyphMarginWidgetPosition) -> GlyphMarginLane;

    #[wasm_bindgen(method, js_class = "IGlyphMarginWidgetPosition", js_name = "lane", setter)]
    pub fn set_lane(this: &IGlyphMarginWidgetPosition, lane: GlyphMarginLane);

    /**
         * The priority order of the widget, used for determining which widget
         * to render when there are multiple.
         */ 
    #[wasm_bindgen(method, js_class = "IGlyphMarginWidgetPosition", js_name = "zIndex", getter)]
    pub fn z_index(this: &IGlyphMarginWidgetPosition) -> f64;

    #[wasm_bindgen(method, js_class = "IGlyphMarginWidgetPosition", js_name = "zIndex", setter)]
    pub fn set_z_index(this: &IGlyphMarginWidgetPosition, z_index: f64);

    /**
         * The editor range that this widget applies to.
         */ 
    #[wasm_bindgen(method, js_class = "IGlyphMarginWidgetPosition", js_name = "range", getter)]
    pub fn range(this: &IGlyphMarginWidgetPosition) -> IRange;

    #[wasm_bindgen(method, js_class = "IGlyphMarginWidgetPosition", js_name = "range", setter)]
    pub fn set_range(this: &IGlyphMarginWidgetPosition, range: IRange);


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum MouseTargetType {
    /**
         * Mouse is on top of an unknown element.
         */ 
    UNKNOWN = 0,
    /**
         * Mouse is on top of the textarea used for input.
         */ 
    TEXTAREA = 1,
    /**
         * Mouse is on top of the glyph margin
         */ 
    GUTTER_GLYPH_MARGIN = 2,
    /**
         * Mouse is on top of the line numbers
         */ 
    GUTTER_LINE_NUMBERS = 3,
    /**
         * Mouse is on top of the line decorations
         */ 
    GUTTER_LINE_DECORATIONS = 4,
    /**
         * Mouse is on top of the whitespace left in the gutter by a view zone.
         */ 
    GUTTER_VIEW_ZONE = 5,
    /**
         * Mouse is on top of text in the content.
         */ 
    CONTENT_TEXT = 6,
    /**
         * Mouse is on top of empty space in the content (e.g. after line text or below last line)
         */ 
    CONTENT_EMPTY = 7,
    /**
         * Mouse is on top of a view zone in the content.
         */ 
    CONTENT_VIEW_ZONE = 8,
    /**
         * Mouse is on top of a content widget.
         */ 
    CONTENT_WIDGET = 9,
    /**
         * Mouse is on top of the decorations overview ruler.
         */ 
    OVERVIEW_RULER = 10,
    /**
         * Mouse is on top of a scrollbar.
         */ 
    SCROLLBAR = 11,
    /**
         * Mouse is on top of an overlay widget.
         */ 
    OVERLAY_WIDGET = 12,
    /**
         * Mouse is outside of the editor.
         */ 
    OUTSIDE_EDITOR = 13,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IBaseMouseTarget;

    /**
         * The target element
         */ 
    #[wasm_bindgen(method, js_class = "IBaseMouseTarget", js_name = "element", getter)]
    pub fn element(this: &IBaseMouseTarget) -> JsValue;
    /**
         * The 'approximate' editor position
         */ 
    #[wasm_bindgen(method, js_class = "IBaseMouseTarget", js_name = "position", getter)]
    pub fn position(this: &IBaseMouseTarget) -> JsValue;
    /**
         * Desired mouse column (e.g. when position.column gets clamped to text length -- clicking after text on a line).
         */ 
    #[wasm_bindgen(method, js_class = "IBaseMouseTarget", js_name = "mouseColumn", getter)]
    pub fn mouse_column(this: &IBaseMouseTarget) -> f64;
    /**
         * The 'approximate' editor range
         */ 
    #[wasm_bindgen(method, js_class = "IBaseMouseTarget", js_name = "range", getter)]
    pub fn range(this: &IBaseMouseTarget) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetUnknown;

    #[wasm_bindgen(method, js_class = "IMouseTargetUnknown", js_name = "type", getter)]
    pub fn r#type(this: &IMouseTargetUnknown) -> MouseTargetType;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetTextarea;

    #[wasm_bindgen(method, js_class = "IMouseTargetTextarea", js_name = "type", getter)]
    pub fn r#type(this: &IMouseTargetTextarea) -> MouseTargetType;
    #[wasm_bindgen(method, js_class = "IMouseTargetTextarea", js_name = "position", getter)]
    pub fn position(this: &IMouseTargetTextarea) -> JsValue;
    #[wasm_bindgen(method, js_class = "IMouseTargetTextarea", js_name = "range", getter)]
    pub fn range(this: &IMouseTargetTextarea) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetMarginData;

    #[wasm_bindgen(method, js_class = "IMouseTargetMarginData", js_name = "isAfterLines", getter)]
    pub fn is_after_lines(this: &IMouseTargetMarginData) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseTargetMarginData", js_name = "glyphMarginLeft", getter)]
    pub fn glyph_margin_left(this: &IMouseTargetMarginData) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseTargetMarginData", js_name = "glyphMarginWidth", getter)]
    pub fn glyph_margin_width(this: &IMouseTargetMarginData) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseTargetMarginData", js_name = "lineNumbersWidth", getter)]
    pub fn line_numbers_width(this: &IMouseTargetMarginData) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseTargetMarginData", js_name = "offsetX", getter)]
    pub fn offset_x(this: &IMouseTargetMarginData) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetMargin;

    #[wasm_bindgen(method, js_class = "IMouseTargetMargin", js_name = "type", getter)]
    pub fn r#type(this: &IMouseTargetMargin) -> JsValue;
    #[wasm_bindgen(method, js_class = "IMouseTargetMargin", js_name = "position", getter)]
    pub fn position(this: &IMouseTargetMargin) -> Position;
    #[wasm_bindgen(method, js_class = "IMouseTargetMargin", js_name = "range", getter)]
    pub fn range(this: &IMouseTargetMargin) -> Range;
    #[wasm_bindgen(method, js_class = "IMouseTargetMargin", js_name = "detail", getter)]
    pub fn detail(this: &IMouseTargetMargin) -> IMouseTargetMarginData;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetViewZoneData;

    #[wasm_bindgen(method, js_class = "IMouseTargetViewZoneData", js_name = "viewZoneId", getter)]
    pub fn view_zone_id(this: &IMouseTargetViewZoneData) -> String;
    #[wasm_bindgen(method, js_class = "IMouseTargetViewZoneData", js_name = "positionBefore", getter)]
    pub fn position_before(this: &IMouseTargetViewZoneData) -> JsValue;
    #[wasm_bindgen(method, js_class = "IMouseTargetViewZoneData", js_name = "positionAfter", getter)]
    pub fn position_after(this: &IMouseTargetViewZoneData) -> JsValue;
    #[wasm_bindgen(method, js_class = "IMouseTargetViewZoneData", js_name = "position", getter)]
    pub fn position(this: &IMouseTargetViewZoneData) -> Position;
    #[wasm_bindgen(method, js_class = "IMouseTargetViewZoneData", js_name = "afterLineNumber", getter)]
    pub fn after_line_number(this: &IMouseTargetViewZoneData) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetViewZone;

    #[wasm_bindgen(method, js_class = "IMouseTargetViewZone", js_name = "type", getter)]
    pub fn r#type(this: &IMouseTargetViewZone) -> JsValue;
    #[wasm_bindgen(method, js_class = "IMouseTargetViewZone", js_name = "position", getter)]
    pub fn position(this: &IMouseTargetViewZone) -> Position;
    #[wasm_bindgen(method, js_class = "IMouseTargetViewZone", js_name = "range", getter)]
    pub fn range(this: &IMouseTargetViewZone) -> Range;
    #[wasm_bindgen(method, js_class = "IMouseTargetViewZone", js_name = "detail", getter)]
    pub fn detail(this: &IMouseTargetViewZone) -> IMouseTargetViewZoneData;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetContentTextData;

    #[wasm_bindgen(method, js_class = "IMouseTargetContentTextData", js_name = "mightBeForeignElement", getter)]
    pub fn might_be_foreign_element(this: &IMouseTargetContentTextData) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetContentText;

    #[wasm_bindgen(method, js_class = "IMouseTargetContentText", js_name = "type", getter)]
    pub fn r#type(this: &IMouseTargetContentText) -> MouseTargetType;
    #[wasm_bindgen(method, js_class = "IMouseTargetContentText", js_name = "position", getter)]
    pub fn position(this: &IMouseTargetContentText) -> Position;
    #[wasm_bindgen(method, js_class = "IMouseTargetContentText", js_name = "range", getter)]
    pub fn range(this: &IMouseTargetContentText) -> Range;
    #[wasm_bindgen(method, js_class = "IMouseTargetContentText", js_name = "detail", getter)]
    pub fn detail(this: &IMouseTargetContentText) -> IMouseTargetContentTextData;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetContentEmptyData;

    #[wasm_bindgen(method, js_class = "IMouseTargetContentEmptyData", js_name = "isAfterLines", getter)]
    pub fn is_after_lines(this: &IMouseTargetContentEmptyData) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseTargetContentEmptyData", js_name = "horizontalDistanceToText", getter)]
    pub fn horizontal_distance_to_text(this: &IMouseTargetContentEmptyData) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetContentEmpty;

    #[wasm_bindgen(method, js_class = "IMouseTargetContentEmpty", js_name = "type", getter)]
    pub fn r#type(this: &IMouseTargetContentEmpty) -> MouseTargetType;
    #[wasm_bindgen(method, js_class = "IMouseTargetContentEmpty", js_name = "position", getter)]
    pub fn position(this: &IMouseTargetContentEmpty) -> Position;
    #[wasm_bindgen(method, js_class = "IMouseTargetContentEmpty", js_name = "range", getter)]
    pub fn range(this: &IMouseTargetContentEmpty) -> Range;
    #[wasm_bindgen(method, js_class = "IMouseTargetContentEmpty", js_name = "detail", getter)]
    pub fn detail(this: &IMouseTargetContentEmpty) -> IMouseTargetContentEmptyData;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetContentWidget;

    #[wasm_bindgen(method, js_class = "IMouseTargetContentWidget", js_name = "type", getter)]
    pub fn r#type(this: &IMouseTargetContentWidget) -> MouseTargetType;
    #[wasm_bindgen(method, js_class = "IMouseTargetContentWidget", js_name = "position", getter)]
    pub fn position(this: &IMouseTargetContentWidget) -> JsValue;
    #[wasm_bindgen(method, js_class = "IMouseTargetContentWidget", js_name = "range", getter)]
    pub fn range(this: &IMouseTargetContentWidget) -> JsValue;
    #[wasm_bindgen(method, js_class = "IMouseTargetContentWidget", js_name = "detail", getter)]
    pub fn detail(this: &IMouseTargetContentWidget) -> String;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetOverlayWidget;

    #[wasm_bindgen(method, js_class = "IMouseTargetOverlayWidget", js_name = "type", getter)]
    pub fn r#type(this: &IMouseTargetOverlayWidget) -> MouseTargetType;
    #[wasm_bindgen(method, js_class = "IMouseTargetOverlayWidget", js_name = "position", getter)]
    pub fn position(this: &IMouseTargetOverlayWidget) -> JsValue;
    #[wasm_bindgen(method, js_class = "IMouseTargetOverlayWidget", js_name = "range", getter)]
    pub fn range(this: &IMouseTargetOverlayWidget) -> JsValue;
    #[wasm_bindgen(method, js_class = "IMouseTargetOverlayWidget", js_name = "detail", getter)]
    pub fn detail(this: &IMouseTargetOverlayWidget) -> String;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetScrollbar;

    #[wasm_bindgen(method, js_class = "IMouseTargetScrollbar", js_name = "type", getter)]
    pub fn r#type(this: &IMouseTargetScrollbar) -> MouseTargetType;
    #[wasm_bindgen(method, js_class = "IMouseTargetScrollbar", js_name = "position", getter)]
    pub fn position(this: &IMouseTargetScrollbar) -> Position;
    #[wasm_bindgen(method, js_class = "IMouseTargetScrollbar", js_name = "range", getter)]
    pub fn range(this: &IMouseTargetScrollbar) -> Range;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetOverviewRuler;

    #[wasm_bindgen(method, js_class = "IMouseTargetOverviewRuler", js_name = "type", getter)]
    pub fn r#type(this: &IMouseTargetOverviewRuler) -> MouseTargetType;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMouseTargetOutsideEditor;

    #[wasm_bindgen(method, js_class = "IMouseTargetOutsideEditor", js_name = "type", getter)]
    pub fn r#type(this: &IMouseTargetOutsideEditor) -> MouseTargetType;
    #[wasm_bindgen(method, js_class = "IMouseTargetOutsideEditor", js_name = "outsidePosition", getter)]
    pub fn outside_position(this: &IMouseTargetOutsideEditor) -> JsValue;
    #[wasm_bindgen(method, js_class = "IMouseTargetOutsideEditor", js_name = "outsideDistance", getter)]
    pub fn outside_distance(this: &IMouseTargetOutsideEditor) -> f64;

}

pub type IMouseTarget = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorMouseEvent;

    #[wasm_bindgen(method, js_class = "IEditorMouseEvent", js_name = "event", getter)]
    pub fn event(this: &IEditorMouseEvent) -> IMouseEvent;
    #[wasm_bindgen(method, js_class = "IEditorMouseEvent", js_name = "target", getter)]
    pub fn target(this: &IEditorMouseEvent) -> IMouseTarget;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IPartialEditorMouseEvent;

    #[wasm_bindgen(method, js_class = "IPartialEditorMouseEvent", js_name = "event", getter)]
    pub fn event(this: &IPartialEditorMouseEvent) -> IMouseEvent;
    #[wasm_bindgen(method, js_class = "IPartialEditorMouseEvent", js_name = "target", getter)]
    pub fn target(this: &IPartialEditorMouseEvent) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IPasteEvent;

    #[wasm_bindgen(method, js_class = "IPasteEvent", js_name = "range", getter)]
    pub fn range(this: &IPasteEvent) -> Range;
    #[wasm_bindgen(method, js_class = "IPasteEvent", js_name = "languageId", getter)]
    pub fn language_id(this: &IPasteEvent) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDiffEditorConstructionOptions;

    /**
         * Place overflow widgets inside an external DOM node.
         * Defaults to an internal DOM node.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorConstructionOptions", js_name = "overflowWidgetsDomNode", getter)]
    pub fn overflow_widgets_dom_node(this: &IDiffEditorConstructionOptions) -> HTMLElement;

    #[wasm_bindgen(method, js_class = "IDiffEditorConstructionOptions", js_name = "overflowWidgetsDomNode", setter)]
    pub fn set_overflow_widgets_dom_node(this: &IDiffEditorConstructionOptions, overflow_widgets_dom_node: HTMLElement);

    /**
         * Aria label for original editor.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorConstructionOptions", js_name = "originalAriaLabel", getter)]
    pub fn original_aria_label(this: &IDiffEditorConstructionOptions) -> String;

    #[wasm_bindgen(method, js_class = "IDiffEditorConstructionOptions", js_name = "originalAriaLabel", setter)]
    pub fn set_original_aria_label(this: &IDiffEditorConstructionOptions, original_aria_label: String);

    /**
         * Aria label for modified editor.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditorConstructionOptions", js_name = "modifiedAriaLabel", getter)]
    pub fn modified_aria_label(this: &IDiffEditorConstructionOptions) -> String;

    #[wasm_bindgen(method, js_class = "IDiffEditorConstructionOptions", js_name = "modifiedAriaLabel", setter)]
    pub fn set_modified_aria_label(this: &IDiffEditorConstructionOptions, modified_aria_label: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ICodeEditor;

    /**
         * An event emitted when the content of the current model has changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeModelContent", getter)]
    pub fn on_did_change_model_content(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the language of the current model has changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeModelLanguage", getter)]
    pub fn on_did_change_model_language(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the language configuration of the current model has changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeModelLanguageConfiguration", getter)]
    pub fn on_did_change_model_language_configuration(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the options of the current model has changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeModelOptions", getter)]
    pub fn on_did_change_model_options(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the configuration of the editor has changed. (e.g. `editor.updateOptions()`)
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeConfiguration", getter)]
    pub fn on_did_change_configuration(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the cursor position has changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeCursorPosition", getter)]
    pub fn on_did_change_cursor_position(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the cursor selection has changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeCursorSelection", getter)]
    pub fn on_did_change_cursor_selection(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the model of this editor has changed (e.g. `editor.setModel()`).
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeModel", getter)]
    pub fn on_did_change_model(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the decorations of the current model have changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeModelDecorations", getter)]
    pub fn on_did_change_model_decorations(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the text inside this editor gained focus (i.e. cursor starts blinking).
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidFocusEditorText", getter)]
    pub fn on_did_focus_editor_text(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the text inside this editor lost focus (i.e. cursor stops blinking).
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidBlurEditorText", getter)]
    pub fn on_did_blur_editor_text(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the text inside this editor or an editor widget gained focus.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidFocusEditorWidget", getter)]
    pub fn on_did_focus_editor_widget(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the text inside this editor or an editor widget lost focus.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidBlurEditorWidget", getter)]
    pub fn on_did_blur_editor_widget(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted after composition has started.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidCompositionStart", getter)]
    pub fn on_did_composition_start(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted after composition has ended.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidCompositionEnd", getter)]
    pub fn on_did_composition_end(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when editing failed because the editor is read-only.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidAttemptReadOnlyEdit", getter)]
    pub fn on_did_attempt_read_only_edit(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when users paste text in the editor.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidPaste", getter)]
    pub fn on_did_paste(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted on a "mouseup".
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onMouseUp", getter)]
    pub fn on_mouse_up(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted on a "mousedown".
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onMouseDown", getter)]
    pub fn on_mouse_down(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted on a "contextmenu".
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onContextMenu", getter)]
    pub fn on_context_menu(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted on a "mousemove".
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onMouseMove", getter)]
    pub fn on_mouse_move(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted on a "mouseleave".
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onMouseLeave", getter)]
    pub fn on_mouse_leave(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted on a "keyup".
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onKeyUp", getter)]
    pub fn on_key_up(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted on a "keydown".
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onKeyDown", getter)]
    pub fn on_key_down(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the layout of the editor has changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidLayoutChange", getter)]
    pub fn on_did_layout_change(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the content width or content height in the editor has changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidContentSizeChange", getter)]
    pub fn on_did_content_size_change(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when the scroll in the editor has changed.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidScrollChange", getter)]
    pub fn on_did_scroll_change(this: &ICodeEditor) -> IEvent;
    /**
         * An event emitted when hidden areas change in the editor (e.g. due to folding).
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "onDidChangeHiddenAreas", getter)]
    pub fn on_did_change_hidden_areas(this: &ICodeEditor) -> IEvent;
    /**
         * Saves current view state of the editor in a serializable object.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "saveViewState")]
    pub fn save_view_state(this: &ICodeEditor, ) -> JsValue;

    /**
         * Restores the view state of the editor from a serializable object generated by `saveViewState`.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "restoreViewState")]
    pub fn restore_view_state(this: &ICodeEditor, state: JsValue, );

    /**
         * Returns true if the text inside this editor or an editor widget has focus.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "hasWidgetFocus")]
    pub fn has_widget_focus(this: &ICodeEditor, ) -> bool;

    /**
         * Get a contribution of this editor.
         * @id Unique identifier of the contribution.
         * @return The contribution or null if contribution not found.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getContribution")]
    pub fn get_contribution(this: &ICodeEditor, id: String, ) -> JsValue;

    /**
         * Type the getModel() of IEditor.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getModel")]
    pub fn get_model(this: &ICodeEditor, ) -> JsValue;

    /**
         * Sets the current model attached to this editor.
         * If the previous model was created by the editor via the value key in the options
         * literal object, it will be destroyed. Otherwise, if the previous model was set
         * via setModel, or the model key in the options literal object, the previous model
         * will not be destroyed.
         * It is safe to call setModel(null) to simply detach the current model from the editor.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "setModel")]
    pub fn set_model(this: &ICodeEditor, model: JsValue, );

    /**
         * Gets all the editor computed options.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getOptions")]
    pub fn get_options(this: &ICodeEditor, ) -> IComputedEditorOptions;

    /**
         * Gets a specific editor option.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getOption")]
    pub fn get_option(this: &ICodeEditor, id: JsValue, ) -> FindComputedEditorOptionValueById;

    /**
         * Returns the editor's configuration (without any validation or defaults).
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getRawOptions")]
    pub fn get_raw_options(this: &ICodeEditor, ) -> IEditorOptions;

    /**
         * Get value of the current model attached to this editor.
         * @see {@link ITextModel.getValue}
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getValue")]
    pub fn get_value(this: &ICodeEditor, options: Option<js_sys::Object>, ) -> String;

    /**
         * Set the value of the current model attached to this editor.
         * @see {@link ITextModel.setValue}
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "setValue")]
    pub fn set_value(this: &ICodeEditor, new_value: String, );

    /**
         * Get the width of the editor's content.
         * This is information that is "erased" when computing `scrollWidth = Math.max(contentWidth, width)`
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getContentWidth")]
    pub fn get_content_width(this: &ICodeEditor, ) -> f64;

    /**
         * Get the scrollWidth of the editor's viewport.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getScrollWidth")]
    pub fn get_scroll_width(this: &ICodeEditor, ) -> f64;

    /**
         * Get the scrollLeft of the editor's viewport.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getScrollLeft")]
    pub fn get_scroll_left(this: &ICodeEditor, ) -> f64;

    /**
         * Get the height of the editor's content.
         * This is information that is "erased" when computing `scrollHeight = Math.max(contentHeight, height)`
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getContentHeight")]
    pub fn get_content_height(this: &ICodeEditor, ) -> f64;

    /**
         * Get the scrollHeight of the editor's viewport.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getScrollHeight")]
    pub fn get_scroll_height(this: &ICodeEditor, ) -> f64;

    /**
         * Get the scrollTop of the editor's viewport.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getScrollTop")]
    pub fn get_scroll_top(this: &ICodeEditor, ) -> f64;

    /**
         * Change the scrollLeft of the editor's viewport.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "setScrollLeft")]
    pub fn set_scroll_left(this: &ICodeEditor, new_scroll_left: f64, scroll_type: Option<ScrollType>, );

    /**
         * Change the scrollTop of the editor's viewport.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "setScrollTop")]
    pub fn set_scroll_top(this: &ICodeEditor, new_scroll_top: f64, scroll_type: Option<ScrollType>, );

    /**
         * Change the scroll position of the editor's viewport.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "setScrollPosition")]
    pub fn set_scroll_position(this: &ICodeEditor, position: INewScrollPosition, scroll_type: Option<ScrollType>, );

    /**
         * Check if the editor is currently scrolling towards a different scroll position.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "hasPendingScrollAnimation")]
    pub fn has_pending_scroll_animation(this: &ICodeEditor, ) -> bool;

    /**
         * Get an action that is a contribution to this editor.
         * @id Unique identifier of the contribution.
         * @return The action or null if action not found.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getAction")]
    pub fn get_action(this: &ICodeEditor, id: String, ) -> JsValue;

    /**
         * Execute a command on the editor.
         * The edits will land on the undo-redo stack, but no "undo stop" will be pushed.
         * @param source The source of the call.
         * @param command The command to execute
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "executeCommand")]
    pub fn execute_command(this: &ICodeEditor, source: JsValue, command: ICommand, );

    /**
         * Create an "undo stop" in the undo-redo stack.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "pushUndoStop")]
    pub fn push_undo_stop(this: &ICodeEditor, ) -> bool;

    /**
         * Remove the "undo stop" in the undo-redo stack.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "popUndoStop")]
    pub fn pop_undo_stop(this: &ICodeEditor, ) -> bool;

    /**
         * Execute edits on the editor.
         * The edits will land on the undo-redo stack, but no "undo stop" will be pushed.
         * @param source The source of the call.
         * @param edits The edits to execute.
         * @param endCursorState Cursor state after the edits were applied.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "executeEdits")]
    pub fn execute_edits(this: &ICodeEditor, source: JsValue, edits: js_sys::Array, end_cursor_state: JsValue, ) -> bool;

    /**
         * Execute multiple (concomitant) commands on the editor.
         * @param source The source of the call.
         * @param command The commands to execute
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "executeCommands")]
    pub fn execute_commands(this: &ICodeEditor, source: JsValue, commands: js_sys::Array, );

    /**
         * Get all the decorations on a line (filtering out decorations from other editors).
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getLineDecorations")]
    pub fn get_line_decorations(this: &ICodeEditor, line_number: f64, ) -> JsValue;

    /**
         * Get all the decorations for a range (filtering out decorations from other editors).
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getDecorationsInRange")]
    pub fn get_decorations_in_range(this: &ICodeEditor, range: Range, ) -> JsValue;

    /**
         * All decorations added through this call will get the ownerId of this editor.
         * @deprecated Use `createDecorationsCollection`
         * @see createDecorationsCollection
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "deltaDecorations")]
    pub fn delta_decorations(this: &ICodeEditor, old_decorations: js_sys::Array, new_decorations: js_sys::Array, ) -> js_sys::Array;

    /**
         * Remove previously added decorations.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "removeDecorations")]
    pub fn remove_decorations(this: &ICodeEditor, decoration_ids: js_sys::Array, );

    /**
         * Get the layout info for the editor.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getLayoutInfo")]
    pub fn get_layout_info(this: &ICodeEditor, ) -> EditorLayoutInfo;

    /**
         * Returns the ranges that are currently visible.
         * Does not account for horizontal scrolling.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getVisibleRanges")]
    pub fn get_visible_ranges(this: &ICodeEditor, ) -> js_sys::Array;

    /**
         * Get the vertical position (top offset) for the line's top w.r.t. to the first line.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getTopForLineNumber")]
    pub fn get_top_for_line_number(this: &ICodeEditor, line_number: f64, ) -> f64;

    /**
         * Get the vertical position (top offset) for the line's bottom w.r.t. to the first line.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getBottomForLineNumber")]
    pub fn get_bottom_for_line_number(this: &ICodeEditor, line_number: f64, ) -> f64;

    /**
         * Get the vertical position (top offset) for the position w.r.t. to the first line.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getTopForPosition")]
    pub fn get_top_for_position(this: &ICodeEditor, line_number: f64, column: f64, ) -> f64;

    /**
         * Write the screen reader content to be the current selection
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "writeScreenReaderContent")]
    pub fn write_screen_reader_content(this: &ICodeEditor, reason: String, );

    /**
         * Returns the editor's container dom node
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getContainerDomNode")]
    pub fn get_container_dom_node(this: &ICodeEditor, ) -> HTMLElement;

    /**
         * Returns the editor's dom node
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getDomNode")]
    pub fn get_dom_node(this: &ICodeEditor, ) -> JsValue;

    /**
         * Add a content widget. Widgets must have unique ids, otherwise they will be overwritten.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "addContentWidget")]
    pub fn add_content_widget(this: &ICodeEditor, widget: IContentWidget, );

    /**
         * Layout/Reposition a content widget. This is a ping to the editor to call widget.getPosition()
         * and update appropriately.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "layoutContentWidget")]
    pub fn layout_content_widget(this: &ICodeEditor, widget: IContentWidget, );

    /**
         * Remove a content widget.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "removeContentWidget")]
    pub fn remove_content_widget(this: &ICodeEditor, widget: IContentWidget, );

    /**
         * Add an overlay widget. Widgets must have unique ids, otherwise they will be overwritten.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "addOverlayWidget")]
    pub fn add_overlay_widget(this: &ICodeEditor, widget: IOverlayWidget, );

    /**
         * Layout/Reposition an overlay widget. This is a ping to the editor to call widget.getPosition()
         * and update appropriately.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "layoutOverlayWidget")]
    pub fn layout_overlay_widget(this: &ICodeEditor, widget: IOverlayWidget, );

    /**
         * Remove an overlay widget.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "removeOverlayWidget")]
    pub fn remove_overlay_widget(this: &ICodeEditor, widget: IOverlayWidget, );

    /**
         * Add a glyph margin widget. Widgets must have unique ids, otherwise they will be overwritten.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "addGlyphMarginWidget")]
    pub fn add_glyph_margin_widget(this: &ICodeEditor, widget: IGlyphMarginWidget, );

    /**
         * Layout/Reposition a glyph margin widget. This is a ping to the editor to call widget.getPosition()
         * and update appropriately.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "layoutGlyphMarginWidget")]
    pub fn layout_glyph_margin_widget(this: &ICodeEditor, widget: IGlyphMarginWidget, );

    /**
         * Remove a glyph margin widget.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "removeGlyphMarginWidget")]
    pub fn remove_glyph_margin_widget(this: &ICodeEditor, widget: IGlyphMarginWidget, );

    /**
         * Change the view zones. View zones are lost when a new model is attached to the editor.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "changeViewZones")]
    pub fn change_view_zones(this: &ICodeEditor, callback: JsValue, );

    /**
         * Get the horizontal position (left offset) for the column w.r.t to the beginning of the line.
         * This method works only if the line `lineNumber` is currently rendered (in the editor's viewport).
         * Use this method with caution.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getOffsetForColumn")]
    pub fn get_offset_for_column(this: &ICodeEditor, line_number: f64, column: f64, ) -> f64;

    /**
         * Force an editor render now.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "render")]
    pub fn render(this: &ICodeEditor, force_redraw: Option<bool>, );

    /**
         * Get the hit test target at coordinates `clientX` and `clientY`.
         * The coordinates are relative to the top-left of the viewport.
         *
         * @returns Hit test target or null if the coordinates fall outside the editor or the editor has no model.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getTargetAtClientPoint")]
    pub fn get_target_at_client_point(this: &ICodeEditor, client_x: f64, client_y: f64, ) -> JsValue;

    /**
         * Get the visible position for `position`.
         * The result position takes scrolling into account and is relative to the top left corner of the editor.
         * Explanation 1: the results of this method will change for the same `position` if the user scrolls the editor.
         * Explanation 2: the results of this method will not change if the container of the editor gets repositioned.
         * Warning: the results of this method are inaccurate for positions that are outside the current editor viewport.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "getScrolledVisiblePosition")]
    pub fn get_scrolled_visible_position(this: &ICodeEditor, position: IPosition, ) -> JsValue;

    /**
         * Apply the same font settings as the editor to `target`.
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "applyFontInfo")]
    pub fn apply_font_info(this: &ICodeEditor, target: HTMLElement, );

    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "setBanner")]
    pub fn set_banner(this: &ICodeEditor, banner_dom_node: JsValue, height: f64, );

    /**
         * Is called when the model has been set, view state was restored and options are updated.
         * This is the best place to compute data for the viewport (such as tokens).
         */ 
    #[wasm_bindgen(method, js_class = "ICodeEditor", js_name = "handleInitialized")]
    pub fn handle_initialized(this: &ICodeEditor, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDiffEditor;

    /**
         * @see {@link ICodeEditor.getContainerDomNode}
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "getContainerDomNode")]
    pub fn get_container_dom_node(this: &IDiffEditor, ) -> HTMLElement;

    /**
         * An event emitted when the diff information computed by this diff editor has been updated.
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "onDidUpdateDiff", getter)]
    pub fn on_did_update_diff(this: &IDiffEditor) -> IEvent;
    /**
         * An event emitted when the diff model is changed (i.e. the diff editor shows new content).
         * @event
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "onDidChangeModel", getter)]
    pub fn on_did_change_model(this: &IDiffEditor) -> IEvent;
    /**
         * Saves current view state of the editor in a serializable object.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "saveViewState")]
    pub fn save_view_state(this: &IDiffEditor, ) -> JsValue;

    /**
         * Restores the view state of the editor from a serializable object generated by `saveViewState`.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "restoreViewState")]
    pub fn restore_view_state(this: &IDiffEditor, state: JsValue, );

    /**
         * Type the getModel() of IEditor.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "getModel")]
    pub fn get_model(this: &IDiffEditor, ) -> JsValue;

    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "createViewModel")]
    pub fn create_view_model(this: &IDiffEditor, model: IDiffEditorModel, ) -> IDiffEditorViewModel;

    /**
         * Sets the current model attached to this editor.
         * If the previous model was created by the editor via the value key in the options
         * literal object, it will be destroyed. Otherwise, if the previous model was set
         * via setModel, or the model key in the options literal object, the previous model
         * will not be destroyed.
         * It is safe to call setModel(null) to simply detach the current model from the editor.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "setModel")]
    pub fn set_model(this: &IDiffEditor, model: JsValue, );

    /**
         * Get the `original` editor.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "getOriginalEditor")]
    pub fn get_original_editor(this: &IDiffEditor, ) -> ICodeEditor;

    /**
         * Get the `modified` editor.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "getModifiedEditor")]
    pub fn get_modified_editor(this: &IDiffEditor, ) -> ICodeEditor;

    /**
         * Get the computed diff information.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "getLineChanges")]
    pub fn get_line_changes(this: &IDiffEditor, ) -> JsValue;

    /**
         * Update the editor's options after the editor has been created.
         */ 
    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "updateOptions")]
    pub fn update_options(this: &IDiffEditor, new_options: IDiffEditorOptions, );

    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "accessibleDiffViewerNext")]
    pub fn accessible_diff_viewer_next(this: &IDiffEditor, );

    #[wasm_bindgen(method, js_class = "IDiffEditor", js_name = "accessibleDiffViewerPrev")]
    pub fn accessible_diff_viewer_prev(this: &IDiffEditor, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object, extends = BareFontInfo)]
    pub type FontInfo;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type BareFontInfo;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEditorZoom;

    #[wasm_bindgen(method, js_class = "IEditorZoom", js_name = "onDidChangeZoomLevel", getter)]
    pub fn on_did_change_zoom_level(this: &IEditorZoom) -> IEvent;

    #[wasm_bindgen(method, js_class = "IEditorZoom", js_name = "onDidChangeZoomLevel", setter)]
    pub fn set_on_did_change_zoom_level(this: &IEditorZoom, on_did_change_zoom_level: IEvent);

    #[wasm_bindgen(method, js_class = "IEditorZoom", js_name = "getZoomLevel")]
    pub fn get_zoom_level(this: &IEditorZoom, ) -> f64;

    #[wasm_bindgen(method, js_class = "IEditorZoom", js_name = "setZoomLevel")]
    pub fn set_zoom_level(this: &IEditorZoom, zoom_level: f64, );


}

pub type IReadOnlyModel = ITextModel
;
pub type IModel = ITextModel
;
}
pub mod languages {

#[allow(unused)]
use wasm_bindgen::prelude::*;
#[allow(unused)]
use web_sys::*;
#[allow(unused)]
use js_sys::*;
#[allow(unused)]
use super::*;

#[allow(unused)]
pub type HTMLElement = HtmlElement;
#[allow(unused)]
pub type ReadonlyArray = Array;
#[allow(unused)]
pub type NonNullable = JsValue;
#[allow(unused)]
pub type Record = JsValue;
#[allow(unused)]
pub type PromiseLike = JsValue;



#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IRelativePattern;

    /**
         * A base file path to which this pattern will be matched against relatively.
         */ 
    #[wasm_bindgen(method, js_class = "IRelativePattern", js_name = "base", getter)]
    pub fn base(this: &IRelativePattern) -> String;
    /**
         * A file glob pattern like `*.{ts,js}` that will be matched on file paths
         * relative to the base path.
         *
         * Example: Given a base of `/home/work/folder` and a file path of `/home/work/folder/index.js`,
         * the file glob pattern will match on `index.js`.
         */ 
    #[wasm_bindgen(method, js_class = "IRelativePattern", js_name = "pattern", getter)]
    pub fn pattern(this: &IRelativePattern) -> String;

}

pub type LanguageSelector = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LanguageFilter;

    #[wasm_bindgen(method, js_class = "LanguageFilter", js_name = "language", getter)]
    pub fn language(this: &LanguageFilter) -> String;
    #[wasm_bindgen(method, js_class = "LanguageFilter", js_name = "scheme", getter)]
    pub fn scheme(this: &LanguageFilter) -> String;
    #[wasm_bindgen(method, js_class = "LanguageFilter", js_name = "pattern", getter)]
    pub fn pattern(this: &LanguageFilter) -> JsValue;
    #[wasm_bindgen(method, js_class = "LanguageFilter", js_name = "notebookType", getter)]
    pub fn notebook_type(this: &LanguageFilter) -> String;
    /**
         * This provider is implemented in the UI thread.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageFilter", js_name = "hasAccessToAllModels", getter)]
    pub fn has_access_to_all_models(this: &LanguageFilter) -> bool;
    #[wasm_bindgen(method, js_class = "LanguageFilter", js_name = "exclusive", getter)]
    pub fn exclusive(this: &LanguageFilter) -> bool;
    /**
         * This provider comes from a builtin extension.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageFilter", js_name = "isBuiltin", getter)]
    pub fn is_builtin(this: &LanguageFilter) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IToken;

    #[wasm_bindgen(method, js_class = "IToken", js_name = "startIndex", getter)]
    pub fn start_index(this: &IToken) -> f64;

    #[wasm_bindgen(method, js_class = "IToken", js_name = "startIndex", setter)]
    pub fn set_start_index(this: &IToken, start_index: f64);

    #[wasm_bindgen(method, js_class = "IToken", js_name = "scopes", getter)]
    pub fn scopes(this: &IToken) -> String;

    #[wasm_bindgen(method, js_class = "IToken", js_name = "scopes", setter)]
    pub fn set_scopes(this: &IToken, scopes: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ILineTokens;

    /**
         * The list of tokens on the line.
         */ 
    #[wasm_bindgen(method, js_class = "ILineTokens", js_name = "tokens", getter)]
    pub fn tokens(this: &ILineTokens) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "ILineTokens", js_name = "tokens", setter)]
    pub fn set_tokens(this: &ILineTokens, tokens: js_sys::Array);

    /**
         * The tokenization end state.
         * A pointer will be held to this and the object should not be modified by the tokenizer after the pointer is returned.
         */ 
    #[wasm_bindgen(method, js_class = "ILineTokens", js_name = "endState", getter)]
    pub fn end_state(this: &ILineTokens) -> IState;

    #[wasm_bindgen(method, js_class = "ILineTokens", js_name = "endState", setter)]
    pub fn set_end_state(this: &ILineTokens, end_state: IState);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IEncodedLineTokens;

    /**
         * The tokens on the line in a binary, encoded format. Each token occupies two array indices. For token i:
         *  - at offset 2*i => startIndex
         *  - at offset 2*i + 1 => metadata
         * Meta data is in binary format:
         * - -------------------------------------------
         *     3322 2222 2222 1111 1111 1100 0000 0000
         *     1098 7654 3210 9876 5432 1098 7654 3210
         * - -------------------------------------------
         *     bbbb bbbb bfff ffff ffFF FFTT LLLL LLLL
         * - -------------------------------------------
         *  - L = EncodedLanguageId (8 bits): Use `getEncodedLanguageId` to get the encoded ID of a language.
         *  - T = StandardTokenType (2 bits): Other = 0, Comment = 1, String = 2, RegEx = 3.
         *  - F = FontStyle (4 bits): None = 0, Italic = 1, Bold = 2, Underline = 4, Strikethrough = 8.
         *  - f = foreground ColorId (9 bits)
         *  - b = background ColorId (9 bits)
         *  - The color value for each colorId is defined in IStandaloneThemeData.customTokenColors:
         * e.g. colorId = 1 is stored in IStandaloneThemeData.customTokenColors[1]. Color id = 0 means no color,
         * id = 1 is for the default foreground color, id = 2 for the default background.
         */ 
    #[wasm_bindgen(method, js_class = "IEncodedLineTokens", js_name = "tokens", getter)]
    pub fn tokens(this: &IEncodedLineTokens) -> Uint32Array;

    #[wasm_bindgen(method, js_class = "IEncodedLineTokens", js_name = "tokens", setter)]
    pub fn set_tokens(this: &IEncodedLineTokens, tokens: Uint32Array);

    /**
         * The tokenization end state.
         * A pointer will be held to this and the object should not be modified by the tokenizer after the pointer is returned.
         */ 
    #[wasm_bindgen(method, js_class = "IEncodedLineTokens", js_name = "endState", getter)]
    pub fn end_state(this: &IEncodedLineTokens) -> IState;

    #[wasm_bindgen(method, js_class = "IEncodedLineTokens", js_name = "endState", setter)]
    pub fn set_end_state(this: &IEncodedLineTokens, end_state: IState);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type TokensProviderFactory;

    #[wasm_bindgen(method, js_class = "TokensProviderFactory", js_name = "create")]
    pub fn create(this: &TokensProviderFactory, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type TokensProvider;

    /**
         * The initial state of a language. Will be the state passed in to tokenize the first line.
         */ 
    #[wasm_bindgen(method, js_class = "TokensProvider", js_name = "getInitialState")]
    pub fn get_initial_state(this: &TokensProvider, ) -> IState;

    /**
         * Tokenize a line given the state at the beginning of the line.
         */ 
    #[wasm_bindgen(method, js_class = "TokensProvider", js_name = "tokenize")]
    pub fn tokenize(this: &TokensProvider, line: String, state: IState, ) -> ILineTokens;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type EncodedTokensProvider;

    /**
         * The initial state of a language. Will be the state passed in to tokenize the first line.
         */ 
    #[wasm_bindgen(method, js_class = "EncodedTokensProvider", js_name = "getInitialState")]
    pub fn get_initial_state(this: &EncodedTokensProvider, ) -> IState;

    /**
         * Tokenize a line given the state at the beginning of the line.
         */ 
    #[wasm_bindgen(method, js_class = "EncodedTokensProvider", js_name = "tokenizeEncoded")]
    pub fn tokenize_encoded(this: &EncodedTokensProvider, line: String, state: IState, ) -> IEncodedLineTokens;

    /**
         * Tokenize a line given the state at the beginning of the line.
         */ 
    #[wasm_bindgen(method, js_class = "EncodedTokensProvider", js_name = "tokenize")]
    pub fn tokenize(this: &EncodedTokensProvider, line: String, state: IState, ) -> Option<ILineTokens>;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CodeActionContext;

    /**
         * An array of diagnostics.
         */ 
    #[wasm_bindgen(method, js_class = "CodeActionContext", js_name = "markers", getter)]
    pub fn markers(this: &CodeActionContext) -> js_sys::Array;
    /**
         * Requested kind of actions to return.
         */ 
    #[wasm_bindgen(method, js_class = "CodeActionContext", js_name = "only", getter)]
    pub fn only(this: &CodeActionContext) -> String;
    /**
         * The reason why code actions were requested.
         */ 
    #[wasm_bindgen(method, js_class = "CodeActionContext", js_name = "trigger", getter)]
    pub fn trigger(this: &CodeActionContext) -> CodeActionTriggerType;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CodeActionProvider;

    /**
         * Provide commands for the given document and range.
         */ 
    #[wasm_bindgen(method, js_class = "CodeActionProvider", js_name = "provideCodeActions")]
    pub fn provide_code_actions(this: &CodeActionProvider, model: editor::ITextModel, range: Range, context: CodeActionContext, token: CancellationToken, ) -> ProviderResult;

    /**
         * Given a code action fill in the edit. Will only invoked when missing.
         */ 
    #[wasm_bindgen(method, js_class = "CodeActionProvider", js_name = "resolveCodeAction")]
    pub fn resolve_code_action(this: &CodeActionProvider, code_action: CodeAction, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CodeActionProviderMetadata;

    /**
         * List of code action kinds that a {@link CodeActionProvider} may return.
         *
         * This list is used to determine if a given `CodeActionProvider` should be invoked or not.
         * To avoid unnecessary computation, every `CodeActionProvider` should list use `providedCodeActionKinds`. The
         * list of kinds may either be generic, such as `["quickfix", "refactor", "source"]`, or list out every kind provided,
         * such as `["quickfix.removeLine", "source.fixAll" ...]`.
         */ 
    #[wasm_bindgen(method, js_class = "CodeActionProviderMetadata", js_name = "providedCodeActionKinds", getter)]
    pub fn provided_code_action_kinds(this: &CodeActionProviderMetadata) -> JsValue;
    #[wasm_bindgen(method, js_class = "CodeActionProviderMetadata", js_name = "documentation", getter)]
    pub fn documentation(this: &CodeActionProviderMetadata) -> ReadonlyArray;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CommentRule;

    /**
         * The line comment token, like `// this is a comment`
         */ 
    #[wasm_bindgen(method, js_class = "CommentRule", js_name = "lineComment", getter)]
    pub fn line_comment(this: &CommentRule) -> JsValue;

    #[wasm_bindgen(method, js_class = "CommentRule", js_name = "lineComment", setter)]
    pub fn set_line_comment(this: &CommentRule, line_comment: JsValue);

    /**
         * The block comment character pair, like `\/\* block comment *&#47;`
         */ 
    #[wasm_bindgen(method, js_class = "CommentRule", js_name = "blockComment", getter)]
    pub fn block_comment(this: &CommentRule) -> JsValue;

    #[wasm_bindgen(method, js_class = "CommentRule", js_name = "blockComment", setter)]
    pub fn set_block_comment(this: &CommentRule, block_comment: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LanguageConfiguration;

    /**
         * The language's comment settings.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "comments", getter)]
    pub fn comments(this: &LanguageConfiguration) -> CommentRule;

    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "comments", setter)]
    pub fn set_comments(this: &LanguageConfiguration, comments: CommentRule);

    /**
         * The language's brackets.
         * This configuration implicitly affects pressing Enter around these brackets.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "brackets", getter)]
    pub fn brackets(this: &LanguageConfiguration) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "brackets", setter)]
    pub fn set_brackets(this: &LanguageConfiguration, brackets: js_sys::Array);

    /**
         * The language's word definition.
         * If the language supports Unicode identifiers (e.g. JavaScript), it is preferable
         * to provide a word definition that uses exclusion of known separators.
         * e.g.: A regex that matches anything except known separators (and dot is allowed to occur in a floating point number):
         *   /(-?\d*\.\d\w*)|([^\`\~\!\@\#\%\^\&\*\(\)\-\=\+\[\{\]\}\\\|\;\:\'\"\,\.\<\>\/\?\s]+)/g
         */ 
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "wordPattern", getter)]
    pub fn word_pattern(this: &LanguageConfiguration) -> RegExp;

    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "wordPattern", setter)]
    pub fn set_word_pattern(this: &LanguageConfiguration, word_pattern: RegExp);

    /**
         * The language's indentation settings.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "indentationRules", getter)]
    pub fn indentation_rules(this: &LanguageConfiguration) -> IndentationRule;

    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "indentationRules", setter)]
    pub fn set_indentation_rules(this: &LanguageConfiguration, indentation_rules: IndentationRule);

    /**
         * The language's rules to be evaluated when pressing Enter.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "onEnterRules", getter)]
    pub fn on_enter_rules(this: &LanguageConfiguration) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "onEnterRules", setter)]
    pub fn set_on_enter_rules(this: &LanguageConfiguration, on_enter_rules: js_sys::Array);

    /**
         * The language's auto closing pairs. The 'close' character is automatically inserted with the
         * 'open' character is typed. If not set, the configured brackets will be used.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "autoClosingPairs", getter)]
    pub fn auto_closing_pairs(this: &LanguageConfiguration) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "autoClosingPairs", setter)]
    pub fn set_auto_closing_pairs(this: &LanguageConfiguration, auto_closing_pairs: js_sys::Array);

    /**
         * The language's surrounding pairs. When the 'open' character is typed on a selection, the
         * selected string is surrounded by the open and close characters. If not set, the autoclosing pairs
         * settings will be used.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "surroundingPairs", getter)]
    pub fn surrounding_pairs(this: &LanguageConfiguration) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "surroundingPairs", setter)]
    pub fn set_surrounding_pairs(this: &LanguageConfiguration, surrounding_pairs: js_sys::Array);

    /**
         * Defines a list of bracket pairs that are colorized depending on their nesting level.
         * If not set, the configured brackets will be used.
        */ 
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "colorizedBracketPairs", getter)]
    pub fn colorized_bracket_pairs(this: &LanguageConfiguration) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "colorizedBracketPairs", setter)]
    pub fn set_colorized_bracket_pairs(this: &LanguageConfiguration, colorized_bracket_pairs: js_sys::Array);

    /**
         * Defines what characters must be after the cursor for bracket or quote autoclosing to occur when using the \'languageDefined\' autoclosing setting.
         *
         * This is typically the set of characters which can not start an expression, such as whitespace, closing brackets, non-unary operators, etc.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "autoCloseBefore", getter)]
    pub fn auto_close_before(this: &LanguageConfiguration) -> String;

    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "autoCloseBefore", setter)]
    pub fn set_auto_close_before(this: &LanguageConfiguration, auto_close_before: String);

    /**
         * The language's folding rules.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "folding", getter)]
    pub fn folding(this: &LanguageConfiguration) -> FoldingRules;

    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "folding", setter)]
    pub fn set_folding(this: &LanguageConfiguration, folding: FoldingRules);

    /**
         * **Deprecated** Do not use.
         *
         * @deprecated Will be replaced by a better API soon.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "__electricCharacterSupport", getter)]
    pub fn electric_character_support(this: &LanguageConfiguration) -> js_sys::Object;

    #[wasm_bindgen(method, js_class = "LanguageConfiguration", js_name = "__electricCharacterSupport", setter)]
    pub fn set_electric_character_support(this: &LanguageConfiguration, electric_character_support: js_sys::Object);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IndentationRule;

    /**
         * If a line matches this pattern, then all the lines after it should be unindented once (until another rule matches).
         */ 
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "decreaseIndentPattern", getter)]
    pub fn decrease_indent_pattern(this: &IndentationRule) -> RegExp;

    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "decreaseIndentPattern", setter)]
    pub fn set_decrease_indent_pattern(this: &IndentationRule, decrease_indent_pattern: RegExp);

    /**
         * If a line matches this pattern, then all the lines after it should be indented once (until another rule matches).
         */ 
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "increaseIndentPattern", getter)]
    pub fn increase_indent_pattern(this: &IndentationRule) -> RegExp;

    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "increaseIndentPattern", setter)]
    pub fn set_increase_indent_pattern(this: &IndentationRule, increase_indent_pattern: RegExp);

    /**
         * If a line matches this pattern, then **only the next line** after it should be indented once.
         */ 
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "indentNextLinePattern", getter)]
    pub fn indent_next_line_pattern(this: &IndentationRule) -> JsValue;

    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "indentNextLinePattern", setter)]
    pub fn set_indent_next_line_pattern(this: &IndentationRule, indent_next_line_pattern: JsValue);

    /**
         * If a line matches this pattern, then its indentation should not be changed and it should not be evaluated against the other rules.
         */ 
    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "unIndentedLinePattern", getter)]
    pub fn un_indented_line_pattern(this: &IndentationRule) -> JsValue;

    #[wasm_bindgen(method, js_class = "IndentationRule", js_name = "unIndentedLinePattern", setter)]
    pub fn set_un_indented_line_pattern(this: &IndentationRule, un_indented_line_pattern: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type FoldingMarkers;

    #[wasm_bindgen(method, js_class = "FoldingMarkers", js_name = "start", getter)]
    pub fn start(this: &FoldingMarkers) -> RegExp;

    #[wasm_bindgen(method, js_class = "FoldingMarkers", js_name = "start", setter)]
    pub fn set_start(this: &FoldingMarkers, start: RegExp);

    #[wasm_bindgen(method, js_class = "FoldingMarkers", js_name = "end", getter)]
    pub fn end(this: &FoldingMarkers) -> RegExp;

    #[wasm_bindgen(method, js_class = "FoldingMarkers", js_name = "end", setter)]
    pub fn set_end(this: &FoldingMarkers, end: RegExp);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type FoldingRules;

    /**
         * Used by the indentation based strategy to decide whether empty lines belong to the previous or the next block.
         * A language adheres to the off-side rule if blocks in that language are expressed by their indentation.
         * See [wikipedia](https://en.wikipedia.org/wiki/Off-side_rule) for more information.
         * If not set, `false` is used and empty lines belong to the previous block.
         */ 
    #[wasm_bindgen(method, js_class = "FoldingRules", js_name = "offSide", getter)]
    pub fn off_side(this: &FoldingRules) -> bool;

    #[wasm_bindgen(method, js_class = "FoldingRules", js_name = "offSide", setter)]
    pub fn set_off_side(this: &FoldingRules, off_side: bool);

    /**
         * Region markers used by the language.
         */ 
    #[wasm_bindgen(method, js_class = "FoldingRules", js_name = "markers", getter)]
    pub fn markers(this: &FoldingRules) -> FoldingMarkers;

    #[wasm_bindgen(method, js_class = "FoldingRules", js_name = "markers", setter)]
    pub fn set_markers(this: &FoldingRules, markers: FoldingMarkers);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type OnEnterRule;

    /**
         * This rule will only execute if the text before the cursor matches this regular expression.
         */ 
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "beforeText", getter)]
    pub fn before_text(this: &OnEnterRule) -> RegExp;

    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "beforeText", setter)]
    pub fn set_before_text(this: &OnEnterRule, before_text: RegExp);

    /**
         * This rule will only execute if the text after the cursor matches this regular expression.
         */ 
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "afterText", getter)]
    pub fn after_text(this: &OnEnterRule) -> RegExp;

    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "afterText", setter)]
    pub fn set_after_text(this: &OnEnterRule, after_text: RegExp);

    /**
         * This rule will only execute if the text above the this line matches this regular expression.
         */ 
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "previousLineText", getter)]
    pub fn previous_line_text(this: &OnEnterRule) -> RegExp;

    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "previousLineText", setter)]
    pub fn set_previous_line_text(this: &OnEnterRule, previous_line_text: RegExp);

    /**
         * The action to execute.
         */ 
    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "action", getter)]
    pub fn action(this: &OnEnterRule) -> EnterAction;

    #[wasm_bindgen(method, js_class = "OnEnterRule", js_name = "action", setter)]
    pub fn set_action(this: &OnEnterRule, action: EnterAction);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IDocComment;

    /**
         * The string that starts a doc comment (e.g. '\/\**')
         */ 
    #[wasm_bindgen(method, js_class = "IDocComment", js_name = "open", getter)]
    pub fn open(this: &IDocComment) -> String;

    #[wasm_bindgen(method, js_class = "IDocComment", js_name = "open", setter)]
    pub fn set_open(this: &IDocComment, open: String);

    /**
         * The string that appears on the last line and closes the doc comment (e.g. ' * /').
         */ 
    #[wasm_bindgen(method, js_class = "IDocComment", js_name = "close", getter)]
    pub fn close(this: &IDocComment) -> String;

    #[wasm_bindgen(method, js_class = "IDocComment", js_name = "close", setter)]
    pub fn set_close(this: &IDocComment, close: String);


}

pub type CharacterPair = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IAutoClosingPair;

    #[wasm_bindgen(method, js_class = "IAutoClosingPair", js_name = "open", getter)]
    pub fn open(this: &IAutoClosingPair) -> String;

    #[wasm_bindgen(method, js_class = "IAutoClosingPair", js_name = "open", setter)]
    pub fn set_open(this: &IAutoClosingPair, open: String);

    #[wasm_bindgen(method, js_class = "IAutoClosingPair", js_name = "close", getter)]
    pub fn close(this: &IAutoClosingPair) -> String;

    #[wasm_bindgen(method, js_class = "IAutoClosingPair", js_name = "close", setter)]
    pub fn set_close(this: &IAutoClosingPair, close: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IAutoClosingPairConditional;

    #[wasm_bindgen(method, js_class = "IAutoClosingPairConditional", js_name = "notIn", getter)]
    pub fn not_in(this: &IAutoClosingPairConditional) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IAutoClosingPairConditional", js_name = "notIn", setter)]
    pub fn set_not_in(this: &IAutoClosingPairConditional, not_in: js_sys::Array);


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum IndentAction {
    /**
         * Insert new line and copy the previous line's indentation.
         */ 
    None = 0,
    /**
         * Insert new line and indent once (relative to the previous line's indentation).
         */ 
    Indent = 1,
    /**
         * Insert two new lines:
         *  - the first one indented which will hold the cursor
         *  - the second one at the same indentation level
         */ 
    IndentOutdent = 2,
    /**
         * Insert new line and outdent once (relative to the previous line's indentation).
         */ 
    Outdent = 3,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type EnterAction;

    /**
         * Describe what to do with the indentation.
         */ 
    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "indentAction", getter)]
    pub fn indent_action(this: &EnterAction) -> IndentAction;

    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "indentAction", setter)]
    pub fn set_indent_action(this: &EnterAction, indent_action: IndentAction);

    /**
         * Describes text to be appended after the new line and after the indentation.
         */ 
    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "appendText", getter)]
    pub fn append_text(this: &EnterAction) -> String;

    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "appendText", setter)]
    pub fn set_append_text(this: &EnterAction, append_text: String);

    /**
         * Describes the number of characters to remove from the new line's indentation.
         */ 
    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "removeText", getter)]
    pub fn remove_text(this: &EnterAction) -> f64;

    #[wasm_bindgen(method, js_class = "EnterAction", js_name = "removeText", setter)]
    pub fn set_remove_text(this: &EnterAction, remove_text: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IState;

    #[wasm_bindgen(method, js_class = "IState", js_name = "clone")]
    pub fn clone(this: &IState, ) -> IState;

    #[wasm_bindgen(method, js_class = "IState", js_name = "equals")]
    pub fn equals(this: &IState, other: IState, ) -> bool;


}

pub type ProviderResult = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Hover;

    /**
         * The contents of this hover.
         */ 
    #[wasm_bindgen(method, js_class = "Hover", js_name = "contents", getter)]
    pub fn contents(this: &Hover) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "Hover", js_name = "contents", setter)]
    pub fn set_contents(this: &Hover, contents: js_sys::Array);

    /**
         * The range to which this hover applies. When missing, the
         * editor will use the range at the current position or the
         * current position itself.
         */ 
    #[wasm_bindgen(method, js_class = "Hover", js_name = "range", getter)]
    pub fn range(this: &Hover) -> IRange;

    #[wasm_bindgen(method, js_class = "Hover", js_name = "range", setter)]
    pub fn set_range(this: &Hover, range: IRange);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type HoverProvider;

    /**
         * Provide a hover for the given position and document. Multiple hovers at the same
         * position will be merged by the editor. A hover can have a range which defaults
         * to the word range at the position when omitted.
         */ 
    #[wasm_bindgen(method, js_class = "HoverProvider", js_name = "provideHover")]
    pub fn provide_hover(this: &HoverProvider, model: editor::ITextModel, position: Position, token: CancellationToken, ) -> ProviderResult;


}


            int_enum! {
#[allow(non_camel_case_types)]
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
    EnumMember = 16,
    Keyword = 17,
    Text = 18,
    Color = 19,
    File = 20,
    Reference = 21,
    Customcolor = 22,
    Folder = 23,
    TypeParameter = 24,
    User = 25,
    Issue = 26,
    Snippet = 27,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CompletionItemLabel;

    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "label", getter)]
    pub fn label(this: &CompletionItemLabel) -> String;

    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "label", setter)]
    pub fn set_label(this: &CompletionItemLabel, label: String);

    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "detail", getter)]
    pub fn detail(this: &CompletionItemLabel) -> String;

    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "detail", setter)]
    pub fn set_detail(this: &CompletionItemLabel, detail: String);

    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "description", getter)]
    pub fn description(this: &CompletionItemLabel) -> String;

    #[wasm_bindgen(method, js_class = "CompletionItemLabel", js_name = "description", setter)]
    pub fn set_description(this: &CompletionItemLabel, description: String);


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum CompletionItemTag {
    Deprecated = 1,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum CompletionItemInsertTextRule {
    None = 0,
    /**
         * Adjust whitespace/indentation of multiline insert texts to
         * match the current line indentation.
         */ 
    KeepWhitespace = 1,
    /**
         * `insertText` is a snippet.
         */ 
    InsertAsSnippet = 4,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CompletionItemRanges;

    #[wasm_bindgen(method, js_class = "CompletionItemRanges", js_name = "insert", getter)]
    pub fn insert(this: &CompletionItemRanges) -> IRange;

    #[wasm_bindgen(method, js_class = "CompletionItemRanges", js_name = "insert", setter)]
    pub fn set_insert(this: &CompletionItemRanges, insert: IRange);

    #[wasm_bindgen(method, js_class = "CompletionItemRanges", js_name = "replace", getter)]
    pub fn replace(this: &CompletionItemRanges) -> IRange;

    #[wasm_bindgen(method, js_class = "CompletionItemRanges", js_name = "replace", setter)]
    pub fn set_replace(this: &CompletionItemRanges, replace: IRange);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CompletionItem;

    /**
         * The label of this completion item. By default
         * this is also the text that is inserted when selecting
         * this completion.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "label", getter)]
    pub fn label(this: &CompletionItem) -> JsValue;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "label", setter)]
    pub fn set_label(this: &CompletionItem, label: JsValue);

    /**
         * The kind of this completion item. Based on the kind
         * an icon is chosen by the editor.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "kind", getter)]
    pub fn kind(this: &CompletionItem) -> CompletionItemKind;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "kind", setter)]
    pub fn set_kind(this: &CompletionItem, kind: CompletionItemKind);

    /**
         * A modifier to the `kind` which affect how the item
         * is rendered, e.g. Deprecated is rendered with a strikeout
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "tags", getter)]
    pub fn tags(this: &CompletionItem) -> ReadonlyArray;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "tags", setter)]
    pub fn set_tags(this: &CompletionItem, tags: ReadonlyArray);

    /**
         * A human-readable string with additional information
         * about this item, like type or symbol information.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "detail", getter)]
    pub fn detail(this: &CompletionItem) -> String;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "detail", setter)]
    pub fn set_detail(this: &CompletionItem, detail: String);

    /**
         * A human-readable string that represents a doc-comment.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "documentation", getter)]
    pub fn documentation(this: &CompletionItem) -> JsValue;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "documentation", setter)]
    pub fn set_documentation(this: &CompletionItem, documentation: JsValue);

    /**
         * A string that should be used when comparing this item
         * with other items. When `falsy` the {@link CompletionItem.label label}
         * is used.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "sortText", getter)]
    pub fn sort_text(this: &CompletionItem) -> String;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "sortText", setter)]
    pub fn set_sort_text(this: &CompletionItem, sort_text: String);

    /**
         * A string that should be used when filtering a set of
         * completion items. When `falsy` the {@link CompletionItem.label label}
         * is used.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "filterText", getter)]
    pub fn filter_text(this: &CompletionItem) -> String;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "filterText", setter)]
    pub fn set_filter_text(this: &CompletionItem, filter_text: String);

    /**
         * Select this item when showing. *Note* that only one completion item can be selected and
         * that the editor decides which item that is. The rule is that the *first* item of those
         * that match best is selected.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "preselect", getter)]
    pub fn preselect(this: &CompletionItem) -> bool;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "preselect", setter)]
    pub fn set_preselect(this: &CompletionItem, preselect: bool);

    /**
         * A string or snippet that should be inserted in a document when selecting
         * this completion.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "insertText", getter)]
    pub fn insert_text(this: &CompletionItem) -> String;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "insertText", setter)]
    pub fn set_insert_text(this: &CompletionItem, insert_text: String);

    /**
         * Additional rules (as bitmask) that should be applied when inserting
         * this completion.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "insertTextRules", getter)]
    pub fn insert_text_rules(this: &CompletionItem) -> CompletionItemInsertTextRule;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "insertTextRules", setter)]
    pub fn set_insert_text_rules(this: &CompletionItem, insert_text_rules: CompletionItemInsertTextRule);

    /**
         * A range of text that should be replaced by this completion item.
         *
         * Defaults to a range from the start of the {@link TextDocument.getWordRangeAtPosition current word} to the
         * current position.
         *
         * *Note:* The range must be a {@link Range.isSingleLine single line} and it must
         * {@link Range.contains contain} the position at which completion has been {@link CompletionItemProvider.provideCompletionItems requested}.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "range", getter)]
    pub fn range(this: &CompletionItem) -> JsValue;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "range", setter)]
    pub fn set_range(this: &CompletionItem, range: JsValue);

    /**
         * An optional set of characters that when pressed while this completion is active will accept it first and
         * then type that character. *Note* that all commit characters should have `length=1` and that superfluous
         * characters will be ignored.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "commitCharacters", getter)]
    pub fn commit_characters(this: &CompletionItem) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "commitCharacters", setter)]
    pub fn set_commit_characters(this: &CompletionItem, commit_characters: js_sys::Array);

    /**
         * An optional array of additional text edits that are applied when
         * selecting this completion. Edits must not overlap with the main edit
         * nor with themselves.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "additionalTextEdits", getter)]
    pub fn additional_text_edits(this: &CompletionItem) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "additionalTextEdits", setter)]
    pub fn set_additional_text_edits(this: &CompletionItem, additional_text_edits: js_sys::Array);

    /**
         * A command that should be run upon acceptance of this item.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "command", getter)]
    pub fn command(this: &CompletionItem) -> Command;

    #[wasm_bindgen(method, js_class = "CompletionItem", js_name = "command", setter)]
    pub fn set_command(this: &CompletionItem, command: Command);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CompletionList;

    #[wasm_bindgen(method, js_class = "CompletionList", js_name = "suggestions", getter)]
    pub fn suggestions(this: &CompletionList) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CompletionList", js_name = "suggestions", setter)]
    pub fn set_suggestions(this: &CompletionList, suggestions: js_sys::Array);

    #[wasm_bindgen(method, js_class = "CompletionList", js_name = "incomplete", getter)]
    pub fn incomplete(this: &CompletionList) -> bool;

    #[wasm_bindgen(method, js_class = "CompletionList", js_name = "incomplete", setter)]
    pub fn set_incomplete(this: &CompletionList, incomplete: bool);

    #[wasm_bindgen(method, js_class = "CompletionList", js_name = "dispose")]
    pub fn dispose(this: &CompletionList, );


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum CompletionTriggerKind {
    Invoke = 0,
    TriggerCharacter = 1,
    TriggerForIncompleteCompletions = 2,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CompletionContext;

    /**
         * How the completion was triggered.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionContext", js_name = "triggerKind", getter)]
    pub fn trigger_kind(this: &CompletionContext) -> CompletionTriggerKind;

    #[wasm_bindgen(method, js_class = "CompletionContext", js_name = "triggerKind", setter)]
    pub fn set_trigger_kind(this: &CompletionContext, trigger_kind: CompletionTriggerKind);

    /**
         * Character that triggered the completion item provider.
         *
         * `undefined` if provider was not triggered by a character.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionContext", js_name = "triggerCharacter", getter)]
    pub fn trigger_character(this: &CompletionContext) -> String;

    #[wasm_bindgen(method, js_class = "CompletionContext", js_name = "triggerCharacter", setter)]
    pub fn set_trigger_character(this: &CompletionContext, trigger_character: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CompletionItemProvider;

    #[wasm_bindgen(method, js_class = "CompletionItemProvider", js_name = "triggerCharacters", getter)]
    pub fn trigger_characters(this: &CompletionItemProvider) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CompletionItemProvider", js_name = "triggerCharacters", setter)]
    pub fn set_trigger_characters(this: &CompletionItemProvider, trigger_characters: js_sys::Array);

    /**
         * Provide completion items for the given position and document.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItemProvider", js_name = "provideCompletionItems")]
    pub fn provide_completion_items(this: &CompletionItemProvider, model: editor::ITextModel, position: Position, context: CompletionContext, token: CancellationToken, ) -> ProviderResult;

    /**
         * Given a completion item fill in more data, like {@link CompletionItem.documentation doc-comment}
         * or {@link CompletionItem.detail details}.
         *
         * The editor will only resolve a completion item once.
         */ 
    #[wasm_bindgen(method, js_class = "CompletionItemProvider", js_name = "resolveCompletionItem")]
    pub fn resolve_completion_item(this: &CompletionItemProvider, item: CompletionItem, token: CancellationToken, ) -> ProviderResult;


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum InlineCompletionTriggerKind {
    /**
         * Completion was triggered automatically while editing.
         * It is sufficient to return a single completion item in this case.
         */ 
    Automatic = 0,
    /**
         * Completion was triggered explicitly by a user gesture.
         * Return multiple completion items to enable cycling through them.
         */ 
    Explicit = 1,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InlineCompletionContext;

    /**
         * How the completion was triggered.
         */ 
    #[wasm_bindgen(method, js_class = "InlineCompletionContext", js_name = "triggerKind", getter)]
    pub fn trigger_kind(this: &InlineCompletionContext) -> InlineCompletionTriggerKind;
    #[wasm_bindgen(method, js_class = "InlineCompletionContext", js_name = "selectedSuggestionInfo", getter)]
    pub fn selected_suggestion_info(this: &InlineCompletionContext) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SelectedSuggestionInfo;

  #[wasm_bindgen(constructor, js_class = "SelectedSuggestionInfo")]
  pub fn new() -> SelectedSuggestionInfo;


    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", method)]
    pub fn equals(this: &SelectedSuggestionInfo, other: SelectedSuggestionInfo, ) -> bool;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InlineCompletion;

    /**
         * The text to insert.
         * If the text contains a line break, the range must end at the end of a line.
         * If existing text should be replaced, the existing text must be a prefix of the text to insert.
         *
         * The text can also be a snippet. In that case, a preview with default parameters is shown.
         * When accepting the suggestion, the full snippet is inserted.
        */ 
    #[wasm_bindgen(method, js_class = "InlineCompletion", js_name = "insertText", getter)]
    pub fn insert_text(this: &InlineCompletion) -> JsValue;
    /**
         * A text that is used to decide if this inline completion should be shown.
         * An inline completion is shown if the text to replace is a subword of the filter text.
         */ 
    #[wasm_bindgen(method, js_class = "InlineCompletion", js_name = "filterText", getter)]
    pub fn filter_text(this: &InlineCompletion) -> String;
    /**
         * An optional array of additional text edits that are applied when
         * selecting this completion. Edits must not overlap with the main edit
         * nor with themselves.
         */ 
    #[wasm_bindgen(method, js_class = "InlineCompletion", js_name = "additionalTextEdits", getter)]
    pub fn additional_text_edits(this: &InlineCompletion) -> js_sys::Array;
    /**
         * The range to replace.
         * Must begin and end on the same line.
        */ 
    #[wasm_bindgen(method, js_class = "InlineCompletion", js_name = "range", getter)]
    pub fn range(this: &InlineCompletion) -> IRange;
    #[wasm_bindgen(method, js_class = "InlineCompletion", js_name = "command", getter)]
    pub fn command(this: &InlineCompletion) -> Command;
    /**
         * If set to `true`, unopened closing brackets are removed and unclosed opening brackets are closed.
         * Defaults to `false`.
        */ 
    #[wasm_bindgen(method, js_class = "InlineCompletion", js_name = "completeBracketPairs", getter)]
    pub fn complete_bracket_pairs(this: &InlineCompletion) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InlineCompletions;

    #[wasm_bindgen(method, js_class = "InlineCompletions", js_name = "items", getter)]
    pub fn items(this: &InlineCompletions) -> JsValue;
    /**
         * A list of commands associated with the inline completions of this list.
         */ 
    #[wasm_bindgen(method, js_class = "InlineCompletions", js_name = "commands", getter)]
    pub fn commands(this: &InlineCompletions) -> js_sys::Array;
    #[wasm_bindgen(method, js_class = "InlineCompletions", js_name = "suppressSuggestions", getter)]
    pub fn suppress_suggestions(this: &InlineCompletions) -> JsValue;
    /**
         * When set and the user types a suggestion without derivating from it, the inline suggestion is not updated.
         */ 
    #[wasm_bindgen(method, js_class = "InlineCompletions", js_name = "enableForwardStability", getter)]
    pub fn enable_forward_stability(this: &InlineCompletions) -> JsValue;

}

pub type InlineCompletionProviderGroupId = String
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InlineCompletionsProvider;

    #[wasm_bindgen(method, js_class = "InlineCompletionsProvider", js_name = "provideInlineCompletions")]
    pub fn provide_inline_completions(this: &InlineCompletionsProvider, model: editor::ITextModel, position: Position, context: InlineCompletionContext, token: CancellationToken, ) -> ProviderResult;

    /**
         * Will be called when an item is shown.
         * @param updatedInsertText Is useful to understand bracket completion.
        */ 
    #[wasm_bindgen(method, js_class = "InlineCompletionsProvider", js_name = "handleItemDidShow")]
    pub fn handle_item_did_show(this: &InlineCompletionsProvider, completions: JsValue, item: JsValue, updated_insert_text: String, );

    /**
         * Will be called when an item is partially accepted.
         */ 
    #[wasm_bindgen(method, js_class = "InlineCompletionsProvider", js_name = "handlePartialAccept")]
    pub fn handle_partial_accept(this: &InlineCompletionsProvider, completions: JsValue, item: JsValue, accepted_characters: f64, );

    /**
         * Will be called when a completions list is no longer in use and can be garbage-collected.
        */ 
    #[wasm_bindgen(method, js_class = "InlineCompletionsProvider", js_name = "freeInlineCompletions")]
    pub fn free_inline_completions(this: &InlineCompletionsProvider, completions: JsValue, );

    /**
         * Only used for {@link yieldsToGroupIds}.
         * Multiple providers can have the same group id.
         */ 
    #[wasm_bindgen(method, js_class = "InlineCompletionsProvider", js_name = "groupId", getter)]
    pub fn group_id(this: &InlineCompletionsProvider) -> InlineCompletionProviderGroupId;

    #[wasm_bindgen(method, js_class = "InlineCompletionsProvider", js_name = "groupId", setter)]
    pub fn set_group_id(this: &InlineCompletionsProvider, group_id: InlineCompletionProviderGroupId);

    /**
         * Returns a list of preferred provider {@link groupId}s.
         * The current provider is only requested for completions if no provider with a preferred group id returned a result.
         */ 
    #[wasm_bindgen(method, js_class = "InlineCompletionsProvider", js_name = "yieldsToGroupIds", getter)]
    pub fn yields_to_group_ids(this: &InlineCompletionsProvider) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "InlineCompletionsProvider", js_name = "yieldsToGroupIds", setter)]
    pub fn set_yields_to_group_ids(this: &InlineCompletionsProvider, yields_to_group_ids: js_sys::Array);

    #[wasm_bindgen(method, js_class = "InlineCompletionsProvider", js_name = "toString")]
    pub fn to_string(this: &InlineCompletionsProvider, ) -> Option<String>;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CodeAction;

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "title", getter)]
    pub fn title(this: &CodeAction) -> String;

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "title", setter)]
    pub fn set_title(this: &CodeAction, title: String);

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "command", getter)]
    pub fn command(this: &CodeAction) -> Command;

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "command", setter)]
    pub fn set_command(this: &CodeAction, command: Command);

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "edit", getter)]
    pub fn edit(this: &CodeAction) -> WorkspaceEdit;

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "edit", setter)]
    pub fn set_edit(this: &CodeAction, edit: WorkspaceEdit);

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "diagnostics", getter)]
    pub fn diagnostics(this: &CodeAction) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "diagnostics", setter)]
    pub fn set_diagnostics(this: &CodeAction, diagnostics: js_sys::Array);

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "kind", getter)]
    pub fn kind(this: &CodeAction) -> String;

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "kind", setter)]
    pub fn set_kind(this: &CodeAction, kind: String);

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "isPreferred", getter)]
    pub fn is_preferred(this: &CodeAction) -> bool;

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "isPreferred", setter)]
    pub fn set_is_preferred(this: &CodeAction, is_preferred: bool);

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "disabled", getter)]
    pub fn disabled(this: &CodeAction) -> String;

    #[wasm_bindgen(method, js_class = "CodeAction", js_name = "disabled", setter)]
    pub fn set_disabled(this: &CodeAction, disabled: String);


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum CodeActionTriggerType {
    Invoke = 1,
    Auto = 2,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CodeActionList;

    #[wasm_bindgen(method, js_class = "CodeActionList", js_name = "actions", getter)]
    pub fn actions(this: &CodeActionList) -> ReadonlyArray;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ParameterInformation;

    /**
         * The label of this signature. Will be shown in
         * the UI.
         */ 
    #[wasm_bindgen(method, js_class = "ParameterInformation", js_name = "label", getter)]
    pub fn label(this: &ParameterInformation) -> JsValue;

    #[wasm_bindgen(method, js_class = "ParameterInformation", js_name = "label", setter)]
    pub fn set_label(this: &ParameterInformation, label: JsValue);

    /**
         * The human-readable doc-comment of this signature. Will be shown
         * in the UI but can be omitted.
         */ 
    #[wasm_bindgen(method, js_class = "ParameterInformation", js_name = "documentation", getter)]
    pub fn documentation(this: &ParameterInformation) -> JsValue;

    #[wasm_bindgen(method, js_class = "ParameterInformation", js_name = "documentation", setter)]
    pub fn set_documentation(this: &ParameterInformation, documentation: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SignatureInformation;

    /**
         * The label of this signature. Will be shown in
         * the UI.
         */ 
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "label", getter)]
    pub fn label(this: &SignatureInformation) -> String;

    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "label", setter)]
    pub fn set_label(this: &SignatureInformation, label: String);

    /**
         * The human-readable doc-comment of this signature. Will be shown
         * in the UI but can be omitted.
         */ 
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "documentation", getter)]
    pub fn documentation(this: &SignatureInformation) -> JsValue;

    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "documentation", setter)]
    pub fn set_documentation(this: &SignatureInformation, documentation: JsValue);

    /**
         * The parameters of this signature.
         */ 
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "parameters", getter)]
    pub fn parameters(this: &SignatureInformation) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "parameters", setter)]
    pub fn set_parameters(this: &SignatureInformation, parameters: js_sys::Array);

    /**
         * Index of the active parameter.
         *
         * If provided, this is used in place of `SignatureHelp.activeSignature`.
         */ 
    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "activeParameter", getter)]
    pub fn active_parameter(this: &SignatureInformation) -> f64;

    #[wasm_bindgen(method, js_class = "SignatureInformation", js_name = "activeParameter", setter)]
    pub fn set_active_parameter(this: &SignatureInformation, active_parameter: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SignatureHelp;

    /**
         * One or more signatures.
         */ 
    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "signatures", getter)]
    pub fn signatures(this: &SignatureHelp) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "signatures", setter)]
    pub fn set_signatures(this: &SignatureHelp, signatures: js_sys::Array);

    /**
         * The active signature.
         */ 
    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "activeSignature", getter)]
    pub fn active_signature(this: &SignatureHelp) -> f64;

    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "activeSignature", setter)]
    pub fn set_active_signature(this: &SignatureHelp, active_signature: f64);

    /**
         * The active parameter of the active signature.
         */ 
    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "activeParameter", getter)]
    pub fn active_parameter(this: &SignatureHelp) -> f64;

    #[wasm_bindgen(method, js_class = "SignatureHelp", js_name = "activeParameter", setter)]
    pub fn set_active_parameter(this: &SignatureHelp, active_parameter: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SignatureHelpResult;

    #[wasm_bindgen(method, js_class = "SignatureHelpResult", js_name = "value", getter)]
    pub fn value(this: &SignatureHelpResult) -> SignatureHelp;

    #[wasm_bindgen(method, js_class = "SignatureHelpResult", js_name = "value", setter)]
    pub fn set_value(this: &SignatureHelpResult, value: SignatureHelp);


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum SignatureHelpTriggerKind {
    Invoke = 1,
    TriggerCharacter = 2,
    ContentChange = 3,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SignatureHelpContext;

    #[wasm_bindgen(method, js_class = "SignatureHelpContext", js_name = "triggerKind", getter)]
    pub fn trigger_kind(this: &SignatureHelpContext) -> SignatureHelpTriggerKind;
    #[wasm_bindgen(method, js_class = "SignatureHelpContext", js_name = "triggerCharacter", getter)]
    pub fn trigger_character(this: &SignatureHelpContext) -> String;
    #[wasm_bindgen(method, js_class = "SignatureHelpContext", js_name = "isRetrigger", getter)]
    pub fn is_retrigger(this: &SignatureHelpContext) -> bool;
    #[wasm_bindgen(method, js_class = "SignatureHelpContext", js_name = "activeSignatureHelp", getter)]
    pub fn active_signature_help(this: &SignatureHelpContext) -> SignatureHelp;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SignatureHelpProvider;

    #[wasm_bindgen(method, js_class = "SignatureHelpProvider", js_name = "signatureHelpTriggerCharacters", getter)]
    pub fn signature_help_trigger_characters(this: &SignatureHelpProvider) -> ReadonlyArray;
    #[wasm_bindgen(method, js_class = "SignatureHelpProvider", js_name = "signatureHelpRetriggerCharacters", getter)]
    pub fn signature_help_retrigger_characters(this: &SignatureHelpProvider) -> ReadonlyArray;
    /**
         * Provide help for the signature at the given position and document.
         */ 
    #[wasm_bindgen(method, js_class = "SignatureHelpProvider", js_name = "provideSignatureHelp")]
    pub fn provide_signature_help(this: &SignatureHelpProvider, model: editor::ITextModel, position: Position, token: CancellationToken, context: SignatureHelpContext, ) -> ProviderResult;


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum DocumentHighlightKind {
    /**
         * A textual occurrence.
         */ 
    Text = 0,
    /**
         * Read-access of a symbol, like reading a variable.
         */ 
    Read = 1,
    /**
         * Write-access of a symbol, like writing to a variable.
         */ 
    Write = 2,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DocumentHighlight;

    /**
         * The range this highlight applies to.
         */ 
    #[wasm_bindgen(method, js_class = "DocumentHighlight", js_name = "range", getter)]
    pub fn range(this: &DocumentHighlight) -> IRange;

    #[wasm_bindgen(method, js_class = "DocumentHighlight", js_name = "range", setter)]
    pub fn set_range(this: &DocumentHighlight, range: IRange);

    /**
         * The highlight kind, default is {@link DocumentHighlightKind.Text text}.
         */ 
    #[wasm_bindgen(method, js_class = "DocumentHighlight", js_name = "kind", getter)]
    pub fn kind(this: &DocumentHighlight) -> DocumentHighlightKind;

    #[wasm_bindgen(method, js_class = "DocumentHighlight", js_name = "kind", setter)]
    pub fn set_kind(this: &DocumentHighlight, kind: DocumentHighlightKind);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DocumentHighlightProvider;

    /**
         * Provide a set of document highlights, like all occurrences of a variable or
         * all exit-points of a function.
         */ 
    #[wasm_bindgen(method, js_class = "DocumentHighlightProvider", js_name = "provideDocumentHighlights")]
    pub fn provide_document_highlights(this: &DocumentHighlightProvider, model: editor::ITextModel, position: Position, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LinkedEditingRangeProvider;

    /**
         * Provide a list of ranges that can be edited together.
         */ 
    #[wasm_bindgen(method, js_class = "LinkedEditingRangeProvider", js_name = "provideLinkedEditingRanges")]
    pub fn provide_linked_editing_ranges(this: &LinkedEditingRangeProvider, model: editor::ITextModel, position: Position, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LinkedEditingRanges;

    /**
         * A list of ranges that can be edited together. The ranges must have
         * identical length and text content. The ranges cannot overlap
         */ 
    #[wasm_bindgen(method, js_class = "LinkedEditingRanges", js_name = "ranges", getter)]
    pub fn ranges(this: &LinkedEditingRanges) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "LinkedEditingRanges", js_name = "ranges", setter)]
    pub fn set_ranges(this: &LinkedEditingRanges, ranges: js_sys::Array);

    /**
         * An optional word pattern that describes valid contents for the given ranges.
         * If no pattern is provided, the language configuration's word pattern will be used.
         */ 
    #[wasm_bindgen(method, js_class = "LinkedEditingRanges", js_name = "wordPattern", getter)]
    pub fn word_pattern(this: &LinkedEditingRanges) -> RegExp;

    #[wasm_bindgen(method, js_class = "LinkedEditingRanges", js_name = "wordPattern", setter)]
    pub fn set_word_pattern(this: &LinkedEditingRanges, word_pattern: RegExp);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ReferenceContext;

    /**
         * Include the declaration of the current symbol.
         */ 
    #[wasm_bindgen(method, js_class = "ReferenceContext", js_name = "includeDeclaration", getter)]
    pub fn include_declaration(this: &ReferenceContext) -> bool;

    #[wasm_bindgen(method, js_class = "ReferenceContext", js_name = "includeDeclaration", setter)]
    pub fn set_include_declaration(this: &ReferenceContext, include_declaration: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ReferenceProvider;

    /**
         * Provide a set of project-wide references for the given position and document.
         */ 
    #[wasm_bindgen(method, js_class = "ReferenceProvider", js_name = "provideReferences")]
    pub fn provide_references(this: &ReferenceProvider, model: editor::ITextModel, position: Position, context: ReferenceContext, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Location;

    /**
         * The resource identifier of this location.
         */ 
    #[wasm_bindgen(method, js_class = "Location", js_name = "uri", getter)]
    pub fn uri(this: &Location) -> Uri;

    #[wasm_bindgen(method, js_class = "Location", js_name = "uri", setter)]
    pub fn set_uri(this: &Location, uri: Uri);

    /**
         * The document range of this locations.
         */ 
    #[wasm_bindgen(method, js_class = "Location", js_name = "range", getter)]
    pub fn range(this: &Location) -> IRange;

    #[wasm_bindgen(method, js_class = "Location", js_name = "range", setter)]
    pub fn set_range(this: &Location, range: IRange);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LocationLink;

    /**
         * A range to select where this link originates from.
         */ 
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "originSelectionRange", getter)]
    pub fn origin_selection_range(this: &LocationLink) -> IRange;

    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "originSelectionRange", setter)]
    pub fn set_origin_selection_range(this: &LocationLink, origin_selection_range: IRange);

    /**
         * The target uri this link points to.
         */ 
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "uri", getter)]
    pub fn uri(this: &LocationLink) -> Uri;

    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "uri", setter)]
    pub fn set_uri(this: &LocationLink, uri: Uri);

    /**
         * The full range this link points to.
         */ 
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "range", getter)]
    pub fn range(this: &LocationLink) -> IRange;

    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "range", setter)]
    pub fn set_range(this: &LocationLink, range: IRange);

    /**
         * A range to select this link points to. Must be contained
         * in `LocationLink.range`.
         */ 
    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "targetSelectionRange", getter)]
    pub fn target_selection_range(this: &LocationLink) -> IRange;

    #[wasm_bindgen(method, js_class = "LocationLink", js_name = "targetSelectionRange", setter)]
    pub fn set_target_selection_range(this: &LocationLink, target_selection_range: IRange);


}

pub type Definition = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DefinitionProvider;

    /**
         * Provide the definition of the symbol at the given position and document.
         */ 
    #[wasm_bindgen(method, js_class = "DefinitionProvider", js_name = "provideDefinition")]
    pub fn provide_definition(this: &DefinitionProvider, model: editor::ITextModel, position: Position, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DeclarationProvider;

    /**
         * Provide the declaration of the symbol at the given position and document.
         */ 
    #[wasm_bindgen(method, js_class = "DeclarationProvider", js_name = "provideDeclaration")]
    pub fn provide_declaration(this: &DeclarationProvider, model: editor::ITextModel, position: Position, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ImplementationProvider;

    /**
         * Provide the implementation of the symbol at the given position and document.
         */ 
    #[wasm_bindgen(method, js_class = "ImplementationProvider", js_name = "provideImplementation")]
    pub fn provide_implementation(this: &ImplementationProvider, model: editor::ITextModel, position: Position, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type TypeDefinitionProvider;

    /**
         * Provide the type definition of the symbol at the given position and document.
         */ 
    #[wasm_bindgen(method, js_class = "TypeDefinitionProvider", js_name = "provideTypeDefinition")]
    pub fn provide_type_definition(this: &TypeDefinitionProvider, model: editor::ITextModel, position: Position, token: CancellationToken, ) -> ProviderResult;


}


            int_enum! {
#[allow(non_camel_case_types)]
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
    EnumMember = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum SymbolTag {
    Deprecated = 1,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DocumentSymbol;

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "name", getter)]
    pub fn name(this: &DocumentSymbol) -> String;

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "name", setter)]
    pub fn set_name(this: &DocumentSymbol, name: String);

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "detail", getter)]
    pub fn detail(this: &DocumentSymbol) -> String;

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "detail", setter)]
    pub fn set_detail(this: &DocumentSymbol, detail: String);

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "kind", getter)]
    pub fn kind(this: &DocumentSymbol) -> SymbolKind;

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "kind", setter)]
    pub fn set_kind(this: &DocumentSymbol, kind: SymbolKind);

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "tags", getter)]
    pub fn tags(this: &DocumentSymbol) -> ReadonlyArray;

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "tags", setter)]
    pub fn set_tags(this: &DocumentSymbol, tags: ReadonlyArray);

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "containerName", getter)]
    pub fn container_name(this: &DocumentSymbol) -> String;

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "containerName", setter)]
    pub fn set_container_name(this: &DocumentSymbol, container_name: String);

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "range", getter)]
    pub fn range(this: &DocumentSymbol) -> IRange;

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "range", setter)]
    pub fn set_range(this: &DocumentSymbol, range: IRange);

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "selectionRange", getter)]
    pub fn selection_range(this: &DocumentSymbol) -> IRange;

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "selectionRange", setter)]
    pub fn set_selection_range(this: &DocumentSymbol, selection_range: IRange);

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "children", getter)]
    pub fn children(this: &DocumentSymbol) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "DocumentSymbol", js_name = "children", setter)]
    pub fn set_children(this: &DocumentSymbol, children: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DocumentSymbolProvider;

    #[wasm_bindgen(method, js_class = "DocumentSymbolProvider", js_name = "displayName", getter)]
    pub fn display_name(this: &DocumentSymbolProvider) -> String;

    #[wasm_bindgen(method, js_class = "DocumentSymbolProvider", js_name = "displayName", setter)]
    pub fn set_display_name(this: &DocumentSymbolProvider, display_name: String);

    /**
         * Provide symbol information for the given document.
         */ 
    #[wasm_bindgen(method, js_class = "DocumentSymbolProvider", js_name = "provideDocumentSymbols")]
    pub fn provide_document_symbols(this: &DocumentSymbolProvider, model: editor::ITextModel, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type TextEdit;

    #[wasm_bindgen(method, js_class = "TextEdit", js_name = "range", getter)]
    pub fn range(this: &TextEdit) -> IRange;

    #[wasm_bindgen(method, js_class = "TextEdit", js_name = "range", setter)]
    pub fn set_range(this: &TextEdit, range: IRange);

    #[wasm_bindgen(method, js_class = "TextEdit", js_name = "text", getter)]
    pub fn text(this: &TextEdit) -> String;

    #[wasm_bindgen(method, js_class = "TextEdit", js_name = "text", setter)]
    pub fn set_text(this: &TextEdit, text: String);

    #[wasm_bindgen(method, js_class = "TextEdit", js_name = "eol", getter)]
    pub fn eol(this: &TextEdit) -> editor::EndOfLineSequence;

    #[wasm_bindgen(method, js_class = "TextEdit", js_name = "eol", setter)]
    pub fn set_eol(this: &TextEdit, eol: editor::EndOfLineSequence);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type FormattingOptions;

    /**
         * Size of a tab in spaces.
         */ 
    #[wasm_bindgen(method, js_class = "FormattingOptions", js_name = "tabSize", getter)]
    pub fn tab_size(this: &FormattingOptions) -> f64;

    #[wasm_bindgen(method, js_class = "FormattingOptions", js_name = "tabSize", setter)]
    pub fn set_tab_size(this: &FormattingOptions, tab_size: f64);

    /**
         * Prefer spaces over tabs.
         */ 
    #[wasm_bindgen(method, js_class = "FormattingOptions", js_name = "insertSpaces", getter)]
    pub fn insert_spaces(this: &FormattingOptions) -> bool;

    #[wasm_bindgen(method, js_class = "FormattingOptions", js_name = "insertSpaces", setter)]
    pub fn set_insert_spaces(this: &FormattingOptions, insert_spaces: bool);

    /**
         * The list of multiple ranges to format at once, if the provider supports it.
         */ 
    #[wasm_bindgen(method, js_class = "FormattingOptions", js_name = "ranges", getter)]
    pub fn ranges(this: &FormattingOptions) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "FormattingOptions", js_name = "ranges", setter)]
    pub fn set_ranges(this: &FormattingOptions, ranges: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DocumentFormattingEditProvider;

    #[wasm_bindgen(method, js_class = "DocumentFormattingEditProvider", js_name = "displayName", getter)]
    pub fn display_name(this: &DocumentFormattingEditProvider) -> String;
    /**
         * Provide formatting edits for a whole document.
         */ 
    #[wasm_bindgen(method, js_class = "DocumentFormattingEditProvider", js_name = "provideDocumentFormattingEdits")]
    pub fn provide_document_formatting_edits(this: &DocumentFormattingEditProvider, model: editor::ITextModel, options: FormattingOptions, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DocumentRangeFormattingEditProvider;

    #[wasm_bindgen(method, js_class = "DocumentRangeFormattingEditProvider", js_name = "displayName", getter)]
    pub fn display_name(this: &DocumentRangeFormattingEditProvider) -> String;
    /**
         * Provide formatting edits for a range in a document.
         *
         * The given range is a hint and providers can decide to format a smaller
         * or larger range. Often this is done by adjusting the start and end
         * of the range to full syntax nodes.
         */ 
    #[wasm_bindgen(method, js_class = "DocumentRangeFormattingEditProvider", js_name = "provideDocumentRangeFormattingEdits")]
    pub fn provide_document_range_formatting_edits(this: &DocumentRangeFormattingEditProvider, model: editor::ITextModel, range: Range, options: FormattingOptions, token: CancellationToken, ) -> ProviderResult;

    #[wasm_bindgen(method, js_class = "DocumentRangeFormattingEditProvider", js_name = "provideDocumentRangesFormattingEdits")]
    pub fn provide_document_ranges_formatting_edits(this: &DocumentRangeFormattingEditProvider, model: editor::ITextModel, ranges: js_sys::Array, options: FormattingOptions, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type OnTypeFormattingEditProvider;

    #[wasm_bindgen(method, js_class = "OnTypeFormattingEditProvider", js_name = "autoFormatTriggerCharacters", getter)]
    pub fn auto_format_trigger_characters(this: &OnTypeFormattingEditProvider) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "OnTypeFormattingEditProvider", js_name = "autoFormatTriggerCharacters", setter)]
    pub fn set_auto_format_trigger_characters(this: &OnTypeFormattingEditProvider, auto_format_trigger_characters: js_sys::Array);

    /**
         * Provide formatting edits after a character has been typed.
         *
         * The given position and character should hint to the provider
         * what range the position to expand to, like find the matching `{`
         * when `}` has been entered.
         */ 
    #[wasm_bindgen(method, js_class = "OnTypeFormattingEditProvider", js_name = "provideOnTypeFormattingEdits")]
    pub fn provide_on_type_formatting_edits(this: &OnTypeFormattingEditProvider, model: editor::ITextModel, position: Position, ch: String, options: FormattingOptions, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ILink;

    #[wasm_bindgen(method, js_class = "ILink", js_name = "range", getter)]
    pub fn range(this: &ILink) -> IRange;

    #[wasm_bindgen(method, js_class = "ILink", js_name = "range", setter)]
    pub fn set_range(this: &ILink, range: IRange);

    #[wasm_bindgen(method, js_class = "ILink", js_name = "url", getter)]
    pub fn url(this: &ILink) -> JsValue;

    #[wasm_bindgen(method, js_class = "ILink", js_name = "url", setter)]
    pub fn set_url(this: &ILink, url: JsValue);

    #[wasm_bindgen(method, js_class = "ILink", js_name = "tooltip", getter)]
    pub fn tooltip(this: &ILink) -> String;

    #[wasm_bindgen(method, js_class = "ILink", js_name = "tooltip", setter)]
    pub fn set_tooltip(this: &ILink, tooltip: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ILinksList;

    #[wasm_bindgen(method, js_class = "ILinksList", js_name = "links", getter)]
    pub fn links(this: &ILinksList) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "ILinksList", js_name = "links", setter)]
    pub fn set_links(this: &ILinksList, links: js_sys::Array);

    #[wasm_bindgen(method, js_class = "ILinksList", js_name = "dispose")]
    pub fn dispose(this: &ILinksList, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LinkProvider;

    #[wasm_bindgen(method, js_class = "LinkProvider", js_name = "provideLinks")]
    pub fn provide_links(this: &LinkProvider, model: editor::ITextModel, token: CancellationToken, ) -> ProviderResult;

    #[wasm_bindgen(method, js_class = "LinkProvider", js_name = "resolveLink", getter)]
    pub fn resolve_link(this: &LinkProvider) -> JsValue;

    #[wasm_bindgen(method, js_class = "LinkProvider", js_name = "resolveLink", setter)]
    pub fn set_resolve_link(this: &LinkProvider, resolve_link: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IColor;

    /**
         * The red component in the range [0-1].
         */ 
    #[wasm_bindgen(method, js_class = "IColor", js_name = "red", getter)]
    pub fn red(this: &IColor) -> f64;
    /**
         * The green component in the range [0-1].
         */ 
    #[wasm_bindgen(method, js_class = "IColor", js_name = "green", getter)]
    pub fn green(this: &IColor) -> f64;
    /**
         * The blue component in the range [0-1].
         */ 
    #[wasm_bindgen(method, js_class = "IColor", js_name = "blue", getter)]
    pub fn blue(this: &IColor) -> f64;
    /**
         * The alpha component in the range [0-1].
         */ 
    #[wasm_bindgen(method, js_class = "IColor", js_name = "alpha", getter)]
    pub fn alpha(this: &IColor) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IColorPresentation;

    /**
         * The label of this color presentation. It will be shown on the color
         * picker header. By default this is also the text that is inserted when selecting
         * this color presentation.
         */ 
    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "label", getter)]
    pub fn label(this: &IColorPresentation) -> String;

    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "label", setter)]
    pub fn set_label(this: &IColorPresentation, label: String);

    /**
         * An {@link TextEdit edit} which is applied to a document when selecting
         * this presentation for the color.
         */ 
    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "textEdit", getter)]
    pub fn text_edit(this: &IColorPresentation) -> TextEdit;

    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "textEdit", setter)]
    pub fn set_text_edit(this: &IColorPresentation, text_edit: TextEdit);

    /**
         * An optional array of additional {@link TextEdit text edits} that are applied when
         * selecting this color presentation.
         */ 
    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "additionalTextEdits", getter)]
    pub fn additional_text_edits(this: &IColorPresentation) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IColorPresentation", js_name = "additionalTextEdits", setter)]
    pub fn set_additional_text_edits(this: &IColorPresentation, additional_text_edits: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IColorInformation;

    /**
         * The range within the model.
         */ 
    #[wasm_bindgen(method, js_class = "IColorInformation", js_name = "range", getter)]
    pub fn range(this: &IColorInformation) -> IRange;

    #[wasm_bindgen(method, js_class = "IColorInformation", js_name = "range", setter)]
    pub fn set_range(this: &IColorInformation, range: IRange);

    /**
         * The color represented in this range.
         */ 
    #[wasm_bindgen(method, js_class = "IColorInformation", js_name = "color", getter)]
    pub fn color(this: &IColorInformation) -> IColor;

    #[wasm_bindgen(method, js_class = "IColorInformation", js_name = "color", setter)]
    pub fn set_color(this: &IColorInformation, color: IColor);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DocumentColorProvider;

    /**
         * Provides the color ranges for a specific model.
         */ 
    #[wasm_bindgen(method, js_class = "DocumentColorProvider", js_name = "provideDocumentColors")]
    pub fn provide_document_colors(this: &DocumentColorProvider, model: editor::ITextModel, token: CancellationToken, ) -> ProviderResult;

    /**
         * Provide the string representations for a color.
         */ 
    #[wasm_bindgen(method, js_class = "DocumentColorProvider", js_name = "provideColorPresentations")]
    pub fn provide_color_presentations(this: &DocumentColorProvider, model: editor::ITextModel, color_info: IColorInformation, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SelectionRange;

    #[wasm_bindgen(method, js_class = "SelectionRange", js_name = "range", getter)]
    pub fn range(this: &SelectionRange) -> IRange;

    #[wasm_bindgen(method, js_class = "SelectionRange", js_name = "range", setter)]
    pub fn set_range(this: &SelectionRange, range: IRange);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SelectionRangeProvider;

    /**
         * Provide ranges that should be selected from the given position.
         */ 
    #[wasm_bindgen(method, js_class = "SelectionRangeProvider", js_name = "provideSelectionRanges")]
    pub fn provide_selection_ranges(this: &SelectionRangeProvider, model: editor::ITextModel, positions: js_sys::Array, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type FoldingContext;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type FoldingRangeProvider;

    /**
         * An optional event to signal that the folding ranges from this provider have changed.
         */ 
    #[wasm_bindgen(method, js_class = "FoldingRangeProvider", js_name = "onDidChange", getter)]
    pub fn on_did_change(this: &FoldingRangeProvider) -> IEvent;

    #[wasm_bindgen(method, js_class = "FoldingRangeProvider", js_name = "onDidChange", setter)]
    pub fn set_on_did_change(this: &FoldingRangeProvider, on_did_change: IEvent);

    /**
         * Provides the folding ranges for a specific model.
         */ 
    #[wasm_bindgen(method, js_class = "FoldingRangeProvider", js_name = "provideFoldingRanges")]
    pub fn provide_folding_ranges(this: &FoldingRangeProvider, model: editor::ITextModel, context: FoldingContext, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type FoldingRange;

    /**
         * The one-based start line of the range to fold. The folded area starts after the line's last character.
         */ 
    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "start", getter)]
    pub fn start(this: &FoldingRange) -> f64;

    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "start", setter)]
    pub fn set_start(this: &FoldingRange, start: f64);

    /**
         * The one-based end line of the range to fold. The folded area ends with the line's last character.
         */ 
    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "end", getter)]
    pub fn end(this: &FoldingRange) -> f64;

    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "end", setter)]
    pub fn set_end(this: &FoldingRange, end: f64);

    /**
         * Describes the {@link FoldingRangeKind Kind} of the folding range such as {@link FoldingRangeKind.Comment Comment} or
         * {@link FoldingRangeKind.Region Region}. The kind is used to categorize folding ranges and used by commands
         * like 'Fold all comments'. See
         * {@link FoldingRangeKind} for an enumeration of standardized kinds.
         */ 
    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "kind", getter)]
    pub fn kind(this: &FoldingRange) -> FoldingRangeKind;

    #[wasm_bindgen(method, js_class = "FoldingRange", js_name = "kind", setter)]
    pub fn set_kind(this: &FoldingRange, kind: FoldingRangeKind);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type FoldingRangeKind;

    /**
         * Returns a {@link FoldingRangeKind} for the given value.
         *
         * @param value of the kind.
         */ 

    #[wasm_bindgen(js_class = "{type}", js_name = "{name}", static_method_of = FoldingRangeKind)]
    pub fn from_value(value: String, ) -> FoldingRangeKind;

    /**
         * Creates a new {@link FoldingRangeKind}.
         *
         * @param value of the kind.
         */ 
  #[wasm_bindgen(constructor, js_class = "FoldingRangeKind")]
  pub fn new() -> FoldingRangeKind;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type WorkspaceEditMetadata;

    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "needsConfirmation", getter)]
    pub fn needs_confirmation(this: &WorkspaceEditMetadata) -> bool;

    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "needsConfirmation", setter)]
    pub fn set_needs_confirmation(this: &WorkspaceEditMetadata, needs_confirmation: bool);

    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "label", getter)]
    pub fn label(this: &WorkspaceEditMetadata) -> String;

    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "label", setter)]
    pub fn set_label(this: &WorkspaceEditMetadata, label: String);

    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "description", getter)]
    pub fn description(this: &WorkspaceEditMetadata) -> String;

    #[wasm_bindgen(method, js_class = "WorkspaceEditMetadata", js_name = "description", setter)]
    pub fn set_description(this: &WorkspaceEditMetadata, description: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type WorkspaceFileEditOptions;

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "overwrite", getter)]
    pub fn overwrite(this: &WorkspaceFileEditOptions) -> bool;

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "overwrite", setter)]
    pub fn set_overwrite(this: &WorkspaceFileEditOptions, overwrite: bool);

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "ignoreIfNotExists", getter)]
    pub fn ignore_if_not_exists(this: &WorkspaceFileEditOptions) -> bool;

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "ignoreIfNotExists", setter)]
    pub fn set_ignore_if_not_exists(this: &WorkspaceFileEditOptions, ignore_if_not_exists: bool);

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "ignoreIfExists", getter)]
    pub fn ignore_if_exists(this: &WorkspaceFileEditOptions) -> bool;

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "ignoreIfExists", setter)]
    pub fn set_ignore_if_exists(this: &WorkspaceFileEditOptions, ignore_if_exists: bool);

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "recursive", getter)]
    pub fn recursive(this: &WorkspaceFileEditOptions) -> bool;

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "recursive", setter)]
    pub fn set_recursive(this: &WorkspaceFileEditOptions, recursive: bool);

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "copy", getter)]
    pub fn copy(this: &WorkspaceFileEditOptions) -> bool;

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "copy", setter)]
    pub fn set_copy(this: &WorkspaceFileEditOptions, copy: bool);

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "folder", getter)]
    pub fn folder(this: &WorkspaceFileEditOptions) -> bool;

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "folder", setter)]
    pub fn set_folder(this: &WorkspaceFileEditOptions, folder: bool);

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "skipTrashBin", getter)]
    pub fn skip_trash_bin(this: &WorkspaceFileEditOptions) -> bool;

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "skipTrashBin", setter)]
    pub fn set_skip_trash_bin(this: &WorkspaceFileEditOptions, skip_trash_bin: bool);

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "maxSize", getter)]
    pub fn max_size(this: &WorkspaceFileEditOptions) -> f64;

    #[wasm_bindgen(method, js_class = "WorkspaceFileEditOptions", js_name = "maxSize", setter)]
    pub fn set_max_size(this: &WorkspaceFileEditOptions, max_size: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IWorkspaceFileEdit;

    #[wasm_bindgen(method, js_class = "IWorkspaceFileEdit", js_name = "oldResource", getter)]
    pub fn old_resource(this: &IWorkspaceFileEdit) -> Uri;

    #[wasm_bindgen(method, js_class = "IWorkspaceFileEdit", js_name = "oldResource", setter)]
    pub fn set_old_resource(this: &IWorkspaceFileEdit, old_resource: Uri);

    #[wasm_bindgen(method, js_class = "IWorkspaceFileEdit", js_name = "newResource", getter)]
    pub fn new_resource(this: &IWorkspaceFileEdit) -> Uri;

    #[wasm_bindgen(method, js_class = "IWorkspaceFileEdit", js_name = "newResource", setter)]
    pub fn set_new_resource(this: &IWorkspaceFileEdit, new_resource: Uri);

    #[wasm_bindgen(method, js_class = "IWorkspaceFileEdit", js_name = "options", getter)]
    pub fn options(this: &IWorkspaceFileEdit) -> WorkspaceFileEditOptions;

    #[wasm_bindgen(method, js_class = "IWorkspaceFileEdit", js_name = "options", setter)]
    pub fn set_options(this: &IWorkspaceFileEdit, options: WorkspaceFileEditOptions);

    #[wasm_bindgen(method, js_class = "IWorkspaceFileEdit", js_name = "metadata", getter)]
    pub fn metadata(this: &IWorkspaceFileEdit) -> WorkspaceEditMetadata;

    #[wasm_bindgen(method, js_class = "IWorkspaceFileEdit", js_name = "metadata", setter)]
    pub fn set_metadata(this: &IWorkspaceFileEdit, metadata: WorkspaceEditMetadata);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IWorkspaceTextEdit;

    #[wasm_bindgen(method, js_class = "IWorkspaceTextEdit", js_name = "resource", getter)]
    pub fn resource(this: &IWorkspaceTextEdit) -> Uri;

    #[wasm_bindgen(method, js_class = "IWorkspaceTextEdit", js_name = "resource", setter)]
    pub fn set_resource(this: &IWorkspaceTextEdit, resource: Uri);

    #[wasm_bindgen(method, js_class = "IWorkspaceTextEdit", js_name = "textEdit", getter)]
    pub fn text_edit(this: &IWorkspaceTextEdit) -> JsValue;

    #[wasm_bindgen(method, js_class = "IWorkspaceTextEdit", js_name = "textEdit", setter)]
    pub fn set_text_edit(this: &IWorkspaceTextEdit, text_edit: JsValue);

    #[wasm_bindgen(method, js_class = "IWorkspaceTextEdit", js_name = "versionId", getter)]
    pub fn version_id(this: &IWorkspaceTextEdit) -> JsValue;

    #[wasm_bindgen(method, js_class = "IWorkspaceTextEdit", js_name = "versionId", setter)]
    pub fn set_version_id(this: &IWorkspaceTextEdit, version_id: JsValue);

    #[wasm_bindgen(method, js_class = "IWorkspaceTextEdit", js_name = "metadata", getter)]
    pub fn metadata(this: &IWorkspaceTextEdit) -> WorkspaceEditMetadata;

    #[wasm_bindgen(method, js_class = "IWorkspaceTextEdit", js_name = "metadata", setter)]
    pub fn set_metadata(this: &IWorkspaceTextEdit, metadata: WorkspaceEditMetadata);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type WorkspaceEdit;

    #[wasm_bindgen(method, js_class = "WorkspaceEdit", js_name = "edits", getter)]
    pub fn edits(this: &WorkspaceEdit) -> Array;

    #[wasm_bindgen(method, js_class = "WorkspaceEdit", js_name = "edits", setter)]
    pub fn set_edits(this: &WorkspaceEdit, edits: Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Rejection;

    #[wasm_bindgen(method, js_class = "Rejection", js_name = "rejectReason", getter)]
    pub fn reject_reason(this: &Rejection) -> String;

    #[wasm_bindgen(method, js_class = "Rejection", js_name = "rejectReason", setter)]
    pub fn set_reject_reason(this: &Rejection, reject_reason: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type RenameLocation;

    #[wasm_bindgen(method, js_class = "RenameLocation", js_name = "range", getter)]
    pub fn range(this: &RenameLocation) -> IRange;

    #[wasm_bindgen(method, js_class = "RenameLocation", js_name = "range", setter)]
    pub fn set_range(this: &RenameLocation, range: IRange);

    #[wasm_bindgen(method, js_class = "RenameLocation", js_name = "text", getter)]
    pub fn text(this: &RenameLocation) -> String;

    #[wasm_bindgen(method, js_class = "RenameLocation", js_name = "text", setter)]
    pub fn set_text(this: &RenameLocation, text: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type RenameProvider;

    #[wasm_bindgen(method, js_class = "RenameProvider", js_name = "provideRenameEdits")]
    pub fn provide_rename_edits(this: &RenameProvider, model: editor::ITextModel, position: Position, new_name: String, token: CancellationToken, ) -> ProviderResult;

    #[wasm_bindgen(method, js_class = "RenameProvider", js_name = "resolveRenameLocation")]
    pub fn resolve_rename_location(this: &RenameProvider, model: editor::ITextModel, position: Position, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Command;

    #[wasm_bindgen(method, js_class = "Command", js_name = "id", getter)]
    pub fn id(this: &Command) -> String;

    #[wasm_bindgen(method, js_class = "Command", js_name = "id", setter)]
    pub fn set_id(this: &Command, id: String);

    #[wasm_bindgen(method, js_class = "Command", js_name = "title", getter)]
    pub fn title(this: &Command) -> String;

    #[wasm_bindgen(method, js_class = "Command", js_name = "title", setter)]
    pub fn set_title(this: &Command, title: String);

    #[wasm_bindgen(method, js_class = "Command", js_name = "tooltip", getter)]
    pub fn tooltip(this: &Command) -> String;

    #[wasm_bindgen(method, js_class = "Command", js_name = "tooltip", setter)]
    pub fn set_tooltip(this: &Command, tooltip: String);

    #[wasm_bindgen(method, js_class = "Command", js_name = "arguments", getter)]
    pub fn arguments(this: &Command) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "Command", js_name = "arguments", setter)]
    pub fn set_arguments(this: &Command, arguments: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CodeLens;

    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "range", getter)]
    pub fn range(this: &CodeLens) -> IRange;

    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "range", setter)]
    pub fn set_range(this: &CodeLens, range: IRange);

    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "id", getter)]
    pub fn id(this: &CodeLens) -> String;

    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "id", setter)]
    pub fn set_id(this: &CodeLens, id: String);

    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "command", getter)]
    pub fn command(this: &CodeLens) -> Command;

    #[wasm_bindgen(method, js_class = "CodeLens", js_name = "command", setter)]
    pub fn set_command(this: &CodeLens, command: Command);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CodeLensList;

    #[wasm_bindgen(method, js_class = "CodeLensList", js_name = "lenses", getter)]
    pub fn lenses(this: &CodeLensList) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CodeLensList", js_name = "lenses", setter)]
    pub fn set_lenses(this: &CodeLensList, lenses: js_sys::Array);

    #[wasm_bindgen(method, js_class = "CodeLensList", js_name = "dispose")]
    pub fn dispose(this: &CodeLensList, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CodeLensProvider;

    #[wasm_bindgen(method, js_class = "CodeLensProvider", js_name = "onDidChange", getter)]
    pub fn on_did_change(this: &CodeLensProvider) -> IEvent;

    #[wasm_bindgen(method, js_class = "CodeLensProvider", js_name = "onDidChange", setter)]
    pub fn set_on_did_change(this: &CodeLensProvider, on_did_change: IEvent);

    #[wasm_bindgen(method, js_class = "CodeLensProvider", js_name = "provideCodeLenses")]
    pub fn provide_code_lenses(this: &CodeLensProvider, model: editor::ITextModel, token: CancellationToken, ) -> ProviderResult;

    #[wasm_bindgen(method, js_class = "CodeLensProvider", js_name = "resolveCodeLens")]
    pub fn resolve_code_lens(this: &CodeLensProvider, model: editor::ITextModel, code_lens: CodeLens, token: CancellationToken, ) -> ProviderResult;


}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum InlayHintKind {
    Type = 1,
    Parameter = 2,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InlayHintLabelPart;

    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "label", getter)]
    pub fn label(this: &InlayHintLabelPart) -> String;

    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "label", setter)]
    pub fn set_label(this: &InlayHintLabelPart, label: String);

    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "tooltip", getter)]
    pub fn tooltip(this: &InlayHintLabelPart) -> JsValue;

    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "tooltip", setter)]
    pub fn set_tooltip(this: &InlayHintLabelPart, tooltip: JsValue);

    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "command", getter)]
    pub fn command(this: &InlayHintLabelPart) -> Command;

    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "command", setter)]
    pub fn set_command(this: &InlayHintLabelPart, command: Command);

    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "location", getter)]
    pub fn location(this: &InlayHintLabelPart) -> Location;

    #[wasm_bindgen(method, js_class = "InlayHintLabelPart", js_name = "location", setter)]
    pub fn set_location(this: &InlayHintLabelPart, location: Location);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InlayHint;

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "label", getter)]
    pub fn label(this: &InlayHint) -> JsValue;

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "label", setter)]
    pub fn set_label(this: &InlayHint, label: JsValue);

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "tooltip", getter)]
    pub fn tooltip(this: &InlayHint) -> JsValue;

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "tooltip", setter)]
    pub fn set_tooltip(this: &InlayHint, tooltip: JsValue);

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "textEdits", getter)]
    pub fn text_edits(this: &InlayHint) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "textEdits", setter)]
    pub fn set_text_edits(this: &InlayHint, text_edits: js_sys::Array);

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "position", getter)]
    pub fn position(this: &InlayHint) -> IPosition;

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "position", setter)]
    pub fn set_position(this: &InlayHint, position: IPosition);

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "kind", getter)]
    pub fn kind(this: &InlayHint) -> InlayHintKind;

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "kind", setter)]
    pub fn set_kind(this: &InlayHint, kind: InlayHintKind);

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "paddingLeft", getter)]
    pub fn padding_left(this: &InlayHint) -> bool;

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "paddingLeft", setter)]
    pub fn set_padding_left(this: &InlayHint, padding_left: bool);

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "paddingRight", getter)]
    pub fn padding_right(this: &InlayHint) -> bool;

    #[wasm_bindgen(method, js_class = "InlayHint", js_name = "paddingRight", setter)]
    pub fn set_padding_right(this: &InlayHint, padding_right: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InlayHintList;

    #[wasm_bindgen(method, js_class = "InlayHintList", js_name = "hints", getter)]
    pub fn hints(this: &InlayHintList) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "InlayHintList", js_name = "hints", setter)]
    pub fn set_hints(this: &InlayHintList, hints: js_sys::Array);

    #[wasm_bindgen(method, js_class = "InlayHintList", js_name = "dispose")]
    pub fn dispose(this: &InlayHintList, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InlayHintsProvider;

    #[wasm_bindgen(method, js_class = "InlayHintsProvider", js_name = "displayName", getter)]
    pub fn display_name(this: &InlayHintsProvider) -> String;

    #[wasm_bindgen(method, js_class = "InlayHintsProvider", js_name = "displayName", setter)]
    pub fn set_display_name(this: &InlayHintsProvider, display_name: String);

    #[wasm_bindgen(method, js_class = "InlayHintsProvider", js_name = "onDidChangeInlayHints", getter)]
    pub fn on_did_change_inlay_hints(this: &InlayHintsProvider) -> IEvent;

    #[wasm_bindgen(method, js_class = "InlayHintsProvider", js_name = "onDidChangeInlayHints", setter)]
    pub fn set_on_did_change_inlay_hints(this: &InlayHintsProvider, on_did_change_inlay_hints: IEvent);

    #[wasm_bindgen(method, js_class = "InlayHintsProvider", js_name = "provideInlayHints")]
    pub fn provide_inlay_hints(this: &InlayHintsProvider, model: editor::ITextModel, range: Range, token: CancellationToken, ) -> ProviderResult;

    #[wasm_bindgen(method, js_class = "InlayHintsProvider", js_name = "resolveInlayHint")]
    pub fn resolve_inlay_hint(this: &InlayHintsProvider, hint: InlayHint, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SemanticTokensLegend;

    #[wasm_bindgen(method, js_class = "SemanticTokensLegend", js_name = "tokenTypes", getter)]
    pub fn token_types(this: &SemanticTokensLegend) -> js_sys::Array;
    #[wasm_bindgen(method, js_class = "SemanticTokensLegend", js_name = "tokenModifiers", getter)]
    pub fn token_modifiers(this: &SemanticTokensLegend) -> js_sys::Array;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SemanticTokens;

    #[wasm_bindgen(method, js_class = "SemanticTokens", js_name = "resultId", getter)]
    pub fn result_id(this: &SemanticTokens) -> String;
    #[wasm_bindgen(method, js_class = "SemanticTokens", js_name = "data", getter)]
    pub fn data(this: &SemanticTokens) -> Uint32Array;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SemanticTokensEdit;

    #[wasm_bindgen(method, js_class = "SemanticTokensEdit", js_name = "start", getter)]
    pub fn start(this: &SemanticTokensEdit) -> f64;
    #[wasm_bindgen(method, js_class = "SemanticTokensEdit", js_name = "deleteCount", getter)]
    pub fn delete_count(this: &SemanticTokensEdit) -> f64;
    #[wasm_bindgen(method, js_class = "SemanticTokensEdit", js_name = "data", getter)]
    pub fn data(this: &SemanticTokensEdit) -> Uint32Array;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type SemanticTokensEdits;

    #[wasm_bindgen(method, js_class = "SemanticTokensEdits", js_name = "resultId", getter)]
    pub fn result_id(this: &SemanticTokensEdits) -> String;
    #[wasm_bindgen(method, js_class = "SemanticTokensEdits", js_name = "edits", getter)]
    pub fn edits(this: &SemanticTokensEdits) -> js_sys::Array;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DocumentSemanticTokensProvider;

    #[wasm_bindgen(method, js_class = "DocumentSemanticTokensProvider", js_name = "onDidChange", getter)]
    pub fn on_did_change(this: &DocumentSemanticTokensProvider) -> IEvent;

    #[wasm_bindgen(method, js_class = "DocumentSemanticTokensProvider", js_name = "onDidChange", setter)]
    pub fn set_on_did_change(this: &DocumentSemanticTokensProvider, on_did_change: IEvent);

    #[wasm_bindgen(method, js_class = "DocumentSemanticTokensProvider", js_name = "getLegend")]
    pub fn get_legend(this: &DocumentSemanticTokensProvider, ) -> SemanticTokensLegend;

    #[wasm_bindgen(method, js_class = "DocumentSemanticTokensProvider", js_name = "provideDocumentSemanticTokens")]
    pub fn provide_document_semantic_tokens(this: &DocumentSemanticTokensProvider, model: editor::ITextModel, last_result_id: JsValue, token: CancellationToken, ) -> ProviderResult;

    #[wasm_bindgen(method, js_class = "DocumentSemanticTokensProvider", js_name = "releaseDocumentSemanticTokens")]
    pub fn release_document_semantic_tokens(this: &DocumentSemanticTokensProvider, result_id: JsValue, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DocumentRangeSemanticTokensProvider;

    #[wasm_bindgen(method, js_class = "DocumentRangeSemanticTokensProvider", js_name = "getLegend")]
    pub fn get_legend(this: &DocumentRangeSemanticTokensProvider, ) -> SemanticTokensLegend;

    #[wasm_bindgen(method, js_class = "DocumentRangeSemanticTokensProvider", js_name = "provideDocumentRangeSemanticTokens")]
    pub fn provide_document_range_semantic_tokens(this: &DocumentRangeSemanticTokensProvider, model: editor::ITextModel, range: Range, token: CancellationToken, ) -> ProviderResult;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ILanguageExtensionPoint;

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "id", getter)]
    pub fn id(this: &ILanguageExtensionPoint) -> String;

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "id", setter)]
    pub fn set_id(this: &ILanguageExtensionPoint, id: String);

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "extensions", getter)]
    pub fn extensions(this: &ILanguageExtensionPoint) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "extensions", setter)]
    pub fn set_extensions(this: &ILanguageExtensionPoint, extensions: js_sys::Array);

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "filenames", getter)]
    pub fn filenames(this: &ILanguageExtensionPoint) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "filenames", setter)]
    pub fn set_filenames(this: &ILanguageExtensionPoint, filenames: js_sys::Array);

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "filenamePatterns", getter)]
    pub fn filename_patterns(this: &ILanguageExtensionPoint) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "filenamePatterns", setter)]
    pub fn set_filename_patterns(this: &ILanguageExtensionPoint, filename_patterns: js_sys::Array);

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "firstLine", getter)]
    pub fn first_line(this: &ILanguageExtensionPoint) -> String;

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "firstLine", setter)]
    pub fn set_first_line(this: &ILanguageExtensionPoint, first_line: String);

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "aliases", getter)]
    pub fn aliases(this: &ILanguageExtensionPoint) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "aliases", setter)]
    pub fn set_aliases(this: &ILanguageExtensionPoint, aliases: js_sys::Array);

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "mimetypes", getter)]
    pub fn mimetypes(this: &ILanguageExtensionPoint) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "mimetypes", setter)]
    pub fn set_mimetypes(this: &ILanguageExtensionPoint, mimetypes: js_sys::Array);

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "configuration", getter)]
    pub fn configuration(this: &ILanguageExtensionPoint) -> Uri;

    #[wasm_bindgen(method, js_class = "ILanguageExtensionPoint", js_name = "configuration", setter)]
    pub fn set_configuration(this: &ILanguageExtensionPoint, configuration: Uri);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMonarchLanguage;

    /**
         * map from string to ILanguageRule[]
         */ 
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "tokenizer", getter)]
    pub fn tokenizer(this: &IMonarchLanguage) -> js_sys::Object;

    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "tokenizer", setter)]
    pub fn set_tokenizer(this: &IMonarchLanguage, tokenizer: js_sys::Object);

    /**
         * is the language case insensitive?
         */ 
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "ignoreCase", getter)]
    pub fn ignore_case(this: &IMonarchLanguage) -> bool;

    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "ignoreCase", setter)]
    pub fn set_ignore_case(this: &IMonarchLanguage, ignore_case: bool);

    /**
         * is the language unicode-aware? (i.e., /\u{1D306}/)
         */ 
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "unicode", getter)]
    pub fn unicode(this: &IMonarchLanguage) -> bool;

    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "unicode", setter)]
    pub fn set_unicode(this: &IMonarchLanguage, unicode: bool);

    /**
         * if no match in the tokenizer assign this token class (default 'source')
         */ 
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "defaultToken", getter)]
    pub fn default_token(this: &IMonarchLanguage) -> String;

    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "defaultToken", setter)]
    pub fn set_default_token(this: &IMonarchLanguage, default_token: String);

    /**
         * for example [['{','}','delimiter.curly']]
         */ 
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "brackets", getter)]
    pub fn brackets(this: &IMonarchLanguage) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "brackets", setter)]
    pub fn set_brackets(this: &IMonarchLanguage, brackets: js_sys::Array);

    /**
         * start symbol in the tokenizer (by default the first entry is used)
         */ 
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "start", getter)]
    pub fn start(this: &IMonarchLanguage) -> String;

    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "start", setter)]
    pub fn set_start(this: &IMonarchLanguage, start: String);

    /**
         * attach this to every token class (by default '.' + name)
         */ 
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "tokenPostfix", getter)]
    pub fn token_postfix(this: &IMonarchLanguage) -> String;

    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "tokenPostfix", setter)]
    pub fn set_token_postfix(this: &IMonarchLanguage, token_postfix: String);

    /**
         * include line feeds (in the form of a \n character) at the end of lines
         * Defaults to false
         */ 
    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "includeLF", getter)]
    pub fn include_lf(this: &IMonarchLanguage) -> bool;

    #[wasm_bindgen(method, js_class = "IMonarchLanguage", js_name = "includeLF", setter)]
    pub fn set_include_lf(this: &IMonarchLanguage, include_lf: bool);


}

pub type IShortMonarchLanguageRule1 = JsValue
;
pub type IShortMonarchLanguageRule2 = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IExpandedMonarchLanguageRule;

    /**
         * match tokens
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "regex", getter)]
    pub fn regex(this: &IExpandedMonarchLanguageRule) -> JsValue;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "regex", setter)]
    pub fn set_regex(this: &IExpandedMonarchLanguageRule, regex: JsValue);

    /**
         * action to take on match
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "action", getter)]
    pub fn action(this: &IExpandedMonarchLanguageRule) -> IMonarchLanguageAction;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "action", setter)]
    pub fn set_action(this: &IExpandedMonarchLanguageRule, action: IMonarchLanguageAction);

    /**
         * or an include rule. include all rules from the included state
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "include", getter)]
    pub fn include(this: &IExpandedMonarchLanguageRule) -> String;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageRule", js_name = "include", setter)]
    pub fn set_include(this: &IExpandedMonarchLanguageRule, include: String);


}

pub type IMonarchLanguageRule = JsValue
;
pub type IShortMonarchLanguageAction = String
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IExpandedMonarchLanguageAction;

    /**
         * array of actions for each parenthesized match group
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "group", getter)]
    pub fn group(this: &IExpandedMonarchLanguageAction) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "group", setter)]
    pub fn set_group(this: &IExpandedMonarchLanguageAction, group: js_sys::Array);

    /**
         * map from string to ILanguageAction
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "cases", getter)]
    pub fn cases(this: &IExpandedMonarchLanguageAction) -> Object;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "cases", setter)]
    pub fn set_cases(this: &IExpandedMonarchLanguageAction, cases: Object);

    /**
         * token class (ie. css class) (or "@brackets" or "@rematch")
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "token", getter)]
    pub fn token(this: &IExpandedMonarchLanguageAction) -> String;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "token", setter)]
    pub fn set_token(this: &IExpandedMonarchLanguageAction, token: String);

    /**
         * the next state to push, or "@push", "@pop", "@popall"
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "next", getter)]
    pub fn next(this: &IExpandedMonarchLanguageAction) -> String;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "next", setter)]
    pub fn set_next(this: &IExpandedMonarchLanguageAction, next: String);

    /**
         * switch to this state
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "switchTo", getter)]
    pub fn switch_to(this: &IExpandedMonarchLanguageAction) -> String;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "switchTo", setter)]
    pub fn set_switch_to(this: &IExpandedMonarchLanguageAction, switch_to: String);

    /**
         * go back n characters in the stream
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "goBack", getter)]
    pub fn go_back(this: &IExpandedMonarchLanguageAction) -> f64;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "goBack", setter)]
    pub fn set_go_back(this: &IExpandedMonarchLanguageAction, go_back: f64);

    /**
         * @open or @close
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "bracket", getter)]
    pub fn bracket(this: &IExpandedMonarchLanguageAction) -> String;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "bracket", setter)]
    pub fn set_bracket(this: &IExpandedMonarchLanguageAction, bracket: String);

    /**
         * switch to embedded language (using the mimetype) or get out using "@pop"
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "nextEmbedded", getter)]
    pub fn next_embedded(this: &IExpandedMonarchLanguageAction) -> String;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "nextEmbedded", setter)]
    pub fn set_next_embedded(this: &IExpandedMonarchLanguageAction, next_embedded: String);

    /**
         * log a message to the browser console window
         */ 
    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "log", getter)]
    pub fn log(this: &IExpandedMonarchLanguageAction) -> String;

    #[wasm_bindgen(method, js_class = "IExpandedMonarchLanguageAction", js_name = "log", setter)]
    pub fn set_log(this: &IExpandedMonarchLanguageAction, log: String);


}

pub type IMonarchLanguageAction = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMonarchLanguageBracket;

    /**
         * open bracket
         */ 
    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "open", getter)]
    pub fn open(this: &IMonarchLanguageBracket) -> String;

    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "open", setter)]
    pub fn set_open(this: &IMonarchLanguageBracket, open: String);

    /**
         * closing bracket
         */ 
    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "close", getter)]
    pub fn close(this: &IMonarchLanguageBracket) -> String;

    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "close", setter)]
    pub fn set_close(this: &IMonarchLanguageBracket, close: String);

    /**
         * token class
         */ 
    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "token", getter)]
    pub fn token(this: &IMonarchLanguageBracket) -> String;

    #[wasm_bindgen(method, js_class = "IMonarchLanguageBracket", js_name = "token", setter)]
    pub fn set_token(this: &IMonarchLanguageBracket, token: String);


}

pub mod css {

#[allow(unused)]
use wasm_bindgen::prelude::*;
#[allow(unused)]
use web_sys::*;
#[allow(unused)]
use js_sys::*;
#[allow(unused)]
use super::*;

#[allow(unused)]
pub type HTMLElement = HtmlElement;
#[allow(unused)]
pub type ReadonlyArray = Array;
#[allow(unused)]
pub type NonNullable = JsValue;
#[allow(unused)]
pub type Record = JsValue;
#[allow(unused)]
pub type PromiseLike = JsValue;



#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CSSFormatConfiguration;

    /** separate selectors with newline (e.g. "a,\nbr" or "a, br"): Default: true */ 
    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "newlineBetweenSelectors", getter)]
    pub fn newline_between_selectors(this: &CSSFormatConfiguration) -> bool;

    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "newlineBetweenSelectors", setter)]
    pub fn set_newline_between_selectors(this: &CSSFormatConfiguration, newline_between_selectors: bool);

    /** add a new line after every css rule: Default: true */ 
    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "newlineBetweenRules", getter)]
    pub fn newline_between_rules(this: &CSSFormatConfiguration) -> bool;

    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "newlineBetweenRules", setter)]
    pub fn set_newline_between_rules(this: &CSSFormatConfiguration, newline_between_rules: bool);

    /** ensure space around selector separators:  '>', '+', '~' (e.g. "a>b" -> "a > b"): Default: false */ 
    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "spaceAroundSelectorSeparator", getter)]
    pub fn space_around_selector_separator(this: &CSSFormatConfiguration) -> bool;

    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "spaceAroundSelectorSeparator", setter)]
    pub fn set_space_around_selector_separator(this: &CSSFormatConfiguration, space_around_selector_separator: bool);

    /** put braces on the same line as rules (`collapse`), or put braces on own line, Allman / ANSI style (`expand`). Default `collapse` */ 
    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "braceStyle", getter)]
    pub fn brace_style(this: &CSSFormatConfiguration) -> JsValue;

    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "braceStyle", setter)]
    pub fn set_brace_style(this: &CSSFormatConfiguration, brace_style: JsValue);

    /** whether existing line breaks before elements should be preserved. Default: true */ 
    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "preserveNewLines", getter)]
    pub fn preserve_new_lines(this: &CSSFormatConfiguration) -> bool;

    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "preserveNewLines", setter)]
    pub fn set_preserve_new_lines(this: &CSSFormatConfiguration, preserve_new_lines: bool);

    /** maximum number of line breaks to be preserved in one chunk. Default: unlimited */ 
    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "maxPreserveNewLines", getter)]
    pub fn max_preserve_new_lines(this: &CSSFormatConfiguration) -> f64;

    #[wasm_bindgen(method, js_class = "CSSFormatConfiguration", js_name = "maxPreserveNewLines", setter)]
    pub fn set_max_preserve_new_lines(this: &CSSFormatConfiguration, max_preserve_new_lines: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Options;

    #[wasm_bindgen(method, js_class = "Options", js_name = "validate", getter)]
    pub fn validate(this: &Options) -> bool;
    #[wasm_bindgen(method, js_class = "Options", js_name = "lint", getter)]
    pub fn lint(this: &Options) -> js_sys::Object;
    /**
         * Configures the CSS data types known by the langauge service.
         */ 
    #[wasm_bindgen(method, js_class = "Options", js_name = "data", getter)]
    pub fn data(this: &Options) -> CSSDataConfiguration;
    /**
         * Settings for the CSS formatter.
         */ 
    #[wasm_bindgen(method, js_class = "Options", js_name = "format", getter)]
    pub fn format(this: &Options) -> CSSFormatConfiguration;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ModeConfiguration;

    /**
         * Defines whether the built-in completionItemProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "completionItems", getter)]
    pub fn completion_items(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in hoverProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "hovers", getter)]
    pub fn hovers(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in documentSymbolProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentSymbols", getter)]
    pub fn document_symbols(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in definitions provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "definitions", getter)]
    pub fn definitions(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in references provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "references", getter)]
    pub fn references(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in references provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentHighlights", getter)]
    pub fn document_highlights(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in rename provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "rename", getter)]
    pub fn rename(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in color provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "colors", getter)]
    pub fn colors(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in foldingRange provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "foldingRanges", getter)]
    pub fn folding_ranges(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in diagnostic provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "diagnostics", getter)]
    pub fn diagnostics(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in selection range provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "selectionRanges", getter)]
    pub fn selection_ranges(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in document formatting edit provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentFormattingEdits", getter)]
    pub fn document_formatting_edits(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in document formatting range edit provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentRangeFormattingEdits", getter)]
    pub fn document_range_formatting_edits(this: &ModeConfiguration) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LanguageServiceDefaults;

    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "languageId", getter)]
    pub fn language_id(this: &LanguageServiceDefaults) -> String;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "onDidChange", getter)]
    pub fn on_did_change(this: &LanguageServiceDefaults) -> IEvent;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "modeConfiguration", getter)]
    pub fn mode_configuration(this: &LanguageServiceDefaults) -> ModeConfiguration;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "options", getter)]
    pub fn options(this: &LanguageServiceDefaults) -> Options;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setOptions")]
    pub fn set_options(this: &LanguageServiceDefaults, options: Options, );

    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setModeConfiguration")]
    pub fn set_mode_configuration(this: &LanguageServiceDefaults, mode_configuration: ModeConfiguration, );

    /** @deprecated Use options instead */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "diagnosticsOptions", getter)]
    pub fn diagnostics_options(this: &LanguageServiceDefaults) -> DiagnosticsOptions;
    /** @deprecated Use setOptions instead */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setDiagnosticsOptions")]
    pub fn set_diagnostics_options(this: &LanguageServiceDefaults, options: DiagnosticsOptions, );


}

pub type DiagnosticsOptions = Options
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CSSDataConfiguration;

    /**
         * Defines whether the standard CSS properties, at-directives, pseudoClasses and pseudoElements are shown.
         */ 
    #[wasm_bindgen(method, js_class = "CSSDataConfiguration", js_name = "useDefaultDataProvider", getter)]
    pub fn use_default_data_provider(this: &CSSDataConfiguration) -> bool;

    #[wasm_bindgen(method, js_class = "CSSDataConfiguration", js_name = "useDefaultDataProvider", setter)]
    pub fn set_use_default_data_provider(this: &CSSDataConfiguration, use_default_data_provider: bool);

    /**
         * Provides a set of custom data providers.
         */ 
    #[wasm_bindgen(method, js_class = "CSSDataConfiguration", js_name = "dataProviders", getter)]
    pub fn data_providers(this: &CSSDataConfiguration) -> js_sys::Object;

    #[wasm_bindgen(method, js_class = "CSSDataConfiguration", js_name = "dataProviders", setter)]
    pub fn set_data_providers(this: &CSSDataConfiguration, data_providers: js_sys::Object);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CSSDataV1;

    #[wasm_bindgen(method, js_class = "CSSDataV1", js_name = "version", getter)]
    pub fn version(this: &CSSDataV1) -> JsValue;

    #[wasm_bindgen(method, js_class = "CSSDataV1", js_name = "version", setter)]
    pub fn set_version(this: &CSSDataV1, version: JsValue);

    #[wasm_bindgen(method, js_class = "CSSDataV1", js_name = "properties", getter)]
    pub fn properties(this: &CSSDataV1) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CSSDataV1", js_name = "properties", setter)]
    pub fn set_properties(this: &CSSDataV1, properties: js_sys::Array);

    #[wasm_bindgen(method, js_class = "CSSDataV1", js_name = "atDirectives", getter)]
    pub fn at_directives(this: &CSSDataV1) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CSSDataV1", js_name = "atDirectives", setter)]
    pub fn set_at_directives(this: &CSSDataV1, at_directives: js_sys::Array);

    #[wasm_bindgen(method, js_class = "CSSDataV1", js_name = "pseudoClasses", getter)]
    pub fn pseudo_classes(this: &CSSDataV1) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CSSDataV1", js_name = "pseudoClasses", setter)]
    pub fn set_pseudo_classes(this: &CSSDataV1, pseudo_classes: js_sys::Array);

    #[wasm_bindgen(method, js_class = "CSSDataV1", js_name = "pseudoElements", getter)]
    pub fn pseudo_elements(this: &CSSDataV1) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CSSDataV1", js_name = "pseudoElements", setter)]
    pub fn set_pseudo_elements(this: &CSSDataV1, pseudo_elements: js_sys::Array);


}

pub type EntryStatus = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IReference;

    #[wasm_bindgen(method, js_class = "IReference", js_name = "name", getter)]
    pub fn name(this: &IReference) -> String;

    #[wasm_bindgen(method, js_class = "IReference", js_name = "name", setter)]
    pub fn set_name(this: &IReference, name: String);

    #[wasm_bindgen(method, js_class = "IReference", js_name = "url", getter)]
    pub fn url(this: &IReference) -> String;

    #[wasm_bindgen(method, js_class = "IReference", js_name = "url", setter)]
    pub fn set_url(this: &IReference, url: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IPropertyData;

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "name", getter)]
    pub fn name(this: &IPropertyData) -> String;

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "name", setter)]
    pub fn set_name(this: &IPropertyData, name: String);

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "description", getter)]
    pub fn description(this: &IPropertyData) -> JsValue;

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "description", setter)]
    pub fn set_description(this: &IPropertyData, description: JsValue);

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "browsers", getter)]
    pub fn browsers(this: &IPropertyData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "browsers", setter)]
    pub fn set_browsers(this: &IPropertyData, browsers: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "restrictions", getter)]
    pub fn restrictions(this: &IPropertyData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "restrictions", setter)]
    pub fn set_restrictions(this: &IPropertyData, restrictions: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "status", getter)]
    pub fn status(this: &IPropertyData) -> EntryStatus;

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "status", setter)]
    pub fn set_status(this: &IPropertyData, status: EntryStatus);

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "syntax", getter)]
    pub fn syntax(this: &IPropertyData) -> String;

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "syntax", setter)]
    pub fn set_syntax(this: &IPropertyData, syntax: String);

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "values", getter)]
    pub fn values(this: &IPropertyData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "values", setter)]
    pub fn set_values(this: &IPropertyData, values: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "references", getter)]
    pub fn references(this: &IPropertyData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "references", setter)]
    pub fn set_references(this: &IPropertyData, references: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "relevance", getter)]
    pub fn relevance(this: &IPropertyData) -> f64;

    #[wasm_bindgen(method, js_class = "IPropertyData", js_name = "relevance", setter)]
    pub fn set_relevance(this: &IPropertyData, relevance: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IAtDirectiveData;

    #[wasm_bindgen(method, js_class = "IAtDirectiveData", js_name = "name", getter)]
    pub fn name(this: &IAtDirectiveData) -> String;

    #[wasm_bindgen(method, js_class = "IAtDirectiveData", js_name = "name", setter)]
    pub fn set_name(this: &IAtDirectiveData, name: String);

    #[wasm_bindgen(method, js_class = "IAtDirectiveData", js_name = "description", getter)]
    pub fn description(this: &IAtDirectiveData) -> JsValue;

    #[wasm_bindgen(method, js_class = "IAtDirectiveData", js_name = "description", setter)]
    pub fn set_description(this: &IAtDirectiveData, description: JsValue);

    #[wasm_bindgen(method, js_class = "IAtDirectiveData", js_name = "browsers", getter)]
    pub fn browsers(this: &IAtDirectiveData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IAtDirectiveData", js_name = "browsers", setter)]
    pub fn set_browsers(this: &IAtDirectiveData, browsers: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IAtDirectiveData", js_name = "status", getter)]
    pub fn status(this: &IAtDirectiveData) -> EntryStatus;

    #[wasm_bindgen(method, js_class = "IAtDirectiveData", js_name = "status", setter)]
    pub fn set_status(this: &IAtDirectiveData, status: EntryStatus);

    #[wasm_bindgen(method, js_class = "IAtDirectiveData", js_name = "references", getter)]
    pub fn references(this: &IAtDirectiveData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IAtDirectiveData", js_name = "references", setter)]
    pub fn set_references(this: &IAtDirectiveData, references: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IPseudoClassData;

    #[wasm_bindgen(method, js_class = "IPseudoClassData", js_name = "name", getter)]
    pub fn name(this: &IPseudoClassData) -> String;

    #[wasm_bindgen(method, js_class = "IPseudoClassData", js_name = "name", setter)]
    pub fn set_name(this: &IPseudoClassData, name: String);

    #[wasm_bindgen(method, js_class = "IPseudoClassData", js_name = "description", getter)]
    pub fn description(this: &IPseudoClassData) -> JsValue;

    #[wasm_bindgen(method, js_class = "IPseudoClassData", js_name = "description", setter)]
    pub fn set_description(this: &IPseudoClassData, description: JsValue);

    #[wasm_bindgen(method, js_class = "IPseudoClassData", js_name = "browsers", getter)]
    pub fn browsers(this: &IPseudoClassData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IPseudoClassData", js_name = "browsers", setter)]
    pub fn set_browsers(this: &IPseudoClassData, browsers: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IPseudoClassData", js_name = "status", getter)]
    pub fn status(this: &IPseudoClassData) -> EntryStatus;

    #[wasm_bindgen(method, js_class = "IPseudoClassData", js_name = "status", setter)]
    pub fn set_status(this: &IPseudoClassData, status: EntryStatus);

    #[wasm_bindgen(method, js_class = "IPseudoClassData", js_name = "references", getter)]
    pub fn references(this: &IPseudoClassData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IPseudoClassData", js_name = "references", setter)]
    pub fn set_references(this: &IPseudoClassData, references: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IPseudoElementData;

    #[wasm_bindgen(method, js_class = "IPseudoElementData", js_name = "name", getter)]
    pub fn name(this: &IPseudoElementData) -> String;

    #[wasm_bindgen(method, js_class = "IPseudoElementData", js_name = "name", setter)]
    pub fn set_name(this: &IPseudoElementData, name: String);

    #[wasm_bindgen(method, js_class = "IPseudoElementData", js_name = "description", getter)]
    pub fn description(this: &IPseudoElementData) -> JsValue;

    #[wasm_bindgen(method, js_class = "IPseudoElementData", js_name = "description", setter)]
    pub fn set_description(this: &IPseudoElementData, description: JsValue);

    #[wasm_bindgen(method, js_class = "IPseudoElementData", js_name = "browsers", getter)]
    pub fn browsers(this: &IPseudoElementData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IPseudoElementData", js_name = "browsers", setter)]
    pub fn set_browsers(this: &IPseudoElementData, browsers: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IPseudoElementData", js_name = "status", getter)]
    pub fn status(this: &IPseudoElementData) -> EntryStatus;

    #[wasm_bindgen(method, js_class = "IPseudoElementData", js_name = "status", setter)]
    pub fn set_status(this: &IPseudoElementData, status: EntryStatus);

    #[wasm_bindgen(method, js_class = "IPseudoElementData", js_name = "references", getter)]
    pub fn references(this: &IPseudoElementData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IPseudoElementData", js_name = "references", setter)]
    pub fn set_references(this: &IPseudoElementData, references: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IValueData;

    #[wasm_bindgen(method, js_class = "IValueData", js_name = "name", getter)]
    pub fn name(this: &IValueData) -> String;

    #[wasm_bindgen(method, js_class = "IValueData", js_name = "name", setter)]
    pub fn set_name(this: &IValueData, name: String);

    #[wasm_bindgen(method, js_class = "IValueData", js_name = "description", getter)]
    pub fn description(this: &IValueData) -> JsValue;

    #[wasm_bindgen(method, js_class = "IValueData", js_name = "description", setter)]
    pub fn set_description(this: &IValueData, description: JsValue);

    #[wasm_bindgen(method, js_class = "IValueData", js_name = "browsers", getter)]
    pub fn browsers(this: &IValueData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IValueData", js_name = "browsers", setter)]
    pub fn set_browsers(this: &IValueData, browsers: js_sys::Array);

    #[wasm_bindgen(method, js_class = "IValueData", js_name = "status", getter)]
    pub fn status(this: &IValueData) -> EntryStatus;

    #[wasm_bindgen(method, js_class = "IValueData", js_name = "status", setter)]
    pub fn set_status(this: &IValueData, status: EntryStatus);

    #[wasm_bindgen(method, js_class = "IValueData", js_name = "references", getter)]
    pub fn references(this: &IValueData) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "IValueData", js_name = "references", setter)]
    pub fn set_references(this: &IValueData, references: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type MarkupContent;

    #[wasm_bindgen(method, js_class = "MarkupContent", js_name = "kind", getter)]
    pub fn kind(this: &MarkupContent) -> MarkupKind;

    #[wasm_bindgen(method, js_class = "MarkupContent", js_name = "kind", setter)]
    pub fn set_kind(this: &MarkupContent, kind: MarkupKind);

    #[wasm_bindgen(method, js_class = "MarkupContent", js_name = "value", getter)]
    pub fn value(this: &MarkupContent) -> String;

    #[wasm_bindgen(method, js_class = "MarkupContent", js_name = "value", setter)]
    pub fn set_value(this: &MarkupContent, value: String);


}

pub type MarkupKind = JsValue
;
}
pub mod html {

#[allow(unused)]
use wasm_bindgen::prelude::*;
#[allow(unused)]
use web_sys::*;
#[allow(unused)]
use js_sys::*;
#[allow(unused)]
use super::*;

#[allow(unused)]
pub type HTMLElement = HtmlElement;
#[allow(unused)]
pub type ReadonlyArray = Array;
#[allow(unused)]
pub type NonNullable = JsValue;
#[allow(unused)]
pub type Record = JsValue;
#[allow(unused)]
pub type PromiseLike = JsValue;



#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type HTMLFormatConfiguration;

    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "tabSize", getter)]
    pub fn tab_size(this: &HTMLFormatConfiguration) -> f64;
    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "insertSpaces", getter)]
    pub fn insert_spaces(this: &HTMLFormatConfiguration) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "wrapLineLength", getter)]
    pub fn wrap_line_length(this: &HTMLFormatConfiguration) -> f64;
    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "unformatted", getter)]
    pub fn unformatted(this: &HTMLFormatConfiguration) -> String;
    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "contentUnformatted", getter)]
    pub fn content_unformatted(this: &HTMLFormatConfiguration) -> String;
    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "indentInnerHtml", getter)]
    pub fn indent_inner_html(this: &HTMLFormatConfiguration) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "preserveNewLines", getter)]
    pub fn preserve_new_lines(this: &HTMLFormatConfiguration) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "maxPreserveNewLines", getter)]
    pub fn max_preserve_new_lines(this: &HTMLFormatConfiguration) -> JsValue;
    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "indentHandlebars", getter)]
    pub fn indent_handlebars(this: &HTMLFormatConfiguration) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "endWithNewline", getter)]
    pub fn end_with_newline(this: &HTMLFormatConfiguration) -> bool;
    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "extraLiners", getter)]
    pub fn extra_liners(this: &HTMLFormatConfiguration) -> String;
    #[wasm_bindgen(method, js_class = "HTMLFormatConfiguration", js_name = "wrapAttributes", getter)]
    pub fn wrap_attributes(this: &HTMLFormatConfiguration) -> JsValue;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CompletionConfiguration;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Options;

    /**
         * Settings for the HTML formatter.
         */ 
    #[wasm_bindgen(method, js_class = "Options", js_name = "format", getter)]
    pub fn format(this: &Options) -> HTMLFormatConfiguration;
    /**
         * Code completion settings.
         */ 
    #[wasm_bindgen(method, js_class = "Options", js_name = "suggest", getter)]
    pub fn suggest(this: &Options) -> CompletionConfiguration;
    /**
         * Configures the HTML data types known by the HTML langauge service.
         */ 
    #[wasm_bindgen(method, js_class = "Options", js_name = "data", getter)]
    pub fn data(this: &Options) -> HTMLDataConfiguration;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ModeConfiguration;

    /**
         * Defines whether the built-in completionItemProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "completionItems", getter)]
    pub fn completion_items(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in hoverProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "hovers", getter)]
    pub fn hovers(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in documentSymbolProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentSymbols", getter)]
    pub fn document_symbols(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in definitions provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "links", getter)]
    pub fn links(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in references provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentHighlights", getter)]
    pub fn document_highlights(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in rename provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "rename", getter)]
    pub fn rename(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in color provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "colors", getter)]
    pub fn colors(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in foldingRange provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "foldingRanges", getter)]
    pub fn folding_ranges(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in diagnostic provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "diagnostics", getter)]
    pub fn diagnostics(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in selection range provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "selectionRanges", getter)]
    pub fn selection_ranges(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in documentFormattingEdit provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentFormattingEdits", getter)]
    pub fn document_formatting_edits(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in documentRangeFormattingEdit provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentRangeFormattingEdits", getter)]
    pub fn document_range_formatting_edits(this: &ModeConfiguration) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LanguageServiceDefaults;

    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "languageId", getter)]
    pub fn language_id(this: &LanguageServiceDefaults) -> String;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "modeConfiguration", getter)]
    pub fn mode_configuration(this: &LanguageServiceDefaults) -> ModeConfiguration;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "onDidChange", getter)]
    pub fn on_did_change(this: &LanguageServiceDefaults) -> IEvent;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "options", getter)]
    pub fn options(this: &LanguageServiceDefaults) -> Options;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setOptions")]
    pub fn set_options(this: &LanguageServiceDefaults, options: Options, );

    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setModeConfiguration")]
    pub fn set_mode_configuration(this: &LanguageServiceDefaults, mode_configuration: ModeConfiguration, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LanguageServiceRegistration;

    #[wasm_bindgen(method, js_class = "LanguageServiceRegistration", js_name = "defaults", getter)]
    pub fn defaults(this: &LanguageServiceRegistration) -> LanguageServiceDefaults;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type HTMLDataConfiguration;

    /**
         * Defines whether the standard HTML tags and attributes are shown
         */ 
    #[wasm_bindgen(method, js_class = "HTMLDataConfiguration", js_name = "useDefaultDataProvider", getter)]
    pub fn use_default_data_provider(this: &HTMLDataConfiguration) -> bool;
    /**
         * Provides a set of custom data providers.
         */ 
    #[wasm_bindgen(method, js_class = "HTMLDataConfiguration", js_name = "dataProviders", getter)]
    pub fn data_providers(this: &HTMLDataConfiguration) -> js_sys::Object;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type HTMLDataV1;

    #[wasm_bindgen(method, js_class = "HTMLDataV1", js_name = "version", getter)]
    pub fn version(this: &HTMLDataV1) -> JsValue;
    #[wasm_bindgen(method, js_class = "HTMLDataV1", js_name = "tags", getter)]
    pub fn tags(this: &HTMLDataV1) -> js_sys::Array;
    #[wasm_bindgen(method, js_class = "HTMLDataV1", js_name = "globalAttributes", getter)]
    pub fn global_attributes(this: &HTMLDataV1) -> js_sys::Array;
    #[wasm_bindgen(method, js_class = "HTMLDataV1", js_name = "valueSets", getter)]
    pub fn value_sets(this: &HTMLDataV1) -> js_sys::Array;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IReference;

    #[wasm_bindgen(method, js_class = "IReference", js_name = "name", getter)]
    pub fn name(this: &IReference) -> String;
    #[wasm_bindgen(method, js_class = "IReference", js_name = "url", getter)]
    pub fn url(this: &IReference) -> String;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ITagData;

    #[wasm_bindgen(method, js_class = "ITagData", js_name = "name", getter)]
    pub fn name(this: &ITagData) -> String;
    #[wasm_bindgen(method, js_class = "ITagData", js_name = "description", getter)]
    pub fn description(this: &ITagData) -> JsValue;
    #[wasm_bindgen(method, js_class = "ITagData", js_name = "attributes", getter)]
    pub fn attributes(this: &ITagData) -> js_sys::Array;
    #[wasm_bindgen(method, js_class = "ITagData", js_name = "references", getter)]
    pub fn references(this: &ITagData) -> js_sys::Array;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IAttributeData;

    #[wasm_bindgen(method, js_class = "IAttributeData", js_name = "name", getter)]
    pub fn name(this: &IAttributeData) -> String;
    #[wasm_bindgen(method, js_class = "IAttributeData", js_name = "description", getter)]
    pub fn description(this: &IAttributeData) -> JsValue;
    #[wasm_bindgen(method, js_class = "IAttributeData", js_name = "valueSet", getter)]
    pub fn value_set(this: &IAttributeData) -> String;
    #[wasm_bindgen(method, js_class = "IAttributeData", js_name = "values", getter)]
    pub fn values(this: &IAttributeData) -> js_sys::Array;
    #[wasm_bindgen(method, js_class = "IAttributeData", js_name = "references", getter)]
    pub fn references(this: &IAttributeData) -> js_sys::Array;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IValueData;

    #[wasm_bindgen(method, js_class = "IValueData", js_name = "name", getter)]
    pub fn name(this: &IValueData) -> String;
    #[wasm_bindgen(method, js_class = "IValueData", js_name = "description", getter)]
    pub fn description(this: &IValueData) -> JsValue;
    #[wasm_bindgen(method, js_class = "IValueData", js_name = "references", getter)]
    pub fn references(this: &IValueData) -> js_sys::Array;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IValueSet;

    #[wasm_bindgen(method, js_class = "IValueSet", js_name = "name", getter)]
    pub fn name(this: &IValueSet) -> String;
    #[wasm_bindgen(method, js_class = "IValueSet", js_name = "values", getter)]
    pub fn values(this: &IValueSet) -> js_sys::Array;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type MarkupContent;

    #[wasm_bindgen(method, js_class = "MarkupContent", js_name = "kind", getter)]
    pub fn kind(this: &MarkupContent) -> MarkupKind;
    #[wasm_bindgen(method, js_class = "MarkupContent", js_name = "value", getter)]
    pub fn value(this: &MarkupContent) -> String;

}

pub type MarkupKind = JsValue
;
}
pub mod json {

#[allow(unused)]
use wasm_bindgen::prelude::*;
#[allow(unused)]
use web_sys::*;
#[allow(unused)]
use js_sys::*;
#[allow(unused)]
use super::*;

#[allow(unused)]
pub type HTMLElement = HtmlElement;
#[allow(unused)]
pub type ReadonlyArray = Array;
#[allow(unused)]
pub type NonNullable = JsValue;
#[allow(unused)]
pub type Record = JsValue;
#[allow(unused)]
pub type PromiseLike = JsValue;



#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DiagnosticsOptions;

    /**
         * If set, the validator will be enabled and perform syntax and schema based validation,
         * unless `DiagnosticsOptions.schemaValidation` is set to `ignore`.
         */ 
    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "validate", getter)]
    pub fn validate(this: &DiagnosticsOptions) -> bool;
    /**
         * If set, comments are tolerated. If set to false, syntax errors will be emitted for comments.
         * `DiagnosticsOptions.allowComments` will override this setting.
         */ 
    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "allowComments", getter)]
    pub fn allow_comments(this: &DiagnosticsOptions) -> bool;
    /**
         * A list of known schemas and/or associations of schemas to file names.
         */ 
    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "schemas", getter)]
    pub fn schemas(this: &DiagnosticsOptions) -> js_sys::Array;
    /**
         *  If set, the schema service would load schema content on-demand with 'fetch' if available
         */ 
    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "enableSchemaRequest", getter)]
    pub fn enable_schema_request(this: &DiagnosticsOptions) -> bool;
    /**
         * The severity of problems from schema validation. If set to 'ignore', schema validation will be skipped. If not set, 'warning' is used.
         */ 
    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "schemaValidation", getter)]
    pub fn schema_validation(this: &DiagnosticsOptions) -> SeverityLevel;
    /**
         * The severity of problems that occurred when resolving and loading schemas. If set to 'ignore', schema resolving problems are not reported. If not set, 'warning' is used.
         */ 
    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "schemaRequest", getter)]
    pub fn schema_request(this: &DiagnosticsOptions) -> SeverityLevel;
    /**
         * The severity of reported trailing commas. If not set, trailing commas will be reported as errors.
         */ 
    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "trailingCommas", getter)]
    pub fn trailing_commas(this: &DiagnosticsOptions) -> SeverityLevel;
    /**
         * The severity of reported comments. If not set, 'DiagnosticsOptions.allowComments' defines whether comments are ignored or reported as errors.
         */ 
    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "comments", getter)]
    pub fn comments(this: &DiagnosticsOptions) -> SeverityLevel;

}

pub type SeverityLevel = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ModeConfiguration;

    /**
         * Defines whether the built-in documentFormattingEdit provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentFormattingEdits", getter)]
    pub fn document_formatting_edits(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in documentRangeFormattingEdit provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentRangeFormattingEdits", getter)]
    pub fn document_range_formatting_edits(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in completionItemProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "completionItems", getter)]
    pub fn completion_items(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in hoverProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "hovers", getter)]
    pub fn hovers(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in documentSymbolProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentSymbols", getter)]
    pub fn document_symbols(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in tokens provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "tokens", getter)]
    pub fn tokens(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in color provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "colors", getter)]
    pub fn colors(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in foldingRange provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "foldingRanges", getter)]
    pub fn folding_ranges(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in diagnostic provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "diagnostics", getter)]
    pub fn diagnostics(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in selection range provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "selectionRanges", getter)]
    pub fn selection_ranges(this: &ModeConfiguration) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LanguageServiceDefaults;

    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "languageId", getter)]
    pub fn language_id(this: &LanguageServiceDefaults) -> String;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "onDidChange", getter)]
    pub fn on_did_change(this: &LanguageServiceDefaults) -> IEvent;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "diagnosticsOptions", getter)]
    pub fn diagnostics_options(this: &LanguageServiceDefaults) -> DiagnosticsOptions;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "modeConfiguration", getter)]
    pub fn mode_configuration(this: &LanguageServiceDefaults) -> ModeConfiguration;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setDiagnosticsOptions")]
    pub fn set_diagnostics_options(this: &LanguageServiceDefaults, options: DiagnosticsOptions, );

    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setModeConfiguration")]
    pub fn set_mode_configuration(this: &LanguageServiceDefaults, mode_configuration: ModeConfiguration, );


}

}
pub mod typescript {

#[allow(unused)]
use wasm_bindgen::prelude::*;
#[allow(unused)]
use web_sys::*;
#[allow(unused)]
use js_sys::*;
#[allow(unused)]
use super::*;

#[allow(unused)]
pub type HTMLElement = HtmlElement;
#[allow(unused)]
pub type ReadonlyArray = Array;
#[allow(unused)]
pub type NonNullable = JsValue;
#[allow(unused)]
pub type Record = JsValue;
#[allow(unused)]
pub type PromiseLike = JsValue;



            int_enum! {
#[allow(non_camel_case_types)]
pub enum ModuleKind {
    None = 0,
    CommonJS = 1,
    AMD = 2,
    UMD = 3,
    System = 4,
    ES2015 = 5,
    ESNext = 99,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum JsxEmit {
    None = 0,
    Preserve = 1,
    React = 2,
    ReactNative = 3,
    ReactJSX = 4,
    ReactJSXDev = 5,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum NewLineKind {
    CarriageReturnLineFeed = 0,
    LineFeed = 1,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum ScriptTarget {
    ES3 = 0,
    ES5 = 1,
    ES2015 = 2,
    ES2016 = 3,
    ES2017 = 4,
    ES2018 = 5,
    ES2019 = 6,
    ES2020 = 7,
    ESNext = 99,
    JSON = 100,
    Latest = 99,

  }
}


            int_enum! {
#[allow(non_camel_case_types)]
pub enum ModuleResolutionKind {
    Classic = 1,
    NodeJs = 2,

  }
}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type MapLike;


}

pub type CompilerOptionsValue = JsValue
;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type CompilerOptions;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "allowJs", getter)]
    pub fn allow_js(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "allowJs", setter)]
    pub fn set_allow_js(this: &CompilerOptions, allow_js: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "allowSyntheticDefaultImports", getter)]
    pub fn allow_synthetic_default_imports(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "allowSyntheticDefaultImports", setter)]
    pub fn set_allow_synthetic_default_imports(this: &CompilerOptions, allow_synthetic_default_imports: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "allowUmdGlobalAccess", getter)]
    pub fn allow_umd_global_access(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "allowUmdGlobalAccess", setter)]
    pub fn set_allow_umd_global_access(this: &CompilerOptions, allow_umd_global_access: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "allowUnreachableCode", getter)]
    pub fn allow_unreachable_code(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "allowUnreachableCode", setter)]
    pub fn set_allow_unreachable_code(this: &CompilerOptions, allow_unreachable_code: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "allowUnusedLabels", getter)]
    pub fn allow_unused_labels(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "allowUnusedLabels", setter)]
    pub fn set_allow_unused_labels(this: &CompilerOptions, allow_unused_labels: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "alwaysStrict", getter)]
    pub fn always_strict(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "alwaysStrict", setter)]
    pub fn set_always_strict(this: &CompilerOptions, always_strict: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "baseUrl", getter)]
    pub fn base_url(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "baseUrl", setter)]
    pub fn set_base_url(this: &CompilerOptions, base_url: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "charset", getter)]
    pub fn charset(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "charset", setter)]
    pub fn set_charset(this: &CompilerOptions, charset: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "checkJs", getter)]
    pub fn check_js(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "checkJs", setter)]
    pub fn set_check_js(this: &CompilerOptions, check_js: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "declaration", getter)]
    pub fn declaration(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "declaration", setter)]
    pub fn set_declaration(this: &CompilerOptions, declaration: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "declarationMap", getter)]
    pub fn declaration_map(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "declarationMap", setter)]
    pub fn set_declaration_map(this: &CompilerOptions, declaration_map: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "emitDeclarationOnly", getter)]
    pub fn emit_declaration_only(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "emitDeclarationOnly", setter)]
    pub fn set_emit_declaration_only(this: &CompilerOptions, emit_declaration_only: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "declarationDir", getter)]
    pub fn declaration_dir(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "declarationDir", setter)]
    pub fn set_declaration_dir(this: &CompilerOptions, declaration_dir: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "disableSizeLimit", getter)]
    pub fn disable_size_limit(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "disableSizeLimit", setter)]
    pub fn set_disable_size_limit(this: &CompilerOptions, disable_size_limit: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "disableSourceOfProjectReferenceRedirect", getter)]
    pub fn disable_source_of_project_reference_redirect(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "disableSourceOfProjectReferenceRedirect", setter)]
    pub fn set_disable_source_of_project_reference_redirect(this: &CompilerOptions, disable_source_of_project_reference_redirect: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "downlevelIteration", getter)]
    pub fn downlevel_iteration(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "downlevelIteration", setter)]
    pub fn set_downlevel_iteration(this: &CompilerOptions, downlevel_iteration: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "emitBOM", getter)]
    pub fn emit_bom(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "emitBOM", setter)]
    pub fn set_emit_bom(this: &CompilerOptions, emit_bom: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "emitDecoratorMetadata", getter)]
    pub fn emit_decorator_metadata(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "emitDecoratorMetadata", setter)]
    pub fn set_emit_decorator_metadata(this: &CompilerOptions, emit_decorator_metadata: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "experimentalDecorators", getter)]
    pub fn experimental_decorators(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "experimentalDecorators", setter)]
    pub fn set_experimental_decorators(this: &CompilerOptions, experimental_decorators: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "forceConsistentCasingInFileNames", getter)]
    pub fn force_consistent_casing_in_file_names(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "forceConsistentCasingInFileNames", setter)]
    pub fn set_force_consistent_casing_in_file_names(this: &CompilerOptions, force_consistent_casing_in_file_names: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "importHelpers", getter)]
    pub fn import_helpers(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "importHelpers", setter)]
    pub fn set_import_helpers(this: &CompilerOptions, import_helpers: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "inlineSourceMap", getter)]
    pub fn inline_source_map(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "inlineSourceMap", setter)]
    pub fn set_inline_source_map(this: &CompilerOptions, inline_source_map: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "inlineSources", getter)]
    pub fn inline_sources(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "inlineSources", setter)]
    pub fn set_inline_sources(this: &CompilerOptions, inline_sources: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "isolatedModules", getter)]
    pub fn isolated_modules(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "isolatedModules", setter)]
    pub fn set_isolated_modules(this: &CompilerOptions, isolated_modules: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "jsx", getter)]
    pub fn jsx(this: &CompilerOptions) -> JsxEmit;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "jsx", setter)]
    pub fn set_jsx(this: &CompilerOptions, jsx: JsxEmit);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "keyofStringsOnly", getter)]
    pub fn keyof_strings_only(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "keyofStringsOnly", setter)]
    pub fn set_keyof_strings_only(this: &CompilerOptions, keyof_strings_only: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "lib", getter)]
    pub fn lib(this: &CompilerOptions) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "lib", setter)]
    pub fn set_lib(this: &CompilerOptions, lib: js_sys::Array);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "locale", getter)]
    pub fn locale(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "locale", setter)]
    pub fn set_locale(this: &CompilerOptions, locale: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "mapRoot", getter)]
    pub fn map_root(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "mapRoot", setter)]
    pub fn set_map_root(this: &CompilerOptions, map_root: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "maxNodeModuleJsDepth", getter)]
    pub fn max_node_module_js_depth(this: &CompilerOptions) -> f64;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "maxNodeModuleJsDepth", setter)]
    pub fn set_max_node_module_js_depth(this: &CompilerOptions, max_node_module_js_depth: f64);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "module", getter)]
    pub fn module(this: &CompilerOptions) -> ModuleKind;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "module", setter)]
    pub fn set_module(this: &CompilerOptions, module: ModuleKind);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "moduleResolution", getter)]
    pub fn module_resolution(this: &CompilerOptions) -> ModuleResolutionKind;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "moduleResolution", setter)]
    pub fn set_module_resolution(this: &CompilerOptions, module_resolution: ModuleResolutionKind);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "newLine", getter)]
    pub fn new_line(this: &CompilerOptions) -> NewLineKind;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "newLine", setter)]
    pub fn set_new_line(this: &CompilerOptions, new_line: NewLineKind);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noEmit", getter)]
    pub fn no_emit(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noEmit", setter)]
    pub fn set_no_emit(this: &CompilerOptions, no_emit: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noEmitHelpers", getter)]
    pub fn no_emit_helpers(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noEmitHelpers", setter)]
    pub fn set_no_emit_helpers(this: &CompilerOptions, no_emit_helpers: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noEmitOnError", getter)]
    pub fn no_emit_on_error(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noEmitOnError", setter)]
    pub fn set_no_emit_on_error(this: &CompilerOptions, no_emit_on_error: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noErrorTruncation", getter)]
    pub fn no_error_truncation(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noErrorTruncation", setter)]
    pub fn set_no_error_truncation(this: &CompilerOptions, no_error_truncation: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noFallthroughCasesInSwitch", getter)]
    pub fn no_fallthrough_cases_in_switch(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noFallthroughCasesInSwitch", setter)]
    pub fn set_no_fallthrough_cases_in_switch(this: &CompilerOptions, no_fallthrough_cases_in_switch: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noImplicitAny", getter)]
    pub fn no_implicit_any(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noImplicitAny", setter)]
    pub fn set_no_implicit_any(this: &CompilerOptions, no_implicit_any: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noImplicitReturns", getter)]
    pub fn no_implicit_returns(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noImplicitReturns", setter)]
    pub fn set_no_implicit_returns(this: &CompilerOptions, no_implicit_returns: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noImplicitThis", getter)]
    pub fn no_implicit_this(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noImplicitThis", setter)]
    pub fn set_no_implicit_this(this: &CompilerOptions, no_implicit_this: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noStrictGenericChecks", getter)]
    pub fn no_strict_generic_checks(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noStrictGenericChecks", setter)]
    pub fn set_no_strict_generic_checks(this: &CompilerOptions, no_strict_generic_checks: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noUnusedLocals", getter)]
    pub fn no_unused_locals(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noUnusedLocals", setter)]
    pub fn set_no_unused_locals(this: &CompilerOptions, no_unused_locals: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noUnusedParameters", getter)]
    pub fn no_unused_parameters(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noUnusedParameters", setter)]
    pub fn set_no_unused_parameters(this: &CompilerOptions, no_unused_parameters: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noImplicitUseStrict", getter)]
    pub fn no_implicit_use_strict(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noImplicitUseStrict", setter)]
    pub fn set_no_implicit_use_strict(this: &CompilerOptions, no_implicit_use_strict: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noLib", getter)]
    pub fn no_lib(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noLib", setter)]
    pub fn set_no_lib(this: &CompilerOptions, no_lib: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noResolve", getter)]
    pub fn no_resolve(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "noResolve", setter)]
    pub fn set_no_resolve(this: &CompilerOptions, no_resolve: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "out", getter)]
    pub fn out(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "out", setter)]
    pub fn set_out(this: &CompilerOptions, out: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "outDir", getter)]
    pub fn out_dir(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "outDir", setter)]
    pub fn set_out_dir(this: &CompilerOptions, out_dir: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "outFile", getter)]
    pub fn out_file(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "outFile", setter)]
    pub fn set_out_file(this: &CompilerOptions, out_file: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "paths", getter)]
    pub fn paths(this: &CompilerOptions) -> MapLike;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "paths", setter)]
    pub fn set_paths(this: &CompilerOptions, paths: MapLike);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "preserveConstEnums", getter)]
    pub fn preserve_const_enums(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "preserveConstEnums", setter)]
    pub fn set_preserve_const_enums(this: &CompilerOptions, preserve_const_enums: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "preserveSymlinks", getter)]
    pub fn preserve_symlinks(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "preserveSymlinks", setter)]
    pub fn set_preserve_symlinks(this: &CompilerOptions, preserve_symlinks: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "project", getter)]
    pub fn project(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "project", setter)]
    pub fn set_project(this: &CompilerOptions, project: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "reactNamespace", getter)]
    pub fn react_namespace(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "reactNamespace", setter)]
    pub fn set_react_namespace(this: &CompilerOptions, react_namespace: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "jsxFactory", getter)]
    pub fn jsx_factory(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "jsxFactory", setter)]
    pub fn set_jsx_factory(this: &CompilerOptions, jsx_factory: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "composite", getter)]
    pub fn composite(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "composite", setter)]
    pub fn set_composite(this: &CompilerOptions, composite: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "removeComments", getter)]
    pub fn remove_comments(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "removeComments", setter)]
    pub fn set_remove_comments(this: &CompilerOptions, remove_comments: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "rootDir", getter)]
    pub fn root_dir(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "rootDir", setter)]
    pub fn set_root_dir(this: &CompilerOptions, root_dir: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "rootDirs", getter)]
    pub fn root_dirs(this: &CompilerOptions) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "rootDirs", setter)]
    pub fn set_root_dirs(this: &CompilerOptions, root_dirs: js_sys::Array);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "skipLibCheck", getter)]
    pub fn skip_lib_check(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "skipLibCheck", setter)]
    pub fn set_skip_lib_check(this: &CompilerOptions, skip_lib_check: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "skipDefaultLibCheck", getter)]
    pub fn skip_default_lib_check(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "skipDefaultLibCheck", setter)]
    pub fn set_skip_default_lib_check(this: &CompilerOptions, skip_default_lib_check: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "sourceMap", getter)]
    pub fn source_map(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "sourceMap", setter)]
    pub fn set_source_map(this: &CompilerOptions, source_map: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "sourceRoot", getter)]
    pub fn source_root(this: &CompilerOptions) -> String;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "sourceRoot", setter)]
    pub fn set_source_root(this: &CompilerOptions, source_root: String);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "strict", getter)]
    pub fn strict(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "strict", setter)]
    pub fn set_strict(this: &CompilerOptions, strict: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "strictFunctionTypes", getter)]
    pub fn strict_function_types(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "strictFunctionTypes", setter)]
    pub fn set_strict_function_types(this: &CompilerOptions, strict_function_types: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "strictBindCallApply", getter)]
    pub fn strict_bind_call_apply(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "strictBindCallApply", setter)]
    pub fn set_strict_bind_call_apply(this: &CompilerOptions, strict_bind_call_apply: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "strictNullChecks", getter)]
    pub fn strict_null_checks(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "strictNullChecks", setter)]
    pub fn set_strict_null_checks(this: &CompilerOptions, strict_null_checks: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "strictPropertyInitialization", getter)]
    pub fn strict_property_initialization(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "strictPropertyInitialization", setter)]
    pub fn set_strict_property_initialization(this: &CompilerOptions, strict_property_initialization: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "stripInternal", getter)]
    pub fn strip_internal(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "stripInternal", setter)]
    pub fn set_strip_internal(this: &CompilerOptions, strip_internal: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "suppressExcessPropertyErrors", getter)]
    pub fn suppress_excess_property_errors(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "suppressExcessPropertyErrors", setter)]
    pub fn set_suppress_excess_property_errors(this: &CompilerOptions, suppress_excess_property_errors: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "suppressImplicitAnyIndexErrors", getter)]
    pub fn suppress_implicit_any_index_errors(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "suppressImplicitAnyIndexErrors", setter)]
    pub fn set_suppress_implicit_any_index_errors(this: &CompilerOptions, suppress_implicit_any_index_errors: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "target", getter)]
    pub fn target(this: &CompilerOptions) -> ScriptTarget;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "target", setter)]
    pub fn set_target(this: &CompilerOptions, target: ScriptTarget);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "traceResolution", getter)]
    pub fn trace_resolution(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "traceResolution", setter)]
    pub fn set_trace_resolution(this: &CompilerOptions, trace_resolution: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "resolveJsonModule", getter)]
    pub fn resolve_json_module(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "resolveJsonModule", setter)]
    pub fn set_resolve_json_module(this: &CompilerOptions, resolve_json_module: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "types", getter)]
    pub fn types(this: &CompilerOptions) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "types", setter)]
    pub fn set_types(this: &CompilerOptions, types: js_sys::Array);

    /** Paths used to compute primary types search locations */ 
    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "typeRoots", getter)]
    pub fn type_roots(this: &CompilerOptions) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "typeRoots", setter)]
    pub fn set_type_roots(this: &CompilerOptions, type_roots: js_sys::Array);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "esModuleInterop", getter)]
    pub fn es_module_interop(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "esModuleInterop", setter)]
    pub fn set_es_module_interop(this: &CompilerOptions, es_module_interop: bool);

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "useDefineForClassFields", getter)]
    pub fn use_define_for_class_fields(this: &CompilerOptions) -> bool;

    #[wasm_bindgen(method, js_class = "CompilerOptions", js_name = "useDefineForClassFields", setter)]
    pub fn set_use_define_for_class_fields(this: &CompilerOptions, use_define_for_class_fields: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DiagnosticsOptions;

    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "noSemanticValidation", getter)]
    pub fn no_semantic_validation(this: &DiagnosticsOptions) -> bool;

    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "noSemanticValidation", setter)]
    pub fn set_no_semantic_validation(this: &DiagnosticsOptions, no_semantic_validation: bool);

    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "noSyntaxValidation", getter)]
    pub fn no_syntax_validation(this: &DiagnosticsOptions) -> bool;

    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "noSyntaxValidation", setter)]
    pub fn set_no_syntax_validation(this: &DiagnosticsOptions, no_syntax_validation: bool);

    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "noSuggestionDiagnostics", getter)]
    pub fn no_suggestion_diagnostics(this: &DiagnosticsOptions) -> bool;

    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "noSuggestionDiagnostics", setter)]
    pub fn set_no_suggestion_diagnostics(this: &DiagnosticsOptions, no_suggestion_diagnostics: bool);

    /**
         * Limit diagnostic computation to only visible files.
         * Defaults to false.
         */ 
    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "onlyVisible", getter)]
    pub fn only_visible(this: &DiagnosticsOptions) -> bool;

    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "onlyVisible", setter)]
    pub fn set_only_visible(this: &DiagnosticsOptions, only_visible: bool);

    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "diagnosticCodesToIgnore", getter)]
    pub fn diagnostic_codes_to_ignore(this: &DiagnosticsOptions) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "DiagnosticsOptions", js_name = "diagnosticCodesToIgnore", setter)]
    pub fn set_diagnostic_codes_to_ignore(this: &DiagnosticsOptions, diagnostic_codes_to_ignore: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type WorkerOptions;

    /** A full HTTP path to a JavaScript file which adds a function `customTSWorkerFactory` to the self inside a web-worker */ 
    #[wasm_bindgen(method, js_class = "WorkerOptions", js_name = "customWorkerPath", getter)]
    pub fn custom_worker_path(this: &WorkerOptions) -> String;

    #[wasm_bindgen(method, js_class = "WorkerOptions", js_name = "customWorkerPath", setter)]
    pub fn set_custom_worker_path(this: &WorkerOptions, custom_worker_path: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type InlayHintsOptions;

    #[wasm_bindgen(method, js_class = "InlayHintsOptions", js_name = "includeInlayParameterNameHints", getter)]
    pub fn include_inlay_parameter_name_hints(this: &InlayHintsOptions) -> JsValue;
    #[wasm_bindgen(method, js_class = "InlayHintsOptions", js_name = "includeInlayParameterNameHintsWhenArgumentMatchesName", getter)]
    pub fn include_inlay_parameter_name_hints_when_argument_matches_name(this: &InlayHintsOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InlayHintsOptions", js_name = "includeInlayFunctionParameterTypeHints", getter)]
    pub fn include_inlay_function_parameter_type_hints(this: &InlayHintsOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InlayHintsOptions", js_name = "includeInlayVariableTypeHints", getter)]
    pub fn include_inlay_variable_type_hints(this: &InlayHintsOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InlayHintsOptions", js_name = "includeInlayPropertyDeclarationTypeHints", getter)]
    pub fn include_inlay_property_declaration_type_hints(this: &InlayHintsOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InlayHintsOptions", js_name = "includeInlayFunctionLikeReturnTypeHints", getter)]
    pub fn include_inlay_function_like_return_type_hints(this: &InlayHintsOptions) -> bool;
    #[wasm_bindgen(method, js_class = "InlayHintsOptions", js_name = "includeInlayEnumMemberValueHints", getter)]
    pub fn include_inlay_enum_member_value_hints(this: &InlayHintsOptions) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IExtraLib;

    #[wasm_bindgen(method, js_class = "IExtraLib", js_name = "content", getter)]
    pub fn content(this: &IExtraLib) -> String;

    #[wasm_bindgen(method, js_class = "IExtraLib", js_name = "content", setter)]
    pub fn set_content(this: &IExtraLib, content: String);

    #[wasm_bindgen(method, js_class = "IExtraLib", js_name = "version", getter)]
    pub fn version(this: &IExtraLib) -> f64;

    #[wasm_bindgen(method, js_class = "IExtraLib", js_name = "version", setter)]
    pub fn set_version(this: &IExtraLib, version: f64);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IExtraLibs;


}

    /**
     * A linked list of formatted diagnostic messages to be used as part of a multiline message.
     * It is built from the bottom up, leaving the head to be the "main" diagnostic.
     */ 

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DiagnosticMessageChain;

    #[wasm_bindgen(method, js_class = "DiagnosticMessageChain", js_name = "messageText", getter)]
    pub fn message_text(this: &DiagnosticMessageChain) -> String;

    #[wasm_bindgen(method, js_class = "DiagnosticMessageChain", js_name = "messageText", setter)]
    pub fn set_message_text(this: &DiagnosticMessageChain, message_text: String);

    /** Diagnostic category: warning = 0, error = 1, suggestion = 2, message = 3 */ 
    #[wasm_bindgen(method, js_class = "DiagnosticMessageChain", js_name = "category", getter)]
    pub fn category(this: &DiagnosticMessageChain) -> JsValue;

    #[wasm_bindgen(method, js_class = "DiagnosticMessageChain", js_name = "category", setter)]
    pub fn set_category(this: &DiagnosticMessageChain, category: JsValue);

    #[wasm_bindgen(method, js_class = "DiagnosticMessageChain", js_name = "code", getter)]
    pub fn code(this: &DiagnosticMessageChain) -> f64;

    #[wasm_bindgen(method, js_class = "DiagnosticMessageChain", js_name = "code", setter)]
    pub fn set_code(this: &DiagnosticMessageChain, code: f64);

    #[wasm_bindgen(method, js_class = "DiagnosticMessageChain", js_name = "next", getter)]
    pub fn next(this: &DiagnosticMessageChain) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "DiagnosticMessageChain", js_name = "next", setter)]
    pub fn set_next(this: &DiagnosticMessageChain, next: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type Diagnostic;

    /** May store more in future. For now, this will simply be `true` to indicate when a diagnostic is an unused-identifier diagnostic. */ 
    #[wasm_bindgen(method, js_class = "Diagnostic", js_name = "reportsUnnecessary", getter)]
    pub fn reports_unnecessary(this: &Diagnostic) -> js_sys::Object;

    #[wasm_bindgen(method, js_class = "Diagnostic", js_name = "reportsUnnecessary", setter)]
    pub fn set_reports_unnecessary(this: &Diagnostic, reports_unnecessary: js_sys::Object);

    #[wasm_bindgen(method, js_class = "Diagnostic", js_name = "reportsDeprecated", getter)]
    pub fn reports_deprecated(this: &Diagnostic) -> js_sys::Object;

    #[wasm_bindgen(method, js_class = "Diagnostic", js_name = "reportsDeprecated", setter)]
    pub fn set_reports_deprecated(this: &Diagnostic, reports_deprecated: js_sys::Object);

    #[wasm_bindgen(method, js_class = "Diagnostic", js_name = "source", getter)]
    pub fn source(this: &Diagnostic) -> String;

    #[wasm_bindgen(method, js_class = "Diagnostic", js_name = "source", setter)]
    pub fn set_source(this: &Diagnostic, source: String);

    #[wasm_bindgen(method, js_class = "Diagnostic", js_name = "relatedInformation", getter)]
    pub fn related_information(this: &Diagnostic) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "Diagnostic", js_name = "relatedInformation", setter)]
    pub fn set_related_information(this: &Diagnostic, related_information: js_sys::Array);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type DiagnosticRelatedInformation;

    /** Diagnostic category: warning = 0, error = 1, suggestion = 2, message = 3 */ 
    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "category", getter)]
    pub fn category(this: &DiagnosticRelatedInformation) -> JsValue;

    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "category", setter)]
    pub fn set_category(this: &DiagnosticRelatedInformation, category: JsValue);

    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "code", getter)]
    pub fn code(this: &DiagnosticRelatedInformation) -> f64;

    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "code", setter)]
    pub fn set_code(this: &DiagnosticRelatedInformation, code: f64);

    /** TypeScriptWorker removes all but the `fileName` property to avoid serializing circular JSON structures. */ 
    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "file", getter)]
    pub fn file(this: &DiagnosticRelatedInformation) -> JsValue;

    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "file", setter)]
    pub fn set_file(this: &DiagnosticRelatedInformation, file: JsValue);

    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "start", getter)]
    pub fn start(this: &DiagnosticRelatedInformation) -> JsValue;

    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "start", setter)]
    pub fn set_start(this: &DiagnosticRelatedInformation, start: JsValue);

    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "length", getter)]
    pub fn length(this: &DiagnosticRelatedInformation) -> JsValue;

    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "length", setter)]
    pub fn set_length(this: &DiagnosticRelatedInformation, length: JsValue);

    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "messageText", getter)]
    pub fn message_text(this: &DiagnosticRelatedInformation) -> JsValue;

    #[wasm_bindgen(method, js_class = "DiagnosticRelatedInformation", js_name = "messageText", setter)]
    pub fn set_message_text(this: &DiagnosticRelatedInformation, message_text: JsValue);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type EmitOutput;

    #[wasm_bindgen(method, js_class = "EmitOutput", js_name = "outputFiles", getter)]
    pub fn output_files(this: &EmitOutput) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "EmitOutput", js_name = "outputFiles", setter)]
    pub fn set_output_files(this: &EmitOutput, output_files: js_sys::Array);

    #[wasm_bindgen(method, js_class = "EmitOutput", js_name = "emitSkipped", getter)]
    pub fn emit_skipped(this: &EmitOutput) -> bool;

    #[wasm_bindgen(method, js_class = "EmitOutput", js_name = "emitSkipped", setter)]
    pub fn set_emit_skipped(this: &EmitOutput, emit_skipped: bool);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type OutputFile;

    #[wasm_bindgen(method, js_class = "OutputFile", js_name = "name", getter)]
    pub fn name(this: &OutputFile) -> String;

    #[wasm_bindgen(method, js_class = "OutputFile", js_name = "name", setter)]
    pub fn set_name(this: &OutputFile, name: String);

    #[wasm_bindgen(method, js_class = "OutputFile", js_name = "writeByteOrderMark", getter)]
    pub fn write_byte_order_mark(this: &OutputFile) -> bool;

    #[wasm_bindgen(method, js_class = "OutputFile", js_name = "writeByteOrderMark", setter)]
    pub fn set_write_byte_order_mark(this: &OutputFile, write_byte_order_mark: bool);

    #[wasm_bindgen(method, js_class = "OutputFile", js_name = "text", getter)]
    pub fn text(this: &OutputFile) -> String;

    #[wasm_bindgen(method, js_class = "OutputFile", js_name = "text", setter)]
    pub fn set_text(this: &OutputFile, text: String);


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type ModeConfiguration;

    /**
         * Defines whether the built-in completionItemProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "completionItems", getter)]
    pub fn completion_items(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in hoverProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "hovers", getter)]
    pub fn hovers(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in documentSymbolProvider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentSymbols", getter)]
    pub fn document_symbols(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in definitions provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "definitions", getter)]
    pub fn definitions(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in references provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "references", getter)]
    pub fn references(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in references provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentHighlights", getter)]
    pub fn document_highlights(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in rename provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "rename", getter)]
    pub fn rename(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in diagnostic provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "diagnostics", getter)]
    pub fn diagnostics(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in document formatting range edit provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "documentRangeFormattingEdits", getter)]
    pub fn document_range_formatting_edits(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in signature help provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "signatureHelp", getter)]
    pub fn signature_help(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in onType formatting edit provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "onTypeFormattingEdits", getter)]
    pub fn on_type_formatting_edits(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in code actions provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "codeActions", getter)]
    pub fn code_actions(this: &ModeConfiguration) -> bool;
    /**
         * Defines whether the built-in inlay hints provider is enabled.
         */ 
    #[wasm_bindgen(method, js_class = "ModeConfiguration", js_name = "inlayHints", getter)]
    pub fn inlay_hints(this: &ModeConfiguration) -> bool;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type LanguageServiceDefaults;

    /**
         * Event fired when compiler options or diagnostics options are changed.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "onDidChange", getter)]
    pub fn on_did_change(this: &LanguageServiceDefaults) -> IEvent;
    /**
         * Event fired when extra libraries registered with the language service change.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "onDidExtraLibsChange", getter)]
    pub fn on_did_extra_libs_change(this: &LanguageServiceDefaults) -> IEvent;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "workerOptions", getter)]
    pub fn worker_options(this: &LanguageServiceDefaults) -> WorkerOptions;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "inlayHintsOptions", getter)]
    pub fn inlay_hints_options(this: &LanguageServiceDefaults) -> InlayHintsOptions;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "modeConfiguration", getter)]
    pub fn mode_configuration(this: &LanguageServiceDefaults) -> ModeConfiguration;
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setModeConfiguration")]
    pub fn set_mode_configuration(this: &LanguageServiceDefaults, mode_configuration: ModeConfiguration, );

    /**
         * Get the current extra libs registered with the language service.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "getExtraLibs")]
    pub fn get_extra_libs(this: &LanguageServiceDefaults, ) -> IExtraLibs;

    /**
         * Add an additional source file to the language service. Use this
         * for typescript (definition) files that won't be loaded as editor
         * documents, like `jquery.d.ts`.
         *
         * @param content The file content
         * @param filePath An optional file path
         * @returns A disposable which will remove the file from the
         * language service upon disposal.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "addExtraLib")]
    pub fn add_extra_lib(this: &LanguageServiceDefaults, content: String, file_path: Option<String>, ) -> IDisposable;

    /**
         * Remove all existing extra libs and set the additional source
         * files to the language service. Use this for typescript definition
         * files that won't be loaded as editor documents, like `jquery.d.ts`.
         * @param libs An array of entries to register.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setExtraLibs")]
    pub fn set_extra_libs(this: &LanguageServiceDefaults, libs: js_sys::Array, );

    /**
         * Get current TypeScript compiler options for the language service.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "getCompilerOptions")]
    pub fn get_compiler_options(this: &LanguageServiceDefaults, ) -> CompilerOptions;

    /**
         * Set TypeScript compiler options.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setCompilerOptions")]
    pub fn set_compiler_options(this: &LanguageServiceDefaults, options: CompilerOptions, );

    /**
         * Get the current diagnostics options for the language service.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "getDiagnosticsOptions")]
    pub fn get_diagnostics_options(this: &LanguageServiceDefaults, ) -> DiagnosticsOptions;

    /**
         * Configure whether syntactic and/or semantic validation should
         * be performed
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setDiagnosticsOptions")]
    pub fn set_diagnostics_options(this: &LanguageServiceDefaults, options: DiagnosticsOptions, );

    /**
         * Configure webworker options
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setWorkerOptions")]
    pub fn set_worker_options(this: &LanguageServiceDefaults, options: WorkerOptions, );

    /**
         * No-op.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setMaximumWorkerIdleTime")]
    pub fn set_maximum_worker_idle_time(this: &LanguageServiceDefaults, value: f64, );

    /**
         * Configure if all existing models should be eagerly sync'd
         * to the worker on start or restart.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setEagerModelSync")]
    pub fn set_eager_model_sync(this: &LanguageServiceDefaults, value: bool, );

    /**
         * Get the current setting for whether all existing models should be eagerly sync'd
         * to the worker on start or restart.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "getEagerModelSync")]
    pub fn get_eager_model_sync(this: &LanguageServiceDefaults, ) -> bool;

    /**
         * Configure inlay hints options.
         */ 
    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "setInlayHintsOptions")]
    pub fn set_inlay_hints_options(this: &LanguageServiceDefaults, options: InlayHintsOptions, );


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type TypeScriptWorker;

    /**
         * Get diagnostic messages for any syntax issues in the given file.
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getSyntacticDiagnostics")]
    pub fn get_syntactic_diagnostics(this: &TypeScriptWorker, file_name: String, ) -> Promise;

    /**
         * Get diagnostic messages for any semantic issues in the given file.
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getSemanticDiagnostics")]
    pub fn get_semantic_diagnostics(this: &TypeScriptWorker, file_name: String, ) -> Promise;

    /**
         * Get diagnostic messages for any suggestions related to the given file.
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getSuggestionDiagnostics")]
    pub fn get_suggestion_diagnostics(this: &TypeScriptWorker, file_name: String, ) -> Promise;

    /**
         * Get the content of a given file.
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getScriptText")]
    pub fn get_script_text(this: &TypeScriptWorker, file_name: String, ) -> Promise;

    /**
         * Get diagnostic messages related to the current compiler options.
         * @param fileName Not used
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getCompilerOptionsDiagnostics")]
    pub fn get_compiler_options_diagnostics(this: &TypeScriptWorker, file_name: String, ) -> Promise;

    /**
         * Get code completions for the given file and position.
         * @returns `Promise<typescript.CompletionInfo | undefined>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getCompletionsAtPosition")]
    pub fn get_completions_at_position(this: &TypeScriptWorker, file_name: String, position: f64, ) -> Promise;

    /**
         * Get code completion details for the given file, position, and entry.
         * @returns `Promise<typescript.CompletionEntryDetails | undefined>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getCompletionEntryDetails")]
    pub fn get_completion_entry_details(this: &TypeScriptWorker, file_name: String, position: f64, entry: String, ) -> Promise;

    /**
         * Get signature help items for the item at the given file and position.
         * @returns `Promise<typescript.SignatureHelpItems | undefined>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getSignatureHelpItems")]
    pub fn get_signature_help_items(this: &TypeScriptWorker, file_name: String, position: f64, options: JsValue, ) -> Promise;

    /**
         * Get quick info for the item at the given position in the file.
         * @returns `Promise<typescript.QuickInfo | undefined>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getQuickInfoAtPosition")]
    pub fn get_quick_info_at_position(this: &TypeScriptWorker, file_name: String, position: f64, ) -> Promise;

    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getDocumentHighlights")]
    pub fn get_document_highlights(this: &TypeScriptWorker, file_name: String, position: f64, files_to_search: js_sys::Array, ) -> Promise;

    /**
         * Get the definition of the item at the given position in the file.
         * @returns `Promise<ReadonlyArray<typescript.DefinitionInfo> | undefined>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getDefinitionAtPosition")]
    pub fn get_definition_at_position(this: &TypeScriptWorker, file_name: String, position: f64, ) -> Promise;

    /**
         * Get references to the item at the given position in the file.
         * @returns `Promise<typescript.ReferenceEntry[] | undefined>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getReferencesAtPosition")]
    pub fn get_references_at_position(this: &TypeScriptWorker, file_name: String, position: f64, ) -> Promise;

    /**
         * Get outline entries for the item at the given position in the file.
         * @returns `Promise<typescript.NavigationTree | undefined>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getNavigationTree")]
    pub fn get_navigation_tree(this: &TypeScriptWorker, file_name: String, ) -> Promise;

    /**
         * Get changes which should be applied to format the given file.
         * @param options `typescript.FormatCodeOptions`
         * @returns `Promise<typescript.TextChange[]>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getFormattingEditsForDocument")]
    pub fn get_formatting_edits_for_document(this: &TypeScriptWorker, file_name: String, options: JsValue, ) -> Promise;

    /**
         * Get changes which should be applied to format the given range in the file.
         * @param options `typescript.FormatCodeOptions`
         * @returns `Promise<typescript.TextChange[]>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getFormattingEditsForRange")]
    pub fn get_formatting_edits_for_range(this: &TypeScriptWorker, file_name: String, start: f64, end: f64, options: JsValue, ) -> Promise;

    /**
         * Get formatting changes which should be applied after the given keystroke.
         * @param options `typescript.FormatCodeOptions`
         * @returns `Promise<typescript.TextChange[]>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getFormattingEditsAfterKeystroke")]
    pub fn get_formatting_edits_after_keystroke(this: &TypeScriptWorker, file_name: String, postion: f64, ch: String, options: JsValue, ) -> Promise;

    /**
         * Get other occurrences which should be updated when renaming the item at the given file and position.
         * @returns `Promise<readonly typescript.RenameLocation[] | undefined>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "findRenameLocations")]
    pub fn find_rename_locations(this: &TypeScriptWorker, file_name: String, positon: f64, find_in_strings: bool, find_in_comments: bool, provide_prefix_and_suffix_text_for_rename: bool, ) -> Promise;

    /**
         * Get edits which should be applied to rename the item at the given file and position (or a failure reason).
         * @param options `typescript.RenameInfoOptions`
         * @returns `Promise<typescript.RenameInfo>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getRenameInfo")]
    pub fn get_rename_info(this: &TypeScriptWorker, file_name: String, positon: f64, options: JsValue, ) -> Promise;

    /**
         * Get transpiled output for the given file.
         * @returns `typescript.EmitOutput`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getEmitOutput")]
    pub fn get_emit_output(this: &TypeScriptWorker, file_name: String, ) -> Promise;

    /**
         * Get possible code fixes at the given position in the file.
         * @param formatOptions `typescript.FormatCodeOptions`
         * @returns `Promise<ReadonlyArray<typescript.CodeFixAction>>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "getCodeFixesAtPosition")]
    pub fn get_code_fixes_at_position(this: &TypeScriptWorker, file_name: String, start: f64, end: f64, error_codes: js_sys::Array, format_options: JsValue, ) -> Promise;

    /**
         * Get inlay hints in the range of the file.
         * @param fileName
         * @returns `Promise<typescript.InlayHint[]>`
         */ 
    #[wasm_bindgen(method, js_class = "TypeScriptWorker", js_name = "provideInlayHints")]
    pub fn provide_inlay_hints(this: &TypeScriptWorker, file_name: String, start: f64, end: f64, ) -> Promise;


}

}
}
pub mod worker {

#[allow(unused)]
use wasm_bindgen::prelude::*;
#[allow(unused)]
use web_sys::*;
#[allow(unused)]
use js_sys::*;
#[allow(unused)]
use super::*;

#[allow(unused)]
pub type HTMLElement = HtmlElement;
#[allow(unused)]
pub type ReadonlyArray = Array;
#[allow(unused)]
pub type NonNullable = JsValue;
#[allow(unused)]
pub type Record = JsValue;
#[allow(unused)]
pub type PromiseLike = JsValue;



#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMirrorTextModel;

    #[wasm_bindgen(method, js_class = "IMirrorTextModel", js_name = "version", getter)]
    pub fn version(this: &IMirrorTextModel) -> f64;

}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IMirrorModel;

    #[wasm_bindgen(method, js_class = "IMirrorModel", js_name = "uri", getter)]
    pub fn uri(this: &IMirrorModel) -> Uri;
    #[wasm_bindgen(method, js_class = "IMirrorModel", js_name = "version", getter)]
    pub fn version(this: &IMirrorModel) -> f64;
    #[wasm_bindgen(method, js_class = "IMirrorModel", js_name = "getValue")]
    pub fn get_value(this: &IMirrorModel, ) -> String;


}


#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = js_sys::Object)]
    pub type IWorkerContext;

    /**
         * A proxy to the main thread host object.
         */ 
    #[wasm_bindgen(method, js_class = "IWorkerContext", js_name = "host", getter)]
    pub fn host(this: &IWorkerContext) -> JsValue;

    #[wasm_bindgen(method, js_class = "IWorkerContext", js_name = "host", setter)]
    pub fn set_host(this: &IWorkerContext, host: JsValue);

    /**
         * Get all available mirror models in this worker.
         */ 
    #[wasm_bindgen(method, js_class = "IWorkerContext", js_name = "getMirrorModels")]
    pub fn get_mirror_models(this: &IWorkerContext, ) -> js_sys::Array;


}

}
}
