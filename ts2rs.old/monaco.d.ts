declare namespace monaco {
  export interface Environment {
    baseUrl?: string;
    getWorker?: (workerId: string, label: string) => Worker;
    getWorkerUrl?: (workerId: string, label: string) => string;
  }

  export interface IDisposable {
    dispose(): void;
  }

  /**
   * A helper that allows to emit and listen to typed events
   */
  export class Emitter {
    constructor();
    readonly event: (listener: (e: any) => any, thisArg?: any) => IDisposable;
    fire(event: any): void;
    dispose(): void;
  }

  export enum MarkerTag {
    Unnecessary = 1,
    Deprecated = 2,
  }

  export enum MarkerSeverity {
    Hint = 1,
    Info = 2,
    Warning = 4,
    Error = 8,
  }

  export class CancellationTokenSource {
    constructor(parent?: CancellationToken);
    get token(): CancellationToken;
    cancel(): void;
    dispose(cancel?: boolean): void;
  }

  export interface CancellationToken {
    readonly isCancellationRequested: boolean;
    /**
     * An event emitted when cancellation is requested
     * @event
     */
    readonly onCancellationRequested: (
      listener: (e: any) => any,
      thisArg?: any
    ) => IDisposable;
  }
  /**
   * Uniform Resource Identifier (Uri) http://tools.ietf.org/html/rfc3986.
   * This class is a simple parser which creates the basic component parts
   * (http://tools.ietf.org/html/rfc3986#section-3) with minimal validation
   * and encoding.
   *
   *       foo://example.com:8042/over/there?name=ferret#nose
   *       \_/   \______________/\_________/ \_________/ \__/
   *        |           |            |            |        |
   *     scheme     authority       path        query   fragment
   *        |   _____________________|__
   *       / \ /                        \
   *       urn:example:animal:ferret:nose
   */
  export class Uri implements UriComponents {
    static isUri(thing: any): thing is Uri;
    /**
     * scheme is the 'http' part of 'http://www.msft.com/some/path?query#fragment'.
     * The part before the first colon.
     */
    readonly scheme: string;
    /**
     * authority is the 'www.msft.com' part of 'http://www.msft.com/some/path?query#fragment'.
     * The part between the first double slashes and the next slash.
     */
    readonly authority: string;
    /**
     * path is the '/some/path' part of 'http://www.msft.com/some/path?query#fragment'.
     */
    readonly path: string;
    /**
     * query is the 'query' part of 'http://www.msft.com/some/path?query#fragment'.
     */
    readonly query: string;
    /**
     * fragment is the 'fragment' part of 'http://www.msft.com/some/path?query#fragment'.
     */
    readonly fragment: string;
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
     * Using `Uri#path` to read a file (using fs-apis) would not be enough because parts of the path,
     * namely the server name, would be missing. Therefore `Uri#fsPath` exists - it's sugar to ease working
     * with URIs that represent files on disk (`file` scheme).
     */
    get fsPath(): string;
    with(change: {
      scheme?: string;
      authority?: string | null;
      path?: string | null;
      query?: string | null;
      fragment?: string | null;
    }): Uri;
    /**
     * Creates a new Uri from a string, e.g. `http://www.msft.com/some/path`,
     * `file:///usr/home`, or `scheme:with/path`.
     *
     * @param value A string which represents an Uri (see `Uri#toString`).
     */
    static parse(value: string, _strict?: boolean): Uri;
    /**
     * Creates a new Uri from a file system path, e.g. `c:\my\files`,
     * `/usr/home`, or `\\server\share\some\path`.
     *
     * The *difference* between `Uri#parse` and `Uri#file` is that the latter treats the argument
     * as path, not as stringified-uri. E.g. `Uri.file(path)` is **not the same as**
     * `Uri.parse('file://' + path)` because the path might contain characters that are
     * interpreted (# and ?).
     *
     * @param path A file system path (see `Uri#fsPath`)
     */
    static file(path: string): Uri;
    static from(components: {
      scheme: string;
      authority?: string;
      path?: string;
      query?: string;
      fragment?: string;
    }): Uri;
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
    toString(skipEncoding?: boolean): string;
    toJSON(): UriComponents;
    static revive(data: UriComponents | Uri): Uri;
    static revive(data: UriComponents | Uri | undefined): Uri | undefined;
    static revive(data: UriComponents | Uri | null): Uri | null;
    static revive(
      data: UriComponents | Uri | undefined | null
    ): Uri | undefined | null;
  }

  export interface UriComponents {
    scheme: string;
    authority: string;
    path: string;
    query: string;
    fragment: string;
  }

  /**
   * Virtual Key Codes, the value does not hold any inherent meaning.
   * Inspired somewhat from https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx
   * But these are "more general", as they should work across browsers & OS`s.
   */
  export enum KeyCode {
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
    KEY_0 = 21,
    KEY_1 = 22,
    KEY_2 = 23,
    KEY_3 = 24,
    KEY_4 = 25,
    KEY_5 = 26,
    KEY_6 = 27,
    KEY_7 = 28,
    KEY_8 = 29,
    KEY_9 = 30,
    KEY_A = 31,
    KEY_B = 32,
    KEY_C = 33,
    KEY_D = 34,
    KEY_E = 35,
    KEY_F = 36,
    KEY_G = 37,
    KEY_H = 38,
    KEY_I = 39,
    KEY_J = 40,
    KEY_K = 41,
    KEY_L = 42,
    KEY_M = 43,
    KEY_N = 44,
    KEY_O = 45,
    KEY_P = 46,
    KEY_Q = 47,
    KEY_R = 48,
    KEY_S = 49,
    KEY_T = 50,
    KEY_U = 51,
    KEY_V = 52,
    KEY_W = 53,
    KEY_X = 54,
    KEY_Y = 55,
    KEY_Z = 56,
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
    NumLock = 78,
    ScrollLock = 79,
    /**
     * Used for miscellaneous characters; it can vary by keyboard.
     * For the US standard keyboard, the ';:' key
     */
    US_SEMICOLON = 80,
    /**
     * For any country/region, the '+' key
     * For the US standard keyboard, the '=+' key
     */
    US_EQUAL = 81,
    /**
     * For any country/region, the ',' key
     * For the US standard keyboard, the ',<' key
     */
    US_COMMA = 82,
    /**
     * For any country/region, the '-' key
     * For the US standard keyboard, the '-_' key
     */
    US_MINUS = 83,
    /**
     * For any country/region, the '.' key
     * For the US standard keyboard, the '.>' key
     */
    US_DOT = 84,
    /**
     * Used for miscellaneous characters; it can vary by keyboard.
     * For the US standard keyboard, the '/?' key
     */
    US_SLASH = 85,
    /**
     * Used for miscellaneous characters; it can vary by keyboard.
     * For the US standard keyboard, the '`~' key
     */
    US_BACKTICK = 86,
    /**
     * Used for miscellaneous characters; it can vary by keyboard.
     * For the US standard keyboard, the '[{' key
     */
    US_OPEN_SQUARE_BRACKET = 87,
    /**
     * Used for miscellaneous characters; it can vary by keyboard.
     * For the US standard keyboard, the '\|' key
     */
    US_BACKSLASH = 88,
    /**
     * Used for miscellaneous characters; it can vary by keyboard.
     * For the US standard keyboard, the ']}' key
     */
    US_CLOSE_SQUARE_BRACKET = 89,
    /**
     * Used for miscellaneous characters; it can vary by keyboard.
     * For the US standard keyboard, the ''"' key
     */
    US_QUOTE = 90,
    /**
     * Used for miscellaneous characters; it can vary by keyboard.
     */
    OEM_8 = 91,
    /**
     * Either the angle bracket key or the backslash key on the RT 102-key keyboard.
     */
    OEM_102 = 92,
    NUMPAD_0 = 93,
    NUMPAD_1 = 94,
    NUMPAD_2 = 95,
    NUMPAD_3 = 96,
    NUMPAD_4 = 97,
    NUMPAD_5 = 98,
    NUMPAD_6 = 99,
    NUMPAD_7 = 100,
    NUMPAD_8 = 101,
    NUMPAD_9 = 102,
    NUMPAD_MULTIPLY = 103,
    NUMPAD_ADD = 104,
    NUMPAD_SEPARATOR = 105,
    NUMPAD_SUBTRACT = 106,
    NUMPAD_DECIMAL = 107,
    NUMPAD_DIVIDE = 108,
    /**
     * Cover all key codes when IME is processing input.
     */
    KEY_IN_COMPOSITION = 109,
    ABNT_C1 = 110,
    ABNT_C2 = 111,
    /**
     * Placed last to cover the length of the enum.
     * Please do not depend on this value!
     */
    MAX_VALUE = 112,
  }

  export class KeyMod {
    static readonly CtrlCmd: number;
    static readonly Shift: number;
    static readonly Alt: number;
    static readonly WinCtrl: number;
    static chord(firstPart: number, secondPart: number): number;
  }

  export interface IMarkdownString {
    readonly value: string;
    readonly isTrusted?: boolean;
    readonly supportThemeIcons?: boolean;
    uris?: { [href: string]: UriComponents };
  }

  export interface IKeyboardEvent {
    readonly _standardKeyboardEventBrand: true;
    readonly browserEvent: KeyboardEvent;
    readonly target: HTMLElement;
    readonly ctrlKey: boolean;
    readonly shiftKey: boolean;
    readonly altKey: boolean;
    readonly metaKey: boolean;
    readonly keyCode: KeyCode;
    readonly code: string;
    equals(keybinding: number): boolean;
    preventDefault(): void;
    stopPropagation(): void;
  }
  export interface IMouseEvent {
    readonly browserEvent: MouseEvent;
    readonly leftButton: boolean;
    readonly middleButton: boolean;
    readonly rightButton: boolean;
    readonly buttons: number;
    readonly target: HTMLElement;
    readonly detail: number;
    readonly posx: number;
    readonly posy: number;
    readonly ctrlKey: boolean;
    readonly shiftKey: boolean;
    readonly altKey: boolean;
    readonly metaKey: boolean;
    readonly timestamp: number;
    preventDefault(): void;
    stopPropagation(): void;
  }

  export interface IScrollEvent {
    readonly scrollTop: number;
    readonly scrollLeft: number;
    readonly scrollWidth: number;
    readonly scrollHeight: number;
    readonly scrollTopChanged: boolean;
    readonly scrollLeftChanged: boolean;
    readonly scrollWidthChanged: boolean;
    readonly scrollHeightChanged: boolean;
  }
  /**
   * A position in the editor. This interface is suitable for serialization.
   */
  export interface IPosition {
    /**
     * line number (starts at 1)
     */
    readonly lineNumber: number;
    /**
     * column (the first character in a line is between column 1 and column 2)
     */
    readonly column: number;
  }

  /**
   * A position in the editor.
   */
  export class Position {
    /**
     * line number (starts at 1)
     */
    readonly lineNumber: number;
    /**
     * column (the first character in a line is between column 1 and column 2)
     */
    readonly column: number;
    constructor(lineNumber: number, column: number);
    /**
     * Create a new position from this position.
     *
     * @param newLineNumber new line number
     * @param newColumn new column
     */
    with(newLineNumber?: number, newColumn?: number): Position;
    /**
     * Derive a new position from this position.
     *
     * @param deltaLineNumber line number delta
     * @param deltaColumn column delta
     */
    delta(deltaLineNumber?: number, deltaColumn?: number): Position;
    /**
     * Test if this position equals other position
     */
    equals(other: IPosition): boolean;
    /**
     * Test if position `a` equals position `b`
     */
    static equals(a: IPosition | null, b: IPosition | null): boolean;
    /**
     * Test if this position is before other position.
     * If the two positions are equal, the result will be false.
     */
    isBefore(other: IPosition): boolean;
    /**
     * Test if position `a` is before position `b`.
     * If the two positions are equal, the result will be false.
     */
    static isBefore(a: IPosition, b: IPosition): boolean;
    /**
     * Test if this position is before other position.
     * If the two positions are equal, the result will be true.
     */
    isBeforeOrEqual(other: IPosition): boolean;
    /**
     * Test if position `a` is before position `b`.
     * If the two positions are equal, the result will be true.
     */
    static isBeforeOrEqual(a: IPosition, b: IPosition): boolean;
    /**
     * A function that compares positions, useful for sorting
     */
    static compare(a: IPosition, b: IPosition): number;
    /**
     * Clone this position.
     */
    clone(): Position;
    /**
     * Convert to a human-readable representation.
     */
    toString(): string;
    /**
     * Create a `Position` from an `IPosition`.
     */
    static lift(pos: IPosition): Position;
    /**
     * Test if `obj` is an `IPosition`.
     */
    static isIPosition(obj: any): obj is IPosition;
  }

  /**
   * A range in the editor. This interface is suitable for serialization.
   */
  export interface IRange {
    /**
     * Line number on which the range starts (starts at 1).
     */
    readonly startLineNumber: number;
    /**
     * Column on which the range starts in line `startLineNumber` (starts at 1).
     */
    readonly startColumn: number;
    /**
     * Line number on which the range ends.
     */
    readonly endLineNumber: number;
    /**
     * Column on which the range ends in line `endLineNumber`.
     */
    readonly endColumn: number;
  }

  /**
   * A range in the editor. (startLineNumber,startColumn) is <= (endLineNumber,endColumn)
   */
  export class Range {
    /**
     * Line number on which the range starts (starts at 1).
     */
    readonly startLineNumber: number;
    /**
     * Column on which the range starts in line `startLineNumber` (starts at 1).
     */
    readonly startColumn: number;
    /**
     * Line number on which the range ends.
     */
    readonly endLineNumber: number;
    /**
     * Column on which the range ends in line `endLineNumber`.
     */
    readonly endColumn: number;
    constructor(
      startLineNumber: number,
      startColumn: number,
      endLineNumber: number,
      endColumn: number
    );
    /**
     * Test if this range is empty.
     */
    isEmpty(): boolean;
    /**
     * Test if `range` is empty.
     */
    static isEmpty(range: IRange): boolean;
    /**
     * Test if position is in this range. If the position is at the edges, will return true.
     */
    containsPosition(position: IPosition): boolean;
    /**
     * Test if `position` is in `range`. If the position is at the edges, will return true.
     */
    static containsPosition(range: IRange, position: IPosition): boolean;
    /**
     * Test if range is in this range. If the range is equal to this range, will return true.
     */
    containsRange(range: IRange): boolean;
    /**
     * Test if `otherRange` is in `range`. If the ranges are equal, will return true.
     */
    static containsRange(range: IRange, otherRange: IRange): boolean;
    /**
     * Test if `range` is strictly in this range. `range` must start after and end before this range for the result to be true.
     */
    strictContainsRange(range: IRange): boolean;
    /**
     * Test if `otherRange` is strinctly in `range` (must start after, and end before). If the ranges are equal, will return false.
     */
    static strictContainsRange(range: IRange, otherRange: IRange): boolean;
    /**
     * A reunion of the two ranges.
     * The smallest position will be used as the start point, and the largest one as the end point.
     */
    plusRange(range: IRange): Range;
    /**
     * A reunion of the two ranges.
     * The smallest position will be used as the start point, and the largest one as the end point.
     */
    static plusRange(a: IRange, b: IRange): Range;
    /**
     * A intersection of the two ranges.
     */
    intersectRanges(range: IRange): Range | null;
    /**
     * A intersection of the two ranges.
     */
    static intersectRanges(a: IRange, b: IRange): Range | null;
    /**
     * Test if this range equals other.
     */
    equalsRange(other: IRange | null): boolean;
    /**
     * Test if range `a` equals `b`.
     */
    static equalsRange(a: IRange | null, b: IRange | null): boolean;
    /**
     * Return the end position (which will be after or equal to the start position)
     */
    getEndPosition(): Position;
    /**
     * Return the start position (which will be before or equal to the end position)
     */
    getStartPosition(): Position;
    /**
     * Transform to a user presentable string representation.
     */
    toString(): string;
    /**
     * Create a new range using this range's start position, and using endLineNumber and endColumn as the end position.
     */
    setEndPosition(endLineNumber: number, endColumn: number): Range;
    /**
     * Create a new range using this range's end position, and using startLineNumber and startColumn as the start position.
     */
    setStartPosition(startLineNumber: number, startColumn: number): Range;
    /**
     * Create a new empty range using this range's start position.
     */
    collapseToStart(): Range;
    /**
     * Create a new empty range using this range's start position.
     */
    static collapseToStart(range: IRange): Range;
    static fromPositions(start: IPosition, end?: IPosition): Range;
    /**
     * Create a `Range` from an `IRange`.
     */
    static lift(range: undefined | null): null;
    static lift(range: IRange): Range;
    /**
     * Test if `obj` is an `IRange`.
     */
    static isIRange(obj: any): obj is IRange;
    /**
     * Test if the two ranges are touching in any way.
     */
    static areIntersectingOrTouching(a: IRange, b: IRange): boolean;
    /**
     * Test if the two ranges are intersecting. If the ranges are touching it returns true.
     */
    static areIntersecting(a: IRange, b: IRange): boolean;
    /**
     * A function that compares ranges, useful for sorting ranges
     * It will first compare ranges on the startPosition and then on the endPosition
     */
    static compareRangesUsingStarts(
      a: IRange | null | undefined,
      b: IRange | null | undefined
    ): number;
    /**
     * A function that compares ranges, useful for sorting ranges
     * It will first compare ranges on the endPosition and then on the startPosition
     */
    static compareRangesUsingEnds(a: IRange, b: IRange): number;
    /**
     * Test if the range spans multiple lines.
     */
    static spansMultipleLines(range: IRange): boolean;
  }

  /**
   * A selection in the editor.
   * The selection is a range that has an orientation.
   */
  export interface ISelection {
    /**
     * The line number on which the selection has started.
     */
    readonly selectionStartLineNumber: number;
    /**
     * The column on `selectionStartLineNumber` where the selection has started.
     */
    readonly selectionStartColumn: number;
    /**
     * The line number on which the selection has ended.
     */
    readonly positionLineNumber: number;
    /**
     * The column on `positionLineNumber` where the selection has ended.
     */
    readonly positionColumn: number;
  }

  /**
   * A selection in the editor.
   * The selection is a range that has an orientation.
   */
  export class Selection extends Range {
    /**
     * The line number on which the selection has started.
     */
    readonly selectionStartLineNumber: number;
    /**
     * The column on `selectionStartLineNumber` where the selection has started.
     */
    readonly selectionStartColumn: number;
    /**
     * The line number on which the selection has ended.
     */
    readonly positionLineNumber: number;
    /**
     * The column on `positionLineNumber` where the selection has ended.
     */
    readonly positionColumn: number;
    constructor(
      selectionStartLineNumber: number,
      selectionStartColumn: number,
      positionLineNumber: number,
      positionColumn: number
    );
    /**
     * Transform to a human-readable representation.
     */
    toString(): string;
    /**
     * Test if equals other selection.
     */
    equalsSelection(other: ISelection): boolean;
    /**
     * Test if the two selections are equal.
     */
    static selectionsEqual(a: ISelection, b: ISelection): boolean;
    /**
     * Get directions (LTR or RTL).
     */
    getDirection(): SelectionDirection;
    /**
     * Create a new selection with a different `positionLineNumber` and `positionColumn`.
     */
    setEndPosition(endLineNumber: number, endColumn: number): Selection;
    /**
     * Get the position at `positionLineNumber` and `positionColumn`.
     */
    getPosition(): Position;
    /**
     * Create a new selection with a different `selectionStartLineNumber` and `selectionStartColumn`.
     */
    setStartPosition(startLineNumber: number, startColumn: number): Selection;
    /**
     * Create a `Selection` from one or two positions
     */
    static fromPositions(start: IPosition, end?: IPosition): Selection;
    /**
     * Create a `Selection` from an `ISelection`.
     */
    static liftSelection(sel: ISelection): Selection;
    /**
     * `a` equals `b`.
     */
    static selectionsArrEqual(a: ISelection[], b: ISelection[]): boolean;
    /**
     * Test if `obj` is an `ISelection`.
     */
    static isISelection(obj: any): obj is ISelection;
    /**
     * Create with a direction.
     */
    static createWithDirection(
      startLineNumber: number,
      startColumn: number,
      endLineNumber: number,
      endColumn: number,
      direction: SelectionDirection
    ): Selection;
  }

  /**
   * The direction of a selection.
   */
  export enum SelectionDirection {
    /**
     * The selection starts above where it ends.
     */
    LTR = 0,
    /**
     * The selection starts below where it ends.
     */
    RTL = 1,
  }

  export class Token {
    _tokenBrand: void;
    readonly offset: number;
    readonly type: string;
    readonly language: string;
    constructor(offset: number, type: string, language: string);
    toString(): string;
  }
}
declare namespace monaco.editor {
  export interface IDiffNavigator {
    canNavigate(): boolean;
    next(): void;
    previous(): void;
    dispose(): void;
  }

