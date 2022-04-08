//! Bindings for the `monaco` namespace.
use js_sys::{Array, Function, Object};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent, Worker};

pub mod editor;
pub mod languages;

/// Signature of the closure used by [`Environment::get_worker`]
pub type GetWorkerFn = dyn FnMut(String, String) -> Worker;
/// Signature of the closure used by [`Environment::get_worker_url`]
pub type GetWorkerUrlFn = dyn FnMut(String, String) -> String;

impl Default for Environment {
    fn default() -> Self {
        Object::new().unchecked_into()
    }
}

// You're entering generated land, tread with care.

#[cfg_attr(debug_assertions, wasm_bindgen(module = "/js/debug/editor.js"))]
#[cfg_attr(not(debug_assertions), wasm_bindgen(module = "/js/release/editor.js"))]
extern "C" {
    /// A helper that allows to emit and listen to typed events
    #[derive(Debug)]
    pub type Emitter;
    #[wasm_bindgen(constructor, js_class = "Emitter")]
    pub fn new() -> Emitter;
    /// Type: `(listener: (e: any) => any, thisArg?: any) => IDisposable`
    #[wasm_bindgen(method, js_class = "Emitter", js_name = "event", getter = event)]
    pub fn event(this: &Emitter) -> Function;
    #[wasm_bindgen(method, js_class = "Emitter", js_name = "fire")]
    pub fn fire(this: &Emitter, event: &JsValue);
    #[wasm_bindgen(method, js_class = "Emitter", js_name = "dispose")]
    pub fn dispose(this: &Emitter);

    #[derive(Debug)]
    pub type CancellationTokenSource;
    #[wasm_bindgen(constructor, js_class = "CancellationTokenSource")]
    pub fn new(parent: &CancellationToken) -> CancellationTokenSource;
    #[wasm_bindgen(method, js_class = "CancellationTokenSource", js_name = "token", getter = token)]
    pub fn token(this: &CancellationTokenSource) -> CancellationToken;
    #[wasm_bindgen(method, js_class = "CancellationTokenSource", js_name = "cancel")]
    pub fn cancel(this: &CancellationTokenSource);
    #[wasm_bindgen(method, js_class = "CancellationTokenSource", js_name = "dispose")]
    pub fn dispose(this: &CancellationTokenSource, cancel: bool);

    /// Uniform Resource Identifier (Uri) http://tools.ietf.org/html/rfc3986.
    /// This class is a simple parser which creates the basic component parts
    /// (http://tools.ietf.org/html/rfc3986#section-3) with minimal validation
    /// and encoding.
    ///
    /// ```text
    ///       foo://example.com:8042/over/there?name=ferret#nose
    ///       \_/   \______________/\_________/ \_________/ \__/
    ///        |           |            |            |        |
    ///     scheme     authority       path        query   fragment
    ///        |   _____________________|__
    ///       / \ /                        \
    ///       urn:example:animal:ferret:nose
    /// ```
    #[derive(Debug)]
    #[wasm_bindgen(extends = UriComponents)]
    pub type Uri;
    #[wasm_bindgen(js_class = "Uri", js_name = "isUri", static_method_of = Uri)]
    pub fn is_uri(thing: &JsValue) -> bool;
    /// scheme is the 'http' part of 'http://www.msft.com/some/path?query#fragment'.
    /// The part before the first colon.
    #[wasm_bindgen(method, js_class = "Uri", js_name = "scheme", getter = scheme)]
    pub fn scheme(this: &Uri) -> String;
    /// authority is the 'www.msft.com' part of 'http://www.msft.com/some/path?query#fragment'.
    /// The part between the first double slashes and the next slash.
    #[wasm_bindgen(method, js_class = "Uri", js_name = "authority", getter = authority)]
    pub fn authority(this: &Uri) -> String;
    /// path is the '/some/path' part of 'http://www.msft.com/some/path?query#fragment'.
    #[wasm_bindgen(method, js_class = "Uri", js_name = "path", getter = path)]
    pub fn path(this: &Uri) -> String;
    /// query is the 'query' part of 'http://www.msft.com/some/path?query#fragment'.
    #[wasm_bindgen(method, js_class = "Uri", js_name = "query", getter = query)]
    pub fn query(this: &Uri) -> String;
    /// fragment is the 'fragment' part of 'http://www.msft.com/some/path?query#fragment'.
    #[wasm_bindgen(method, js_class = "Uri", js_name = "fragment", getter = fragment)]
    pub fn fragment(this: &Uri) -> String;
    /// Returns a string representing the corresponding file system path of this
    /// Uri. Will handle UNC paths, normalizes windows drive letters to
    /// lower-case, and uses the platform specific path separator.
    ///
    /// * Will *not* validate the path for invalid characters and semantics.
    /// * Will *not* look at the scheme of this Uri.
    /// * The result shall *not* be used for display purposes but for accessing
    ///   a file on disk.
    ///
    ///
    /// The *difference* to `Uri#path` is the use of the platform specific
    /// separator and the handling of UNC paths. See the below sample of a
    /// file-uri with an authority (UNC path).
    ///
    /// Using `Uri#path` to read a file (using fs-apis) would not be enough
    /// because parts of the path, namely the server name, would be missing.
    /// Therefore `Uri#fsPath` exists - it's sugar to ease working with URIs
    /// that represent files on disk (`file` scheme).
    #[wasm_bindgen(method, js_class = "Uri", js_name = "fsPath", getter = fsPath)]
    pub fn fs_path(this: &Uri) -> String;
    /// # Arguments
    ///
    /// * `change` - `{ scheme?: string; authority?: string | null; path?:
    ///   string | null; query?: string | null; fragment?: string | null; }`
    #[wasm_bindgen(method, js_class = "Uri", js_name = "with")]
    pub fn with(this: &Uri, change: &Object) -> Uri;
    /// Creates a new Uri from a string, e.g. `http://www.msft.com/some/path`,
    /// `file:///usr/home`, or `scheme:with/path`.
    ///
    /// @param value A string which represents an Uri (see `Uri#toString`).
    #[wasm_bindgen(js_class = "Uri", js_name = "parse", static_method_of = Uri)]
    pub fn parse(value: &str, _strict: bool) -> Uri;
    /// Creates a new Uri from a file system path, e.g. `c:\my\files`,
    /// `/usr/home`, or `\\server\share\some\path`.
    ///
    /// The *difference* between `Uri#parse` and `Uri#file` is that the latter
    /// treats the argument as path, not as stringified-uri. E.g.
    /// `Uri.file(path)` is **not the same as** `Uri.parse('file://' + path)` because the path might contain characters that are
    /// interpreted (# and ?).
    ///
    /// @param path A file system path (see `Uri#fsPath`)
    #[wasm_bindgen(js_class = "Uri", js_name = "file", static_method_of = Uri)]
    pub fn file(path: &str) -> Uri;
    /// # Arguments
    ///
    /// * `components` - `{ scheme: string; authority?: string; path?: string;
    ///   query?: string; fragment?: string; }`
    #[wasm_bindgen(js_class = "Uri", js_name = "from", static_method_of = Uri)]
    pub fn from(components: &Object) -> Uri;
    /// Creates a string representation for this Uri. It's guaranteed that
    /// calling `Uri.parse` with the result of this function creates an Uri
    /// which is equal to this Uri.
    ///
    /// * The result shall *not* be used for display purposes but for
    ///   externalization or transport.
    /// * The result will be encoded using the percentage encoding and encoding
    ///   happens mostly
    /// ignore the scheme-specific encoding rules.
    ///
    /// @param skipEncoding Do not encode the result, default is `false`
    #[wasm_bindgen(method, js_class = "Uri", js_name = "toString")]
    pub fn to_string(this: &Uri, skip_encoding: bool) -> String;
    #[wasm_bindgen(method, js_class = "Uri", js_name = "toJSON")]
    pub fn to_json(this: &Uri) -> UriComponents;
    #[wasm_bindgen(js_class = "Uri", js_name = "revive", static_method_of = Uri)]
    pub fn revive(data: &UriComponents) -> Uri;
    #[wasm_bindgen(js_class = "Uri", js_name = "revive", static_method_of = Uri)]
    pub fn revive_option(data: Option<&UriComponents>) -> Option<Uri>;

    #[derive(Debug)]
    pub type KeyMod;
    #[wasm_bindgen(static_method_of = KeyMod, js_class = "KeyMod", js_name = "CtrlCmd", getter = CtrlCmd)]
    pub fn ctrl_cmd() -> f64;
    #[wasm_bindgen(static_method_of = KeyMod, js_class = "KeyMod", js_name = "Shift", getter = Shift)]
    pub fn shift() -> f64;
    #[wasm_bindgen(static_method_of = KeyMod, js_class = "KeyMod", js_name = "Alt", getter = Alt)]
    pub fn alt() -> f64;
    #[wasm_bindgen(static_method_of = KeyMod, js_class = "KeyMod", js_name = "WinCtrl", getter = WinCtrl)]
    pub fn win_ctrl() -> f64;
    #[wasm_bindgen(js_class = "KeyMod", js_name = "chord", static_method_of = KeyMod)]
    pub fn chord(first_part: f64, second_part: f64) -> f64;

    /// A position in the editor.
    #[derive(Debug)]
    pub type Position;
    /// line number (starts at 1)
    #[wasm_bindgen(method, js_class = "Position", js_name = "lineNumber", getter = lineNumber)]
    pub fn line_number(this: &Position) -> f64;
    /// column (the first character in a line is between column 1 and column 2)
    #[wasm_bindgen(method, js_class = "Position", js_name = "column", getter = column)]
    pub fn column(this: &Position) -> f64;
    #[wasm_bindgen(constructor, js_class = "Position")]
    pub fn new(line_number: f64, column: f64) -> Position;
    /// Create a new position from this position.
    ///
    /// @param newLineNumber new line number
    /// @param newColumn new column
    #[wasm_bindgen(method, js_class = "Position", js_name = "with")]
    pub fn with(this: &Position, new_line_number: f64, new_column: f64) -> Position;
    /// Derive a new position from this position.
    ///
    /// @param deltaLineNumber line number delta
    /// @param deltaColumn column delta
    #[wasm_bindgen(method, js_class = "Position", js_name = "delta")]
    pub fn delta(this: &Position, delta_line_number: f64, delta_column: f64) -> Position;
    /// Test if this position equals other position
    #[wasm_bindgen(method, js_class = "Position", js_name = "equals")]
    pub fn equals(this: &Position, other: &IPosition) -> bool;
    /// Test if position `a` equals position `b`
    #[wasm_bindgen(js_class = "Position", js_name = "equals", static_method_of = Position)]
    pub fn equals_static(a: Option<&IPosition>, b: Option<&IPosition>) -> bool;
    /// Test if this position is before other position.
    /// If the two positions are equal, the result will be false.
    #[wasm_bindgen(method, js_class = "Position", js_name = "isBefore")]
    pub fn is_before(this: &Position, other: &IPosition) -> bool;
    /// Test if position `a` is before position `b`.
    /// If the two positions are equal, the result will be false.
    #[wasm_bindgen(js_class = "Position", js_name = "isBefore", static_method_of = Position)]
    pub fn is_before_static(a: &IPosition, b: &IPosition) -> bool;
    /// Test if this position is before other position.
    /// If the two positions are equal, the result will be true.
    #[wasm_bindgen(method, js_class = "Position", js_name = "isBeforeOrEqual")]
    pub fn is_before_or_equal(this: &Position, other: &IPosition) -> bool;
    /// Test if position `a` is before position `b`.
    /// If the two positions are equal, the result will be true.
    #[wasm_bindgen(js_class = "Position", js_name = "isBeforeOrEqual", static_method_of = Position)]
    pub fn is_before_or_equal_static(a: &IPosition, b: &IPosition) -> bool;
    /// A function that compares positions, useful for sorting
    #[wasm_bindgen(js_class = "Position", js_name = "compare", static_method_of = Position)]
    pub fn compare(a: &IPosition, b: &IPosition) -> f64;
    /// Clone this position.
    #[wasm_bindgen(method, js_class = "Position", js_name = "clone")]
    pub fn clone(this: &Position) -> Position;
    /// Convert to a human-readable representation.
    #[wasm_bindgen(method, js_class = "Position", js_name = "toString")]
    pub fn to_string(this: &Position) -> String;
    /// Create a `Position` from an `IPosition`.
    #[wasm_bindgen(js_class = "Position", js_name = "lift", static_method_of = Position)]
    pub fn lift(pos: &IPosition) -> Position;
    /// Test if `obj` is an `IPosition`.
    #[wasm_bindgen(js_class = "Position", js_name = "isIPosition", static_method_of = Position)]
    pub fn is_iposition(obj: &JsValue) -> bool;

    /// A range in the editor. (startLineNumber,startColumn) is <=
    /// (endLineNumber,endColumn)
    #[derive(Debug)]
    pub type Range;
    /// Line number on which the range starts (starts at 1).
    #[wasm_bindgen(method, js_class = "Range", js_name = "startLineNumber", getter = startLineNumber)]
    pub fn start_line_number(this: &Range) -> f64;
    /// Column on which the range starts in line `startLineNumber` (starts at
    /// 1).
    #[wasm_bindgen(method, js_class = "Range", js_name = "startColumn", getter = startColumn)]
    pub fn start_column(this: &Range) -> f64;
    /// Line number on which the range ends.
    #[wasm_bindgen(method, js_class = "Range", js_name = "endLineNumber", getter = endLineNumber)]
    pub fn end_line_number(this: &Range) -> f64;
    /// Column on which the range ends in line `endLineNumber`.
    #[wasm_bindgen(method, js_class = "Range", js_name = "endColumn", getter = endColumn)]
    pub fn end_column(this: &Range) -> f64;
    #[wasm_bindgen(constructor, js_class = "Range")]
    pub fn new(
        start_line_number: f64,
        start_column: f64,
        end_line_number: f64,
        end_column: f64,
    ) -> Range;
    /// Test if this range is empty.
    #[wasm_bindgen(method, js_class = "Range", js_name = "isEmpty")]
    pub fn is_empty(this: &Range) -> bool;
    /// Test if `range` is empty.
    #[wasm_bindgen(js_class = "Range", js_name = "isEmpty", static_method_of = Range)]
    pub fn is_empty_static(range: &IRange) -> bool;
    /// Test if position is in this range. If the position is at the edges, will
    /// return true.
    #[wasm_bindgen(method, js_class = "Range", js_name = "containsPosition")]
    pub fn contains_position(this: &Range, position: &IPosition) -> bool;
    /// Test if `position` is in `range`. If the position is at the edges, will
    /// return true.
    #[wasm_bindgen(js_class = "Range", js_name = "containsPosition", static_method_of = Range)]
    pub fn contains_position_static(range: &IRange, position: &IPosition) -> bool;
    /// Test if range is in this range. If the range is equal to this range,
    /// will return true.
    #[wasm_bindgen(method, js_class = "Range", js_name = "containsRange")]
    pub fn contains_range(this: &Range, range: &IRange) -> bool;
    /// Test if `otherRange` is in `range`. If the ranges are equal, will return
    /// true.
    #[wasm_bindgen(js_class = "Range", js_name = "containsRange", static_method_of = Range)]
    pub fn contains_range_static(range: &IRange, other_range: &IRange) -> bool;
    /// Test if `range` is strictly in this range. `range` must start after and
    /// end before this range for the result to be true.
    #[wasm_bindgen(method, js_class = "Range", js_name = "strictContainsRange")]
    pub fn strict_contains_range(this: &Range, range: &IRange) -> bool;
    /// Test if `otherRange` is strinctly in `range` (must start after, and end
    /// before). If the ranges are equal, will return false.
    #[wasm_bindgen(js_class = "Range", js_name = "strictContainsRange", static_method_of = Range)]
    pub fn strict_contains_range_static(range: &IRange, other_range: &IRange) -> bool;
    /// A reunion of the two ranges.
    /// The smallest position will be used as the start point, and the largest
    /// one as the end point.
    #[wasm_bindgen(method, js_class = "Range", js_name = "plusRange")]
    pub fn plus_range(this: &Range, range: &IRange) -> Range;
    /// A reunion of the two ranges.
    /// The smallest position will be used as the start point, and the largest
    /// one as the end point.
    #[wasm_bindgen(js_class = "Range", js_name = "plusRange", static_method_of = Range)]
    pub fn plus_range_static(a: &IRange, b: &IRange) -> Range;
    /// A intersection of the two ranges.
    #[wasm_bindgen(method, js_class = "Range", js_name = "intersectRanges")]
    pub fn intersect_ranges(this: &Range, range: &IRange) -> Option<Range>;
    /// A intersection of the two ranges.
    #[wasm_bindgen(js_class = "Range", js_name = "intersectRanges", static_method_of = Range)]
    pub fn intersect_ranges_static(a: &IRange, b: &IRange) -> Option<Range>;
    /// Test if this range equals other.
    #[wasm_bindgen(method, js_class = "Range", js_name = "equalsRange")]
    pub fn equals_range(this: &Range, other: Option<&IRange>) -> bool;
    /// Test if range `a` equals `b`.
    #[wasm_bindgen(js_class = "Range", js_name = "equalsRange", static_method_of = Range)]
    pub fn equals_range_static(a: Option<&IRange>, b: Option<&IRange>) -> bool;
    /// Return the end position (which will be after or equal to the start
    /// position)
    #[wasm_bindgen(method, js_class = "Range", js_name = "getEndPosition")]
    pub fn get_end_position(this: &Range) -> Position;
    /// Return the start position (which will be before or equal to the end
    /// position)
    #[wasm_bindgen(method, js_class = "Range", js_name = "getStartPosition")]
    pub fn get_start_position(this: &Range) -> Position;
    /// Transform to a user presentable string representation.
    #[wasm_bindgen(method, js_class = "Range", js_name = "toString")]
    pub fn to_string(this: &Range) -> String;
    /// Create a new range using this range's start position, and using
    /// endLineNumber and endColumn as the end position.
    #[wasm_bindgen(method, js_class = "Range", js_name = "setEndPosition")]
    pub fn set_end_position(this: &Range, end_line_number: f64, end_column: f64) -> Range;
    /// Create a new range using this range's end position, and using
    /// startLineNumber and startColumn as the start position.
    #[wasm_bindgen(method, js_class = "Range", js_name = "setStartPosition")]
    pub fn set_start_position(this: &Range, start_line_number: f64, start_column: f64) -> Range;
    /// Create a new empty range using this range's start position.
    #[wasm_bindgen(method, js_class = "Range", js_name = "collapseToStart")]
    pub fn collapse_to_start(this: &Range) -> Range;
    /// Create a new empty range using this range's start position.
    #[wasm_bindgen(js_class = "Range", js_name = "collapseToStart", static_method_of = Range)]
    pub fn collapse_to_start_static(range: &IRange) -> Range;
    #[wasm_bindgen(js_class = "Range", js_name = "fromPositions", static_method_of = Range)]
    pub fn from_positions(start: &IPosition, end: &IPosition) -> Range;
    #[wasm_bindgen(js_class = "Range", js_name = "lift", static_method_of = Range)]
    pub fn lift(range: &IRange) -> Range;
    /// Test if `obj` is an `IRange`.
    #[wasm_bindgen(js_class = "Range", js_name = "isIRange", static_method_of = Range)]
    pub fn is_irange(obj: &JsValue) -> bool;
    /// Test if the two ranges are touching in any way.
    #[wasm_bindgen(js_class = "Range", js_name = "areIntersectingOrTouching", static_method_of = Range)]
    pub fn are_intersecting_or_touching(a: &IRange, b: &IRange) -> bool;
    /// Test if the two ranges are intersecting. If the ranges are touching it
    /// returns true.
    #[wasm_bindgen(js_class = "Range", js_name = "areIntersecting", static_method_of = Range)]
    pub fn are_intersecting(a: &IRange, b: &IRange) -> bool;
    /// A function that compares ranges, useful for sorting ranges
    /// It will first compare ranges on the startPosition and then on the
    /// endPosition
    #[wasm_bindgen(js_class = "Range", js_name = "compareRangesUsingStarts", static_method_of = Range)]
    pub fn compare_ranges_using_starts(a: Option<&IRange>, b: Option<&IRange>) -> f64;
    /// A function that compares ranges, useful for sorting ranges
    /// It will first compare ranges on the endPosition and then on the
    /// startPosition
    #[wasm_bindgen(js_class = "Range", js_name = "compareRangesUsingEnds", static_method_of = Range)]
    pub fn compare_ranges_using_ends(a: &IRange, b: &IRange) -> f64;
    /// Test if the range spans multiple lines.
    #[wasm_bindgen(js_class = "Range", js_name = "spansMultipleLines", static_method_of = Range)]
    pub fn spans_multiple_lines(range: &IRange) -> bool;

    /// A selection in the editor.
    /// The selection is a range that has an orientation.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Range)]
    pub type Selection;
    /// The line number on which the selection has started.
    #[wasm_bindgen(method, js_class = "Selection", js_name = "selectionStartLineNumber", getter = selectionStartLineNumber)]
    pub fn selection_start_line_number(this: &Selection) -> f64;
    /// The column on `selectionStartLineNumber` where the selection has
    /// started.
    #[wasm_bindgen(method, js_class = "Selection", js_name = "selectionStartColumn", getter = selectionStartColumn)]
    pub fn selection_start_column(this: &Selection) -> f64;
    /// The line number on which the selection has ended.
    #[wasm_bindgen(method, js_class = "Selection", js_name = "positionLineNumber", getter = positionLineNumber)]
    pub fn position_line_number(this: &Selection) -> f64;
    /// The column on `positionLineNumber` where the selection has ended.
    #[wasm_bindgen(method, js_class = "Selection", js_name = "positionColumn", getter = positionColumn)]
    pub fn position_column(this: &Selection) -> f64;
    #[wasm_bindgen(constructor, js_class = "Selection")]
    pub fn new(
        selection_start_line_number: f64,
        selection_start_column: f64,
        position_line_number: f64,
        position_column: f64,
    ) -> Selection;
    /// Transform to a human-readable representation.
    #[wasm_bindgen(method, js_class = "Selection", js_name = "toString")]
    pub fn to_string(this: &Selection) -> String;
    /// Test if equals other selection.
    #[wasm_bindgen(method, js_class = "Selection", js_name = "equalsSelection")]
    pub fn equals_selection(this: &Selection, other: &ISelection) -> bool;
    /// Test if the two selections are equal.
    #[wasm_bindgen(js_class = "Selection", js_name = "selectionsEqual", static_method_of = Selection)]
    pub fn selections_equal(a: &ISelection, b: &ISelection) -> bool;
    /// Get directions (LTR or RTL).
    #[wasm_bindgen(method, js_class = "Selection", js_name = "getDirection")]
    pub fn get_direction(this: &Selection) -> SelectionDirection;
    /// Create a new selection with a different `positionLineNumber` and
    /// `positionColumn`.
    #[wasm_bindgen(method, js_class = "Selection", js_name = "setEndPosition")]
    pub fn set_end_position(this: &Selection, end_line_number: f64, end_column: f64) -> Selection;
    /// Get the position at `positionLineNumber` and `positionColumn`.
    #[wasm_bindgen(method, js_class = "Selection", js_name = "getPosition")]
    pub fn get_position(this: &Selection) -> Position;
    /// Create a new selection with a different `selectionStartLineNumber` and
    /// `selectionStartColumn`.
    #[wasm_bindgen(method, js_class = "Selection", js_name = "setStartPosition")]
    pub fn set_start_position(
        this: &Selection,
        start_line_number: f64,
        start_column: f64,
    ) -> Selection;
    /// Create a `Selection` from one or two positions
    #[wasm_bindgen(js_class = "Selection", js_name = "fromPositions", static_method_of = Selection)]
    pub fn from_positions(start: &IPosition, end: &IPosition) -> Selection;
    /// Create a `Selection` from an `ISelection`.
    #[wasm_bindgen(js_class = "Selection", js_name = "liftSelection", static_method_of = Selection)]
    pub fn lift_selection(sel: &ISelection) -> Selection;
    /// `a` equals `b`.
    ///
    /// # Arguments
    ///
    /// * `a` - `ISelection[]`
    /// * `b` - `ISelection[]`
    #[wasm_bindgen(js_class = "Selection", js_name = "selectionsArrEqual", static_method_of = Selection)]
    pub fn selections_arr_equal(a: &Array, b: &Array) -> bool;
    /// Test if `obj` is an `ISelection`.
    #[wasm_bindgen(js_class = "Selection", js_name = "isISelection", static_method_of = Selection)]
    pub fn is_iselection(obj: &JsValue) -> bool;
    /// Create with a direction.
    #[wasm_bindgen(js_class = "Selection", js_name = "createWithDirection", static_method_of = Selection)]
    pub fn create_with_direction(
        start_line_number: f64,
        start_column: f64,
        end_line_number: f64,
        end_column: f64,
        direction: SelectionDirection,
    ) -> Selection;

    #[derive(Debug)]
    pub type Token;
    #[wasm_bindgen(method, js_class = "Token", js_name = "offset", getter = offset)]
    pub fn offset(this: &Token) -> f64;
    #[wasm_bindgen(method, js_class = "Token", js_name = "type", getter = type)]
    pub fn type_(this: &Token) -> String;
    #[wasm_bindgen(method, js_class = "Token", js_name = "language", getter = language)]
    pub fn language(this: &Token) -> String;
    #[wasm_bindgen(constructor, js_class = "Token")]
    pub fn new(offset: f64, type_: &str, language: &str) -> Token;
    #[wasm_bindgen(method, js_class = "Token", js_name = "toString")]
    pub fn to_string(this: &Token) -> String;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type Environment;
    #[wasm_bindgen(method, js_class = "Environment", js_name = "baseUrl", getter = baseUrl)]
    pub fn base_url(this: &Environment) -> Option<String>;
    /// Set the `baseUrl` property.
    #[wasm_bindgen(method, js_class = "Environment", js_name = "baseUrl", setter = baseUrl)]
    pub fn set_base_url(this: &Environment, val: Option<&str>);
    /// Type: `(workerId: string, label: string) => Worker`
    #[wasm_bindgen(method, js_class = "Environment", js_name = "getWorker", getter = getWorker)]
    pub fn get_worker(this: &Environment) -> Option<Function>;
    /// Set the `getWorker` property.
    #[wasm_bindgen(method, js_class = "Environment", js_name = "getWorker", setter = getWorker)]
    pub fn set_get_worker(this: &Environment, val: Option<&Function>);
    /// Type: `(workerId: string, label: string) => string`
    #[wasm_bindgen(method, js_class = "Environment", js_name = "getWorkerUrl", getter = getWorkerUrl)]
    pub fn get_worker_url(this: &Environment) -> Option<Function>;
    /// Set the `getWorkerUrl` property.
    #[wasm_bindgen(method, js_class = "Environment", js_name = "getWorkerUrl", setter = getWorkerUrl)]
    pub fn set_get_worker_url(this: &Environment, val: Option<&Function>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IDisposable;
    #[wasm_bindgen(method, js_class = "IDisposable", js_name = "dispose")]
    pub fn dispose(this: &IDisposable);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type CancellationToken;
    #[wasm_bindgen(method, js_class = "CancellationToken", js_name = "isCancellationRequested", getter = isCancellationRequested)]
    pub fn is_cancellation_requested(this: &CancellationToken) -> bool;
    /// An event emitted when cancellation is requested
    /// @event
    ///
    /// Type: `(
    ///       listener: (e: any) => any,
    ///       thisArg?: any
    ///     ) => IDisposable`
    #[wasm_bindgen(method, js_class = "CancellationToken", js_name = "onCancellationRequested", getter = onCancellationRequested)]
    pub fn on_cancellation_requested(this: &CancellationToken) -> Function;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type UriComponents;
    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "scheme", getter = scheme)]
    pub fn scheme(this: &UriComponents) -> String;
    /// Set the `scheme` property.
    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "scheme", setter = scheme)]
    pub fn set_scheme(this: &UriComponents, val: &str);
    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "authority", getter = authority)]
    pub fn authority(this: &UriComponents) -> String;
    /// Set the `authority` property.
    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "authority", setter = authority)]
    pub fn set_authority(this: &UriComponents, val: &str);
    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "path", getter = path)]
    pub fn path(this: &UriComponents) -> String;
    /// Set the `path` property.
    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "path", setter = path)]
    pub fn set_path(this: &UriComponents, val: &str);
    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "query", getter = query)]
    pub fn query(this: &UriComponents) -> String;
    /// Set the `query` property.
    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "query", setter = query)]
    pub fn set_query(this: &UriComponents, val: &str);
    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "fragment", getter = fragment)]
    pub fn fragment(this: &UriComponents) -> String;
    /// Set the `fragment` property.
    #[wasm_bindgen(method, js_class = "UriComponents", js_name = "fragment", setter = fragment)]
    pub fn set_fragment(this: &UriComponents, val: &str);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IMarkdownString;
    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "value", getter = value)]
    pub fn value(this: &IMarkdownString) -> String;
    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "isTrusted", getter = isTrusted)]
    pub fn is_trusted(this: &IMarkdownString) -> Option<bool>;
    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "supportThemeIcons", getter = supportThemeIcons)]
    pub fn support_theme_icons(this: &IMarkdownString) -> Option<bool>;
    /// Type: `{ [href: string]: UriComponents }`
    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "uris", getter = uris)]
    pub fn uris(this: &IMarkdownString) -> Option<Object>;
    /// Set the `uris` property.
    #[wasm_bindgen(method, js_class = "IMarkdownString", js_name = "uris", setter = uris)]
    pub fn set_uris(this: &IMarkdownString, val: Option<&Object>);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IKeyboardEvent;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "browserEvent", getter = browserEvent)]
    pub fn browser_event(this: &IKeyboardEvent) -> KeyboardEvent;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "target", getter = target)]
    pub fn target(this: &IKeyboardEvent) -> HtmlElement;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "ctrlKey", getter = ctrlKey)]
    pub fn ctrl_key(this: &IKeyboardEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "shiftKey", getter = shiftKey)]
    pub fn shift_key(this: &IKeyboardEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "altKey", getter = altKey)]
    pub fn alt_key(this: &IKeyboardEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "metaKey", getter = metaKey)]
    pub fn meta_key(this: &IKeyboardEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "keyCode", getter = keyCode)]
    pub fn key_code(this: &IKeyboardEvent) -> KeyCode;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "code", getter = code)]
    pub fn code(this: &IKeyboardEvent) -> String;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "equals")]
    pub fn equals(this: &IKeyboardEvent, keybinding: f64) -> bool;
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "preventDefault")]
    pub fn prevent_default(this: &IKeyboardEvent);
    #[wasm_bindgen(method, js_class = "IKeyboardEvent", js_name = "stopPropagation")]
    pub fn stop_propagation(this: &IKeyboardEvent);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IMouseEvent;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "browserEvent", getter = browserEvent)]
    pub fn browser_event(this: &IMouseEvent) -> MouseEvent;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "leftButton", getter = leftButton)]
    pub fn left_button(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "middleButton", getter = middleButton)]
    pub fn middle_button(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "rightButton", getter = rightButton)]
    pub fn right_button(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "buttons", getter = buttons)]
    pub fn buttons(this: &IMouseEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "target", getter = target)]
    pub fn target(this: &IMouseEvent) -> HtmlElement;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "detail", getter = detail)]
    pub fn detail(this: &IMouseEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "posx", getter = posx)]
    pub fn posx(this: &IMouseEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "posy", getter = posy)]
    pub fn posy(this: &IMouseEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "ctrlKey", getter = ctrlKey)]
    pub fn ctrl_key(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "shiftKey", getter = shiftKey)]
    pub fn shift_key(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "altKey", getter = altKey)]
    pub fn alt_key(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "metaKey", getter = metaKey)]
    pub fn meta_key(this: &IMouseEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "timestamp", getter = timestamp)]
    pub fn timestamp(this: &IMouseEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "preventDefault")]
    pub fn prevent_default(this: &IMouseEvent);
    #[wasm_bindgen(method, js_class = "IMouseEvent", js_name = "stopPropagation")]
    pub fn stop_propagation(this: &IMouseEvent);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IScrollEvent;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollTop", getter = scrollTop)]
    pub fn scroll_top(this: &IScrollEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollLeft", getter = scrollLeft)]
    pub fn scroll_left(this: &IScrollEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollWidth", getter = scrollWidth)]
    pub fn scroll_width(this: &IScrollEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollHeight", getter = scrollHeight)]
    pub fn scroll_height(this: &IScrollEvent) -> f64;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollTopChanged", getter = scrollTopChanged)]
    pub fn scroll_top_changed(this: &IScrollEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollLeftChanged", getter = scrollLeftChanged)]
    pub fn scroll_left_changed(this: &IScrollEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollWidthChanged", getter = scrollWidthChanged)]
    pub fn scroll_width_changed(this: &IScrollEvent) -> bool;
    #[wasm_bindgen(method, js_class = "IScrollEvent", js_name = "scrollHeightChanged", getter = scrollHeightChanged)]
    pub fn scroll_height_changed(this: &IScrollEvent) -> bool;
}

#[wasm_bindgen]
extern "C" {
    /// A position in the editor. This interface is suitable for serialization.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IPosition;
    /// line number (starts at 1)
    #[wasm_bindgen(method, js_class = "IPosition", js_name = "lineNumber", getter = lineNumber)]
    pub fn line_number(this: &IPosition) -> f64;
    /// column (the first character in a line is between column 1 and column 2)
    #[wasm_bindgen(method, js_class = "IPosition", js_name = "column", getter = column)]
    pub fn column(this: &IPosition) -> f64;
}

#[wasm_bindgen]
extern "C" {
    /// A range in the editor. This interface is suitable for serialization.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type IRange;
    /// Line number on which the range starts (starts at 1).
    #[wasm_bindgen(method, js_class = "IRange", js_name = "startLineNumber", getter = startLineNumber)]
    pub fn start_line_number(this: &IRange) -> f64;
    /// Column on which the range starts in line `startLineNumber` (starts at
    /// 1).
    #[wasm_bindgen(method, js_class = "IRange", js_name = "startColumn", getter = startColumn)]
    pub fn start_column(this: &IRange) -> f64;
    /// Line number on which the range ends.
    #[wasm_bindgen(method, js_class = "IRange", js_name = "endLineNumber", getter = endLineNumber)]
    pub fn end_line_number(this: &IRange) -> f64;
    /// Column on which the range ends in line `endLineNumber`.
    #[wasm_bindgen(method, js_class = "IRange", js_name = "endColumn", getter = endColumn)]
    pub fn end_column(this: &IRange) -> f64;
}

#[wasm_bindgen]
extern "C" {
    /// A selection in the editor.
    /// The selection is a range that has an orientation.
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ISelection;
    /// The line number on which the selection has started.
    #[wasm_bindgen(method, js_class = "ISelection", js_name = "selectionStartLineNumber", getter = selectionStartLineNumber)]
    pub fn selection_start_line_number(this: &ISelection) -> f64;
    /// The column on `selectionStartLineNumber` where the selection has
    /// started.
    #[wasm_bindgen(method, js_class = "ISelection", js_name = "selectionStartColumn", getter = selectionStartColumn)]
    pub fn selection_start_column(this: &ISelection) -> f64;
    /// The line number on which the selection has ended.
    #[wasm_bindgen(method, js_class = "ISelection", js_name = "positionLineNumber", getter = positionLineNumber)]
    pub fn position_line_number(this: &ISelection) -> f64;
    /// The column on `positionLineNumber` where the selection has ended.
    #[wasm_bindgen(method, js_class = "ISelection", js_name = "positionColumn", getter = positionColumn)]
    pub fn position_column(this: &ISelection) -> f64;
}

int_enum! {
    pub enum MarkerTag {
        Unnecessary = 1,
        Deprecated = 2,
    }
}

int_enum! {
    pub enum MarkerSeverity {
        Hint = 1,
        Info = 2,
        Warning = 4,
        Error = 8,
    }
}

int_enum! {
    pub enum KeyCode {
        /// Placed first to cover the 0 value of the enum.
        Unknown = 0,
        Backspace = 1,
        Tab = 2,
        Enter = 3,
        Shift = 4,
        Ctrl = 5,
        Alt = 6,
        Pausebreak = 7,
        Capslock = 8,
        Escape = 9,
        Space = 10,
        Pageup = 11,
        Pagedown = 12,
        End = 13,
        Home = 14,
        Leftarrow = 15,
        Uparrow = 16,
        Rightarrow = 17,
        Downarrow = 18,
        Insert = 19,
        Delete = 20,
        Key0 = 21,
        Key1 = 22,
        Key2 = 23,
        Key3 = 24,
        Key4 = 25,
        Key5 = 26,
        Key6 = 27,
        Key7 = 28,
        Key8 = 29,
        Key9 = 30,
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
        Contextmenu = 58,
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
        Numlock = 78,
        Scrolllock = 79,
        /// Used for miscellaneous characters; it can vary by keyboard.
        /// For the US standard keyboard, the ';:' key
        UsSemicolon = 80,
        /// For any country/region, the '+' key
        /// For the US standard keyboard, the '=+' key
        UsEqual = 81,
        /// For any country/region, the ',' key
        /// For the US standard keyboard, the ',<' key
        UsComma = 82,
        /// For any country/region, the '-' key
        /// For the US standard keyboard, the '-_' key
        UsMinus = 83,
        /// For any country/region, the '.' key
        /// For the US standard keyboard, the '.>' key
        UsDot = 84,
        /// Used for miscellaneous characters; it can vary by keyboard.
        /// For the US standard keyboard, the '/?' key
        UsSlash = 85,
        /// Used for miscellaneous characters; it can vary by keyboard.
        /// For the US standard keyboard, the '`~' key
        UsBacktick = 86,
        /// Used for miscellaneous characters; it can vary by keyboard.
        /// For the US standard keyboard, the '[{' key
        UsOpenSquareBracket = 87,
        /// Used for miscellaneous characters; it can vary by keyboard.
        /// For the US standard keyboard, the '\|' key
        UsBackslash = 88,
        /// Used for miscellaneous characters; it can vary by keyboard.
        /// For the US standard keyboard, the ']}' key
        UsCloseSquareBracket = 89,
        /// Used for miscellaneous characters; it can vary by keyboard.
        /// For the US standard keyboard, the ''"' key
        UsQuote = 90,
        /// Used for miscellaneous characters; it can vary by keyboard.
        Oem8 = 91,
        /// Either the angle bracket key or the backslash key on the RT 102-key keyboard.
        Oem102 = 92,
        Numpad0 = 93,
        Numpad1 = 94,
        Numpad2 = 95,
        Numpad3 = 96,
        Numpad4 = 97,
        Numpad5 = 98,
        Numpad6 = 99,
        Numpad7 = 100,
        Numpad8 = 101,
        Numpad9 = 102,
        NumpadMultiply = 103,
        NumpadAdd = 104,
        NumpadSeparator = 105,
        NumpadSubtract = 106,
        NumpadDecimal = 107,
        NumpadDivide = 108,
        /// Cover all key codes when IME is processing input.
        KeyInComposition = 109,
        AbntC1 = 110,
        AbntC2 = 111,
        /// Placed last to cover the length of the enum.
        /// Please do not depend on this value!
        MaxValue = 112,
    }
}

int_enum! {
    pub enum SelectionDirection {
        /// The selection starts above where it ends.
        Ltr = 0,
        /// The selection starts below where it ends.
        Rtl = 1,
    }
}
