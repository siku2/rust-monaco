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