  /**
   * Create a new editor under `domElement`.
   * `domElement` should be empty (not contain other dom nodes).
   * The editor will read the size of `domElement`.
   */
  export function create(
    domElement: HTMLElement,
    options?: IStandaloneEditorConstructionOptions,
    override?: { [index: string]: any }
  ): IStandaloneCodeEditor;

  /**
   * Emitted when an editor is created.
   * Creating a diff editor might cause this listener to be invoked with the two editors.
   * @event
   */
  export function onDidCreateEditor(
    listener: (codeEditor: ICodeEditor) => void
  ): IDisposable;

  /**
   * Create a new diff editor under `domElement`.
   * `domElement` should be empty (not contain other dom nodes).
   * The editor will read the size of `domElement`.
   */
  export function createDiffEditor(
    domElement: HTMLElement,
    options?: IDiffEditorConstructionOptions,
    override?: { [index: string]: any }
  ): IStandaloneDiffEditor;

  export interface IDiffNavigatorOptions {
    readonly followsCaret?: boolean;
    readonly ignoreCharChanges?: boolean;
    readonly alwaysRevealFirst?: boolean;
  }

  export function createDiffNavigator(
    diffEditor: IStandaloneDiffEditor,
    opts?: IDiffNavigatorOptions
  ): IDiffNavigator;

  /**
   * Create a new editor model.
   * You can specify the language that should be set for this model or let the language be inferred from the `uri`.
   */
  export function createModel(
    value: string,
    language?: string,
    uri?: Uri
  ): ITextModel;

  /**
   * Change the language for a model.
   */
  export function setModelLanguage(model: ITextModel, languageId: string): void;

  /**
   * Set the markers for a model.
   */
  export function setModelMarkers(
    model: ITextModel,
    owner: string,
    markers: IMarkerData[]
  ): void;

  /**
   * Get markers for owner and/or resource
   *
   * @returns list of markers
   */
  export function getModelMarkers(filter: {
    owner?: string;
    resource?: Uri;
    take?: number;
  }): IMarker[];

  /**
   * Get the model that has `uri` if it exists.
   */
  export function getModel(uri: Uri): ITextModel | null;

  /**
   * Get all the created models.
   */
  export function getModels(): ITextModel[];

  /**
   * Emitted when a model is created.
   * @event
   */
  export function onDidCreateModel(
    listener: (model: ITextModel) => void
  ): IDisposable;

  /**
   * Emitted right before a model is disposed.
   * @event
   */
  export function onWillDisposeModel(
    listener: (model: ITextModel) => void
  ): IDisposable;

  /**
   * Emitted when a different language is set to a model.
   * @event
   */
  export function onDidChangeModelLanguage(
    listener: (e: {
      readonly model: ITextModel;
      readonly oldLanguage: string;
    }) => void
  ): IDisposable;

  /**
   * Create a new web worker that has model syncing capabilities built in.
   * Specify an AMD module to load that will `create` an object that will be proxied.
   */
  export function createWebWorker(opts: IWebWorkerOptions): MonacoWebWorker;

  /**
   * Colorize the contents of `domNode` using attribute `data-lang`.
   */
  export function colorizeElement(
    domNode: HTMLElement,
    options: IColorizerElementOptions
  ): Promise<void>;

  /**
   * Colorize `text` using language `languageId`.
   */
  export function colorize(
    text: string,
    languageId: string,
    options: IColorizerOptions
  ): Promise<string>;

  /**
   * Colorize a line in a model.
   */
  export function colorizeModelLine(
    model: ITextModel,
    lineNumber: number,
    tabSize?: number
  ): string;

  /**
   * Tokenize `text` using language `languageId`
   */
  export function tokenize(text: string, languageId: string): Token[][];

  /**
   * Define a new theme or update an existing theme.
   */
  export function defineTheme(
    themeName: string,
    themeData: IStandaloneThemeData
  ): void;

  /**
   * Switches to a theme.
   */
  export function setTheme(themeName: string): void;

  /**
   * Clears all cached font measurements and triggers re-measurement.
   */
  export function remeasureFonts(): void;

  export type BuiltinTheme = "vs" | "vs-dark" | "hc-black";

  export interface IStandaloneThemeData {
    base: BuiltinTheme;
    inherit: boolean;
    rules: ITokenThemeRule[];
    encodedTokensColors?: string[];
    colors: IColors;
  }

  // export type IColors = { [colorId: string]: string };

  export interface ITokenThemeRule {
    token: string;
    foreground?: string;
    background?: string;
    fontStyle?: string;
  }

  /**
   * A web worker that can provide a proxy to an arbitrary file.
   */
  export interface MonacoWebWorker {
    /**
     * Terminate the web worker, thus invalidating the returned proxy.
     */
    dispose(): void;
    /**
     * Get a proxy to the arbitrary loaded code.
     */
    getProxy(): Promise<any>;
    /**
     * Synchronize (send) the models at `resources` to the web worker,
     * making them available in the monaco.worker.getMirrorModels().
     */
    withSyncedResources(resources: Uri[]): Promise<any>;
  }

  export interface IWebWorkerOptions {
    /**
     * The AMD moduleId to load.
     * It should export a function `create` that should return the exported proxy.
     */
    moduleId: string;
    /**
     * The data to send over when calling create on the module.
     */
    createData?: any;
    /**
     * A label to be used to identify the web worker for debugging purposes.
     */
    label?: string;
    /**
     * An object that can be used by the web worker to make calls back to the main thread.
     */
    host?: any;
    /**
     * Keep idle models.
     * Defaults to false, which means that idle models will stop syncing after a while.
     */
    keepIdleModels?: boolean;
  }

  /**
   * Description of an action contribution
   */
  export interface IActionDescriptor {
    /**
     * An unique identifier of the contributed action.
     */
    id: string;
    /**
     * A label of the action that will be presented to the user.
     */
    label: string;
    /**
     * Precondition rule.
     */
    precondition?: string;
    /**
     * An array of keybindings for the action.
     */
    keybindings?: number[];
    /**
     * The keybinding rule (condition on top of precondition).
     */
    keybindingContext?: string;
    /**
     * Control if the action should show up in the context menu and where.
     * The context menu of the editor has these default:
     *   navigation - The navigation group comes first in all cases.
     *   1_modification - This group comes next and contains commands that modify your code.
     *   9_cutcopypaste - The last default group with the basic editing commands.
     * You can also create your own group.
     * Defaults to null (don't show in context menu).
     */
    contextMenuGroupId?: string;
    /**
     * Control the order in the context menu group.
     */
    contextMenuOrder?: number;
    /**
     * Method that will be executed when the action is triggered.
     * @param editor The editor instance is passed in as a convenience
     */
    run(editor: ICodeEditor, ...args: any[]): void | Promise<void>;
  }

  /**
   * Options which apply for all editors.
   */
  export interface IGlobalEditorOptions {
    /**
     * The number of spaces a tab is equal to.
     * This setting is overridden based on the file contents when `detectIndentation` is on.
     * Defaults to 4.
     */
    tabSize?: number;
    /**
     * Insert spaces when pressing `Tab`.
     * This setting is overridden based on the file contents when `detectIndentation` is on.
     * Defaults to true.
     */
    insertSpaces?: boolean;
    /**
     * Controls whether `tabSize` and `insertSpaces` will be automatically detected when a file is opened based on the file contents.
     * Defaults to true.
     */
    detectIndentation?: boolean;
    /**
     * Remove trailing auto inserted whitespace.
     * Defaults to true.
     */
    trimAutoWhitespace?: boolean;
    /**
     * Special handling for large files to disable certain memory intensive features.
     * Defaults to true.
     */
    largeFileOptimizations?: boolean;
    /**
     * Controls whether completions should be computed based on words in the document.
     * Defaults to true.
     */
    wordBasedSuggestions?: boolean;
    /**
     * Keep peek editors open even when double clicking their content or when hitting `Escape`.
     * Defaults to false.
     */
    stablePeek?: boolean;
    /**
     * Lines above this length will not be tokenized for performance reasons.
     * Defaults to 20000.
     */
    maxTokenizationLineLength?: number;
  }

  /**
   * The options to create an editor.
   */
  export interface IStandaloneEditorConstructionOptions
    extends IEditorConstructionOptions,
      IGlobalEditorOptions {
    /**
     * The initial model associated with this code editor.
     */
    model?: ITextModel | null;
    /**
     * The initial value of the auto created model in the editor.
     * To not create automatically a model, use `model: null`.
     */
    value?: string;
    /**
     * The initial language of the auto created model in the editor.
     * To not create automatically a model, use `model: null`.
     */
    language?: string;
    /**
     * Initial theme to be used for rendering.
     * The current out-of-the-box available themes are: 'vs' (default), 'vs-dark', 'hc-black'.
     * You can create custom themes via `monaco.editor.defineTheme`.
     * To switch a theme, use `monaco.editor.setTheme`
     */
    theme?: string;
    /**
     * An URL to open when Ctrl+H (Windows and Linux) or Cmd+H (OSX) is pressed in
     * the accessibility help dialog in the editor.
     *
     * Defaults to "https://go.microsoft.com/fwlink/?linkid=852450"
     */
    accessibilityHelpUrl?: string;
  }

  /**
   * The options to create a diff editor.
   */
  export interface IDiffEditorConstructionOptions extends IDiffEditorOptions {
    /**
     * Initial theme to be used for rendering.
     * The current out-of-the-box available themes are: 'vs' (default), 'vs-dark', 'hc-black'.
     * You can create custom themes via `monaco.editor.defineTheme`.
     * To switch a theme, use `monaco.editor.setTheme`
     */
    theme?: string;
  }

  export interface IStandaloneCodeEditor extends ICodeEditor {
    updateOptions(newOptions: IEditorOptions & IGlobalEditorOptions): void;
    addCommand(
      keybinding: number,
      handler: (...args: any[]) => void,
      context?: string
    ): string | null;
    createContextKey(key: string, defaultValue: any): IContextKey;
    addAction(descriptor: IActionDescriptor): IDisposable;
  }

  export interface IStandaloneDiffEditor extends IDiffEditor {
    addCommand(
      keybinding: number,
      handler: (...args: any[]) => void,
      context?: string
    ): string | null;
    createContextKey(key: string, defaultValue: any): IContextKey;
    addAction(descriptor: IActionDescriptor): IDisposable;
    getOriginalEditor(): IStandaloneCodeEditor;
    getModifiedEditor(): IStandaloneCodeEditor;
  }

  export interface IContextKey {
    set(value: any): void;
    reset(): void;
    get(): any;
  }

  export interface IMarker {
    owner: string;
    resource: Uri;
    severity: MarkerSeverity;
    code?: { value: string; link: Uri };
    message: string;
    source?: string;
    startLineNumber: number;
    startColumn: number;
    endLineNumber: number;
    endColumn: number;
    relatedInformation?: IRelatedInformation[];
    tags?: MarkerTag[];
  }

  /**
   * A structure defining a problem/warning/etc.
   */
  export interface IMarkerData {
    code?: { value: string; link: Uri };
    severity: MarkerSeverity;
    message: string;
    source?: string;
    startLineNumber: number;
    startColumn: number;
    endLineNumber: number;
    endColumn: number;
    relatedInformation?: IRelatedInformation[];
    tags?: MarkerTag[];
  }

  /**
   *
   */
  export interface IRelatedInformation {
    resource: Uri;
    message: string;
    startLineNumber: number;
    startColumn: number;
    endLineNumber: number;
    endColumn: number;
  }

  export interface IColorizerOptions {
    tabSize?: number;
  }

  export interface IColorizerElementOptions extends IColorizerOptions {
    theme?: string;
    mimeType?: string;
  }

  export enum ScrollbarVisibility {
    Auto = 1,
    Hidden = 2,
    Visible = 3,
  }

  export interface ThemeColor {
    id: string;
  }

  /**
   * Vertical Lane in the overview ruler of the editor.
   */
  export enum OverviewRulerLane {
    Left = 1,
    Center = 2,
    Right = 4,
    Full = 7,
  }

  /**
   * Position in the minimap to render the decoration.
   */
  export enum MinimapPosition {
    Inline = 1,
    Gutter = 2,
  }

  export interface IDecorationOptions {
    /**
     * CSS color to render.
     * e.g.: rgba(100, 100, 100, 0.5) or a color from the color registry
     */
    color: string | ThemeColor | undefined;
    /**
     * CSS color to render.
     * e.g.: rgba(100, 100, 100, 0.5) or a color from the color registry
     */
    darkColor?: string | ThemeColor;
  }

  /**
   * Options for rendering a model decoration in the overview ruler.
   */
  export interface IModelDecorationOverviewRulerOptions
    extends IDecorationOptions {
    /**
     * The position in the overview ruler.
     */
    position: OverviewRulerLane;
  }

  /**
   * Options for rendering a model decoration in the overview ruler.
   */
  export interface IModelDecorationMinimapOptions extends IDecorationOptions {
    /**
     * The position in the overview ruler.
     */
    position: MinimapPosition;
  }

  /**
   * Options for a model decoration.
   */
  export interface IModelDecorationOptions {
    /**
     * Customize the growing behavior of the decoration when typing at the edges of the decoration.
     * Defaults to TrackedRangeStickiness.AlwaysGrowsWhenTypingAtEdges
     */
    stickiness?: TrackedRangeStickiness;
    /**
     * CSS class name describing the decoration.
     */
    className?: string | null;
    /**
     * Message to be rendered when hovering over the glyph margin decoration.
     */
    glyphMarginHoverMessage?: IMarkdownString | IMarkdownString[] | null;
    /**
     * Array of MarkdownString to render as the decoration message.
     */
    hoverMessage?: IMarkdownString | IMarkdownString[] | null;
    /**
     * Should the decoration expand to encompass a whole line.
     */
    isWholeLine?: boolean;
    /**
     * Specifies the stack order of a decoration.
     * A decoration with greater stack order is always in front of a decoration with a lower stack order.
     */
    zIndex?: number;
    /**
     * If set, render this decoration in the overview ruler.
     */
    overviewRuler?: IModelDecorationOverviewRulerOptions | null;
    /**
     * If set, render this decoration in the minimap.
     */
    minimap?: IModelDecorationMinimapOptions | null;
    /**
     * If set, the decoration will be rendered in the glyph margin with this CSS class name.
     */
    glyphMarginClassName?: string | null;
    /**
     * If set, the decoration will be rendered in the lines decorations with this CSS class name.
     */
    linesDecorationsClassName?: string | null;
    /**
     * If set, the decoration will be rendered in the margin (covering its full width) with this CSS class name.
     */
    marginClassName?: string | null;
    /**
     * If set, the decoration will be rendered inline with the text with this CSS class name.
     * Please use this only for CSS rules that must impact the text. For example, use `className`
     * to have a background color decoration.
     */
    inlineClassName?: string | null;
    /**
     * If there is an `inlineClassName` which affects letter spacing.
     */
    inlineClassNameAffectsLetterSpacing?: boolean;
    /**
     * If set, the decoration will be rendered before the text with this CSS class name.
     */
    beforeContentClassName?: string | null;
    /**
     * If set, the decoration will be rendered after the text with this CSS class name.
     */
    afterContentClassName?: string | null;
  }

  /**
   * New model decorations.
   */
  export interface IModelDeltaDecoration {
    /**
     * Range that this decoration covers.
     */
    range: IRange;
    /**
     * Options associated with this decoration.
     */
    options: IModelDecorationOptions;
  }

  /**
   * A decoration in the model.
   */
  export interface IModelDecoration {
    /**
     * Identifier for a decoration.
     */
    readonly id: string;
    /**
     * Identifier for a decoration's owner.
     */
    readonly ownerId: number;
    /**
     * Range that this decoration covers.
     */
    readonly range: Range;
    /**
     * Options associated with this decoration.
     */
    readonly options: IModelDecorationOptions;
  }

  /**
   * Word inside a model.
   */
  export interface IWordAtPosition {
    /**
     * The word.
     */
    readonly word: string;
    /**
     * The column where the word starts.
     */
    readonly startColumn: number;
    /**
     * The column where the word ends.
     */
    readonly endColumn: number;
  }

  /**
   * End of line character preference.
   */
  export enum EndOfLinePreference {
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

  /**
   * The default end of line to use when instantiating models.
   */
  export enum DefaultEndOfLine {
    /**
     * Use line feed (\n) as the end of line character.
     */
    LF = 1,
    /**
     * Use carriage return and line feed (\r\n) as the end of line character.
     */
    CRLF = 2,
  }

  /**
   * End of line character preference.
   */
  export enum EndOfLineSequence {
    /**
     * Use line feed (\n) as the end of line character.
     */
    LF = 0,
    /**
     * Use carriage return and line feed (\r\n) as the end of line character.
     */
    CRLF = 1,
  }

  /**
   * A single edit operation, that acts as a simple replace.
   * i.e. Replace text at `range` with `text` in model.
   */
  export interface ISingleEditOperation {
    /**
     * The range to replace. This can be empty to emulate a simple insert.
     */
    range: IRange;
    /**
     * The text to replace with. This can be null to emulate a simple delete.
     */
    text: string | null;
    /**
     * This indicates that this operation has "insert" semantics.
     * i.e. forceMoveMarkers = true => if `range` is collapsed, all markers at the position will be moved.
     */
    forceMoveMarkers?: boolean;
  }

  /**
   * A single edit operation, that has an identifier.
   */
  export interface IIdentifiedSingleEditOperation {
    /**
     * The range to replace. This can be empty to emulate a simple insert.
     */
    range: Range;
    /**
     * The text to replace with. This can be null to emulate a simple delete.
     */
    text: string | null;
    /**
     * This indicates that this operation has "insert" semantics.
     * i.e. forceMoveMarkers = true => if `range` is collapsed, all markers at the position will be moved.
     */
    forceMoveMarkers?: boolean;
  }

  export class TextModelResolvedOptions {
    _textModelResolvedOptionsBrand: void;
    readonly tabSize: number;
    readonly indentSize: number;
    readonly insertSpaces: boolean;
    readonly defaultEOL: DefaultEndOfLine;
    readonly trimAutoWhitespace: boolean;
  }

  export interface ITextModelUpdateOptions {
    tabSize?: number;
    indentSize?: number;
    insertSpaces?: boolean;
    trimAutoWhitespace?: boolean;
  }

  export class FindMatch {
    _findMatchBrand: void;
    readonly range: Range;
    readonly matches: string[] | null;
  }

  /**
   * Describes the behavior of decorations when typing/editing near their edges.
   * Note: Please do not edit the values, as they very carefully match `DecorationRangeBehavior`
   */
  export enum TrackedRangeStickiness {
    AlwaysGrowsWhenTypingAtEdges = 0,
    NeverGrowsWhenTypingAtEdges = 1,
    GrowsOnlyWhenTypingBefore = 2,
    GrowsOnlyWhenTypingAfter = 3,
  }

  /**
   * A model.
   */
  export interface ITextModel {
    /**
     * Gets the resource associated with this editor model.
     */
    readonly uri: Uri;
    /**
     * A unique identifier associated with this model.
     */
    readonly id: string;
    /**
     * Get the resolved options for this model.
     */
    getOptions(): TextModelResolvedOptions;
    /**
     * Get the current version id of the model.
     * Anytime a change happens to the model (even undo/redo),
     * the version id is incremented.
     */
    getVersionId(): number;
    /**
     * Get the alternative version id of the model.
     * This alternative version id is not always incremented,
     * it will return the same values in the case of undo-redo.
     */
    getAlternativeVersionId(): number;
    /**
     * Replace the entire text buffer value contained in this model.
     */
    setValue(newValue: string): void;
    /**
     * Get the text stored in this model.
     * @param eol The end of line character preference. Defaults to `EndOfLinePreference.TextDefined`.
     * @param preserverBOM Preserve a BOM character if it was detected when the model was constructed.
     * @return The text.
     */
    getValue(eol?: EndOfLinePreference, preserveBOM?: boolean): string;
    /**
     * Get the length of the text stored in this model.
     */
    getValueLength(eol?: EndOfLinePreference, preserveBOM?: boolean): number;
    /**
     * Get the text in a certain range.
     * @param range The range describing what text to get.
     * @param eol The end of line character preference. This will only be used for multiline ranges. Defaults to `EndOfLinePreference.TextDefined`.
     * @return The text.
     */
    getValueInRange(range: IRange, eol?: EndOfLinePreference): string;
    /**
     * Get the length of text in a certain range.
     * @param range The range describing what text length to get.
     * @return The text length.
     */
    getValueLengthInRange(range: IRange): number;
    /**
     * Get the character count of text in a certain range.
     * @param range The range describing what text length to get.
     */
    getCharacterCountInRange(range: IRange): number;
    /**
     * Get the number of lines in the model.
     */
    getLineCount(): number;
    /**
     * Get the text for a certain line.
     */
    getLineContent(lineNumber: number): string;
    /**
     * Get the text length for a certain line.
     */
    getLineLength(lineNumber: number): number;
    /**
     * Get the text for all lines.
     */
    getLinesContent(): string[];
    /**
     * Get the end of line sequence predominantly used in the text buffer.
     * @return EOL char sequence (e.g.: '\n' or '\r\n').
     */
    getEOL(): string;
    /**
     * Get the minimum legal column for line at `lineNumber`
     */
    getLineMinColumn(lineNumber: number): number;
    /**
     * Get the maximum legal column for line at `lineNumber`
     */
    getLineMaxColumn(lineNumber: number): number;
    /**
     * Returns the column before the first non whitespace character for line at `lineNumber`.
     * Returns 0 if line is empty or contains only whitespace.
     */
    getLineFirstNonWhitespaceColumn(lineNumber: number): number;
    /**
     * Returns the column after the last non whitespace character for line at `lineNumber`.
     * Returns 0 if line is empty or contains only whitespace.
     */
    getLineLastNonWhitespaceColumn(lineNumber: number): number;
    /**
     * Create a valid position,
     */
    validatePosition(position: IPosition): Position;
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
    modifyPosition(position: IPosition, offset: number): Position;
    /**
     * Create a valid range.
     */
    validateRange(range: IRange): Range;
    /**
     * Converts the position to a zero-based offset.
     *
     * The position will be [adjusted](#TextDocument.validatePosition).
     *
     * @param position A position.
     * @return A valid zero-based offset.
     */
    getOffsetAt(position: IPosition): number;
    /**
     * Converts a zero-based offset to a position.
     *
     * @param offset A zero-based offset.
     * @return A valid [position](#Position).
     */
    getPositionAt(offset: number): Position;
    /**
     * Get a range covering the entire model
     */
    getFullModelRange(): Range;
    /**
     * Returns if the model was disposed or not.
     */
    isDisposed(): boolean;
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
    findMatches(
      searchString: string,
      searchOnlyEditableRange: boolean,
      isRegex: boolean,
      matchCase: boolean,
      wordSeparators: string | null,
      captureMatches: boolean,
      limitResultCount?: number
    ): FindMatch[];
    /**
     * Search the model.
     * @param searchString The string used to search. If it is a regular expression, set `isRegex` to true.
     * @param searchScope Limit the searching to only search inside this range.
     * @param isRegex Used to indicate that `searchString` is a regular expression.
     * @param matchCase Force the matching to match lower/upper case exactly.
     * @param wordSeparators Force the matching to match entire words only. Pass null otherwise.
     * @param captureMatches The result will contain the captured groups.
     * @param limitResultCount Limit the number of results
     * @return The ranges where the matches are. It is empty if no matches have been found.
     */
    findMatches(
      searchString: string,
      searchScope: IRange,
      isRegex: boolean,
      matchCase: boolean,
      wordSeparators: string | null,
      captureMatches: boolean,
      limitResultCount?: number
    ): FindMatch[];
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
    findNextMatch(
      searchString: string,
      searchStart: IPosition,
      isRegex: boolean,
      matchCase: boolean,
      wordSeparators: string | null,
      captureMatches: boolean
    ): FindMatch | null;
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
    findPreviousMatch(
      searchString: string,
      searchStart: IPosition,
      isRegex: boolean,
      matchCase: boolean,
      wordSeparators: string | null,
      captureMatches: boolean
    ): FindMatch | null;
    /**
     * Get the language associated with this model.
     */
    getModeId(): string;
    /**
     * Get the word under or besides `position`.
     * @param position The position to look for a word.
     * @return The word under or besides `position`. Might be null.
     */
    getWordAtPosition(position: IPosition): IWordAtPosition | null;
    /**
     * Get the word under or besides `position` trimmed to `position`.column
     * @param position The position to look for a word.
     * @return The word under or besides `position`. Will never be null.
     */
    getWordUntilPosition(position: IPosition): IWordAtPosition;
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
    deltaDecorations(
      oldDecorations: string[],
      newDecorations: IModelDeltaDecoration[],
      ownerId?: number
    ): string[];
    /**
     * Get the options associated with a decoration.
     * @param id The decoration id.
     * @return The decoration options or null if the decoration was not found.
     */
    getDecorationOptions(id: string): IModelDecorationOptions | null;
    /**
     * Get the range associated with a decoration.
     * @param id The decoration id.
     * @return The decoration range or null if the decoration was not found.
     */
    getDecorationRange(id: string): Range | null;
    /**
     * Gets all the decorations for the line `lineNumber` as an array.
     * @param lineNumber The line number
     * @param ownerId If set, it will ignore decorations belonging to other owners.
     * @param filterOutValidation If set, it will ignore decorations specific to validation (i.e. warnings, errors).
     * @return An array with the decorations
     */
    getLineDecorations(
      lineNumber: number,
      ownerId?: number,
      filterOutValidation?: boolean
    ): IModelDecoration[];
    /**
     * Gets all the decorations for the lines between `startLineNumber` and `endLineNumber` as an array.
     * @param startLineNumber The start line number
     * @param endLineNumber The end line number
     * @param ownerId If set, it will ignore decorations belonging to other owners.
     * @param filterOutValidation If set, it will ignore decorations specific to validation (i.e. warnings, errors).
     * @return An array with the decorations
     */
    getLinesDecorations(
      startLineNumber: number,
      endLineNumber: number,
      ownerId?: number,
      filterOutValidation?: boolean
    ): IModelDecoration[];
    /**
     * Gets all the decorations in a range as an array. Only `startLineNumber` and `endLineNumber` from `range` are used for filtering.
     * So for now it returns all the decorations on the same line as `range`.
     * @param range The range to search in
     * @param ownerId If set, it will ignore decorations belonging to other owners.
     * @param filterOutValidation If set, it will ignore decorations specific to validation (i.e. warnings, errors).
     * @return An array with the decorations
     */
    getDecorationsInRange(
      range: IRange,
      ownerId?: number,
      filterOutValidation?: boolean
    ): IModelDecoration[];
    /**
     * Gets all the decorations as an array.
     * @param ownerId If set, it will ignore decorations belonging to other owners.
     * @param filterOutValidation If set, it will ignore decorations specific to validation (i.e. warnings, errors).
     */
    getAllDecorations(
      ownerId?: number,
      filterOutValidation?: boolean
    ): IModelDecoration[];
    /**
     * Gets all the decorations that should be rendered in the overview ruler as an array.
     * @param ownerId If set, it will ignore decorations belonging to other owners.
     * @param filterOutValidation If set, it will ignore decorations specific to validation (i.e. warnings, errors).
     */
    getOverviewRulerDecorations(
      ownerId?: number,
      filterOutValidation?: boolean
    ): IModelDecoration[];
    /**
     * Normalize a string containing whitespace according to indentation rules (converts to spaces or to tabs).
     */
    normalizeIndentation(str: string): string;
    /**
     * Change the options of this model.
     */
    updateOptions(newOpts: ITextModelUpdateOptions): void;
    /**
     * Detect the indentation options for this model from its content.
     */
    detectIndentation(
      defaultInsertSpaces: boolean,
      defaultTabSize: number
    ): void;
    /**
     * Push a stack element onto the undo stack. This acts as an undo/redo point.
     * The idea is to use `pushEditOperations` to edit the model and then to
     * `pushStackElement` to create an undo/redo stop point.
     */
    pushStackElement(): void;
    /**
     * Push edit operations, basically editing the model. This is the preferred way
     * of editing the model. The edit operations will land on the undo stack.
     * @param beforeCursorState The cursor state before the edit operations. This cursor state will be returned when `undo` or `redo` are invoked.
     * @param editOperations The edit operations.
     * @param cursorStateComputer A callback that can compute the resulting cursors state after the edit operations have been executed.
     * @return The cursor state returned by the `cursorStateComputer`.
     */
    pushEditOperations(
      beforeCursorState: Selection[],
      editOperations: IIdentifiedSingleEditOperation[],
      cursorStateComputer: (
        inverseEditOperations: IIdentifiedSingleEditOperation[]
      ) => Selection[] | null
    ): Selection[] | null;
    /**
     * Change the end of line sequence. This is the preferred way of
     * changing the eol sequence. This will land on the undo stack.
     */
    pushEOL(eol: EndOfLineSequence): void;
    /**
     * Edit the model without adding the edits to the undo stack.
     * This can have dire consequences on the undo stack! See @pushEditOperations for the preferred way.
     * @param operations The edit operations.
     * @return The inverse edit operations, that, when applied, will bring the model back to the previous state.
     */
    applyEdits(
      operations: IIdentifiedSingleEditOperation[]
    ): IIdentifiedSingleEditOperation[];
    /**
     * Change the end of line sequence without recording in the undo stack.
     * This can have dire consequences on the undo stack! See @pushEOL for the preferred way.
     */
    setEOL(eol: EndOfLineSequence): void;
    /**
     * An event emitted when the contents of the model have changed.
     * @event
     */
    onDidChangeContent(
      listener: (e: IModelContentChangedEvent) => void
    ): IDisposable;
    /**
     * An event emitted when decorations of the model have changed.
     * @event
     */
    onDidChangeDecorations(listener: (e: {}) => void): IDisposable;
    /**
     * An event emitted when the model options have changed.
     * @event
     */
    onDidChangeOptions(
      listener: (e: IModelOptionsChangedEvent) => void
    ): IDisposable;
    /**
     * An event emitted when the language associated with the model has changed.
     * @event
     */
    onDidChangeLanguage(
      listener: (e: IModelLanguageChangedEvent) => void
    ): IDisposable;
    /**
     * An event emitted when the language configuration associated with the model has changed.
     * @event
     */
    onDidChangeLanguageConfiguration(listener: (e: {}) => void): IDisposable;
    /**
     * An event emitted right before disposing the model.
     * @event
     */
    onWillDispose(listener: () => void): IDisposable;
    /**
     * Destroy this model. This will unbind the model from the mode
     * and make all necessary clean-up to release this object to the GC.
     */
    dispose(): void;
  }

  /**
   * A builder and helper for edit operations for a command.
   */
  export interface IEditOperationBuilder {
    /**
     * Add a new edit operation (a replace operation).
     * @param range The range to replace (delete). May be empty to represent a simple insert.
     * @param text The text to replace with. May be null to represent a simple delete.
     */
    addEditOperation(
      range: Range,
      text: string | null,
      forceMoveMarkers?: boolean
    ): void;
    /**
     * Add a new edit operation (a replace operation).
     * The inverse edits will be accessible in `ICursorStateComputerData.getInverseEditOperations()`
     * @param range The range to replace (delete). May be empty to represent a simple insert.
     * @param text The text to replace with. May be null to represent a simple delete.
     */
    addTrackedEditOperation(
      range: Range,
      text: string | null,
      forceMoveMarkers?: boolean
    ): void;
    /**
     * Track `selection` when applying edit operations.
     * A best effort will be made to not grow/expand the selection.
     * An empty selection will clamp to a nearby character.
     * @param selection The selection to track.
     * @param trackPreviousOnEmpty If set, and the selection is empty, indicates whether the selection
     *           should clamp to the previous or the next character.
     * @return A unique identifier.
     */
    trackSelection(
      selection: Selection,
      trackPreviousOnEmpty?: boolean
    ): string;
  }

  /**
   * A helper for computing cursor state after a command.
   */
  export interface ICursorStateComputerData {
    /**
     * Get the inverse edit operations of the added edit operations.
     */
    getInverseEditOperations(): IIdentifiedSingleEditOperation[];
    /**
     * Get a previously tracked selection.
     * @param id The unique identifier returned by `trackSelection`.
     * @return The selection.
     */
    getTrackedSelection(id: string): Selection;
  }

  /**
   * A command that modifies text / cursor state on a model.
   */
  export interface ICommand {
    /**
     * Get the edit operations needed to execute this command.
     * @param model The model the command will execute on.
     * @param builder A helper to collect the needed edit operations and to track selections.
     */
    getEditOperations(model: ITextModel, builder: IEditOperationBuilder): void;
    /**
     * Compute the cursor state after the edit operations were applied.
     * @param model The model the command has executed on.
     * @param helper A helper to get inverse edit operations and to get previously tracked selections.
     * @return The cursor state after the command executed.
     */
    computeCursorState(
      model: ITextModel,
      helper: ICursorStateComputerData
    ): Selection;
  }

  /**
   * A model for the diff editor.
   */
  export interface IDiffEditorModel {
    /**
     * Original model.
     */
    original: ITextModel;
    /**
     * Modified model.
     */
    modified: ITextModel;
  }

  /**
   * An event describing that an editor has had its model reset (i.e. `editor.setModel()`).
   */
  export interface IModelChangedEvent {
    /**
     * The `uri` of the previous model or null.
     */
    readonly oldModelUrl: Uri | null;
    /**
     * The `uri` of the new model or null.
     */
    readonly newModelUrl: Uri | null;
  }

  export interface IDimension {
    width: number;
    height: number;
  }

  /**
   * A change
   */
  export interface IChange {
    readonly originalStartLineNumber: number;
    readonly originalEndLineNumber: number;
    readonly modifiedStartLineNumber: number;
    readonly modifiedEndLineNumber: number;
  }

  /**
   * A character level change.
   */
  export interface ICharChange extends IChange {
    readonly originalStartColumn: number;
    readonly originalEndColumn: number;
    readonly modifiedStartColumn: number;
    readonly modifiedEndColumn: number;
  }

  /**
   * A line change
   */
  export interface ILineChange extends IChange {
    readonly charChanges: ICharChange[] | undefined;
  }

  export interface IContentSizeChangedEvent {
    readonly contentWidth: number;
    readonly contentHeight: number;
    readonly contentWidthChanged: boolean;
    readonly contentHeightChanged: boolean;
  }

  export interface INewScrollPosition {
    scrollLeft?: number;
    scrollTop?: number;
  }

  export interface IEditorAction {
    readonly id: string;
    readonly label: string;
    readonly alias: string;
    isSupported(): boolean;
    run(): Promise<void>;
  }

  // export type IEditorModel = ITextModel | IDiffEditorModel;

  /**
   * A (serializable) state of the cursors.
   */
  export interface ICursorState {
    inSelectionMode: boolean;
    selectionStart: IPosition;
    position: IPosition;
  }

  /**
   * A (serializable) state of the view.
   */
  export interface IViewState {
    /** written by previous versions */
    scrollTop?: number;
    /** written by previous versions */
    scrollTopWithoutViewZones?: number;
    scrollLeft: number;
    firstPosition: IPosition;
    firstPositionDeltaTop: number;
  }

  /**
   * A (serializable) state of the code editor.
   */
  export interface ICodeEditorViewState {
    cursorState: ICursorState[];
    viewState: IViewState;
    contributionsState: { [id: string]: any };
  }

  /**
   * (Serializable) View state for the diff editor.
   */
  export interface IDiffEditorViewState {
    original: ICodeEditorViewState | null;
    modified: ICodeEditorViewState | null;
  }

  // /**
  //  * An editor view state.
  //  */
  // export type IEditorViewState = ICodeEditorViewState | IDiffEditorViewState;

  export enum ScrollType {
    Smooth = 0,
    Immediate = 1,
  }

  /**
   * An editor.
   */
  export interface IEditor {
    /**
     * An event emitted when the editor has been disposed.
     * @event
     */
    onDidDispose(listener: () => void): IDisposable;
    /**
     * Dispose the editor.
     */
    dispose(): void;
    /**
     * Get a unique id for this editor instance.
     */
    getId(): string;
    /**
     * Get the editor type. Please see `EditorType`.
     * This is to avoid an instanceof check
     */
    getEditorType(): string;
    /**
     * Update the editor's options after the editor has been created.
     */
    updateOptions(newOptions: IEditorOptions): void;
    /**
     * Instructs the editor to remeasure its container. This method should
     * be called when the container of the editor gets resized.
     *
     * If a dimension is passed in, the passed in value will be used.
     */
    layout(dimension?: IDimension): void;
    /**
     * Brings browser focus to the editor text
     */
    focus(): void;
    /**
     * Returns true if the text inside this editor is focused (i.e. cursor is blinking).
     */
    hasTextFocus(): boolean;
    /**
     * Returns all actions associated with this editor.
     */
    getSupportedActions(): IEditorAction[];
    /**
     * Saves current view state of the editor in a serializable object.
     */
    saveViewState(): IEditorViewState | null;
    /**
     * Restores the view state of the editor from a serializable object generated by `saveViewState`.
     */
    restoreViewState(state: IEditorViewState): void;
    /**
     * Given a position, returns a column number that takes tab-widths into account.
     */
    getVisibleColumnFromPosition(position: IPosition): number;
    /**
     * Returns the primary position of the cursor.
     */
    getPosition(): Position | null;
    /**
     * Set the primary position of the cursor. This will remove any secondary cursors.
     * @param position New primary cursor's position
     */
    setPosition(position: IPosition): void;
    /**
     * Scroll vertically as necessary and reveal a line.
     */
    revealLine(lineNumber: number, scrollType?: ScrollType): void;
    /**
     * Scroll vertically as necessary and reveal a line centered vertically.
     */
    revealLineInCenter(lineNumber: number, scrollType?: ScrollType): void;
    /**
     * Scroll vertically as necessary and reveal a line centered vertically only if it lies outside the viewport.
     */
    revealLineInCenterIfOutsideViewport(
      lineNumber: number,
      scrollType?: ScrollType
    ): void;
    /**
     * Scroll vertically or horizontally as necessary and reveal a position.
     */
    revealPosition(position: IPosition, scrollType?: ScrollType): void;
    /**
     * Scroll vertically or horizontally as necessary and reveal a position centered vertically.
     */
    revealPositionInCenter(position: IPosition, scrollType?: ScrollType): void;
    /**
     * Scroll vertically or horizontally as necessary and reveal a position centered vertically only if it lies outside the viewport.
     */
    revealPositionInCenterIfOutsideViewport(
      position: IPosition,
      scrollType?: ScrollType
    ): void;
    /**
     * Returns the primary selection of the editor.
     */
    getSelection(): Selection | null;
    /**
     * Returns all the selections of the editor.
     */
    getSelections(): Selection[] | null;
    /**
     * Set the primary selection of the editor. This will remove any secondary cursors.
     * @param selection The new selection
     */
    setSelection(selection: IRange): void;
    /**
     * Set the primary selection of the editor. This will remove any secondary cursors.
     * @param selection The new selection
     */
    setSelection(selection: Range): void;
    /**
     * Set the primary selection of the editor. This will remove any secondary cursors.
     * @param selection The new selection
     */
    setSelection(selection: ISelection): void;
    /**
     * Set the primary selection of the editor. This will remove any secondary cursors.
     * @param selection The new selection
     */
    setSelection(selection: Selection): void;
    /**
     * Set the selections for all the cursors of the editor.
     * Cursors will be removed or added, as necessary.
     */
    setSelections(selections: readonly ISelection[]): void;
    /**
     * Scroll vertically as necessary and reveal lines.
     */
    revealLines(
      startLineNumber: number,
      endLineNumber: number,
      scrollType?: ScrollType
    ): void;
    /**
     * Scroll vertically as necessary and reveal lines centered vertically.
     */
    revealLinesInCenter(
      lineNumber: number,
      endLineNumber: number,
      scrollType?: ScrollType
    ): void;
    /**
     * Scroll vertically as necessary and reveal lines centered vertically only if it lies outside the viewport.
     */
    revealLinesInCenterIfOutsideViewport(
      lineNumber: number,
      endLineNumber: number,
      scrollType?: ScrollType
    ): void;
    /**
     * Scroll vertically or horizontally as necessary and reveal a range.
     */
    revealRange(range: IRange, scrollType?: ScrollType): void;
    /**
     * Scroll vertically or horizontally as necessary and reveal a range centered vertically.
     */
    revealRangeInCenter(range: IRange, scrollType?: ScrollType): void;
    /**
     * Scroll vertically or horizontally as necessary and reveal a range at the top of the viewport.
     */
    revealRangeAtTop(range: IRange, scrollType?: ScrollType): void;
    /**
     * Scroll vertically or horizontally as necessary and reveal a range centered vertically only if it lies outside the viewport.
     */
    revealRangeInCenterIfOutsideViewport(
      range: IRange,
      scrollType?: ScrollType
    ): void;
    /**
     * Directly trigger a handler or an editor action.
     * @param source The source of the call.
     * @param handlerId The id of the handler or the id of a contribution.
     * @param payload Extra data to be sent to the handler.
     */
    trigger(source: string, handlerId: string, payload: any): void;
    /**
     * Gets the current model attached to this editor.
     */
    getModel(): IEditorModel | null;
    /**
     * Sets the current model attached to this editor.
     * If the previous model was created by the editor via the value key in the options
     * literal object, it will be destroyed. Otherwise, if the previous model was set
     * via setModel, or the model key in the options literal object, the previous model
     * will not be destroyed.
     * It is safe to call setModel(null) to simply detach the current model from the editor.
     */
    setModel(model: IEditorModel | null): void;
  }

  /**
   * An editor contribution that gets created every time a new editor gets created and gets disposed when the editor gets disposed.
   */
  export interface IEditorContribution {
    /**
     * Dispose this contribution.
     */
    dispose(): void;
    /**
     * Store view state.
     */
    saveViewState?: () => any;
    /**
     * Restore view state.
     */
    restoreViewState?: (state: any) => void;
  }

  // /**
  //  * The type of the `IEditor`.
  //  */
  // export const EditorType: {
  //   ICodeEditor: string;
  //   IDiffEditor: string;
  // };

  /**
   * An event describing that the current mode associated with a model has changed.
   */
  export interface IModelLanguageChangedEvent {
    /**
     * Previous language
     */
    readonly oldLanguage: string;
    /**
     * New language
     */
    readonly newLanguage: string;
  }

  export interface IModelContentChange {
    /**
     * The range that got replaced.
     */
    readonly range: IRange;
    /**
     * The offset of the range that got replaced.
     */
    readonly rangeOffset: number;
    /**
     * The length of the range that got replaced.
     */
    readonly rangeLength: number;
    /**
     * The new text for the range.
     */
    readonly text: string;
  }

  /**
   * An event describing a change in the text of a model.
   */
  export interface IModelContentChangedEvent {
    readonly changes: IModelContentChange[];
    /**
     * The (new) end-of-line character.
     */
    readonly eol: string;
    /**
     * The new version id the model has transitioned to.
     */
    readonly versionId: number;
    /**
     * Flag that indicates that this event was generated while undoing.
     */
    readonly isUndoing: boolean;
    /**
     * Flag that indicates that this event was generated while redoing.
     */
    readonly isRedoing: boolean;
    /**
     * Flag that indicates that all decorations were lost with this edit.
     * The model has been reset to a new value.
     */
    readonly isFlush: boolean;
  }

  export interface IModelOptionsChangedEvent {
    readonly tabSize: boolean;
    readonly indentSize: boolean;
    readonly insertSpaces: boolean;
    readonly trimAutoWhitespace: boolean;
  }

  /**
   * Describes the reason the cursor has changed its position.
   */
  export enum CursorChangeReason {
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

  /**
   * An event describing that the cursor position has changed.
   */
  export interface ICursorPositionChangedEvent {
    /**
     * Primary cursor's position.
     */
    readonly position: Position;
    /**
     * Secondary cursors' position.
     */
    readonly secondaryPositions: Position[];
    /**
     * Reason.
     */
    readonly reason: CursorChangeReason;
    /**
     * Source of the call that caused the event.
     */
    readonly source: string;
  }

  /**
   * An event describing that the cursor selection has changed.
   */
  export interface ICursorSelectionChangedEvent {
    /**
     * The primary selection.
     */
    readonly selection: Selection;
    /**
     * The secondary selections.
     */
    readonly secondarySelections: Selection[];
    /**
     * The model version id.
     */
    readonly modelVersionId: number;
    /**
     * The old selections.
     */
    readonly oldSelections: Selection[] | null;
    /**
     * The model version id the that `oldSelections` refer to.
     */
    readonly oldModelVersionId: number;
    /**
     * Source of the call that caused the event.
     */
    readonly source: string;
    /**
     * Reason.
     */
    readonly reason: CursorChangeReason;
  }

  export enum AccessibilitySupport {
    /**
     * This should be the browser case where it is not known if a screen reader is attached or no.
     */
    Unknown = 0,
    Disabled = 1,
    Enabled = 2,
  }

  /**
   * Configuration options for auto closing quotes and brackets
   */
  export type EditorAutoClosingStrategy =
    | "always"
    | "languageDefined"
    | "beforeWhitespace"
    | "never";

  /**
   * Configuration options for auto wrapping quotes and brackets
   */
  export type EditorAutoSurroundStrategy =
    | "languageDefined"
    | "quotes"
    | "brackets"
    | "never";

  /**
   * Configuration options for typing over closing quotes or brackets
   */
  export type EditorAutoClosingOvertypeStrategy = "always" | "auto" | "never";

  /**
   * Configuration options for auto indentation in the editor
   */
  export enum EditorAutoIndentStrategy {
    None = 0,
    Keep = 1,
    Brackets = 2,
    Advanced = 3,
    Full = 4,
  }

  /**
   * Configuration options for the editor.
   */
  export interface IEditorOptions {
    /**
     * This editor is used inside a diff editor.
     */
    inDiffEditor?: boolean;
    /**
     * The aria label for the editor's textarea (when it is focused).
     */
    ariaLabel?: string;
    /**
     * Render vertical lines at the specified columns.
     * Defaults to empty array.
     */
    rulers?: number[];
    /**
     * A string containing the word separators used when doing word navigation.
     * Defaults to `~!@#$%^&*()-=+[{]}\\|;:\'",.<>/?
     */
    wordSeparators?: string;
    /**
     * Enable Linux primary clipboard.
     * Defaults to true.
     */
    selectionClipboard?: boolean;
    /**
     * Control the rendering of line numbers.
     * If it is a function, it will be invoked when rendering a line number and the return value will be rendered.
     * Otherwise, if it is a truey, line numbers will be rendered normally (equivalent of using an identity function).
     * Otherwise, line numbers will not be rendered.
     * Defaults to `on`.
     */
    lineNumbers?: LineNumbersType;
    /**
     * Controls the minimal number of visible leading and trailing lines surrounding the cursor.
     * Defaults to 0.
     */
    cursorSurroundingLines?: number;
    /**
     * Controls when `cursorSurroundingLines` should be enforced
     * Defaults to `default`, `cursorSurroundingLines` is not enforced when cursor position is changed
     * by mouse.
     */
    cursorSurroundingLinesStyle?: "default" | "all";
    /**
     * Render last line number when the file ends with a newline.
     * Defaults to true.
     */
    renderFinalNewline?: boolean;
    /**
     * Should the corresponding line be selected when clicking on the line number?
     * Defaults to true.
     */
    selectOnLineNumbers?: boolean;
    /**
     * Control the width of line numbers, by reserving horizontal space for rendering at least an amount of digits.
     * Defaults to 5.
     */
    lineNumbersMinChars?: number;
    /**
     * Enable the rendering of the glyph margin.
     * Defaults to true in vscode and to false in monaco-editor.
     */
    glyphMargin?: boolean;
    /**
     * The width reserved for line decorations (in px).
     * Line decorations are placed between line numbers and the editor content.
     * You can pass in a string in the format floating point followed by "ch". e.g. 1.3ch.
     * Defaults to 10.
     */
    lineDecorationsWidth?: number | string;
    /**
     * When revealing the cursor, a virtual padding (px) is added to the cursor, turning it into a rectangle.
     * This virtual padding ensures that the cursor gets revealed before hitting the edge of the viewport.
     * Defaults to 30 (px).
     */
    revealHorizontalRightPadding?: number;
    /**
     * Render the editor selection with rounded borders.
     * Defaults to true.
     */
    roundedSelection?: boolean;
    /**
     * Class name to be added to the editor.
     */
    extraEditorClassName?: string;
    /**
     * Should the editor be read only.
     * Defaults to false.
     */
    readOnly?: boolean;
    /**
     * Should the editor render validation decorations.
     * Defaults to editable.
     */
    renderValidationDecorations?: "editable" | "on" | "off";
    /**
     * Control the behavior and rendering of the scrollbars.
     */
    scrollbar?: IEditorScrollbarOptions;
    /**
     * Control the behavior and rendering of the minimap.
     */
    minimap?: IEditorMinimapOptions;
    /**
     * Control the behavior of the find widget.
     */
    find?: IEditorFindOptions;
    /**
     * Display overflow widgets as `fixed`.
     * Defaults to `false`.
     */
    fixedOverflowWidgets?: boolean;
    /**
     * The number of vertical lanes the overview ruler should render.
     * Defaults to 3.
     */
    overviewRulerLanes?: number;
    /**
     * Controls if a border should be drawn around the overview ruler.
     * Defaults to `true`.
     */
    overviewRulerBorder?: boolean;
    /**
     * Control the cursor animation style, possible values are 'blink', 'smooth', 'phase', 'expand' and 'solid'.
     * Defaults to 'blink'.
     */
    cursorBlinking?: "blink" | "smooth" | "phase" | "expand" | "solid";
    /**
     * Zoom the font in the editor when using the mouse wheel in combination with holding Ctrl.
     * Defaults to false.
     */
    mouseWheelZoom?: boolean;
    /**
     * Control the mouse pointer style, either 'text' or 'default' or 'copy'
     * Defaults to 'text'
     */
    mouseStyle?: "text" | "default" | "copy";
    /**
     * Enable smooth caret animation.
     * Defaults to false.
     */
    cursorSmoothCaretAnimation?: boolean;
    /**
     * Control the cursor style, either 'block' or 'line'.
     * Defaults to 'line'.
     */
    cursorStyle?:
      | "line"
      | "block"
      | "underline"
      | "line-thin"
      | "block-outline"
      | "underline-thin";
    /**
     * Control the width of the cursor when cursorStyle is set to 'line'
     */
    cursorWidth?: number;
    /**
     * Enable font ligatures.
     * Defaults to false.
     */
    fontLigatures?: boolean | string;
    /**
     * Disable the use of `transform: translate3d(0px, 0px, 0px)` for the editor margin and lines layers.
     * The usage of `transform: translate3d(0px, 0px, 0px)` acts as a hint for browsers to create an extra layer.
     * Defaults to false.
     */
    disableLayerHinting?: boolean;
    /**
     * Disable the optimizations for monospace fonts.
     * Defaults to false.
     */
    disableMonospaceOptimizations?: boolean;
    /**
     * Should the cursor be hidden in the overview ruler.
     * Defaults to false.
     */
    hideCursorInOverviewRuler?: boolean;
    /**
     * Enable that scrolling can go one screen size after the last line.
     * Defaults to true.
     */
    scrollBeyondLastLine?: boolean;
    /**
     * Enable that scrolling can go beyond the last column by a number of columns.
     * Defaults to 5.
     */
    scrollBeyondLastColumn?: number;
    /**
     * Enable that the editor animates scrolling to a position.
     * Defaults to false.
     */
    smoothScrolling?: boolean;
    /**
     * Enable that the editor will install an interval to check if its container dom node size has changed.
     * Enabling this might have a severe performance impact.
     * Defaults to false.
     */
    automaticLayout?: boolean;
    /**
     * Control the wrapping of the editor.
     * When `wordWrap` = "off", the lines will never wrap.
     * When `wordWrap` = "on", the lines will wrap at the viewport width.
     * When `wordWrap` = "wordWrapColumn", the lines will wrap at `wordWrapColumn`.
     * When `wordWrap` = "bounded", the lines will wrap at min(viewport width, wordWrapColumn).
     * Defaults to "off".
     */
    wordWrap?: "off" | "on" | "wordWrapColumn" | "bounded";
    /**
     * Control the wrapping of the editor.
     * When `wordWrap` = "off", the lines will never wrap.
     * When `wordWrap` = "on", the lines will wrap at the viewport width.
     * When `wordWrap` = "wordWrapColumn", the lines will wrap at `wordWrapColumn`.
     * When `wordWrap` = "bounded", the lines will wrap at min(viewport width, wordWrapColumn).
     * Defaults to 80.
     */
    wordWrapColumn?: number;
    /**
     * Force word wrapping when the text appears to be of a minified/generated file.
     * Defaults to true.
     */
    wordWrapMinified?: boolean;
    /**
     * Control indentation of wrapped lines. Can be: 'none', 'same', 'indent' or 'deepIndent'.
     * Defaults to 'same' in vscode and to 'none' in monaco-editor.
     */
    wrappingIndent?: "none" | "same" | "indent" | "deepIndent";
    /**
     * Controls the wrapping strategy to use.
     * Defaults to 'simple'.
     */
    wrappingStrategy?: "simple" | "advanced";
    /**
     * Configure word wrapping characters. A break will be introduced before these characters.
     * Defaults to '([{‘“〈《「『【〔（［｛｢£¥＄￡￥+＋'.
     */
    wordWrapBreakBeforeCharacters?: string;
    /**
     * Configure word wrapping characters. A break will be introduced after these characters.
     * Defaults to ' \t})]?|/&.,;¢°′″‰℃、。｡､￠，．：；？！％・･ゝゞヽヾーァィゥェォッャュョヮヵヶぁぃぅぇぉっゃゅょゎゕゖㇰㇱㇲㇳㇴㇵㇶㇷㇸㇹㇺㇻㇼㇽㇾㇿ々〻ｧｨｩｪｫｬｭｮｯｰ”〉》」』】〕）］｝｣'.
     */
    wordWrapBreakAfterCharacters?: string;
    /**
     * Performance guard: Stop rendering a line after x characters.
     * Defaults to 10000.
     * Use -1 to never stop rendering
     */
    stopRenderingLineAfter?: number;
    /**
     * Configure the editor's hover.
     */
    hover?: IEditorHoverOptions;
    /**
     * Enable detecting links and making them clickable.
     * Defaults to true.
     */
    links?: boolean;
    /**
     * Enable inline color decorators and color picker rendering.
     */
    colorDecorators?: boolean;
    /**
     * Control the behaviour of comments in the editor.
     */
    comments?: IEditorCommentsOptions;
    /**
     * Enable custom contextmenu.
     * Defaults to true.
     */
    contextmenu?: boolean;
    /**
     * A multiplier to be used on the `deltaX` and `deltaY` of mouse wheel scroll events.
     * Defaults to 1.
     */
    mouseWheelScrollSensitivity?: number;
    /**
     * FastScrolling mulitplier speed when pressing `Alt`
     * Defaults to 5.
     */
    fastScrollSensitivity?: number;
    /**
     * The modifier to be used to add multiple cursors with the mouse.
     * Defaults to 'alt'
     */
    multiCursorModifier?: "ctrlCmd" | "alt";
    /**
     * Merge overlapping selections.
     * Defaults to true
     */
    multiCursorMergeOverlapping?: boolean;
    /**
     * Configure the behaviour when pasting a text with the line count equal to the cursor count.
     * Defaults to 'spread'.
     */
    multiCursorPaste?: "spread" | "full";
    /**
     * Configure the editor's accessibility support.
     * Defaults to 'auto'. It is best to leave this to 'auto'.
     */
    accessibilitySupport?: "auto" | "off" | "on";
    /**
     * Controls the number of lines in the editor that can be read out by a screen reader
     */
    accessibilityPageSize?: number;
    /**
     * Suggest options.
     */
    suggest?: ISuggestOptions;
    /**
     *
     */
    gotoLocation?: IGotoLocationOptions;
    /**
     * Enable quick suggestions (shadow suggestions)
     * Defaults to true.
     */
    quickSuggestions?: boolean | IQuickSuggestionsOptions;
    /**
     * Quick suggestions show delay (in ms)
     * Defaults to 10 (ms)
     */
    quickSuggestionsDelay?: number;
    /**
     * Parameter hint options.
     */
    parameterHints?: IEditorParameterHintOptions;
    /**
     * Options for auto closing brackets.
     * Defaults to language defined behavior.
     */
    autoClosingBrackets?: EditorAutoClosingStrategy;
    /**
     * Options for auto closing quotes.
     * Defaults to language defined behavior.
     */
    autoClosingQuotes?: EditorAutoClosingStrategy;
    /**
     * Options for typing over closing quotes or brackets.
     */
    autoClosingOvertype?: EditorAutoClosingOvertypeStrategy;
    /**
     * Options for auto surrounding.
     * Defaults to always allowing auto surrounding.
     */
    autoSurround?: EditorAutoSurroundStrategy;
    /**
     * Controls whether the editor should automatically adjust the indentation when users type, paste, move or indent lines.
     * Defaults to advanced.
     */
    autoIndent?: "none" | "keep" | "brackets" | "advanced" | "full";
    /**
     * Enable format on type.
     * Defaults to false.
     */
    formatOnType?: boolean;
    /**
     * Enable format on paste.
     * Defaults to false.
     */
    formatOnPaste?: boolean;
    /**
     * Controls if the editor should allow to move selections via drag and drop.
     * Defaults to false.
     */
    dragAndDrop?: boolean;
    /**
     * Enable the suggestion box to pop-up on trigger characters.
     * Defaults to true.
     */
    suggestOnTriggerCharacters?: boolean;
    /**
     * Accept suggestions on ENTER.
     * Defaults to 'on'.
     */
    acceptSuggestionOnEnter?: "on" | "smart" | "off";
    /**
     * Accept suggestions on provider defined characters.
     * Defaults to true.
     */
    acceptSuggestionOnCommitCharacter?: boolean;
    /**
     * Enable snippet suggestions. Default to 'true'.
     */
    snippetSuggestions?: "top" | "bottom" | "inline" | "none";
    /**
     * Copying without a selection copies the current line.
     */
    emptySelectionClipboard?: boolean;
    /**
     * Syntax highlighting is copied.
     */
    copyWithSyntaxHighlighting?: boolean;
    /**
     * The history mode for suggestions.
     */
    suggestSelection?: "first" | "recentlyUsed" | "recentlyUsedByPrefix";
    /**
     * The font size for the suggest widget.
     * Defaults to the editor font size.
     */
    suggestFontSize?: number;
    /**
     * The line height for the suggest widget.
     * Defaults to the editor line height.
     */
    suggestLineHeight?: number;
    /**
     * Enable tab completion.
     */
    tabCompletion?: "on" | "off" | "onlySnippets";
    /**
     * Enable selection highlight.
     * Defaults to true.
     */
    selectionHighlight?: boolean;
    /**
     * Enable semantic occurrences highlight.
     * Defaults to true.
     */
    occurrencesHighlight?: boolean;
    /**
     * Show code lens
     * Defaults to true.
     */
    codeLens?: boolean;
    /**
     * Control the behavior and rendering of the code action lightbulb.
     */
    lightbulb?: IEditorLightbulbOptions;
    /**
     * Timeout for running code actions on save.
     */
    codeActionsOnSaveTimeout?: number;
    /**
     * Enable code folding.
     * Defaults to true.
     */
    folding?: boolean;
    /**
     * Selects the folding strategy. 'auto' uses the strategies contributed for the current document, 'indentation' uses the indentation based folding strategy.
     * Defaults to 'auto'.
     */
    foldingStrategy?: "auto" | "indentation";
    /**
     * Enable highlight for folded regions.
     * Defaults to true.
     */
    foldingHighlight?: boolean;
    /**
     * Controls whether the fold actions in the gutter stay always visible or hide unless the mouse is over the gutter.
     * Defaults to 'mouseover'.
     */
    showFoldingControls?: "always" | "mouseover";
    /**
     * Enable highlighting of matching brackets.
     * Defaults to 'always'.
     */
    matchBrackets?: "never" | "near" | "always";
    /**
     * Enable rendering of whitespace.
     * Defaults to none.
     */
    renderWhitespace?: "none" | "boundary" | "selection" | "all";
    /**
     * Enable rendering of control characters.
     * Defaults to false.
     */
    renderControlCharacters?: boolean;
    /**
     * Enable rendering of indent guides.
     * Defaults to true.
     */
    renderIndentGuides?: boolean;
    /**
     * Enable highlighting of the active indent guide.
     * Defaults to true.
     */
    highlightActiveIndentGuide?: boolean;
    /**
     * Enable rendering of current line highlight.
     * Defaults to all.
     */
    renderLineHighlight?: "none" | "gutter" | "line" | "all";
    /**
     * Inserting and deleting whitespace follows tab stops.
     */
    useTabStops?: boolean;
    /**
     * The font family
     */
    fontFamily?: string;
    /**
     * The font weight
     */
    fontWeight?: string;
    /**
     * The font size
     */
    fontSize?: number;
    /**
     * The line height
     */
    lineHeight?: number;
    /**
     * The letter spacing
     */
    letterSpacing?: number;
    /**
     * Controls fading out of unused variables.
     */
    showUnused?: boolean;
    /**
     * Controls whether to focus the inline editor in the peek widget by default.
     * Defaults to false.
     */
    peekWidgetDefaultFocus?: "tree" | "editor";
  }

  export interface IEditorConstructionOptions extends IEditorOptions {
    /**
     * The initial editor dimension (to avoid measuring the container).
     */
    dimension?: IDimension;
  }

  /**
   * Configuration options for the diff editor.
   */
  export interface IDiffEditorOptions extends IEditorOptions {
    /**
     * Allow the user to resize the diff editor split view.
     * Defaults to true.
     */
    enableSplitViewResizing?: boolean;
    /**
     * Render the differences in two side-by-side editors.
     * Defaults to true.
     */
    renderSideBySide?: boolean;
    /**
     * Timeout in milliseconds after which diff computation is cancelled.
     * Defaults to 5000.
     */
    maxComputationTime?: number;
    /**
     * Compute the diff by ignoring leading/trailing whitespace
     * Defaults to true.
     */
    ignoreTrimWhitespace?: boolean;
    /**
     * Render +/- indicators for added/deleted changes.
     * Defaults to true.
     */
    renderIndicators?: boolean;
    /**
     * Original model should be editable?
     * Defaults to false.
     */
    originalEditable?: boolean;
  }

  /**
   * An event describing that the configuration of the editor has changed.
   */
  export class ConfigurationChangedEvent {
    hasChanged(id: EditorOption): boolean;
  }

  /**
   * All computed editor options.
   */
  export interface IComputedEditorOptions {
    get(id: any): any;
  }

  export interface IEditorOption {
    readonly id: string;
    readonly name: string;
    defaultValue: any;
  }

  /**
   * Configuration options for editor comments
   */
  export interface IEditorCommentsOptions {
    /**
     * Insert a space after the line comment token and inside the block comments tokens.
     * Defaults to true.
     */
    insertSpace?: boolean;
  }

  // export type EditorCommentsOptions = Readonly<
  //   Required<IEditorCommentsOptions>
  // >;

  /**
   * The kind of animation in which the editor's cursor should be rendered.
   */
  export enum TextEditorCursorBlinkingStyle {
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

  /**
   * The style in which the editor's cursor should be rendered.
   */
  export enum TextEditorCursorStyle {
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

  /**
   * Configuration options for editor find widget
   */
  export interface IEditorFindOptions {
    /**
     * Controls if we seed search string in the Find Widget with editor selection.
     */
    seedSearchStringFromSelection?: boolean;
    /**
     * Controls if Find in Selection flag is turned on in the editor.
     */
    autoFindInSelection?: "never" | "always" | "multiline";
    addExtraSpaceOnTop?: boolean;
  }

  // export type EditorFindOptions = Readonly<Required<IEditorFindOptions>>;

  export type GoToLocationValues = "peek" | "gotoAndPeek" | "goto";

  /**
   * Configuration options for go to location
   */
  export interface IGotoLocationOptions {
    multiple?: GoToLocationValues;
    multipleDefinitions?: GoToLocationValues;
    multipleTypeDefinitions?: GoToLocationValues;
    multipleDeclarations?: GoToLocationValues;
    multipleImplementations?: GoToLocationValues;
    multipleReferences?: GoToLocationValues;
    alternativeDefinitionCommand?: string;
    alternativeTypeDefinitionCommand?: string;
    alternativeDeclarationCommand?: string;
    alternativeImplementationCommand?: string;
    alternativeReferenceCommand?: string;
  }

  // export type GoToLocationOptions = Readonly<Required<IGotoLocationOptions>>;

  /**
   * Configuration options for editor hover
   */
  export interface IEditorHoverOptions {
    /**
     * Enable the hover.
     * Defaults to true.
     */
    enabled?: boolean;
    /**
     * Delay for showing the hover.
     * Defaults to 300.
     */
    delay?: number;
    /**
     * Is the hover sticky such that it can be clicked and its contents selected?
     * Defaults to true.
     */
    sticky?: boolean;
  }

  // export type EditorHoverOptions = Readonly<Required<IEditorHoverOptions>>;

  /**
   * A description for the overview ruler position.
   */
  export interface OverviewRulerPosition {
    /**
     * Width of the overview ruler
     */
    readonly width: number;
    /**
     * Height of the overview ruler
     */
    readonly height: number;
    /**
     * Top position for the overview ruler
     */
    readonly top: number;
    /**
     * Right position for the overview ruler
     */
    readonly right: number;
  }

  export enum RenderMinimap {
    None = 0,
    Text = 1,
    Blocks = 2,
  }

  /**
   * The internal layout details of the editor.
   */
  export interface EditorLayoutInfo {
    /**
     * Full editor width.
     */
    readonly width: number;
    /**
     * Full editor height.
     */
    readonly height: number;
    /**
     * Left position for the glyph margin.
     */
    readonly glyphMarginLeft: number;
    /**
     * The width of the glyph margin.
     */
    readonly glyphMarginWidth: number;
    /**
     * Left position for the line numbers.
     */
    readonly lineNumbersLeft: number;
    /**
     * The width of the line numbers.
     */
    readonly lineNumbersWidth: number;
    /**
     * Left position for the line decorations.
     */
    readonly decorationsLeft: number;
    /**
     * The width of the line decorations.
     */
    readonly decorationsWidth: number;
    /**
     * Left position for the content (actual text)
     */
    readonly contentLeft: number;
    /**
     * The width of the content (actual text)
     */
    readonly contentWidth: number;
    /**
     * The position for the minimap
     */
    readonly minimapLeft: number;
    /**
     * The width of the minimap
     */
    readonly minimapWidth: number;
    /**
     * Minimap render type
     */
    readonly renderMinimap: RenderMinimap;
    /**
     * The number of columns (of typical characters) fitting on a viewport line.
     */
    readonly viewportColumn: number;
    /**
     * The width of the vertical scrollbar.
     */
    readonly verticalScrollbarWidth: number;
    /**
     * The height of the horizontal scrollbar.
     */
    readonly horizontalScrollbarHeight: number;
    /**
     * The position of the overview ruler.
     */
    readonly overviewRuler: OverviewRulerPosition;
  }

  /**
   * Configuration options for editor lightbulb
   */
  export interface IEditorLightbulbOptions {
    /**
     * Enable the lightbulb code action.
     * Defaults to true.
     */
    enabled?: boolean;
  }

  // export type EditorLightbulbOptions = Readonly<
  //   Required<IEditorLightbulbOptions>
  // >;

  /**
   * Configuration options for editor minimap
   */
  export interface IEditorMinimapOptions {
    /**
     * Enable the rendering of the minimap.
     * Defaults to true.
     */
    enabled?: boolean;
    /**
     * Control the side of the minimap in editor.
     * Defaults to 'right'.
     */
    side?: "right" | "left";
    /**
     * Control the rendering of the minimap slider.
     * Defaults to 'mouseover'.
     */
    showSlider?: "always" | "mouseover";
    /**
     * Render the actual text on a line (as opposed to color blocks).
     * Defaults to true.
     */
    renderCharacters?: boolean;
    /**
     * Limit the width of the minimap to render at most a certain number of columns.
     * Defaults to 120.
     */
    maxColumn?: number;
    /**
     * Relative size of the font in the minimap. Defaults to 1.
     */
    scale?: number;
  }

  // export type EditorMinimapOptions = Readonly<Required<IEditorMinimapOptions>>;

  /**
   * Configuration options for parameter hints
   */
  export interface IEditorParameterHintOptions {
    /**
     * Enable parameter hints.
     * Defaults to true.
     */
    enabled?: boolean;
    /**
     * Enable cycling of parameter hints.
     * Defaults to false.
     */
    cycle?: boolean;
  }

  // export type InternalParameterHintOptions = Readonly<
  //   Required<IEditorParameterHintOptions>
  // >;

  /**
   * Configuration options for quick suggestions
   */
  export interface IQuickSuggestionsOptions {
    other: boolean;
    comments: boolean;
    strings: boolean;
  }

  // export type ValidQuickSuggestionsOptions =
  //   | boolean
  //   | Readonly<Required<IQuickSuggestionsOptions>>;

  export type LineNumbersType =
    | "on"
    | "off"
    | "relative"
    | "interval"
    | ((lineNumber: number) => string);

  export enum RenderLineNumbersType {
    Off = 0,
    On = 1,
    Relative = 2,
    Interval = 3,
    Custom = 4,
  }

  export interface InternalEditorRenderLineNumbersOptions {
    readonly renderType: RenderLineNumbersType;
    readonly renderFn: ((lineNumber: number) => string) | null;
  }

  /**
   * Configuration options for editor scrollbars
   */
  export interface IEditorScrollbarOptions {
    /**
     * The size of arrows (if displayed).
     * Defaults to 11.
     */
    arrowSize?: number;
    /**
     * Render vertical scrollbar.
     * Defaults to 'auto'.
     */
    vertical?: "auto" | "visible" | "hidden";
    /**
     * Render horizontal scrollbar.
     * Defaults to 'auto'.
     */
    horizontal?: "auto" | "visible" | "hidden";
    /**
     * Cast horizontal and vertical shadows when the content is scrolled.
     * Defaults to true.
     */
    useShadows?: boolean;
    /**
     * Render arrows at the top and bottom of the vertical scrollbar.
     * Defaults to false.
     */
    verticalHasArrows?: boolean;
    /**
     * Render arrows at the left and right of the horizontal scrollbar.
     * Defaults to false.
     */
    horizontalHasArrows?: boolean;
    /**
     * Listen to mouse wheel events and react to them by scrolling.
     * Defaults to true.
     */
    handleMouseWheel?: boolean;
    /**
     * Always consume mouse wheel events (always call preventDefault() and stopPropagation() on the browser events).
     * Defaults to true.
     */
    alwaysConsumeMouseWheel?: boolean;
    /**
     * Height in pixels for the horizontal scrollbar.
     * Defaults to 10 (px).
     */
    horizontalScrollbarSize?: number;
    /**
     * Width in pixels for the vertical scrollbar.
     * Defaults to 10 (px).
     */
    verticalScrollbarSize?: number;
    /**
     * Width in pixels for the vertical slider.
     * Defaults to `verticalScrollbarSize`.
     */
    verticalSliderSize?: number;
    /**
     * Height in pixels for the horizontal slider.
     * Defaults to `horizontalScrollbarSize`.
     */
    horizontalSliderSize?: number;
  }

  export interface InternalEditorScrollbarOptions {
    readonly arrowSize: number;
    readonly vertical: ScrollbarVisibility;
    readonly horizontal: ScrollbarVisibility;
    readonly useShadows: boolean;
    readonly verticalHasArrows: boolean;
    readonly horizontalHasArrows: boolean;
    readonly handleMouseWheel: boolean;
    readonly alwaysConsumeMouseWheel: boolean;
    readonly horizontalScrollbarSize: number;
    readonly horizontalSliderSize: number;
    readonly verticalScrollbarSize: number;
    readonly verticalSliderSize: number;
  }

  /**
   * Configuration options for editor suggest widget
   */
  export interface ISuggestOptions {
    /**
     * Overwrite word ends on accept. Default to false.
     */
    insertMode?: "insert" | "replace";
    /**
     * Show a highlight when suggestion replaces or keep text after the cursor. Defaults to false.
     */
    insertHighlight?: boolean;
    /**
     * Enable graceful matching. Defaults to true.
     */
    filterGraceful?: boolean;
    /**
     * Prevent quick suggestions when a snippet is active. Defaults to true.
     */
    snippetsPreventQuickSuggestions?: boolean;
    /**
     * Favours words that appear close to the cursor.
     */
    localityBonus?: boolean;
    /**
     * Enable using global storage for remembering suggestions.
     */
    shareSuggestSelections?: boolean;
    /**
     * Enable or disable icons in suggestions. Defaults to true.
     */
    showIcons?: boolean;
    /**
     * Max suggestions to show in suggestions. Defaults to 12.
     */
    maxVisibleSuggestions?: number;
    /**
     * Show method-suggestions.
     */
    showMethods?: boolean;
    /**
     * Show function-suggestions.
     */
    showFunctions?: boolean;
    /**
     * Show constructor-suggestions.
     */
    showConstructors?: boolean;
    /**
     * Show field-suggestions.
     */
    showFields?: boolean;
    /**
     * Show variable-suggestions.
     */
    showVariables?: boolean;
    /**
     * Show class-suggestions.
     */
    showClasses?: boolean;
    /**
     * Show struct-suggestions.
     */
    showStructs?: boolean;
    /**
     * Show interface-suggestions.
     */
    showInterfaces?: boolean;
    /**
     * Show module-suggestions.
     */
    showModules?: boolean;
    /**
     * Show property-suggestions.
     */
    showProperties?: boolean;
    /**
     * Show event-suggestions.
     */
    showEvents?: boolean;
    /**
     * Show operator-suggestions.
     */
    showOperators?: boolean;
    /**
     * Show unit-suggestions.
     */
    showUnits?: boolean;
    /**
     * Show value-suggestions.
     */
    showValues?: boolean;
    /**
     * Show constant-suggestions.
     */
    showConstants?: boolean;
    /**
     * Show enum-suggestions.
     */
    showEnums?: boolean;
    /**
     * Show enumMember-suggestions.
     */
    showEnumMembers?: boolean;
    /**
     * Show keyword-suggestions.
     */
    showKeywords?: boolean;
    /**
     * Show text-suggestions.
     */
    showWords?: boolean;
    /**
     * Show color-suggestions.
     */
    showColors?: boolean;
    /**
     * Show file-suggestions.
     */
    showFiles?: boolean;
    /**
     * Show reference-suggestions.
     */
    showReferences?: boolean;
    /**
     * Show folder-suggestions.
     */
    showFolders?: boolean;
    /**
     * Show typeParameter-suggestions.
     */
    showTypeParameters?: boolean;
    /**
     * Show snippet-suggestions.
     */
    showSnippets?: boolean;
    /**
     * Controls the visibility of the status bar at the bottom of the suggest widget.
     */
    hideStatusBar?: boolean;
  }

  // export type InternalSuggestOptions = Readonly<Required<ISuggestOptions>>;

  /**
   * Describes how to indent wrapped lines.
   */
  export enum WrappingIndent {
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

  export interface EditorWrappingInfo {
    readonly isDominatedByLongLines: boolean;
    readonly isWordWrapMinified: boolean;
    readonly isViewportWrapping: boolean;
    readonly wrappingColumn: number;
  }

  export enum EditorOption {
    acceptSuggestionOnCommitCharacter = 0,
    acceptSuggestionOnEnter = 1,
    accessibilitySupport = 2,
    accessibilityPageSize = 3,
    ariaLabel = 4,
    autoClosingBrackets = 5,
    autoClosingOvertype = 6,
    autoClosingQuotes = 7,
    autoIndent = 8,
    automaticLayout = 9,
    autoSurround = 10,
    codeLens = 11,
    colorDecorators = 12,
    comments = 13,
    contextmenu = 14,
    copyWithSyntaxHighlighting = 15,
    cursorBlinking = 16,
    cursorSmoothCaretAnimation = 17,
    cursorStyle = 18,
    cursorSurroundingLines = 19,
    cursorSurroundingLinesStyle = 20,
    cursorWidth = 21,
    disableLayerHinting = 22,
    disableMonospaceOptimizations = 23,
    dragAndDrop = 24,
    emptySelectionClipboard = 25,
    extraEditorClassName = 26,
    fastScrollSensitivity = 27,
    find = 28,
    fixedOverflowWidgets = 29,
    folding = 30,
    foldingStrategy = 31,
    foldingHighlight = 32,
    fontFamily = 33,
    fontInfo = 34,
    fontLigatures = 35,
    fontSize = 36,
    fontWeight = 37,
    formatOnPaste = 38,
    formatOnType = 39,
    glyphMargin = 40,
    gotoLocation = 41,
    hideCursorInOverviewRuler = 42,
    highlightActiveIndentGuide = 43,
    hover = 44,
    inDiffEditor = 45,
    letterSpacing = 46,
    lightbulb = 47,
    lineDecorationsWidth = 48,
    lineHeight = 49,
    lineNumbers = 50,
    lineNumbersMinChars = 51,
    links = 52,
    matchBrackets = 53,
    minimap = 54,
    mouseStyle = 55,
    mouseWheelScrollSensitivity = 56,
    mouseWheelZoom = 57,
    multiCursorMergeOverlapping = 58,
    multiCursorModifier = 59,
    multiCursorPaste = 60,
    occurrencesHighlight = 61,
    overviewRulerBorder = 62,
    overviewRulerLanes = 63,
    parameterHints = 64,
    peekWidgetDefaultFocus = 65,
    quickSuggestions = 66,
    quickSuggestionsDelay = 67,
    readOnly = 68,
    renderControlCharacters = 69,
    renderIndentGuides = 70,
    renderFinalNewline = 71,
    renderLineHighlight = 72,
    renderValidationDecorations = 73,
    renderWhitespace = 74,
    revealHorizontalRightPadding = 75,
    roundedSelection = 76,
    rulers = 77,
    scrollbar = 78,
    scrollBeyondLastColumn = 79,
    scrollBeyondLastLine = 80,
    selectionClipboard = 81,
    selectionHighlight = 82,
    selectOnLineNumbers = 83,
    showFoldingControls = 84,
    showUnused = 85,
    snippetSuggestions = 86,
    smoothScrolling = 87,
    stopRenderingLineAfter = 88,
    suggest = 89,
    suggestFontSize = 90,
    suggestLineHeight = 91,
    suggestOnTriggerCharacters = 92,
    suggestSelection = 93,
    tabCompletion = 94,
    useTabStops = 95,
    wordSeparators = 96,
    wordWrap = 97,
    wordWrapBreakAfterCharacters = 98,
    wordWrapBreakBeforeCharacters = 99,
    wordWrapColumn = 100,
    wordWrapMinified = 101,
    wrappingIndent = 102,
    wrappingStrategy = 103,
    editorClassName = 104,
    pixelRatio = 105,
    tabFocusMode = 106,
    layoutInfo = 107,
    wrappingInfo = 108,
  }
  // export const EditorOptions: {
  //   acceptSuggestionOnCommitCharacter: IEditorOption;
  //   acceptSuggestionOnEnter: IEditorOption;
  //   accessibilitySupport: IEditorOption;
  //   accessibilityPageSize: IEditorOption;
  //   ariaLabel: IEditorOption;
  //   autoClosingBrackets: IEditorOption;
  //   autoClosingOvertype: IEditorOption;
  //   autoClosingQuotes: IEditorOption;
  //   autoIndent: IEditorOption;
  //   automaticLayout: IEditorOption;
  //   autoSurround: IEditorOption;
  //   codeLens: IEditorOption;
  //   colorDecorators: IEditorOption;
  //   comments: IEditorOption;
  //   contextmenu: IEditorOption;
  //   copyWithSyntaxHighlighting: IEditorOption;
  //   cursorBlinking: IEditorOption;
  //   cursorSmoothCaretAnimation: IEditorOption;
  //   cursorStyle: IEditorOption;
  //   cursorSurroundingLines: IEditorOption;
  //   cursorSurroundingLinesStyle: IEditorOption;
  //   cursorWidth: IEditorOption;
  //   disableLayerHinting: IEditorOption;
  //   disableMonospaceOptimizations: IEditorOption;
  //   dragAndDrop: IEditorOption;
  //   emptySelectionClipboard: IEditorOption;
  //   extraEditorClassName: IEditorOption;
  //   fastScrollSensitivity: IEditorOption;
  //   find: IEditorOption;
  //   fixedOverflowWidgets: IEditorOption;
  //   folding: IEditorOption;
  //   foldingStrategy: IEditorOption;
  //   foldingHighlight: IEditorOption;
  //   fontFamily: IEditorOption;
  //   fontInfo: IEditorOption;
  //   fontLigatures2: IEditorOption;
  //   fontSize: IEditorOption;
  //   fontWeight: IEditorOption;
  //   formatOnPaste: IEditorOption;
  //   formatOnType: IEditorOption;
  //   glyphMargin: IEditorOption;
  //   gotoLocation: IEditorOption;
  //   hideCursorInOverviewRuler: IEditorOption;
  //   highlightActiveIndentGuide: IEditorOption;
  //   hover: IEditorOption;
  //   inDiffEditor: IEditorOption;
  //   letterSpacing: IEditorOption;
  //   lightbulb: IEditorOption;
  //   lineDecorationsWidth: IEditorOption;
  //   lineHeight: IEditorOption;
  //   lineNumbers: IEditorOption;
  //   lineNumbersMinChars: IEditorOption;
  //   links: IEditorOption;
  //   matchBrackets: IEditorOption;
  //   minimap: IEditorOption;
  //   mouseStyle: IEditorOption;
  //   mouseWheelScrollSensitivity: IEditorOption;
  //   mouseWheelZoom: IEditorOption;
  //   multiCursorMergeOverlapping: IEditorOption;
  //   multiCursorModifier: IEditorOption;
  //   multiCursorPaste: IEditorOption;
  //   occurrencesHighlight: IEditorOption;
  //   overviewRulerBorder: IEditorOption;
  //   overviewRulerLanes: IEditorOption;
  //   parameterHints: IEditorOption;
  //   peekWidgetDefaultFocus: IEditorOption;
  //   quickSuggestions: IEditorOption;
  //   quickSuggestionsDelay: IEditorOption;
  //   readOnly: IEditorOption;
  //   renderControlCharacters: IEditorOption;
  //   renderIndentGuides: IEditorOption;
  //   renderFinalNewline: IEditorOption;
  //   renderLineHighlight: IEditorOption;
  //   renderValidationDecorations: IEditorOption;
  //   renderWhitespace: IEditorOption;
  //   revealHorizontalRightPadding: IEditorOption;
  //   roundedSelection: IEditorOption;
  //   rulers: IEditorOption;
  //   scrollbar: IEditorOption;
  //   scrollBeyondLastColumn: IEditorOption;
  //   scrollBeyondLastLine: IEditorOption;
  //   selectionClipboard: IEditorOption;
  //   selectionHighlight: IEditorOption;
  //   selectOnLineNumbers: IEditorOption;
  //   showFoldingControls: IEditorOption;
  //   showUnused: IEditorOption;
  //   snippetSuggestions: IEditorOption;
  //   smoothScrolling: IEditorOption;
  //   stopRenderingLineAfter: IEditorOption;
  //   suggest: IEditorOption;
  //   suggestFontSize: IEditorOption;
  //   suggestLineHeight: IEditorOption;
  //   suggestOnTriggerCharacters: IEditorOption;
  //   suggestSelection: IEditorOption;
  //   tabCompletion: IEditorOption;
  //   useTabStops: IEditorOption;
  //   wordSeparators: IEditorOption;
  //   wordWrap: IEditorOption;
  //   wordWrapBreakAfterCharacters: IEditorOption;
  //   wordWrapBreakBeforeCharacters: IEditorOption;
  //   wordWrapColumn: IEditorOption;
  //   wordWrapMinified: IEditorOption;
  //   wrappingIndent: IEditorOption;
  //   wrappingStrategy: IEditorOption;
  //   editorClassName: IEditorOption;
  //   pixelRatio: IEditorOption;
  //   tabFocusMode: IEditorOption;
  //   layoutInfo: IEditorOption;
  //   wrappingInfo: IEditorOption;
  // };

  /**
   * A view zone is a full horizontal rectangle that 'pushes' text down.
   * The editor reserves space for view zones when rendering.
   */
  export interface IViewZone {
    /**
     * The line number after which this zone should appear.
     * Use 0 to place a view zone before the first line number.
     */
    afterLineNumber: number;
    /**
     * The column after which this zone should appear.
     * If not set, the maxLineColumn of `afterLineNumber` will be used.
     */
    afterColumn?: number;
    /**
     * Suppress mouse down events.
     * If set, the editor will attach a mouse down listener to the view zone and .preventDefault on it.
     * Defaults to false
     */
    suppressMouseDown?: boolean;
    /**
     * The height in lines of the view zone.
     * If specified, `heightInPx` will be used instead of this.
     * If neither `heightInPx` nor `heightInLines` is specified, a default of `heightInLines` = 1 will be chosen.
     */
    heightInLines?: number;
    /**
     * The height in px of the view zone.
     * If this is set, the editor will give preference to it rather than `heightInLines` above.
     * If neither `heightInPx` nor `heightInLines` is specified, a default of `heightInLines` = 1 will be chosen.
     */
    heightInPx?: number;
    /**
     * The minimum width in px of the view zone.
     * If this is set, the editor will ensure that the scroll width is >= than this value.
     */
    minWidthInPx?: number;
    /**
     * The dom node of the view zone
     */
    domNode: HTMLElement;
    /**
     * An optional dom node for the view zone that will be placed in the margin area.
     */
    marginDomNode?: HTMLElement | null;
    /**
     * Callback which gives the relative top of the view zone as it appears (taking scrolling into account).
     */
    onDomNodeTop?: (top: number) => void;
    /**
     * Callback which gives the height in pixels of the view zone.
     */
    onComputedHeight?: (height: number) => void;
  }

  /**
   * An accessor that allows for zones to be added or removed.
   */
  export interface IViewZoneChangeAccessor {
    /**
     * Create a new view zone.
     * @param zone Zone to create
     * @return A unique identifier to the view zone.
     */
    addZone(zone: IViewZone): string;
    /**
     * Remove a zone
     * @param id A unique identifier to the view zone, as returned by the `addZone` call.
     */
    removeZone(id: string): void;
    /**
     * Change a zone's position.
     * The editor will rescan the `afterLineNumber` and `afterColumn` properties of a view zone.
     */
    layoutZone(id: string): void;
  }

  /**
   * A positioning preference for rendering content widgets.
   */
  export enum ContentWidgetPositionPreference {
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

  /**
   * A position for rendering content widgets.
   */
  export interface IContentWidgetPosition {
    /**
     * Desired position for the content widget.
     * `preference` will also affect the placement.
     */
    position: IPosition | null;
    /**
     * Optionally, a range can be provided to further
     * define the position of the content widget.
     */
    range?: IRange | null;
    /**
     * Placement preference for position, in order of preference.
     */
    preference: ContentWidgetPositionPreference[];
  }

  /**
   * A content widget renders inline with the text and can be easily placed 'near' an editor position.
   */
  export interface IContentWidget {
    /**
     * Render this content widget in a location where it could overflow the editor's view dom node.
     */
    allowEditorOverflow?: boolean;
    suppressMouseDown?: boolean;
    /**
     * Get a unique identifier of the content widget.
     */
    getId(): string;
    /**
     * Get the dom node of the content widget.
     */
    getDomNode(): HTMLElement;
    /**
     * Get the placement of the content widget.
     * If null is returned, the content widget will be placed off screen.
     */
    getPosition(): IContentWidgetPosition | null;
  }

  /**
   * A positioning preference for rendering overlay widgets.
   */
  export enum OverlayWidgetPositionPreference {
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

  /**
   * A position for rendering overlay widgets.
   */
  export interface IOverlayWidgetPosition {
    /**
     * The position preference for the overlay widget.
     */
    preference: OverlayWidgetPositionPreference | null;
  }

  /**
   * An overlay widgets renders on top of the text.
   */
  export interface IOverlayWidget {
    /**
     * Get a unique identifier of the overlay widget.
     */
    getId(): string;
    /**
     * Get the dom node of the overlay widget.
     */
    getDomNode(): HTMLElement;
    /**
     * Get the placement of the overlay widget.
     * If null is returned, the overlay widget is responsible to place itself.
     */
    getPosition(): IOverlayWidgetPosition | null;
  }

  /**
   * Type of hit element with the mouse in the editor.
   */
  export enum MouseTargetType {
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

  /**
   * Target hit with the mouse in the editor.
   */
  export interface IMouseTarget {
    /**
     * The target element
     */
    readonly element: Element | null;
    /**
     * The target type
     */
    readonly type: MouseTargetType;
    /**
     * The 'approximate' editor position
     */
    readonly position: Position | null;
    /**
     * Desired mouse column (e.g. when position.column gets clamped to text length -- clicking after text on a line).
     */
    readonly mouseColumn: number;
    /**
     * The 'approximate' editor range
     */
    readonly range: Range | null;
    /**
     * Some extra detail.
     */
    readonly detail: any;
  }

  /**
   * A mouse event originating from the editor.
   */
  export interface IEditorMouseEvent {
    readonly event: IMouseEvent;
    readonly target: IMouseTarget;
  }

  export interface IPartialEditorMouseEvent {
    readonly event: IMouseEvent;
    readonly target: IMouseTarget | null;
  }

  /**
   * A paste event originating from the editor.
   */
  export interface IPasteEvent {
    readonly range: Range;
    readonly mode: string | null;
  }

  /**
   * A rich code editor.
   */
  export interface ICodeEditor extends IEditor {
    /**
     * An event emitted when the content of the current model has changed.
     * @event
     */
    onDidChangeModelContent(
      listener: (e: IModelContentChangedEvent) => void
    ): IDisposable;
    /**
     * An event emitted when the language of the current model has changed.
     * @event
     */
    onDidChangeModelLanguage(
      listener: (e: IModelLanguageChangedEvent) => void
    ): IDisposable;
    /**
     * An event emitted when the language configuration of the current model has changed.
     * @event
     */
    onDidChangeModelLanguageConfiguration(
      listener: (e: {}) => void
    ): IDisposable;
    /**
     * An event emitted when the options of the current model has changed.
     * @event
     */
    onDidChangeModelOptions(
      listener: (e: IModelOptionsChangedEvent) => void
    ): IDisposable;
    /**
     * An event emitted when the configuration of the editor has changed. (e.g. `editor.updateOptions()`)
     * @event
     */
    onDidChangeConfiguration(
      listener: (e: ConfigurationChangedEvent) => void
    ): IDisposable;
    /**
     * An event emitted when the cursor position has changed.
     * @event
     */
    onDidChangeCursorPosition(
      listener: (e: ICursorPositionChangedEvent) => void
    ): IDisposable;
    /**
     * An event emitted when the cursor selection has changed.
     * @event
     */
    onDidChangeCursorSelection(
      listener: (e: ICursorSelectionChangedEvent) => void
    ): IDisposable;
    /**
     * An event emitted when the model of this editor has changed (e.g. `editor.setModel()`).
     * @event
     */
    onDidChangeModel(listener: (e: IModelChangedEvent) => void): IDisposable;
    /**
     * An event emitted when the decorations of the current model have changed.
     * @event
     */
    onDidChangeModelDecorations(listener: (e: {}) => void): IDisposable;
    /**
     * An event emitted when the text inside this editor gained focus (i.e. cursor starts blinking).
     * @event
     */
    onDidFocusEditorText(listener: () => void): IDisposable;
    /**
     * An event emitted when the text inside this editor lost focus (i.e. cursor stops blinking).
     * @event
     */
    onDidBlurEditorText(listener: () => void): IDisposable;
    /**
     * An event emitted when the text inside this editor or an editor widget gained focus.
     * @event
     */
    onDidFocusEditorWidget(listener: () => void): IDisposable;
    /**
     * An event emitted when the text inside this editor or an editor widget lost focus.
     * @event
     */
    onDidBlurEditorWidget(listener: () => void): IDisposable;
    /**
     * An event emitted after composition has started.
     */
    onDidCompositionStart(listener: () => void): IDisposable;
    /**
     * An event emitted after composition has ended.
     */
    onDidCompositionEnd(listener: () => void): IDisposable;
    /**
     * An event emitted when users paste text in the editor.
     * @event
     */
    onDidPaste(listener: (e: IPasteEvent) => void): IDisposable;
    /**
     * An event emitted on a "mouseup".
     * @event
     */
    onMouseUp(listener: (e: IEditorMouseEvent) => void): IDisposable;
    /**
     * An event emitted on a "mousedown".
     * @event
     */
    onMouseDown(listener: (e: IEditorMouseEvent) => void): IDisposable;
    /**
     * An event emitted on a "contextmenu".
     * @event
     */
    onContextMenu(listener: (e: IEditorMouseEvent) => void): IDisposable;
    /**
     * An event emitted on a "mousemove".
     * @event
     */
    onMouseMove(listener: (e: IEditorMouseEvent) => void): IDisposable;
    /**
     * An event emitted on a "mouseleave".
     * @event
     */
    onMouseLeave(listener: (e: IPartialEditorMouseEvent) => void): IDisposable;
    /**
     * An event emitted on a "keyup".
     * @event
     */
    onKeyUp(listener: (e: IKeyboardEvent) => void): IDisposable;
    /**
     * An event emitted on a "keydown".
     * @event
     */
    onKeyDown(listener: (e: IKeyboardEvent) => void): IDisposable;
    /**
     * An event emitted when the layout of the editor has changed.
     * @event
     */
    onDidLayoutChange(listener: (e: EditorLayoutInfo) => void): IDisposable;
    /**
     * An event emitted when the content width or content height in the editor has changed.
     * @event
     */
    onDidContentSizeChange(
      listener: (e: IContentSizeChangedEvent) => void
    ): IDisposable;
    /**
     * An event emitted when the scroll in the editor has changed.
     * @event
     */
    onDidScrollChange(listener: (e: IScrollEvent) => void): IDisposable;
    /**
     * Saves current view state of the editor in a serializable object.
     */
    saveViewState(): ICodeEditorViewState | null;
    /**
     * Restores the view state of the editor from a serializable object generated by `saveViewState`.
     */
    restoreViewState(state: ICodeEditorViewState): void;
    /**
     * Returns true if the text inside this editor or an editor widget has focus.
     */
    hasWidgetFocus(): boolean;
    /**
     * Get a contribution of this editor.
     * @id Unique identifier of the contribution.
     * @return The contribution or null if contribution not found.
     */
    getContribution(id: string): any;
    /**
     * Type the getModel() of IEditor.
     */
    getModel(): ITextModel | null;
    /**
     * Sets the current model attached to this editor.
     * If the previous model was created by the editor via the value key in the options
     * literal object, it will be destroyed. Otherwise, if the previous model was set
     * via setModel, or the model key in the options literal object, the previous model
     * will not be destroyed.
     * It is safe to call setModel(null) to simply detach the current model from the editor.
     */
    setModel(model: ITextModel | null): void;
    /**
     * Gets all the editor computed options.
     */
    getOptions(): IComputedEditorOptions;
    /**
     * Gets a specific editor option.
     */
    getOption(id: any): any;
    /**
     * Returns the editor's configuration (without any validation or defaults).
     */
    getRawOptions(): IEditorOptions;
    /**
     * Get value of the current model attached to this editor.
     * @see `ITextModel.getValue`
     */
    getValue(options?: { preserveBOM: boolean; lineEnding: string }): string;
    /**
     * Set the value of the current model attached to this editor.
     * @see `ITextModel.setValue`
     */
    setValue(newValue: string): void;
    /**
     * Get the width of the editor's content.
     * This is information that is "erased" when computing `scrollWidth = Math.max(contentWidth, width)`
     */
    getContentWidth(): number;
    /**
     * Get the scrollWidth of the editor's viewport.
     */
    getScrollWidth(): number;
    /**
     * Get the scrollLeft of the editor's viewport.
     */
    getScrollLeft(): number;
    /**
     * Get the height of the editor's content.
     * This is information that is "erased" when computing `scrollHeight = Math.max(contentHeight, height)`
     */
    getContentHeight(): number;
    /**
     * Get the scrollHeight of the editor's viewport.
     */
    getScrollHeight(): number;
    /**
     * Get the scrollTop of the editor's viewport.
     */
    getScrollTop(): number;
    /**
     * Change the scrollLeft of the editor's viewport.
     */
    setScrollLeft(newScrollLeft: number): void;
    /**
     * Change the scrollTop of the editor's viewport.
     */
    setScrollTop(newScrollTop: number): void;
    /**
     * Change the scroll position of the editor's viewport.
     */
    setScrollPosition(position: INewScrollPosition): void;
    /**
     * Get an action that is a contribution to this editor.
     * @id Unique identifier of the contribution.
     * @return The action or null if action not found.
     */
    getAction(id: string): IEditorAction;
    /**
     * Execute a command on the editor.
     * The edits will land on the undo-redo stack, but no "undo stop" will be pushed.
     * @param source The source of the call.
     * @param command The command to execute
     */
    executeCommand(source: string, command: ICommand): void;
    /**
     * Push an "undo stop" in the undo-redo stack.
     */
    pushUndoStop(): boolean;
    /**
     * Execute edits on the editor.
     * The edits will land on the undo-redo stack, but no "undo stop" will be pushed.
     * @param source The source of the call.
     * @param edits The edits to execute.
     * @param endCursorState Cursor state after the edits were applied.
     */
    executeEdits(
      source: string,
      edits: IIdentifiedSingleEditOperation[],
      endCursorState?: (
        inverseEditOperations: IIdentifiedSingleEditOperation[]
      ) => Selection[] | null | Selection[]
    ): boolean;
    /**
     * Execute multiple (concomitant) commands on the editor.
     * @param source The source of the call.
     * @param command The commands to execute
     */
    executeCommands(source: string, commands: (ICommand | null)[]): void;
    /**
     * Get all the decorations on a line (filtering out decorations from other editors).
     */
    getLineDecorations(lineNumber: number): IModelDecoration[] | null;
    /**
     * All decorations added through this call will get the ownerId of this editor.
     * @see `ITextModel.deltaDecorations`
     */
    deltaDecorations(
      oldDecorations: string[],
      newDecorations: IModelDeltaDecoration[]
    ): string[];
    /**
     * Get the layout info for the editor.
     */
    getLayoutInfo(): EditorLayoutInfo;
    /**
     * Returns the ranges that are currently visible.
     * Does not account for horizontal scrolling.
     */
    getVisibleRanges(): Range[];
    /**
     * Get the vertical position (top offset) for the line w.r.t. to the first line.
     */
    getTopForLineNumber(lineNumber: number): number;
    /**
     * Get the vertical position (top offset) for the position w.r.t. to the first line.
     */
    getTopForPosition(lineNumber: number, column: number): number;
    /**
     * Returns the editor's container dom node
     */
    getContainerDomNode(): HTMLElement;
    /**
     * Returns the editor's dom node
     */
    getDomNode(): HTMLElement | null;
    /**
     * Add a content widget. Widgets must have unique ids, otherwise they will be overwritten.
     */
    addContentWidget(widget: IContentWidget): void;
    /**
     * Layout/Reposition a content widget. This is a ping to the editor to call widget.getPosition()
     * and update appropriately.
     */
    layoutContentWidget(widget: IContentWidget): void;
    /**
     * Remove a content widget.
     */
    removeContentWidget(widget: IContentWidget): void;
    /**
     * Add an overlay widget. Widgets must have unique ids, otherwise they will be overwritten.
     */
    addOverlayWidget(widget: IOverlayWidget): void;
    /**
     * Layout/Reposition an overlay widget. This is a ping to the editor to call widget.getPosition()
     * and update appropriately.
     */
    layoutOverlayWidget(widget: IOverlayWidget): void;
    /**
     * Remove an overlay widget.
     */
    removeOverlayWidget(widget: IOverlayWidget): void;
    /**
     * Change the view zones. View zones are lost when a new model is attached to the editor.
     */
    changeViewZones(
      callback: (accessor: IViewZoneChangeAccessor) => void
    ): void;
    /**
     * Get the horizontal position (left offset) for the column w.r.t to the beginning of the line.
     * This method works only if the line `lineNumber` is currently rendered (in the editor's viewport).
     * Use this method with caution.
     */
    getOffsetForColumn(lineNumber: number, column: number): number;
    /**
     * Force an editor render now.
     */
    render(forceRedraw?: boolean): void;
    /**
     * Get the hit test target at coordinates `clientX` and `clientY`.
     * The coordinates are relative to the top-left of the viewport.
     *
     * @returns Hit test target or null if the coordinates fall outside the editor or the editor has no model.
     */
    getTargetAtClientPoint(
      clientX: number,
      clientY: number
    ): IMouseTarget | null;
    /**
     * Get the visible position for `position`.
     * The result position takes scrolling into account and is relative to the top left corner of the editor.
     * Explanation 1: the results of this method will change for the same `position` if the user scrolls the editor.
     * Explanation 2: the results of this method will not change if the container of the editor gets repositioned.
     * Warning: the results of this method are inaccurate for positions that are outside the current editor viewport.
     */
    getScrolledVisiblePosition(
      position: IPosition
    ): { top: number; left: number; height: number } | null;
    /**
     * Apply the same font settings as the editor to `target`.
     */
    applyFontInfo(target: HTMLElement): void;
  }

  /**
   * Information about a line in the diff editor
   */
  export interface IDiffLineInformation {
    readonly equivalentLineNumber: number;
  }

  /**
   * A rich diff editor.
   */
  export interface IDiffEditor extends IEditor {
    /**
     * @see ICodeEditor.getDomNode
     */
    getDomNode(): HTMLElement;
    /**
     * An event emitted when the diff information computed by this diff editor has been updated.
     * @event
     */
    onDidUpdateDiff(listener: () => void): IDisposable;
    /**
     * Saves current view state of the editor in a serializable object.
     */
    saveViewState(): IDiffEditorViewState | null;
    /**
     * Restores the view state of the editor from a serializable object generated by `saveViewState`.
     */
    restoreViewState(state: IDiffEditorViewState): void;
    /**
     * Type the getModel() of IEditor.
     */
    getModel(): IDiffEditorModel | null;
    /**
     * Sets the current model attached to this editor.
     * If the previous model was created by the editor via the value key in the options
     * literal object, it will be destroyed. Otherwise, if the previous model was set
     * via setModel, or the model key in the options literal object, the previous model
     * will not be destroyed.
     * It is safe to call setModel(null) to simply detach the current model from the editor.
     */
    setModel(model: IDiffEditorModel | null): void;
    /**
     * Get the `original` editor.
     */
    getOriginalEditor(): ICodeEditor;
    /**
     * Get the `modified` editor.
     */
    getModifiedEditor(): ICodeEditor;
    /**
     * Get the computed diff information.
     */
    getLineChanges(): ILineChange[] | null;
    /**
     * Get information based on computed diff about a line number from the original model.
     * If the diff computation is not finished or the model is missing, will return null.
     */
    getDiffLineInformationForOriginal(
      lineNumber: number
    ): IDiffLineInformation | null;
    /**
     * Get information based on computed diff about a line number from the modified model.
     * If the diff computation is not finished or the model is missing, will return null.
     */
    getDiffLineInformationForModified(
      lineNumber: number
    ): IDiffLineInformation | null;
    /**
     * Update the editor's options after the editor has been created.
     */
    updateOptions(newOptions: IDiffEditorOptions): void;
  }

  export class FontInfo extends BareFontInfo {
    readonly _editorStylingBrand: void;
    readonly isTrusted: boolean;
    readonly isMonospace: boolean;
    readonly typicalHalfwidthCharacterWidth: number;
    readonly typicalFullwidthCharacterWidth: number;
    readonly canUseHalfwidthRightwardsArrow: boolean;
    readonly spaceWidth: number;
    readonly middotWidth: number;
    readonly maxDigitWidth: number;
  }

  export class BareFontInfo {
    readonly _bareFontInfoBrand: void;
    readonly zoomLevel: number;
    readonly fontFamily: string;
    readonly fontWeight: string;
    readonly fontSize: number;
    readonly fontFeatureSettings: string;
    readonly lineHeight: number;
    readonly letterSpacing: number;
  }

  // //compatibility:
  // export type IReadOnlyModel = ITextModel;
  // export type IModel = ITextModel;
}
