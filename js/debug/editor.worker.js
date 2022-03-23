(function (exports) {
    'use strict';

    // Avoid circular dependency on EventEmitter by implementing a subset of the interface.
    class ErrorHandler {
        constructor() {
            this.listeners = [];
            this.unexpectedErrorHandler = function (e) {
                setTimeout(() => {
                    if (e.stack) {
                        throw new Error(e.message + '\n\n' + e.stack);
                    }
                    throw e;
                }, 0);
            };
        }
        emit(e) {
            this.listeners.forEach((listener) => {
                listener(e);
            });
        }
        onUnexpectedError(e) {
            this.unexpectedErrorHandler(e);
            this.emit(e);
        }
        // For external errors, we don't want the listeners to be called
        onUnexpectedExternalError(e) {
            this.unexpectedErrorHandler(e);
        }
    }
    const errorHandler = new ErrorHandler();
    function onUnexpectedError(e) {
        // ignore errors from cancelled promises
        if (!isCancellationError(e)) {
            errorHandler.onUnexpectedError(e);
        }
        return undefined;
    }
    function transformErrorForSerialization(error) {
        if (error instanceof Error) {
            let { name, message } = error;
            const stack = error.stacktrace || error.stack;
            return {
                $isError: true,
                name,
                message,
                stack
            };
        }
        // return as is
        return error;
    }
    const canceledName = 'Canceled';
    /**
     * Checks if the given error is a promise in canceled state
     */
    function isCancellationError(error) {
        if (error instanceof CancellationError) {
            return true;
        }
        return error instanceof Error && error.name === canceledName && error.message === canceledName;
    }
    // !!!IMPORTANT!!!
    // Do NOT change this class because it is also used as an API-type.
    class CancellationError extends Error {
        constructor() {
            super(canceledName);
            this.name = this.message;
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    function once(fn) {
        const _this = this;
        let didCall = false;
        let result;
        return function () {
            if (didCall) {
                return result;
            }
            didCall = true;
            result = fn.apply(_this, arguments);
            return result;
        };
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    var Iterable;
    (function (Iterable) {
        function is(thing) {
            return thing && typeof thing === 'object' && typeof thing[Symbol.iterator] === 'function';
        }
        Iterable.is = is;
        const _empty = Object.freeze([]);
        function empty() {
            return _empty;
        }
        Iterable.empty = empty;
        function* single(element) {
            yield element;
        }
        Iterable.single = single;
        function from(iterable) {
            return iterable || _empty;
        }
        Iterable.from = from;
        function isEmpty(iterable) {
            return !iterable || iterable[Symbol.iterator]().next().done === true;
        }
        Iterable.isEmpty = isEmpty;
        function first(iterable) {
            return iterable[Symbol.iterator]().next().value;
        }
        Iterable.first = first;
        function some(iterable, predicate) {
            for (const element of iterable) {
                if (predicate(element)) {
                    return true;
                }
            }
            return false;
        }
        Iterable.some = some;
        function find(iterable, predicate) {
            for (const element of iterable) {
                if (predicate(element)) {
                    return element;
                }
            }
            return undefined;
        }
        Iterable.find = find;
        function* filter(iterable, predicate) {
            for (const element of iterable) {
                if (predicate(element)) {
                    yield element;
                }
            }
        }
        Iterable.filter = filter;
        function* map(iterable, fn) {
            let index = 0;
            for (const element of iterable) {
                yield fn(element, index++);
            }
        }
        Iterable.map = map;
        function* concat(...iterables) {
            for (const iterable of iterables) {
                for (const element of iterable) {
                    yield element;
                }
            }
        }
        Iterable.concat = concat;
        function* concatNested(iterables) {
            for (const iterable of iterables) {
                for (const element of iterable) {
                    yield element;
                }
            }
        }
        Iterable.concatNested = concatNested;
        function reduce(iterable, reducer, initialValue) {
            let value = initialValue;
            for (const element of iterable) {
                value = reducer(value, element);
            }
            return value;
        }
        Iterable.reduce = reduce;
        /**
         * Returns an iterable slice of the array, with the same semantics as `array.slice()`.
         */
        function* slice(arr, from, to = arr.length) {
            if (from < 0) {
                from += arr.length;
            }
            if (to < 0) {
                to += arr.length;
            }
            else if (to > arr.length) {
                to = arr.length;
            }
            for (; from < to; from++) {
                yield arr[from];
            }
        }
        Iterable.slice = slice;
        /**
         * Consumes `atMost` elements from iterable and returns the consumed elements,
         * and an iterable for the rest of the elements.
         */
        function consume(iterable, atMost = Number.POSITIVE_INFINITY) {
            const consumed = [];
            if (atMost === 0) {
                return [consumed, iterable];
            }
            const iterator = iterable[Symbol.iterator]();
            for (let i = 0; i < atMost; i++) {
                const next = iterator.next();
                if (next.done) {
                    return [consumed, Iterable.empty()];
                }
                consumed.push(next.value);
            }
            return [consumed, { [Symbol.iterator]() { return iterator; } }];
        }
        Iterable.consume = consume;
        /**
         * Returns whether the iterables are the same length and all items are
         * equal using the comparator function.
         */
        function equals(a, b, comparator = (at, bt) => at === bt) {
            const ai = a[Symbol.iterator]();
            const bi = b[Symbol.iterator]();
            while (true) {
                const an = ai.next();
                const bn = bi.next();
                if (an.done !== bn.done) {
                    return false;
                }
                else if (an.done) {
                    return true;
                }
                else if (!comparator(an.value, bn.value)) {
                    return false;
                }
            }
        }
        Iterable.equals = equals;
    })(Iterable || (Iterable = {}));

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    function trackDisposable(x) {
        return x;
    }
    function setParentOfDisposable(child, parent) {
    }
    class MultiDisposeError extends Error {
        constructor(errors) {
            super(`Encountered errors while disposing of store. Errors: [${errors.join(', ')}]`);
            this.errors = errors;
        }
    }
    function dispose(arg) {
        if (Iterable.is(arg)) {
            let errors = [];
            for (const d of arg) {
                if (d) {
                    try {
                        d.dispose();
                    }
                    catch (e) {
                        errors.push(e);
                    }
                }
            }
            if (errors.length === 1) {
                throw errors[0];
            }
            else if (errors.length > 1) {
                throw new MultiDisposeError(errors);
            }
            return Array.isArray(arg) ? [] : arg;
        }
        else if (arg) {
            arg.dispose();
            return arg;
        }
    }
    function combinedDisposable(...disposables) {
        const parent = toDisposable(() => dispose(disposables));
        return parent;
    }
    function toDisposable(fn) {
        const self = trackDisposable({
            dispose: once(() => {
                fn();
            })
        });
        return self;
    }
    class DisposableStore {
        constructor() {
            this._toDispose = new Set();
            this._isDisposed = false;
        }
        /**
         * Dispose of all registered disposables and mark this object as disposed.
         *
         * Any future disposables added to this object will be disposed of on `add`.
         */
        dispose() {
            if (this._isDisposed) {
                return;
            }
            this._isDisposed = true;
            this.clear();
        }
        /**
         * Returns `true` if this object has been disposed
         */
        get isDisposed() {
            return this._isDisposed;
        }
        /**
         * Dispose of all registered disposables but do not mark this object as disposed.
         */
        clear() {
            try {
                dispose(this._toDispose.values());
            }
            finally {
                this._toDispose.clear();
            }
        }
        add(o) {
            if (!o) {
                return o;
            }
            if (o === this) {
                throw new Error('Cannot register a disposable on itself!');
            }
            if (this._isDisposed) {
                if (!DisposableStore.DISABLE_DISPOSED_WARNING) {
                    console.warn(new Error('Trying to add a disposable to a DisposableStore that has already been disposed of. The added object will be leaked!').stack);
                }
            }
            else {
                this._toDispose.add(o);
            }
            return o;
        }
    }
    DisposableStore.DISABLE_DISPOSED_WARNING = false;
    class Disposable {
        constructor() {
            this._store = new DisposableStore();
            setParentOfDisposable(this._store);
        }
        dispose() {
            this._store.dispose();
        }
        _register(o) {
            if (o === this) {
                throw new Error('Cannot register a disposable on itself!');
            }
            return this._store.add(o);
        }
    }
    Disposable.None = Object.freeze({ dispose() { } });

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    class Node {
        constructor(element) {
            this.element = element;
            this.next = Node.Undefined;
            this.prev = Node.Undefined;
        }
    }
    Node.Undefined = new Node(undefined);
    class LinkedList {
        constructor() {
            this._first = Node.Undefined;
            this._last = Node.Undefined;
            this._size = 0;
        }
        get size() {
            return this._size;
        }
        isEmpty() {
            return this._first === Node.Undefined;
        }
        clear() {
            let node = this._first;
            while (node !== Node.Undefined) {
                const next = node.next;
                node.prev = Node.Undefined;
                node.next = Node.Undefined;
                node = next;
            }
            this._first = Node.Undefined;
            this._last = Node.Undefined;
            this._size = 0;
        }
        unshift(element) {
            return this._insert(element, false);
        }
        push(element) {
            return this._insert(element, true);
        }
        _insert(element, atTheEnd) {
            const newNode = new Node(element);
            if (this._first === Node.Undefined) {
                this._first = newNode;
                this._last = newNode;
            }
            else if (atTheEnd) {
                // push
                const oldLast = this._last;
                this._last = newNode;
                newNode.prev = oldLast;
                oldLast.next = newNode;
            }
            else {
                // unshift
                const oldFirst = this._first;
                this._first = newNode;
                newNode.next = oldFirst;
                oldFirst.prev = newNode;
            }
            this._size += 1;
            let didRemove = false;
            return () => {
                if (!didRemove) {
                    didRemove = true;
                    this._remove(newNode);
                }
            };
        }
        shift() {
            if (this._first === Node.Undefined) {
                return undefined;
            }
            else {
                const res = this._first.element;
                this._remove(this._first);
                return res;
            }
        }
        pop() {
            if (this._last === Node.Undefined) {
                return undefined;
            }
            else {
                const res = this._last.element;
                this._remove(this._last);
                return res;
            }
        }
        _remove(node) {
            if (node.prev !== Node.Undefined && node.next !== Node.Undefined) {
                // middle
                const anchor = node.prev;
                anchor.next = node.next;
                node.next.prev = anchor;
            }
            else if (node.prev === Node.Undefined && node.next === Node.Undefined) {
                // only node
                this._first = Node.Undefined;
                this._last = Node.Undefined;
            }
            else if (node.next === Node.Undefined) {
                // last
                this._last = this._last.prev;
                this._last.next = Node.Undefined;
            }
            else if (node.prev === Node.Undefined) {
                // first
                this._first = this._first.next;
                this._first.prev = Node.Undefined;
            }
            // done
            this._size -= 1;
        }
        *[Symbol.iterator]() {
            let node = this._first;
            while (node !== Node.Undefined) {
                yield node.element;
                node = node.next;
            }
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    var _a$1;
    const LANGUAGE_DEFAULT = 'en';
    let _isWindows = false;
    let _isMacintosh = false;
    let _isLinux = false;
    let _locale = undefined;
    let _language = LANGUAGE_DEFAULT;
    let _translationsConfigFile = undefined;
    let _userAgent = undefined;
    const globals = (typeof self === 'object' ? self : typeof global === 'object' ? global : {});
    let nodeProcess = undefined;
    if (typeof globals.vscode !== 'undefined' && typeof globals.vscode.process !== 'undefined') {
        // Native environment (sandboxed)
        nodeProcess = globals.vscode.process;
    }
    else if (typeof process !== 'undefined') {
        // Native environment (non-sandboxed)
        nodeProcess = process;
    }
    const isElectronProcess = typeof ((_a$1 = nodeProcess === null || nodeProcess === void 0 ? void 0 : nodeProcess.versions) === null || _a$1 === void 0 ? void 0 : _a$1.electron) === 'string';
    const isElectronRenderer = isElectronProcess && (nodeProcess === null || nodeProcess === void 0 ? void 0 : nodeProcess.type) === 'renderer';
    // Web environment
    if (typeof navigator === 'object' && !isElectronRenderer) {
        _userAgent = navigator.userAgent;
        _isWindows = _userAgent.indexOf('Windows') >= 0;
        _isMacintosh = _userAgent.indexOf('Macintosh') >= 0;
        (_userAgent.indexOf('Macintosh') >= 0 || _userAgent.indexOf('iPad') >= 0 || _userAgent.indexOf('iPhone') >= 0) && !!navigator.maxTouchPoints && navigator.maxTouchPoints > 0;
        _isLinux = _userAgent.indexOf('Linux') >= 0;
        _locale = navigator.language;
        _language = _locale;
    }
    // Native environment
    else if (typeof nodeProcess === 'object') {
        _isWindows = (nodeProcess.platform === 'win32');
        _isMacintosh = (nodeProcess.platform === 'darwin');
        _isLinux = (nodeProcess.platform === 'linux');
        _isLinux && !!nodeProcess.env['SNAP'] && !!nodeProcess.env['SNAP_REVISION'];
        _locale = LANGUAGE_DEFAULT;
        _language = LANGUAGE_DEFAULT;
        const rawNlsConfig = nodeProcess.env['VSCODE_NLS_CONFIG'];
        if (rawNlsConfig) {
            try {
                const nlsConfig = JSON.parse(rawNlsConfig);
                const resolved = nlsConfig.availableLanguages['*'];
                _locale = nlsConfig.locale;
                // VSCode's default language is 'en'
                _language = resolved ? resolved : LANGUAGE_DEFAULT;
                _translationsConfigFile = nlsConfig._translationsConfigFile;
            }
            catch (e) {
            }
        }
    }
    // Unknown environment
    else {
        console.error('Unable to resolve platform.');
    }
    const isWindows = _isWindows;
    const isMacintosh = _isMacintosh;
    const isLinux = _isLinux;
    const userAgent = _userAgent;
    /**
     * See https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#:~:text=than%204%2C%20then-,set%20timeout%20to%204,-.
     *
     * Works similarly to `setTimeout(0)` but doesn't suffer from the 4ms artificial delay
     * that browsers set when the nesting level is > 5.
     */
    (() => {
        if (typeof globals.postMessage === 'function' && !globals.importScripts) {
            let pending = [];
            globals.addEventListener('message', (e) => {
                if (e.data && e.data.vscodeScheduleAsyncWork) {
                    for (let i = 0, len = pending.length; i < len; i++) {
                        const candidate = pending[i];
                        if (candidate.id === e.data.vscodeScheduleAsyncWork) {
                            pending.splice(i, 1);
                            candidate.callback();
                            return;
                        }
                    }
                }
            });
            let lastId = 0;
            return (callback) => {
                const myId = ++lastId;
                pending.push({
                    id: myId,
                    callback: callback
                });
                globals.postMessage({ vscodeScheduleAsyncWork: myId }, '*');
            };
        }
        return (callback) => setTimeout(callback);
    })();
    const isChrome = !!(userAgent && userAgent.indexOf('Chrome') >= 0);
    !!(userAgent && userAgent.indexOf('Firefox') >= 0);
    !!(!isChrome && (userAgent && userAgent.indexOf('Safari') >= 0));
    !!(userAgent && userAgent.indexOf('Edg/') >= 0);
    !!(userAgent && userAgent.indexOf('Android') >= 0);

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    const hasPerformanceNow = (globals.performance && typeof globals.performance.now === 'function');
    class StopWatch {
        constructor(highResolution) {
            this._highResolution = hasPerformanceNow && highResolution;
            this._startTime = this._now();
            this._stopTime = -1;
        }
        static create(highResolution = true) {
            return new StopWatch(highResolution);
        }
        stop() {
            this._stopTime = this._now();
        }
        elapsed() {
            if (this._stopTime !== -1) {
                return this._stopTime - this._startTime;
            }
            return this._now() - this._startTime;
        }
        _now() {
            return this._highResolution ? globals.performance.now() : Date.now();
        }
    }

    var Event;
    (function (Event) {
        Event.None = () => Disposable.None;
        /**
         * Given an event, returns another event which only fires once.
         */
        function once(event) {
            return (listener, thisArgs = null, disposables) => {
                // we need this, in case the event fires during the listener call
                let didFire = false;
                let result;
                result = event(e => {
                    if (didFire) {
                        return;
                    }
                    else if (result) {
                        result.dispose();
                    }
                    else {
                        didFire = true;
                    }
                    return listener.call(thisArgs, e);
                }, null, disposables);
                if (didFire) {
                    result.dispose();
                }
                return result;
            };
        }
        Event.once = once;
        /**
         * @deprecated DO NOT use, this leaks memory
         */
        function map(event, map) {
            return snapshot((listener, thisArgs = null, disposables) => event(i => listener.call(thisArgs, map(i)), null, disposables));
        }
        Event.map = map;
        /**
         * @deprecated DO NOT use, this leaks memory
         */
        function forEach(event, each) {
            return snapshot((listener, thisArgs = null, disposables) => event(i => { each(i); listener.call(thisArgs, i); }, null, disposables));
        }
        Event.forEach = forEach;
        function filter(event, filter) {
            return snapshot((listener, thisArgs = null, disposables) => event(e => filter(e) && listener.call(thisArgs, e), null, disposables));
        }
        Event.filter = filter;
        /**
         * Given an event, returns the same event but typed as `Event<void>`.
         */
        function signal(event) {
            return event;
        }
        Event.signal = signal;
        function any(...events) {
            return (listener, thisArgs = null, disposables) => combinedDisposable(...events.map(event => event(e => listener.call(thisArgs, e), null, disposables)));
        }
        Event.any = any;
        /**
         * @deprecated DO NOT use, this leaks memory
         */
        function reduce(event, merge, initial) {
            let output = initial;
            return map(event, e => {
                output = merge(output, e);
                return output;
            });
        }
        Event.reduce = reduce;
        /**
         * @deprecated DO NOT use, this leaks memory
         */
        function snapshot(event) {
            let listener;
            const emitter = new Emitter({
                onFirstListenerAdd() {
                    listener = event(emitter.fire, emitter);
                },
                onLastListenerRemove() {
                    listener.dispose();
                }
            });
            return emitter.event;
        }
        function debouncedListener(event, listener, merge, delay = 100, leading = false) {
            let output = undefined;
            let handle = undefined;
            let numDebouncedCalls = 0;
            return event(cur => {
                numDebouncedCalls++;
                output = merge(output, cur);
                if (leading && !handle) {
                    listener(output);
                    output = undefined;
                }
                clearTimeout(handle);
                handle = setTimeout(() => {
                    const _output = output;
                    output = undefined;
                    handle = undefined;
                    if (!leading || numDebouncedCalls > 1) {
                        listener(_output);
                    }
                    numDebouncedCalls = 0;
                }, delay);
            });
        }
        Event.debouncedListener = debouncedListener;
        /**
         * @deprecated this leaks memory, {@link debouncedListener} or {@link DebounceEmitter} instead
         */
        function debounce(event, merge, delay = 100, leading = false, leakWarningThreshold) {
            let subscription;
            let output = undefined;
            let handle = undefined;
            let numDebouncedCalls = 0;
            const emitter = new Emitter({
                leakWarningThreshold,
                onFirstListenerAdd() {
                    subscription = event(cur => {
                        numDebouncedCalls++;
                        output = merge(output, cur);
                        if (leading && !handle) {
                            emitter.fire(output);
                            output = undefined;
                        }
                        clearTimeout(handle);
                        handle = setTimeout(() => {
                            const _output = output;
                            output = undefined;
                            handle = undefined;
                            if (!leading || numDebouncedCalls > 1) {
                                emitter.fire(_output);
                            }
                            numDebouncedCalls = 0;
                        }, delay);
                    });
                },
                onLastListenerRemove() {
                    subscription.dispose();
                }
            });
            return emitter.event;
        }
        Event.debounce = debounce;
        /**
         * @deprecated DO NOT use, this leaks memory
         */
        function latch(event, equals = (a, b) => a === b) {
            let firstCall = true;
            let cache;
            return filter(event, value => {
                const shouldEmit = firstCall || !equals(value, cache);
                firstCall = false;
                cache = value;
                return shouldEmit;
            });
        }
        Event.latch = latch;
        /**
         * @deprecated DO NOT use, this leaks memory
         */
        function split(event, isT) {
            return [
                Event.filter(event, isT),
                Event.filter(event, e => !isT(e)),
            ];
        }
        Event.split = split;
        /**
         * @deprecated DO NOT use, this leaks memory
         */
        function buffer(event, flushAfterTimeout = false, _buffer = []) {
            let buffer = _buffer.slice();
            let listener = event(e => {
                if (buffer) {
                    buffer.push(e);
                }
                else {
                    emitter.fire(e);
                }
            });
            const flush = () => {
                if (buffer) {
                    buffer.forEach(e => emitter.fire(e));
                }
                buffer = null;
            };
            const emitter = new Emitter({
                onFirstListenerAdd() {
                    if (!listener) {
                        listener = event(e => emitter.fire(e));
                    }
                },
                onFirstListenerDidAdd() {
                    if (buffer) {
                        if (flushAfterTimeout) {
                            setTimeout(flush);
                        }
                        else {
                            flush();
                        }
                    }
                },
                onLastListenerRemove() {
                    if (listener) {
                        listener.dispose();
                    }
                    listener = null;
                }
            });
            return emitter.event;
        }
        Event.buffer = buffer;
        class ChainableEvent {
            constructor(event) {
                this.event = event;
            }
            map(fn) {
                return new ChainableEvent(map(this.event, fn));
            }
            forEach(fn) {
                return new ChainableEvent(forEach(this.event, fn));
            }
            filter(fn) {
                return new ChainableEvent(filter(this.event, fn));
            }
            reduce(merge, initial) {
                return new ChainableEvent(reduce(this.event, merge, initial));
            }
            latch() {
                return new ChainableEvent(latch(this.event));
            }
            debounce(merge, delay = 100, leading = false, leakWarningThreshold) {
                return new ChainableEvent(debounce(this.event, merge, delay, leading, leakWarningThreshold));
            }
            on(listener, thisArgs, disposables) {
                return this.event(listener, thisArgs, disposables);
            }
            once(listener, thisArgs, disposables) {
                return once(this.event)(listener, thisArgs, disposables);
            }
        }
        /**
         * @deprecated DO NOT use, this leaks memory
         */
        function chain(event) {
            return new ChainableEvent(event);
        }
        Event.chain = chain;
        function fromNodeEventEmitter(emitter, eventName, map = id => id) {
            const fn = (...args) => result.fire(map(...args));
            const onFirstListenerAdd = () => emitter.on(eventName, fn);
            const onLastListenerRemove = () => emitter.removeListener(eventName, fn);
            const result = new Emitter({ onFirstListenerAdd, onLastListenerRemove });
            return result.event;
        }
        Event.fromNodeEventEmitter = fromNodeEventEmitter;
        function fromDOMEventEmitter(emitter, eventName, map = id => id) {
            const fn = (...args) => result.fire(map(...args));
            const onFirstListenerAdd = () => emitter.addEventListener(eventName, fn);
            const onLastListenerRemove = () => emitter.removeEventListener(eventName, fn);
            const result = new Emitter({ onFirstListenerAdd, onLastListenerRemove });
            return result.event;
        }
        Event.fromDOMEventEmitter = fromDOMEventEmitter;
        function toPromise(event) {
            return new Promise(resolve => once(event)(resolve));
        }
        Event.toPromise = toPromise;
        function runAndSubscribe(event, handler) {
            handler(undefined);
            return event(e => handler(e));
        }
        Event.runAndSubscribe = runAndSubscribe;
        function runAndSubscribeWithStore(event, handler) {
            let store = null;
            function run(e) {
                store === null || store === void 0 ? void 0 : store.dispose();
                store = new DisposableStore();
                handler(e, store);
            }
            run(undefined);
            const disposable = event(e => run(e));
            return toDisposable(() => {
                disposable.dispose();
                store === null || store === void 0 ? void 0 : store.dispose();
            });
        }
        Event.runAndSubscribeWithStore = runAndSubscribeWithStore;
    })(Event || (Event = {}));
    class EventProfiling {
        constructor(name) {
            this._listenerCount = 0;
            this._invocationCount = 0;
            this._elapsedOverall = 0;
            this._name = `${name}_${EventProfiling._idPool++}`;
        }
        start(listenerCount) {
            this._stopWatch = new StopWatch(true);
            this._listenerCount = listenerCount;
        }
        stop() {
            if (this._stopWatch) {
                const elapsed = this._stopWatch.elapsed();
                this._elapsedOverall += elapsed;
                this._invocationCount += 1;
                console.info(`did FIRE ${this._name}: elapsed_ms: ${elapsed.toFixed(5)}, listener: ${this._listenerCount} (elapsed_overall: ${this._elapsedOverall.toFixed(2)}, invocations: ${this._invocationCount})`);
                this._stopWatch = undefined;
            }
        }
    }
    EventProfiling._idPool = 0;
    /**
     * The Emitter can be used to expose an Event to the public
     * to fire it from the insides.
     * Sample:
        class Document {

            private readonly _onDidChange = new Emitter<(value:string)=>any>();

            public onDidChange = this._onDidChange.event;

            // getter-style
            // get onDidChange(): Event<(value:string)=>any> {
            // 	return this._onDidChange.event;
            // }

            private _doIt() {
                //...
                this._onDidChange.fire(value);
            }
        }
     */
    class Emitter {
        constructor(options) {
            var _a;
            this._disposed = false;
            this._options = options;
            this._leakageMon = undefined;
            this._perfMon = ((_a = this._options) === null || _a === void 0 ? void 0 : _a._profName) ? new EventProfiling(this._options._profName) : undefined;
        }
        /**
         * For the public to allow to subscribe
         * to events from this Emitter
         */
        get event() {
            if (!this._event) {
                this._event = (listener, thisArgs, disposables) => {
                    var _a;
                    if (!this._listeners) {
                        this._listeners = new LinkedList();
                    }
                    const firstListener = this._listeners.isEmpty();
                    if (firstListener && this._options && this._options.onFirstListenerAdd) {
                        this._options.onFirstListenerAdd(this);
                    }
                    const remove = this._listeners.push(!thisArgs ? listener : [listener, thisArgs]);
                    if (firstListener && this._options && this._options.onFirstListenerDidAdd) {
                        this._options.onFirstListenerDidAdd(this);
                    }
                    if (this._options && this._options.onListenerDidAdd) {
                        this._options.onListenerDidAdd(this, listener, thisArgs);
                    }
                    // check and record this emitter for potential leakage
                    const removeMonitor = (_a = this._leakageMon) === null || _a === void 0 ? void 0 : _a.check(this._listeners.size);
                    const result = toDisposable(() => {
                        if (removeMonitor) {
                            removeMonitor();
                        }
                        if (!this._disposed) {
                            remove();
                            if (this._options && this._options.onLastListenerRemove) {
                                const hasListeners = (this._listeners && !this._listeners.isEmpty());
                                if (!hasListeners) {
                                    this._options.onLastListenerRemove(this);
                                }
                            }
                        }
                    });
                    if (disposables instanceof DisposableStore) {
                        disposables.add(result);
                    }
                    else if (Array.isArray(disposables)) {
                        disposables.push(result);
                    }
                    return result;
                };
            }
            return this._event;
        }
        /**
         * To be kept private to fire an event to
         * subscribers
         */
        fire(event) {
            var _a, _b;
            if (this._listeners) {
                // put all [listener,event]-pairs into delivery queue
                // then emit all event. an inner/nested event might be
                // the driver of this
                if (!this._deliveryQueue) {
                    this._deliveryQueue = new LinkedList();
                }
                for (let listener of this._listeners) {
                    this._deliveryQueue.push([listener, event]);
                }
                // start/stop performance insight collection
                (_a = this._perfMon) === null || _a === void 0 ? void 0 : _a.start(this._deliveryQueue.size);
                while (this._deliveryQueue.size > 0) {
                    const [listener, event] = this._deliveryQueue.shift();
                    try {
                        if (typeof listener === 'function') {
                            listener.call(undefined, event);
                        }
                        else {
                            listener[0].call(listener[1], event);
                        }
                    }
                    catch (e) {
                        onUnexpectedError(e);
                    }
                }
                (_b = this._perfMon) === null || _b === void 0 ? void 0 : _b.stop();
            }
        }
        dispose() {
            var _a, _b, _c, _d, _e;
            if (!this._disposed) {
                this._disposed = true;
                (_a = this._listeners) === null || _a === void 0 ? void 0 : _a.clear();
                (_b = this._deliveryQueue) === null || _b === void 0 ? void 0 : _b.clear();
                (_d = (_c = this._options) === null || _c === void 0 ? void 0 : _c.onLastListenerRemove) === null || _d === void 0 ? void 0 : _d.call(_c);
                (_e = this._leakageMon) === null || _e === void 0 ? void 0 : _e.dispose();
            }
        }
    }

    /**
     * @returns whether the provided parameter is a JavaScript Array or not.
     */
    function getAllPropertyNames(obj) {
        let res = [];
        let proto = Object.getPrototypeOf(obj);
        while (Object.prototype !== proto) {
            res = res.concat(Object.getOwnPropertyNames(proto));
            proto = Object.getPrototypeOf(proto);
        }
        return res;
    }
    function getAllMethodNames(obj) {
        const methods = [];
        for (const prop of getAllPropertyNames(obj)) {
            if (typeof obj[prop] === 'function') {
                methods.push(prop);
            }
        }
        return methods;
    }
    function createProxyObject$1(methodNames, invoke) {
        const createProxyMethod = (method) => {
            return function () {
                const args = Array.prototype.slice.call(arguments, 0);
                return invoke(method, args);
            };
        };
        let result = {};
        for (const methodName of methodNames) {
            result[methodName] = createProxyMethod(methodName);
        }
        return result;
    }
    function assertNever(value, message = 'Unreachable') {
        throw new Error(message);
    }

    /**
     * Uses a LRU cache to make a given parametrized function cached.
     * Caches just the last value.
     * The key must be JSON serializable.
    */
    class LRUCachedComputed {
        constructor(computeFn) {
            this.computeFn = computeFn;
            this.lastCache = undefined;
            this.lastArgKey = undefined;
        }
        get(arg) {
            const key = JSON.stringify(arg);
            if (this.lastArgKey !== key) {
                this.lastArgKey = key;
                this.lastCache = this.computeFn(arg);
            }
            return this.lastCache;
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    class Lazy {
        constructor(executor) {
            this.executor = executor;
            this._didRun = false;
        }
        /**
         * Get the wrapped value.
         *
         * This will force evaluation of the lazy value if it has not been resolved yet. Lazy values are only
         * resolved once. `getValue` will re-throw exceptions that are hit while resolving the value
         */
        getValue() {
            if (!this._didRun) {
                try {
                    this._value = this.executor();
                }
                catch (err) {
                    this._error = err;
                }
                finally {
                    this._didRun = true;
                }
            }
            if (this._error) {
                throw this._error;
            }
            return this._value;
        }
        /**
         * Get the wrapped value without forcing evaluation.
         */
        get rawValue() { return this._value; }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    var _a;
    /**
     * Escapes regular expression characters in a given string
     */
    function escapeRegExpCharacters(value) {
        return value.replace(/[\\\{\}\*\+\?\|\^\$\.\[\]\(\)]/g, '\\$&');
    }
    function splitLines(str) {
        return str.split(/\r\n|\r|\n/);
    }
    /**
     * Returns first index of the string that is not whitespace.
     * If string is empty or contains only whitespaces, returns -1
     */
    function firstNonWhitespaceIndex(str) {
        for (let i = 0, len = str.length; i < len; i++) {
            const chCode = str.charCodeAt(i);
            if (chCode !== 32 /* Space */ && chCode !== 9 /* Tab */) {
                return i;
            }
        }
        return -1;
    }
    /**
     * Returns last index of the string that is not whitespace.
     * If string is empty or contains only whitespaces, returns -1
     */
    function lastNonWhitespaceIndex(str, startIndex = str.length - 1) {
        for (let i = startIndex; i >= 0; i--) {
            const chCode = str.charCodeAt(i);
            if (chCode !== 32 /* Space */ && chCode !== 9 /* Tab */) {
                return i;
            }
        }
        return -1;
    }
    function compareSubstring(a, b, aStart = 0, aEnd = a.length, bStart = 0, bEnd = b.length) {
        for (; aStart < aEnd && bStart < bEnd; aStart++, bStart++) {
            let codeA = a.charCodeAt(aStart);
            let codeB = b.charCodeAt(bStart);
            if (codeA < codeB) {
                return -1;
            }
            else if (codeA > codeB) {
                return 1;
            }
        }
        const aLen = aEnd - aStart;
        const bLen = bEnd - bStart;
        if (aLen < bLen) {
            return -1;
        }
        else if (aLen > bLen) {
            return 1;
        }
        return 0;
    }
    function compareSubstringIgnoreCase(a, b, aStart = 0, aEnd = a.length, bStart = 0, bEnd = b.length) {
        for (; aStart < aEnd && bStart < bEnd; aStart++, bStart++) {
            let codeA = a.charCodeAt(aStart);
            let codeB = b.charCodeAt(bStart);
            if (codeA === codeB) {
                // equal
                continue;
            }
            if (codeA >= 128 || codeB >= 128) {
                // not ASCII letters -> fallback to lower-casing strings
                return compareSubstring(a.toLowerCase(), b.toLowerCase(), aStart, aEnd, bStart, bEnd);
            }
            // mapper lower-case ascii letter onto upper-case varinats
            // [97-122] (lower ascii) --> [65-90] (upper ascii)
            if (isLowerAsciiLetter(codeA)) {
                codeA -= 32;
            }
            if (isLowerAsciiLetter(codeB)) {
                codeB -= 32;
            }
            // compare both code points
            const diff = codeA - codeB;
            if (diff === 0) {
                continue;
            }
            return diff;
        }
        const aLen = aEnd - aStart;
        const bLen = bEnd - bStart;
        if (aLen < bLen) {
            return -1;
        }
        else if (aLen > bLen) {
            return 1;
        }
        return 0;
    }
    function isLowerAsciiLetter(code) {
        return code >= 97 /* a */ && code <= 122 /* z */;
    }
    function isUpperAsciiLetter(code) {
        return code >= 65 /* A */ && code <= 90 /* Z */;
    }
    function startsWithIgnoreCase(str, candidate) {
        const candidateLength = candidate.length;
        if (candidate.length > str.length) {
            return false;
        }
        return compareSubstringIgnoreCase(str, candidate, 0, candidateLength) === 0;
    }
    /**
     * See http://en.wikipedia.org/wiki/Surrogate_pair
     */
    function isHighSurrogate(charCode) {
        return (0xD800 <= charCode && charCode <= 0xDBFF);
    }
    /**
     * See http://en.wikipedia.org/wiki/Surrogate_pair
     */
    function isLowSurrogate(charCode) {
        return (0xDC00 <= charCode && charCode <= 0xDFFF);
    }
    /**
     * See http://en.wikipedia.org/wiki/Surrogate_pair
     */
    function computeCodePoint(highSurrogate, lowSurrogate) {
        return ((highSurrogate - 0xD800) << 10) + (lowSurrogate - 0xDC00) + 0x10000;
    }
    /**
     * get the code point that begins at offset `offset`
     */
    function getNextCodePoint(str, len, offset) {
        const charCode = str.charCodeAt(offset);
        if (isHighSurrogate(charCode) && offset + 1 < len) {
            const nextCharCode = str.charCodeAt(offset + 1);
            if (isLowSurrogate(nextCharCode)) {
                return computeCodePoint(charCode, nextCharCode);
            }
        }
        return charCode;
    }
    class AmbiguousCharacters {
        constructor(confusableDictionary) {
            this.confusableDictionary = confusableDictionary;
        }
        static getInstance(locales) {
            return AmbiguousCharacters.cache.get(Array.from(locales));
        }
        static getLocales() {
            return AmbiguousCharacters._locales.getValue();
        }
        isAmbiguous(codePoint) {
            return this.confusableDictionary.has(codePoint);
        }
        /**
         * Returns the non basic ASCII code point that the given code point can be confused,
         * or undefined if such code point does note exist.
         */
        getPrimaryConfusable(codePoint) {
            return this.confusableDictionary.get(codePoint);
        }
        getConfusableCodePoints() {
            return new Set(this.confusableDictionary.keys());
        }
    }
    _a = AmbiguousCharacters;
    AmbiguousCharacters.ambiguousCharacterData = new Lazy(() => {
        // Generated using https://github.com/hediet/vscode-unicode-data
        // Stored as key1, value1, key2, value2, ...
        return JSON.parse('{\"_common\":[8232,32,8233,32,5760,32,8192,32,8193,32,8194,32,8195,32,8196,32,8197,32,8198,32,8200,32,8201,32,8202,32,8287,32,8199,32,8239,32,2042,95,65101,95,65102,95,65103,95,8208,45,8209,45,8210,45,65112,45,1748,45,8259,45,727,45,8722,45,10134,45,11450,45,1549,44,1643,44,8218,44,184,44,42233,44,894,59,2307,58,2691,58,1417,58,1795,58,1796,58,5868,58,65072,58,6147,58,6153,58,8282,58,1475,58,760,58,42889,58,8758,58,720,58,42237,58,451,33,11601,33,660,63,577,63,2429,63,5038,63,42731,63,119149,46,8228,46,1793,46,1794,46,42510,46,68176,46,1632,46,1776,46,42232,46,1373,96,65287,96,8219,96,8242,96,1370,96,1523,96,8175,96,65344,96,900,96,8189,96,8125,96,8127,96,8190,96,697,96,884,96,712,96,714,96,715,96,756,96,699,96,701,96,700,96,702,96,42892,96,1497,96,2036,96,2037,96,5194,96,5836,96,94033,96,94034,96,65339,91,10088,40,10098,40,12308,40,64830,40,65341,93,10089,41,10099,41,12309,41,64831,41,10100,123,119060,123,10101,125,65342,94,8270,42,1645,42,8727,42,66335,42,5941,47,8257,47,8725,47,8260,47,9585,47,10187,47,10744,47,119354,47,12755,47,12339,47,11462,47,20031,47,12035,47,65340,92,65128,92,8726,92,10189,92,10741,92,10745,92,119311,92,119355,92,12756,92,20022,92,12034,92,42872,38,708,94,710,94,5869,43,10133,43,66203,43,8249,60,10094,60,706,60,119350,60,5176,60,5810,60,5120,61,11840,61,12448,61,42239,61,8250,62,10095,62,707,62,119351,62,5171,62,94015,62,8275,126,732,126,8128,126,8764,126,65372,124,65293,45,120784,50,120794,50,120804,50,120814,50,120824,50,130034,50,42842,50,423,50,1000,50,42564,50,5311,50,42735,50,119302,51,120785,51,120795,51,120805,51,120815,51,120825,51,130035,51,42923,51,540,51,439,51,42858,51,11468,51,1248,51,94011,51,71882,51,120786,52,120796,52,120806,52,120816,52,120826,52,130036,52,5070,52,71855,52,120787,53,120797,53,120807,53,120817,53,120827,53,130037,53,444,53,71867,53,120788,54,120798,54,120808,54,120818,54,120828,54,130038,54,11474,54,5102,54,71893,54,119314,55,120789,55,120799,55,120809,55,120819,55,120829,55,130039,55,66770,55,71878,55,2819,56,2538,56,2666,56,125131,56,120790,56,120800,56,120810,56,120820,56,120830,56,130040,56,547,56,546,56,66330,56,2663,57,2920,57,2541,57,3437,57,120791,57,120801,57,120811,57,120821,57,120831,57,130041,57,42862,57,11466,57,71884,57,71852,57,71894,57,9082,97,65345,97,119834,97,119886,97,119938,97,119990,97,120042,97,120094,97,120146,97,120198,97,120250,97,120302,97,120354,97,120406,97,120458,97,593,97,945,97,120514,97,120572,97,120630,97,120688,97,120746,97,65313,65,119808,65,119860,65,119912,65,119964,65,120016,65,120068,65,120120,65,120172,65,120224,65,120276,65,120328,65,120380,65,120432,65,913,65,120488,65,120546,65,120604,65,120662,65,120720,65,5034,65,5573,65,42222,65,94016,65,66208,65,119835,98,119887,98,119939,98,119991,98,120043,98,120095,98,120147,98,120199,98,120251,98,120303,98,120355,98,120407,98,120459,98,388,98,5071,98,5234,98,5551,98,65314,66,8492,66,119809,66,119861,66,119913,66,120017,66,120069,66,120121,66,120173,66,120225,66,120277,66,120329,66,120381,66,120433,66,42932,66,914,66,120489,66,120547,66,120605,66,120663,66,120721,66,5108,66,5623,66,42192,66,66178,66,66209,66,66305,66,65347,99,8573,99,119836,99,119888,99,119940,99,119992,99,120044,99,120096,99,120148,99,120200,99,120252,99,120304,99,120356,99,120408,99,120460,99,7428,99,1010,99,11429,99,43951,99,66621,99,128844,67,71922,67,71913,67,65315,67,8557,67,8450,67,8493,67,119810,67,119862,67,119914,67,119966,67,120018,67,120174,67,120226,67,120278,67,120330,67,120382,67,120434,67,1017,67,11428,67,5087,67,42202,67,66210,67,66306,67,66581,67,66844,67,8574,100,8518,100,119837,100,119889,100,119941,100,119993,100,120045,100,120097,100,120149,100,120201,100,120253,100,120305,100,120357,100,120409,100,120461,100,1281,100,5095,100,5231,100,42194,100,8558,68,8517,68,119811,68,119863,68,119915,68,119967,68,120019,68,120071,68,120123,68,120175,68,120227,68,120279,68,120331,68,120383,68,120435,68,5024,68,5598,68,5610,68,42195,68,8494,101,65349,101,8495,101,8519,101,119838,101,119890,101,119942,101,120046,101,120098,101,120150,101,120202,101,120254,101,120306,101,120358,101,120410,101,120462,101,43826,101,1213,101,8959,69,65317,69,8496,69,119812,69,119864,69,119916,69,120020,69,120072,69,120124,69,120176,69,120228,69,120280,69,120332,69,120384,69,120436,69,917,69,120492,69,120550,69,120608,69,120666,69,120724,69,11577,69,5036,69,42224,69,71846,69,71854,69,66182,69,119839,102,119891,102,119943,102,119995,102,120047,102,120099,102,120151,102,120203,102,120255,102,120307,102,120359,102,120411,102,120463,102,43829,102,42905,102,383,102,7837,102,1412,102,119315,70,8497,70,119813,70,119865,70,119917,70,120021,70,120073,70,120125,70,120177,70,120229,70,120281,70,120333,70,120385,70,120437,70,42904,70,988,70,120778,70,5556,70,42205,70,71874,70,71842,70,66183,70,66213,70,66853,70,65351,103,8458,103,119840,103,119892,103,119944,103,120048,103,120100,103,120152,103,120204,103,120256,103,120308,103,120360,103,120412,103,120464,103,609,103,7555,103,397,103,1409,103,119814,71,119866,71,119918,71,119970,71,120022,71,120074,71,120126,71,120178,71,120230,71,120282,71,120334,71,120386,71,120438,71,1292,71,5056,71,5107,71,42198,71,65352,104,8462,104,119841,104,119945,104,119997,104,120049,104,120101,104,120153,104,120205,104,120257,104,120309,104,120361,104,120413,104,120465,104,1211,104,1392,104,5058,104,65320,72,8459,72,8460,72,8461,72,119815,72,119867,72,119919,72,120023,72,120179,72,120231,72,120283,72,120335,72,120387,72,120439,72,919,72,120494,72,120552,72,120610,72,120668,72,120726,72,11406,72,5051,72,5500,72,42215,72,66255,72,731,105,9075,105,65353,105,8560,105,8505,105,8520,105,119842,105,119894,105,119946,105,119998,105,120050,105,120102,105,120154,105,120206,105,120258,105,120310,105,120362,105,120414,105,120466,105,120484,105,618,105,617,105,953,105,8126,105,890,105,120522,105,120580,105,120638,105,120696,105,120754,105,1110,105,42567,105,1231,105,43893,105,5029,105,71875,105,65354,106,8521,106,119843,106,119895,106,119947,106,119999,106,120051,106,120103,106,120155,106,120207,106,120259,106,120311,106,120363,106,120415,106,120467,106,1011,106,1112,106,65322,74,119817,74,119869,74,119921,74,119973,74,120025,74,120077,74,120129,74,120181,74,120233,74,120285,74,120337,74,120389,74,120441,74,42930,74,895,74,1032,74,5035,74,5261,74,42201,74,119844,107,119896,107,119948,107,120000,107,120052,107,120104,107,120156,107,120208,107,120260,107,120312,107,120364,107,120416,107,120468,107,8490,75,65323,75,119818,75,119870,75,119922,75,119974,75,120026,75,120078,75,120130,75,120182,75,120234,75,120286,75,120338,75,120390,75,120442,75,922,75,120497,75,120555,75,120613,75,120671,75,120729,75,11412,75,5094,75,5845,75,42199,75,66840,75,1472,108,8739,73,9213,73,65512,73,1633,108,1777,73,66336,108,125127,108,120783,73,120793,73,120803,73,120813,73,120823,73,130033,73,65321,73,8544,73,8464,73,8465,73,119816,73,119868,73,119920,73,120024,73,120128,73,120180,73,120232,73,120284,73,120336,73,120388,73,120440,73,65356,108,8572,73,8467,108,119845,108,119897,108,119949,108,120001,108,120053,108,120105,73,120157,73,120209,73,120261,73,120313,73,120365,73,120417,73,120469,73,448,73,120496,73,120554,73,120612,73,120670,73,120728,73,11410,73,1030,73,1216,73,1493,108,1503,108,1575,108,126464,108,126592,108,65166,108,65165,108,1994,108,11599,73,5825,73,42226,73,93992,73,66186,124,66313,124,119338,76,8556,76,8466,76,119819,76,119871,76,119923,76,120027,76,120079,76,120131,76,120183,76,120235,76,120287,76,120339,76,120391,76,120443,76,11472,76,5086,76,5290,76,42209,76,93974,76,71843,76,71858,76,66587,76,66854,76,65325,77,8559,77,8499,77,119820,77,119872,77,119924,77,120028,77,120080,77,120132,77,120184,77,120236,77,120288,77,120340,77,120392,77,120444,77,924,77,120499,77,120557,77,120615,77,120673,77,120731,77,1018,77,11416,77,5047,77,5616,77,5846,77,42207,77,66224,77,66321,77,119847,110,119899,110,119951,110,120003,110,120055,110,120107,110,120159,110,120211,110,120263,110,120315,110,120367,110,120419,110,120471,110,1400,110,1404,110,65326,78,8469,78,119821,78,119873,78,119925,78,119977,78,120029,78,120081,78,120185,78,120237,78,120289,78,120341,78,120393,78,120445,78,925,78,120500,78,120558,78,120616,78,120674,78,120732,78,11418,78,42208,78,66835,78,3074,111,3202,111,3330,111,3458,111,2406,111,2662,111,2790,111,3046,111,3174,111,3302,111,3430,111,3664,111,3792,111,4160,111,1637,111,1781,111,65359,111,8500,111,119848,111,119900,111,119952,111,120056,111,120108,111,120160,111,120212,111,120264,111,120316,111,120368,111,120420,111,120472,111,7439,111,7441,111,43837,111,959,111,120528,111,120586,111,120644,111,120702,111,120760,111,963,111,120532,111,120590,111,120648,111,120706,111,120764,111,11423,111,4351,111,1413,111,1505,111,1607,111,126500,111,126564,111,126596,111,65259,111,65260,111,65258,111,65257,111,1726,111,64428,111,64429,111,64427,111,64426,111,1729,111,64424,111,64425,111,64423,111,64422,111,1749,111,3360,111,4125,111,66794,111,71880,111,71895,111,66604,111,1984,79,2534,79,2918,79,12295,79,70864,79,71904,79,120782,79,120792,79,120802,79,120812,79,120822,79,130032,79,65327,79,119822,79,119874,79,119926,79,119978,79,120030,79,120082,79,120134,79,120186,79,120238,79,120290,79,120342,79,120394,79,120446,79,927,79,120502,79,120560,79,120618,79,120676,79,120734,79,11422,79,1365,79,11604,79,4816,79,2848,79,66754,79,42227,79,71861,79,66194,79,66219,79,66564,79,66838,79,9076,112,65360,112,119849,112,119901,112,119953,112,120005,112,120057,112,120109,112,120161,112,120213,112,120265,112,120317,112,120369,112,120421,112,120473,112,961,112,120530,112,120544,112,120588,112,120602,112,120646,112,120660,112,120704,112,120718,112,120762,112,120776,112,11427,112,65328,80,8473,80,119823,80,119875,80,119927,80,119979,80,120031,80,120083,80,120187,80,120239,80,120291,80,120343,80,120395,80,120447,80,929,80,120504,80,120562,80,120620,80,120678,80,120736,80,11426,80,5090,80,5229,80,42193,80,66197,80,119850,113,119902,113,119954,113,120006,113,120058,113,120110,113,120162,113,120214,113,120266,113,120318,113,120370,113,120422,113,120474,113,1307,113,1379,113,1382,113,8474,81,119824,81,119876,81,119928,81,119980,81,120032,81,120084,81,120188,81,120240,81,120292,81,120344,81,120396,81,120448,81,11605,81,119851,114,119903,114,119955,114,120007,114,120059,114,120111,114,120163,114,120215,114,120267,114,120319,114,120371,114,120423,114,120475,114,43847,114,43848,114,7462,114,11397,114,43905,114,119318,82,8475,82,8476,82,8477,82,119825,82,119877,82,119929,82,120033,82,120189,82,120241,82,120293,82,120345,82,120397,82,120449,82,422,82,5025,82,5074,82,66740,82,5511,82,42211,82,94005,82,65363,115,119852,115,119904,115,119956,115,120008,115,120060,115,120112,115,120164,115,120216,115,120268,115,120320,115,120372,115,120424,115,120476,115,42801,115,445,115,1109,115,43946,115,71873,115,66632,115,65331,83,119826,83,119878,83,119930,83,119982,83,120034,83,120086,83,120138,83,120190,83,120242,83,120294,83,120346,83,120398,83,120450,83,1029,83,1359,83,5077,83,5082,83,42210,83,94010,83,66198,83,66592,83,119853,116,119905,116,119957,116,120009,116,120061,116,120113,116,120165,116,120217,116,120269,116,120321,116,120373,116,120425,116,120477,116,8868,84,10201,84,128872,84,65332,84,119827,84,119879,84,119931,84,119983,84,120035,84,120087,84,120139,84,120191,84,120243,84,120295,84,120347,84,120399,84,120451,84,932,84,120507,84,120565,84,120623,84,120681,84,120739,84,11430,84,5026,84,42196,84,93962,84,71868,84,66199,84,66225,84,66325,84,119854,117,119906,117,119958,117,120010,117,120062,117,120114,117,120166,117,120218,117,120270,117,120322,117,120374,117,120426,117,120478,117,42911,117,7452,117,43854,117,43858,117,651,117,965,117,120534,117,120592,117,120650,117,120708,117,120766,117,1405,117,66806,117,71896,117,8746,85,8899,85,119828,85,119880,85,119932,85,119984,85,120036,85,120088,85,120140,85,120192,85,120244,85,120296,85,120348,85,120400,85,120452,85,1357,85,4608,85,66766,85,5196,85,42228,85,94018,85,71864,85,8744,118,8897,118,65366,118,8564,118,119855,118,119907,118,119959,118,120011,118,120063,118,120115,118,120167,118,120219,118,120271,118,120323,118,120375,118,120427,118,120479,118,7456,118,957,118,120526,118,120584,118,120642,118,120700,118,120758,118,1141,118,1496,118,71430,118,43945,118,71872,118,119309,86,1639,86,1783,86,8548,86,119829,86,119881,86,119933,86,119985,86,120037,86,120089,86,120141,86,120193,86,120245,86,120297,86,120349,86,120401,86,120453,86,1140,86,11576,86,5081,86,5167,86,42719,86,42214,86,93960,86,71840,86,66845,86,623,119,119856,119,119908,119,119960,119,120012,119,120064,119,120116,119,120168,119,120220,119,120272,119,120324,119,120376,119,120428,119,120480,119,7457,119,1121,119,1309,119,1377,119,71434,119,71438,119,71439,119,43907,119,71919,87,71910,87,119830,87,119882,87,119934,87,119986,87,120038,87,120090,87,120142,87,120194,87,120246,87,120298,87,120350,87,120402,87,120454,87,1308,87,5043,87,5076,87,42218,87,5742,120,10539,120,10540,120,10799,120,65368,120,8569,120,119857,120,119909,120,119961,120,120013,120,120065,120,120117,120,120169,120,120221,120,120273,120,120325,120,120377,120,120429,120,120481,120,5441,120,5501,120,5741,88,9587,88,66338,88,71916,88,65336,88,8553,88,119831,88,119883,88,119935,88,119987,88,120039,88,120091,88,120143,88,120195,88,120247,88,120299,88,120351,88,120403,88,120455,88,42931,88,935,88,120510,88,120568,88,120626,88,120684,88,120742,88,11436,88,11613,88,5815,88,42219,88,66192,88,66228,88,66327,88,66855,88,611,121,7564,121,65369,121,119858,121,119910,121,119962,121,120014,121,120066,121,120118,121,120170,121,120222,121,120274,121,120326,121,120378,121,120430,121,120482,121,655,121,7935,121,43866,121,947,121,8509,121,120516,121,120574,121,120632,121,120690,121,120748,121,1199,121,4327,121,71900,121,65337,89,119832,89,119884,89,119936,89,119988,89,120040,89,120092,89,120144,89,120196,89,120248,89,120300,89,120352,89,120404,89,120456,89,933,89,978,89,120508,89,120566,89,120624,89,120682,89,120740,89,11432,89,1198,89,5033,89,5053,89,42220,89,94019,89,71844,89,66226,89,119859,122,119911,122,119963,122,120015,122,120067,122,120119,122,120171,122,120223,122,120275,122,120327,122,120379,122,120431,122,120483,122,7458,122,43923,122,71876,122,66293,90,71909,90,65338,90,8484,90,8488,90,119833,90,119885,90,119937,90,119989,90,120041,90,120197,90,120249,90,120301,90,120353,90,120405,90,120457,90,918,90,120493,90,120551,90,120609,90,120667,90,120725,90,5059,90,42204,90,71849,90,65282,34,65284,36,65285,37,65286,38,65290,42,65291,43,65294,46,65295,47,65296,48,65297,49,65298,50,65299,51,65300,52,65301,53,65302,54,65303,55,65304,56,65305,57,65308,60,65309,61,65310,62,65312,64,65316,68,65318,70,65319,71,65324,76,65329,81,65330,82,65333,85,65334,86,65335,87,65343,95,65346,98,65348,100,65350,102,65355,107,65357,109,65358,110,65361,113,65362,114,65364,116,65365,117,65367,119,65370,122,65371,123,65373,125],\"_default\":[160,32,8211,45,65374,126,65306,58,65281,33,8216,96,8217,96,8245,96,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"cs\":[65374,126,65306,58,65281,33,8216,96,8217,96,8245,96,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,1093,120,1061,88,1091,121,1059,89,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"de\":[65374,126,65306,58,65281,33,8216,96,8217,96,8245,96,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,1093,120,1061,88,1091,121,1059,89,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"es\":[8211,45,65374,126,65306,58,65281,33,8245,96,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"fr\":[65374,126,65306,58,65281,33,8216,96,8245,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"it\":[160,32,8211,45,65374,126,65306,58,65281,33,8216,96,8245,96,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"ja\":[8211,45,65306,58,65281,33,8216,96,8217,96,8245,96,180,96,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65283,35,65292,44,65307,59],\"ko\":[8211,45,65374,126,65306,58,65281,33,8245,96,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"pl\":[65374,126,65306,58,65281,33,8216,96,8217,96,8245,96,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"pt-BR\":[65374,126,65306,58,65281,33,8216,96,8217,96,8245,96,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"qps-ploc\":[160,32,8211,45,65374,126,65306,58,65281,33,8216,96,8217,96,8245,96,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"ru\":[65374,126,65306,58,65281,33,8216,96,8217,96,8245,96,180,96,12494,47,305,105,921,73,1009,112,215,120,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"tr\":[160,32,8211,45,65374,126,65306,58,65281,33,8245,96,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65283,35,65288,40,65289,41,65292,44,65307,59,65311,63],\"zh-hans\":[65374,126,65306,58,65281,33,8245,96,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65288,40,65289,41],\"zh-hant\":[8211,45,65374,126,180,96,12494,47,1047,51,1073,54,1072,97,1040,65,1068,98,1042,66,1089,99,1057,67,1077,101,1045,69,1053,72,305,105,1050,75,921,73,1052,77,1086,111,1054,79,1009,112,1088,112,1056,80,1075,114,1058,84,215,120,1093,120,1061,88,1091,121,1059,89,65283,35,65307,59]}');
    });
    AmbiguousCharacters.cache = new LRUCachedComputed((locales) => {
        function arrayToMap(arr) {
            const result = new Map();
            for (let i = 0; i < arr.length; i += 2) {
                result.set(arr[i], arr[i + 1]);
            }
            return result;
        }
        function mergeMaps(map1, map2) {
            const result = new Map(map1);
            for (const [key, value] of map2) {
                result.set(key, value);
            }
            return result;
        }
        function intersectMaps(map1, map2) {
            if (!map1) {
                return map2;
            }
            const result = new Map();
            for (const [key, value] of map1) {
                if (map2.has(key)) {
                    result.set(key, value);
                }
            }
            return result;
        }
        const data = _a.ambiguousCharacterData.getValue();
        let filteredLocales = locales.filter((l) => !l.startsWith('_') && l in data);
        if (filteredLocales.length === 0) {
            filteredLocales = ['_default'];
        }
        let languageSpecificMap = undefined;
        for (const locale of filteredLocales) {
            const map = arrayToMap(data[locale]);
            languageSpecificMap = intersectMaps(languageSpecificMap, map);
        }
        const commonMap = arrayToMap(data['_common']);
        const map = mergeMaps(commonMap, languageSpecificMap);
        return new AmbiguousCharacters(map);
    });
    AmbiguousCharacters._locales = new Lazy(() => Object.keys(AmbiguousCharacters.ambiguousCharacterData.getValue()).filter((k) => !k.startsWith('_')));
    class InvisibleCharacters {
        static getRawData() {
            // Generated using https://github.com/hediet/vscode-unicode-data
            return JSON.parse('[9,10,11,12,13,32,127,160,173,847,1564,4447,4448,6068,6069,6155,6156,6157,6158,7355,7356,8192,8193,8194,8195,8196,8197,8198,8199,8200,8201,8202,8203,8204,8205,8206,8207,8234,8235,8236,8237,8238,8239,8287,8288,8289,8290,8291,8292,8293,8294,8295,8296,8297,8298,8299,8300,8301,8302,8303,10240,12288,12644,65024,65025,65026,65027,65028,65029,65030,65031,65032,65033,65034,65035,65036,65037,65038,65039,65279,65440,65520,65521,65522,65523,65524,65525,65526,65527,65528,65532,78844,119155,119156,119157,119158,119159,119160,119161,119162,917504,917505,917506,917507,917508,917509,917510,917511,917512,917513,917514,917515,917516,917517,917518,917519,917520,917521,917522,917523,917524,917525,917526,917527,917528,917529,917530,917531,917532,917533,917534,917535,917536,917537,917538,917539,917540,917541,917542,917543,917544,917545,917546,917547,917548,917549,917550,917551,917552,917553,917554,917555,917556,917557,917558,917559,917560,917561,917562,917563,917564,917565,917566,917567,917568,917569,917570,917571,917572,917573,917574,917575,917576,917577,917578,917579,917580,917581,917582,917583,917584,917585,917586,917587,917588,917589,917590,917591,917592,917593,917594,917595,917596,917597,917598,917599,917600,917601,917602,917603,917604,917605,917606,917607,917608,917609,917610,917611,917612,917613,917614,917615,917616,917617,917618,917619,917620,917621,917622,917623,917624,917625,917626,917627,917628,917629,917630,917631,917760,917761,917762,917763,917764,917765,917766,917767,917768,917769,917770,917771,917772,917773,917774,917775,917776,917777,917778,917779,917780,917781,917782,917783,917784,917785,917786,917787,917788,917789,917790,917791,917792,917793,917794,917795,917796,917797,917798,917799,917800,917801,917802,917803,917804,917805,917806,917807,917808,917809,917810,917811,917812,917813,917814,917815,917816,917817,917818,917819,917820,917821,917822,917823,917824,917825,917826,917827,917828,917829,917830,917831,917832,917833,917834,917835,917836,917837,917838,917839,917840,917841,917842,917843,917844,917845,917846,917847,917848,917849,917850,917851,917852,917853,917854,917855,917856,917857,917858,917859,917860,917861,917862,917863,917864,917865,917866,917867,917868,917869,917870,917871,917872,917873,917874,917875,917876,917877,917878,917879,917880,917881,917882,917883,917884,917885,917886,917887,917888,917889,917890,917891,917892,917893,917894,917895,917896,917897,917898,917899,917900,917901,917902,917903,917904,917905,917906,917907,917908,917909,917910,917911,917912,917913,917914,917915,917916,917917,917918,917919,917920,917921,917922,917923,917924,917925,917926,917927,917928,917929,917930,917931,917932,917933,917934,917935,917936,917937,917938,917939,917940,917941,917942,917943,917944,917945,917946,917947,917948,917949,917950,917951,917952,917953,917954,917955,917956,917957,917958,917959,917960,917961,917962,917963,917964,917965,917966,917967,917968,917969,917970,917971,917972,917973,917974,917975,917976,917977,917978,917979,917980,917981,917982,917983,917984,917985,917986,917987,917988,917989,917990,917991,917992,917993,917994,917995,917996,917997,917998,917999]');
        }
        static getData() {
            if (!this._data) {
                this._data = new Set(InvisibleCharacters.getRawData());
            }
            return this._data;
        }
        static isInvisibleCharacter(codePoint) {
            return InvisibleCharacters.getData().has(codePoint);
        }
        static get codePoints() {
            return InvisibleCharacters.getData();
        }
    }
    InvisibleCharacters._data = undefined;

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    const INITIALIZE = '$initialize';
    class RequestMessage {
        constructor(vsWorker, req, method, args) {
            this.vsWorker = vsWorker;
            this.req = req;
            this.method = method;
            this.args = args;
            this.type = 0 /* Request */;
        }
    }
    class ReplyMessage {
        constructor(vsWorker, seq, res, err) {
            this.vsWorker = vsWorker;
            this.seq = seq;
            this.res = res;
            this.err = err;
            this.type = 1 /* Reply */;
        }
    }
    class SubscribeEventMessage {
        constructor(vsWorker, req, eventName, arg) {
            this.vsWorker = vsWorker;
            this.req = req;
            this.eventName = eventName;
            this.arg = arg;
            this.type = 2 /* SubscribeEvent */;
        }
    }
    class EventMessage {
        constructor(vsWorker, req, event) {
            this.vsWorker = vsWorker;
            this.req = req;
            this.event = event;
            this.type = 3 /* Event */;
        }
    }
    class UnsubscribeEventMessage {
        constructor(vsWorker, req) {
            this.vsWorker = vsWorker;
            this.req = req;
            this.type = 4 /* UnsubscribeEvent */;
        }
    }
    class SimpleWorkerProtocol {
        constructor(handler) {
            this._workerId = -1;
            this._handler = handler;
            this._lastSentReq = 0;
            this._pendingReplies = Object.create(null);
            this._pendingEmitters = new Map();
            this._pendingEvents = new Map();
        }
        setWorkerId(workerId) {
            this._workerId = workerId;
        }
        sendMessage(method, args) {
            const req = String(++this._lastSentReq);
            return new Promise((resolve, reject) => {
                this._pendingReplies[req] = {
                    resolve: resolve,
                    reject: reject
                };
                this._send(new RequestMessage(this._workerId, req, method, args));
            });
        }
        listen(eventName, arg) {
            let req = null;
            const emitter = new Emitter({
                onFirstListenerAdd: () => {
                    req = String(++this._lastSentReq);
                    this._pendingEmitters.set(req, emitter);
                    this._send(new SubscribeEventMessage(this._workerId, req, eventName, arg));
                },
                onLastListenerRemove: () => {
                    this._pendingEmitters.delete(req);
                    this._send(new UnsubscribeEventMessage(this._workerId, req));
                    req = null;
                }
            });
            return emitter.event;
        }
        handleMessage(message) {
            if (!message || !message.vsWorker) {
                return;
            }
            if (this._workerId !== -1 && message.vsWorker !== this._workerId) {
                return;
            }
            this._handleMessage(message);
        }
        _handleMessage(msg) {
            switch (msg.type) {
                case 1 /* Reply */:
                    return this._handleReplyMessage(msg);
                case 0 /* Request */:
                    return this._handleRequestMessage(msg);
                case 2 /* SubscribeEvent */:
                    return this._handleSubscribeEventMessage(msg);
                case 3 /* Event */:
                    return this._handleEventMessage(msg);
                case 4 /* UnsubscribeEvent */:
                    return this._handleUnsubscribeEventMessage(msg);
            }
        }
        _handleReplyMessage(replyMessage) {
            if (!this._pendingReplies[replyMessage.seq]) {
                console.warn('Got reply to unknown seq');
                return;
            }
            let reply = this._pendingReplies[replyMessage.seq];
            delete this._pendingReplies[replyMessage.seq];
            if (replyMessage.err) {
                let err = replyMessage.err;
                if (replyMessage.err.$isError) {
                    err = new Error();
                    err.name = replyMessage.err.name;
                    err.message = replyMessage.err.message;
                    err.stack = replyMessage.err.stack;
                }
                reply.reject(err);
                return;
            }
            reply.resolve(replyMessage.res);
        }
        _handleRequestMessage(requestMessage) {
            let req = requestMessage.req;
            let result = this._handler.handleMessage(requestMessage.method, requestMessage.args);
            result.then((r) => {
                this._send(new ReplyMessage(this._workerId, req, r, undefined));
            }, (e) => {
                if (e.detail instanceof Error) {
                    // Loading errors have a detail property that points to the actual error
                    e.detail = transformErrorForSerialization(e.detail);
                }
                this._send(new ReplyMessage(this._workerId, req, undefined, transformErrorForSerialization(e)));
            });
        }
        _handleSubscribeEventMessage(msg) {
            const req = msg.req;
            const disposable = this._handler.handleEvent(msg.eventName, msg.arg)((event) => {
                this._send(new EventMessage(this._workerId, req, event));
            });
            this._pendingEvents.set(req, disposable);
        }
        _handleEventMessage(msg) {
            if (!this._pendingEmitters.has(msg.req)) {
                console.warn('Got event for unknown req');
                return;
            }
            this._pendingEmitters.get(msg.req).fire(msg.event);
        }
        _handleUnsubscribeEventMessage(msg) {
            if (!this._pendingEvents.has(msg.req)) {
                console.warn('Got unsubscribe for unknown req');
                return;
            }
            this._pendingEvents.get(msg.req).dispose();
            this._pendingEvents.delete(msg.req);
        }
        _send(msg) {
            let transfer = [];
            if (msg.type === 0 /* Request */) {
                for (let i = 0; i < msg.args.length; i++) {
                    if (msg.args[i] instanceof ArrayBuffer) {
                        transfer.push(msg.args[i]);
                    }
                }
            }
            else if (msg.type === 1 /* Reply */) {
                if (msg.res instanceof ArrayBuffer) {
                    transfer.push(msg.res);
                }
            }
            this._handler.sendMessage(msg, transfer);
        }
    }
    function propertyIsEvent(name) {
        // Assume a property is an event if it has a form of "onSomething"
        return name[0] === 'o' && name[1] === 'n' && isUpperAsciiLetter(name.charCodeAt(2));
    }
    function propertyIsDynamicEvent(name) {
        // Assume a property is a dynamic event (a method that returns an event) if it has a form of "onDynamicSomething"
        return /^onDynamic/.test(name) && isUpperAsciiLetter(name.charCodeAt(9));
    }
    function createProxyObject(methodNames, invoke, proxyListen) {
        const createProxyMethod = (method) => {
            return function () {
                const args = Array.prototype.slice.call(arguments, 0);
                return invoke(method, args);
            };
        };
        const createProxyDynamicEvent = (eventName) => {
            return function (arg) {
                return proxyListen(eventName, arg);
            };
        };
        let result = {};
        for (const methodName of methodNames) {
            if (propertyIsDynamicEvent(methodName)) {
                result[methodName] = createProxyDynamicEvent(methodName);
                continue;
            }
            if (propertyIsEvent(methodName)) {
                result[methodName] = proxyListen(methodName, undefined);
                continue;
            }
            result[methodName] = createProxyMethod(methodName);
        }
        return result;
    }
    /**
     * Worker side
     */
    class SimpleWorkerServer {
        constructor(postMessage, requestHandlerFactory) {
            this._requestHandlerFactory = requestHandlerFactory;
            this._requestHandler = null;
            this._protocol = new SimpleWorkerProtocol({
                sendMessage: (msg, transfer) => {
                    postMessage(msg, transfer);
                },
                handleMessage: (method, args) => this._handleMessage(method, args),
                handleEvent: (eventName, arg) => this._handleEvent(eventName, arg)
            });
        }
        onmessage(msg) {
            this._protocol.handleMessage(msg);
        }
        _handleMessage(method, args) {
            if (method === INITIALIZE) {
                return this.initialize(args[0], args[1], args[2], args[3]);
            }
            if (!this._requestHandler || typeof this._requestHandler[method] !== 'function') {
                return Promise.reject(new Error('Missing requestHandler or method: ' + method));
            }
            try {
                return Promise.resolve(this._requestHandler[method].apply(this._requestHandler, args));
            }
            catch (e) {
                return Promise.reject(e);
            }
        }
        _handleEvent(eventName, arg) {
            if (!this._requestHandler) {
                throw new Error(`Missing requestHandler`);
            }
            if (propertyIsDynamicEvent(eventName)) {
                const event = this._requestHandler[eventName].call(this._requestHandler, arg);
                if (typeof event !== 'function') {
                    throw new Error(`Missing dynamic event ${eventName} on request handler.`);
                }
                return event;
            }
            if (propertyIsEvent(eventName)) {
                const event = this._requestHandler[eventName];
                if (typeof event !== 'function') {
                    throw new Error(`Missing event ${eventName} on request handler.`);
                }
                return event;
            }
            throw new Error(`Malformed event name ${eventName}`);
        }
        initialize(workerId, loaderConfig, moduleId, hostMethods) {
            this._protocol.setWorkerId(workerId);
            const proxyMethodRequest = (method, args) => {
                return this._protocol.sendMessage(method, args);
            };
            const proxyListen = (eventName, arg) => {
                return this._protocol.listen(eventName, arg);
            };
            const hostProxy = createProxyObject(hostMethods, proxyMethodRequest, proxyListen);
            if (this._requestHandlerFactory) {
                // static request handler
                this._requestHandler = this._requestHandlerFactory(hostProxy);
                return Promise.resolve(getAllMethodNames(this._requestHandler));
            }
            if (loaderConfig) {
                // Remove 'baseUrl', handling it is beyond scope for now
                if (typeof loaderConfig.baseUrl !== 'undefined') {
                    delete loaderConfig['baseUrl'];
                }
                if (typeof loaderConfig.paths !== 'undefined') {
                    if (typeof loaderConfig.paths.vs !== 'undefined') {
                        delete loaderConfig.paths['vs'];
                    }
                }
                if (typeof loaderConfig.trustedTypesPolicy !== undefined) {
                    // don't use, it has been destroyed during serialize
                    delete loaderConfig['trustedTypesPolicy'];
                }
                // Since this is in a web worker, enable catching errors
                loaderConfig.catchError = true;
                globals.require.config(loaderConfig);
            }
            return new Promise((resolve, reject) => {
                // Use the global require to be sure to get the global config
                // ESM-comment-begin
                // 			const req = (globals.require || require);
                // ESM-comment-end
                // ESM-uncomment-begin
                const req = globals.require;
                // ESM-uncomment-end
                req([moduleId], (module) => {
                    this._requestHandler = module.create(hostProxy);
                    if (!this._requestHandler) {
                        reject(new Error(`No RequestHandler!`));
                        return;
                    }
                    resolve(getAllMethodNames(this._requestHandler));
                }, reject);
            });
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    /**
     * Represents information about a specific difference between two sequences.
     */
    class DiffChange {
        /**
         * Constructs a new DiffChange with the given sequence information
         * and content.
         */
        constructor(originalStart, originalLength, modifiedStart, modifiedLength) {
            //Debug.Assert(originalLength > 0 || modifiedLength > 0, "originalLength and modifiedLength cannot both be <= 0");
            this.originalStart = originalStart;
            this.originalLength = originalLength;
            this.modifiedStart = modifiedStart;
            this.modifiedLength = modifiedLength;
        }
        /**
         * The end point (exclusive) of the change in the original sequence.
         */
        getOriginalEnd() {
            return this.originalStart + this.originalLength;
        }
        /**
         * The end point (exclusive) of the change in the modified sequence.
         */
        getModifiedEnd() {
            return this.modifiedStart + this.modifiedLength;
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    function numberHash(val, initialHashVal) {
        return (((initialHashVal << 5) - initialHashVal) + val) | 0; // hashVal * 31 + ch, keep as int32
    }
    function stringHash(s, hashVal) {
        hashVal = numberHash(149417, hashVal);
        for (let i = 0, length = s.length; i < length; i++) {
            hashVal = numberHash(s.charCodeAt(i), hashVal);
        }
        return hashVal;
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    class StringDiffSequence {
        constructor(source) {
            this.source = source;
        }
        getElements() {
            const source = this.source;
            const characters = new Int32Array(source.length);
            for (let i = 0, len = source.length; i < len; i++) {
                characters[i] = source.charCodeAt(i);
            }
            return characters;
        }
    }
    function stringDiff(original, modified, pretty) {
        return new LcsDiff(new StringDiffSequence(original), new StringDiffSequence(modified)).ComputeDiff(pretty).changes;
    }
    //
    // The code below has been ported from a C# implementation in VS
    //
    class Debug {
        static Assert(condition, message) {
            if (!condition) {
                throw new Error(message);
            }
        }
    }
    class MyArray {
        /**
         * Copies a range of elements from an Array starting at the specified source index and pastes
         * them to another Array starting at the specified destination index. The length and the indexes
         * are specified as 64-bit integers.
         * sourceArray:
         *		The Array that contains the data to copy.
         * sourceIndex:
         *		A 64-bit integer that represents the index in the sourceArray at which copying begins.
         * destinationArray:
         *		The Array that receives the data.
         * destinationIndex:
         *		A 64-bit integer that represents the index in the destinationArray at which storing begins.
         * length:
         *		A 64-bit integer that represents the number of elements to copy.
         */
        static Copy(sourceArray, sourceIndex, destinationArray, destinationIndex, length) {
            for (let i = 0; i < length; i++) {
                destinationArray[destinationIndex + i] = sourceArray[sourceIndex + i];
            }
        }
        static Copy2(sourceArray, sourceIndex, destinationArray, destinationIndex, length) {
            for (let i = 0; i < length; i++) {
                destinationArray[destinationIndex + i] = sourceArray[sourceIndex + i];
            }
        }
    }
    /**
     * A utility class which helps to create the set of DiffChanges from
     * a difference operation. This class accepts original DiffElements and
     * modified DiffElements that are involved in a particular change. The
     * MarkNextChange() method can be called to mark the separation between
     * distinct changes. At the end, the Changes property can be called to retrieve
     * the constructed changes.
     */
    class DiffChangeHelper {
        /**
         * Constructs a new DiffChangeHelper for the given DiffSequences.
         */
        constructor() {
            this.m_changes = [];
            this.m_originalStart = 1073741824 /* MAX_SAFE_SMALL_INTEGER */;
            this.m_modifiedStart = 1073741824 /* MAX_SAFE_SMALL_INTEGER */;
            this.m_originalCount = 0;
            this.m_modifiedCount = 0;
        }
        /**
         * Marks the beginning of the next change in the set of differences.
         */
        MarkNextChange() {
            // Only add to the list if there is something to add
            if (this.m_originalCount > 0 || this.m_modifiedCount > 0) {
                // Add the new change to our list
                this.m_changes.push(new DiffChange(this.m_originalStart, this.m_originalCount, this.m_modifiedStart, this.m_modifiedCount));
            }
            // Reset for the next change
            this.m_originalCount = 0;
            this.m_modifiedCount = 0;
            this.m_originalStart = 1073741824 /* MAX_SAFE_SMALL_INTEGER */;
            this.m_modifiedStart = 1073741824 /* MAX_SAFE_SMALL_INTEGER */;
        }
        /**
         * Adds the original element at the given position to the elements
         * affected by the current change. The modified index gives context
         * to the change position with respect to the original sequence.
         * @param originalIndex The index of the original element to add.
         * @param modifiedIndex The index of the modified element that provides corresponding position in the modified sequence.
         */
        AddOriginalElement(originalIndex, modifiedIndex) {
            // The 'true' start index is the smallest of the ones we've seen
            this.m_originalStart = Math.min(this.m_originalStart, originalIndex);
            this.m_modifiedStart = Math.min(this.m_modifiedStart, modifiedIndex);
            this.m_originalCount++;
        }
        /**
         * Adds the modified element at the given position to the elements
         * affected by the current change. The original index gives context
         * to the change position with respect to the modified sequence.
         * @param originalIndex The index of the original element that provides corresponding position in the original sequence.
         * @param modifiedIndex The index of the modified element to add.
         */
        AddModifiedElement(originalIndex, modifiedIndex) {
            // The 'true' start index is the smallest of the ones we've seen
            this.m_originalStart = Math.min(this.m_originalStart, originalIndex);
            this.m_modifiedStart = Math.min(this.m_modifiedStart, modifiedIndex);
            this.m_modifiedCount++;
        }
        /**
         * Retrieves all of the changes marked by the class.
         */
        getChanges() {
            if (this.m_originalCount > 0 || this.m_modifiedCount > 0) {
                // Finish up on whatever is left
                this.MarkNextChange();
            }
            return this.m_changes;
        }
        /**
         * Retrieves all of the changes marked by the class in the reverse order
         */
        getReverseChanges() {
            if (this.m_originalCount > 0 || this.m_modifiedCount > 0) {
                // Finish up on whatever is left
                this.MarkNextChange();
            }
            this.m_changes.reverse();
            return this.m_changes;
        }
    }
    /**
     * An implementation of the difference algorithm described in
     * "An O(ND) Difference Algorithm and its variations" by Eugene W. Myers
     */
    class LcsDiff {
        /**
         * Constructs the DiffFinder
         */
        constructor(originalSequence, modifiedSequence, continueProcessingPredicate = null) {
            this.ContinueProcessingPredicate = continueProcessingPredicate;
            this._originalSequence = originalSequence;
            this._modifiedSequence = modifiedSequence;
            const [originalStringElements, originalElementsOrHash, originalHasStrings] = LcsDiff._getElements(originalSequence);
            const [modifiedStringElements, modifiedElementsOrHash, modifiedHasStrings] = LcsDiff._getElements(modifiedSequence);
            this._hasStrings = (originalHasStrings && modifiedHasStrings);
            this._originalStringElements = originalStringElements;
            this._originalElementsOrHash = originalElementsOrHash;
            this._modifiedStringElements = modifiedStringElements;
            this._modifiedElementsOrHash = modifiedElementsOrHash;
            this.m_forwardHistory = [];
            this.m_reverseHistory = [];
        }
        static _isStringArray(arr) {
            return (arr.length > 0 && typeof arr[0] === 'string');
        }
        static _getElements(sequence) {
            const elements = sequence.getElements();
            if (LcsDiff._isStringArray(elements)) {
                const hashes = new Int32Array(elements.length);
                for (let i = 0, len = elements.length; i < len; i++) {
                    hashes[i] = stringHash(elements[i], 0);
                }
                return [elements, hashes, true];
            }
            if (elements instanceof Int32Array) {
                return [[], elements, false];
            }
            return [[], new Int32Array(elements), false];
        }
        ElementsAreEqual(originalIndex, newIndex) {
            if (this._originalElementsOrHash[originalIndex] !== this._modifiedElementsOrHash[newIndex]) {
                return false;
            }
            return (this._hasStrings ? this._originalStringElements[originalIndex] === this._modifiedStringElements[newIndex] : true);
        }
        ElementsAreStrictEqual(originalIndex, newIndex) {
            if (!this.ElementsAreEqual(originalIndex, newIndex)) {
                return false;
            }
            const originalElement = LcsDiff._getStrictElement(this._originalSequence, originalIndex);
            const modifiedElement = LcsDiff._getStrictElement(this._modifiedSequence, newIndex);
            return (originalElement === modifiedElement);
        }
        static _getStrictElement(sequence, index) {
            if (typeof sequence.getStrictElement === 'function') {
                return sequence.getStrictElement(index);
            }
            return null;
        }
        OriginalElementsAreEqual(index1, index2) {
            if (this._originalElementsOrHash[index1] !== this._originalElementsOrHash[index2]) {
                return false;
            }
            return (this._hasStrings ? this._originalStringElements[index1] === this._originalStringElements[index2] : true);
        }
        ModifiedElementsAreEqual(index1, index2) {
            if (this._modifiedElementsOrHash[index1] !== this._modifiedElementsOrHash[index2]) {
                return false;
            }
            return (this._hasStrings ? this._modifiedStringElements[index1] === this._modifiedStringElements[index2] : true);
        }
        ComputeDiff(pretty) {
            return this._ComputeDiff(0, this._originalElementsOrHash.length - 1, 0, this._modifiedElementsOrHash.length - 1, pretty);
        }
        /**
         * Computes the differences between the original and modified input
         * sequences on the bounded range.
         * @returns An array of the differences between the two input sequences.
         */
        _ComputeDiff(originalStart, originalEnd, modifiedStart, modifiedEnd, pretty) {
            const quitEarlyArr = [false];
            let changes = this.ComputeDiffRecursive(originalStart, originalEnd, modifiedStart, modifiedEnd, quitEarlyArr);
            if (pretty) {
                // We have to clean up the computed diff to be more intuitive
                // but it turns out this cannot be done correctly until the entire set
                // of diffs have been computed
                changes = this.PrettifyChanges(changes);
            }
            return {
                quitEarly: quitEarlyArr[0],
                changes: changes
            };
        }
        /**
         * Private helper method which computes the differences on the bounded range
         * recursively.
         * @returns An array of the differences between the two input sequences.
         */
        ComputeDiffRecursive(originalStart, originalEnd, modifiedStart, modifiedEnd, quitEarlyArr) {
            quitEarlyArr[0] = false;
            // Find the start of the differences
            while (originalStart <= originalEnd && modifiedStart <= modifiedEnd && this.ElementsAreEqual(originalStart, modifiedStart)) {
                originalStart++;
                modifiedStart++;
            }
            // Find the end of the differences
            while (originalEnd >= originalStart && modifiedEnd >= modifiedStart && this.ElementsAreEqual(originalEnd, modifiedEnd)) {
                originalEnd--;
                modifiedEnd--;
            }
            // In the special case where we either have all insertions or all deletions or the sequences are identical
            if (originalStart > originalEnd || modifiedStart > modifiedEnd) {
                let changes;
                if (modifiedStart <= modifiedEnd) {
                    Debug.Assert(originalStart === originalEnd + 1, 'originalStart should only be one more than originalEnd');
                    // All insertions
                    changes = [
                        new DiffChange(originalStart, 0, modifiedStart, modifiedEnd - modifiedStart + 1)
                    ];
                }
                else if (originalStart <= originalEnd) {
                    Debug.Assert(modifiedStart === modifiedEnd + 1, 'modifiedStart should only be one more than modifiedEnd');
                    // All deletions
                    changes = [
                        new DiffChange(originalStart, originalEnd - originalStart + 1, modifiedStart, 0)
                    ];
                }
                else {
                    Debug.Assert(originalStart === originalEnd + 1, 'originalStart should only be one more than originalEnd');
                    Debug.Assert(modifiedStart === modifiedEnd + 1, 'modifiedStart should only be one more than modifiedEnd');
                    // Identical sequences - No differences
                    changes = [];
                }
                return changes;
            }
            // This problem can be solved using the Divide-And-Conquer technique.
            const midOriginalArr = [0];
            const midModifiedArr = [0];
            const result = this.ComputeRecursionPoint(originalStart, originalEnd, modifiedStart, modifiedEnd, midOriginalArr, midModifiedArr, quitEarlyArr);
            const midOriginal = midOriginalArr[0];
            const midModified = midModifiedArr[0];
            if (result !== null) {
                // Result is not-null when there was enough memory to compute the changes while
                // searching for the recursion point
                return result;
            }
            else if (!quitEarlyArr[0]) {
                // We can break the problem down recursively by finding the changes in the
                // First Half:   (originalStart, modifiedStart) to (midOriginal, midModified)
                // Second Half:  (midOriginal + 1, minModified + 1) to (originalEnd, modifiedEnd)
                // NOTE: ComputeDiff() is inclusive, therefore the second range starts on the next point
                const leftChanges = this.ComputeDiffRecursive(originalStart, midOriginal, modifiedStart, midModified, quitEarlyArr);
                let rightChanges = [];
                if (!quitEarlyArr[0]) {
                    rightChanges = this.ComputeDiffRecursive(midOriginal + 1, originalEnd, midModified + 1, modifiedEnd, quitEarlyArr);
                }
                else {
                    // We didn't have time to finish the first half, so we don't have time to compute this half.
                    // Consider the entire rest of the sequence different.
                    rightChanges = [
                        new DiffChange(midOriginal + 1, originalEnd - (midOriginal + 1) + 1, midModified + 1, modifiedEnd - (midModified + 1) + 1)
                    ];
                }
                return this.ConcatenateChanges(leftChanges, rightChanges);
            }
            // If we hit here, we quit early, and so can't return anything meaningful
            return [
                new DiffChange(originalStart, originalEnd - originalStart + 1, modifiedStart, modifiedEnd - modifiedStart + 1)
            ];
        }
        WALKTRACE(diagonalForwardBase, diagonalForwardStart, diagonalForwardEnd, diagonalForwardOffset, diagonalReverseBase, diagonalReverseStart, diagonalReverseEnd, diagonalReverseOffset, forwardPoints, reversePoints, originalIndex, originalEnd, midOriginalArr, modifiedIndex, modifiedEnd, midModifiedArr, deltaIsEven, quitEarlyArr) {
            let forwardChanges = null;
            let reverseChanges = null;
            // First, walk backward through the forward diagonals history
            let changeHelper = new DiffChangeHelper();
            let diagonalMin = diagonalForwardStart;
            let diagonalMax = diagonalForwardEnd;
            let diagonalRelative = (midOriginalArr[0] - midModifiedArr[0]) - diagonalForwardOffset;
            let lastOriginalIndex = -1073741824 /* MIN_SAFE_SMALL_INTEGER */;
            let historyIndex = this.m_forwardHistory.length - 1;
            do {
                // Get the diagonal index from the relative diagonal number
                const diagonal = diagonalRelative + diagonalForwardBase;
                // Figure out where we came from
                if (diagonal === diagonalMin || (diagonal < diagonalMax && forwardPoints[diagonal - 1] < forwardPoints[diagonal + 1])) {
                    // Vertical line (the element is an insert)
                    originalIndex = forwardPoints[diagonal + 1];
                    modifiedIndex = originalIndex - diagonalRelative - diagonalForwardOffset;
                    if (originalIndex < lastOriginalIndex) {
                        changeHelper.MarkNextChange();
                    }
                    lastOriginalIndex = originalIndex;
                    changeHelper.AddModifiedElement(originalIndex + 1, modifiedIndex);
                    diagonalRelative = (diagonal + 1) - diagonalForwardBase; //Setup for the next iteration
                }
                else {
                    // Horizontal line (the element is a deletion)
                    originalIndex = forwardPoints[diagonal - 1] + 1;
                    modifiedIndex = originalIndex - diagonalRelative - diagonalForwardOffset;
                    if (originalIndex < lastOriginalIndex) {
                        changeHelper.MarkNextChange();
                    }
                    lastOriginalIndex = originalIndex - 1;
                    changeHelper.AddOriginalElement(originalIndex, modifiedIndex + 1);
                    diagonalRelative = (diagonal - 1) - diagonalForwardBase; //Setup for the next iteration
                }
                if (historyIndex >= 0) {
                    forwardPoints = this.m_forwardHistory[historyIndex];
                    diagonalForwardBase = forwardPoints[0]; //We stored this in the first spot
                    diagonalMin = 1;
                    diagonalMax = forwardPoints.length - 1;
                }
            } while (--historyIndex >= -1);
            // Ironically, we get the forward changes as the reverse of the
            // order we added them since we technically added them backwards
            forwardChanges = changeHelper.getReverseChanges();
            if (quitEarlyArr[0]) {
                // TODO: Calculate a partial from the reverse diagonals.
                //       For now, just assume everything after the midOriginal/midModified point is a diff
                let originalStartPoint = midOriginalArr[0] + 1;
                let modifiedStartPoint = midModifiedArr[0] + 1;
                if (forwardChanges !== null && forwardChanges.length > 0) {
                    const lastForwardChange = forwardChanges[forwardChanges.length - 1];
                    originalStartPoint = Math.max(originalStartPoint, lastForwardChange.getOriginalEnd());
                    modifiedStartPoint = Math.max(modifiedStartPoint, lastForwardChange.getModifiedEnd());
                }
                reverseChanges = [
                    new DiffChange(originalStartPoint, originalEnd - originalStartPoint + 1, modifiedStartPoint, modifiedEnd - modifiedStartPoint + 1)
                ];
            }
            else {
                // Now walk backward through the reverse diagonals history
                changeHelper = new DiffChangeHelper();
                diagonalMin = diagonalReverseStart;
                diagonalMax = diagonalReverseEnd;
                diagonalRelative = (midOriginalArr[0] - midModifiedArr[0]) - diagonalReverseOffset;
                lastOriginalIndex = 1073741824 /* MAX_SAFE_SMALL_INTEGER */;
                historyIndex = (deltaIsEven) ? this.m_reverseHistory.length - 1 : this.m_reverseHistory.length - 2;
                do {
                    // Get the diagonal index from the relative diagonal number
                    const diagonal = diagonalRelative + diagonalReverseBase;
                    // Figure out where we came from
                    if (diagonal === diagonalMin || (diagonal < diagonalMax && reversePoints[diagonal - 1] >= reversePoints[diagonal + 1])) {
                        // Horizontal line (the element is a deletion))
                        originalIndex = reversePoints[diagonal + 1] - 1;
                        modifiedIndex = originalIndex - diagonalRelative - diagonalReverseOffset;
                        if (originalIndex > lastOriginalIndex) {
                            changeHelper.MarkNextChange();
                        }
                        lastOriginalIndex = originalIndex + 1;
                        changeHelper.AddOriginalElement(originalIndex + 1, modifiedIndex + 1);
                        diagonalRelative = (diagonal + 1) - diagonalReverseBase; //Setup for the next iteration
                    }
                    else {
                        // Vertical line (the element is an insertion)
                        originalIndex = reversePoints[diagonal - 1];
                        modifiedIndex = originalIndex - diagonalRelative - diagonalReverseOffset;
                        if (originalIndex > lastOriginalIndex) {
                            changeHelper.MarkNextChange();
                        }
                        lastOriginalIndex = originalIndex;
                        changeHelper.AddModifiedElement(originalIndex + 1, modifiedIndex + 1);
                        diagonalRelative = (diagonal - 1) - diagonalReverseBase; //Setup for the next iteration
                    }
                    if (historyIndex >= 0) {
                        reversePoints = this.m_reverseHistory[historyIndex];
                        diagonalReverseBase = reversePoints[0]; //We stored this in the first spot
                        diagonalMin = 1;
                        diagonalMax = reversePoints.length - 1;
                    }
                } while (--historyIndex >= -1);
                // There are cases where the reverse history will find diffs that
                // are correct, but not intuitive, so we need shift them.
                reverseChanges = changeHelper.getChanges();
            }
            return this.ConcatenateChanges(forwardChanges, reverseChanges);
        }
        /**
         * Given the range to compute the diff on, this method finds the point:
         * (midOriginal, midModified)
         * that exists in the middle of the LCS of the two sequences and
         * is the point at which the LCS problem may be broken down recursively.
         * This method will try to keep the LCS trace in memory. If the LCS recursion
         * point is calculated and the full trace is available in memory, then this method
         * will return the change list.
         * @param originalStart The start bound of the original sequence range
         * @param originalEnd The end bound of the original sequence range
         * @param modifiedStart The start bound of the modified sequence range
         * @param modifiedEnd The end bound of the modified sequence range
         * @param midOriginal The middle point of the original sequence range
         * @param midModified The middle point of the modified sequence range
         * @returns The diff changes, if available, otherwise null
         */
        ComputeRecursionPoint(originalStart, originalEnd, modifiedStart, modifiedEnd, midOriginalArr, midModifiedArr, quitEarlyArr) {
            let originalIndex = 0, modifiedIndex = 0;
            let diagonalForwardStart = 0, diagonalForwardEnd = 0;
            let diagonalReverseStart = 0, diagonalReverseEnd = 0;
            // To traverse the edit graph and produce the proper LCS, our actual
            // start position is just outside the given boundary
            originalStart--;
            modifiedStart--;
            // We set these up to make the compiler happy, but they will
            // be replaced before we return with the actual recursion point
            midOriginalArr[0] = 0;
            midModifiedArr[0] = 0;
            // Clear out the history
            this.m_forwardHistory = [];
            this.m_reverseHistory = [];
            // Each cell in the two arrays corresponds to a diagonal in the edit graph.
            // The integer value in the cell represents the originalIndex of the furthest
            // reaching point found so far that ends in that diagonal.
            // The modifiedIndex can be computed mathematically from the originalIndex and the diagonal number.
            const maxDifferences = (originalEnd - originalStart) + (modifiedEnd - modifiedStart);
            const numDiagonals = maxDifferences + 1;
            const forwardPoints = new Int32Array(numDiagonals);
            const reversePoints = new Int32Array(numDiagonals);
            // diagonalForwardBase: Index into forwardPoints of the diagonal which passes through (originalStart, modifiedStart)
            // diagonalReverseBase: Index into reversePoints of the diagonal which passes through (originalEnd, modifiedEnd)
            const diagonalForwardBase = (modifiedEnd - modifiedStart);
            const diagonalReverseBase = (originalEnd - originalStart);
            // diagonalForwardOffset: Geometric offset which allows modifiedIndex to be computed from originalIndex and the
            //    diagonal number (relative to diagonalForwardBase)
            // diagonalReverseOffset: Geometric offset which allows modifiedIndex to be computed from originalIndex and the
            //    diagonal number (relative to diagonalReverseBase)
            const diagonalForwardOffset = (originalStart - modifiedStart);
            const diagonalReverseOffset = (originalEnd - modifiedEnd);
            // delta: The difference between the end diagonal and the start diagonal. This is used to relate diagonal numbers
            //   relative to the start diagonal with diagonal numbers relative to the end diagonal.
            // The Even/Oddn-ness of this delta is important for determining when we should check for overlap
            const delta = diagonalReverseBase - diagonalForwardBase;
            const deltaIsEven = (delta % 2 === 0);
            // Here we set up the start and end points as the furthest points found so far
            // in both the forward and reverse directions, respectively
            forwardPoints[diagonalForwardBase] = originalStart;
            reversePoints[diagonalReverseBase] = originalEnd;
            // Remember if we quit early, and thus need to do a best-effort result instead of a real result.
            quitEarlyArr[0] = false;
            // A couple of points:
            // --With this method, we iterate on the number of differences between the two sequences.
            //   The more differences there actually are, the longer this will take.
            // --Also, as the number of differences increases, we have to search on diagonals further
            //   away from the reference diagonal (which is diagonalForwardBase for forward, diagonalReverseBase for reverse).
            // --We extend on even diagonals (relative to the reference diagonal) only when numDifferences
            //   is even and odd diagonals only when numDifferences is odd.
            for (let numDifferences = 1; numDifferences <= (maxDifferences / 2) + 1; numDifferences++) {
                let furthestOriginalIndex = 0;
                let furthestModifiedIndex = 0;
                // Run the algorithm in the forward direction
                diagonalForwardStart = this.ClipDiagonalBound(diagonalForwardBase - numDifferences, numDifferences, diagonalForwardBase, numDiagonals);
                diagonalForwardEnd = this.ClipDiagonalBound(diagonalForwardBase + numDifferences, numDifferences, diagonalForwardBase, numDiagonals);
                for (let diagonal = diagonalForwardStart; diagonal <= diagonalForwardEnd; diagonal += 2) {
                    // STEP 1: We extend the furthest reaching point in the present diagonal
                    // by looking at the diagonals above and below and picking the one whose point
                    // is further away from the start point (originalStart, modifiedStart)
                    if (diagonal === diagonalForwardStart || (diagonal < diagonalForwardEnd && forwardPoints[diagonal - 1] < forwardPoints[diagonal + 1])) {
                        originalIndex = forwardPoints[diagonal + 1];
                    }
                    else {
                        originalIndex = forwardPoints[diagonal - 1] + 1;
                    }
                    modifiedIndex = originalIndex - (diagonal - diagonalForwardBase) - diagonalForwardOffset;
                    // Save the current originalIndex so we can test for false overlap in step 3
                    const tempOriginalIndex = originalIndex;
                    // STEP 2: We can continue to extend the furthest reaching point in the present diagonal
                    // so long as the elements are equal.
                    while (originalIndex < originalEnd && modifiedIndex < modifiedEnd && this.ElementsAreEqual(originalIndex + 1, modifiedIndex + 1)) {
                        originalIndex++;
                        modifiedIndex++;
                    }
                    forwardPoints[diagonal] = originalIndex;
                    if (originalIndex + modifiedIndex > furthestOriginalIndex + furthestModifiedIndex) {
                        furthestOriginalIndex = originalIndex;
                        furthestModifiedIndex = modifiedIndex;
                    }
                    // STEP 3: If delta is odd (overlap first happens on forward when delta is odd)
                    // and diagonal is in the range of reverse diagonals computed for numDifferences-1
                    // (the previous iteration; we haven't computed reverse diagonals for numDifferences yet)
                    // then check for overlap.
                    if (!deltaIsEven && Math.abs(diagonal - diagonalReverseBase) <= (numDifferences - 1)) {
                        if (originalIndex >= reversePoints[diagonal]) {
                            midOriginalArr[0] = originalIndex;
                            midModifiedArr[0] = modifiedIndex;
                            if (tempOriginalIndex <= reversePoints[diagonal] && 1447 /* MaxDifferencesHistory */ > 0 && numDifferences <= (1447 /* MaxDifferencesHistory */ + 1)) {
                                // BINGO! We overlapped, and we have the full trace in memory!
                                return this.WALKTRACE(diagonalForwardBase, diagonalForwardStart, diagonalForwardEnd, diagonalForwardOffset, diagonalReverseBase, diagonalReverseStart, diagonalReverseEnd, diagonalReverseOffset, forwardPoints, reversePoints, originalIndex, originalEnd, midOriginalArr, modifiedIndex, modifiedEnd, midModifiedArr, deltaIsEven, quitEarlyArr);
                            }
                            else {
                                // Either false overlap, or we didn't have enough memory for the full trace
                                // Just return the recursion point
                                return null;
                            }
                        }
                    }
                }
                // Check to see if we should be quitting early, before moving on to the next iteration.
                const matchLengthOfLongest = ((furthestOriginalIndex - originalStart) + (furthestModifiedIndex - modifiedStart) - numDifferences) / 2;
                if (this.ContinueProcessingPredicate !== null && !this.ContinueProcessingPredicate(furthestOriginalIndex, matchLengthOfLongest)) {
                    // We can't finish, so skip ahead to generating a result from what we have.
                    quitEarlyArr[0] = true;
                    // Use the furthest distance we got in the forward direction.
                    midOriginalArr[0] = furthestOriginalIndex;
                    midModifiedArr[0] = furthestModifiedIndex;
                    if (matchLengthOfLongest > 0 && 1447 /* MaxDifferencesHistory */ > 0 && numDifferences <= (1447 /* MaxDifferencesHistory */ + 1)) {
                        // Enough of the history is in memory to walk it backwards
                        return this.WALKTRACE(diagonalForwardBase, diagonalForwardStart, diagonalForwardEnd, diagonalForwardOffset, diagonalReverseBase, diagonalReverseStart, diagonalReverseEnd, diagonalReverseOffset, forwardPoints, reversePoints, originalIndex, originalEnd, midOriginalArr, modifiedIndex, modifiedEnd, midModifiedArr, deltaIsEven, quitEarlyArr);
                    }
                    else {
                        // We didn't actually remember enough of the history.
                        //Since we are quitting the diff early, we need to shift back the originalStart and modified start
                        //back into the boundary limits since we decremented their value above beyond the boundary limit.
                        originalStart++;
                        modifiedStart++;
                        return [
                            new DiffChange(originalStart, originalEnd - originalStart + 1, modifiedStart, modifiedEnd - modifiedStart + 1)
                        ];
                    }
                }
                // Run the algorithm in the reverse direction
                diagonalReverseStart = this.ClipDiagonalBound(diagonalReverseBase - numDifferences, numDifferences, diagonalReverseBase, numDiagonals);
                diagonalReverseEnd = this.ClipDiagonalBound(diagonalReverseBase + numDifferences, numDifferences, diagonalReverseBase, numDiagonals);
                for (let diagonal = diagonalReverseStart; diagonal <= diagonalReverseEnd; diagonal += 2) {
                    // STEP 1: We extend the furthest reaching point in the present diagonal
                    // by looking at the diagonals above and below and picking the one whose point
                    // is further away from the start point (originalEnd, modifiedEnd)
                    if (diagonal === diagonalReverseStart || (diagonal < diagonalReverseEnd && reversePoints[diagonal - 1] >= reversePoints[diagonal + 1])) {
                        originalIndex = reversePoints[diagonal + 1] - 1;
                    }
                    else {
                        originalIndex = reversePoints[diagonal - 1];
                    }
                    modifiedIndex = originalIndex - (diagonal - diagonalReverseBase) - diagonalReverseOffset;
                    // Save the current originalIndex so we can test for false overlap
                    const tempOriginalIndex = originalIndex;
                    // STEP 2: We can continue to extend the furthest reaching point in the present diagonal
                    // as long as the elements are equal.
                    while (originalIndex > originalStart && modifiedIndex > modifiedStart && this.ElementsAreEqual(originalIndex, modifiedIndex)) {
                        originalIndex--;
                        modifiedIndex--;
                    }
                    reversePoints[diagonal] = originalIndex;
                    // STEP 4: If delta is even (overlap first happens on reverse when delta is even)
                    // and diagonal is in the range of forward diagonals computed for numDifferences
                    // then check for overlap.
                    if (deltaIsEven && Math.abs(diagonal - diagonalForwardBase) <= numDifferences) {
                        if (originalIndex <= forwardPoints[diagonal]) {
                            midOriginalArr[0] = originalIndex;
                            midModifiedArr[0] = modifiedIndex;
                            if (tempOriginalIndex >= forwardPoints[diagonal] && 1447 /* MaxDifferencesHistory */ > 0 && numDifferences <= (1447 /* MaxDifferencesHistory */ + 1)) {
                                // BINGO! We overlapped, and we have the full trace in memory!
                                return this.WALKTRACE(diagonalForwardBase, diagonalForwardStart, diagonalForwardEnd, diagonalForwardOffset, diagonalReverseBase, diagonalReverseStart, diagonalReverseEnd, diagonalReverseOffset, forwardPoints, reversePoints, originalIndex, originalEnd, midOriginalArr, modifiedIndex, modifiedEnd, midModifiedArr, deltaIsEven, quitEarlyArr);
                            }
                            else {
                                // Either false overlap, or we didn't have enough memory for the full trace
                                // Just return the recursion point
                                return null;
                            }
                        }
                    }
                }
                // Save current vectors to history before the next iteration
                if (numDifferences <= 1447 /* MaxDifferencesHistory */) {
                    // We are allocating space for one extra int, which we fill with
                    // the index of the diagonal base index
                    let temp = new Int32Array(diagonalForwardEnd - diagonalForwardStart + 2);
                    temp[0] = diagonalForwardBase - diagonalForwardStart + 1;
                    MyArray.Copy2(forwardPoints, diagonalForwardStart, temp, 1, diagonalForwardEnd - diagonalForwardStart + 1);
                    this.m_forwardHistory.push(temp);
                    temp = new Int32Array(diagonalReverseEnd - diagonalReverseStart + 2);
                    temp[0] = diagonalReverseBase - diagonalReverseStart + 1;
                    MyArray.Copy2(reversePoints, diagonalReverseStart, temp, 1, diagonalReverseEnd - diagonalReverseStart + 1);
                    this.m_reverseHistory.push(temp);
                }
            }
            // If we got here, then we have the full trace in history. We just have to convert it to a change list
            // NOTE: This part is a bit messy
            return this.WALKTRACE(diagonalForwardBase, diagonalForwardStart, diagonalForwardEnd, diagonalForwardOffset, diagonalReverseBase, diagonalReverseStart, diagonalReverseEnd, diagonalReverseOffset, forwardPoints, reversePoints, originalIndex, originalEnd, midOriginalArr, modifiedIndex, modifiedEnd, midModifiedArr, deltaIsEven, quitEarlyArr);
        }
        /**
         * Shifts the given changes to provide a more intuitive diff.
         * While the first element in a diff matches the first element after the diff,
         * we shift the diff down.
         *
         * @param changes The list of changes to shift
         * @returns The shifted changes
         */
        PrettifyChanges(changes) {
            // Shift all the changes down first
            for (let i = 0; i < changes.length; i++) {
                const change = changes[i];
                const originalStop = (i < changes.length - 1) ? changes[i + 1].originalStart : this._originalElementsOrHash.length;
                const modifiedStop = (i < changes.length - 1) ? changes[i + 1].modifiedStart : this._modifiedElementsOrHash.length;
                const checkOriginal = change.originalLength > 0;
                const checkModified = change.modifiedLength > 0;
                while (change.originalStart + change.originalLength < originalStop
                    && change.modifiedStart + change.modifiedLength < modifiedStop
                    && (!checkOriginal || this.OriginalElementsAreEqual(change.originalStart, change.originalStart + change.originalLength))
                    && (!checkModified || this.ModifiedElementsAreEqual(change.modifiedStart, change.modifiedStart + change.modifiedLength))) {
                    const startStrictEqual = this.ElementsAreStrictEqual(change.originalStart, change.modifiedStart);
                    const endStrictEqual = this.ElementsAreStrictEqual(change.originalStart + change.originalLength, change.modifiedStart + change.modifiedLength);
                    if (endStrictEqual && !startStrictEqual) {
                        // moving the change down would create an equal change, but the elements are not strict equal
                        break;
                    }
                    change.originalStart++;
                    change.modifiedStart++;
                }
                let mergedChangeArr = [null];
                if (i < changes.length - 1 && this.ChangesOverlap(changes[i], changes[i + 1], mergedChangeArr)) {
                    changes[i] = mergedChangeArr[0];
                    changes.splice(i + 1, 1);
                    i--;
                    continue;
                }
            }
            // Shift changes back up until we hit empty or whitespace-only lines
            for (let i = changes.length - 1; i >= 0; i--) {
                const change = changes[i];
                let originalStop = 0;
                let modifiedStop = 0;
                if (i > 0) {
                    const prevChange = changes[i - 1];
                    originalStop = prevChange.originalStart + prevChange.originalLength;
                    modifiedStop = prevChange.modifiedStart + prevChange.modifiedLength;
                }
                const checkOriginal = change.originalLength > 0;
                const checkModified = change.modifiedLength > 0;
                let bestDelta = 0;
                let bestScore = this._boundaryScore(change.originalStart, change.originalLength, change.modifiedStart, change.modifiedLength);
                for (let delta = 1;; delta++) {
                    const originalStart = change.originalStart - delta;
                    const modifiedStart = change.modifiedStart - delta;
                    if (originalStart < originalStop || modifiedStart < modifiedStop) {
                        break;
                    }
                    if (checkOriginal && !this.OriginalElementsAreEqual(originalStart, originalStart + change.originalLength)) {
                        break;
                    }
                    if (checkModified && !this.ModifiedElementsAreEqual(modifiedStart, modifiedStart + change.modifiedLength)) {
                        break;
                    }
                    const touchingPreviousChange = (originalStart === originalStop && modifiedStart === modifiedStop);
                    const score = ((touchingPreviousChange ? 5 : 0)
                        + this._boundaryScore(originalStart, change.originalLength, modifiedStart, change.modifiedLength));
                    if (score > bestScore) {
                        bestScore = score;
                        bestDelta = delta;
                    }
                }
                change.originalStart -= bestDelta;
                change.modifiedStart -= bestDelta;
                const mergedChangeArr = [null];
                if (i > 0 && this.ChangesOverlap(changes[i - 1], changes[i], mergedChangeArr)) {
                    changes[i - 1] = mergedChangeArr[0];
                    changes.splice(i, 1);
                    i++;
                    continue;
                }
            }
            // There could be multiple longest common substrings.
            // Give preference to the ones containing longer lines
            if (this._hasStrings) {
                for (let i = 1, len = changes.length; i < len; i++) {
                    const aChange = changes[i - 1];
                    const bChange = changes[i];
                    const matchedLength = bChange.originalStart - aChange.originalStart - aChange.originalLength;
                    const aOriginalStart = aChange.originalStart;
                    const bOriginalEnd = bChange.originalStart + bChange.originalLength;
                    const abOriginalLength = bOriginalEnd - aOriginalStart;
                    const aModifiedStart = aChange.modifiedStart;
                    const bModifiedEnd = bChange.modifiedStart + bChange.modifiedLength;
                    const abModifiedLength = bModifiedEnd - aModifiedStart;
                    // Avoid wasting a lot of time with these searches
                    if (matchedLength < 5 && abOriginalLength < 20 && abModifiedLength < 20) {
                        const t = this._findBetterContiguousSequence(aOriginalStart, abOriginalLength, aModifiedStart, abModifiedLength, matchedLength);
                        if (t) {
                            const [originalMatchStart, modifiedMatchStart] = t;
                            if (originalMatchStart !== aChange.originalStart + aChange.originalLength || modifiedMatchStart !== aChange.modifiedStart + aChange.modifiedLength) {
                                // switch to another sequence that has a better score
                                aChange.originalLength = originalMatchStart - aChange.originalStart;
                                aChange.modifiedLength = modifiedMatchStart - aChange.modifiedStart;
                                bChange.originalStart = originalMatchStart + matchedLength;
                                bChange.modifiedStart = modifiedMatchStart + matchedLength;
                                bChange.originalLength = bOriginalEnd - bChange.originalStart;
                                bChange.modifiedLength = bModifiedEnd - bChange.modifiedStart;
                            }
                        }
                    }
                }
            }
            return changes;
        }
        _findBetterContiguousSequence(originalStart, originalLength, modifiedStart, modifiedLength, desiredLength) {
            if (originalLength < desiredLength || modifiedLength < desiredLength) {
                return null;
            }
            const originalMax = originalStart + originalLength - desiredLength + 1;
            const modifiedMax = modifiedStart + modifiedLength - desiredLength + 1;
            let bestScore = 0;
            let bestOriginalStart = 0;
            let bestModifiedStart = 0;
            for (let i = originalStart; i < originalMax; i++) {
                for (let j = modifiedStart; j < modifiedMax; j++) {
                    const score = this._contiguousSequenceScore(i, j, desiredLength);
                    if (score > 0 && score > bestScore) {
                        bestScore = score;
                        bestOriginalStart = i;
                        bestModifiedStart = j;
                    }
                }
            }
            if (bestScore > 0) {
                return [bestOriginalStart, bestModifiedStart];
            }
            return null;
        }
        _contiguousSequenceScore(originalStart, modifiedStart, length) {
            let score = 0;
            for (let l = 0; l < length; l++) {
                if (!this.ElementsAreEqual(originalStart + l, modifiedStart + l)) {
                    return 0;
                }
                score += this._originalStringElements[originalStart + l].length;
            }
            return score;
        }
        _OriginalIsBoundary(index) {
            if (index <= 0 || index >= this._originalElementsOrHash.length - 1) {
                return true;
            }
            return (this._hasStrings && /^\s*$/.test(this._originalStringElements[index]));
        }
        _OriginalRegionIsBoundary(originalStart, originalLength) {
            if (this._OriginalIsBoundary(originalStart) || this._OriginalIsBoundary(originalStart - 1)) {
                return true;
            }
            if (originalLength > 0) {
                const originalEnd = originalStart + originalLength;
                if (this._OriginalIsBoundary(originalEnd - 1) || this._OriginalIsBoundary(originalEnd)) {
                    return true;
                }
            }
            return false;
        }
        _ModifiedIsBoundary(index) {
            if (index <= 0 || index >= this._modifiedElementsOrHash.length - 1) {
                return true;
            }
            return (this._hasStrings && /^\s*$/.test(this._modifiedStringElements[index]));
        }
        _ModifiedRegionIsBoundary(modifiedStart, modifiedLength) {
            if (this._ModifiedIsBoundary(modifiedStart) || this._ModifiedIsBoundary(modifiedStart - 1)) {
                return true;
            }
            if (modifiedLength > 0) {
                const modifiedEnd = modifiedStart + modifiedLength;
                if (this._ModifiedIsBoundary(modifiedEnd - 1) || this._ModifiedIsBoundary(modifiedEnd)) {
                    return true;
                }
            }
            return false;
        }
        _boundaryScore(originalStart, originalLength, modifiedStart, modifiedLength) {
            const originalScore = (this._OriginalRegionIsBoundary(originalStart, originalLength) ? 1 : 0);
            const modifiedScore = (this._ModifiedRegionIsBoundary(modifiedStart, modifiedLength) ? 1 : 0);
            return (originalScore + modifiedScore);
        }
        /**
         * Concatenates the two input DiffChange lists and returns the resulting
         * list.
         * @param The left changes
         * @param The right changes
         * @returns The concatenated list
         */
        ConcatenateChanges(left, right) {
            let mergedChangeArr = [];
            if (left.length === 0 || right.length === 0) {
                return (right.length > 0) ? right : left;
            }
            else if (this.ChangesOverlap(left[left.length - 1], right[0], mergedChangeArr)) {
                // Since we break the problem down recursively, it is possible that we
                // might recurse in the middle of a change thereby splitting it into
                // two changes. Here in the combining stage, we detect and fuse those
                // changes back together
                const result = new Array(left.length + right.length - 1);
                MyArray.Copy(left, 0, result, 0, left.length - 1);
                result[left.length - 1] = mergedChangeArr[0];
                MyArray.Copy(right, 1, result, left.length, right.length - 1);
                return result;
            }
            else {
                const result = new Array(left.length + right.length);
                MyArray.Copy(left, 0, result, 0, left.length);
                MyArray.Copy(right, 0, result, left.length, right.length);
                return result;
            }
        }
        /**
         * Returns true if the two changes overlap and can be merged into a single
         * change
         * @param left The left change
         * @param right The right change
         * @param mergedChange The merged change if the two overlap, null otherwise
         * @returns True if the two changes overlap
         */
        ChangesOverlap(left, right, mergedChangeArr) {
            Debug.Assert(left.originalStart <= right.originalStart, 'Left change is not less than or equal to right change');
            Debug.Assert(left.modifiedStart <= right.modifiedStart, 'Left change is not less than or equal to right change');
            if (left.originalStart + left.originalLength >= right.originalStart || left.modifiedStart + left.modifiedLength >= right.modifiedStart) {
                const originalStart = left.originalStart;
                let originalLength = left.originalLength;
                const modifiedStart = left.modifiedStart;
                let modifiedLength = left.modifiedLength;
                if (left.originalStart + left.originalLength >= right.originalStart) {
                    originalLength = right.originalStart + right.originalLength - left.originalStart;
                }
                if (left.modifiedStart + left.modifiedLength >= right.modifiedStart) {
                    modifiedLength = right.modifiedStart + right.modifiedLength - left.modifiedStart;
                }
                mergedChangeArr[0] = new DiffChange(originalStart, originalLength, modifiedStart, modifiedLength);
                return true;
            }
            else {
                mergedChangeArr[0] = null;
                return false;
            }
        }
        /**
         * Helper method used to clip a diagonal index to the range of valid
         * diagonals. This also decides whether or not the diagonal index,
         * if it exceeds the boundary, should be clipped to the boundary or clipped
         * one inside the boundary depending on the Even/Odd status of the boundary
         * and numDifferences.
         * @param diagonal The index of the diagonal to clip.
         * @param numDifferences The current number of differences being iterated upon.
         * @param diagonalBaseIndex The base reference diagonal.
         * @param numDiagonals The total number of diagonals.
         * @returns The clipped diagonal index.
         */
        ClipDiagonalBound(diagonal, numDifferences, diagonalBaseIndex, numDiagonals) {
            if (diagonal >= 0 && diagonal < numDiagonals) {
                // Nothing to clip, its in range
                return diagonal;
            }
            // diagonalsBelow: The number of diagonals below the reference diagonal
            // diagonalsAbove: The number of diagonals above the reference diagonal
            const diagonalsBelow = diagonalBaseIndex;
            const diagonalsAbove = numDiagonals - diagonalBaseIndex - 1;
            const diffEven = (numDifferences % 2 === 0);
            if (diagonal < 0) {
                const lowerBoundEven = (diagonalsBelow % 2 === 0);
                return (diffEven === lowerBoundEven) ? 0 : 1;
            }
            else {
                const upperBoundEven = (diagonalsAbove % 2 === 0);
                return (diffEven === upperBoundEven) ? numDiagonals - 1 : numDiagonals - 2;
            }
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    let safeProcess;
    // Native sandbox environment
    if (typeof globals.vscode !== 'undefined' && typeof globals.vscode.process !== 'undefined') {
        const sandboxProcess = globals.vscode.process;
        safeProcess = {
            get platform() { return sandboxProcess.platform; },
            get arch() { return sandboxProcess.arch; },
            get env() { return sandboxProcess.env; },
            cwd() { return sandboxProcess.cwd(); }
        };
    }
    // Native node.js environment
    else if (typeof process !== 'undefined') {
        safeProcess = {
            get platform() { return process.platform; },
            get arch() { return process.arch; },
            get env() { return process.env; },
            cwd() { return process.env['VSCODE_CWD'] || process.cwd(); }
        };
    }
    // Web environment
    else {
        safeProcess = {
            // Supported
            get platform() { return isWindows ? 'win32' : isMacintosh ? 'darwin' : 'linux'; },
            get arch() { return undefined; /* arch is undefined in web */ },
            // Unsupported
            get env() { return {}; },
            cwd() { return '/'; }
        };
    }
    /**
     * Provides safe access to the `cwd` property in node.js, sandboxed or web
     * environments.
     *
     * Note: in web, this property is hardcoded to be `/`.
     */
    const cwd = safeProcess.cwd;
    /**
     * Provides safe access to the `env` property in node.js, sandboxed or web
     * environments.
     *
     * Note: in web, this property is hardcoded to be `{}`.
     */
    const env = safeProcess.env;
    /**
     * Provides safe access to the `platform` property in node.js, sandboxed or web
     * environments.
     */
    const platform = safeProcess.platform;

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    const CHAR_UPPERCASE_A = 65; /* A */
    const CHAR_LOWERCASE_A = 97; /* a */
    const CHAR_UPPERCASE_Z = 90; /* Z */
    const CHAR_LOWERCASE_Z = 122; /* z */
    const CHAR_DOT = 46; /* . */
    const CHAR_FORWARD_SLASH = 47; /* / */
    const CHAR_BACKWARD_SLASH = 92; /* \ */
    const CHAR_COLON = 58; /* : */
    const CHAR_QUESTION_MARK = 63; /* ? */
    class ErrorInvalidArgType extends Error {
        constructor(name, expected, actual) {
            // determiner: 'must be' or 'must not be'
            let determiner;
            if (typeof expected === 'string' && expected.indexOf('not ') === 0) {
                determiner = 'must not be';
                expected = expected.replace(/^not /, '');
            }
            else {
                determiner = 'must be';
            }
            const type = name.indexOf('.') !== -1 ? 'property' : 'argument';
            let msg = `The "${name}" ${type} ${determiner} of type ${expected}`;
            msg += `. Received type ${typeof actual}`;
            super(msg);
            this.code = 'ERR_INVALID_ARG_TYPE';
        }
    }
    function validateString(value, name) {
        if (typeof value !== 'string') {
            throw new ErrorInvalidArgType(name, 'string', value);
        }
    }
    function isPathSeparator(code) {
        return code === CHAR_FORWARD_SLASH || code === CHAR_BACKWARD_SLASH;
    }
    function isPosixPathSeparator(code) {
        return code === CHAR_FORWARD_SLASH;
    }
    function isWindowsDeviceRoot(code) {
        return (code >= CHAR_UPPERCASE_A && code <= CHAR_UPPERCASE_Z) ||
            (code >= CHAR_LOWERCASE_A && code <= CHAR_LOWERCASE_Z);
    }
    // Resolves . and .. elements in a path with directory names
    function normalizeString(path, allowAboveRoot, separator, isPathSeparator) {
        let res = '';
        let lastSegmentLength = 0;
        let lastSlash = -1;
        let dots = 0;
        let code = 0;
        for (let i = 0; i <= path.length; ++i) {
            if (i < path.length) {
                code = path.charCodeAt(i);
            }
            else if (isPathSeparator(code)) {
                break;
            }
            else {
                code = CHAR_FORWARD_SLASH;
            }
            if (isPathSeparator(code)) {
                if (lastSlash === i - 1 || dots === 1) ;
                else if (dots === 2) {
                    if (res.length < 2 || lastSegmentLength !== 2 ||
                        res.charCodeAt(res.length - 1) !== CHAR_DOT ||
                        res.charCodeAt(res.length - 2) !== CHAR_DOT) {
                        if (res.length > 2) {
                            const lastSlashIndex = res.lastIndexOf(separator);
                            if (lastSlashIndex === -1) {
                                res = '';
                                lastSegmentLength = 0;
                            }
                            else {
                                res = res.slice(0, lastSlashIndex);
                                lastSegmentLength = res.length - 1 - res.lastIndexOf(separator);
                            }
                            lastSlash = i;
                            dots = 0;
                            continue;
                        }
                        else if (res.length !== 0) {
                            res = '';
                            lastSegmentLength = 0;
                            lastSlash = i;
                            dots = 0;
                            continue;
                        }
                    }
                    if (allowAboveRoot) {
                        res += res.length > 0 ? `${separator}..` : '..';
                        lastSegmentLength = 2;
                    }
                }
                else {
                    if (res.length > 0) {
                        res += `${separator}${path.slice(lastSlash + 1, i)}`;
                    }
                    else {
                        res = path.slice(lastSlash + 1, i);
                    }
                    lastSegmentLength = i - lastSlash - 1;
                }
                lastSlash = i;
                dots = 0;
            }
            else if (code === CHAR_DOT && dots !== -1) {
                ++dots;
            }
            else {
                dots = -1;
            }
        }
        return res;
    }
    function _format(sep, pathObject) {
        if (pathObject === null || typeof pathObject !== 'object') {
            throw new ErrorInvalidArgType('pathObject', 'Object', pathObject);
        }
        const dir = pathObject.dir || pathObject.root;
        const base = pathObject.base ||
            `${pathObject.name || ''}${pathObject.ext || ''}`;
        if (!dir) {
            return base;
        }
        return dir === pathObject.root ? `${dir}${base}` : `${dir}${sep}${base}`;
    }
    const win32 = {
        // path.resolve([from ...], to)
        resolve(...pathSegments) {
            let resolvedDevice = '';
            let resolvedTail = '';
            let resolvedAbsolute = false;
            for (let i = pathSegments.length - 1; i >= -1; i--) {
                let path;
                if (i >= 0) {
                    path = pathSegments[i];
                    validateString(path, 'path');
                    // Skip empty entries
                    if (path.length === 0) {
                        continue;
                    }
                }
                else if (resolvedDevice.length === 0) {
                    path = cwd();
                }
                else {
                    // Windows has the concept of drive-specific current working
                    // directories. If we've resolved a drive letter but not yet an
                    // absolute path, get cwd for that drive, or the process cwd if
                    // the drive cwd is not available. We're sure the device is not
                    // a UNC path at this points, because UNC paths are always absolute.
                    path = env[`=${resolvedDevice}`] || cwd();
                    // Verify that a cwd was found and that it actually points
                    // to our drive. If not, default to the drive's root.
                    if (path === undefined ||
                        (path.slice(0, 2).toLowerCase() !== resolvedDevice.toLowerCase() &&
                            path.charCodeAt(2) === CHAR_BACKWARD_SLASH)) {
                        path = `${resolvedDevice}\\`;
                    }
                }
                const len = path.length;
                let rootEnd = 0;
                let device = '';
                let isAbsolute = false;
                const code = path.charCodeAt(0);
                // Try to match a root
                if (len === 1) {
                    if (isPathSeparator(code)) {
                        // `path` contains just a path separator
                        rootEnd = 1;
                        isAbsolute = true;
                    }
                }
                else if (isPathSeparator(code)) {
                    // Possible UNC root
                    // If we started with a separator, we know we at least have an
                    // absolute path of some kind (UNC or otherwise)
                    isAbsolute = true;
                    if (isPathSeparator(path.charCodeAt(1))) {
                        // Matched double path separator at beginning
                        let j = 2;
                        let last = j;
                        // Match 1 or more non-path separators
                        while (j < len && !isPathSeparator(path.charCodeAt(j))) {
                            j++;
                        }
                        if (j < len && j !== last) {
                            const firstPart = path.slice(last, j);
                            // Matched!
                            last = j;
                            // Match 1 or more path separators
                            while (j < len && isPathSeparator(path.charCodeAt(j))) {
                                j++;
                            }
                            if (j < len && j !== last) {
                                // Matched!
                                last = j;
                                // Match 1 or more non-path separators
                                while (j < len && !isPathSeparator(path.charCodeAt(j))) {
                                    j++;
                                }
                                if (j === len || j !== last) {
                                    // We matched a UNC root
                                    device = `\\\\${firstPart}\\${path.slice(last, j)}`;
                                    rootEnd = j;
                                }
                            }
                        }
                    }
                    else {
                        rootEnd = 1;
                    }
                }
                else if (isWindowsDeviceRoot(code) &&
                    path.charCodeAt(1) === CHAR_COLON) {
                    // Possible device root
                    device = path.slice(0, 2);
                    rootEnd = 2;
                    if (len > 2 && isPathSeparator(path.charCodeAt(2))) {
                        // Treat separator following drive name as an absolute path
                        // indicator
                        isAbsolute = true;
                        rootEnd = 3;
                    }
                }
                if (device.length > 0) {
                    if (resolvedDevice.length > 0) {
                        if (device.toLowerCase() !== resolvedDevice.toLowerCase()) {
                            // This path points to another device so it is not applicable
                            continue;
                        }
                    }
                    else {
                        resolvedDevice = device;
                    }
                }
                if (resolvedAbsolute) {
                    if (resolvedDevice.length > 0) {
                        break;
                    }
                }
                else {
                    resolvedTail = `${path.slice(rootEnd)}\\${resolvedTail}`;
                    resolvedAbsolute = isAbsolute;
                    if (isAbsolute && resolvedDevice.length > 0) {
                        break;
                    }
                }
            }
            // At this point the path should be resolved to a full absolute path,
            // but handle relative paths to be safe (might happen when process.cwd()
            // fails)
            // Normalize the tail path
            resolvedTail = normalizeString(resolvedTail, !resolvedAbsolute, '\\', isPathSeparator);
            return resolvedAbsolute ?
                `${resolvedDevice}\\${resolvedTail}` :
                `${resolvedDevice}${resolvedTail}` || '.';
        },
        normalize(path) {
            validateString(path, 'path');
            const len = path.length;
            if (len === 0) {
                return '.';
            }
            let rootEnd = 0;
            let device;
            let isAbsolute = false;
            const code = path.charCodeAt(0);
            // Try to match a root
            if (len === 1) {
                // `path` contains just a single char, exit early to avoid
                // unnecessary work
                return isPosixPathSeparator(code) ? '\\' : path;
            }
            if (isPathSeparator(code)) {
                // Possible UNC root
                // If we started with a separator, we know we at least have an absolute
                // path of some kind (UNC or otherwise)
                isAbsolute = true;
                if (isPathSeparator(path.charCodeAt(1))) {
                    // Matched double path separator at beginning
                    let j = 2;
                    let last = j;
                    // Match 1 or more non-path separators
                    while (j < len && !isPathSeparator(path.charCodeAt(j))) {
                        j++;
                    }
                    if (j < len && j !== last) {
                        const firstPart = path.slice(last, j);
                        // Matched!
                        last = j;
                        // Match 1 or more path separators
                        while (j < len && isPathSeparator(path.charCodeAt(j))) {
                            j++;
                        }
                        if (j < len && j !== last) {
                            // Matched!
                            last = j;
                            // Match 1 or more non-path separators
                            while (j < len && !isPathSeparator(path.charCodeAt(j))) {
                                j++;
                            }
                            if (j === len) {
                                // We matched a UNC root only
                                // Return the normalized version of the UNC root since there
                                // is nothing left to process
                                return `\\\\${firstPart}\\${path.slice(last)}\\`;
                            }
                            if (j !== last) {
                                // We matched a UNC root with leftovers
                                device = `\\\\${firstPart}\\${path.slice(last, j)}`;
                                rootEnd = j;
                            }
                        }
                    }
                }
                else {
                    rootEnd = 1;
                }
            }
            else if (isWindowsDeviceRoot(code) && path.charCodeAt(1) === CHAR_COLON) {
                // Possible device root
                device = path.slice(0, 2);
                rootEnd = 2;
                if (len > 2 && isPathSeparator(path.charCodeAt(2))) {
                    // Treat separator following drive name as an absolute path
                    // indicator
                    isAbsolute = true;
                    rootEnd = 3;
                }
            }
            let tail = rootEnd < len ?
                normalizeString(path.slice(rootEnd), !isAbsolute, '\\', isPathSeparator) :
                '';
            if (tail.length === 0 && !isAbsolute) {
                tail = '.';
            }
            if (tail.length > 0 && isPathSeparator(path.charCodeAt(len - 1))) {
                tail += '\\';
            }
            if (device === undefined) {
                return isAbsolute ? `\\${tail}` : tail;
            }
            return isAbsolute ? `${device}\\${tail}` : `${device}${tail}`;
        },
        isAbsolute(path) {
            validateString(path, 'path');
            const len = path.length;
            if (len === 0) {
                return false;
            }
            const code = path.charCodeAt(0);
            return isPathSeparator(code) ||
                // Possible device root
                (len > 2 &&
                    isWindowsDeviceRoot(code) &&
                    path.charCodeAt(1) === CHAR_COLON &&
                    isPathSeparator(path.charCodeAt(2)));
        },
        join(...paths) {
            if (paths.length === 0) {
                return '.';
            }
            let joined;
            let firstPart;
            for (let i = 0; i < paths.length; ++i) {
                const arg = paths[i];
                validateString(arg, 'path');
                if (arg.length > 0) {
                    if (joined === undefined) {
                        joined = firstPart = arg;
                    }
                    else {
                        joined += `\\${arg}`;
                    }
                }
            }
            if (joined === undefined) {
                return '.';
            }
            // Make sure that the joined path doesn't start with two slashes, because
            // normalize() will mistake it for a UNC path then.
            //
            // This step is skipped when it is very clear that the user actually
            // intended to point at a UNC path. This is assumed when the first
            // non-empty string arguments starts with exactly two slashes followed by
            // at least one more non-slash character.
            //
            // Note that for normalize() to treat a path as a UNC path it needs to
            // have at least 2 components, so we don't filter for that here.
            // This means that the user can use join to construct UNC paths from
            // a server name and a share name; for example:
            //   path.join('//server', 'share') -> '\\\\server\\share\\')
            let needsReplace = true;
            let slashCount = 0;
            if (typeof firstPart === 'string' && isPathSeparator(firstPart.charCodeAt(0))) {
                ++slashCount;
                const firstLen = firstPart.length;
                if (firstLen > 1 && isPathSeparator(firstPart.charCodeAt(1))) {
                    ++slashCount;
                    if (firstLen > 2) {
                        if (isPathSeparator(firstPart.charCodeAt(2))) {
                            ++slashCount;
                        }
                        else {
                            // We matched a UNC path in the first part
                            needsReplace = false;
                        }
                    }
                }
            }
            if (needsReplace) {
                // Find any more consecutive slashes we need to replace
                while (slashCount < joined.length &&
                    isPathSeparator(joined.charCodeAt(slashCount))) {
                    slashCount++;
                }
                // Replace the slashes if needed
                if (slashCount >= 2) {
                    joined = `\\${joined.slice(slashCount)}`;
                }
            }
            return win32.normalize(joined);
        },
        // It will solve the relative path from `from` to `to`, for instance:
        //  from = 'C:\\orandea\\test\\aaa'
        //  to = 'C:\\orandea\\impl\\bbb'
        // The output of the function should be: '..\\..\\impl\\bbb'
        relative(from, to) {
            validateString(from, 'from');
            validateString(to, 'to');
            if (from === to) {
                return '';
            }
            const fromOrig = win32.resolve(from);
            const toOrig = win32.resolve(to);
            if (fromOrig === toOrig) {
                return '';
            }
            from = fromOrig.toLowerCase();
            to = toOrig.toLowerCase();
            if (from === to) {
                return '';
            }
            // Trim any leading backslashes
            let fromStart = 0;
            while (fromStart < from.length &&
                from.charCodeAt(fromStart) === CHAR_BACKWARD_SLASH) {
                fromStart++;
            }
            // Trim trailing backslashes (applicable to UNC paths only)
            let fromEnd = from.length;
            while (fromEnd - 1 > fromStart &&
                from.charCodeAt(fromEnd - 1) === CHAR_BACKWARD_SLASH) {
                fromEnd--;
            }
            const fromLen = fromEnd - fromStart;
            // Trim any leading backslashes
            let toStart = 0;
            while (toStart < to.length &&
                to.charCodeAt(toStart) === CHAR_BACKWARD_SLASH) {
                toStart++;
            }
            // Trim trailing backslashes (applicable to UNC paths only)
            let toEnd = to.length;
            while (toEnd - 1 > toStart &&
                to.charCodeAt(toEnd - 1) === CHAR_BACKWARD_SLASH) {
                toEnd--;
            }
            const toLen = toEnd - toStart;
            // Compare paths to find the longest common path from root
            const length = fromLen < toLen ? fromLen : toLen;
            let lastCommonSep = -1;
            let i = 0;
            for (; i < length; i++) {
                const fromCode = from.charCodeAt(fromStart + i);
                if (fromCode !== to.charCodeAt(toStart + i)) {
                    break;
                }
                else if (fromCode === CHAR_BACKWARD_SLASH) {
                    lastCommonSep = i;
                }
            }
            // We found a mismatch before the first common path separator was seen, so
            // return the original `to`.
            if (i !== length) {
                if (lastCommonSep === -1) {
                    return toOrig;
                }
            }
            else {
                if (toLen > length) {
                    if (to.charCodeAt(toStart + i) === CHAR_BACKWARD_SLASH) {
                        // We get here if `from` is the exact base path for `to`.
                        // For example: from='C:\\foo\\bar'; to='C:\\foo\\bar\\baz'
                        return toOrig.slice(toStart + i + 1);
                    }
                    if (i === 2) {
                        // We get here if `from` is the device root.
                        // For example: from='C:\\'; to='C:\\foo'
                        return toOrig.slice(toStart + i);
                    }
                }
                if (fromLen > length) {
                    if (from.charCodeAt(fromStart + i) === CHAR_BACKWARD_SLASH) {
                        // We get here if `to` is the exact base path for `from`.
                        // For example: from='C:\\foo\\bar'; to='C:\\foo'
                        lastCommonSep = i;
                    }
                    else if (i === 2) {
                        // We get here if `to` is the device root.
                        // For example: from='C:\\foo\\bar'; to='C:\\'
                        lastCommonSep = 3;
                    }
                }
                if (lastCommonSep === -1) {
                    lastCommonSep = 0;
                }
            }
            let out = '';
            // Generate the relative path based on the path difference between `to` and
            // `from`
            for (i = fromStart + lastCommonSep + 1; i <= fromEnd; ++i) {
                if (i === fromEnd || from.charCodeAt(i) === CHAR_BACKWARD_SLASH) {
                    out += out.length === 0 ? '..' : '\\..';
                }
            }
            toStart += lastCommonSep;
            // Lastly, append the rest of the destination (`to`) path that comes after
            // the common path parts
            if (out.length > 0) {
                return `${out}${toOrig.slice(toStart, toEnd)}`;
            }
            if (toOrig.charCodeAt(toStart) === CHAR_BACKWARD_SLASH) {
                ++toStart;
            }
            return toOrig.slice(toStart, toEnd);
        },
        toNamespacedPath(path) {
            // Note: this will *probably* throw somewhere.
            if (typeof path !== 'string') {
                return path;
            }
            if (path.length === 0) {
                return '';
            }
            const resolvedPath = win32.resolve(path);
            if (resolvedPath.length <= 2) {
                return path;
            }
            if (resolvedPath.charCodeAt(0) === CHAR_BACKWARD_SLASH) {
                // Possible UNC root
                if (resolvedPath.charCodeAt(1) === CHAR_BACKWARD_SLASH) {
                    const code = resolvedPath.charCodeAt(2);
                    if (code !== CHAR_QUESTION_MARK && code !== CHAR_DOT) {
                        // Matched non-long UNC root, convert the path to a long UNC path
                        return `\\\\?\\UNC\\${resolvedPath.slice(2)}`;
                    }
                }
            }
            else if (isWindowsDeviceRoot(resolvedPath.charCodeAt(0)) &&
                resolvedPath.charCodeAt(1) === CHAR_COLON &&
                resolvedPath.charCodeAt(2) === CHAR_BACKWARD_SLASH) {
                // Matched device root, convert the path to a long UNC path
                return `\\\\?\\${resolvedPath}`;
            }
            return path;
        },
        dirname(path) {
            validateString(path, 'path');
            const len = path.length;
            if (len === 0) {
                return '.';
            }
            let rootEnd = -1;
            let offset = 0;
            const code = path.charCodeAt(0);
            if (len === 1) {
                // `path` contains just a path separator, exit early to avoid
                // unnecessary work or a dot.
                return isPathSeparator(code) ? path : '.';
            }
            // Try to match a root
            if (isPathSeparator(code)) {
                // Possible UNC root
                rootEnd = offset = 1;
                if (isPathSeparator(path.charCodeAt(1))) {
                    // Matched double path separator at beginning
                    let j = 2;
                    let last = j;
                    // Match 1 or more non-path separators
                    while (j < len && !isPathSeparator(path.charCodeAt(j))) {
                        j++;
                    }
                    if (j < len && j !== last) {
                        // Matched!
                        last = j;
                        // Match 1 or more path separators
                        while (j < len && isPathSeparator(path.charCodeAt(j))) {
                            j++;
                        }
                        if (j < len && j !== last) {
                            // Matched!
                            last = j;
                            // Match 1 or more non-path separators
                            while (j < len && !isPathSeparator(path.charCodeAt(j))) {
                                j++;
                            }
                            if (j === len) {
                                // We matched a UNC root only
                                return path;
                            }
                            if (j !== last) {
                                // We matched a UNC root with leftovers
                                // Offset by 1 to include the separator after the UNC root to
                                // treat it as a "normal root" on top of a (UNC) root
                                rootEnd = offset = j + 1;
                            }
                        }
                    }
                }
                // Possible device root
            }
            else if (isWindowsDeviceRoot(code) && path.charCodeAt(1) === CHAR_COLON) {
                rootEnd = len > 2 && isPathSeparator(path.charCodeAt(2)) ? 3 : 2;
                offset = rootEnd;
            }
            let end = -1;
            let matchedSlash = true;
            for (let i = len - 1; i >= offset; --i) {
                if (isPathSeparator(path.charCodeAt(i))) {
                    if (!matchedSlash) {
                        end = i;
                        break;
                    }
                }
                else {
                    // We saw the first non-path separator
                    matchedSlash = false;
                }
            }
            if (end === -1) {
                if (rootEnd === -1) {
                    return '.';
                }
                end = rootEnd;
            }
            return path.slice(0, end);
        },
        basename(path, ext) {
            if (ext !== undefined) {
                validateString(ext, 'ext');
            }
            validateString(path, 'path');
            let start = 0;
            let end = -1;
            let matchedSlash = true;
            let i;
            // Check for a drive letter prefix so as not to mistake the following
            // path separator as an extra separator at the end of the path that can be
            // disregarded
            if (path.length >= 2 &&
                isWindowsDeviceRoot(path.charCodeAt(0)) &&
                path.charCodeAt(1) === CHAR_COLON) {
                start = 2;
            }
            if (ext !== undefined && ext.length > 0 && ext.length <= path.length) {
                if (ext === path) {
                    return '';
                }
                let extIdx = ext.length - 1;
                let firstNonSlashEnd = -1;
                for (i = path.length - 1; i >= start; --i) {
                    const code = path.charCodeAt(i);
                    if (isPathSeparator(code)) {
                        // If we reached a path separator that was not part of a set of path
                        // separators at the end of the string, stop now
                        if (!matchedSlash) {
                            start = i + 1;
                            break;
                        }
                    }
                    else {
                        if (firstNonSlashEnd === -1) {
                            // We saw the first non-path separator, remember this index in case
                            // we need it if the extension ends up not matching
                            matchedSlash = false;
                            firstNonSlashEnd = i + 1;
                        }
                        if (extIdx >= 0) {
                            // Try to match the explicit extension
                            if (code === ext.charCodeAt(extIdx)) {
                                if (--extIdx === -1) {
                                    // We matched the extension, so mark this as the end of our path
                                    // component
                                    end = i;
                                }
                            }
                            else {
                                // Extension does not match, so our result is the entire path
                                // component
                                extIdx = -1;
                                end = firstNonSlashEnd;
                            }
                        }
                    }
                }
                if (start === end) {
                    end = firstNonSlashEnd;
                }
                else if (end === -1) {
                    end = path.length;
                }
                return path.slice(start, end);
            }
            for (i = path.length - 1; i >= start; --i) {
                if (isPathSeparator(path.charCodeAt(i))) {
                    // If we reached a path separator that was not part of a set of path
                    // separators at the end of the string, stop now
                    if (!matchedSlash) {
                        start = i + 1;
                        break;
                    }
                }
                else if (end === -1) {
                    // We saw the first non-path separator, mark this as the end of our
                    // path component
                    matchedSlash = false;
                    end = i + 1;
                }
            }
            if (end === -1) {
                return '';
            }
            return path.slice(start, end);
        },
        extname(path) {
            validateString(path, 'path');
            let start = 0;
            let startDot = -1;
            let startPart = 0;
            let end = -1;
            let matchedSlash = true;
            // Track the state of characters (if any) we see before our first dot and
            // after any path separator we find
            let preDotState = 0;
            // Check for a drive letter prefix so as not to mistake the following
            // path separator as an extra separator at the end of the path that can be
            // disregarded
            if (path.length >= 2 &&
                path.charCodeAt(1) === CHAR_COLON &&
                isWindowsDeviceRoot(path.charCodeAt(0))) {
                start = startPart = 2;
            }
            for (let i = path.length - 1; i >= start; --i) {
                const code = path.charCodeAt(i);
                if (isPathSeparator(code)) {
                    // If we reached a path separator that was not part of a set of path
                    // separators at the end of the string, stop now
                    if (!matchedSlash) {
                        startPart = i + 1;
                        break;
                    }
                    continue;
                }
                if (end === -1) {
                    // We saw the first non-path separator, mark this as the end of our
                    // extension
                    matchedSlash = false;
                    end = i + 1;
                }
                if (code === CHAR_DOT) {
                    // If this is our first dot, mark it as the start of our extension
                    if (startDot === -1) {
                        startDot = i;
                    }
                    else if (preDotState !== 1) {
                        preDotState = 1;
                    }
                }
                else if (startDot !== -1) {
                    // We saw a non-dot and non-path separator before our dot, so we should
                    // have a good chance at having a non-empty extension
                    preDotState = -1;
                }
            }
            if (startDot === -1 ||
                end === -1 ||
                // We saw a non-dot character immediately before the dot
                preDotState === 0 ||
                // The (right-most) trimmed path component is exactly '..'
                (preDotState === 1 &&
                    startDot === end - 1 &&
                    startDot === startPart + 1)) {
                return '';
            }
            return path.slice(startDot, end);
        },
        format: _format.bind(null, '\\'),
        parse(path) {
            validateString(path, 'path');
            const ret = { root: '', dir: '', base: '', ext: '', name: '' };
            if (path.length === 0) {
                return ret;
            }
            const len = path.length;
            let rootEnd = 0;
            let code = path.charCodeAt(0);
            if (len === 1) {
                if (isPathSeparator(code)) {
                    // `path` contains just a path separator, exit early to avoid
                    // unnecessary work
                    ret.root = ret.dir = path;
                    return ret;
                }
                ret.base = ret.name = path;
                return ret;
            }
            // Try to match a root
            if (isPathSeparator(code)) {
                // Possible UNC root
                rootEnd = 1;
                if (isPathSeparator(path.charCodeAt(1))) {
                    // Matched double path separator at beginning
                    let j = 2;
                    let last = j;
                    // Match 1 or more non-path separators
                    while (j < len && !isPathSeparator(path.charCodeAt(j))) {
                        j++;
                    }
                    if (j < len && j !== last) {
                        // Matched!
                        last = j;
                        // Match 1 or more path separators
                        while (j < len && isPathSeparator(path.charCodeAt(j))) {
                            j++;
                        }
                        if (j < len && j !== last) {
                            // Matched!
                            last = j;
                            // Match 1 or more non-path separators
                            while (j < len && !isPathSeparator(path.charCodeAt(j))) {
                                j++;
                            }
                            if (j === len) {
                                // We matched a UNC root only
                                rootEnd = j;
                            }
                            else if (j !== last) {
                                // We matched a UNC root with leftovers
                                rootEnd = j + 1;
                            }
                        }
                    }
                }
            }
            else if (isWindowsDeviceRoot(code) && path.charCodeAt(1) === CHAR_COLON) {
                // Possible device root
                if (len <= 2) {
                    // `path` contains just a drive root, exit early to avoid
                    // unnecessary work
                    ret.root = ret.dir = path;
                    return ret;
                }
                rootEnd = 2;
                if (isPathSeparator(path.charCodeAt(2))) {
                    if (len === 3) {
                        // `path` contains just a drive root, exit early to avoid
                        // unnecessary work
                        ret.root = ret.dir = path;
                        return ret;
                    }
                    rootEnd = 3;
                }
            }
            if (rootEnd > 0) {
                ret.root = path.slice(0, rootEnd);
            }
            let startDot = -1;
            let startPart = rootEnd;
            let end = -1;
            let matchedSlash = true;
            let i = path.length - 1;
            // Track the state of characters (if any) we see before our first dot and
            // after any path separator we find
            let preDotState = 0;
            // Get non-dir info
            for (; i >= rootEnd; --i) {
                code = path.charCodeAt(i);
                if (isPathSeparator(code)) {
                    // If we reached a path separator that was not part of a set of path
                    // separators at the end of the string, stop now
                    if (!matchedSlash) {
                        startPart = i + 1;
                        break;
                    }
                    continue;
                }
                if (end === -1) {
                    // We saw the first non-path separator, mark this as the end of our
                    // extension
                    matchedSlash = false;
                    end = i + 1;
                }
                if (code === CHAR_DOT) {
                    // If this is our first dot, mark it as the start of our extension
                    if (startDot === -1) {
                        startDot = i;
                    }
                    else if (preDotState !== 1) {
                        preDotState = 1;
                    }
                }
                else if (startDot !== -1) {
                    // We saw a non-dot and non-path separator before our dot, so we should
                    // have a good chance at having a non-empty extension
                    preDotState = -1;
                }
            }
            if (end !== -1) {
                if (startDot === -1 ||
                    // We saw a non-dot character immediately before the dot
                    preDotState === 0 ||
                    // The (right-most) trimmed path component is exactly '..'
                    (preDotState === 1 &&
                        startDot === end - 1 &&
                        startDot === startPart + 1)) {
                    ret.base = ret.name = path.slice(startPart, end);
                }
                else {
                    ret.name = path.slice(startPart, startDot);
                    ret.base = path.slice(startPart, end);
                    ret.ext = path.slice(startDot, end);
                }
            }
            // If the directory is the root, use the entire root as the `dir` including
            // the trailing slash if any (`C:\abc` -> `C:\`). Otherwise, strip out the
            // trailing slash (`C:\abc\def` -> `C:\abc`).
            if (startPart > 0 && startPart !== rootEnd) {
                ret.dir = path.slice(0, startPart - 1);
            }
            else {
                ret.dir = ret.root;
            }
            return ret;
        },
        sep: '\\',
        delimiter: ';',
        win32: null,
        posix: null
    };
    const posix = {
        // path.resolve([from ...], to)
        resolve(...pathSegments) {
            let resolvedPath = '';
            let resolvedAbsolute = false;
            for (let i = pathSegments.length - 1; i >= -1 && !resolvedAbsolute; i--) {
                const path = i >= 0 ? pathSegments[i] : cwd();
                validateString(path, 'path');
                // Skip empty entries
                if (path.length === 0) {
                    continue;
                }
                resolvedPath = `${path}/${resolvedPath}`;
                resolvedAbsolute = path.charCodeAt(0) === CHAR_FORWARD_SLASH;
            }
            // At this point the path should be resolved to a full absolute path, but
            // handle relative paths to be safe (might happen when process.cwd() fails)
            // Normalize the path
            resolvedPath = normalizeString(resolvedPath, !resolvedAbsolute, '/', isPosixPathSeparator);
            if (resolvedAbsolute) {
                return `/${resolvedPath}`;
            }
            return resolvedPath.length > 0 ? resolvedPath : '.';
        },
        normalize(path) {
            validateString(path, 'path');
            if (path.length === 0) {
                return '.';
            }
            const isAbsolute = path.charCodeAt(0) === CHAR_FORWARD_SLASH;
            const trailingSeparator = path.charCodeAt(path.length - 1) === CHAR_FORWARD_SLASH;
            // Normalize the path
            path = normalizeString(path, !isAbsolute, '/', isPosixPathSeparator);
            if (path.length === 0) {
                if (isAbsolute) {
                    return '/';
                }
                return trailingSeparator ? './' : '.';
            }
            if (trailingSeparator) {
                path += '/';
            }
            return isAbsolute ? `/${path}` : path;
        },
        isAbsolute(path) {
            validateString(path, 'path');
            return path.length > 0 && path.charCodeAt(0) === CHAR_FORWARD_SLASH;
        },
        join(...paths) {
            if (paths.length === 0) {
                return '.';
            }
            let joined;
            for (let i = 0; i < paths.length; ++i) {
                const arg = paths[i];
                validateString(arg, 'path');
                if (arg.length > 0) {
                    if (joined === undefined) {
                        joined = arg;
                    }
                    else {
                        joined += `/${arg}`;
                    }
                }
            }
            if (joined === undefined) {
                return '.';
            }
            return posix.normalize(joined);
        },
        relative(from, to) {
            validateString(from, 'from');
            validateString(to, 'to');
            if (from === to) {
                return '';
            }
            // Trim leading forward slashes.
            from = posix.resolve(from);
            to = posix.resolve(to);
            if (from === to) {
                return '';
            }
            const fromStart = 1;
            const fromEnd = from.length;
            const fromLen = fromEnd - fromStart;
            const toStart = 1;
            const toLen = to.length - toStart;
            // Compare paths to find the longest common path from root
            const length = (fromLen < toLen ? fromLen : toLen);
            let lastCommonSep = -1;
            let i = 0;
            for (; i < length; i++) {
                const fromCode = from.charCodeAt(fromStart + i);
                if (fromCode !== to.charCodeAt(toStart + i)) {
                    break;
                }
                else if (fromCode === CHAR_FORWARD_SLASH) {
                    lastCommonSep = i;
                }
            }
            if (i === length) {
                if (toLen > length) {
                    if (to.charCodeAt(toStart + i) === CHAR_FORWARD_SLASH) {
                        // We get here if `from` is the exact base path for `to`.
                        // For example: from='/foo/bar'; to='/foo/bar/baz'
                        return to.slice(toStart + i + 1);
                    }
                    if (i === 0) {
                        // We get here if `from` is the root
                        // For example: from='/'; to='/foo'
                        return to.slice(toStart + i);
                    }
                }
                else if (fromLen > length) {
                    if (from.charCodeAt(fromStart + i) === CHAR_FORWARD_SLASH) {
                        // We get here if `to` is the exact base path for `from`.
                        // For example: from='/foo/bar/baz'; to='/foo/bar'
                        lastCommonSep = i;
                    }
                    else if (i === 0) {
                        // We get here if `to` is the root.
                        // For example: from='/foo/bar'; to='/'
                        lastCommonSep = 0;
                    }
                }
            }
            let out = '';
            // Generate the relative path based on the path difference between `to`
            // and `from`.
            for (i = fromStart + lastCommonSep + 1; i <= fromEnd; ++i) {
                if (i === fromEnd || from.charCodeAt(i) === CHAR_FORWARD_SLASH) {
                    out += out.length === 0 ? '..' : '/..';
                }
            }
            // Lastly, append the rest of the destination (`to`) path that comes after
            // the common path parts.
            return `${out}${to.slice(toStart + lastCommonSep)}`;
        },
        toNamespacedPath(path) {
            // Non-op on posix systems
            return path;
        },
        dirname(path) {
            validateString(path, 'path');
            if (path.length === 0) {
                return '.';
            }
            const hasRoot = path.charCodeAt(0) === CHAR_FORWARD_SLASH;
            let end = -1;
            let matchedSlash = true;
            for (let i = path.length - 1; i >= 1; --i) {
                if (path.charCodeAt(i) === CHAR_FORWARD_SLASH) {
                    if (!matchedSlash) {
                        end = i;
                        break;
                    }
                }
                else {
                    // We saw the first non-path separator
                    matchedSlash = false;
                }
            }
            if (end === -1) {
                return hasRoot ? '/' : '.';
            }
            if (hasRoot && end === 1) {
                return '//';
            }
            return path.slice(0, end);
        },
        basename(path, ext) {
            if (ext !== undefined) {
                validateString(ext, 'ext');
            }
            validateString(path, 'path');
            let start = 0;
            let end = -1;
            let matchedSlash = true;
            let i;
            if (ext !== undefined && ext.length > 0 && ext.length <= path.length) {
                if (ext === path) {
                    return '';
                }
                let extIdx = ext.length - 1;
                let firstNonSlashEnd = -1;
                for (i = path.length - 1; i >= 0; --i) {
                    const code = path.charCodeAt(i);
                    if (code === CHAR_FORWARD_SLASH) {
                        // If we reached a path separator that was not part of a set of path
                        // separators at the end of the string, stop now
                        if (!matchedSlash) {
                            start = i + 1;
                            break;
                        }
                    }
                    else {
                        if (firstNonSlashEnd === -1) {
                            // We saw the first non-path separator, remember this index in case
                            // we need it if the extension ends up not matching
                            matchedSlash = false;
                            firstNonSlashEnd = i + 1;
                        }
                        if (extIdx >= 0) {
                            // Try to match the explicit extension
                            if (code === ext.charCodeAt(extIdx)) {
                                if (--extIdx === -1) {
                                    // We matched the extension, so mark this as the end of our path
                                    // component
                                    end = i;
                                }
                            }
                            else {
                                // Extension does not match, so our result is the entire path
                                // component
                                extIdx = -1;
                                end = firstNonSlashEnd;
                            }
                        }
                    }
                }
                if (start === end) {
                    end = firstNonSlashEnd;
                }
                else if (end === -1) {
                    end = path.length;
                }
                return path.slice(start, end);
            }
            for (i = path.length - 1; i >= 0; --i) {
                if (path.charCodeAt(i) === CHAR_FORWARD_SLASH) {
                    // If we reached a path separator that was not part of a set of path
                    // separators at the end of the string, stop now
                    if (!matchedSlash) {
                        start = i + 1;
                        break;
                    }
                }
                else if (end === -1) {
                    // We saw the first non-path separator, mark this as the end of our
                    // path component
                    matchedSlash = false;
                    end = i + 1;
                }
            }
            if (end === -1) {
                return '';
            }
            return path.slice(start, end);
        },
        extname(path) {
            validateString(path, 'path');
            let startDot = -1;
            let startPart = 0;
            let end = -1;
            let matchedSlash = true;
            // Track the state of characters (if any) we see before our first dot and
            // after any path separator we find
            let preDotState = 0;
            for (let i = path.length - 1; i >= 0; --i) {
                const code = path.charCodeAt(i);
                if (code === CHAR_FORWARD_SLASH) {
                    // If we reached a path separator that was not part of a set of path
                    // separators at the end of the string, stop now
                    if (!matchedSlash) {
                        startPart = i + 1;
                        break;
                    }
                    continue;
                }
                if (end === -1) {
                    // We saw the first non-path separator, mark this as the end of our
                    // extension
                    matchedSlash = false;
                    end = i + 1;
                }
                if (code === CHAR_DOT) {
                    // If this is our first dot, mark it as the start of our extension
                    if (startDot === -1) {
                        startDot = i;
                    }
                    else if (preDotState !== 1) {
                        preDotState = 1;
                    }
                }
                else if (startDot !== -1) {
                    // We saw a non-dot and non-path separator before our dot, so we should
                    // have a good chance at having a non-empty extension
                    preDotState = -1;
                }
            }
            if (startDot === -1 ||
                end === -1 ||
                // We saw a non-dot character immediately before the dot
                preDotState === 0 ||
                // The (right-most) trimmed path component is exactly '..'
                (preDotState === 1 &&
                    startDot === end - 1 &&
                    startDot === startPart + 1)) {
                return '';
            }
            return path.slice(startDot, end);
        },
        format: _format.bind(null, '/'),
        parse(path) {
            validateString(path, 'path');
            const ret = { root: '', dir: '', base: '', ext: '', name: '' };
            if (path.length === 0) {
                return ret;
            }
            const isAbsolute = path.charCodeAt(0) === CHAR_FORWARD_SLASH;
            let start;
            if (isAbsolute) {
                ret.root = '/';
                start = 1;
            }
            else {
                start = 0;
            }
            let startDot = -1;
            let startPart = 0;
            let end = -1;
            let matchedSlash = true;
            let i = path.length - 1;
            // Track the state of characters (if any) we see before our first dot and
            // after any path separator we find
            let preDotState = 0;
            // Get non-dir info
            for (; i >= start; --i) {
                const code = path.charCodeAt(i);
                if (code === CHAR_FORWARD_SLASH) {
                    // If we reached a path separator that was not part of a set of path
                    // separators at the end of the string, stop now
                    if (!matchedSlash) {
                        startPart = i + 1;
                        break;
                    }
                    continue;
                }
                if (end === -1) {
                    // We saw the first non-path separator, mark this as the end of our
                    // extension
                    matchedSlash = false;
                    end = i + 1;
                }
                if (code === CHAR_DOT) {
                    // If this is our first dot, mark it as the start of our extension
                    if (startDot === -1) {
                        startDot = i;
                    }
                    else if (preDotState !== 1) {
                        preDotState = 1;
                    }
                }
                else if (startDot !== -1) {
                    // We saw a non-dot and non-path separator before our dot, so we should
                    // have a good chance at having a non-empty extension
                    preDotState = -1;
                }
            }
            if (end !== -1) {
                const start = startPart === 0 && isAbsolute ? 1 : startPart;
                if (startDot === -1 ||
                    // We saw a non-dot character immediately before the dot
                    preDotState === 0 ||
                    // The (right-most) trimmed path component is exactly '..'
                    (preDotState === 1 &&
                        startDot === end - 1 &&
                        startDot === startPart + 1)) {
                    ret.base = ret.name = path.slice(start, end);
                }
                else {
                    ret.name = path.slice(start, startDot);
                    ret.base = path.slice(start, end);
                    ret.ext = path.slice(startDot, end);
                }
            }
            if (startPart > 0) {
                ret.dir = path.slice(0, startPart - 1);
            }
            else if (isAbsolute) {
                ret.dir = '/';
            }
            return ret;
        },
        sep: '/',
        delimiter: ':',
        win32: null,
        posix: null
    };
    posix.win32 = win32.win32 = win32;
    posix.posix = win32.posix = posix;
    const normalize = (platform === 'win32' ? win32.normalize : posix.normalize);
    (platform === 'win32' ? win32.resolve : posix.resolve);
    (platform === 'win32' ? win32.relative : posix.relative);
    (platform === 'win32' ? win32.dirname : posix.dirname);
    const basename = (platform === 'win32' ? win32.basename : posix.basename);
    const extname = (platform === 'win32' ? win32.extname : posix.extname);
    const sep = (platform === 'win32' ? win32.sep : posix.sep);

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    const _schemePattern = /^\w[\w\d+.-]*$/;
    const _singleSlashStart = /^\//;
    const _doubleSlashStart = /^\/\//;
    function _validateUri(ret, _strict) {
        // scheme, must be set
        if (!ret.scheme && _strict) {
            throw new Error(`[UriError]: Scheme is missing: {scheme: "", authority: "${ret.authority}", path: "${ret.path}", query: "${ret.query}", fragment: "${ret.fragment}"}`);
        }
        // scheme, https://tools.ietf.org/html/rfc3986#section-3.1
        // ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
        if (ret.scheme && !_schemePattern.test(ret.scheme)) {
            throw new Error('[UriError]: Scheme contains illegal characters.');
        }
        // path, http://tools.ietf.org/html/rfc3986#section-3.3
        // If a URI contains an authority component, then the path component
        // must either be empty or begin with a slash ("/") character.  If a URI
        // does not contain an authority component, then the path cannot begin
        // with two slash characters ("//").
        if (ret.path) {
            if (ret.authority) {
                if (!_singleSlashStart.test(ret.path)) {
                    throw new Error('[UriError]: If a URI contains an authority component, then the path component must either be empty or begin with a slash ("/") character');
                }
            }
            else {
                if (_doubleSlashStart.test(ret.path)) {
                    throw new Error('[UriError]: If a URI does not contain an authority component, then the path cannot begin with two slash characters ("//")');
                }
            }
        }
    }
    // for a while we allowed uris *without* schemes and this is the migration
    // for them, e.g. an uri without scheme and without strict-mode warns and falls
    // back to the file-scheme. that should cause the least carnage and still be a
    // clear warning
    function _schemeFix(scheme, _strict) {
        if (!scheme && !_strict) {
            return 'file';
        }
        return scheme;
    }
    // implements a bit of https://tools.ietf.org/html/rfc3986#section-5
    function _referenceResolution(scheme, path) {
        // the slash-character is our 'default base' as we don't
        // support constructing URIs relative to other URIs. This
        // also means that we alter and potentially break paths.
        // see https://tools.ietf.org/html/rfc3986#section-5.1.4
        switch (scheme) {
            case 'https':
            case 'http':
            case 'file':
                if (!path) {
                    path = _slash;
                }
                else if (path[0] !== _slash) {
                    path = _slash + path;
                }
                break;
        }
        return path;
    }
    const _empty = '';
    const _slash = '/';
    const _regexp = /^(([^:/?#]+?):)?(\/\/([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?/;
    /**
     * Uniform Resource Identifier (URI) http://tools.ietf.org/html/rfc3986.
     * This class is a simple parser which creates the basic component parts
     * (http://tools.ietf.org/html/rfc3986#section-3) with minimal validation
     * and encoding.
     *
     * ```txt
     *       foo://example.com:8042/over/there?name=ferret#nose
     *       \_/   \______________/\_________/ \_________/ \__/
     *        |           |            |            |        |
     *     scheme     authority       path        query   fragment
     *        |   _____________________|__
     *       / \ /                        \
     *       urn:example:animal:ferret:nose
     * ```
     */
    class URI {
        /**
         * @internal
         */
        constructor(schemeOrData, authority, path, query, fragment, _strict = false) {
            if (typeof schemeOrData === 'object') {
                this.scheme = schemeOrData.scheme || _empty;
                this.authority = schemeOrData.authority || _empty;
                this.path = schemeOrData.path || _empty;
                this.query = schemeOrData.query || _empty;
                this.fragment = schemeOrData.fragment || _empty;
                // no validation because it's this URI
                // that creates uri components.
                // _validateUri(this);
            }
            else {
                this.scheme = _schemeFix(schemeOrData, _strict);
                this.authority = authority || _empty;
                this.path = _referenceResolution(this.scheme, path || _empty);
                this.query = query || _empty;
                this.fragment = fragment || _empty;
                _validateUri(this, _strict);
            }
        }
        static isUri(thing) {
            if (thing instanceof URI) {
                return true;
            }
            if (!thing) {
                return false;
            }
            return typeof thing.authority === 'string'
                && typeof thing.fragment === 'string'
                && typeof thing.path === 'string'
                && typeof thing.query === 'string'
                && typeof thing.scheme === 'string'
                && typeof thing.fsPath === 'string'
                && typeof thing.with === 'function'
                && typeof thing.toString === 'function';
        }
        // ---- filesystem path -----------------------
        /**
         * Returns a string representing the corresponding file system path of this URI.
         * Will handle UNC paths, normalizes windows drive letters to lower-case, and uses the
         * platform specific path separator.
         *
         * * Will *not* validate the path for invalid characters and semantics.
         * * Will *not* look at the scheme of this URI.
         * * The result shall *not* be used for display purposes but for accessing a file on disk.
         *
         *
         * The *difference* to `URI#path` is the use of the platform specific separator and the handling
         * of UNC paths. See the below sample of a file-uri with an authority (UNC path).
         *
         * ```ts
            const u = URI.parse('file://server/c$/folder/file.txt')
            u.authority === 'server'
            u.path === '/shares/c$/file.txt'
            u.fsPath === '\\server\c$\folder\file.txt'
        ```
         *
         * Using `URI#path` to read a file (using fs-apis) would not be enough because parts of the path,
         * namely the server name, would be missing. Therefore `URI#fsPath` exists - it's sugar to ease working
         * with URIs that represent files on disk (`file` scheme).
         */
        get fsPath() {
            // if (this.scheme !== 'file') {
            // 	console.warn(`[UriError] calling fsPath with scheme ${this.scheme}`);
            // }
            return uriToFsPath(this, false);
        }
        // ---- modify to new -------------------------
        with(change) {
            if (!change) {
                return this;
            }
            let { scheme, authority, path, query, fragment } = change;
            if (scheme === undefined) {
                scheme = this.scheme;
            }
            else if (scheme === null) {
                scheme = _empty;
            }
            if (authority === undefined) {
                authority = this.authority;
            }
            else if (authority === null) {
                authority = _empty;
            }
            if (path === undefined) {
                path = this.path;
            }
            else if (path === null) {
                path = _empty;
            }
            if (query === undefined) {
                query = this.query;
            }
            else if (query === null) {
                query = _empty;
            }
            if (fragment === undefined) {
                fragment = this.fragment;
            }
            else if (fragment === null) {
                fragment = _empty;
            }
            if (scheme === this.scheme
                && authority === this.authority
                && path === this.path
                && query === this.query
                && fragment === this.fragment) {
                return this;
            }
            return new Uri(scheme, authority, path, query, fragment);
        }
        // ---- parse & validate ------------------------
        /**
         * Creates a new URI from a string, e.g. `http://www.example.com/some/path`,
         * `file:///usr/home`, or `scheme:with/path`.
         *
         * @param value A string which represents an URI (see `URI#toString`).
         */
        static parse(value, _strict = false) {
            const match = _regexp.exec(value);
            if (!match) {
                return new Uri(_empty, _empty, _empty, _empty, _empty);
            }
            return new Uri(match[2] || _empty, percentDecode(match[4] || _empty), percentDecode(match[5] || _empty), percentDecode(match[7] || _empty), percentDecode(match[9] || _empty), _strict);
        }
        /**
         * Creates a new URI from a file system path, e.g. `c:\my\files`,
         * `/usr/home`, or `\\server\share\some\path`.
         *
         * The *difference* between `URI#parse` and `URI#file` is that the latter treats the argument
         * as path, not as stringified-uri. E.g. `URI.file(path)` is **not the same as**
         * `URI.parse('file://' + path)` because the path might contain characters that are
         * interpreted (# and ?). See the following sample:
         * ```ts
        const good = URI.file('/coding/c#/project1');
        good.scheme === 'file';
        good.path === '/coding/c#/project1';
        good.fragment === '';
        const bad = URI.parse('file://' + '/coding/c#/project1');
        bad.scheme === 'file';
        bad.path === '/coding/c'; // path is now broken
        bad.fragment === '/project1';
        ```
         *
         * @param path A file system path (see `URI#fsPath`)
         */
        static file(path) {
            let authority = _empty;
            // normalize to fwd-slashes on windows,
            // on other systems bwd-slashes are valid
            // filename character, eg /f\oo/ba\r.txt
            if (isWindows) {
                path = path.replace(/\\/g, _slash);
            }
            // check for authority as used in UNC shares
            // or use the path as given
            if (path[0] === _slash && path[1] === _slash) {
                const idx = path.indexOf(_slash, 2);
                if (idx === -1) {
                    authority = path.substring(2);
                    path = _slash;
                }
                else {
                    authority = path.substring(2, idx);
                    path = path.substring(idx) || _slash;
                }
            }
            return new Uri('file', authority, path, _empty, _empty);
        }
        static from(components) {
            const result = new Uri(components.scheme, components.authority, components.path, components.query, components.fragment);
            _validateUri(result, true);
            return result;
        }
        /**
         * Join a URI path with path fragments and normalizes the resulting path.
         *
         * @param uri The input URI.
         * @param pathFragment The path fragment to add to the URI path.
         * @returns The resulting URI.
         */
        static joinPath(uri, ...pathFragment) {
            if (!uri.path) {
                throw new Error(`[UriError]: cannot call joinPath on URI without path`);
            }
            let newPath;
            if (isWindows && uri.scheme === 'file') {
                newPath = URI.file(win32.join(uriToFsPath(uri, true), ...pathFragment)).path;
            }
            else {
                newPath = posix.join(uri.path, ...pathFragment);
            }
            return uri.with({ path: newPath });
        }
        // ---- printing/externalize ---------------------------
        /**
         * Creates a string representation for this URI. It's guaranteed that calling
         * `URI.parse` with the result of this function creates an URI which is equal
         * to this URI.
         *
         * * The result shall *not* be used for display purposes but for externalization or transport.
         * * The result will be encoded using the percentage encoding and encoding happens mostly
         * ignore the scheme-specific encoding rules.
         *
         * @param skipEncoding Do not encode the result, default is `false`
         */
        toString(skipEncoding = false) {
            return _asFormatted(this, skipEncoding);
        }
        toJSON() {
            return this;
        }
        static revive(data) {
            if (!data) {
                return data;
            }
            else if (data instanceof URI) {
                return data;
            }
            else {
                const result = new Uri(data);
                result._formatted = data.external;
                result._fsPath = data._sep === _pathSepMarker ? data.fsPath : null;
                return result;
            }
        }
    }
    const _pathSepMarker = isWindows ? 1 : undefined;
    // This class exists so that URI is compatible with vscode.Uri (API).
    class Uri extends URI {
        constructor() {
            super(...arguments);
            this._formatted = null;
            this._fsPath = null;
        }
        get fsPath() {
            if (!this._fsPath) {
                this._fsPath = uriToFsPath(this, false);
            }
            return this._fsPath;
        }
        toString(skipEncoding = false) {
            if (!skipEncoding) {
                if (!this._formatted) {
                    this._formatted = _asFormatted(this, false);
                }
                return this._formatted;
            }
            else {
                // we don't cache that
                return _asFormatted(this, true);
            }
        }
        toJSON() {
            const res = {
                $mid: 1 /* Uri */
            };
            // cached state
            if (this._fsPath) {
                res.fsPath = this._fsPath;
                res._sep = _pathSepMarker;
            }
            if (this._formatted) {
                res.external = this._formatted;
            }
            // uri components
            if (this.path) {
                res.path = this.path;
            }
            if (this.scheme) {
                res.scheme = this.scheme;
            }
            if (this.authority) {
                res.authority = this.authority;
            }
            if (this.query) {
                res.query = this.query;
            }
            if (this.fragment) {
                res.fragment = this.fragment;
            }
            return res;
        }
    }
    // reserved characters: https://tools.ietf.org/html/rfc3986#section-2.2
    const encodeTable = {
        [58 /* Colon */]: '%3A',
        [47 /* Slash */]: '%2F',
        [63 /* QuestionMark */]: '%3F',
        [35 /* Hash */]: '%23',
        [91 /* OpenSquareBracket */]: '%5B',
        [93 /* CloseSquareBracket */]: '%5D',
        [64 /* AtSign */]: '%40',
        [33 /* ExclamationMark */]: '%21',
        [36 /* DollarSign */]: '%24',
        [38 /* Ampersand */]: '%26',
        [39 /* SingleQuote */]: '%27',
        [40 /* OpenParen */]: '%28',
        [41 /* CloseParen */]: '%29',
        [42 /* Asterisk */]: '%2A',
        [43 /* Plus */]: '%2B',
        [44 /* Comma */]: '%2C',
        [59 /* Semicolon */]: '%3B',
        [61 /* Equals */]: '%3D',
        [32 /* Space */]: '%20',
    };
    function encodeURIComponentFast(uriComponent, allowSlash) {
        let res = undefined;
        let nativeEncodePos = -1;
        for (let pos = 0; pos < uriComponent.length; pos++) {
            const code = uriComponent.charCodeAt(pos);
            // unreserved characters: https://tools.ietf.org/html/rfc3986#section-2.3
            if ((code >= 97 /* a */ && code <= 122 /* z */)
                || (code >= 65 /* A */ && code <= 90 /* Z */)
                || (code >= 48 /* Digit0 */ && code <= 57 /* Digit9 */)
                || code === 45 /* Dash */
                || code === 46 /* Period */
                || code === 95 /* Underline */
                || code === 126 /* Tilde */
                || (allowSlash && code === 47 /* Slash */)) {
                // check if we are delaying native encode
                if (nativeEncodePos !== -1) {
                    res += encodeURIComponent(uriComponent.substring(nativeEncodePos, pos));
                    nativeEncodePos = -1;
                }
                // check if we write into a new string (by default we try to return the param)
                if (res !== undefined) {
                    res += uriComponent.charAt(pos);
                }
            }
            else {
                // encoding needed, we need to allocate a new string
                if (res === undefined) {
                    res = uriComponent.substr(0, pos);
                }
                // check with default table first
                const escaped = encodeTable[code];
                if (escaped !== undefined) {
                    // check if we are delaying native encode
                    if (nativeEncodePos !== -1) {
                        res += encodeURIComponent(uriComponent.substring(nativeEncodePos, pos));
                        nativeEncodePos = -1;
                    }
                    // append escaped variant to result
                    res += escaped;
                }
                else if (nativeEncodePos === -1) {
                    // use native encode only when needed
                    nativeEncodePos = pos;
                }
            }
        }
        if (nativeEncodePos !== -1) {
            res += encodeURIComponent(uriComponent.substring(nativeEncodePos));
        }
        return res !== undefined ? res : uriComponent;
    }
    function encodeURIComponentMinimal(path) {
        let res = undefined;
        for (let pos = 0; pos < path.length; pos++) {
            const code = path.charCodeAt(pos);
            if (code === 35 /* Hash */ || code === 63 /* QuestionMark */) {
                if (res === undefined) {
                    res = path.substr(0, pos);
                }
                res += encodeTable[code];
            }
            else {
                if (res !== undefined) {
                    res += path[pos];
                }
            }
        }
        return res !== undefined ? res : path;
    }
    /**
     * Compute `fsPath` for the given uri
     */
    function uriToFsPath(uri, keepDriveLetterCasing) {
        let value;
        if (uri.authority && uri.path.length > 1 && uri.scheme === 'file') {
            // unc path: file://shares/c$/far/boo
            value = `//${uri.authority}${uri.path}`;
        }
        else if (uri.path.charCodeAt(0) === 47 /* Slash */
            && (uri.path.charCodeAt(1) >= 65 /* A */ && uri.path.charCodeAt(1) <= 90 /* Z */ || uri.path.charCodeAt(1) >= 97 /* a */ && uri.path.charCodeAt(1) <= 122 /* z */)
            && uri.path.charCodeAt(2) === 58 /* Colon */) {
            if (!keepDriveLetterCasing) {
                // windows drive letter: file:///c:/far/boo
                value = uri.path[1].toLowerCase() + uri.path.substr(2);
            }
            else {
                value = uri.path.substr(1);
            }
        }
        else {
            // other path
            value = uri.path;
        }
        if (isWindows) {
            value = value.replace(/\//g, '\\');
        }
        return value;
    }
    /**
     * Create the external version of a uri
     */
    function _asFormatted(uri, skipEncoding) {
        const encoder = !skipEncoding
            ? encodeURIComponentFast
            : encodeURIComponentMinimal;
        let res = '';
        let { scheme, authority, path, query, fragment } = uri;
        if (scheme) {
            res += scheme;
            res += ':';
        }
        if (authority || scheme === 'file') {
            res += _slash;
            res += _slash;
        }
        if (authority) {
            let idx = authority.indexOf('@');
            if (idx !== -1) {
                // <user>@<auth>
                const userinfo = authority.substr(0, idx);
                authority = authority.substr(idx + 1);
                idx = userinfo.indexOf(':');
                if (idx === -1) {
                    res += encoder(userinfo, false);
                }
                else {
                    // <user>:<pass>@<auth>
                    res += encoder(userinfo.substr(0, idx), false);
                    res += ':';
                    res += encoder(userinfo.substr(idx + 1), false);
                }
                res += '@';
            }
            authority = authority.toLowerCase();
            idx = authority.indexOf(':');
            if (idx === -1) {
                res += encoder(authority, false);
            }
            else {
                // <auth>:<port>
                res += encoder(authority.substr(0, idx), false);
                res += authority.substr(idx);
            }
        }
        if (path) {
            // lower-case windows drive letters in /C:/fff or C:/fff
            if (path.length >= 3 && path.charCodeAt(0) === 47 /* Slash */ && path.charCodeAt(2) === 58 /* Colon */) {
                const code = path.charCodeAt(1);
                if (code >= 65 /* A */ && code <= 90 /* Z */) {
                    path = `/${String.fromCharCode(code + 32)}:${path.substr(3)}`; // "/c:".length === 3
                }
            }
            else if (path.length >= 2 && path.charCodeAt(1) === 58 /* Colon */) {
                const code = path.charCodeAt(0);
                if (code >= 65 /* A */ && code <= 90 /* Z */) {
                    path = `${String.fromCharCode(code + 32)}:${path.substr(2)}`; // "/c:".length === 3
                }
            }
            // encode the rest of the path
            res += encoder(path, true);
        }
        if (query) {
            res += '?';
            res += encoder(query, false);
        }
        if (fragment) {
            res += '#';
            res += !skipEncoding ? encodeURIComponentFast(fragment, false) : fragment;
        }
        return res;
    }
    // --- decode
    function decodeURIComponentGraceful(str) {
        try {
            return decodeURIComponent(str);
        }
        catch (_a) {
            if (str.length > 3) {
                return str.substr(0, 3) + decodeURIComponentGraceful(str.substr(3));
            }
            else {
                return str;
            }
        }
    }
    const _rEncodedAsHex = /(%[0-9A-Za-z][0-9A-Za-z])+/g;
    function percentDecode(str) {
        if (!str.match(_rEncodedAsHex)) {
            return str;
        }
        return str.replace(_rEncodedAsHex, (match) => decodeURIComponentGraceful(match));
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    /**
     * A position in the editor.
     */
    class Position {
        constructor(lineNumber, column) {
            this.lineNumber = lineNumber;
            this.column = column;
        }
        /**
         * Create a new position from this position.
         *
         * @param newLineNumber new line number
         * @param newColumn new column
         */
        with(newLineNumber = this.lineNumber, newColumn = this.column) {
            if (newLineNumber === this.lineNumber && newColumn === this.column) {
                return this;
            }
            else {
                return new Position(newLineNumber, newColumn);
            }
        }
        /**
         * Derive a new position from this position.
         *
         * @param deltaLineNumber line number delta
         * @param deltaColumn column delta
         */
        delta(deltaLineNumber = 0, deltaColumn = 0) {
            return this.with(this.lineNumber + deltaLineNumber, this.column + deltaColumn);
        }
        /**
         * Test if this position equals other position
         */
        equals(other) {
            return Position.equals(this, other);
        }
        /**
         * Test if position `a` equals position `b`
         */
        static equals(a, b) {
            if (!a && !b) {
                return true;
            }
            return (!!a &&
                !!b &&
                a.lineNumber === b.lineNumber &&
                a.column === b.column);
        }
        /**
         * Test if this position is before other position.
         * If the two positions are equal, the result will be false.
         */
        isBefore(other) {
            return Position.isBefore(this, other);
        }
        /**
         * Test if position `a` is before position `b`.
         * If the two positions are equal, the result will be false.
         */
        static isBefore(a, b) {
            if (a.lineNumber < b.lineNumber) {
                return true;
            }
            if (b.lineNumber < a.lineNumber) {
                return false;
            }
            return a.column < b.column;
        }
        /**
         * Test if this position is before other position.
         * If the two positions are equal, the result will be true.
         */
        isBeforeOrEqual(other) {
            return Position.isBeforeOrEqual(this, other);
        }
        /**
         * Test if position `a` is before position `b`.
         * If the two positions are equal, the result will be true.
         */
        static isBeforeOrEqual(a, b) {
            if (a.lineNumber < b.lineNumber) {
                return true;
            }
            if (b.lineNumber < a.lineNumber) {
                return false;
            }
            return a.column <= b.column;
        }
        /**
         * A function that compares positions, useful for sorting
         */
        static compare(a, b) {
            const aLineNumber = a.lineNumber | 0;
            const bLineNumber = b.lineNumber | 0;
            if (aLineNumber === bLineNumber) {
                const aColumn = a.column | 0;
                const bColumn = b.column | 0;
                return aColumn - bColumn;
            }
            return aLineNumber - bLineNumber;
        }
        /**
         * Clone this position.
         */
        clone() {
            return new Position(this.lineNumber, this.column);
        }
        /**
         * Convert to a human-readable representation.
         */
        toString() {
            return '(' + this.lineNumber + ',' + this.column + ')';
        }
        // ---
        /**
         * Create a `Position` from an `IPosition`.
         */
        static lift(pos) {
            return new Position(pos.lineNumber, pos.column);
        }
        /**
         * Test if `obj` is an `IPosition`.
         */
        static isIPosition(obj) {
            return (obj
                && (typeof obj.lineNumber === 'number')
                && (typeof obj.column === 'number'));
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    /**
     * A range in the editor. (startLineNumber,startColumn) is <= (endLineNumber,endColumn)
     */
    class Range {
        constructor(startLineNumber, startColumn, endLineNumber, endColumn) {
            if ((startLineNumber > endLineNumber) || (startLineNumber === endLineNumber && startColumn > endColumn)) {
                this.startLineNumber = endLineNumber;
                this.startColumn = endColumn;
                this.endLineNumber = startLineNumber;
                this.endColumn = startColumn;
            }
            else {
                this.startLineNumber = startLineNumber;
                this.startColumn = startColumn;
                this.endLineNumber = endLineNumber;
                this.endColumn = endColumn;
            }
        }
        /**
         * Test if this range is empty.
         */
        isEmpty() {
            return Range.isEmpty(this);
        }
        /**
         * Test if `range` is empty.
         */
        static isEmpty(range) {
            return (range.startLineNumber === range.endLineNumber && range.startColumn === range.endColumn);
        }
        /**
         * Test if position is in this range. If the position is at the edges, will return true.
         */
        containsPosition(position) {
            return Range.containsPosition(this, position);
        }
        /**
         * Test if `position` is in `range`. If the position is at the edges, will return true.
         */
        static containsPosition(range, position) {
            if (position.lineNumber < range.startLineNumber || position.lineNumber > range.endLineNumber) {
                return false;
            }
            if (position.lineNumber === range.startLineNumber && position.column < range.startColumn) {
                return false;
            }
            if (position.lineNumber === range.endLineNumber && position.column > range.endColumn) {
                return false;
            }
            return true;
        }
        /**
         * Test if `position` is in `range`. If the position is at the edges, will return false.
         * @internal
         */
        static strictContainsPosition(range, position) {
            if (position.lineNumber < range.startLineNumber || position.lineNumber > range.endLineNumber) {
                return false;
            }
            if (position.lineNumber === range.startLineNumber && position.column <= range.startColumn) {
                return false;
            }
            if (position.lineNumber === range.endLineNumber && position.column >= range.endColumn) {
                return false;
            }
            return true;
        }
        /**
         * Test if range is in this range. If the range is equal to this range, will return true.
         */
        containsRange(range) {
            return Range.containsRange(this, range);
        }
        /**
         * Test if `otherRange` is in `range`. If the ranges are equal, will return true.
         */
        static containsRange(range, otherRange) {
            if (otherRange.startLineNumber < range.startLineNumber || otherRange.endLineNumber < range.startLineNumber) {
                return false;
            }
            if (otherRange.startLineNumber > range.endLineNumber || otherRange.endLineNumber > range.endLineNumber) {
                return false;
            }
            if (otherRange.startLineNumber === range.startLineNumber && otherRange.startColumn < range.startColumn) {
                return false;
            }
            if (otherRange.endLineNumber === range.endLineNumber && otherRange.endColumn > range.endColumn) {
                return false;
            }
            return true;
        }
        /**
         * Test if `range` is strictly in this range. `range` must start after and end before this range for the result to be true.
         */
        strictContainsRange(range) {
            return Range.strictContainsRange(this, range);
        }
        /**
         * Test if `otherRange` is strictly in `range` (must start after, and end before). If the ranges are equal, will return false.
         */
        static strictContainsRange(range, otherRange) {
            if (otherRange.startLineNumber < range.startLineNumber || otherRange.endLineNumber < range.startLineNumber) {
                return false;
            }
            if (otherRange.startLineNumber > range.endLineNumber || otherRange.endLineNumber > range.endLineNumber) {
                return false;
            }
            if (otherRange.startLineNumber === range.startLineNumber && otherRange.startColumn <= range.startColumn) {
                return false;
            }
            if (otherRange.endLineNumber === range.endLineNumber && otherRange.endColumn >= range.endColumn) {
                return false;
            }
            return true;
        }
        /**
         * A reunion of the two ranges.
         * The smallest position will be used as the start point, and the largest one as the end point.
         */
        plusRange(range) {
            return Range.plusRange(this, range);
        }
        /**
         * A reunion of the two ranges.
         * The smallest position will be used as the start point, and the largest one as the end point.
         */
        static plusRange(a, b) {
            let startLineNumber;
            let startColumn;
            let endLineNumber;
            let endColumn;
            if (b.startLineNumber < a.startLineNumber) {
                startLineNumber = b.startLineNumber;
                startColumn = b.startColumn;
            }
            else if (b.startLineNumber === a.startLineNumber) {
                startLineNumber = b.startLineNumber;
                startColumn = Math.min(b.startColumn, a.startColumn);
            }
            else {
                startLineNumber = a.startLineNumber;
                startColumn = a.startColumn;
            }
            if (b.endLineNumber > a.endLineNumber) {
                endLineNumber = b.endLineNumber;
                endColumn = b.endColumn;
            }
            else if (b.endLineNumber === a.endLineNumber) {
                endLineNumber = b.endLineNumber;
                endColumn = Math.max(b.endColumn, a.endColumn);
            }
            else {
                endLineNumber = a.endLineNumber;
                endColumn = a.endColumn;
            }
            return new Range(startLineNumber, startColumn, endLineNumber, endColumn);
        }
        /**
         * A intersection of the two ranges.
         */
        intersectRanges(range) {
            return Range.intersectRanges(this, range);
        }
        /**
         * A intersection of the two ranges.
         */
        static intersectRanges(a, b) {
            let resultStartLineNumber = a.startLineNumber;
            let resultStartColumn = a.startColumn;
            let resultEndLineNumber = a.endLineNumber;
            let resultEndColumn = a.endColumn;
            let otherStartLineNumber = b.startLineNumber;
            let otherStartColumn = b.startColumn;
            let otherEndLineNumber = b.endLineNumber;
            let otherEndColumn = b.endColumn;
            if (resultStartLineNumber < otherStartLineNumber) {
                resultStartLineNumber = otherStartLineNumber;
                resultStartColumn = otherStartColumn;
            }
            else if (resultStartLineNumber === otherStartLineNumber) {
                resultStartColumn = Math.max(resultStartColumn, otherStartColumn);
            }
            if (resultEndLineNumber > otherEndLineNumber) {
                resultEndLineNumber = otherEndLineNumber;
                resultEndColumn = otherEndColumn;
            }
            else if (resultEndLineNumber === otherEndLineNumber) {
                resultEndColumn = Math.min(resultEndColumn, otherEndColumn);
            }
            // Check if selection is now empty
            if (resultStartLineNumber > resultEndLineNumber) {
                return null;
            }
            if (resultStartLineNumber === resultEndLineNumber && resultStartColumn > resultEndColumn) {
                return null;
            }
            return new Range(resultStartLineNumber, resultStartColumn, resultEndLineNumber, resultEndColumn);
        }
        /**
         * Test if this range equals other.
         */
        equalsRange(other) {
            return Range.equalsRange(this, other);
        }
        /**
         * Test if range `a` equals `b`.
         */
        static equalsRange(a, b) {
            return (!!a &&
                !!b &&
                a.startLineNumber === b.startLineNumber &&
                a.startColumn === b.startColumn &&
                a.endLineNumber === b.endLineNumber &&
                a.endColumn === b.endColumn);
        }
        /**
         * Return the end position (which will be after or equal to the start position)
         */
        getEndPosition() {
            return Range.getEndPosition(this);
        }
        /**
         * Return the end position (which will be after or equal to the start position)
         */
        static getEndPosition(range) {
            return new Position(range.endLineNumber, range.endColumn);
        }
        /**
         * Return the start position (which will be before or equal to the end position)
         */
        getStartPosition() {
            return Range.getStartPosition(this);
        }
        /**
         * Return the start position (which will be before or equal to the end position)
         */
        static getStartPosition(range) {
            return new Position(range.startLineNumber, range.startColumn);
        }
        /**
         * Transform to a user presentable string representation.
         */
        toString() {
            return '[' + this.startLineNumber + ',' + this.startColumn + ' -> ' + this.endLineNumber + ',' + this.endColumn + ']';
        }
        /**
         * Create a new range using this range's start position, and using endLineNumber and endColumn as the end position.
         */
        setEndPosition(endLineNumber, endColumn) {
            return new Range(this.startLineNumber, this.startColumn, endLineNumber, endColumn);
        }
        /**
         * Create a new range using this range's end position, and using startLineNumber and startColumn as the start position.
         */
        setStartPosition(startLineNumber, startColumn) {
            return new Range(startLineNumber, startColumn, this.endLineNumber, this.endColumn);
        }
        /**
         * Create a new empty range using this range's start position.
         */
        collapseToStart() {
            return Range.collapseToStart(this);
        }
        /**
         * Create a new empty range using this range's start position.
         */
        static collapseToStart(range) {
            return new Range(range.startLineNumber, range.startColumn, range.startLineNumber, range.startColumn);
        }
        // ---
        static fromPositions(start, end = start) {
            return new Range(start.lineNumber, start.column, end.lineNumber, end.column);
        }
        static lift(range) {
            if (!range) {
                return null;
            }
            return new Range(range.startLineNumber, range.startColumn, range.endLineNumber, range.endColumn);
        }
        /**
         * Test if `obj` is an `IRange`.
         */
        static isIRange(obj) {
            return (obj
                && (typeof obj.startLineNumber === 'number')
                && (typeof obj.startColumn === 'number')
                && (typeof obj.endLineNumber === 'number')
                && (typeof obj.endColumn === 'number'));
        }
        /**
         * Test if the two ranges are touching in any way.
         */
        static areIntersectingOrTouching(a, b) {
            // Check if `a` is before `b`
            if (a.endLineNumber < b.startLineNumber || (a.endLineNumber === b.startLineNumber && a.endColumn < b.startColumn)) {
                return false;
            }
            // Check if `b` is before `a`
            if (b.endLineNumber < a.startLineNumber || (b.endLineNumber === a.startLineNumber && b.endColumn < a.startColumn)) {
                return false;
            }
            // These ranges must intersect
            return true;
        }
        /**
         * Test if the two ranges are intersecting. If the ranges are touching it returns true.
         */
        static areIntersecting(a, b) {
            // Check if `a` is before `b`
            if (a.endLineNumber < b.startLineNumber || (a.endLineNumber === b.startLineNumber && a.endColumn <= b.startColumn)) {
                return false;
            }
            // Check if `b` is before `a`
            if (b.endLineNumber < a.startLineNumber || (b.endLineNumber === a.startLineNumber && b.endColumn <= a.startColumn)) {
                return false;
            }
            // These ranges must intersect
            return true;
        }
        /**
         * A function that compares ranges, useful for sorting ranges
         * It will first compare ranges on the startPosition and then on the endPosition
         */
        static compareRangesUsingStarts(a, b) {
            if (a && b) {
                const aStartLineNumber = a.startLineNumber | 0;
                const bStartLineNumber = b.startLineNumber | 0;
                if (aStartLineNumber === bStartLineNumber) {
                    const aStartColumn = a.startColumn | 0;
                    const bStartColumn = b.startColumn | 0;
                    if (aStartColumn === bStartColumn) {
                        const aEndLineNumber = a.endLineNumber | 0;
                        const bEndLineNumber = b.endLineNumber | 0;
                        if (aEndLineNumber === bEndLineNumber) {
                            const aEndColumn = a.endColumn | 0;
                            const bEndColumn = b.endColumn | 0;
                            return aEndColumn - bEndColumn;
                        }
                        return aEndLineNumber - bEndLineNumber;
                    }
                    return aStartColumn - bStartColumn;
                }
                return aStartLineNumber - bStartLineNumber;
            }
            const aExists = (a ? 1 : 0);
            const bExists = (b ? 1 : 0);
            return aExists - bExists;
        }
        /**
         * A function that compares ranges, useful for sorting ranges
         * It will first compare ranges on the endPosition and then on the startPosition
         */
        static compareRangesUsingEnds(a, b) {
            if (a.endLineNumber === b.endLineNumber) {
                if (a.endColumn === b.endColumn) {
                    if (a.startLineNumber === b.startLineNumber) {
                        return a.startColumn - b.startColumn;
                    }
                    return a.startLineNumber - b.startLineNumber;
                }
                return a.endColumn - b.endColumn;
            }
            return a.endLineNumber - b.endLineNumber;
        }
        /**
         * Test if the range spans multiple lines.
         */
        static spansMultipleLines(range) {
            return range.endLineNumber > range.startLineNumber;
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    const MINIMUM_MATCHING_CHARACTER_LENGTH = 3;
    function computeDiff(originalSequence, modifiedSequence, continueProcessingPredicate, pretty) {
        const diffAlgo = new LcsDiff(originalSequence, modifiedSequence, continueProcessingPredicate);
        return diffAlgo.ComputeDiff(pretty);
    }
    class LineSequence {
        constructor(lines) {
            const startColumns = [];
            const endColumns = [];
            for (let i = 0, length = lines.length; i < length; i++) {
                startColumns[i] = getFirstNonBlankColumn(lines[i], 1);
                endColumns[i] = getLastNonBlankColumn(lines[i], 1);
            }
            this.lines = lines;
            this._startColumns = startColumns;
            this._endColumns = endColumns;
        }
        getElements() {
            const elements = [];
            for (let i = 0, len = this.lines.length; i < len; i++) {
                elements[i] = this.lines[i].substring(this._startColumns[i] - 1, this._endColumns[i] - 1);
            }
            return elements;
        }
        getStrictElement(index) {
            return this.lines[index];
        }
        getStartLineNumber(i) {
            return i + 1;
        }
        getEndLineNumber(i) {
            return i + 1;
        }
        createCharSequence(shouldIgnoreTrimWhitespace, startIndex, endIndex) {
            const charCodes = [];
            const lineNumbers = [];
            const columns = [];
            let len = 0;
            for (let index = startIndex; index <= endIndex; index++) {
                const lineContent = this.lines[index];
                const startColumn = (shouldIgnoreTrimWhitespace ? this._startColumns[index] : 1);
                const endColumn = (shouldIgnoreTrimWhitespace ? this._endColumns[index] : lineContent.length + 1);
                for (let col = startColumn; col < endColumn; col++) {
                    charCodes[len] = lineContent.charCodeAt(col - 1);
                    lineNumbers[len] = index + 1;
                    columns[len] = col;
                    len++;
                }
            }
            return new CharSequence(charCodes, lineNumbers, columns);
        }
    }
    class CharSequence {
        constructor(charCodes, lineNumbers, columns) {
            this._charCodes = charCodes;
            this._lineNumbers = lineNumbers;
            this._columns = columns;
        }
        getElements() {
            return this._charCodes;
        }
        getStartLineNumber(i) {
            return this._lineNumbers[i];
        }
        getStartColumn(i) {
            return this._columns[i];
        }
        getEndLineNumber(i) {
            return this._lineNumbers[i];
        }
        getEndColumn(i) {
            return this._columns[i] + 1;
        }
    }
    class CharChange {
        constructor(originalStartLineNumber, originalStartColumn, originalEndLineNumber, originalEndColumn, modifiedStartLineNumber, modifiedStartColumn, modifiedEndLineNumber, modifiedEndColumn) {
            this.originalStartLineNumber = originalStartLineNumber;
            this.originalStartColumn = originalStartColumn;
            this.originalEndLineNumber = originalEndLineNumber;
            this.originalEndColumn = originalEndColumn;
            this.modifiedStartLineNumber = modifiedStartLineNumber;
            this.modifiedStartColumn = modifiedStartColumn;
            this.modifiedEndLineNumber = modifiedEndLineNumber;
            this.modifiedEndColumn = modifiedEndColumn;
        }
        static createFromDiffChange(diffChange, originalCharSequence, modifiedCharSequence) {
            let originalStartLineNumber;
            let originalStartColumn;
            let originalEndLineNumber;
            let originalEndColumn;
            let modifiedStartLineNumber;
            let modifiedStartColumn;
            let modifiedEndLineNumber;
            let modifiedEndColumn;
            if (diffChange.originalLength === 0) {
                originalStartLineNumber = 0;
                originalStartColumn = 0;
                originalEndLineNumber = 0;
                originalEndColumn = 0;
            }
            else {
                originalStartLineNumber = originalCharSequence.getStartLineNumber(diffChange.originalStart);
                originalStartColumn = originalCharSequence.getStartColumn(diffChange.originalStart);
                originalEndLineNumber = originalCharSequence.getEndLineNumber(diffChange.originalStart + diffChange.originalLength - 1);
                originalEndColumn = originalCharSequence.getEndColumn(diffChange.originalStart + diffChange.originalLength - 1);
            }
            if (diffChange.modifiedLength === 0) {
                modifiedStartLineNumber = 0;
                modifiedStartColumn = 0;
                modifiedEndLineNumber = 0;
                modifiedEndColumn = 0;
            }
            else {
                modifiedStartLineNumber = modifiedCharSequence.getStartLineNumber(diffChange.modifiedStart);
                modifiedStartColumn = modifiedCharSequence.getStartColumn(diffChange.modifiedStart);
                modifiedEndLineNumber = modifiedCharSequence.getEndLineNumber(diffChange.modifiedStart + diffChange.modifiedLength - 1);
                modifiedEndColumn = modifiedCharSequence.getEndColumn(diffChange.modifiedStart + diffChange.modifiedLength - 1);
            }
            return new CharChange(originalStartLineNumber, originalStartColumn, originalEndLineNumber, originalEndColumn, modifiedStartLineNumber, modifiedStartColumn, modifiedEndLineNumber, modifiedEndColumn);
        }
    }
    function postProcessCharChanges(rawChanges) {
        if (rawChanges.length <= 1) {
            return rawChanges;
        }
        const result = [rawChanges[0]];
        let prevChange = result[0];
        for (let i = 1, len = rawChanges.length; i < len; i++) {
            const currChange = rawChanges[i];
            const originalMatchingLength = currChange.originalStart - (prevChange.originalStart + prevChange.originalLength);
            const modifiedMatchingLength = currChange.modifiedStart - (prevChange.modifiedStart + prevChange.modifiedLength);
            // Both of the above should be equal, but the continueProcessingPredicate may prevent this from being true
            const matchingLength = Math.min(originalMatchingLength, modifiedMatchingLength);
            if (matchingLength < MINIMUM_MATCHING_CHARACTER_LENGTH) {
                // Merge the current change into the previous one
                prevChange.originalLength = (currChange.originalStart + currChange.originalLength) - prevChange.originalStart;
                prevChange.modifiedLength = (currChange.modifiedStart + currChange.modifiedLength) - prevChange.modifiedStart;
            }
            else {
                // Add the current change
                result.push(currChange);
                prevChange = currChange;
            }
        }
        return result;
    }
    class LineChange {
        constructor(originalStartLineNumber, originalEndLineNumber, modifiedStartLineNumber, modifiedEndLineNumber, charChanges) {
            this.originalStartLineNumber = originalStartLineNumber;
            this.originalEndLineNumber = originalEndLineNumber;
            this.modifiedStartLineNumber = modifiedStartLineNumber;
            this.modifiedEndLineNumber = modifiedEndLineNumber;
            this.charChanges = charChanges;
        }
        static createFromDiffResult(shouldIgnoreTrimWhitespace, diffChange, originalLineSequence, modifiedLineSequence, continueCharDiff, shouldComputeCharChanges, shouldPostProcessCharChanges) {
            let originalStartLineNumber;
            let originalEndLineNumber;
            let modifiedStartLineNumber;
            let modifiedEndLineNumber;
            let charChanges = undefined;
            if (diffChange.originalLength === 0) {
                originalStartLineNumber = originalLineSequence.getStartLineNumber(diffChange.originalStart) - 1;
                originalEndLineNumber = 0;
            }
            else {
                originalStartLineNumber = originalLineSequence.getStartLineNumber(diffChange.originalStart);
                originalEndLineNumber = originalLineSequence.getEndLineNumber(diffChange.originalStart + diffChange.originalLength - 1);
            }
            if (diffChange.modifiedLength === 0) {
                modifiedStartLineNumber = modifiedLineSequence.getStartLineNumber(diffChange.modifiedStart) - 1;
                modifiedEndLineNumber = 0;
            }
            else {
                modifiedStartLineNumber = modifiedLineSequence.getStartLineNumber(diffChange.modifiedStart);
                modifiedEndLineNumber = modifiedLineSequence.getEndLineNumber(diffChange.modifiedStart + diffChange.modifiedLength - 1);
            }
            if (shouldComputeCharChanges && diffChange.originalLength > 0 && diffChange.originalLength < 20 && diffChange.modifiedLength > 0 && diffChange.modifiedLength < 20 && continueCharDiff()) {
                // Compute character changes for diff chunks of at most 20 lines...
                const originalCharSequence = originalLineSequence.createCharSequence(shouldIgnoreTrimWhitespace, diffChange.originalStart, diffChange.originalStart + diffChange.originalLength - 1);
                const modifiedCharSequence = modifiedLineSequence.createCharSequence(shouldIgnoreTrimWhitespace, diffChange.modifiedStart, diffChange.modifiedStart + diffChange.modifiedLength - 1);
                let rawChanges = computeDiff(originalCharSequence, modifiedCharSequence, continueCharDiff, true).changes;
                if (shouldPostProcessCharChanges) {
                    rawChanges = postProcessCharChanges(rawChanges);
                }
                charChanges = [];
                for (let i = 0, length = rawChanges.length; i < length; i++) {
                    charChanges.push(CharChange.createFromDiffChange(rawChanges[i], originalCharSequence, modifiedCharSequence));
                }
            }
            return new LineChange(originalStartLineNumber, originalEndLineNumber, modifiedStartLineNumber, modifiedEndLineNumber, charChanges);
        }
    }
    class DiffComputer {
        constructor(originalLines, modifiedLines, opts) {
            this.shouldComputeCharChanges = opts.shouldComputeCharChanges;
            this.shouldPostProcessCharChanges = opts.shouldPostProcessCharChanges;
            this.shouldIgnoreTrimWhitespace = opts.shouldIgnoreTrimWhitespace;
            this.shouldMakePrettyDiff = opts.shouldMakePrettyDiff;
            this.originalLines = originalLines;
            this.modifiedLines = modifiedLines;
            this.original = new LineSequence(originalLines);
            this.modified = new LineSequence(modifiedLines);
            this.continueLineDiff = createContinueProcessingPredicate(opts.maxComputationTime);
            this.continueCharDiff = createContinueProcessingPredicate(opts.maxComputationTime === 0 ? 0 : Math.min(opts.maxComputationTime, 5000)); // never run after 5s for character changes...
        }
        computeDiff() {
            if (this.original.lines.length === 1 && this.original.lines[0].length === 0) {
                // empty original => fast path
                if (this.modified.lines.length === 1 && this.modified.lines[0].length === 0) {
                    return {
                        quitEarly: false,
                        changes: []
                    };
                }
                return {
                    quitEarly: false,
                    changes: [{
                            originalStartLineNumber: 1,
                            originalEndLineNumber: 1,
                            modifiedStartLineNumber: 1,
                            modifiedEndLineNumber: this.modified.lines.length,
                            charChanges: [{
                                    modifiedEndColumn: 0,
                                    modifiedEndLineNumber: 0,
                                    modifiedStartColumn: 0,
                                    modifiedStartLineNumber: 0,
                                    originalEndColumn: 0,
                                    originalEndLineNumber: 0,
                                    originalStartColumn: 0,
                                    originalStartLineNumber: 0
                                }]
                        }]
                };
            }
            if (this.modified.lines.length === 1 && this.modified.lines[0].length === 0) {
                // empty modified => fast path
                return {
                    quitEarly: false,
                    changes: [{
                            originalStartLineNumber: 1,
                            originalEndLineNumber: this.original.lines.length,
                            modifiedStartLineNumber: 1,
                            modifiedEndLineNumber: 1,
                            charChanges: [{
                                    modifiedEndColumn: 0,
                                    modifiedEndLineNumber: 0,
                                    modifiedStartColumn: 0,
                                    modifiedStartLineNumber: 0,
                                    originalEndColumn: 0,
                                    originalEndLineNumber: 0,
                                    originalStartColumn: 0,
                                    originalStartLineNumber: 0
                                }]
                        }]
                };
            }
            const diffResult = computeDiff(this.original, this.modified, this.continueLineDiff, this.shouldMakePrettyDiff);
            const rawChanges = diffResult.changes;
            const quitEarly = diffResult.quitEarly;
            // The diff is always computed with ignoring trim whitespace
            // This ensures we get the prettiest diff
            if (this.shouldIgnoreTrimWhitespace) {
                const lineChanges = [];
                for (let i = 0, length = rawChanges.length; i < length; i++) {
                    lineChanges.push(LineChange.createFromDiffResult(this.shouldIgnoreTrimWhitespace, rawChanges[i], this.original, this.modified, this.continueCharDiff, this.shouldComputeCharChanges, this.shouldPostProcessCharChanges));
                }
                return {
                    quitEarly: quitEarly,
                    changes: lineChanges
                };
            }
            // Need to post-process and introduce changes where the trim whitespace is different
            // Note that we are looping starting at -1 to also cover the lines before the first change
            const result = [];
            let originalLineIndex = 0;
            let modifiedLineIndex = 0;
            for (let i = -1 /* !!!! */, len = rawChanges.length; i < len; i++) {
                const nextChange = (i + 1 < len ? rawChanges[i + 1] : null);
                const originalStop = (nextChange ? nextChange.originalStart : this.originalLines.length);
                const modifiedStop = (nextChange ? nextChange.modifiedStart : this.modifiedLines.length);
                while (originalLineIndex < originalStop && modifiedLineIndex < modifiedStop) {
                    const originalLine = this.originalLines[originalLineIndex];
                    const modifiedLine = this.modifiedLines[modifiedLineIndex];
                    if (originalLine !== modifiedLine) {
                        // These lines differ only in trim whitespace
                        // Check the leading whitespace
                        {
                            let originalStartColumn = getFirstNonBlankColumn(originalLine, 1);
                            let modifiedStartColumn = getFirstNonBlankColumn(modifiedLine, 1);
                            while (originalStartColumn > 1 && modifiedStartColumn > 1) {
                                const originalChar = originalLine.charCodeAt(originalStartColumn - 2);
                                const modifiedChar = modifiedLine.charCodeAt(modifiedStartColumn - 2);
                                if (originalChar !== modifiedChar) {
                                    break;
                                }
                                originalStartColumn--;
                                modifiedStartColumn--;
                            }
                            if (originalStartColumn > 1 || modifiedStartColumn > 1) {
                                this._pushTrimWhitespaceCharChange(result, originalLineIndex + 1, 1, originalStartColumn, modifiedLineIndex + 1, 1, modifiedStartColumn);
                            }
                        }
                        // Check the trailing whitespace
                        {
                            let originalEndColumn = getLastNonBlankColumn(originalLine, 1);
                            let modifiedEndColumn = getLastNonBlankColumn(modifiedLine, 1);
                            const originalMaxColumn = originalLine.length + 1;
                            const modifiedMaxColumn = modifiedLine.length + 1;
                            while (originalEndColumn < originalMaxColumn && modifiedEndColumn < modifiedMaxColumn) {
                                const originalChar = originalLine.charCodeAt(originalEndColumn - 1);
                                const modifiedChar = originalLine.charCodeAt(modifiedEndColumn - 1);
                                if (originalChar !== modifiedChar) {
                                    break;
                                }
                                originalEndColumn++;
                                modifiedEndColumn++;
                            }
                            if (originalEndColumn < originalMaxColumn || modifiedEndColumn < modifiedMaxColumn) {
                                this._pushTrimWhitespaceCharChange(result, originalLineIndex + 1, originalEndColumn, originalMaxColumn, modifiedLineIndex + 1, modifiedEndColumn, modifiedMaxColumn);
                            }
                        }
                    }
                    originalLineIndex++;
                    modifiedLineIndex++;
                }
                if (nextChange) {
                    // Emit the actual change
                    result.push(LineChange.createFromDiffResult(this.shouldIgnoreTrimWhitespace, nextChange, this.original, this.modified, this.continueCharDiff, this.shouldComputeCharChanges, this.shouldPostProcessCharChanges));
                    originalLineIndex += nextChange.originalLength;
                    modifiedLineIndex += nextChange.modifiedLength;
                }
            }
            return {
                quitEarly: quitEarly,
                changes: result
            };
        }
        _pushTrimWhitespaceCharChange(result, originalLineNumber, originalStartColumn, originalEndColumn, modifiedLineNumber, modifiedStartColumn, modifiedEndColumn) {
            if (this._mergeTrimWhitespaceCharChange(result, originalLineNumber, originalStartColumn, originalEndColumn, modifiedLineNumber, modifiedStartColumn, modifiedEndColumn)) {
                // Merged into previous
                return;
            }
            let charChanges = undefined;
            if (this.shouldComputeCharChanges) {
                charChanges = [new CharChange(originalLineNumber, originalStartColumn, originalLineNumber, originalEndColumn, modifiedLineNumber, modifiedStartColumn, modifiedLineNumber, modifiedEndColumn)];
            }
            result.push(new LineChange(originalLineNumber, originalLineNumber, modifiedLineNumber, modifiedLineNumber, charChanges));
        }
        _mergeTrimWhitespaceCharChange(result, originalLineNumber, originalStartColumn, originalEndColumn, modifiedLineNumber, modifiedStartColumn, modifiedEndColumn) {
            const len = result.length;
            if (len === 0) {
                return false;
            }
            const prevChange = result[len - 1];
            if (prevChange.originalEndLineNumber === 0 || prevChange.modifiedEndLineNumber === 0) {
                // Don't merge with inserts/deletes
                return false;
            }
            if (prevChange.originalEndLineNumber + 1 === originalLineNumber && prevChange.modifiedEndLineNumber + 1 === modifiedLineNumber) {
                prevChange.originalEndLineNumber = originalLineNumber;
                prevChange.modifiedEndLineNumber = modifiedLineNumber;
                if (this.shouldComputeCharChanges && prevChange.charChanges) {
                    prevChange.charChanges.push(new CharChange(originalLineNumber, originalStartColumn, originalLineNumber, originalEndColumn, modifiedLineNumber, modifiedStartColumn, modifiedLineNumber, modifiedEndColumn));
                }
                return true;
            }
            return false;
        }
    }
    function getFirstNonBlankColumn(txt, defaultValue) {
        const r = firstNonWhitespaceIndex(txt);
        if (r === -1) {
            return defaultValue;
        }
        return r + 1;
    }
    function getLastNonBlankColumn(txt, defaultValue) {
        const r = lastNonWhitespaceIndex(txt);
        if (r === -1) {
            return defaultValue;
        }
        return r + 2;
    }
    function createContinueProcessingPredicate(maximumRuntime) {
        if (maximumRuntime === 0) {
            return () => true;
        }
        const startTime = Date.now();
        return () => {
            return Date.now() - startTime < maximumRuntime;
        };
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    function toUint8(v) {
        if (v < 0) {
            return 0;
        }
        if (v > 255 /* MAX_UINT_8 */) {
            return 255 /* MAX_UINT_8 */;
        }
        return v | 0;
    }
    function toUint32(v) {
        if (v < 0) {
            return 0;
        }
        if (v > 4294967295 /* MAX_UINT_32 */) {
            return 4294967295 /* MAX_UINT_32 */;
        }
        return v | 0;
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    class PrefixSumComputer {
        constructor(values) {
            this.values = values;
            this.prefixSum = new Uint32Array(values.length);
            this.prefixSumValidIndex = new Int32Array(1);
            this.prefixSumValidIndex[0] = -1;
        }
        insertValues(insertIndex, insertValues) {
            insertIndex = toUint32(insertIndex);
            const oldValues = this.values;
            const oldPrefixSum = this.prefixSum;
            const insertValuesLen = insertValues.length;
            if (insertValuesLen === 0) {
                return false;
            }
            this.values = new Uint32Array(oldValues.length + insertValuesLen);
            this.values.set(oldValues.subarray(0, insertIndex), 0);
            this.values.set(oldValues.subarray(insertIndex), insertIndex + insertValuesLen);
            this.values.set(insertValues, insertIndex);
            if (insertIndex - 1 < this.prefixSumValidIndex[0]) {
                this.prefixSumValidIndex[0] = insertIndex - 1;
            }
            this.prefixSum = new Uint32Array(this.values.length);
            if (this.prefixSumValidIndex[0] >= 0) {
                this.prefixSum.set(oldPrefixSum.subarray(0, this.prefixSumValidIndex[0] + 1));
            }
            return true;
        }
        setValue(index, value) {
            index = toUint32(index);
            value = toUint32(value);
            if (this.values[index] === value) {
                return false;
            }
            this.values[index] = value;
            if (index - 1 < this.prefixSumValidIndex[0]) {
                this.prefixSumValidIndex[0] = index - 1;
            }
            return true;
        }
        removeValues(startIndex, count) {
            startIndex = toUint32(startIndex);
            count = toUint32(count);
            const oldValues = this.values;
            const oldPrefixSum = this.prefixSum;
            if (startIndex >= oldValues.length) {
                return false;
            }
            const maxCount = oldValues.length - startIndex;
            if (count >= maxCount) {
                count = maxCount;
            }
            if (count === 0) {
                return false;
            }
            this.values = new Uint32Array(oldValues.length - count);
            this.values.set(oldValues.subarray(0, startIndex), 0);
            this.values.set(oldValues.subarray(startIndex + count), startIndex);
            this.prefixSum = new Uint32Array(this.values.length);
            if (startIndex - 1 < this.prefixSumValidIndex[0]) {
                this.prefixSumValidIndex[0] = startIndex - 1;
            }
            if (this.prefixSumValidIndex[0] >= 0) {
                this.prefixSum.set(oldPrefixSum.subarray(0, this.prefixSumValidIndex[0] + 1));
            }
            return true;
        }
        getTotalSum() {
            if (this.values.length === 0) {
                return 0;
            }
            return this._getPrefixSum(this.values.length - 1);
        }
        /**
         * Returns the sum of the first `index + 1` many items.
         * @returns `SUM(0 <= j <= index, values[j])`.
         */
        getPrefixSum(index) {
            if (index < 0) {
                return 0;
            }
            index = toUint32(index);
            return this._getPrefixSum(index);
        }
        _getPrefixSum(index) {
            if (index <= this.prefixSumValidIndex[0]) {
                return this.prefixSum[index];
            }
            let startIndex = this.prefixSumValidIndex[0] + 1;
            if (startIndex === 0) {
                this.prefixSum[0] = this.values[0];
                startIndex++;
            }
            if (index >= this.values.length) {
                index = this.values.length - 1;
            }
            for (let i = startIndex; i <= index; i++) {
                this.prefixSum[i] = this.prefixSum[i - 1] + this.values[i];
            }
            this.prefixSumValidIndex[0] = Math.max(this.prefixSumValidIndex[0], index);
            return this.prefixSum[index];
        }
        getIndexOf(sum) {
            sum = Math.floor(sum);
            // Compute all sums (to get a fully valid prefixSum)
            this.getTotalSum();
            let low = 0;
            let high = this.values.length - 1;
            let mid = 0;
            let midStop = 0;
            let midStart = 0;
            while (low <= high) {
                mid = low + ((high - low) / 2) | 0;
                midStop = this.prefixSum[mid];
                midStart = midStop - this.values[mid];
                if (sum < midStart) {
                    high = mid - 1;
                }
                else if (sum >= midStop) {
                    low = mid + 1;
                }
                else {
                    break;
                }
            }
            return new PrefixSumIndexOfResult(mid, sum - midStart);
        }
    }
    class PrefixSumIndexOfResult {
        constructor(index, remainder) {
            this.index = index;
            this.remainder = remainder;
            this._prefixSumIndexOfResultBrand = undefined;
            this.index = index;
            this.remainder = remainder;
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    class MirrorTextModel {
        constructor(uri, lines, eol, versionId) {
            this._uri = uri;
            this._lines = lines;
            this._eol = eol;
            this._versionId = versionId;
            this._lineStarts = null;
            this._cachedTextValue = null;
        }
        dispose() {
            this._lines.length = 0;
        }
        get version() {
            return this._versionId;
        }
        getText() {
            if (this._cachedTextValue === null) {
                this._cachedTextValue = this._lines.join(this._eol);
            }
            return this._cachedTextValue;
        }
        onEvents(e) {
            if (e.eol && e.eol !== this._eol) {
                this._eol = e.eol;
                this._lineStarts = null;
            }
            // Update my lines
            const changes = e.changes;
            for (const change of changes) {
                this._acceptDeleteRange(change.range);
                this._acceptInsertText(new Position(change.range.startLineNumber, change.range.startColumn), change.text);
            }
            this._versionId = e.versionId;
            this._cachedTextValue = null;
        }
        _ensureLineStarts() {
            if (!this._lineStarts) {
                const eolLength = this._eol.length;
                const linesLength = this._lines.length;
                const lineStartValues = new Uint32Array(linesLength);
                for (let i = 0; i < linesLength; i++) {
                    lineStartValues[i] = this._lines[i].length + eolLength;
                }
                this._lineStarts = new PrefixSumComputer(lineStartValues);
            }
        }
        /**
         * All changes to a line's text go through this method
         */
        _setLineText(lineIndex, newValue) {
            this._lines[lineIndex] = newValue;
            if (this._lineStarts) {
                // update prefix sum
                this._lineStarts.setValue(lineIndex, this._lines[lineIndex].length + this._eol.length);
            }
        }
        _acceptDeleteRange(range) {
            if (range.startLineNumber === range.endLineNumber) {
                if (range.startColumn === range.endColumn) {
                    // Nothing to delete
                    return;
                }
                // Delete text on the affected line
                this._setLineText(range.startLineNumber - 1, this._lines[range.startLineNumber - 1].substring(0, range.startColumn - 1)
                    + this._lines[range.startLineNumber - 1].substring(range.endColumn - 1));
                return;
            }
            // Take remaining text on last line and append it to remaining text on first line
            this._setLineText(range.startLineNumber - 1, this._lines[range.startLineNumber - 1].substring(0, range.startColumn - 1)
                + this._lines[range.endLineNumber - 1].substring(range.endColumn - 1));
            // Delete middle lines
            this._lines.splice(range.startLineNumber, range.endLineNumber - range.startLineNumber);
            if (this._lineStarts) {
                // update prefix sum
                this._lineStarts.removeValues(range.startLineNumber, range.endLineNumber - range.startLineNumber);
            }
        }
        _acceptInsertText(position, insertText) {
            if (insertText.length === 0) {
                // Nothing to insert
                return;
            }
            const insertLines = splitLines(insertText);
            if (insertLines.length === 1) {
                // Inserting text on one line
                this._setLineText(position.lineNumber - 1, this._lines[position.lineNumber - 1].substring(0, position.column - 1)
                    + insertLines[0]
                    + this._lines[position.lineNumber - 1].substring(position.column - 1));
                return;
            }
            // Append overflowing text from first line to the end of text to insert
            insertLines[insertLines.length - 1] += this._lines[position.lineNumber - 1].substring(position.column - 1);
            // Delete overflowing text from first line and insert text on first line
            this._setLineText(position.lineNumber - 1, this._lines[position.lineNumber - 1].substring(0, position.column - 1)
                + insertLines[0]);
            // Insert new lines & store lengths
            const newLengths = new Uint32Array(insertLines.length - 1);
            for (let i = 1; i < insertLines.length; i++) {
                this._lines.splice(position.lineNumber + i - 1, 0, insertLines[i]);
                newLengths[i - 1] = insertLines[i].length + this._eol.length;
            }
            if (this._lineStarts) {
                // update prefix sum
                this._lineStarts.insertValues(position.lineNumber, newLengths);
            }
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    const USUAL_WORD_SEPARATORS = '`~!@#$%^&*()-=+[{]}\\|;:\'",.<>/?';
    /**
     * Create a word definition regular expression based on default word separators.
     * Optionally provide allowed separators that should be included in words.
     *
     * The default would look like this:
     * /(-?\d*\.\d\w*)|([^\`\~\!\@\#\$\%\^\&\*\(\)\-\=\+\[\{\]\}\\\|\;\:\'\"\,\.\<\>\/\?\s]+)/g
     */
    function createWordRegExp(allowInWords = '') {
        let source = '(-?\\d*\\.\\d\\w*)|([^';
        for (const sep of USUAL_WORD_SEPARATORS) {
            if (allowInWords.indexOf(sep) >= 0) {
                continue;
            }
            source += '\\' + sep;
        }
        source += '\\s]+)';
        return new RegExp(source, 'g');
    }
    // catches numbers (including floating numbers) in the first group, and alphanum in the second
    const DEFAULT_WORD_REGEXP = createWordRegExp();
    function ensureValidWordDefinition(wordDefinition) {
        let result = DEFAULT_WORD_REGEXP;
        if (wordDefinition && (wordDefinition instanceof RegExp)) {
            if (!wordDefinition.global) {
                let flags = 'g';
                if (wordDefinition.ignoreCase) {
                    flags += 'i';
                }
                if (wordDefinition.multiline) {
                    flags += 'm';
                }
                if (wordDefinition.unicode) {
                    flags += 'u';
                }
                result = new RegExp(wordDefinition.source, flags);
            }
            else {
                result = wordDefinition;
            }
        }
        result.lastIndex = 0;
        return result;
    }
    const _defaultConfig = {
        maxLen: 1000,
        windowSize: 15,
        timeBudget: 150
    };
    function getWordAtText(column, wordDefinition, text, textOffset, config = _defaultConfig) {
        if (text.length > config.maxLen) {
            // don't throw strings that long at the regexp
            // but use a sub-string in which a word must occur
            let start = column - config.maxLen / 2;
            if (start < 0) {
                start = 0;
            }
            else {
                textOffset += start;
            }
            text = text.substring(start, column + config.maxLen / 2);
            return getWordAtText(column, wordDefinition, text, textOffset, config);
        }
        const t1 = Date.now();
        const pos = column - 1 - textOffset;
        let prevRegexIndex = -1;
        let match = null;
        for (let i = 1;; i++) {
            // check time budget
            if (Date.now() - t1 >= config.timeBudget) {
                break;
            }
            // reset the index at which the regexp should start matching, also know where it
            // should stop so that subsequent search don't repeat previous searches
            const regexIndex = pos - config.windowSize * i;
            wordDefinition.lastIndex = Math.max(0, regexIndex);
            const thisMatch = _findRegexMatchEnclosingPosition(wordDefinition, text, pos, prevRegexIndex);
            if (!thisMatch && match) {
                // stop: we have something
                break;
            }
            match = thisMatch;
            // stop: searched at start
            if (regexIndex <= 0) {
                break;
            }
            prevRegexIndex = regexIndex;
        }
        if (match) {
            const result = {
                word: match[0],
                startColumn: textOffset + 1 + match.index,
                endColumn: textOffset + 1 + match.index + match[0].length
            };
            wordDefinition.lastIndex = 0;
            return result;
        }
        return null;
    }
    function _findRegexMatchEnclosingPosition(wordDefinition, text, pos, stopPos) {
        let match;
        while (match = wordDefinition.exec(text)) {
            const matchIndex = match.index || 0;
            if (matchIndex <= pos && wordDefinition.lastIndex >= pos) {
                return match;
            }
            else if (stopPos > 0 && matchIndex > stopPos) {
                return null;
            }
        }
        return null;
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    /**
     * A fast character classifier that uses a compact array for ASCII values.
     */
    class CharacterClassifier {
        constructor(_defaultValue) {
            const defaultValue = toUint8(_defaultValue);
            this._defaultValue = defaultValue;
            this._asciiMap = CharacterClassifier._createAsciiMap(defaultValue);
            this._map = new Map();
        }
        static _createAsciiMap(defaultValue) {
            const asciiMap = new Uint8Array(256);
            for (let i = 0; i < 256; i++) {
                asciiMap[i] = defaultValue;
            }
            return asciiMap;
        }
        set(charCode, _value) {
            const value = toUint8(_value);
            if (charCode >= 0 && charCode < 256) {
                this._asciiMap[charCode] = value;
            }
            else {
                this._map.set(charCode, value);
            }
        }
        get(charCode) {
            if (charCode >= 0 && charCode < 256) {
                return this._asciiMap[charCode];
            }
            else {
                return (this._map.get(charCode) || this._defaultValue);
            }
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    class Uint8Matrix {
        constructor(rows, cols, defaultValue) {
            const data = new Uint8Array(rows * cols);
            for (let i = 0, len = rows * cols; i < len; i++) {
                data[i] = defaultValue;
            }
            this._data = data;
            this.rows = rows;
            this.cols = cols;
        }
        get(row, col) {
            return this._data[row * this.cols + col];
        }
        set(row, col, value) {
            this._data[row * this.cols + col] = value;
        }
    }
    class StateMachine {
        constructor(edges) {
            let maxCharCode = 0;
            let maxState = 0 /* Invalid */;
            for (let i = 0, len = edges.length; i < len; i++) {
                const [from, chCode, to] = edges[i];
                if (chCode > maxCharCode) {
                    maxCharCode = chCode;
                }
                if (from > maxState) {
                    maxState = from;
                }
                if (to > maxState) {
                    maxState = to;
                }
            }
            maxCharCode++;
            maxState++;
            const states = new Uint8Matrix(maxState, maxCharCode, 0 /* Invalid */);
            for (let i = 0, len = edges.length; i < len; i++) {
                const [from, chCode, to] = edges[i];
                states.set(from, chCode, to);
            }
            this._states = states;
            this._maxCharCode = maxCharCode;
        }
        nextState(currentState, chCode) {
            if (chCode < 0 || chCode >= this._maxCharCode) {
                return 0 /* Invalid */;
            }
            return this._states.get(currentState, chCode);
        }
    }
    // State machine for http:// or https:// or file://
    let _stateMachine = null;
    function getStateMachine() {
        if (_stateMachine === null) {
            _stateMachine = new StateMachine([
                [1 /* Start */, 104 /* h */, 2 /* H */],
                [1 /* Start */, 72 /* H */, 2 /* H */],
                [1 /* Start */, 102 /* f */, 6 /* F */],
                [1 /* Start */, 70 /* F */, 6 /* F */],
                [2 /* H */, 116 /* t */, 3 /* HT */],
                [2 /* H */, 84 /* T */, 3 /* HT */],
                [3 /* HT */, 116 /* t */, 4 /* HTT */],
                [3 /* HT */, 84 /* T */, 4 /* HTT */],
                [4 /* HTT */, 112 /* p */, 5 /* HTTP */],
                [4 /* HTT */, 80 /* P */, 5 /* HTTP */],
                [5 /* HTTP */, 115 /* s */, 9 /* BeforeColon */],
                [5 /* HTTP */, 83 /* S */, 9 /* BeforeColon */],
                [5 /* HTTP */, 58 /* Colon */, 10 /* AfterColon */],
                [6 /* F */, 105 /* i */, 7 /* FI */],
                [6 /* F */, 73 /* I */, 7 /* FI */],
                [7 /* FI */, 108 /* l */, 8 /* FIL */],
                [7 /* FI */, 76 /* L */, 8 /* FIL */],
                [8 /* FIL */, 101 /* e */, 9 /* BeforeColon */],
                [8 /* FIL */, 69 /* E */, 9 /* BeforeColon */],
                [9 /* BeforeColon */, 58 /* Colon */, 10 /* AfterColon */],
                [10 /* AfterColon */, 47 /* Slash */, 11 /* AlmostThere */],
                [11 /* AlmostThere */, 47 /* Slash */, 12 /* End */],
            ]);
        }
        return _stateMachine;
    }
    let _classifier = null;
    function getClassifier() {
        if (_classifier === null) {
            _classifier = new CharacterClassifier(0 /* None */);
            // allow-any-unicode-next-line
            const FORCE_TERMINATION_CHARACTERS = ' \t<>\'\"、。｡､，．：；‘〈「『〔（［｛｢｣｝］）〕』」〉’｀～…';
            for (let i = 0; i < FORCE_TERMINATION_CHARACTERS.length; i++) {
                _classifier.set(FORCE_TERMINATION_CHARACTERS.charCodeAt(i), 1 /* ForceTermination */);
            }
            const CANNOT_END_WITH_CHARACTERS = '.,;';
            for (let i = 0; i < CANNOT_END_WITH_CHARACTERS.length; i++) {
                _classifier.set(CANNOT_END_WITH_CHARACTERS.charCodeAt(i), 2 /* CannotEndIn */);
            }
        }
        return _classifier;
    }
    class LinkComputer {
        static _createLink(classifier, line, lineNumber, linkBeginIndex, linkEndIndex) {
            // Do not allow to end link in certain characters...
            let lastIncludedCharIndex = linkEndIndex - 1;
            do {
                const chCode = line.charCodeAt(lastIncludedCharIndex);
                const chClass = classifier.get(chCode);
                if (chClass !== 2 /* CannotEndIn */) {
                    break;
                }
                lastIncludedCharIndex--;
            } while (lastIncludedCharIndex > linkBeginIndex);
            // Handle links enclosed in parens, square brackets and curlys.
            if (linkBeginIndex > 0) {
                const charCodeBeforeLink = line.charCodeAt(linkBeginIndex - 1);
                const lastCharCodeInLink = line.charCodeAt(lastIncludedCharIndex);
                if ((charCodeBeforeLink === 40 /* OpenParen */ && lastCharCodeInLink === 41 /* CloseParen */)
                    || (charCodeBeforeLink === 91 /* OpenSquareBracket */ && lastCharCodeInLink === 93 /* CloseSquareBracket */)
                    || (charCodeBeforeLink === 123 /* OpenCurlyBrace */ && lastCharCodeInLink === 125 /* CloseCurlyBrace */)) {
                    // Do not end in ) if ( is before the link start
                    // Do not end in ] if [ is before the link start
                    // Do not end in } if { is before the link start
                    lastIncludedCharIndex--;
                }
            }
            return {
                range: {
                    startLineNumber: lineNumber,
                    startColumn: linkBeginIndex + 1,
                    endLineNumber: lineNumber,
                    endColumn: lastIncludedCharIndex + 2
                },
                url: line.substring(linkBeginIndex, lastIncludedCharIndex + 1)
            };
        }
        static computeLinks(model, stateMachine = getStateMachine()) {
            const classifier = getClassifier();
            const result = [];
            for (let i = 1, lineCount = model.getLineCount(); i <= lineCount; i++) {
                const line = model.getLineContent(i);
                const len = line.length;
                let j = 0;
                let linkBeginIndex = 0;
                let linkBeginChCode = 0;
                let state = 1 /* Start */;
                let hasOpenParens = false;
                let hasOpenSquareBracket = false;
                let inSquareBrackets = false;
                let hasOpenCurlyBracket = false;
                while (j < len) {
                    let resetStateMachine = false;
                    const chCode = line.charCodeAt(j);
                    if (state === 13 /* Accept */) {
                        let chClass;
                        switch (chCode) {
                            case 40 /* OpenParen */:
                                hasOpenParens = true;
                                chClass = 0 /* None */;
                                break;
                            case 41 /* CloseParen */:
                                chClass = (hasOpenParens ? 0 /* None */ : 1 /* ForceTermination */);
                                break;
                            case 91 /* OpenSquareBracket */:
                                inSquareBrackets = true;
                                hasOpenSquareBracket = true;
                                chClass = 0 /* None */;
                                break;
                            case 93 /* CloseSquareBracket */:
                                inSquareBrackets = false;
                                chClass = (hasOpenSquareBracket ? 0 /* None */ : 1 /* ForceTermination */);
                                break;
                            case 123 /* OpenCurlyBrace */:
                                hasOpenCurlyBracket = true;
                                chClass = 0 /* None */;
                                break;
                            case 125 /* CloseCurlyBrace */:
                                chClass = (hasOpenCurlyBracket ? 0 /* None */ : 1 /* ForceTermination */);
                                break;
                            /* The following three rules make it that ' or " or ` are allowed inside links if the link began with a different one */
                            case 39 /* SingleQuote */:
                                chClass = (linkBeginChCode === 34 /* DoubleQuote */ || linkBeginChCode === 96 /* BackTick */) ? 0 /* None */ : 1 /* ForceTermination */;
                                break;
                            case 34 /* DoubleQuote */:
                                chClass = (linkBeginChCode === 39 /* SingleQuote */ || linkBeginChCode === 96 /* BackTick */) ? 0 /* None */ : 1 /* ForceTermination */;
                                break;
                            case 96 /* BackTick */:
                                chClass = (linkBeginChCode === 39 /* SingleQuote */ || linkBeginChCode === 34 /* DoubleQuote */) ? 0 /* None */ : 1 /* ForceTermination */;
                                break;
                            case 42 /* Asterisk */:
                                // `*` terminates a link if the link began with `*`
                                chClass = (linkBeginChCode === 42 /* Asterisk */) ? 1 /* ForceTermination */ : 0 /* None */;
                                break;
                            case 124 /* Pipe */:
                                // `|` terminates a link if the link began with `|`
                                chClass = (linkBeginChCode === 124 /* Pipe */) ? 1 /* ForceTermination */ : 0 /* None */;
                                break;
                            case 32 /* Space */:
                                // ` ` allow space in between [ and ]
                                chClass = (inSquareBrackets ? 0 /* None */ : 1 /* ForceTermination */);
                                break;
                            default:
                                chClass = classifier.get(chCode);
                        }
                        // Check if character terminates link
                        if (chClass === 1 /* ForceTermination */) {
                            result.push(LinkComputer._createLink(classifier, line, i, linkBeginIndex, j));
                            resetStateMachine = true;
                        }
                    }
                    else if (state === 12 /* End */) {
                        let chClass;
                        if (chCode === 91 /* OpenSquareBracket */) {
                            // Allow for the authority part to contain ipv6 addresses which contain [ and ]
                            hasOpenSquareBracket = true;
                            chClass = 0 /* None */;
                        }
                        else {
                            chClass = classifier.get(chCode);
                        }
                        // Check if character terminates link
                        if (chClass === 1 /* ForceTermination */) {
                            resetStateMachine = true;
                        }
                        else {
                            state = 13 /* Accept */;
                        }
                    }
                    else {
                        state = stateMachine.nextState(state, chCode);
                        if (state === 0 /* Invalid */) {
                            resetStateMachine = true;
                        }
                    }
                    if (resetStateMachine) {
                        state = 1 /* Start */;
                        hasOpenParens = false;
                        hasOpenSquareBracket = false;
                        hasOpenCurlyBracket = false;
                        // Record where the link started
                        linkBeginIndex = j + 1;
                        linkBeginChCode = chCode;
                    }
                    j++;
                }
                if (state === 13 /* Accept */) {
                    result.push(LinkComputer._createLink(classifier, line, i, linkBeginIndex, len));
                }
            }
            return result;
        }
    }
    /**
     * Returns an array of all links contains in the provided
     * document. *Note* that this operation is computational
     * expensive and should not run in the UI thread.
     */
    function computeLinks(model) {
        if (!model || typeof model.getLineCount !== 'function' || typeof model.getLineContent !== 'function') {
            // Unknown caller!
            return [];
        }
        return LinkComputer.computeLinks(model);
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    class BasicInplaceReplace {
        constructor() {
            this._defaultValueSet = [
                ['true', 'false'],
                ['True', 'False'],
                ['Private', 'Public', 'Friend', 'ReadOnly', 'Partial', 'Protected', 'WriteOnly'],
                ['public', 'protected', 'private'],
            ];
        }
        navigateValueSet(range1, text1, range2, text2, up) {
            if (range1 && text1) {
                const result = this.doNavigateValueSet(text1, up);
                if (result) {
                    return {
                        range: range1,
                        value: result
                    };
                }
            }
            if (range2 && text2) {
                const result = this.doNavigateValueSet(text2, up);
                if (result) {
                    return {
                        range: range2,
                        value: result
                    };
                }
            }
            return null;
        }
        doNavigateValueSet(text, up) {
            const numberResult = this.numberReplace(text, up);
            if (numberResult !== null) {
                return numberResult;
            }
            return this.textReplace(text, up);
        }
        numberReplace(value, up) {
            const precision = Math.pow(10, value.length - (value.lastIndexOf('.') + 1));
            let n1 = Number(value);
            let n2 = parseFloat(value);
            if (!isNaN(n1) && !isNaN(n2) && n1 === n2) {
                if (n1 === 0 && !up) {
                    return null; // don't do negative
                    //			} else if(n1 === 9 && up) {
                    //				return null; // don't insert 10 into a number
                }
                else {
                    n1 = Math.floor(n1 * precision);
                    n1 += up ? precision : -precision;
                    return String(n1 / precision);
                }
            }
            return null;
        }
        textReplace(value, up) {
            return this.valueSetsReplace(this._defaultValueSet, value, up);
        }
        valueSetsReplace(valueSets, value, up) {
            let result = null;
            for (let i = 0, len = valueSets.length; result === null && i < len; i++) {
                result = this.valueSetReplace(valueSets[i], value, up);
            }
            return result;
        }
        valueSetReplace(valueSet, value, up) {
            let idx = valueSet.indexOf(value);
            if (idx >= 0) {
                idx += up ? +1 : -1;
                if (idx < 0) {
                    idx = valueSet.length - 1;
                }
                else {
                    idx %= valueSet.length;
                }
                return valueSet[idx];
            }
            return null;
        }
    }
    BasicInplaceReplace.INSTANCE = new BasicInplaceReplace();

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    const shortcutEvent = Object.freeze(function (callback, context) {
        const handle = setTimeout(callback.bind(context), 0);
        return { dispose() { clearTimeout(handle); } };
    });
    var CancellationToken;
    (function (CancellationToken) {
        function isCancellationToken(thing) {
            if (thing === CancellationToken.None || thing === CancellationToken.Cancelled) {
                return true;
            }
            if (thing instanceof MutableToken) {
                return true;
            }
            if (!thing || typeof thing !== 'object') {
                return false;
            }
            return typeof thing.isCancellationRequested === 'boolean'
                && typeof thing.onCancellationRequested === 'function';
        }
        CancellationToken.isCancellationToken = isCancellationToken;
        CancellationToken.None = Object.freeze({
            isCancellationRequested: false,
            onCancellationRequested: Event.None
        });
        CancellationToken.Cancelled = Object.freeze({
            isCancellationRequested: true,
            onCancellationRequested: shortcutEvent
        });
    })(CancellationToken || (CancellationToken = {}));
    class MutableToken {
        constructor() {
            this._isCancelled = false;
            this._emitter = null;
        }
        cancel() {
            if (!this._isCancelled) {
                this._isCancelled = true;
                if (this._emitter) {
                    this._emitter.fire(undefined);
                    this.dispose();
                }
            }
        }
        get isCancellationRequested() {
            return this._isCancelled;
        }
        get onCancellationRequested() {
            if (this._isCancelled) {
                return shortcutEvent;
            }
            if (!this._emitter) {
                this._emitter = new Emitter();
            }
            return this._emitter.event;
        }
        dispose() {
            if (this._emitter) {
                this._emitter.dispose();
                this._emitter = null;
            }
        }
    }
    class CancellationTokenSource {
        constructor(parent) {
            this._token = undefined;
            this._parentListener = undefined;
            this._parentListener = parent && parent.onCancellationRequested(this.cancel, this);
        }
        get token() {
            if (!this._token) {
                // be lazy and create the token only when
                // actually needed
                this._token = new MutableToken();
            }
            return this._token;
        }
        cancel() {
            if (!this._token) {
                // save an object by returning the default
                // cancelled token when cancellation happens
                // before someone asks for the token
                this._token = CancellationToken.Cancelled;
            }
            else if (this._token instanceof MutableToken) {
                // actually cancel
                this._token.cancel();
            }
        }
        dispose(cancel = false) {
            if (cancel) {
                this.cancel();
            }
            if (this._parentListener) {
                this._parentListener.dispose();
            }
            if (!this._token) {
                // ensure to initialize with an empty token if we had none
                this._token = CancellationToken.None;
            }
            else if (this._token instanceof MutableToken) {
                // actually dispose
                this._token.dispose();
            }
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    class KeyCodeStrMap {
        constructor() {
            this._keyCodeToStr = [];
            this._strToKeyCode = Object.create(null);
        }
        define(keyCode, str) {
            this._keyCodeToStr[keyCode] = str;
            this._strToKeyCode[str.toLowerCase()] = keyCode;
        }
        keyCodeToStr(keyCode) {
            return this._keyCodeToStr[keyCode];
        }
        strToKeyCode(str) {
            return this._strToKeyCode[str.toLowerCase()] || 0 /* Unknown */;
        }
    }
    const uiMap = new KeyCodeStrMap();
    const userSettingsUSMap = new KeyCodeStrMap();
    const userSettingsGeneralMap = new KeyCodeStrMap();
    const EVENT_KEY_CODE_MAP = new Array(230);
    const NATIVE_WINDOWS_KEY_CODE_TO_KEY_CODE = {};
    const scanCodeIntToStr = [];
    const scanCodeStrToInt = Object.create(null);
    const scanCodeLowerCaseStrToInt = Object.create(null);
    /**
     * -1 if a ScanCode => KeyCode mapping depends on kb layout.
     */
    const IMMUTABLE_CODE_TO_KEY_CODE = [];
    /**
     * -1 if a KeyCode => ScanCode mapping depends on kb layout.
     */
    const IMMUTABLE_KEY_CODE_TO_CODE = [];
    for (let i = 0; i <= 193 /* MAX_VALUE */; i++) {
        IMMUTABLE_CODE_TO_KEY_CODE[i] = -1 /* DependsOnKbLayout */;
    }
    for (let i = 0; i <= 126 /* MAX_VALUE */; i++) {
        IMMUTABLE_KEY_CODE_TO_CODE[i] = -1 /* DependsOnKbLayout */;
    }
    (function () {
        // See https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx
        // See https://github.com/microsoft/node-native-keymap/blob/master/deps/chromium/keyboard_codes_win.h
        const empty = '';
        const mappings = [
            // keyCodeOrd, immutable, scanCode, scanCodeStr, keyCode, keyCodeStr, eventKeyCode, vkey, usUserSettingsLabel, generalUserSettingsLabel
            [0, 1, 0 /* None */, 'None', 0 /* Unknown */, 'unknown', 0, 'VK_UNKNOWN', empty, empty],
            [0, 1, 1 /* Hyper */, 'Hyper', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 2 /* Super */, 'Super', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 3 /* Fn */, 'Fn', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 4 /* FnLock */, 'FnLock', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 5 /* Suspend */, 'Suspend', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 6 /* Resume */, 'Resume', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 7 /* Turbo */, 'Turbo', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 8 /* Sleep */, 'Sleep', 0 /* Unknown */, empty, 0, 'VK_SLEEP', empty, empty],
            [0, 1, 9 /* WakeUp */, 'WakeUp', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [31, 0, 10 /* KeyA */, 'KeyA', 31 /* KeyA */, 'A', 65, 'VK_A', empty, empty],
            [32, 0, 11 /* KeyB */, 'KeyB', 32 /* KeyB */, 'B', 66, 'VK_B', empty, empty],
            [33, 0, 12 /* KeyC */, 'KeyC', 33 /* KeyC */, 'C', 67, 'VK_C', empty, empty],
            [34, 0, 13 /* KeyD */, 'KeyD', 34 /* KeyD */, 'D', 68, 'VK_D', empty, empty],
            [35, 0, 14 /* KeyE */, 'KeyE', 35 /* KeyE */, 'E', 69, 'VK_E', empty, empty],
            [36, 0, 15 /* KeyF */, 'KeyF', 36 /* KeyF */, 'F', 70, 'VK_F', empty, empty],
            [37, 0, 16 /* KeyG */, 'KeyG', 37 /* KeyG */, 'G', 71, 'VK_G', empty, empty],
            [38, 0, 17 /* KeyH */, 'KeyH', 38 /* KeyH */, 'H', 72, 'VK_H', empty, empty],
            [39, 0, 18 /* KeyI */, 'KeyI', 39 /* KeyI */, 'I', 73, 'VK_I', empty, empty],
            [40, 0, 19 /* KeyJ */, 'KeyJ', 40 /* KeyJ */, 'J', 74, 'VK_J', empty, empty],
            [41, 0, 20 /* KeyK */, 'KeyK', 41 /* KeyK */, 'K', 75, 'VK_K', empty, empty],
            [42, 0, 21 /* KeyL */, 'KeyL', 42 /* KeyL */, 'L', 76, 'VK_L', empty, empty],
            [43, 0, 22 /* KeyM */, 'KeyM', 43 /* KeyM */, 'M', 77, 'VK_M', empty, empty],
            [44, 0, 23 /* KeyN */, 'KeyN', 44 /* KeyN */, 'N', 78, 'VK_N', empty, empty],
            [45, 0, 24 /* KeyO */, 'KeyO', 45 /* KeyO */, 'O', 79, 'VK_O', empty, empty],
            [46, 0, 25 /* KeyP */, 'KeyP', 46 /* KeyP */, 'P', 80, 'VK_P', empty, empty],
            [47, 0, 26 /* KeyQ */, 'KeyQ', 47 /* KeyQ */, 'Q', 81, 'VK_Q', empty, empty],
            [48, 0, 27 /* KeyR */, 'KeyR', 48 /* KeyR */, 'R', 82, 'VK_R', empty, empty],
            [49, 0, 28 /* KeyS */, 'KeyS', 49 /* KeyS */, 'S', 83, 'VK_S', empty, empty],
            [50, 0, 29 /* KeyT */, 'KeyT', 50 /* KeyT */, 'T', 84, 'VK_T', empty, empty],
            [51, 0, 30 /* KeyU */, 'KeyU', 51 /* KeyU */, 'U', 85, 'VK_U', empty, empty],
            [52, 0, 31 /* KeyV */, 'KeyV', 52 /* KeyV */, 'V', 86, 'VK_V', empty, empty],
            [53, 0, 32 /* KeyW */, 'KeyW', 53 /* KeyW */, 'W', 87, 'VK_W', empty, empty],
            [54, 0, 33 /* KeyX */, 'KeyX', 54 /* KeyX */, 'X', 88, 'VK_X', empty, empty],
            [55, 0, 34 /* KeyY */, 'KeyY', 55 /* KeyY */, 'Y', 89, 'VK_Y', empty, empty],
            [56, 0, 35 /* KeyZ */, 'KeyZ', 56 /* KeyZ */, 'Z', 90, 'VK_Z', empty, empty],
            [22, 0, 36 /* Digit1 */, 'Digit1', 22 /* Digit1 */, '1', 49, 'VK_1', empty, empty],
            [23, 0, 37 /* Digit2 */, 'Digit2', 23 /* Digit2 */, '2', 50, 'VK_2', empty, empty],
            [24, 0, 38 /* Digit3 */, 'Digit3', 24 /* Digit3 */, '3', 51, 'VK_3', empty, empty],
            [25, 0, 39 /* Digit4 */, 'Digit4', 25 /* Digit4 */, '4', 52, 'VK_4', empty, empty],
            [26, 0, 40 /* Digit5 */, 'Digit5', 26 /* Digit5 */, '5', 53, 'VK_5', empty, empty],
            [27, 0, 41 /* Digit6 */, 'Digit6', 27 /* Digit6 */, '6', 54, 'VK_6', empty, empty],
            [28, 0, 42 /* Digit7 */, 'Digit7', 28 /* Digit7 */, '7', 55, 'VK_7', empty, empty],
            [29, 0, 43 /* Digit8 */, 'Digit8', 29 /* Digit8 */, '8', 56, 'VK_8', empty, empty],
            [30, 0, 44 /* Digit9 */, 'Digit9', 30 /* Digit9 */, '9', 57, 'VK_9', empty, empty],
            [21, 0, 45 /* Digit0 */, 'Digit0', 21 /* Digit0 */, '0', 48, 'VK_0', empty, empty],
            [3, 1, 46 /* Enter */, 'Enter', 3 /* Enter */, 'Enter', 13, 'VK_RETURN', empty, empty],
            [9, 1, 47 /* Escape */, 'Escape', 9 /* Escape */, 'Escape', 27, 'VK_ESCAPE', empty, empty],
            [1, 1, 48 /* Backspace */, 'Backspace', 1 /* Backspace */, 'Backspace', 8, 'VK_BACK', empty, empty],
            [2, 1, 49 /* Tab */, 'Tab', 2 /* Tab */, 'Tab', 9, 'VK_TAB', empty, empty],
            [10, 1, 50 /* Space */, 'Space', 10 /* Space */, 'Space', 32, 'VK_SPACE', empty, empty],
            [83, 0, 51 /* Minus */, 'Minus', 83 /* Minus */, '-', 189, 'VK_OEM_MINUS', '-', 'OEM_MINUS'],
            [81, 0, 52 /* Equal */, 'Equal', 81 /* Equal */, '=', 187, 'VK_OEM_PLUS', '=', 'OEM_PLUS'],
            [87, 0, 53 /* BracketLeft */, 'BracketLeft', 87 /* BracketLeft */, '[', 219, 'VK_OEM_4', '[', 'OEM_4'],
            [89, 0, 54 /* BracketRight */, 'BracketRight', 89 /* BracketRight */, ']', 221, 'VK_OEM_6', ']', 'OEM_6'],
            [88, 0, 55 /* Backslash */, 'Backslash', 88 /* Backslash */, '\\', 220, 'VK_OEM_5', '\\', 'OEM_5'],
            [0, 0, 56 /* IntlHash */, 'IntlHash', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [80, 0, 57 /* Semicolon */, 'Semicolon', 80 /* Semicolon */, ';', 186, 'VK_OEM_1', ';', 'OEM_1'],
            [90, 0, 58 /* Quote */, 'Quote', 90 /* Quote */, '\'', 222, 'VK_OEM_7', '\'', 'OEM_7'],
            [86, 0, 59 /* Backquote */, 'Backquote', 86 /* Backquote */, '`', 192, 'VK_OEM_3', '`', 'OEM_3'],
            [82, 0, 60 /* Comma */, 'Comma', 82 /* Comma */, ',', 188, 'VK_OEM_COMMA', ',', 'OEM_COMMA'],
            [84, 0, 61 /* Period */, 'Period', 84 /* Period */, '.', 190, 'VK_OEM_PERIOD', '.', 'OEM_PERIOD'],
            [85, 0, 62 /* Slash */, 'Slash', 85 /* Slash */, '/', 191, 'VK_OEM_2', '/', 'OEM_2'],
            [8, 1, 63 /* CapsLock */, 'CapsLock', 8 /* CapsLock */, 'CapsLock', 20, 'VK_CAPITAL', empty, empty],
            [59, 1, 64 /* F1 */, 'F1', 59 /* F1 */, 'F1', 112, 'VK_F1', empty, empty],
            [60, 1, 65 /* F2 */, 'F2', 60 /* F2 */, 'F2', 113, 'VK_F2', empty, empty],
            [61, 1, 66 /* F3 */, 'F3', 61 /* F3 */, 'F3', 114, 'VK_F3', empty, empty],
            [62, 1, 67 /* F4 */, 'F4', 62 /* F4 */, 'F4', 115, 'VK_F4', empty, empty],
            [63, 1, 68 /* F5 */, 'F5', 63 /* F5 */, 'F5', 116, 'VK_F5', empty, empty],
            [64, 1, 69 /* F6 */, 'F6', 64 /* F6 */, 'F6', 117, 'VK_F6', empty, empty],
            [65, 1, 70 /* F7 */, 'F7', 65 /* F7 */, 'F7', 118, 'VK_F7', empty, empty],
            [66, 1, 71 /* F8 */, 'F8', 66 /* F8 */, 'F8', 119, 'VK_F8', empty, empty],
            [67, 1, 72 /* F9 */, 'F9', 67 /* F9 */, 'F9', 120, 'VK_F9', empty, empty],
            [68, 1, 73 /* F10 */, 'F10', 68 /* F10 */, 'F10', 121, 'VK_F10', empty, empty],
            [69, 1, 74 /* F11 */, 'F11', 69 /* F11 */, 'F11', 122, 'VK_F11', empty, empty],
            [70, 1, 75 /* F12 */, 'F12', 70 /* F12 */, 'F12', 123, 'VK_F12', empty, empty],
            [0, 1, 76 /* PrintScreen */, 'PrintScreen', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [79, 1, 77 /* ScrollLock */, 'ScrollLock', 79 /* ScrollLock */, 'ScrollLock', 145, 'VK_SCROLL', empty, empty],
            [7, 1, 78 /* Pause */, 'Pause', 7 /* PauseBreak */, 'PauseBreak', 19, 'VK_PAUSE', empty, empty],
            [19, 1, 79 /* Insert */, 'Insert', 19 /* Insert */, 'Insert', 45, 'VK_INSERT', empty, empty],
            [14, 1, 80 /* Home */, 'Home', 14 /* Home */, 'Home', 36, 'VK_HOME', empty, empty],
            [11, 1, 81 /* PageUp */, 'PageUp', 11 /* PageUp */, 'PageUp', 33, 'VK_PRIOR', empty, empty],
            [20, 1, 82 /* Delete */, 'Delete', 20 /* Delete */, 'Delete', 46, 'VK_DELETE', empty, empty],
            [13, 1, 83 /* End */, 'End', 13 /* End */, 'End', 35, 'VK_END', empty, empty],
            [12, 1, 84 /* PageDown */, 'PageDown', 12 /* PageDown */, 'PageDown', 34, 'VK_NEXT', empty, empty],
            [17, 1, 85 /* ArrowRight */, 'ArrowRight', 17 /* RightArrow */, 'RightArrow', 39, 'VK_RIGHT', 'Right', empty],
            [15, 1, 86 /* ArrowLeft */, 'ArrowLeft', 15 /* LeftArrow */, 'LeftArrow', 37, 'VK_LEFT', 'Left', empty],
            [18, 1, 87 /* ArrowDown */, 'ArrowDown', 18 /* DownArrow */, 'DownArrow', 40, 'VK_DOWN', 'Down', empty],
            [16, 1, 88 /* ArrowUp */, 'ArrowUp', 16 /* UpArrow */, 'UpArrow', 38, 'VK_UP', 'Up', empty],
            [78, 1, 89 /* NumLock */, 'NumLock', 78 /* NumLock */, 'NumLock', 144, 'VK_NUMLOCK', empty, empty],
            [108, 1, 90 /* NumpadDivide */, 'NumpadDivide', 108 /* NumpadDivide */, 'NumPad_Divide', 111, 'VK_DIVIDE', empty, empty],
            [103, 1, 91 /* NumpadMultiply */, 'NumpadMultiply', 103 /* NumpadMultiply */, 'NumPad_Multiply', 106, 'VK_MULTIPLY', empty, empty],
            [106, 1, 92 /* NumpadSubtract */, 'NumpadSubtract', 106 /* NumpadSubtract */, 'NumPad_Subtract', 109, 'VK_SUBTRACT', empty, empty],
            [104, 1, 93 /* NumpadAdd */, 'NumpadAdd', 104 /* NumpadAdd */, 'NumPad_Add', 107, 'VK_ADD', empty, empty],
            [3, 1, 94 /* NumpadEnter */, 'NumpadEnter', 3 /* Enter */, empty, 0, empty, empty, empty],
            [94, 1, 95 /* Numpad1 */, 'Numpad1', 94 /* Numpad1 */, 'NumPad1', 97, 'VK_NUMPAD1', empty, empty],
            [95, 1, 96 /* Numpad2 */, 'Numpad2', 95 /* Numpad2 */, 'NumPad2', 98, 'VK_NUMPAD2', empty, empty],
            [96, 1, 97 /* Numpad3 */, 'Numpad3', 96 /* Numpad3 */, 'NumPad3', 99, 'VK_NUMPAD3', empty, empty],
            [97, 1, 98 /* Numpad4 */, 'Numpad4', 97 /* Numpad4 */, 'NumPad4', 100, 'VK_NUMPAD4', empty, empty],
            [98, 1, 99 /* Numpad5 */, 'Numpad5', 98 /* Numpad5 */, 'NumPad5', 101, 'VK_NUMPAD5', empty, empty],
            [99, 1, 100 /* Numpad6 */, 'Numpad6', 99 /* Numpad6 */, 'NumPad6', 102, 'VK_NUMPAD6', empty, empty],
            [100, 1, 101 /* Numpad7 */, 'Numpad7', 100 /* Numpad7 */, 'NumPad7', 103, 'VK_NUMPAD7', empty, empty],
            [101, 1, 102 /* Numpad8 */, 'Numpad8', 101 /* Numpad8 */, 'NumPad8', 104, 'VK_NUMPAD8', empty, empty],
            [102, 1, 103 /* Numpad9 */, 'Numpad9', 102 /* Numpad9 */, 'NumPad9', 105, 'VK_NUMPAD9', empty, empty],
            [93, 1, 104 /* Numpad0 */, 'Numpad0', 93 /* Numpad0 */, 'NumPad0', 96, 'VK_NUMPAD0', empty, empty],
            [107, 1, 105 /* NumpadDecimal */, 'NumpadDecimal', 107 /* NumpadDecimal */, 'NumPad_Decimal', 110, 'VK_DECIMAL', empty, empty],
            [92, 0, 106 /* IntlBackslash */, 'IntlBackslash', 92 /* IntlBackslash */, 'OEM_102', 226, 'VK_OEM_102', empty, empty],
            [58, 1, 107 /* ContextMenu */, 'ContextMenu', 58 /* ContextMenu */, 'ContextMenu', 93, empty, empty, empty],
            [0, 1, 108 /* Power */, 'Power', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 109 /* NumpadEqual */, 'NumpadEqual', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [71, 1, 110 /* F13 */, 'F13', 71 /* F13 */, 'F13', 124, 'VK_F13', empty, empty],
            [72, 1, 111 /* F14 */, 'F14', 72 /* F14 */, 'F14', 125, 'VK_F14', empty, empty],
            [73, 1, 112 /* F15 */, 'F15', 73 /* F15 */, 'F15', 126, 'VK_F15', empty, empty],
            [74, 1, 113 /* F16 */, 'F16', 74 /* F16 */, 'F16', 127, 'VK_F16', empty, empty],
            [75, 1, 114 /* F17 */, 'F17', 75 /* F17 */, 'F17', 128, 'VK_F17', empty, empty],
            [76, 1, 115 /* F18 */, 'F18', 76 /* F18 */, 'F18', 129, 'VK_F18', empty, empty],
            [77, 1, 116 /* F19 */, 'F19', 77 /* F19 */, 'F19', 130, 'VK_F19', empty, empty],
            [0, 1, 117 /* F20 */, 'F20', 0 /* Unknown */, empty, 0, 'VK_F20', empty, empty],
            [0, 1, 118 /* F21 */, 'F21', 0 /* Unknown */, empty, 0, 'VK_F21', empty, empty],
            [0, 1, 119 /* F22 */, 'F22', 0 /* Unknown */, empty, 0, 'VK_F22', empty, empty],
            [0, 1, 120 /* F23 */, 'F23', 0 /* Unknown */, empty, 0, 'VK_F23', empty, empty],
            [0, 1, 121 /* F24 */, 'F24', 0 /* Unknown */, empty, 0, 'VK_F24', empty, empty],
            [0, 1, 122 /* Open */, 'Open', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 123 /* Help */, 'Help', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 124 /* Select */, 'Select', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 125 /* Again */, 'Again', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 126 /* Undo */, 'Undo', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 127 /* Cut */, 'Cut', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 128 /* Copy */, 'Copy', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 129 /* Paste */, 'Paste', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 130 /* Find */, 'Find', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 131 /* AudioVolumeMute */, 'AudioVolumeMute', 112 /* AudioVolumeMute */, 'AudioVolumeMute', 173, 'VK_VOLUME_MUTE', empty, empty],
            [0, 1, 132 /* AudioVolumeUp */, 'AudioVolumeUp', 113 /* AudioVolumeUp */, 'AudioVolumeUp', 175, 'VK_VOLUME_UP', empty, empty],
            [0, 1, 133 /* AudioVolumeDown */, 'AudioVolumeDown', 114 /* AudioVolumeDown */, 'AudioVolumeDown', 174, 'VK_VOLUME_DOWN', empty, empty],
            [105, 1, 134 /* NumpadComma */, 'NumpadComma', 105 /* NUMPAD_SEPARATOR */, 'NumPad_Separator', 108, 'VK_SEPARATOR', empty, empty],
            [110, 0, 135 /* IntlRo */, 'IntlRo', 110 /* ABNT_C1 */, 'ABNT_C1', 193, 'VK_ABNT_C1', empty, empty],
            [0, 1, 136 /* KanaMode */, 'KanaMode', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 0, 137 /* IntlYen */, 'IntlYen', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 138 /* Convert */, 'Convert', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 139 /* NonConvert */, 'NonConvert', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 140 /* Lang1 */, 'Lang1', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 141 /* Lang2 */, 'Lang2', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 142 /* Lang3 */, 'Lang3', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 143 /* Lang4 */, 'Lang4', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 144 /* Lang5 */, 'Lang5', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 145 /* Abort */, 'Abort', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 146 /* Props */, 'Props', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 147 /* NumpadParenLeft */, 'NumpadParenLeft', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 148 /* NumpadParenRight */, 'NumpadParenRight', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 149 /* NumpadBackspace */, 'NumpadBackspace', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 150 /* NumpadMemoryStore */, 'NumpadMemoryStore', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 151 /* NumpadMemoryRecall */, 'NumpadMemoryRecall', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 152 /* NumpadMemoryClear */, 'NumpadMemoryClear', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 153 /* NumpadMemoryAdd */, 'NumpadMemoryAdd', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 154 /* NumpadMemorySubtract */, 'NumpadMemorySubtract', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 155 /* NumpadClear */, 'NumpadClear', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 156 /* NumpadClearEntry */, 'NumpadClearEntry', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [5, 1, 0 /* None */, empty, 5 /* Ctrl */, 'Ctrl', 17, 'VK_CONTROL', empty, empty],
            [4, 1, 0 /* None */, empty, 4 /* Shift */, 'Shift', 16, 'VK_SHIFT', empty, empty],
            [6, 1, 0 /* None */, empty, 6 /* Alt */, 'Alt', 18, 'VK_MENU', empty, empty],
            [57, 1, 0 /* None */, empty, 57 /* Meta */, 'Meta', 0, 'VK_COMMAND', empty, empty],
            [5, 1, 157 /* ControlLeft */, 'ControlLeft', 5 /* Ctrl */, empty, 0, 'VK_LCONTROL', empty, empty],
            [4, 1, 158 /* ShiftLeft */, 'ShiftLeft', 4 /* Shift */, empty, 0, 'VK_LSHIFT', empty, empty],
            [6, 1, 159 /* AltLeft */, 'AltLeft', 6 /* Alt */, empty, 0, 'VK_LMENU', empty, empty],
            [57, 1, 160 /* MetaLeft */, 'MetaLeft', 57 /* Meta */, empty, 0, 'VK_LWIN', empty, empty],
            [5, 1, 161 /* ControlRight */, 'ControlRight', 5 /* Ctrl */, empty, 0, 'VK_RCONTROL', empty, empty],
            [4, 1, 162 /* ShiftRight */, 'ShiftRight', 4 /* Shift */, empty, 0, 'VK_RSHIFT', empty, empty],
            [6, 1, 163 /* AltRight */, 'AltRight', 6 /* Alt */, empty, 0, 'VK_RMENU', empty, empty],
            [57, 1, 164 /* MetaRight */, 'MetaRight', 57 /* Meta */, empty, 0, 'VK_RWIN', empty, empty],
            [0, 1, 165 /* BrightnessUp */, 'BrightnessUp', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 166 /* BrightnessDown */, 'BrightnessDown', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 167 /* MediaPlay */, 'MediaPlay', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 168 /* MediaRecord */, 'MediaRecord', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 169 /* MediaFastForward */, 'MediaFastForward', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 170 /* MediaRewind */, 'MediaRewind', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [114, 1, 171 /* MediaTrackNext */, 'MediaTrackNext', 119 /* MediaTrackNext */, 'MediaTrackNext', 176, 'VK_MEDIA_NEXT_TRACK', empty, empty],
            [115, 1, 172 /* MediaTrackPrevious */, 'MediaTrackPrevious', 120 /* MediaTrackPrevious */, 'MediaTrackPrevious', 177, 'VK_MEDIA_PREV_TRACK', empty, empty],
            [116, 1, 173 /* MediaStop */, 'MediaStop', 121 /* MediaStop */, 'MediaStop', 178, 'VK_MEDIA_STOP', empty, empty],
            [0, 1, 174 /* Eject */, 'Eject', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [117, 1, 175 /* MediaPlayPause */, 'MediaPlayPause', 122 /* MediaPlayPause */, 'MediaPlayPause', 179, 'VK_MEDIA_PLAY_PAUSE', empty, empty],
            [0, 1, 176 /* MediaSelect */, 'MediaSelect', 123 /* LaunchMediaPlayer */, 'LaunchMediaPlayer', 181, 'VK_MEDIA_LAUNCH_MEDIA_SELECT', empty, empty],
            [0, 1, 177 /* LaunchMail */, 'LaunchMail', 124 /* LaunchMail */, 'LaunchMail', 180, 'VK_MEDIA_LAUNCH_MAIL', empty, empty],
            [0, 1, 178 /* LaunchApp2 */, 'LaunchApp2', 125 /* LaunchApp2 */, 'LaunchApp2', 183, 'VK_MEDIA_LAUNCH_APP2', empty, empty],
            [0, 1, 179 /* LaunchApp1 */, 'LaunchApp1', 0 /* Unknown */, empty, 0, 'VK_MEDIA_LAUNCH_APP1', empty, empty],
            [0, 1, 180 /* SelectTask */, 'SelectTask', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 181 /* LaunchScreenSaver */, 'LaunchScreenSaver', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 182 /* BrowserSearch */, 'BrowserSearch', 115 /* BrowserSearch */, 'BrowserSearch', 170, 'VK_BROWSER_SEARCH', empty, empty],
            [0, 1, 183 /* BrowserHome */, 'BrowserHome', 116 /* BrowserHome */, 'BrowserHome', 172, 'VK_BROWSER_HOME', empty, empty],
            [112, 1, 184 /* BrowserBack */, 'BrowserBack', 117 /* BrowserBack */, 'BrowserBack', 166, 'VK_BROWSER_BACK', empty, empty],
            [113, 1, 185 /* BrowserForward */, 'BrowserForward', 118 /* BrowserForward */, 'BrowserForward', 167, 'VK_BROWSER_FORWARD', empty, empty],
            [0, 1, 186 /* BrowserStop */, 'BrowserStop', 0 /* Unknown */, empty, 0, 'VK_BROWSER_STOP', empty, empty],
            [0, 1, 187 /* BrowserRefresh */, 'BrowserRefresh', 0 /* Unknown */, empty, 0, 'VK_BROWSER_REFRESH', empty, empty],
            [0, 1, 188 /* BrowserFavorites */, 'BrowserFavorites', 0 /* Unknown */, empty, 0, 'VK_BROWSER_FAVORITES', empty, empty],
            [0, 1, 189 /* ZoomToggle */, 'ZoomToggle', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 190 /* MailReply */, 'MailReply', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 191 /* MailForward */, 'MailForward', 0 /* Unknown */, empty, 0, empty, empty, empty],
            [0, 1, 192 /* MailSend */, 'MailSend', 0 /* Unknown */, empty, 0, empty, empty, empty],
            // See https://lists.w3.org/Archives/Public/www-dom/2010JulSep/att-0182/keyCode-spec.html
            // If an Input Method Editor is processing key input and the event is keydown, return 229.
            [109, 1, 0 /* None */, empty, 109 /* KEY_IN_COMPOSITION */, 'KeyInComposition', 229, empty, empty, empty],
            [111, 1, 0 /* None */, empty, 111 /* ABNT_C2 */, 'ABNT_C2', 194, 'VK_ABNT_C2', empty, empty],
            [91, 1, 0 /* None */, empty, 91 /* OEM_8 */, 'OEM_8', 223, 'VK_OEM_8', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_CLEAR', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_KANA', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_HANGUL', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_JUNJA', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_FINAL', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_HANJA', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_KANJI', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_CONVERT', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_NONCONVERT', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_ACCEPT', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_MODECHANGE', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_SELECT', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_PRINT', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_EXECUTE', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_SNAPSHOT', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_HELP', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_APPS', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_PROCESSKEY', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_PACKET', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_DBE_SBCSCHAR', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_DBE_DBCSCHAR', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_ATTN', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_CRSEL', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_EXSEL', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_EREOF', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_PLAY', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_ZOOM', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_NONAME', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_PA1', empty, empty],
            [0, 1, 0 /* None */, empty, 0 /* Unknown */, empty, 0, 'VK_OEM_CLEAR', empty, empty],
        ];
        let seenKeyCode = [];
        let seenScanCode = [];
        for (const mapping of mappings) {
            const [_keyCodeOrd, immutable, scanCode, scanCodeStr, keyCode, keyCodeStr, eventKeyCode, vkey, usUserSettingsLabel, generalUserSettingsLabel] = mapping;
            if (!seenScanCode[scanCode]) {
                seenScanCode[scanCode] = true;
                scanCodeIntToStr[scanCode] = scanCodeStr;
                scanCodeStrToInt[scanCodeStr] = scanCode;
                scanCodeLowerCaseStrToInt[scanCodeStr.toLowerCase()] = scanCode;
                if (immutable) {
                    IMMUTABLE_CODE_TO_KEY_CODE[scanCode] = keyCode;
                    if ((keyCode !== 0 /* Unknown */)
                        && (keyCode !== 3 /* Enter */)
                        && (keyCode !== 5 /* Ctrl */)
                        && (keyCode !== 4 /* Shift */)
                        && (keyCode !== 6 /* Alt */)
                        && (keyCode !== 57 /* Meta */)) {
                        IMMUTABLE_KEY_CODE_TO_CODE[keyCode] = scanCode;
                    }
                }
            }
            if (!seenKeyCode[keyCode]) {
                seenKeyCode[keyCode] = true;
                if (!keyCodeStr) {
                    throw new Error(`String representation missing for key code ${keyCode} around scan code ${scanCodeStr}`);
                }
                uiMap.define(keyCode, keyCodeStr);
                userSettingsUSMap.define(keyCode, usUserSettingsLabel || keyCodeStr);
                userSettingsGeneralMap.define(keyCode, generalUserSettingsLabel || usUserSettingsLabel || keyCodeStr);
            }
            if (eventKeyCode) {
                EVENT_KEY_CODE_MAP[eventKeyCode] = keyCode;
            }
            if (vkey) {
                NATIVE_WINDOWS_KEY_CODE_TO_KEY_CODE[vkey] = keyCode;
            }
        }
        // Manually added due to the exclusion above (due to duplication with NumpadEnter)
        IMMUTABLE_KEY_CODE_TO_CODE[3 /* Enter */] = 46 /* Enter */;
    })();
    var KeyCodeUtils;
    (function (KeyCodeUtils) {
        function toString(keyCode) {
            return uiMap.keyCodeToStr(keyCode);
        }
        KeyCodeUtils.toString = toString;
        function fromString(key) {
            return uiMap.strToKeyCode(key);
        }
        KeyCodeUtils.fromString = fromString;
        function toUserSettingsUS(keyCode) {
            return userSettingsUSMap.keyCodeToStr(keyCode);
        }
        KeyCodeUtils.toUserSettingsUS = toUserSettingsUS;
        function toUserSettingsGeneral(keyCode) {
            return userSettingsGeneralMap.keyCodeToStr(keyCode);
        }
        KeyCodeUtils.toUserSettingsGeneral = toUserSettingsGeneral;
        function fromUserSettings(key) {
            return userSettingsUSMap.strToKeyCode(key) || userSettingsGeneralMap.strToKeyCode(key);
        }
        KeyCodeUtils.fromUserSettings = fromUserSettings;
        function toElectronAccelerator(keyCode) {
            if (keyCode >= 93 /* Numpad0 */ && keyCode <= 108 /* NumpadDivide */) {
                // [Electron Accelerators] Electron is able to parse numpad keys, but unfortunately it
                // renders them just as regular keys in menus. For example, num0 is rendered as "0",
                // numdiv is rendered as "/", numsub is rendered as "-".
                //
                // This can lead to incredible confusion, as it makes numpad based keybindings indistinguishable
                // from keybindings based on regular keys.
                //
                // We therefore need to fall back to custom rendering for numpad keys.
                return null;
            }
            switch (keyCode) {
                case 16 /* UpArrow */:
                    return 'Up';
                case 18 /* DownArrow */:
                    return 'Down';
                case 15 /* LeftArrow */:
                    return 'Left';
                case 17 /* RightArrow */:
                    return 'Right';
            }
            return uiMap.keyCodeToStr(keyCode);
        }
        KeyCodeUtils.toElectronAccelerator = toElectronAccelerator;
    })(KeyCodeUtils || (KeyCodeUtils = {}));
    function KeyChord(firstPart, secondPart) {
        const chordPart = ((secondPart & 0x0000FFFF) << 16) >>> 0;
        return (firstPart | chordPart) >>> 0;
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    /**
     * A selection in the editor.
     * The selection is a range that has an orientation.
     */
    class Selection extends Range {
        constructor(selectionStartLineNumber, selectionStartColumn, positionLineNumber, positionColumn) {
            super(selectionStartLineNumber, selectionStartColumn, positionLineNumber, positionColumn);
            this.selectionStartLineNumber = selectionStartLineNumber;
            this.selectionStartColumn = selectionStartColumn;
            this.positionLineNumber = positionLineNumber;
            this.positionColumn = positionColumn;
        }
        /**
         * Transform to a human-readable representation.
         */
        toString() {
            return '[' + this.selectionStartLineNumber + ',' + this.selectionStartColumn + ' -> ' + this.positionLineNumber + ',' + this.positionColumn + ']';
        }
        /**
         * Test if equals other selection.
         */
        equalsSelection(other) {
            return (Selection.selectionsEqual(this, other));
        }
        /**
         * Test if the two selections are equal.
         */
        static selectionsEqual(a, b) {
            return (a.selectionStartLineNumber === b.selectionStartLineNumber &&
                a.selectionStartColumn === b.selectionStartColumn &&
                a.positionLineNumber === b.positionLineNumber &&
                a.positionColumn === b.positionColumn);
        }
        /**
         * Get directions (LTR or RTL).
         */
        getDirection() {
            if (this.selectionStartLineNumber === this.startLineNumber && this.selectionStartColumn === this.startColumn) {
                return 0 /* LTR */;
            }
            return 1 /* RTL */;
        }
        /**
         * Create a new selection with a different `positionLineNumber` and `positionColumn`.
         */
        setEndPosition(endLineNumber, endColumn) {
            if (this.getDirection() === 0 /* LTR */) {
                return new Selection(this.startLineNumber, this.startColumn, endLineNumber, endColumn);
            }
            return new Selection(endLineNumber, endColumn, this.startLineNumber, this.startColumn);
        }
        /**
         * Get the position at `positionLineNumber` and `positionColumn`.
         */
        getPosition() {
            return new Position(this.positionLineNumber, this.positionColumn);
        }
        /**
         * Get the position at the start of the selection.
        */
        getSelectionStart() {
            return new Position(this.selectionStartLineNumber, this.selectionStartColumn);
        }
        /**
         * Create a new selection with a different `selectionStartLineNumber` and `selectionStartColumn`.
         */
        setStartPosition(startLineNumber, startColumn) {
            if (this.getDirection() === 0 /* LTR */) {
                return new Selection(startLineNumber, startColumn, this.endLineNumber, this.endColumn);
            }
            return new Selection(this.endLineNumber, this.endColumn, startLineNumber, startColumn);
        }
        // ----
        /**
         * Create a `Selection` from one or two positions
         */
        static fromPositions(start, end = start) {
            return new Selection(start.lineNumber, start.column, end.lineNumber, end.column);
        }
        /**
         * Creates a `Selection` from a range, given a direction.
         */
        static fromRange(range, direction) {
            if (direction === 0 /* LTR */) {
                return new Selection(range.startLineNumber, range.startColumn, range.endLineNumber, range.endColumn);
            }
            else {
                return new Selection(range.endLineNumber, range.endColumn, range.startLineNumber, range.startColumn);
            }
        }
        /**
         * Create a `Selection` from an `ISelection`.
         */
        static liftSelection(sel) {
            return new Selection(sel.selectionStartLineNumber, sel.selectionStartColumn, sel.positionLineNumber, sel.positionColumn);
        }
        /**
         * `a` equals `b`.
         */
        static selectionsArrEqual(a, b) {
            if (a && !b || !a && b) {
                return false;
            }
            if (!a && !b) {
                return true;
            }
            if (a.length !== b.length) {
                return false;
            }
            for (let i = 0, len = a.length; i < len; i++) {
                if (!this.selectionsEqual(a[i], b[i])) {
                    return false;
                }
            }
            return true;
        }
        /**
         * Test if `obj` is an `ISelection`.
         */
        static isISelection(obj) {
            return (obj
                && (typeof obj.selectionStartLineNumber === 'number')
                && (typeof obj.selectionStartColumn === 'number')
                && (typeof obj.positionLineNumber === 'number')
                && (typeof obj.positionColumn === 'number'));
        }
        /**
         * Create with a direction.
         */
        static createWithDirection(startLineNumber, startColumn, endLineNumber, endColumn, direction) {
            if (direction === 0 /* LTR */) {
                return new Selection(startLineNumber, startColumn, endLineNumber, endColumn);
            }
            return new Selection(endLineNumber, endColumn, startLineNumber, startColumn);
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    /**
     * Vertical Lane in the overview ruler of the editor.
     */
    var OverviewRulerLane$1;
    (function (OverviewRulerLane) {
        OverviewRulerLane[OverviewRulerLane["Left"] = 1] = "Left";
        OverviewRulerLane[OverviewRulerLane["Center"] = 2] = "Center";
        OverviewRulerLane[OverviewRulerLane["Right"] = 4] = "Right";
        OverviewRulerLane[OverviewRulerLane["Full"] = 7] = "Full";
    })(OverviewRulerLane$1 || (OverviewRulerLane$1 = {}));
    /**
     * Position in the minimap to render the decoration.
     */
    var MinimapPosition$1;
    (function (MinimapPosition) {
        MinimapPosition[MinimapPosition["Inline"] = 1] = "Inline";
        MinimapPosition[MinimapPosition["Gutter"] = 2] = "Gutter";
    })(MinimapPosition$1 || (MinimapPosition$1 = {}));
    var InjectedTextCursorStops$1;
    (function (InjectedTextCursorStops) {
        InjectedTextCursorStops[InjectedTextCursorStops["Both"] = 0] = "Both";
        InjectedTextCursorStops[InjectedTextCursorStops["Right"] = 1] = "Right";
        InjectedTextCursorStops[InjectedTextCursorStops["Left"] = 2] = "Left";
        InjectedTextCursorStops[InjectedTextCursorStops["None"] = 3] = "None";
    })(InjectedTextCursorStops$1 || (InjectedTextCursorStops$1 = {}));
    /**
     * @internal
     */
    function shouldSynchronizeModel(model) {
        return (!model.isTooLargeForSyncing() && !model.isForSimpleWidget);
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    var __awaiter$2 = (undefined && undefined.__awaiter) || function (thisArg, _arguments, P, generator) {
        function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
        return new (P || (P = Promise))(function (resolve, reject) {
            function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
            function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
            function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
            step((generator = generator.apply(thisArg, _arguments || [])).next());
        });
    };
    var __asyncValues = (undefined && undefined.__asyncValues) || function (o) {
        if (!Symbol.asyncIterator) throw new TypeError("Symbol.asyncIterator is not defined.");
        var m = o[Symbol.asyncIterator], i;
        return m ? m.call(o) : (o = typeof __values === "function" ? __values(o) : o[Symbol.iterator](), i = {}, verb("next"), verb("throw"), verb("return"), i[Symbol.asyncIterator] = function () { return this; }, i);
        function verb(n) { i[n] = o[n] && function (v) { return new Promise(function (resolve, reject) { v = o[n](v), settle(resolve, reject, v.done, v.value); }); }; }
        function settle(resolve, reject, d, v) { Promise.resolve(v).then(function(v) { resolve({ value: v, done: d }); }, reject); }
    };
    function isThenable(obj) {
        return !!obj && typeof obj.then === 'function';
    }
    //#endregion
    //#region Promises
    var Promises;
    (function (Promises) {
        /**
         * A drop-in replacement for `Promise.all` with the only difference
         * that the method awaits every promise to either fulfill or reject.
         *
         * Similar to `Promise.all`, only the first error will be returned
         * if any.
         */
        function settled(promises) {
            return __awaiter$2(this, void 0, void 0, function* () {
                let firstError = undefined;
                const result = yield Promise.all(promises.map(promise => promise.then(value => value, error => {
                    if (!firstError) {
                        firstError = error;
                    }
                    return undefined; // do not rethrow so that other promises can settle
                })));
                if (typeof firstError !== 'undefined') {
                    throw firstError;
                }
                return result; // cast is needed and protected by the `throw` above
            });
        }
        Promises.settled = settled;
        /**
         * A helper to create a new `Promise<T>` with a body that is a promise
         * itself. By default, an error that raises from the async body will
         * end up as a unhandled rejection, so this utility properly awaits the
         * body and rejects the promise as a normal promise does without async
         * body.
         *
         * This method should only be used in rare cases where otherwise `async`
         * cannot be used (e.g. when callbacks are involved that require this).
         */
        function withAsyncBody(bodyFn) {
            // eslint-disable-next-line no-async-promise-executor
            return new Promise((resolve, reject) => __awaiter$2(this, void 0, void 0, function* () {
                try {
                    yield bodyFn(resolve, reject);
                }
                catch (error) {
                    reject(error);
                }
            }));
        }
        Promises.withAsyncBody = withAsyncBody;
    })(Promises || (Promises = {}));
    /**
     * A rich implementation for an `AsyncIterable<T>`.
     */
    class AsyncIterableObject {
        constructor(executor) {
            this._state = 0 /* Initial */;
            this._results = [];
            this._error = null;
            this._onStateChanged = new Emitter();
            queueMicrotask(() => __awaiter$2(this, void 0, void 0, function* () {
                const writer = {
                    emitOne: (item) => this.emitOne(item),
                    emitMany: (items) => this.emitMany(items),
                    reject: (error) => this.reject(error)
                };
                try {
                    yield Promise.resolve(executor(writer));
                    this.resolve();
                }
                catch (err) {
                    this.reject(err);
                }
                finally {
                    writer.emitOne = undefined;
                    writer.emitMany = undefined;
                    writer.reject = undefined;
                }
            }));
        }
        static fromArray(items) {
            return new AsyncIterableObject((writer) => {
                writer.emitMany(items);
            });
        }
        static fromPromise(promise) {
            return new AsyncIterableObject((emitter) => __awaiter$2(this, void 0, void 0, function* () {
                emitter.emitMany(yield promise);
            }));
        }
        static fromPromises(promises) {
            return new AsyncIterableObject((emitter) => __awaiter$2(this, void 0, void 0, function* () {
                yield Promise.all(promises.map((p) => __awaiter$2(this, void 0, void 0, function* () { return emitter.emitOne(yield p); })));
            }));
        }
        static merge(iterables) {
            return new AsyncIterableObject((emitter) => __awaiter$2(this, void 0, void 0, function* () {
                yield Promise.all(iterables.map((iterable) => { var iterable_1, iterable_1_1; return __awaiter$2(this, void 0, void 0, function* () {
                    var e_1, _a;
                    try {
                        for (iterable_1 = __asyncValues(iterable); iterable_1_1 = yield iterable_1.next(), !iterable_1_1.done;) {
                            const item = iterable_1_1.value;
                            emitter.emitOne(item);
                        }
                    }
                    catch (e_1_1) { e_1 = { error: e_1_1 }; }
                    finally {
                        try {
                            if (iterable_1_1 && !iterable_1_1.done && (_a = iterable_1.return)) yield _a.call(iterable_1);
                        }
                        finally { if (e_1) throw e_1.error; }
                    }
                }); }));
            }));
        }
        [Symbol.asyncIterator]() {
            let i = 0;
            return {
                next: () => __awaiter$2(this, void 0, void 0, function* () {
                    do {
                        if (this._state === 2 /* DoneError */) {
                            throw this._error;
                        }
                        if (i < this._results.length) {
                            return { done: false, value: this._results[i++] };
                        }
                        if (this._state === 1 /* DoneOK */) {
                            return { done: true, value: undefined };
                        }
                        yield Event.toPromise(this._onStateChanged.event);
                    } while (true);
                })
            };
        }
        static map(iterable, mapFn) {
            return new AsyncIterableObject((emitter) => __awaiter$2(this, void 0, void 0, function* () {
                var e_2, _a;
                try {
                    for (var iterable_2 = __asyncValues(iterable), iterable_2_1; iterable_2_1 = yield iterable_2.next(), !iterable_2_1.done;) {
                        const item = iterable_2_1.value;
                        emitter.emitOne(mapFn(item));
                    }
                }
                catch (e_2_1) { e_2 = { error: e_2_1 }; }
                finally {
                    try {
                        if (iterable_2_1 && !iterable_2_1.done && (_a = iterable_2.return)) yield _a.call(iterable_2);
                    }
                    finally { if (e_2) throw e_2.error; }
                }
            }));
        }
        map(mapFn) {
            return AsyncIterableObject.map(this, mapFn);
        }
        static filter(iterable, filterFn) {
            return new AsyncIterableObject((emitter) => __awaiter$2(this, void 0, void 0, function* () {
                var e_3, _a;
                try {
                    for (var iterable_3 = __asyncValues(iterable), iterable_3_1; iterable_3_1 = yield iterable_3.next(), !iterable_3_1.done;) {
                        const item = iterable_3_1.value;
                        if (filterFn(item)) {
                            emitter.emitOne(item);
                        }
                    }
                }
                catch (e_3_1) { e_3 = { error: e_3_1 }; }
                finally {
                    try {
                        if (iterable_3_1 && !iterable_3_1.done && (_a = iterable_3.return)) yield _a.call(iterable_3);
                    }
                    finally { if (e_3) throw e_3.error; }
                }
            }));
        }
        filter(filterFn) {
            return AsyncIterableObject.filter(this, filterFn);
        }
        static coalesce(iterable) {
            return AsyncIterableObject.filter(iterable, item => !!item);
        }
        coalesce() {
            return AsyncIterableObject.coalesce(this);
        }
        static toPromise(iterable) {
            var iterable_4, iterable_4_1;
            var e_4, _a;
            return __awaiter$2(this, void 0, void 0, function* () {
                const result = [];
                try {
                    for (iterable_4 = __asyncValues(iterable); iterable_4_1 = yield iterable_4.next(), !iterable_4_1.done;) {
                        const item = iterable_4_1.value;
                        result.push(item);
                    }
                }
                catch (e_4_1) { e_4 = { error: e_4_1 }; }
                finally {
                    try {
                        if (iterable_4_1 && !iterable_4_1.done && (_a = iterable_4.return)) yield _a.call(iterable_4);
                    }
                    finally { if (e_4) throw e_4.error; }
                }
                return result;
            });
        }
        toPromise() {
            return AsyncIterableObject.toPromise(this);
        }
        /**
         * The value will be appended at the end.
         *
         * **NOTE** If `resolve()` or `reject()` have already been called, this method has no effect.
         */
        emitOne(value) {
            if (this._state !== 0 /* Initial */) {
                return;
            }
            // it is important to add new values at the end,
            // as we may have iterators already running on the array
            this._results.push(value);
            this._onStateChanged.fire();
        }
        /**
         * The values will be appended at the end.
         *
         * **NOTE** If `resolve()` or `reject()` have already been called, this method has no effect.
         */
        emitMany(values) {
            if (this._state !== 0 /* Initial */) {
                return;
            }
            // it is important to add new values at the end,
            // as we may have iterators already running on the array
            this._results = this._results.concat(values);
            this._onStateChanged.fire();
        }
        /**
         * Calling `resolve()` will mark the result array as complete.
         *
         * **NOTE** `resolve()` must be called, otherwise all consumers of this iterable will hang indefinitely, similar to a non-resolved promise.
         * **NOTE** If `resolve()` or `reject()` have already been called, this method has no effect.
         */
        resolve() {
            if (this._state !== 0 /* Initial */) {
                return;
            }
            this._state = 1 /* DoneOK */;
            this._onStateChanged.fire();
        }
        /**
         * Writing an error will permanently invalidate this iterable.
         * The current users will receive an error thrown, as will all future users.
         *
         * **NOTE** If `resolve()` or `reject()` have already been called, this method has no effect.
         */
        reject(error) {
            if (this._state !== 0 /* Initial */) {
                return;
            }
            this._state = 2 /* DoneError */;
            this._error = error;
            this._onStateChanged.fire();
        }
    }
    AsyncIterableObject.EMPTY = AsyncIterableObject.fromArray([]);
    //#endregion

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    /**
     * @deprecated please use `IUriIdentityService.extUri.isEqualOrParent` instead. If
     * you are in a context without services, consider to pass down the `extUri` from the
     * outside, or use `extUriBiasedIgnorePathCase` if you know what you are doing.
     */
    function isEqualOrParent(base, parentCandidate, ignoreCase, separator = sep) {
        if (base === parentCandidate) {
            return true;
        }
        if (!base || !parentCandidate) {
            return false;
        }
        if (parentCandidate.length > base.length) {
            return false;
        }
        if (ignoreCase) {
            const beginsWith = startsWithIgnoreCase(base, parentCandidate);
            if (!beginsWith) {
                return false;
            }
            if (parentCandidate.length === base.length) {
                return true; // same path, different casing
            }
            let sepOffset = parentCandidate.length;
            if (parentCandidate.charAt(parentCandidate.length - 1) === separator) {
                sepOffset--; // adjust the expected sep offset in case our candidate already ends in separator character
            }
            return base.charAt(sepOffset) === separator;
        }
        if (parentCandidate.charAt(parentCandidate.length - 1) !== separator) {
            parentCandidate += separator;
        }
        return base.indexOf(parentCandidate) === 0;
    }

    var _b;
    class LinkedMap {
        constructor() {
            this[_b] = 'LinkedMap';
            this._map = new Map();
            this._head = undefined;
            this._tail = undefined;
            this._size = 0;
            this._state = 0;
        }
        clear() {
            this._map.clear();
            this._head = undefined;
            this._tail = undefined;
            this._size = 0;
            this._state++;
        }
        isEmpty() {
            return !this._head && !this._tail;
        }
        get size() {
            return this._size;
        }
        get first() {
            var _c;
            return (_c = this._head) === null || _c === void 0 ? void 0 : _c.value;
        }
        get last() {
            var _c;
            return (_c = this._tail) === null || _c === void 0 ? void 0 : _c.value;
        }
        has(key) {
            return this._map.has(key);
        }
        get(key, touch = 0 /* None */) {
            const item = this._map.get(key);
            if (!item) {
                return undefined;
            }
            if (touch !== 0 /* None */) {
                this.touch(item, touch);
            }
            return item.value;
        }
        set(key, value, touch = 0 /* None */) {
            let item = this._map.get(key);
            if (item) {
                item.value = value;
                if (touch !== 0 /* None */) {
                    this.touch(item, touch);
                }
            }
            else {
                item = { key, value, next: undefined, previous: undefined };
                switch (touch) {
                    case 0 /* None */:
                        this.addItemLast(item);
                        break;
                    case 1 /* AsOld */:
                        this.addItemFirst(item);
                        break;
                    case 2 /* AsNew */:
                        this.addItemLast(item);
                        break;
                    default:
                        this.addItemLast(item);
                        break;
                }
                this._map.set(key, item);
                this._size++;
            }
            return this;
        }
        delete(key) {
            return !!this.remove(key);
        }
        remove(key) {
            const item = this._map.get(key);
            if (!item) {
                return undefined;
            }
            this._map.delete(key);
            this.removeItem(item);
            this._size--;
            return item.value;
        }
        shift() {
            if (!this._head && !this._tail) {
                return undefined;
            }
            if (!this._head || !this._tail) {
                throw new Error('Invalid list');
            }
            const item = this._head;
            this._map.delete(item.key);
            this.removeItem(item);
            this._size--;
            return item.value;
        }
        forEach(callbackfn, thisArg) {
            const state = this._state;
            let current = this._head;
            while (current) {
                if (thisArg) {
                    callbackfn.bind(thisArg)(current.value, current.key, this);
                }
                else {
                    callbackfn(current.value, current.key, this);
                }
                if (this._state !== state) {
                    throw new Error(`LinkedMap got modified during iteration.`);
                }
                current = current.next;
            }
        }
        keys() {
            const map = this;
            const state = this._state;
            let current = this._head;
            const iterator = {
                [Symbol.iterator]() {
                    return iterator;
                },
                next() {
                    if (map._state !== state) {
                        throw new Error(`LinkedMap got modified during iteration.`);
                    }
                    if (current) {
                        const result = { value: current.key, done: false };
                        current = current.next;
                        return result;
                    }
                    else {
                        return { value: undefined, done: true };
                    }
                }
            };
            return iterator;
        }
        values() {
            const map = this;
            const state = this._state;
            let current = this._head;
            const iterator = {
                [Symbol.iterator]() {
                    return iterator;
                },
                next() {
                    if (map._state !== state) {
                        throw new Error(`LinkedMap got modified during iteration.`);
                    }
                    if (current) {
                        const result = { value: current.value, done: false };
                        current = current.next;
                        return result;
                    }
                    else {
                        return { value: undefined, done: true };
                    }
                }
            };
            return iterator;
        }
        entries() {
            const map = this;
            const state = this._state;
            let current = this._head;
            const iterator = {
                [Symbol.iterator]() {
                    return iterator;
                },
                next() {
                    if (map._state !== state) {
                        throw new Error(`LinkedMap got modified during iteration.`);
                    }
                    if (current) {
                        const result = { value: [current.key, current.value], done: false };
                        current = current.next;
                        return result;
                    }
                    else {
                        return { value: undefined, done: true };
                    }
                }
            };
            return iterator;
        }
        [(_b = Symbol.toStringTag, Symbol.iterator)]() {
            return this.entries();
        }
        trimOld(newSize) {
            if (newSize >= this.size) {
                return;
            }
            if (newSize === 0) {
                this.clear();
                return;
            }
            let current = this._head;
            let currentSize = this.size;
            while (current && currentSize > newSize) {
                this._map.delete(current.key);
                current = current.next;
                currentSize--;
            }
            this._head = current;
            this._size = currentSize;
            if (current) {
                current.previous = undefined;
            }
            this._state++;
        }
        addItemFirst(item) {
            // First time Insert
            if (!this._head && !this._tail) {
                this._tail = item;
            }
            else if (!this._head) {
                throw new Error('Invalid list');
            }
            else {
                item.next = this._head;
                this._head.previous = item;
            }
            this._head = item;
            this._state++;
        }
        addItemLast(item) {
            // First time Insert
            if (!this._head && !this._tail) {
                this._head = item;
            }
            else if (!this._tail) {
                throw new Error('Invalid list');
            }
            else {
                item.previous = this._tail;
                this._tail.next = item;
            }
            this._tail = item;
            this._state++;
        }
        removeItem(item) {
            if (item === this._head && item === this._tail) {
                this._head = undefined;
                this._tail = undefined;
            }
            else if (item === this._head) {
                // This can only happen if size === 1 which is handled
                // by the case above.
                if (!item.next) {
                    throw new Error('Invalid list');
                }
                item.next.previous = undefined;
                this._head = item.next;
            }
            else if (item === this._tail) {
                // This can only happen if size === 1 which is handled
                // by the case above.
                if (!item.previous) {
                    throw new Error('Invalid list');
                }
                item.previous.next = undefined;
                this._tail = item.previous;
            }
            else {
                const next = item.next;
                const previous = item.previous;
                if (!next || !previous) {
                    throw new Error('Invalid list');
                }
                next.previous = previous;
                previous.next = next;
            }
            item.next = undefined;
            item.previous = undefined;
            this._state++;
        }
        touch(item, touch) {
            if (!this._head || !this._tail) {
                throw new Error('Invalid list');
            }
            if ((touch !== 1 /* AsOld */ && touch !== 2 /* AsNew */)) {
                return;
            }
            if (touch === 1 /* AsOld */) {
                if (item === this._head) {
                    return;
                }
                const next = item.next;
                const previous = item.previous;
                // Unlink the item
                if (item === this._tail) {
                    // previous must be defined since item was not head but is tail
                    // So there are more than on item in the map
                    previous.next = undefined;
                    this._tail = previous;
                }
                else {
                    // Both next and previous are not undefined since item was neither head nor tail.
                    next.previous = previous;
                    previous.next = next;
                }
                // Insert the node at head
                item.previous = undefined;
                item.next = this._head;
                this._head.previous = item;
                this._head = item;
                this._state++;
            }
            else if (touch === 2 /* AsNew */) {
                if (item === this._tail) {
                    return;
                }
                const next = item.next;
                const previous = item.previous;
                // Unlink the item.
                if (item === this._head) {
                    // next must be defined since item was not tail but is head
                    // So there are more than on item in the map
                    next.previous = undefined;
                    this._head = next;
                }
                else {
                    // Both next and previous are not undefined since item was neither head nor tail.
                    next.previous = previous;
                    previous.next = next;
                }
                item.next = undefined;
                item.previous = this._tail;
                this._tail.next = item;
                this._tail = item;
                this._state++;
            }
        }
        toJSON() {
            const data = [];
            this.forEach((value, key) => {
                data.push([key, value]);
            });
            return data;
        }
        fromJSON(data) {
            this.clear();
            for (const [key, value] of data) {
                this.set(key, value);
            }
        }
    }
    class LRUCache extends LinkedMap {
        constructor(limit, ratio = 1) {
            super();
            this._limit = limit;
            this._ratio = Math.min(Math.max(0, ratio), 1);
        }
        get limit() {
            return this._limit;
        }
        set limit(limit) {
            this._limit = limit;
            this.checkTrim();
        }
        get(key, touch = 2 /* AsNew */) {
            return super.get(key, touch);
        }
        peek(key) {
            return super.get(key, 0 /* None */);
        }
        set(key, value) {
            super.set(key, value, 2 /* AsNew */);
            this.checkTrim();
            return this;
        }
        checkTrim() {
            if (this.size > this._limit) {
                this.trimOld(Math.round(this._limit * this._ratio));
            }
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    const GLOBSTAR = '**';
    const GLOB_SPLIT = '/';
    const PATH_REGEX = '[/\\\\]'; // any slash or backslash
    const NO_PATH_REGEX = '[^/\\\\]'; // any non-slash and non-backslash
    const ALL_FORWARD_SLASHES = /\//g;
    function starsToRegExp(starCount) {
        switch (starCount) {
            case 0:
                return '';
            case 1:
                return `${NO_PATH_REGEX}*?`; // 1 star matches any number of characters except path separator (/ and \) - non greedy (?)
            default:
                // Matches:  (Path Sep OR Path Val followed by Path Sep OR Path Sep followed by Path Val) 0-many times
                // Group is non capturing because we don't need to capture at all (?:...)
                // Overall we use non-greedy matching because it could be that we match too much
                return `(?:${PATH_REGEX}|${NO_PATH_REGEX}+${PATH_REGEX}|${PATH_REGEX}${NO_PATH_REGEX}+)*?`;
        }
    }
    function splitGlobAware(pattern, splitChar) {
        if (!pattern) {
            return [];
        }
        const segments = [];
        let inBraces = false;
        let inBrackets = false;
        let curVal = '';
        for (const char of pattern) {
            switch (char) {
                case splitChar:
                    if (!inBraces && !inBrackets) {
                        segments.push(curVal);
                        curVal = '';
                        continue;
                    }
                    break;
                case '{':
                    inBraces = true;
                    break;
                case '}':
                    inBraces = false;
                    break;
                case '[':
                    inBrackets = true;
                    break;
                case ']':
                    inBrackets = false;
                    break;
            }
            curVal += char;
        }
        // Tail
        if (curVal) {
            segments.push(curVal);
        }
        return segments;
    }
    function parseRegExp(pattern) {
        if (!pattern) {
            return '';
        }
        let regEx = '';
        // Split up into segments for each slash found
        const segments = splitGlobAware(pattern, GLOB_SPLIT);
        // Special case where we only have globstars
        if (segments.every(s => s === GLOBSTAR)) {
            regEx = '.*';
        }
        // Build regex over segments
        else {
            let previousSegmentWasGlobStar = false;
            segments.forEach((segment, index) => {
                // Globstar is special
                if (segment === GLOBSTAR) {
                    // if we have more than one globstar after another, just ignore it
                    if (!previousSegmentWasGlobStar) {
                        regEx += starsToRegExp(2);
                        previousSegmentWasGlobStar = true;
                    }
                    return;
                }
                // States
                let inBraces = false;
                let braceVal = '';
                let inBrackets = false;
                let bracketVal = '';
                for (const char of segment) {
                    // Support brace expansion
                    if (char !== '}' && inBraces) {
                        braceVal += char;
                        continue;
                    }
                    // Support brackets
                    if (inBrackets && (char !== ']' || !bracketVal) /* ] is literally only allowed as first character in brackets to match it */) {
                        let res;
                        // range operator
                        if (char === '-') {
                            res = char;
                        }
                        // negation operator (only valid on first index in bracket)
                        else if ((char === '^' || char === '!') && !bracketVal) {
                            res = '^';
                        }
                        // glob split matching is not allowed within character ranges
                        // see http://man7.org/linux/man-pages/man7/glob.7.html
                        else if (char === GLOB_SPLIT) {
                            res = '';
                        }
                        // anything else gets escaped
                        else {
                            res = escapeRegExpCharacters(char);
                        }
                        bracketVal += res;
                        continue;
                    }
                    switch (char) {
                        case '{':
                            inBraces = true;
                            continue;
                        case '[':
                            inBrackets = true;
                            continue;
                        case '}': {
                            const choices = splitGlobAware(braceVal, ',');
                            // Converts {foo,bar} => [foo|bar]
                            const braceRegExp = `(?:${choices.map(c => parseRegExp(c)).join('|')})`;
                            regEx += braceRegExp;
                            inBraces = false;
                            braceVal = '';
                            break;
                        }
                        case ']':
                            regEx += ('[' + bracketVal + ']');
                            inBrackets = false;
                            bracketVal = '';
                            break;
                        case '?':
                            regEx += NO_PATH_REGEX; // 1 ? matches any single character except path separator (/ and \)
                            continue;
                        case '*':
                            regEx += starsToRegExp(1);
                            continue;
                        default:
                            regEx += escapeRegExpCharacters(char);
                    }
                }
                // Tail: Add the slash we had split on if there is more to come and the remaining pattern is not a globstar
                // For example if pattern: some/**/*.js we want the "/" after some to be included in the RegEx to prevent
                // a folder called "something" to match as well.
                // However, if pattern: some/**, we tolerate that we also match on "something" because our globstar behaviour
                // is to match 0-N segments.
                if (index < segments.length - 1 && (segments[index + 1] !== GLOBSTAR || index + 2 < segments.length)) {
                    regEx += PATH_REGEX;
                }
                // reset state
                previousSegmentWasGlobStar = false;
            });
        }
        return regEx;
    }
    // regexes to check for trivial glob patterns that just check for String#endsWith
    const T1 = /^\*\*\/\*\.[\w\.-]+$/; // **/*.something
    const T2 = /^\*\*\/([\w\.-]+)\/?$/; // **/something
    const T3 = /^{\*\*\/[\*\.]?[\w\.-]+\/?(,\*\*\/[\*\.]?[\w\.-]+\/?)*}$/; // {**/*.something,**/*.else} or {**/package.json,**/project.json}
    const T3_2 = /^{\*\*\/[\*\.]?[\w\.-]+(\/(\*\*)?)?(,\*\*\/[\*\.]?[\w\.-]+(\/(\*\*)?)?)*}$/; // Like T3, with optional trailing /**
    const T4 = /^\*\*((\/[\w\.-]+)+)\/?$/; // **/something/else
    const T5 = /^([\w\.-]+(\/[\w\.-]+)*)\/?$/; // something/else
    const CACHE = new LRUCache(10000); // bounded to 10000 elements
    const FALSE = function () {
        return false;
    };
    const NULL = function () {
        return null;
    };
    function parsePattern(arg1, options) {
        if (!arg1) {
            return NULL;
        }
        // Handle IRelativePattern
        let pattern;
        if (typeof arg1 !== 'string') {
            pattern = arg1.pattern;
        }
        else {
            pattern = arg1;
        }
        // Whitespace trimming
        pattern = pattern.trim();
        // Check cache
        const patternKey = `${pattern}_${!!options.trimForExclusions}`;
        let parsedPattern = CACHE.get(patternKey);
        if (parsedPattern) {
            return wrapRelativePattern(parsedPattern, arg1);
        }
        // Check for Trivials
        let match;
        if (T1.test(pattern)) { // common pattern: **/*.txt just need endsWith check
            const base = pattern.substr(4); // '**/*'.length === 4
            parsedPattern = function (path, basename) {
                return typeof path === 'string' && path.endsWith(base) ? pattern : null;
            };
        }
        else if (match = T2.exec(trimForExclusions(pattern, options))) { // common pattern: **/some.txt just need basename check
            parsedPattern = trivia2(match[1], pattern);
        }
        else if ((options.trimForExclusions ? T3_2 : T3).test(pattern)) { // repetition of common patterns (see above) {**/*.txt,**/*.png}
            parsedPattern = trivia3(pattern, options);
        }
        else if (match = T4.exec(trimForExclusions(pattern, options))) { // common pattern: **/something/else just need endsWith check
            parsedPattern = trivia4and5(match[1].substr(1), pattern, true);
        }
        else if (match = T5.exec(trimForExclusions(pattern, options))) { // common pattern: something/else just need equals check
            parsedPattern = trivia4and5(match[1], pattern, false);
        }
        // Otherwise convert to pattern
        else {
            parsedPattern = toRegExp(pattern);
        }
        // Cache
        CACHE.set(patternKey, parsedPattern);
        return wrapRelativePattern(parsedPattern, arg1);
    }
    function wrapRelativePattern(parsedPattern, arg2) {
        if (typeof arg2 === 'string') {
            return parsedPattern;
        }
        return function (path, basename) {
            if (!isEqualOrParent(path, arg2.base, !isLinux)) {
                // skip glob matching if `base` is not a parent of `path`
                return null;
            }
            // Given we have checked `base` being a parent of `path`,
            // we can now remove the `base` portion of the `path`
            // and only match on the remaining path components
            return parsedPattern(path.substr(arg2.base.length + 1), basename);
        };
    }
    function trimForExclusions(pattern, options) {
        return options.trimForExclusions && pattern.endsWith('/**') ? pattern.substr(0, pattern.length - 2) : pattern; // dropping **, tailing / is dropped later
    }
    // common pattern: **/some.txt just need basename check
    function trivia2(base, originalPattern) {
        const slashBase = `/${base}`;
        const backslashBase = `\\${base}`;
        const parsedPattern = function (path, basename) {
            if (typeof path !== 'string') {
                return null;
            }
            if (basename) {
                return basename === base ? originalPattern : null;
            }
            return path === base || path.endsWith(slashBase) || path.endsWith(backslashBase) ? originalPattern : null;
        };
        const basenames = [base];
        parsedPattern.basenames = basenames;
        parsedPattern.patterns = [originalPattern];
        parsedPattern.allBasenames = basenames;
        return parsedPattern;
    }
    // repetition of common patterns (see above) {**/*.txt,**/*.png}
    function trivia3(pattern, options) {
        const parsedPatterns = aggregateBasenameMatches(pattern.slice(1, -1).split(',')
            .map(pattern => parsePattern(pattern, options))
            .filter(pattern => pattern !== NULL), pattern);
        const n = parsedPatterns.length;
        if (!n) {
            return NULL;
        }
        if (n === 1) {
            return parsedPatterns[0];
        }
        const parsedPattern = function (path, basename) {
            for (let i = 0, n = parsedPatterns.length; i < n; i++) {
                if (parsedPatterns[i](path, basename)) {
                    return pattern;
                }
            }
            return null;
        };
        const withBasenames = parsedPatterns.find(pattern => !!pattern.allBasenames);
        if (withBasenames) {
            parsedPattern.allBasenames = withBasenames.allBasenames;
        }
        const allPaths = parsedPatterns.reduce((all, current) => current.allPaths ? all.concat(current.allPaths) : all, []);
        if (allPaths.length) {
            parsedPattern.allPaths = allPaths;
        }
        return parsedPattern;
    }
    // common patterns: **/something/else just need endsWith check, something/else just needs and equals check
    function trivia4and5(targetPath, pattern, matchPathEnds) {
        const usingPosixSep = sep === posix.sep;
        const nativePath = usingPosixSep ? targetPath : targetPath.replace(ALL_FORWARD_SLASHES, sep);
        const nativePathEnd = sep + nativePath;
        const targetPathEnd = posix.sep + targetPath;
        const parsedPattern = matchPathEnds ? function (testPath, basename) {
            return typeof testPath === 'string' &&
                ((testPath === nativePath || testPath.endsWith(nativePathEnd))
                    || !usingPosixSep && (testPath === targetPath || testPath.endsWith(targetPathEnd)))
                ? pattern : null;
        } : function (testPath, basename) {
            return typeof testPath === 'string' &&
                (testPath === nativePath
                    || (!usingPosixSep && testPath === targetPath))
                ? pattern : null;
        };
        parsedPattern.allPaths = [(matchPathEnds ? '*/' : './') + targetPath];
        return parsedPattern;
    }
    function toRegExp(pattern) {
        try {
            const regExp = new RegExp(`^${parseRegExp(pattern)}$`);
            return function (path) {
                regExp.lastIndex = 0; // reset RegExp to its initial state to reuse it!
                return typeof path === 'string' && regExp.test(path) ? pattern : null;
            };
        }
        catch (error) {
            return NULL;
        }
    }
    function match(arg1, path, hasSibling) {
        if (!arg1 || typeof path !== 'string') {
            return false;
        }
        return parse(arg1)(path, undefined, hasSibling);
    }
    function parse(arg1, options = {}) {
        if (!arg1) {
            return FALSE;
        }
        // Glob with String
        if (typeof arg1 === 'string' || isRelativePattern(arg1)) {
            const parsedPattern = parsePattern(arg1, options);
            if (parsedPattern === NULL) {
                return FALSE;
            }
            const resultPattern = function (path, basename) {
                return !!parsedPattern(path, basename);
            };
            if (parsedPattern.allBasenames) {
                resultPattern.allBasenames = parsedPattern.allBasenames;
            }
            if (parsedPattern.allPaths) {
                resultPattern.allPaths = parsedPattern.allPaths;
            }
            return resultPattern;
        }
        // Glob with Expression
        return parsedExpression(arg1, options);
    }
    function isRelativePattern(obj) {
        const rp = obj;
        if (!rp) {
            return false;
        }
        return typeof rp.base === 'string' && typeof rp.pattern === 'string';
    }
    function parsedExpression(expression, options) {
        const parsedPatterns = aggregateBasenameMatches(Object.getOwnPropertyNames(expression)
            .map(pattern => parseExpressionPattern(pattern, expression[pattern], options))
            .filter(pattern => pattern !== NULL));
        const n = parsedPatterns.length;
        if (!n) {
            return NULL;
        }
        if (!parsedPatterns.some(parsedPattern => !!parsedPattern.requiresSiblings)) {
            if (n === 1) {
                return parsedPatterns[0];
            }
            const resultExpression = function (path, basename) {
                for (let i = 0, n = parsedPatterns.length; i < n; i++) {
                    // Pattern matches path
                    const result = parsedPatterns[i](path, basename);
                    if (result) {
                        return result;
                    }
                }
                return null;
            };
            const withBasenames = parsedPatterns.find(pattern => !!pattern.allBasenames);
            if (withBasenames) {
                resultExpression.allBasenames = withBasenames.allBasenames;
            }
            const allPaths = parsedPatterns.reduce((all, current) => current.allPaths ? all.concat(current.allPaths) : all, []);
            if (allPaths.length) {
                resultExpression.allPaths = allPaths;
            }
            return resultExpression;
        }
        const resultExpression = function (path, base, hasSibling) {
            let name = undefined;
            for (let i = 0, n = parsedPatterns.length; i < n; i++) {
                // Pattern matches path
                const parsedPattern = parsedPatterns[i];
                if (parsedPattern.requiresSiblings && hasSibling) {
                    if (!base) {
                        base = basename(path);
                    }
                    if (!name) {
                        name = base.substr(0, base.length - extname(path).length);
                    }
                }
                const result = parsedPattern(path, base, name, hasSibling);
                if (result) {
                    return result;
                }
            }
            return null;
        };
        const withBasenames = parsedPatterns.find(pattern => !!pattern.allBasenames);
        if (withBasenames) {
            resultExpression.allBasenames = withBasenames.allBasenames;
        }
        const allPaths = parsedPatterns.reduce((all, current) => current.allPaths ? all.concat(current.allPaths) : all, []);
        if (allPaths.length) {
            resultExpression.allPaths = allPaths;
        }
        return resultExpression;
    }
    function parseExpressionPattern(pattern, value, options) {
        if (value === false) {
            return NULL; // pattern is disabled
        }
        const parsedPattern = parsePattern(pattern, options);
        if (parsedPattern === NULL) {
            return NULL;
        }
        // Expression Pattern is <boolean>
        if (typeof value === 'boolean') {
            return parsedPattern;
        }
        // Expression Pattern is <SiblingClause>
        if (value) {
            const when = value.when;
            if (typeof when === 'string') {
                const result = (path, basename, name, hasSibling) => {
                    if (!hasSibling || !parsedPattern(path, basename)) {
                        return null;
                    }
                    const clausePattern = when.replace('$(basename)', name);
                    const matched = hasSibling(clausePattern);
                    return isThenable(matched) ?
                        matched.then(m => m ? pattern : null) :
                        matched ? pattern : null;
                };
                result.requiresSiblings = true;
                return result;
            }
        }
        // Expression is Anything
        return parsedPattern;
    }
    function aggregateBasenameMatches(parsedPatterns, result) {
        const basenamePatterns = parsedPatterns.filter(parsedPattern => !!parsedPattern.basenames);
        if (basenamePatterns.length < 2) {
            return parsedPatterns;
        }
        const basenames = basenamePatterns.reduce((all, current) => {
            const basenames = current.basenames;
            return basenames ? all.concat(basenames) : all;
        }, []);
        let patterns;
        if (result) {
            patterns = [];
            for (let i = 0, n = basenames.length; i < n; i++) {
                patterns.push(result);
            }
        }
        else {
            patterns = basenamePatterns.reduce((all, current) => {
                const patterns = current.patterns;
                return patterns ? all.concat(patterns) : all;
            }, []);
        }
        const aggregate = function (path, basename) {
            if (typeof path !== 'string') {
                return null;
            }
            if (!basename) {
                let i;
                for (i = path.length; i > 0; i--) {
                    const ch = path.charCodeAt(i - 1);
                    if (ch === 47 /* Slash */ || ch === 92 /* Backslash */) {
                        break;
                    }
                }
                basename = path.substr(i);
            }
            const index = basenames.indexOf(basename);
            return index !== -1 ? patterns[index] : null;
        };
        aggregate.basenames = basenames;
        aggregate.patterns = patterns;
        aggregate.allBasenames = basenames;
        const aggregatedPatterns = parsedPatterns.filter(parsedPattern => !parsedPattern.basenames);
        aggregatedPatterns.push(aggregate);
        return aggregatedPatterns;
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    function score(selector, candidateUri, candidateLanguage, candidateIsSynchronized) {
        if (Array.isArray(selector)) {
            // array -> take max individual value
            let ret = 0;
            for (const filter of selector) {
                const value = score(filter, candidateUri, candidateLanguage, candidateIsSynchronized);
                if (value === 10) {
                    return value; // already at the highest
                }
                if (value > ret) {
                    ret = value;
                }
            }
            return ret;
        }
        else if (typeof selector === 'string') {
            if (!candidateIsSynchronized) {
                return 0;
            }
            // short-hand notion, desugars to
            // 'fooLang' -> { language: 'fooLang'}
            // '*' -> { language: '*' }
            if (selector === '*') {
                return 5;
            }
            else if (selector === candidateLanguage) {
                return 10;
            }
            else {
                return 0;
            }
        }
        else if (selector) {
            // filter -> select accordingly, use defaults for scheme
            const { language, pattern, scheme, hasAccessToAllModels } = selector; // TODO: microsoft/TypeScript#42768
            if (!candidateIsSynchronized && !hasAccessToAllModels) {
                return 0;
            }
            let ret = 0;
            if (scheme) {
                if (scheme === candidateUri.scheme) {
                    ret = 10;
                }
                else if (scheme === '*') {
                    ret = 5;
                }
                else {
                    return 0;
                }
            }
            if (language) {
                if (language === candidateLanguage) {
                    ret = 10;
                }
                else if (language === '*') {
                    ret = Math.max(ret, 5);
                }
                else {
                    return 0;
                }
            }
            if (pattern) {
                let normalizedPattern;
                if (typeof pattern === 'string') {
                    normalizedPattern = pattern;
                }
                else {
                    // Since this pattern has a `base` property, we need
                    // to normalize this path first before passing it on
                    // because we will compare it against `Uri.fsPath`
                    // which uses platform specific separators.
                    // Refs: https://github.com/microsoft/vscode/issues/99938
                    normalizedPattern = Object.assign(Object.assign({}, pattern), { base: normalize(pattern.base) });
                }
                if (normalizedPattern === candidateUri.fsPath || match(normalizedPattern, candidateUri.fsPath)) {
                    ret = 10;
                }
                else {
                    return 0;
                }
            }
            return ret;
        }
        else {
            return 0;
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    function isExclusive(selector) {
        if (typeof selector === 'string') {
            return false;
        }
        else if (Array.isArray(selector)) {
            return selector.every(isExclusive);
        }
        else {
            return !!selector.exclusive; // TODO: microsoft/TypeScript#42768
        }
    }
    class LanguageFeatureRegistry {
        constructor() {
            this._clock = 0;
            this._entries = [];
            this._onDidChange = new Emitter();
        }
        get onDidChange() {
            return this._onDidChange.event;
        }
        register(selector, provider) {
            let entry = {
                selector,
                provider,
                _score: -1,
                _time: this._clock++
            };
            this._entries.push(entry);
            this._lastCandidate = undefined;
            this._onDidChange.fire(this._entries.length);
            return toDisposable(() => {
                if (entry) {
                    const idx = this._entries.indexOf(entry);
                    if (idx >= 0) {
                        this._entries.splice(idx, 1);
                        this._lastCandidate = undefined;
                        this._onDidChange.fire(this._entries.length);
                        entry = undefined;
                    }
                }
            });
        }
        has(model) {
            return this.all(model).length > 0;
        }
        all(model) {
            if (!model) {
                return [];
            }
            this._updateScores(model);
            const result = [];
            // from registry
            for (let entry of this._entries) {
                if (entry._score > 0) {
                    result.push(entry.provider);
                }
            }
            return result;
        }
        ordered(model) {
            const result = [];
            this._orderedForEach(model, entry => result.push(entry.provider));
            return result;
        }
        orderedGroups(model) {
            const result = [];
            let lastBucket;
            let lastBucketScore;
            this._orderedForEach(model, entry => {
                if (lastBucket && lastBucketScore === entry._score) {
                    lastBucket.push(entry.provider);
                }
                else {
                    lastBucketScore = entry._score;
                    lastBucket = [entry.provider];
                    result.push(lastBucket);
                }
            });
            return result;
        }
        _orderedForEach(model, callback) {
            if (!model) {
                return;
            }
            this._updateScores(model);
            for (const entry of this._entries) {
                if (entry._score > 0) {
                    callback(entry);
                }
            }
        }
        _updateScores(model) {
            const candidate = {
                uri: model.uri.toString(),
                language: model.getLanguageId()
            };
            if (this._lastCandidate
                && this._lastCandidate.language === candidate.language
                && this._lastCandidate.uri === candidate.uri) {
                // nothing has changed
                return;
            }
            this._lastCandidate = candidate;
            for (let entry of this._entries) {
                entry._score = score(entry.selector, model.uri, model.getLanguageId(), shouldSynchronizeModel(model));
                if (isExclusive(entry.selector) && entry._score > 0) {
                    // support for one exclusive selector that overwrites
                    // any other selector
                    for (let entry of this._entries) {
                        entry._score = 0;
                    }
                    entry._score = 1000;
                    break;
                }
            }
            // needs sorting
            this._entries.sort(LanguageFeatureRegistry._compareByScoreAndTime);
        }
        static _compareByScoreAndTime(a, b) {
            if (a._score < b._score) {
                return 1;
            }
            else if (a._score > b._score) {
                return -1;
            }
            else if (a._time < b._time) {
                return 1;
            }
            else if (a._time > b._time) {
                return -1;
            }
            else {
                return 0;
            }
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    var __awaiter$1 = (undefined && undefined.__awaiter) || function (thisArg, _arguments, P, generator) {
        function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
        return new (P || (P = Promise))(function (resolve, reject) {
            function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
            function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
            function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
            step((generator = generator.apply(thisArg, _arguments || [])).next());
        });
    };
    class TokenizationRegistry {
        constructor() {
            this._map = new Map();
            this._factories = new Map();
            this._onDidChange = new Emitter();
            this.onDidChange = this._onDidChange.event;
            this._colorMap = null;
        }
        fire(languages) {
            this._onDidChange.fire({
                changedLanguages: languages,
                changedColorMap: false
            });
        }
        register(language, support) {
            this._map.set(language, support);
            this.fire([language]);
            return toDisposable(() => {
                if (this._map.get(language) !== support) {
                    return;
                }
                this._map.delete(language);
                this.fire([language]);
            });
        }
        registerFactory(languageId, factory) {
            var _a;
            (_a = this._factories.get(languageId)) === null || _a === void 0 ? void 0 : _a.dispose();
            const myData = new TokenizationSupportFactoryData(this, languageId, factory);
            this._factories.set(languageId, myData);
            return toDisposable(() => {
                const v = this._factories.get(languageId);
                if (!v || v !== myData) {
                    return;
                }
                this._factories.delete(languageId);
                v.dispose();
            });
        }
        getOrCreate(languageId) {
            return __awaiter$1(this, void 0, void 0, function* () {
                // check first if the support is already set
                const tokenizationSupport = this.get(languageId);
                if (tokenizationSupport) {
                    return tokenizationSupport;
                }
                const factory = this._factories.get(languageId);
                if (!factory || factory.isResolved) {
                    // no factory or factory.resolve already finished
                    return null;
                }
                yield factory.resolve();
                return this.get(languageId);
            });
        }
        get(language) {
            return (this._map.get(language) || null);
        }
        isResolved(languageId) {
            const tokenizationSupport = this.get(languageId);
            if (tokenizationSupport) {
                return true;
            }
            const factory = this._factories.get(languageId);
            if (!factory || factory.isResolved) {
                return true;
            }
            return false;
        }
        setColorMap(colorMap) {
            this._colorMap = colorMap;
            this._onDidChange.fire({
                changedLanguages: Array.from(this._map.keys()),
                changedColorMap: true
            });
        }
        getColorMap() {
            return this._colorMap;
        }
        getDefaultBackground() {
            if (this._colorMap && this._colorMap.length > 2 /* DefaultBackground */) {
                return this._colorMap[2 /* DefaultBackground */];
            }
            return null;
        }
    }
    class TokenizationSupportFactoryData extends Disposable {
        constructor(_registry, _languageId, _factory) {
            super();
            this._registry = _registry;
            this._languageId = _languageId;
            this._factory = _factory;
            this._isDisposed = false;
            this._resolvePromise = null;
            this._isResolved = false;
        }
        get isResolved() {
            return this._isResolved;
        }
        dispose() {
            this._isDisposed = true;
            super.dispose();
        }
        resolve() {
            return __awaiter$1(this, void 0, void 0, function* () {
                if (!this._resolvePromise) {
                    this._resolvePromise = this._create();
                }
                return this._resolvePromise;
            });
        }
        _create() {
            return __awaiter$1(this, void 0, void 0, function* () {
                const value = yield Promise.resolve(this._factory.createTokenizationSupport());
                this._isResolved = true;
                if (value && !this._isDisposed) {
                    this._register(this._registry.register(this._languageId, value));
                }
            });
        }
    }

    // Selects all codicon names encapsulated in the `$()` syntax and wraps the
    /**
     * The Codicon library is a set of default icons that are built-in in VS Code.
     *
     * In the product (outside of base) Codicons should only be used as defaults. In order to have all icons in VS Code
     * themeable, component should define new, UI component specific icons using `iconRegistry.registerIcon`.
     * In that call a Codicon can be named as default.
     */
    class Codicon {
        constructor(id, definition, description) {
            this.id = id;
            this.definition = definition;
            this.description = description;
            Codicon._allCodicons.push(this);
        }
        get classNames() { return 'codicon codicon-' + this.id; }
        // classNamesArray is useful for migrating to ES6 classlist
        get classNamesArray() { return ['codicon', 'codicon-' + this.id]; }
        get cssSelector() { return '.codicon.codicon-' + this.id; }
        /**
         * @returns Returns all default icons covered by the codicon font. Only to be used by the icon registry in platform.
         */
        static getAll() {
            return Codicon._allCodicons;
        }
    }
    // registry
    Codicon._allCodicons = [];
    // built-in icons, with image name
    Codicon.add = new Codicon('add', { fontCharacter: '\\ea60' });
    Codicon.plus = new Codicon('plus', Codicon.add.definition);
    Codicon.gistNew = new Codicon('gist-new', Codicon.add.definition);
    Codicon.repoCreate = new Codicon('repo-create', Codicon.add.definition);
    Codicon.lightbulb = new Codicon('lightbulb', { fontCharacter: '\\ea61' });
    Codicon.lightBulb = new Codicon('light-bulb', { fontCharacter: '\\ea61' });
    Codicon.repo = new Codicon('repo', { fontCharacter: '\\ea62' });
    Codicon.repoDelete = new Codicon('repo-delete', { fontCharacter: '\\ea62' });
    Codicon.gistFork = new Codicon('gist-fork', { fontCharacter: '\\ea63' });
    Codicon.repoForked = new Codicon('repo-forked', { fontCharacter: '\\ea63' });
    Codicon.gitPullRequest = new Codicon('git-pull-request', { fontCharacter: '\\ea64' });
    Codicon.gitPullRequestAbandoned = new Codicon('git-pull-request-abandoned', { fontCharacter: '\\ea64' });
    Codicon.recordKeys = new Codicon('record-keys', { fontCharacter: '\\ea65' });
    Codicon.keyboard = new Codicon('keyboard', { fontCharacter: '\\ea65' });
    Codicon.tag = new Codicon('tag', { fontCharacter: '\\ea66' });
    Codicon.tagAdd = new Codicon('tag-add', { fontCharacter: '\\ea66' });
    Codicon.tagRemove = new Codicon('tag-remove', { fontCharacter: '\\ea66' });
    Codicon.person = new Codicon('person', { fontCharacter: '\\ea67' });
    Codicon.personFollow = new Codicon('person-follow', { fontCharacter: '\\ea67' });
    Codicon.personOutline = new Codicon('person-outline', { fontCharacter: '\\ea67' });
    Codicon.personFilled = new Codicon('person-filled', { fontCharacter: '\\ea67' });
    Codicon.gitBranch = new Codicon('git-branch', { fontCharacter: '\\ea68' });
    Codicon.gitBranchCreate = new Codicon('git-branch-create', { fontCharacter: '\\ea68' });
    Codicon.gitBranchDelete = new Codicon('git-branch-delete', { fontCharacter: '\\ea68' });
    Codicon.sourceControl = new Codicon('source-control', { fontCharacter: '\\ea68' });
    Codicon.mirror = new Codicon('mirror', { fontCharacter: '\\ea69' });
    Codicon.mirrorPublic = new Codicon('mirror-public', { fontCharacter: '\\ea69' });
    Codicon.star = new Codicon('star', { fontCharacter: '\\ea6a' });
    Codicon.starAdd = new Codicon('star-add', { fontCharacter: '\\ea6a' });
    Codicon.starDelete = new Codicon('star-delete', { fontCharacter: '\\ea6a' });
    Codicon.starEmpty = new Codicon('star-empty', { fontCharacter: '\\ea6a' });
    Codicon.comment = new Codicon('comment', { fontCharacter: '\\ea6b' });
    Codicon.commentAdd = new Codicon('comment-add', { fontCharacter: '\\ea6b' });
    Codicon.alert = new Codicon('alert', { fontCharacter: '\\ea6c' });
    Codicon.warning = new Codicon('warning', { fontCharacter: '\\ea6c' });
    Codicon.search = new Codicon('search', { fontCharacter: '\\ea6d' });
    Codicon.searchSave = new Codicon('search-save', { fontCharacter: '\\ea6d' });
    Codicon.logOut = new Codicon('log-out', { fontCharacter: '\\ea6e' });
    Codicon.signOut = new Codicon('sign-out', { fontCharacter: '\\ea6e' });
    Codicon.logIn = new Codicon('log-in', { fontCharacter: '\\ea6f' });
    Codicon.signIn = new Codicon('sign-in', { fontCharacter: '\\ea6f' });
    Codicon.eye = new Codicon('eye', { fontCharacter: '\\ea70' });
    Codicon.eyeUnwatch = new Codicon('eye-unwatch', { fontCharacter: '\\ea70' });
    Codicon.eyeWatch = new Codicon('eye-watch', { fontCharacter: '\\ea70' });
    Codicon.circleFilled = new Codicon('circle-filled', { fontCharacter: '\\ea71' });
    Codicon.primitiveDot = new Codicon('primitive-dot', { fontCharacter: '\\ea71' });
    Codicon.closeDirty = new Codicon('close-dirty', { fontCharacter: '\\ea71' });
    Codicon.debugBreakpoint = new Codicon('debug-breakpoint', { fontCharacter: '\\ea71' });
    Codicon.debugBreakpointDisabled = new Codicon('debug-breakpoint-disabled', { fontCharacter: '\\ea71' });
    Codicon.debugHint = new Codicon('debug-hint', { fontCharacter: '\\ea71' });
    Codicon.primitiveSquare = new Codicon('primitive-square', { fontCharacter: '\\ea72' });
    Codicon.edit = new Codicon('edit', { fontCharacter: '\\ea73' });
    Codicon.pencil = new Codicon('pencil', { fontCharacter: '\\ea73' });
    Codicon.info = new Codicon('info', { fontCharacter: '\\ea74' });
    Codicon.issueOpened = new Codicon('issue-opened', { fontCharacter: '\\ea74' });
    Codicon.gistPrivate = new Codicon('gist-private', { fontCharacter: '\\ea75' });
    Codicon.gitForkPrivate = new Codicon('git-fork-private', { fontCharacter: '\\ea75' });
    Codicon.lock = new Codicon('lock', { fontCharacter: '\\ea75' });
    Codicon.mirrorPrivate = new Codicon('mirror-private', { fontCharacter: '\\ea75' });
    Codicon.close = new Codicon('close', { fontCharacter: '\\ea76' });
    Codicon.removeClose = new Codicon('remove-close', { fontCharacter: '\\ea76' });
    Codicon.x = new Codicon('x', { fontCharacter: '\\ea76' });
    Codicon.repoSync = new Codicon('repo-sync', { fontCharacter: '\\ea77' });
    Codicon.sync = new Codicon('sync', { fontCharacter: '\\ea77' });
    Codicon.clone = new Codicon('clone', { fontCharacter: '\\ea78' });
    Codicon.desktopDownload = new Codicon('desktop-download', { fontCharacter: '\\ea78' });
    Codicon.beaker = new Codicon('beaker', { fontCharacter: '\\ea79' });
    Codicon.microscope = new Codicon('microscope', { fontCharacter: '\\ea79' });
    Codicon.vm = new Codicon('vm', { fontCharacter: '\\ea7a' });
    Codicon.deviceDesktop = new Codicon('device-desktop', { fontCharacter: '\\ea7a' });
    Codicon.file = new Codicon('file', { fontCharacter: '\\ea7b' });
    Codicon.fileText = new Codicon('file-text', { fontCharacter: '\\ea7b' });
    Codicon.more = new Codicon('more', { fontCharacter: '\\ea7c' });
    Codicon.ellipsis = new Codicon('ellipsis', { fontCharacter: '\\ea7c' });
    Codicon.kebabHorizontal = new Codicon('kebab-horizontal', { fontCharacter: '\\ea7c' });
    Codicon.mailReply = new Codicon('mail-reply', { fontCharacter: '\\ea7d' });
    Codicon.reply = new Codicon('reply', { fontCharacter: '\\ea7d' });
    Codicon.organization = new Codicon('organization', { fontCharacter: '\\ea7e' });
    Codicon.organizationFilled = new Codicon('organization-filled', { fontCharacter: '\\ea7e' });
    Codicon.organizationOutline = new Codicon('organization-outline', { fontCharacter: '\\ea7e' });
    Codicon.newFile = new Codicon('new-file', { fontCharacter: '\\ea7f' });
    Codicon.fileAdd = new Codicon('file-add', { fontCharacter: '\\ea7f' });
    Codicon.newFolder = new Codicon('new-folder', { fontCharacter: '\\ea80' });
    Codicon.fileDirectoryCreate = new Codicon('file-directory-create', { fontCharacter: '\\ea80' });
    Codicon.trash = new Codicon('trash', { fontCharacter: '\\ea81' });
    Codicon.trashcan = new Codicon('trashcan', { fontCharacter: '\\ea81' });
    Codicon.history = new Codicon('history', { fontCharacter: '\\ea82' });
    Codicon.clock = new Codicon('clock', { fontCharacter: '\\ea82' });
    Codicon.folder = new Codicon('folder', { fontCharacter: '\\ea83' });
    Codicon.fileDirectory = new Codicon('file-directory', { fontCharacter: '\\ea83' });
    Codicon.symbolFolder = new Codicon('symbol-folder', { fontCharacter: '\\ea83' });
    Codicon.logoGithub = new Codicon('logo-github', { fontCharacter: '\\ea84' });
    Codicon.markGithub = new Codicon('mark-github', { fontCharacter: '\\ea84' });
    Codicon.github = new Codicon('github', { fontCharacter: '\\ea84' });
    Codicon.terminal = new Codicon('terminal', { fontCharacter: '\\ea85' });
    Codicon.console = new Codicon('console', { fontCharacter: '\\ea85' });
    Codicon.repl = new Codicon('repl', { fontCharacter: '\\ea85' });
    Codicon.zap = new Codicon('zap', { fontCharacter: '\\ea86' });
    Codicon.symbolEvent = new Codicon('symbol-event', { fontCharacter: '\\ea86' });
    Codicon.error = new Codicon('error', { fontCharacter: '\\ea87' });
    Codicon.stop = new Codicon('stop', { fontCharacter: '\\ea87' });
    Codicon.variable = new Codicon('variable', { fontCharacter: '\\ea88' });
    Codicon.symbolVariable = new Codicon('symbol-variable', { fontCharacter: '\\ea88' });
    Codicon.array = new Codicon('array', { fontCharacter: '\\ea8a' });
    Codicon.symbolArray = new Codicon('symbol-array', { fontCharacter: '\\ea8a' });
    Codicon.symbolModule = new Codicon('symbol-module', { fontCharacter: '\\ea8b' });
    Codicon.symbolPackage = new Codicon('symbol-package', { fontCharacter: '\\ea8b' });
    Codicon.symbolNamespace = new Codicon('symbol-namespace', { fontCharacter: '\\ea8b' });
    Codicon.symbolObject = new Codicon('symbol-object', { fontCharacter: '\\ea8b' });
    Codicon.symbolMethod = new Codicon('symbol-method', { fontCharacter: '\\ea8c' });
    Codicon.symbolFunction = new Codicon('symbol-function', { fontCharacter: '\\ea8c' });
    Codicon.symbolConstructor = new Codicon('symbol-constructor', { fontCharacter: '\\ea8c' });
    Codicon.symbolBoolean = new Codicon('symbol-boolean', { fontCharacter: '\\ea8f' });
    Codicon.symbolNull = new Codicon('symbol-null', { fontCharacter: '\\ea8f' });
    Codicon.symbolNumeric = new Codicon('symbol-numeric', { fontCharacter: '\\ea90' });
    Codicon.symbolNumber = new Codicon('symbol-number', { fontCharacter: '\\ea90' });
    Codicon.symbolStructure = new Codicon('symbol-structure', { fontCharacter: '\\ea91' });
    Codicon.symbolStruct = new Codicon('symbol-struct', { fontCharacter: '\\ea91' });
    Codicon.symbolParameter = new Codicon('symbol-parameter', { fontCharacter: '\\ea92' });
    Codicon.symbolTypeParameter = new Codicon('symbol-type-parameter', { fontCharacter: '\\ea92' });
    Codicon.symbolKey = new Codicon('symbol-key', { fontCharacter: '\\ea93' });
    Codicon.symbolText = new Codicon('symbol-text', { fontCharacter: '\\ea93' });
    Codicon.symbolReference = new Codicon('symbol-reference', { fontCharacter: '\\ea94' });
    Codicon.goToFile = new Codicon('go-to-file', { fontCharacter: '\\ea94' });
    Codicon.symbolEnum = new Codicon('symbol-enum', { fontCharacter: '\\ea95' });
    Codicon.symbolValue = new Codicon('symbol-value', { fontCharacter: '\\ea95' });
    Codicon.symbolRuler = new Codicon('symbol-ruler', { fontCharacter: '\\ea96' });
    Codicon.symbolUnit = new Codicon('symbol-unit', { fontCharacter: '\\ea96' });
    Codicon.activateBreakpoints = new Codicon('activate-breakpoints', { fontCharacter: '\\ea97' });
    Codicon.archive = new Codicon('archive', { fontCharacter: '\\ea98' });
    Codicon.arrowBoth = new Codicon('arrow-both', { fontCharacter: '\\ea99' });
    Codicon.arrowDown = new Codicon('arrow-down', { fontCharacter: '\\ea9a' });
    Codicon.arrowLeft = new Codicon('arrow-left', { fontCharacter: '\\ea9b' });
    Codicon.arrowRight = new Codicon('arrow-right', { fontCharacter: '\\ea9c' });
    Codicon.arrowSmallDown = new Codicon('arrow-small-down', { fontCharacter: '\\ea9d' });
    Codicon.arrowSmallLeft = new Codicon('arrow-small-left', { fontCharacter: '\\ea9e' });
    Codicon.arrowSmallRight = new Codicon('arrow-small-right', { fontCharacter: '\\ea9f' });
    Codicon.arrowSmallUp = new Codicon('arrow-small-up', { fontCharacter: '\\eaa0' });
    Codicon.arrowUp = new Codicon('arrow-up', { fontCharacter: '\\eaa1' });
    Codicon.bell = new Codicon('bell', { fontCharacter: '\\eaa2' });
    Codicon.bold = new Codicon('bold', { fontCharacter: '\\eaa3' });
    Codicon.book = new Codicon('book', { fontCharacter: '\\eaa4' });
    Codicon.bookmark = new Codicon('bookmark', { fontCharacter: '\\eaa5' });
    Codicon.debugBreakpointConditionalUnverified = new Codicon('debug-breakpoint-conditional-unverified', { fontCharacter: '\\eaa6' });
    Codicon.debugBreakpointConditional = new Codicon('debug-breakpoint-conditional', { fontCharacter: '\\eaa7' });
    Codicon.debugBreakpointConditionalDisabled = new Codicon('debug-breakpoint-conditional-disabled', { fontCharacter: '\\eaa7' });
    Codicon.debugBreakpointDataUnverified = new Codicon('debug-breakpoint-data-unverified', { fontCharacter: '\\eaa8' });
    Codicon.debugBreakpointData = new Codicon('debug-breakpoint-data', { fontCharacter: '\\eaa9' });
    Codicon.debugBreakpointDataDisabled = new Codicon('debug-breakpoint-data-disabled', { fontCharacter: '\\eaa9' });
    Codicon.debugBreakpointLogUnverified = new Codicon('debug-breakpoint-log-unverified', { fontCharacter: '\\eaaa' });
    Codicon.debugBreakpointLog = new Codicon('debug-breakpoint-log', { fontCharacter: '\\eaab' });
    Codicon.debugBreakpointLogDisabled = new Codicon('debug-breakpoint-log-disabled', { fontCharacter: '\\eaab' });
    Codicon.briefcase = new Codicon('briefcase', { fontCharacter: '\\eaac' });
    Codicon.broadcast = new Codicon('broadcast', { fontCharacter: '\\eaad' });
    Codicon.browser = new Codicon('browser', { fontCharacter: '\\eaae' });
    Codicon.bug = new Codicon('bug', { fontCharacter: '\\eaaf' });
    Codicon.calendar = new Codicon('calendar', { fontCharacter: '\\eab0' });
    Codicon.caseSensitive = new Codicon('case-sensitive', { fontCharacter: '\\eab1' });
    Codicon.check = new Codicon('check', { fontCharacter: '\\eab2' });
    Codicon.checklist = new Codicon('checklist', { fontCharacter: '\\eab3' });
    Codicon.chevronDown = new Codicon('chevron-down', { fontCharacter: '\\eab4' });
    Codicon.dropDownButton = new Codicon('drop-down-button', Codicon.chevronDown.definition);
    Codicon.chevronLeft = new Codicon('chevron-left', { fontCharacter: '\\eab5' });
    Codicon.chevronRight = new Codicon('chevron-right', { fontCharacter: '\\eab6' });
    Codicon.chevronUp = new Codicon('chevron-up', { fontCharacter: '\\eab7' });
    Codicon.chromeClose = new Codicon('chrome-close', { fontCharacter: '\\eab8' });
    Codicon.chromeMaximize = new Codicon('chrome-maximize', { fontCharacter: '\\eab9' });
    Codicon.chromeMinimize = new Codicon('chrome-minimize', { fontCharacter: '\\eaba' });
    Codicon.chromeRestore = new Codicon('chrome-restore', { fontCharacter: '\\eabb' });
    Codicon.circleOutline = new Codicon('circle-outline', { fontCharacter: '\\eabc' });
    Codicon.debugBreakpointUnverified = new Codicon('debug-breakpoint-unverified', { fontCharacter: '\\eabc' });
    Codicon.circleSlash = new Codicon('circle-slash', { fontCharacter: '\\eabd' });
    Codicon.circuitBoard = new Codicon('circuit-board', { fontCharacter: '\\eabe' });
    Codicon.clearAll = new Codicon('clear-all', { fontCharacter: '\\eabf' });
    Codicon.clippy = new Codicon('clippy', { fontCharacter: '\\eac0' });
    Codicon.closeAll = new Codicon('close-all', { fontCharacter: '\\eac1' });
    Codicon.cloudDownload = new Codicon('cloud-download', { fontCharacter: '\\eac2' });
    Codicon.cloudUpload = new Codicon('cloud-upload', { fontCharacter: '\\eac3' });
    Codicon.code = new Codicon('code', { fontCharacter: '\\eac4' });
    Codicon.collapseAll = new Codicon('collapse-all', { fontCharacter: '\\eac5' });
    Codicon.colorMode = new Codicon('color-mode', { fontCharacter: '\\eac6' });
    Codicon.commentDiscussion = new Codicon('comment-discussion', { fontCharacter: '\\eac7' });
    Codicon.compareChanges = new Codicon('compare-changes', { fontCharacter: '\\eafd' });
    Codicon.creditCard = new Codicon('credit-card', { fontCharacter: '\\eac9' });
    Codicon.dash = new Codicon('dash', { fontCharacter: '\\eacc' });
    Codicon.dashboard = new Codicon('dashboard', { fontCharacter: '\\eacd' });
    Codicon.database = new Codicon('database', { fontCharacter: '\\eace' });
    Codicon.debugContinue = new Codicon('debug-continue', { fontCharacter: '\\eacf' });
    Codicon.debugDisconnect = new Codicon('debug-disconnect', { fontCharacter: '\\ead0' });
    Codicon.debugPause = new Codicon('debug-pause', { fontCharacter: '\\ead1' });
    Codicon.debugRestart = new Codicon('debug-restart', { fontCharacter: '\\ead2' });
    Codicon.debugStart = new Codicon('debug-start', { fontCharacter: '\\ead3' });
    Codicon.debugStepInto = new Codicon('debug-step-into', { fontCharacter: '\\ead4' });
    Codicon.debugStepOut = new Codicon('debug-step-out', { fontCharacter: '\\ead5' });
    Codicon.debugStepOver = new Codicon('debug-step-over', { fontCharacter: '\\ead6' });
    Codicon.debugStop = new Codicon('debug-stop', { fontCharacter: '\\ead7' });
    Codicon.debug = new Codicon('debug', { fontCharacter: '\\ead8' });
    Codicon.deviceCameraVideo = new Codicon('device-camera-video', { fontCharacter: '\\ead9' });
    Codicon.deviceCamera = new Codicon('device-camera', { fontCharacter: '\\eada' });
    Codicon.deviceMobile = new Codicon('device-mobile', { fontCharacter: '\\eadb' });
    Codicon.diffAdded = new Codicon('diff-added', { fontCharacter: '\\eadc' });
    Codicon.diffIgnored = new Codicon('diff-ignored', { fontCharacter: '\\eadd' });
    Codicon.diffModified = new Codicon('diff-modified', { fontCharacter: '\\eade' });
    Codicon.diffRemoved = new Codicon('diff-removed', { fontCharacter: '\\eadf' });
    Codicon.diffRenamed = new Codicon('diff-renamed', { fontCharacter: '\\eae0' });
    Codicon.diff = new Codicon('diff', { fontCharacter: '\\eae1' });
    Codicon.discard = new Codicon('discard', { fontCharacter: '\\eae2' });
    Codicon.editorLayout = new Codicon('editor-layout', { fontCharacter: '\\eae3' });
    Codicon.emptyWindow = new Codicon('empty-window', { fontCharacter: '\\eae4' });
    Codicon.exclude = new Codicon('exclude', { fontCharacter: '\\eae5' });
    Codicon.extensions = new Codicon('extensions', { fontCharacter: '\\eae6' });
    Codicon.eyeClosed = new Codicon('eye-closed', { fontCharacter: '\\eae7' });
    Codicon.fileBinary = new Codicon('file-binary', { fontCharacter: '\\eae8' });
    Codicon.fileCode = new Codicon('file-code', { fontCharacter: '\\eae9' });
    Codicon.fileMedia = new Codicon('file-media', { fontCharacter: '\\eaea' });
    Codicon.filePdf = new Codicon('file-pdf', { fontCharacter: '\\eaeb' });
    Codicon.fileSubmodule = new Codicon('file-submodule', { fontCharacter: '\\eaec' });
    Codicon.fileSymlinkDirectory = new Codicon('file-symlink-directory', { fontCharacter: '\\eaed' });
    Codicon.fileSymlinkFile = new Codicon('file-symlink-file', { fontCharacter: '\\eaee' });
    Codicon.fileZip = new Codicon('file-zip', { fontCharacter: '\\eaef' });
    Codicon.files = new Codicon('files', { fontCharacter: '\\eaf0' });
    Codicon.filter = new Codicon('filter', { fontCharacter: '\\eaf1' });
    Codicon.flame = new Codicon('flame', { fontCharacter: '\\eaf2' });
    Codicon.foldDown = new Codicon('fold-down', { fontCharacter: '\\eaf3' });
    Codicon.foldUp = new Codicon('fold-up', { fontCharacter: '\\eaf4' });
    Codicon.fold = new Codicon('fold', { fontCharacter: '\\eaf5' });
    Codicon.folderActive = new Codicon('folder-active', { fontCharacter: '\\eaf6' });
    Codicon.folderOpened = new Codicon('folder-opened', { fontCharacter: '\\eaf7' });
    Codicon.gear = new Codicon('gear', { fontCharacter: '\\eaf8' });
    Codicon.gift = new Codicon('gift', { fontCharacter: '\\eaf9' });
    Codicon.gistSecret = new Codicon('gist-secret', { fontCharacter: '\\eafa' });
    Codicon.gist = new Codicon('gist', { fontCharacter: '\\eafb' });
    Codicon.gitCommit = new Codicon('git-commit', { fontCharacter: '\\eafc' });
    Codicon.gitCompare = new Codicon('git-compare', { fontCharacter: '\\eafd' });
    Codicon.gitMerge = new Codicon('git-merge', { fontCharacter: '\\eafe' });
    Codicon.githubAction = new Codicon('github-action', { fontCharacter: '\\eaff' });
    Codicon.githubAlt = new Codicon('github-alt', { fontCharacter: '\\eb00' });
    Codicon.globe = new Codicon('globe', { fontCharacter: '\\eb01' });
    Codicon.grabber = new Codicon('grabber', { fontCharacter: '\\eb02' });
    Codicon.graph = new Codicon('graph', { fontCharacter: '\\eb03' });
    Codicon.gripper = new Codicon('gripper', { fontCharacter: '\\eb04' });
    Codicon.heart = new Codicon('heart', { fontCharacter: '\\eb05' });
    Codicon.home = new Codicon('home', { fontCharacter: '\\eb06' });
    Codicon.horizontalRule = new Codicon('horizontal-rule', { fontCharacter: '\\eb07' });
    Codicon.hubot = new Codicon('hubot', { fontCharacter: '\\eb08' });
    Codicon.inbox = new Codicon('inbox', { fontCharacter: '\\eb09' });
    Codicon.issueClosed = new Codicon('issue-closed', { fontCharacter: '\\eba4' });
    Codicon.issueReopened = new Codicon('issue-reopened', { fontCharacter: '\\eb0b' });
    Codicon.issues = new Codicon('issues', { fontCharacter: '\\eb0c' });
    Codicon.italic = new Codicon('italic', { fontCharacter: '\\eb0d' });
    Codicon.jersey = new Codicon('jersey', { fontCharacter: '\\eb0e' });
    Codicon.json = new Codicon('json', { fontCharacter: '\\eb0f' });
    Codicon.kebabVertical = new Codicon('kebab-vertical', { fontCharacter: '\\eb10' });
    Codicon.key = new Codicon('key', { fontCharacter: '\\eb11' });
    Codicon.law = new Codicon('law', { fontCharacter: '\\eb12' });
    Codicon.lightbulbAutofix = new Codicon('lightbulb-autofix', { fontCharacter: '\\eb13' });
    Codicon.linkExternal = new Codicon('link-external', { fontCharacter: '\\eb14' });
    Codicon.link = new Codicon('link', { fontCharacter: '\\eb15' });
    Codicon.listOrdered = new Codicon('list-ordered', { fontCharacter: '\\eb16' });
    Codicon.listUnordered = new Codicon('list-unordered', { fontCharacter: '\\eb17' });
    Codicon.liveShare = new Codicon('live-share', { fontCharacter: '\\eb18' });
    Codicon.loading = new Codicon('loading', { fontCharacter: '\\eb19' });
    Codicon.location = new Codicon('location', { fontCharacter: '\\eb1a' });
    Codicon.mailRead = new Codicon('mail-read', { fontCharacter: '\\eb1b' });
    Codicon.mail = new Codicon('mail', { fontCharacter: '\\eb1c' });
    Codicon.markdown = new Codicon('markdown', { fontCharacter: '\\eb1d' });
    Codicon.megaphone = new Codicon('megaphone', { fontCharacter: '\\eb1e' });
    Codicon.mention = new Codicon('mention', { fontCharacter: '\\eb1f' });
    Codicon.milestone = new Codicon('milestone', { fontCharacter: '\\eb20' });
    Codicon.mortarBoard = new Codicon('mortar-board', { fontCharacter: '\\eb21' });
    Codicon.move = new Codicon('move', { fontCharacter: '\\eb22' });
    Codicon.multipleWindows = new Codicon('multiple-windows', { fontCharacter: '\\eb23' });
    Codicon.mute = new Codicon('mute', { fontCharacter: '\\eb24' });
    Codicon.noNewline = new Codicon('no-newline', { fontCharacter: '\\eb25' });
    Codicon.note = new Codicon('note', { fontCharacter: '\\eb26' });
    Codicon.octoface = new Codicon('octoface', { fontCharacter: '\\eb27' });
    Codicon.openPreview = new Codicon('open-preview', { fontCharacter: '\\eb28' });
    Codicon.package_ = new Codicon('package', { fontCharacter: '\\eb29' });
    Codicon.paintcan = new Codicon('paintcan', { fontCharacter: '\\eb2a' });
    Codicon.pin = new Codicon('pin', { fontCharacter: '\\eb2b' });
    Codicon.play = new Codicon('play', { fontCharacter: '\\eb2c' });
    Codicon.run = new Codicon('run', { fontCharacter: '\\eb2c' });
    Codicon.plug = new Codicon('plug', { fontCharacter: '\\eb2d' });
    Codicon.preserveCase = new Codicon('preserve-case', { fontCharacter: '\\eb2e' });
    Codicon.preview = new Codicon('preview', { fontCharacter: '\\eb2f' });
    Codicon.project = new Codicon('project', { fontCharacter: '\\eb30' });
    Codicon.pulse = new Codicon('pulse', { fontCharacter: '\\eb31' });
    Codicon.question = new Codicon('question', { fontCharacter: '\\eb32' });
    Codicon.quote = new Codicon('quote', { fontCharacter: '\\eb33' });
    Codicon.radioTower = new Codicon('radio-tower', { fontCharacter: '\\eb34' });
    Codicon.reactions = new Codicon('reactions', { fontCharacter: '\\eb35' });
    Codicon.references = new Codicon('references', { fontCharacter: '\\eb36' });
    Codicon.refresh = new Codicon('refresh', { fontCharacter: '\\eb37' });
    Codicon.regex = new Codicon('regex', { fontCharacter: '\\eb38' });
    Codicon.remoteExplorer = new Codicon('remote-explorer', { fontCharacter: '\\eb39' });
    Codicon.remote = new Codicon('remote', { fontCharacter: '\\eb3a' });
    Codicon.remove = new Codicon('remove', { fontCharacter: '\\eb3b' });
    Codicon.replaceAll = new Codicon('replace-all', { fontCharacter: '\\eb3c' });
    Codicon.replace = new Codicon('replace', { fontCharacter: '\\eb3d' });
    Codicon.repoClone = new Codicon('repo-clone', { fontCharacter: '\\eb3e' });
    Codicon.repoForcePush = new Codicon('repo-force-push', { fontCharacter: '\\eb3f' });
    Codicon.repoPull = new Codicon('repo-pull', { fontCharacter: '\\eb40' });
    Codicon.repoPush = new Codicon('repo-push', { fontCharacter: '\\eb41' });
    Codicon.report = new Codicon('report', { fontCharacter: '\\eb42' });
    Codicon.requestChanges = new Codicon('request-changes', { fontCharacter: '\\eb43' });
    Codicon.rocket = new Codicon('rocket', { fontCharacter: '\\eb44' });
    Codicon.rootFolderOpened = new Codicon('root-folder-opened', { fontCharacter: '\\eb45' });
    Codicon.rootFolder = new Codicon('root-folder', { fontCharacter: '\\eb46' });
    Codicon.rss = new Codicon('rss', { fontCharacter: '\\eb47' });
    Codicon.ruby = new Codicon('ruby', { fontCharacter: '\\eb48' });
    Codicon.saveAll = new Codicon('save-all', { fontCharacter: '\\eb49' });
    Codicon.saveAs = new Codicon('save-as', { fontCharacter: '\\eb4a' });
    Codicon.save = new Codicon('save', { fontCharacter: '\\eb4b' });
    Codicon.screenFull = new Codicon('screen-full', { fontCharacter: '\\eb4c' });
    Codicon.screenNormal = new Codicon('screen-normal', { fontCharacter: '\\eb4d' });
    Codicon.searchStop = new Codicon('search-stop', { fontCharacter: '\\eb4e' });
    Codicon.server = new Codicon('server', { fontCharacter: '\\eb50' });
    Codicon.settingsGear = new Codicon('settings-gear', { fontCharacter: '\\eb51' });
    Codicon.settings = new Codicon('settings', { fontCharacter: '\\eb52' });
    Codicon.shield = new Codicon('shield', { fontCharacter: '\\eb53' });
    Codicon.smiley = new Codicon('smiley', { fontCharacter: '\\eb54' });
    Codicon.sortPrecedence = new Codicon('sort-precedence', { fontCharacter: '\\eb55' });
    Codicon.splitHorizontal = new Codicon('split-horizontal', { fontCharacter: '\\eb56' });
    Codicon.splitVertical = new Codicon('split-vertical', { fontCharacter: '\\eb57' });
    Codicon.squirrel = new Codicon('squirrel', { fontCharacter: '\\eb58' });
    Codicon.starFull = new Codicon('star-full', { fontCharacter: '\\eb59' });
    Codicon.starHalf = new Codicon('star-half', { fontCharacter: '\\eb5a' });
    Codicon.symbolClass = new Codicon('symbol-class', { fontCharacter: '\\eb5b' });
    Codicon.symbolColor = new Codicon('symbol-color', { fontCharacter: '\\eb5c' });
    Codicon.symbolCustomColor = new Codicon('symbol-customcolor', { fontCharacter: '\\eb5c' });
    Codicon.symbolConstant = new Codicon('symbol-constant', { fontCharacter: '\\eb5d' });
    Codicon.symbolEnumMember = new Codicon('symbol-enum-member', { fontCharacter: '\\eb5e' });
    Codicon.symbolField = new Codicon('symbol-field', { fontCharacter: '\\eb5f' });
    Codicon.symbolFile = new Codicon('symbol-file', { fontCharacter: '\\eb60' });
    Codicon.symbolInterface = new Codicon('symbol-interface', { fontCharacter: '\\eb61' });
    Codicon.symbolKeyword = new Codicon('symbol-keyword', { fontCharacter: '\\eb62' });
    Codicon.symbolMisc = new Codicon('symbol-misc', { fontCharacter: '\\eb63' });
    Codicon.symbolOperator = new Codicon('symbol-operator', { fontCharacter: '\\eb64' });
    Codicon.symbolProperty = new Codicon('symbol-property', { fontCharacter: '\\eb65' });
    Codicon.wrench = new Codicon('wrench', { fontCharacter: '\\eb65' });
    Codicon.wrenchSubaction = new Codicon('wrench-subaction', { fontCharacter: '\\eb65' });
    Codicon.symbolSnippet = new Codicon('symbol-snippet', { fontCharacter: '\\eb66' });
    Codicon.tasklist = new Codicon('tasklist', { fontCharacter: '\\eb67' });
    Codicon.telescope = new Codicon('telescope', { fontCharacter: '\\eb68' });
    Codicon.textSize = new Codicon('text-size', { fontCharacter: '\\eb69' });
    Codicon.threeBars = new Codicon('three-bars', { fontCharacter: '\\eb6a' });
    Codicon.thumbsdown = new Codicon('thumbsdown', { fontCharacter: '\\eb6b' });
    Codicon.thumbsup = new Codicon('thumbsup', { fontCharacter: '\\eb6c' });
    Codicon.tools = new Codicon('tools', { fontCharacter: '\\eb6d' });
    Codicon.triangleDown = new Codicon('triangle-down', { fontCharacter: '\\eb6e' });
    Codicon.triangleLeft = new Codicon('triangle-left', { fontCharacter: '\\eb6f' });
    Codicon.triangleRight = new Codicon('triangle-right', { fontCharacter: '\\eb70' });
    Codicon.triangleUp = new Codicon('triangle-up', { fontCharacter: '\\eb71' });
    Codicon.twitter = new Codicon('twitter', { fontCharacter: '\\eb72' });
    Codicon.unfold = new Codicon('unfold', { fontCharacter: '\\eb73' });
    Codicon.unlock = new Codicon('unlock', { fontCharacter: '\\eb74' });
    Codicon.unmute = new Codicon('unmute', { fontCharacter: '\\eb75' });
    Codicon.unverified = new Codicon('unverified', { fontCharacter: '\\eb76' });
    Codicon.verified = new Codicon('verified', { fontCharacter: '\\eb77' });
    Codicon.versions = new Codicon('versions', { fontCharacter: '\\eb78' });
    Codicon.vmActive = new Codicon('vm-active', { fontCharacter: '\\eb79' });
    Codicon.vmOutline = new Codicon('vm-outline', { fontCharacter: '\\eb7a' });
    Codicon.vmRunning = new Codicon('vm-running', { fontCharacter: '\\eb7b' });
    Codicon.watch = new Codicon('watch', { fontCharacter: '\\eb7c' });
    Codicon.whitespace = new Codicon('whitespace', { fontCharacter: '\\eb7d' });
    Codicon.wholeWord = new Codicon('whole-word', { fontCharacter: '\\eb7e' });
    Codicon.window = new Codicon('window', { fontCharacter: '\\eb7f' });
    Codicon.wordWrap = new Codicon('word-wrap', { fontCharacter: '\\eb80' });
    Codicon.zoomIn = new Codicon('zoom-in', { fontCharacter: '\\eb81' });
    Codicon.zoomOut = new Codicon('zoom-out', { fontCharacter: '\\eb82' });
    Codicon.listFilter = new Codicon('list-filter', { fontCharacter: '\\eb83' });
    Codicon.listFlat = new Codicon('list-flat', { fontCharacter: '\\eb84' });
    Codicon.listSelection = new Codicon('list-selection', { fontCharacter: '\\eb85' });
    Codicon.selection = new Codicon('selection', { fontCharacter: '\\eb85' });
    Codicon.listTree = new Codicon('list-tree', { fontCharacter: '\\eb86' });
    Codicon.debugBreakpointFunctionUnverified = new Codicon('debug-breakpoint-function-unverified', { fontCharacter: '\\eb87' });
    Codicon.debugBreakpointFunction = new Codicon('debug-breakpoint-function', { fontCharacter: '\\eb88' });
    Codicon.debugBreakpointFunctionDisabled = new Codicon('debug-breakpoint-function-disabled', { fontCharacter: '\\eb88' });
    Codicon.debugStackframeActive = new Codicon('debug-stackframe-active', { fontCharacter: '\\eb89' });
    Codicon.debugStackframeDot = new Codicon('debug-stackframe-dot', { fontCharacter: '\\eb8a' });
    Codicon.debugStackframe = new Codicon('debug-stackframe', { fontCharacter: '\\eb8b' });
    Codicon.debugStackframeFocused = new Codicon('debug-stackframe-focused', { fontCharacter: '\\eb8b' });
    Codicon.debugBreakpointUnsupported = new Codicon('debug-breakpoint-unsupported', { fontCharacter: '\\eb8c' });
    Codicon.symbolString = new Codicon('symbol-string', { fontCharacter: '\\eb8d' });
    Codicon.debugReverseContinue = new Codicon('debug-reverse-continue', { fontCharacter: '\\eb8e' });
    Codicon.debugStepBack = new Codicon('debug-step-back', { fontCharacter: '\\eb8f' });
    Codicon.debugRestartFrame = new Codicon('debug-restart-frame', { fontCharacter: '\\eb90' });
    Codicon.callIncoming = new Codicon('call-incoming', { fontCharacter: '\\eb92' });
    Codicon.callOutgoing = new Codicon('call-outgoing', { fontCharacter: '\\eb93' });
    Codicon.menu = new Codicon('menu', { fontCharacter: '\\eb94' });
    Codicon.expandAll = new Codicon('expand-all', { fontCharacter: '\\eb95' });
    Codicon.feedback = new Codicon('feedback', { fontCharacter: '\\eb96' });
    Codicon.groupByRefType = new Codicon('group-by-ref-type', { fontCharacter: '\\eb97' });
    Codicon.ungroupByRefType = new Codicon('ungroup-by-ref-type', { fontCharacter: '\\eb98' });
    Codicon.account = new Codicon('account', { fontCharacter: '\\eb99' });
    Codicon.bellDot = new Codicon('bell-dot', { fontCharacter: '\\eb9a' });
    Codicon.debugConsole = new Codicon('debug-console', { fontCharacter: '\\eb9b' });
    Codicon.library = new Codicon('library', { fontCharacter: '\\eb9c' });
    Codicon.output = new Codicon('output', { fontCharacter: '\\eb9d' });
    Codicon.runAll = new Codicon('run-all', { fontCharacter: '\\eb9e' });
    Codicon.syncIgnored = new Codicon('sync-ignored', { fontCharacter: '\\eb9f' });
    Codicon.pinned = new Codicon('pinned', { fontCharacter: '\\eba0' });
    Codicon.githubInverted = new Codicon('github-inverted', { fontCharacter: '\\eba1' });
    Codicon.debugAlt = new Codicon('debug-alt', { fontCharacter: '\\eb91' });
    Codicon.serverProcess = new Codicon('server-process', { fontCharacter: '\\eba2' });
    Codicon.serverEnvironment = new Codicon('server-environment', { fontCharacter: '\\eba3' });
    Codicon.pass = new Codicon('pass', { fontCharacter: '\\eba4' });
    Codicon.stopCircle = new Codicon('stop-circle', { fontCharacter: '\\eba5' });
    Codicon.playCircle = new Codicon('play-circle', { fontCharacter: '\\eba6' });
    Codicon.record = new Codicon('record', { fontCharacter: '\\eba7' });
    Codicon.debugAltSmall = new Codicon('debug-alt-small', { fontCharacter: '\\eba8' });
    Codicon.vmConnect = new Codicon('vm-connect', { fontCharacter: '\\eba9' });
    Codicon.cloud = new Codicon('cloud', { fontCharacter: '\\ebaa' });
    Codicon.merge = new Codicon('merge', { fontCharacter: '\\ebab' });
    Codicon.exportIcon = new Codicon('export', { fontCharacter: '\\ebac' });
    Codicon.graphLeft = new Codicon('graph-left', { fontCharacter: '\\ebad' });
    Codicon.magnet = new Codicon('magnet', { fontCharacter: '\\ebae' });
    Codicon.notebook = new Codicon('notebook', { fontCharacter: '\\ebaf' });
    Codicon.redo = new Codicon('redo', { fontCharacter: '\\ebb0' });
    Codicon.checkAll = new Codicon('check-all', { fontCharacter: '\\ebb1' });
    Codicon.pinnedDirty = new Codicon('pinned-dirty', { fontCharacter: '\\ebb2' });
    Codicon.passFilled = new Codicon('pass-filled', { fontCharacter: '\\ebb3' });
    Codicon.circleLargeFilled = new Codicon('circle-large-filled', { fontCharacter: '\\ebb4' });
    Codicon.circleLargeOutline = new Codicon('circle-large-outline', { fontCharacter: '\\ebb5' });
    Codicon.combine = new Codicon('combine', { fontCharacter: '\\ebb6' });
    Codicon.gather = new Codicon('gather', { fontCharacter: '\\ebb6' });
    Codicon.table = new Codicon('table', { fontCharacter: '\\ebb7' });
    Codicon.variableGroup = new Codicon('variable-group', { fontCharacter: '\\ebb8' });
    Codicon.typeHierarchy = new Codicon('type-hierarchy', { fontCharacter: '\\ebb9' });
    Codicon.typeHierarchySub = new Codicon('type-hierarchy-sub', { fontCharacter: '\\ebba' });
    Codicon.typeHierarchySuper = new Codicon('type-hierarchy-super', { fontCharacter: '\\ebbb' });
    Codicon.gitPullRequestCreate = new Codicon('git-pull-request-create', { fontCharacter: '\\ebbc' });
    Codicon.runAbove = new Codicon('run-above', { fontCharacter: '\\ebbd' });
    Codicon.runBelow = new Codicon('run-below', { fontCharacter: '\\ebbe' });
    Codicon.notebookTemplate = new Codicon('notebook-template', { fontCharacter: '\\ebbf' });
    Codicon.debugRerun = new Codicon('debug-rerun', { fontCharacter: '\\ebc0' });
    Codicon.workspaceTrusted = new Codicon('workspace-trusted', { fontCharacter: '\\ebc1' });
    Codicon.workspaceUntrusted = new Codicon('workspace-untrusted', { fontCharacter: '\\ebc2' });
    Codicon.workspaceUnspecified = new Codicon('workspace-unspecified', { fontCharacter: '\\ebc3' });
    Codicon.terminalCmd = new Codicon('terminal-cmd', { fontCharacter: '\\ebc4' });
    Codicon.terminalDebian = new Codicon('terminal-debian', { fontCharacter: '\\ebc5' });
    Codicon.terminalLinux = new Codicon('terminal-linux', { fontCharacter: '\\ebc6' });
    Codicon.terminalPowershell = new Codicon('terminal-powershell', { fontCharacter: '\\ebc7' });
    Codicon.terminalTmux = new Codicon('terminal-tmux', { fontCharacter: '\\ebc8' });
    Codicon.terminalUbuntu = new Codicon('terminal-ubuntu', { fontCharacter: '\\ebc9' });
    Codicon.terminalBash = new Codicon('terminal-bash', { fontCharacter: '\\ebca' });
    Codicon.arrowSwap = new Codicon('arrow-swap', { fontCharacter: '\\ebcb' });
    Codicon.copy = new Codicon('copy', { fontCharacter: '\\ebcc' });
    Codicon.personAdd = new Codicon('person-add', { fontCharacter: '\\ebcd' });
    Codicon.filterFilled = new Codicon('filter-filled', { fontCharacter: '\\ebce' });
    Codicon.wand = new Codicon('wand', { fontCharacter: '\\ebcf' });
    Codicon.debugLineByLine = new Codicon('debug-line-by-line', { fontCharacter: '\\ebd0' });
    Codicon.inspect = new Codicon('inspect', { fontCharacter: '\\ebd1' });
    Codicon.layers = new Codicon('layers', { fontCharacter: '\\ebd2' });
    Codicon.layersDot = new Codicon('layers-dot', { fontCharacter: '\\ebd3' });
    Codicon.layersActive = new Codicon('layers-active', { fontCharacter: '\\ebd4' });
    Codicon.compass = new Codicon('compass', { fontCharacter: '\\ebd5' });
    Codicon.compassDot = new Codicon('compass-dot', { fontCharacter: '\\ebd6' });
    Codicon.compassActive = new Codicon('compass-active', { fontCharacter: '\\ebd7' });
    Codicon.azure = new Codicon('azure', { fontCharacter: '\\ebd8' });
    Codicon.issueDraft = new Codicon('issue-draft', { fontCharacter: '\\ebd9' });
    Codicon.gitPullRequestClosed = new Codicon('git-pull-request-closed', { fontCharacter: '\\ebda' });
    Codicon.gitPullRequestDraft = new Codicon('git-pull-request-draft', { fontCharacter: '\\ebdb' });
    Codicon.debugAll = new Codicon('debug-all', { fontCharacter: '\\ebdc' });
    Codicon.debugCoverage = new Codicon('debug-coverage', { fontCharacter: '\\ebdd' });
    Codicon.runErrors = new Codicon('run-errors', { fontCharacter: '\\ebde' });
    Codicon.folderLibrary = new Codicon('folder-library', { fontCharacter: '\\ebdf' });
    Codicon.debugContinueSmall = new Codicon('debug-continue-small', { fontCharacter: '\\ebe0' });
    Codicon.beakerStop = new Codicon('beaker-stop', { fontCharacter: '\\ebe1' });
    Codicon.graphLine = new Codicon('graph-line', { fontCharacter: '\\ebe2' });
    Codicon.graphScatter = new Codicon('graph-scatter', { fontCharacter: '\\ebe3' });
    Codicon.pieChart = new Codicon('pie-chart', { fontCharacter: '\\ebe4' });
    Codicon.bracket = new Codicon('bracket', Codicon.json.definition);
    Codicon.bracketDot = new Codicon('bracket-dot', { fontCharacter: '\\ebe5' });
    Codicon.bracketError = new Codicon('bracket-error', { fontCharacter: '\\ebe6' });
    Codicon.lockSmall = new Codicon('lock-small', { fontCharacter: '\\ebe7' });
    Codicon.azureDevops = new Codicon('azure-devops', { fontCharacter: '\\ebe8' });
    Codicon.verifiedFilled = new Codicon('verified-filled', { fontCharacter: '\\ebe9' });
    Codicon.newLine = new Codicon('newline', { fontCharacter: '\\ebea' });
    Codicon.layout = new Codicon('layout', { fontCharacter: '\\ebeb' });
    Codicon.layoutActivitybarLeft = new Codicon('layout-activitybar-left', { fontCharacter: '\\ebec' });
    Codicon.layoutActivitybarRight = new Codicon('layout-activitybar-right', { fontCharacter: '\\ebed' });
    Codicon.layoutPanelLeft = new Codicon('layout-panel-left', { fontCharacter: '\\ebee' });
    Codicon.layoutPanelCenter = new Codicon('layout-panel-center', { fontCharacter: '\\ebef' });
    Codicon.layoutPanelJustify = new Codicon('layout-panel-justify', { fontCharacter: '\\ebf0' });
    Codicon.layoutPanelRight = new Codicon('layout-panel-right', { fontCharacter: '\\ebf1' });
    Codicon.layoutPanel = new Codicon('layout-panel', { fontCharacter: '\\ebf2' });
    Codicon.layoutSidebarLeft = new Codicon('layout-sidebar-left', { fontCharacter: '\\ebf3' });
    Codicon.layoutSidebarRight = new Codicon('layout-sidebar-right', { fontCharacter: '\\ebf4' });
    Codicon.layoutStatusbar = new Codicon('layout-statusbar', { fontCharacter: '\\ebf5' });
    Codicon.layoutMenubar = new Codicon('layout-menubar', { fontCharacter: '\\ebf6' });
    Codicon.layoutCentered = new Codicon('layout-centered', { fontCharacter: '\\ebf7' });
    Codicon.target = new Codicon('target', { fontCharacter: '\\ebf8' });
    // derived icons, that could become separate icons
    Codicon.dialogError = new Codicon('dialog-error', Codicon.error.definition);
    Codicon.dialogWarning = new Codicon('dialog-warning', Codicon.warning.definition);
    Codicon.dialogInfo = new Codicon('dialog-info', Codicon.info.definition);
    Codicon.dialogClose = new Codicon('dialog-close', Codicon.close.definition);
    Codicon.treeItemExpanded = new Codicon('tree-item-expanded', Codicon.chevronDown.definition); // collapsed is done with rotation
    Codicon.treeFilterOnTypeOn = new Codicon('tree-filter-on-type-on', Codicon.listFilter.definition);
    Codicon.treeFilterOnTypeOff = new Codicon('tree-filter-on-type-off', Codicon.listSelection.definition);
    Codicon.treeFilterClear = new Codicon('tree-filter-clear', Codicon.close.definition);
    Codicon.treeItemLoading = new Codicon('tree-item-loading', Codicon.loading.definition);
    Codicon.menuSelection = new Codicon('menu-selection', Codicon.check.definition);
    Codicon.menuSubmenu = new Codicon('menu-submenu', Codicon.chevronRight.definition);
    Codicon.menuBarMore = new Codicon('menubar-more', Codicon.more.definition);
    Codicon.scrollbarButtonLeft = new Codicon('scrollbar-button-left', Codicon.triangleLeft.definition);
    Codicon.scrollbarButtonRight = new Codicon('scrollbar-button-right', Codicon.triangleRight.definition);
    Codicon.scrollbarButtonUp = new Codicon('scrollbar-button-up', Codicon.triangleUp.definition);
    Codicon.scrollbarButtonDown = new Codicon('scrollbar-button-down', Codicon.triangleDown.definition);
    Codicon.toolBarMore = new Codicon('toolbar-more', Codicon.more.definition);
    Codicon.quickInputBack = new Codicon('quick-input-back', Codicon.arrowLeft.definition);
    var CSSIcon;
    (function (CSSIcon) {
        CSSIcon.iconNameSegment = '[A-Za-z0-9]+';
        CSSIcon.iconNameExpression = '[A-Za-z0-9-]+';
        CSSIcon.iconModifierExpression = '~[A-Za-z]+';
        CSSIcon.iconNameCharacter = '[A-Za-z0-9~-]';
        const cssIconIdRegex = new RegExp(`^(${CSSIcon.iconNameExpression})(${CSSIcon.iconModifierExpression})?$`);
        function asClassNameArray(icon) {
            if (icon instanceof Codicon) {
                return ['codicon', 'codicon-' + icon.id];
            }
            const match = cssIconIdRegex.exec(icon.id);
            if (!match) {
                return asClassNameArray(Codicon.error);
            }
            let [, id, modifier] = match;
            const classNames = ['codicon', 'codicon-' + id];
            if (modifier) {
                classNames.push('codicon-modifier-' + modifier.substr(1));
            }
            return classNames;
        }
        CSSIcon.asClassNameArray = asClassNameArray;
        function asClassName(icon) {
            return asClassNameArray(icon).join(' ');
        }
        CSSIcon.asClassName = asClassName;
        function asCSSSelector(icon) {
            return '.' + asClassNameArray(icon).join('.');
        }
        CSSIcon.asCSSSelector = asCSSSelector;
    })(CSSIcon || (CSSIcon = {}));

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    class Token {
        constructor(offset, type, language) {
            this._tokenBrand = undefined;
            this.offset = offset;
            this.type = type;
            this.language = language;
        }
        toString() {
            return '(' + this.offset + ', ' + this.type + ')';
        }
    }
    /**
     * @internal
     */
    var CompletionItemKinds;
    (function (CompletionItemKinds) {
        const byKind = new Map();
        byKind.set(0 /* Method */, Codicon.symbolMethod);
        byKind.set(1 /* Function */, Codicon.symbolFunction);
        byKind.set(2 /* Constructor */, Codicon.symbolConstructor);
        byKind.set(3 /* Field */, Codicon.symbolField);
        byKind.set(4 /* Variable */, Codicon.symbolVariable);
        byKind.set(5 /* Class */, Codicon.symbolClass);
        byKind.set(6 /* Struct */, Codicon.symbolStruct);
        byKind.set(7 /* Interface */, Codicon.symbolInterface);
        byKind.set(8 /* Module */, Codicon.symbolModule);
        byKind.set(9 /* Property */, Codicon.symbolProperty);
        byKind.set(10 /* Event */, Codicon.symbolEvent);
        byKind.set(11 /* Operator */, Codicon.symbolOperator);
        byKind.set(12 /* Unit */, Codicon.symbolUnit);
        byKind.set(13 /* Value */, Codicon.symbolValue);
        byKind.set(15 /* Enum */, Codicon.symbolEnum);
        byKind.set(14 /* Constant */, Codicon.symbolConstant);
        byKind.set(15 /* Enum */, Codicon.symbolEnum);
        byKind.set(16 /* EnumMember */, Codicon.symbolEnumMember);
        byKind.set(17 /* Keyword */, Codicon.symbolKeyword);
        byKind.set(27 /* Snippet */, Codicon.symbolSnippet);
        byKind.set(18 /* Text */, Codicon.symbolText);
        byKind.set(19 /* Color */, Codicon.symbolColor);
        byKind.set(20 /* File */, Codicon.symbolFile);
        byKind.set(21 /* Reference */, Codicon.symbolReference);
        byKind.set(22 /* Customcolor */, Codicon.symbolCustomColor);
        byKind.set(23 /* Folder */, Codicon.symbolFolder);
        byKind.set(24 /* TypeParameter */, Codicon.symbolTypeParameter);
        byKind.set(25 /* User */, Codicon.account);
        byKind.set(26 /* Issue */, Codicon.issues);
        /**
         * @internal
         */
        function toIcon(kind) {
            let codicon = byKind.get(kind);
            if (!codicon) {
                console.info('No codicon found for CompletionItemKind ' + kind);
                codicon = Codicon.symbolProperty;
            }
            return codicon;
        }
        CompletionItemKinds.toIcon = toIcon;
        const data = new Map();
        data.set('method', 0 /* Method */);
        data.set('function', 1 /* Function */);
        data.set('constructor', 2 /* Constructor */);
        data.set('field', 3 /* Field */);
        data.set('variable', 4 /* Variable */);
        data.set('class', 5 /* Class */);
        data.set('struct', 6 /* Struct */);
        data.set('interface', 7 /* Interface */);
        data.set('module', 8 /* Module */);
        data.set('property', 9 /* Property */);
        data.set('event', 10 /* Event */);
        data.set('operator', 11 /* Operator */);
        data.set('unit', 12 /* Unit */);
        data.set('value', 13 /* Value */);
        data.set('constant', 14 /* Constant */);
        data.set('enum', 15 /* Enum */);
        data.set('enum-member', 16 /* EnumMember */);
        data.set('enumMember', 16 /* EnumMember */);
        data.set('keyword', 17 /* Keyword */);
        data.set('snippet', 27 /* Snippet */);
        data.set('text', 18 /* Text */);
        data.set('color', 19 /* Color */);
        data.set('file', 20 /* File */);
        data.set('reference', 21 /* Reference */);
        data.set('customcolor', 22 /* Customcolor */);
        data.set('folder', 23 /* Folder */);
        data.set('type-parameter', 24 /* TypeParameter */);
        data.set('typeParameter', 24 /* TypeParameter */);
        data.set('account', 25 /* User */);
        data.set('issue', 26 /* Issue */);
        /**
         * @internal
         */
        function fromString(value, strict) {
            let res = data.get(value);
            if (typeof res === 'undefined' && !strict) {
                res = 9 /* Property */;
            }
            return res;
        }
        CompletionItemKinds.fromString = fromString;
    })(CompletionItemKinds || (CompletionItemKinds = {}));
    /**
     * How an {@link InlineCompletionsProvider inline completion provider} was triggered.
     */
    var InlineCompletionTriggerKind$1;
    (function (InlineCompletionTriggerKind) {
        /**
         * Completion was triggered automatically while editing.
         * It is sufficient to return a single completion item in this case.
         */
        InlineCompletionTriggerKind[InlineCompletionTriggerKind["Automatic"] = 0] = "Automatic";
        /**
         * Completion was triggered explicitly by a user gesture.
         * Return multiple completion items to enable cycling through them.
         */
        InlineCompletionTriggerKind[InlineCompletionTriggerKind["Explicit"] = 1] = "Explicit";
    })(InlineCompletionTriggerKind$1 || (InlineCompletionTriggerKind$1 = {}));
    var SignatureHelpTriggerKind$1;
    (function (SignatureHelpTriggerKind) {
        SignatureHelpTriggerKind[SignatureHelpTriggerKind["Invoke"] = 1] = "Invoke";
        SignatureHelpTriggerKind[SignatureHelpTriggerKind["TriggerCharacter"] = 2] = "TriggerCharacter";
        SignatureHelpTriggerKind[SignatureHelpTriggerKind["ContentChange"] = 3] = "ContentChange";
    })(SignatureHelpTriggerKind$1 || (SignatureHelpTriggerKind$1 = {}));
    /**
     * A document highlight kind.
     */
    var DocumentHighlightKind$1;
    (function (DocumentHighlightKind) {
        /**
         * A textual occurrence.
         */
        DocumentHighlightKind[DocumentHighlightKind["Text"] = 0] = "Text";
        /**
         * Read-access of a symbol, like reading a variable.
         */
        DocumentHighlightKind[DocumentHighlightKind["Read"] = 1] = "Read";
        /**
         * Write-access of a symbol, like writing to a variable.
         */
        DocumentHighlightKind[DocumentHighlightKind["Write"] = 2] = "Write";
    })(DocumentHighlightKind$1 || (DocumentHighlightKind$1 = {}));
    /**
     * @internal
     */
    var SymbolKinds;
    (function (SymbolKinds) {
        const byKind = new Map();
        byKind.set(0 /* File */, Codicon.symbolFile);
        byKind.set(1 /* Module */, Codicon.symbolModule);
        byKind.set(2 /* Namespace */, Codicon.symbolNamespace);
        byKind.set(3 /* Package */, Codicon.symbolPackage);
        byKind.set(4 /* Class */, Codicon.symbolClass);
        byKind.set(5 /* Method */, Codicon.symbolMethod);
        byKind.set(6 /* Property */, Codicon.symbolProperty);
        byKind.set(7 /* Field */, Codicon.symbolField);
        byKind.set(8 /* Constructor */, Codicon.symbolConstructor);
        byKind.set(9 /* Enum */, Codicon.symbolEnum);
        byKind.set(10 /* Interface */, Codicon.symbolInterface);
        byKind.set(11 /* Function */, Codicon.symbolFunction);
        byKind.set(12 /* Variable */, Codicon.symbolVariable);
        byKind.set(13 /* Constant */, Codicon.symbolConstant);
        byKind.set(14 /* String */, Codicon.symbolString);
        byKind.set(15 /* Number */, Codicon.symbolNumber);
        byKind.set(16 /* Boolean */, Codicon.symbolBoolean);
        byKind.set(17 /* Array */, Codicon.symbolArray);
        byKind.set(18 /* Object */, Codicon.symbolObject);
        byKind.set(19 /* Key */, Codicon.symbolKey);
        byKind.set(20 /* Null */, Codicon.symbolNull);
        byKind.set(21 /* EnumMember */, Codicon.symbolEnumMember);
        byKind.set(22 /* Struct */, Codicon.symbolStruct);
        byKind.set(23 /* Event */, Codicon.symbolEvent);
        byKind.set(24 /* Operator */, Codicon.symbolOperator);
        byKind.set(25 /* TypeParameter */, Codicon.symbolTypeParameter);
        /**
         * @internal
         */
        function toIcon(kind) {
            let icon = byKind.get(kind);
            if (!icon) {
                console.info('No codicon found for SymbolKind ' + kind);
                icon = Codicon.symbolProperty;
            }
            return icon;
        }
        SymbolKinds.toIcon = toIcon;
    })(SymbolKinds || (SymbolKinds = {}));
    /**
     * @internal
     */
    var Command;
    (function (Command) {
        /**
         * @internal
         */
        function is(obj) {
            if (!obj || typeof obj !== 'object') {
                return false;
            }
            return typeof obj.id === 'string' &&
                typeof obj.title === 'string';
        }
        Command.is = is;
    })(Command || (Command = {}));
    var InlayHintKind$1;
    (function (InlayHintKind) {
        InlayHintKind[InlayHintKind["Other"] = 0] = "Other";
        InlayHintKind[InlayHintKind["Type"] = 1] = "Type";
        InlayHintKind[InlayHintKind["Parameter"] = 2] = "Parameter";
    })(InlayHintKind$1 || (InlayHintKind$1 = {}));
    // --- feature registries ------
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new LanguageFeatureRegistry();
    /**
     * @internal
     */
    new TokenizationRegistry();

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    // THIS IS A GENERATED FILE. DO NOT EDIT DIRECTLY.
    var AccessibilitySupport;
    (function (AccessibilitySupport) {
        /**
         * This should be the browser case where it is not known if a screen reader is attached or no.
         */
        AccessibilitySupport[AccessibilitySupport["Unknown"] = 0] = "Unknown";
        AccessibilitySupport[AccessibilitySupport["Disabled"] = 1] = "Disabled";
        AccessibilitySupport[AccessibilitySupport["Enabled"] = 2] = "Enabled";
    })(AccessibilitySupport || (AccessibilitySupport = {}));
    var CompletionItemInsertTextRule;
    (function (CompletionItemInsertTextRule) {
        /**
         * Adjust whitespace/indentation of multiline insert texts to
         * match the current line indentation.
         */
        CompletionItemInsertTextRule[CompletionItemInsertTextRule["KeepWhitespace"] = 1] = "KeepWhitespace";
        /**
         * `insertText` is a snippet.
         */
        CompletionItemInsertTextRule[CompletionItemInsertTextRule["InsertAsSnippet"] = 4] = "InsertAsSnippet";
    })(CompletionItemInsertTextRule || (CompletionItemInsertTextRule = {}));
    var CompletionItemKind;
    (function (CompletionItemKind) {
        CompletionItemKind[CompletionItemKind["Method"] = 0] = "Method";
        CompletionItemKind[CompletionItemKind["Function"] = 1] = "Function";
        CompletionItemKind[CompletionItemKind["Constructor"] = 2] = "Constructor";
        CompletionItemKind[CompletionItemKind["Field"] = 3] = "Field";
        CompletionItemKind[CompletionItemKind["Variable"] = 4] = "Variable";
        CompletionItemKind[CompletionItemKind["Class"] = 5] = "Class";
        CompletionItemKind[CompletionItemKind["Struct"] = 6] = "Struct";
        CompletionItemKind[CompletionItemKind["Interface"] = 7] = "Interface";
        CompletionItemKind[CompletionItemKind["Module"] = 8] = "Module";
        CompletionItemKind[CompletionItemKind["Property"] = 9] = "Property";
        CompletionItemKind[CompletionItemKind["Event"] = 10] = "Event";
        CompletionItemKind[CompletionItemKind["Operator"] = 11] = "Operator";
        CompletionItemKind[CompletionItemKind["Unit"] = 12] = "Unit";
        CompletionItemKind[CompletionItemKind["Value"] = 13] = "Value";
        CompletionItemKind[CompletionItemKind["Constant"] = 14] = "Constant";
        CompletionItemKind[CompletionItemKind["Enum"] = 15] = "Enum";
        CompletionItemKind[CompletionItemKind["EnumMember"] = 16] = "EnumMember";
        CompletionItemKind[CompletionItemKind["Keyword"] = 17] = "Keyword";
        CompletionItemKind[CompletionItemKind["Text"] = 18] = "Text";
        CompletionItemKind[CompletionItemKind["Color"] = 19] = "Color";
        CompletionItemKind[CompletionItemKind["File"] = 20] = "File";
        CompletionItemKind[CompletionItemKind["Reference"] = 21] = "Reference";
        CompletionItemKind[CompletionItemKind["Customcolor"] = 22] = "Customcolor";
        CompletionItemKind[CompletionItemKind["Folder"] = 23] = "Folder";
        CompletionItemKind[CompletionItemKind["TypeParameter"] = 24] = "TypeParameter";
        CompletionItemKind[CompletionItemKind["User"] = 25] = "User";
        CompletionItemKind[CompletionItemKind["Issue"] = 26] = "Issue";
        CompletionItemKind[CompletionItemKind["Snippet"] = 27] = "Snippet";
    })(CompletionItemKind || (CompletionItemKind = {}));
    var CompletionItemTag;
    (function (CompletionItemTag) {
        CompletionItemTag[CompletionItemTag["Deprecated"] = 1] = "Deprecated";
    })(CompletionItemTag || (CompletionItemTag = {}));
    /**
     * How a suggest provider was triggered.
     */
    var CompletionTriggerKind;
    (function (CompletionTriggerKind) {
        CompletionTriggerKind[CompletionTriggerKind["Invoke"] = 0] = "Invoke";
        CompletionTriggerKind[CompletionTriggerKind["TriggerCharacter"] = 1] = "TriggerCharacter";
        CompletionTriggerKind[CompletionTriggerKind["TriggerForIncompleteCompletions"] = 2] = "TriggerForIncompleteCompletions";
    })(CompletionTriggerKind || (CompletionTriggerKind = {}));
    /**
     * A positioning preference for rendering content widgets.
     */
    var ContentWidgetPositionPreference;
    (function (ContentWidgetPositionPreference) {
        /**
         * Place the content widget exactly at a position
         */
        ContentWidgetPositionPreference[ContentWidgetPositionPreference["EXACT"] = 0] = "EXACT";
        /**
         * Place the content widget above a position
         */
        ContentWidgetPositionPreference[ContentWidgetPositionPreference["ABOVE"] = 1] = "ABOVE";
        /**
         * Place the content widget below a position
         */
        ContentWidgetPositionPreference[ContentWidgetPositionPreference["BELOW"] = 2] = "BELOW";
    })(ContentWidgetPositionPreference || (ContentWidgetPositionPreference = {}));
    /**
     * Describes the reason the cursor has changed its position.
     */
    var CursorChangeReason;
    (function (CursorChangeReason) {
        /**
         * Unknown or not set.
         */
        CursorChangeReason[CursorChangeReason["NotSet"] = 0] = "NotSet";
        /**
         * A `model.setValue()` was called.
         */
        CursorChangeReason[CursorChangeReason["ContentFlush"] = 1] = "ContentFlush";
        /**
         * The `model` has been changed outside of this cursor and the cursor recovers its position from associated markers.
         */
        CursorChangeReason[CursorChangeReason["RecoverFromMarkers"] = 2] = "RecoverFromMarkers";
        /**
         * There was an explicit user gesture.
         */
        CursorChangeReason[CursorChangeReason["Explicit"] = 3] = "Explicit";
        /**
         * There was a Paste.
         */
        CursorChangeReason[CursorChangeReason["Paste"] = 4] = "Paste";
        /**
         * There was an Undo.
         */
        CursorChangeReason[CursorChangeReason["Undo"] = 5] = "Undo";
        /**
         * There was a Redo.
         */
        CursorChangeReason[CursorChangeReason["Redo"] = 6] = "Redo";
    })(CursorChangeReason || (CursorChangeReason = {}));
    /**
     * The default end of line to use when instantiating models.
     */
    var DefaultEndOfLine;
    (function (DefaultEndOfLine) {
        /**
         * Use line feed (\n) as the end of line character.
         */
        DefaultEndOfLine[DefaultEndOfLine["LF"] = 1] = "LF";
        /**
         * Use carriage return and line feed (\r\n) as the end of line character.
         */
        DefaultEndOfLine[DefaultEndOfLine["CRLF"] = 2] = "CRLF";
    })(DefaultEndOfLine || (DefaultEndOfLine = {}));
    /**
     * A document highlight kind.
     */
    var DocumentHighlightKind;
    (function (DocumentHighlightKind) {
        /**
         * A textual occurrence.
         */
        DocumentHighlightKind[DocumentHighlightKind["Text"] = 0] = "Text";
        /**
         * Read-access of a symbol, like reading a variable.
         */
        DocumentHighlightKind[DocumentHighlightKind["Read"] = 1] = "Read";
        /**
         * Write-access of a symbol, like writing to a variable.
         */
        DocumentHighlightKind[DocumentHighlightKind["Write"] = 2] = "Write";
    })(DocumentHighlightKind || (DocumentHighlightKind = {}));
    /**
     * Configuration options for auto indentation in the editor
     */
    var EditorAutoIndentStrategy;
    (function (EditorAutoIndentStrategy) {
        EditorAutoIndentStrategy[EditorAutoIndentStrategy["None"] = 0] = "None";
        EditorAutoIndentStrategy[EditorAutoIndentStrategy["Keep"] = 1] = "Keep";
        EditorAutoIndentStrategy[EditorAutoIndentStrategy["Brackets"] = 2] = "Brackets";
        EditorAutoIndentStrategy[EditorAutoIndentStrategy["Advanced"] = 3] = "Advanced";
        EditorAutoIndentStrategy[EditorAutoIndentStrategy["Full"] = 4] = "Full";
    })(EditorAutoIndentStrategy || (EditorAutoIndentStrategy = {}));
    var EditorOption;
    (function (EditorOption) {
        EditorOption[EditorOption["acceptSuggestionOnCommitCharacter"] = 0] = "acceptSuggestionOnCommitCharacter";
        EditorOption[EditorOption["acceptSuggestionOnEnter"] = 1] = "acceptSuggestionOnEnter";
        EditorOption[EditorOption["accessibilitySupport"] = 2] = "accessibilitySupport";
        EditorOption[EditorOption["accessibilityPageSize"] = 3] = "accessibilityPageSize";
        EditorOption[EditorOption["ariaLabel"] = 4] = "ariaLabel";
        EditorOption[EditorOption["autoClosingBrackets"] = 5] = "autoClosingBrackets";
        EditorOption[EditorOption["autoClosingDelete"] = 6] = "autoClosingDelete";
        EditorOption[EditorOption["autoClosingOvertype"] = 7] = "autoClosingOvertype";
        EditorOption[EditorOption["autoClosingQuotes"] = 8] = "autoClosingQuotes";
        EditorOption[EditorOption["autoIndent"] = 9] = "autoIndent";
        EditorOption[EditorOption["automaticLayout"] = 10] = "automaticLayout";
        EditorOption[EditorOption["autoSurround"] = 11] = "autoSurround";
        EditorOption[EditorOption["bracketPairColorization"] = 12] = "bracketPairColorization";
        EditorOption[EditorOption["guides"] = 13] = "guides";
        EditorOption[EditorOption["codeLens"] = 14] = "codeLens";
        EditorOption[EditorOption["codeLensFontFamily"] = 15] = "codeLensFontFamily";
        EditorOption[EditorOption["codeLensFontSize"] = 16] = "codeLensFontSize";
        EditorOption[EditorOption["colorDecorators"] = 17] = "colorDecorators";
        EditorOption[EditorOption["columnSelection"] = 18] = "columnSelection";
        EditorOption[EditorOption["comments"] = 19] = "comments";
        EditorOption[EditorOption["contextmenu"] = 20] = "contextmenu";
        EditorOption[EditorOption["copyWithSyntaxHighlighting"] = 21] = "copyWithSyntaxHighlighting";
        EditorOption[EditorOption["cursorBlinking"] = 22] = "cursorBlinking";
        EditorOption[EditorOption["cursorSmoothCaretAnimation"] = 23] = "cursorSmoothCaretAnimation";
        EditorOption[EditorOption["cursorStyle"] = 24] = "cursorStyle";
        EditorOption[EditorOption["cursorSurroundingLines"] = 25] = "cursorSurroundingLines";
        EditorOption[EditorOption["cursorSurroundingLinesStyle"] = 26] = "cursorSurroundingLinesStyle";
        EditorOption[EditorOption["cursorWidth"] = 27] = "cursorWidth";
        EditorOption[EditorOption["disableLayerHinting"] = 28] = "disableLayerHinting";
        EditorOption[EditorOption["disableMonospaceOptimizations"] = 29] = "disableMonospaceOptimizations";
        EditorOption[EditorOption["domReadOnly"] = 30] = "domReadOnly";
        EditorOption[EditorOption["dragAndDrop"] = 31] = "dragAndDrop";
        EditorOption[EditorOption["emptySelectionClipboard"] = 32] = "emptySelectionClipboard";
        EditorOption[EditorOption["extraEditorClassName"] = 33] = "extraEditorClassName";
        EditorOption[EditorOption["fastScrollSensitivity"] = 34] = "fastScrollSensitivity";
        EditorOption[EditorOption["find"] = 35] = "find";
        EditorOption[EditorOption["fixedOverflowWidgets"] = 36] = "fixedOverflowWidgets";
        EditorOption[EditorOption["folding"] = 37] = "folding";
        EditorOption[EditorOption["foldingStrategy"] = 38] = "foldingStrategy";
        EditorOption[EditorOption["foldingHighlight"] = 39] = "foldingHighlight";
        EditorOption[EditorOption["foldingImportsByDefault"] = 40] = "foldingImportsByDefault";
        EditorOption[EditorOption["foldingMaximumRegions"] = 41] = "foldingMaximumRegions";
        EditorOption[EditorOption["unfoldOnClickAfterEndOfLine"] = 42] = "unfoldOnClickAfterEndOfLine";
        EditorOption[EditorOption["fontFamily"] = 43] = "fontFamily";
        EditorOption[EditorOption["fontInfo"] = 44] = "fontInfo";
        EditorOption[EditorOption["fontLigatures"] = 45] = "fontLigatures";
        EditorOption[EditorOption["fontSize"] = 46] = "fontSize";
        EditorOption[EditorOption["fontWeight"] = 47] = "fontWeight";
        EditorOption[EditorOption["formatOnPaste"] = 48] = "formatOnPaste";
        EditorOption[EditorOption["formatOnType"] = 49] = "formatOnType";
        EditorOption[EditorOption["glyphMargin"] = 50] = "glyphMargin";
        EditorOption[EditorOption["gotoLocation"] = 51] = "gotoLocation";
        EditorOption[EditorOption["hideCursorInOverviewRuler"] = 52] = "hideCursorInOverviewRuler";
        EditorOption[EditorOption["hover"] = 53] = "hover";
        EditorOption[EditorOption["inDiffEditor"] = 54] = "inDiffEditor";
        EditorOption[EditorOption["inlineSuggest"] = 55] = "inlineSuggest";
        EditorOption[EditorOption["letterSpacing"] = 56] = "letterSpacing";
        EditorOption[EditorOption["lightbulb"] = 57] = "lightbulb";
        EditorOption[EditorOption["lineDecorationsWidth"] = 58] = "lineDecorationsWidth";
        EditorOption[EditorOption["lineHeight"] = 59] = "lineHeight";
        EditorOption[EditorOption["lineNumbers"] = 60] = "lineNumbers";
        EditorOption[EditorOption["lineNumbersMinChars"] = 61] = "lineNumbersMinChars";
        EditorOption[EditorOption["linkedEditing"] = 62] = "linkedEditing";
        EditorOption[EditorOption["links"] = 63] = "links";
        EditorOption[EditorOption["matchBrackets"] = 64] = "matchBrackets";
        EditorOption[EditorOption["minimap"] = 65] = "minimap";
        EditorOption[EditorOption["mouseStyle"] = 66] = "mouseStyle";
        EditorOption[EditorOption["mouseWheelScrollSensitivity"] = 67] = "mouseWheelScrollSensitivity";
        EditorOption[EditorOption["mouseWheelZoom"] = 68] = "mouseWheelZoom";
        EditorOption[EditorOption["multiCursorMergeOverlapping"] = 69] = "multiCursorMergeOverlapping";
        EditorOption[EditorOption["multiCursorModifier"] = 70] = "multiCursorModifier";
        EditorOption[EditorOption["multiCursorPaste"] = 71] = "multiCursorPaste";
        EditorOption[EditorOption["occurrencesHighlight"] = 72] = "occurrencesHighlight";
        EditorOption[EditorOption["overviewRulerBorder"] = 73] = "overviewRulerBorder";
        EditorOption[EditorOption["overviewRulerLanes"] = 74] = "overviewRulerLanes";
        EditorOption[EditorOption["padding"] = 75] = "padding";
        EditorOption[EditorOption["parameterHints"] = 76] = "parameterHints";
        EditorOption[EditorOption["peekWidgetDefaultFocus"] = 77] = "peekWidgetDefaultFocus";
        EditorOption[EditorOption["definitionLinkOpensInPeek"] = 78] = "definitionLinkOpensInPeek";
        EditorOption[EditorOption["quickSuggestions"] = 79] = "quickSuggestions";
        EditorOption[EditorOption["quickSuggestionsDelay"] = 80] = "quickSuggestionsDelay";
        EditorOption[EditorOption["readOnly"] = 81] = "readOnly";
        EditorOption[EditorOption["renameOnType"] = 82] = "renameOnType";
        EditorOption[EditorOption["renderControlCharacters"] = 83] = "renderControlCharacters";
        EditorOption[EditorOption["renderFinalNewline"] = 84] = "renderFinalNewline";
        EditorOption[EditorOption["renderLineHighlight"] = 85] = "renderLineHighlight";
        EditorOption[EditorOption["renderLineHighlightOnlyWhenFocus"] = 86] = "renderLineHighlightOnlyWhenFocus";
        EditorOption[EditorOption["renderValidationDecorations"] = 87] = "renderValidationDecorations";
        EditorOption[EditorOption["renderWhitespace"] = 88] = "renderWhitespace";
        EditorOption[EditorOption["revealHorizontalRightPadding"] = 89] = "revealHorizontalRightPadding";
        EditorOption[EditorOption["roundedSelection"] = 90] = "roundedSelection";
        EditorOption[EditorOption["rulers"] = 91] = "rulers";
        EditorOption[EditorOption["scrollbar"] = 92] = "scrollbar";
        EditorOption[EditorOption["scrollBeyondLastColumn"] = 93] = "scrollBeyondLastColumn";
        EditorOption[EditorOption["scrollBeyondLastLine"] = 94] = "scrollBeyondLastLine";
        EditorOption[EditorOption["scrollPredominantAxis"] = 95] = "scrollPredominantAxis";
        EditorOption[EditorOption["selectionClipboard"] = 96] = "selectionClipboard";
        EditorOption[EditorOption["selectionHighlight"] = 97] = "selectionHighlight";
        EditorOption[EditorOption["selectOnLineNumbers"] = 98] = "selectOnLineNumbers";
        EditorOption[EditorOption["showFoldingControls"] = 99] = "showFoldingControls";
        EditorOption[EditorOption["showUnused"] = 100] = "showUnused";
        EditorOption[EditorOption["snippetSuggestions"] = 101] = "snippetSuggestions";
        EditorOption[EditorOption["smartSelect"] = 102] = "smartSelect";
        EditorOption[EditorOption["smoothScrolling"] = 103] = "smoothScrolling";
        EditorOption[EditorOption["stickyTabStops"] = 104] = "stickyTabStops";
        EditorOption[EditorOption["stopRenderingLineAfter"] = 105] = "stopRenderingLineAfter";
        EditorOption[EditorOption["suggest"] = 106] = "suggest";
        EditorOption[EditorOption["suggestFontSize"] = 107] = "suggestFontSize";
        EditorOption[EditorOption["suggestLineHeight"] = 108] = "suggestLineHeight";
        EditorOption[EditorOption["suggestOnTriggerCharacters"] = 109] = "suggestOnTriggerCharacters";
        EditorOption[EditorOption["suggestSelection"] = 110] = "suggestSelection";
        EditorOption[EditorOption["tabCompletion"] = 111] = "tabCompletion";
        EditorOption[EditorOption["tabIndex"] = 112] = "tabIndex";
        EditorOption[EditorOption["unicodeHighlighting"] = 113] = "unicodeHighlighting";
        EditorOption[EditorOption["unusualLineTerminators"] = 114] = "unusualLineTerminators";
        EditorOption[EditorOption["useShadowDOM"] = 115] = "useShadowDOM";
        EditorOption[EditorOption["useTabStops"] = 116] = "useTabStops";
        EditorOption[EditorOption["wordSeparators"] = 117] = "wordSeparators";
        EditorOption[EditorOption["wordWrap"] = 118] = "wordWrap";
        EditorOption[EditorOption["wordWrapBreakAfterCharacters"] = 119] = "wordWrapBreakAfterCharacters";
        EditorOption[EditorOption["wordWrapBreakBeforeCharacters"] = 120] = "wordWrapBreakBeforeCharacters";
        EditorOption[EditorOption["wordWrapColumn"] = 121] = "wordWrapColumn";
        EditorOption[EditorOption["wordWrapOverride1"] = 122] = "wordWrapOverride1";
        EditorOption[EditorOption["wordWrapOverride2"] = 123] = "wordWrapOverride2";
        EditorOption[EditorOption["wrappingIndent"] = 124] = "wrappingIndent";
        EditorOption[EditorOption["wrappingStrategy"] = 125] = "wrappingStrategy";
        EditorOption[EditorOption["showDeprecated"] = 126] = "showDeprecated";
        EditorOption[EditorOption["inlayHints"] = 127] = "inlayHints";
        EditorOption[EditorOption["editorClassName"] = 128] = "editorClassName";
        EditorOption[EditorOption["pixelRatio"] = 129] = "pixelRatio";
        EditorOption[EditorOption["tabFocusMode"] = 130] = "tabFocusMode";
        EditorOption[EditorOption["layoutInfo"] = 131] = "layoutInfo";
        EditorOption[EditorOption["wrappingInfo"] = 132] = "wrappingInfo";
    })(EditorOption || (EditorOption = {}));
    /**
     * End of line character preference.
     */
    var EndOfLinePreference;
    (function (EndOfLinePreference) {
        /**
         * Use the end of line character identified in the text buffer.
         */
        EndOfLinePreference[EndOfLinePreference["TextDefined"] = 0] = "TextDefined";
        /**
         * Use line feed (\n) as the end of line character.
         */
        EndOfLinePreference[EndOfLinePreference["LF"] = 1] = "LF";
        /**
         * Use carriage return and line feed (\r\n) as the end of line character.
         */
        EndOfLinePreference[EndOfLinePreference["CRLF"] = 2] = "CRLF";
    })(EndOfLinePreference || (EndOfLinePreference = {}));
    /**
     * End of line character preference.
     */
    var EndOfLineSequence;
    (function (EndOfLineSequence) {
        /**
         * Use line feed (\n) as the end of line character.
         */
        EndOfLineSequence[EndOfLineSequence["LF"] = 0] = "LF";
        /**
         * Use carriage return and line feed (\r\n) as the end of line character.
         */
        EndOfLineSequence[EndOfLineSequence["CRLF"] = 1] = "CRLF";
    })(EndOfLineSequence || (EndOfLineSequence = {}));
    /**
     * Describes what to do with the indentation when pressing Enter.
     */
    var IndentAction;
    (function (IndentAction) {
        /**
         * Insert new line and copy the previous line's indentation.
         */
        IndentAction[IndentAction["None"] = 0] = "None";
        /**
         * Insert new line and indent once (relative to the previous line's indentation).
         */
        IndentAction[IndentAction["Indent"] = 1] = "Indent";
        /**
         * Insert two new lines:
         *  - the first one indented which will hold the cursor
         *  - the second one at the same indentation level
         */
        IndentAction[IndentAction["IndentOutdent"] = 2] = "IndentOutdent";
        /**
         * Insert new line and outdent once (relative to the previous line's indentation).
         */
        IndentAction[IndentAction["Outdent"] = 3] = "Outdent";
    })(IndentAction || (IndentAction = {}));
    var InjectedTextCursorStops;
    (function (InjectedTextCursorStops) {
        InjectedTextCursorStops[InjectedTextCursorStops["Both"] = 0] = "Both";
        InjectedTextCursorStops[InjectedTextCursorStops["Right"] = 1] = "Right";
        InjectedTextCursorStops[InjectedTextCursorStops["Left"] = 2] = "Left";
        InjectedTextCursorStops[InjectedTextCursorStops["None"] = 3] = "None";
    })(InjectedTextCursorStops || (InjectedTextCursorStops = {}));
    var InlayHintKind;
    (function (InlayHintKind) {
        InlayHintKind[InlayHintKind["Other"] = 0] = "Other";
        InlayHintKind[InlayHintKind["Type"] = 1] = "Type";
        InlayHintKind[InlayHintKind["Parameter"] = 2] = "Parameter";
    })(InlayHintKind || (InlayHintKind = {}));
    /**
     * How an {@link InlineCompletionsProvider inline completion provider} was triggered.
     */
    var InlineCompletionTriggerKind;
    (function (InlineCompletionTriggerKind) {
        /**
         * Completion was triggered automatically while editing.
         * It is sufficient to return a single completion item in this case.
         */
        InlineCompletionTriggerKind[InlineCompletionTriggerKind["Automatic"] = 0] = "Automatic";
        /**
         * Completion was triggered explicitly by a user gesture.
         * Return multiple completion items to enable cycling through them.
         */
        InlineCompletionTriggerKind[InlineCompletionTriggerKind["Explicit"] = 1] = "Explicit";
    })(InlineCompletionTriggerKind || (InlineCompletionTriggerKind = {}));
    /**
     * Virtual Key Codes, the value does not hold any inherent meaning.
     * Inspired somewhat from https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx
     * But these are "more general", as they should work across browsers & OS`s.
     */
    var KeyCode;
    (function (KeyCode) {
        KeyCode[KeyCode["DependsOnKbLayout"] = -1] = "DependsOnKbLayout";
        /**
         * Placed first to cover the 0 value of the enum.
         */
        KeyCode[KeyCode["Unknown"] = 0] = "Unknown";
        KeyCode[KeyCode["Backspace"] = 1] = "Backspace";
        KeyCode[KeyCode["Tab"] = 2] = "Tab";
        KeyCode[KeyCode["Enter"] = 3] = "Enter";
        KeyCode[KeyCode["Shift"] = 4] = "Shift";
        KeyCode[KeyCode["Ctrl"] = 5] = "Ctrl";
        KeyCode[KeyCode["Alt"] = 6] = "Alt";
        KeyCode[KeyCode["PauseBreak"] = 7] = "PauseBreak";
        KeyCode[KeyCode["CapsLock"] = 8] = "CapsLock";
        KeyCode[KeyCode["Escape"] = 9] = "Escape";
        KeyCode[KeyCode["Space"] = 10] = "Space";
        KeyCode[KeyCode["PageUp"] = 11] = "PageUp";
        KeyCode[KeyCode["PageDown"] = 12] = "PageDown";
        KeyCode[KeyCode["End"] = 13] = "End";
        KeyCode[KeyCode["Home"] = 14] = "Home";
        KeyCode[KeyCode["LeftArrow"] = 15] = "LeftArrow";
        KeyCode[KeyCode["UpArrow"] = 16] = "UpArrow";
        KeyCode[KeyCode["RightArrow"] = 17] = "RightArrow";
        KeyCode[KeyCode["DownArrow"] = 18] = "DownArrow";
        KeyCode[KeyCode["Insert"] = 19] = "Insert";
        KeyCode[KeyCode["Delete"] = 20] = "Delete";
        KeyCode[KeyCode["Digit0"] = 21] = "Digit0";
        KeyCode[KeyCode["Digit1"] = 22] = "Digit1";
        KeyCode[KeyCode["Digit2"] = 23] = "Digit2";
        KeyCode[KeyCode["Digit3"] = 24] = "Digit3";
        KeyCode[KeyCode["Digit4"] = 25] = "Digit4";
        KeyCode[KeyCode["Digit5"] = 26] = "Digit5";
        KeyCode[KeyCode["Digit6"] = 27] = "Digit6";
        KeyCode[KeyCode["Digit7"] = 28] = "Digit7";
        KeyCode[KeyCode["Digit8"] = 29] = "Digit8";
        KeyCode[KeyCode["Digit9"] = 30] = "Digit9";
        KeyCode[KeyCode["KeyA"] = 31] = "KeyA";
        KeyCode[KeyCode["KeyB"] = 32] = "KeyB";
        KeyCode[KeyCode["KeyC"] = 33] = "KeyC";
        KeyCode[KeyCode["KeyD"] = 34] = "KeyD";
        KeyCode[KeyCode["KeyE"] = 35] = "KeyE";
        KeyCode[KeyCode["KeyF"] = 36] = "KeyF";
        KeyCode[KeyCode["KeyG"] = 37] = "KeyG";
        KeyCode[KeyCode["KeyH"] = 38] = "KeyH";
        KeyCode[KeyCode["KeyI"] = 39] = "KeyI";
        KeyCode[KeyCode["KeyJ"] = 40] = "KeyJ";
        KeyCode[KeyCode["KeyK"] = 41] = "KeyK";
        KeyCode[KeyCode["KeyL"] = 42] = "KeyL";
        KeyCode[KeyCode["KeyM"] = 43] = "KeyM";
        KeyCode[KeyCode["KeyN"] = 44] = "KeyN";
        KeyCode[KeyCode["KeyO"] = 45] = "KeyO";
        KeyCode[KeyCode["KeyP"] = 46] = "KeyP";
        KeyCode[KeyCode["KeyQ"] = 47] = "KeyQ";
        KeyCode[KeyCode["KeyR"] = 48] = "KeyR";
        KeyCode[KeyCode["KeyS"] = 49] = "KeyS";
        KeyCode[KeyCode["KeyT"] = 50] = "KeyT";
        KeyCode[KeyCode["KeyU"] = 51] = "KeyU";
        KeyCode[KeyCode["KeyV"] = 52] = "KeyV";
        KeyCode[KeyCode["KeyW"] = 53] = "KeyW";
        KeyCode[KeyCode["KeyX"] = 54] = "KeyX";
        KeyCode[KeyCode["KeyY"] = 55] = "KeyY";
        KeyCode[KeyCode["KeyZ"] = 56] = "KeyZ";
        KeyCode[KeyCode["Meta"] = 57] = "Meta";
        KeyCode[KeyCode["ContextMenu"] = 58] = "ContextMenu";
        KeyCode[KeyCode["F1"] = 59] = "F1";
        KeyCode[KeyCode["F2"] = 60] = "F2";
        KeyCode[KeyCode["F3"] = 61] = "F3";
        KeyCode[KeyCode["F4"] = 62] = "F4";
        KeyCode[KeyCode["F5"] = 63] = "F5";
        KeyCode[KeyCode["F6"] = 64] = "F6";
        KeyCode[KeyCode["F7"] = 65] = "F7";
        KeyCode[KeyCode["F8"] = 66] = "F8";
        KeyCode[KeyCode["F9"] = 67] = "F9";
        KeyCode[KeyCode["F10"] = 68] = "F10";
        KeyCode[KeyCode["F11"] = 69] = "F11";
        KeyCode[KeyCode["F12"] = 70] = "F12";
        KeyCode[KeyCode["F13"] = 71] = "F13";
        KeyCode[KeyCode["F14"] = 72] = "F14";
        KeyCode[KeyCode["F15"] = 73] = "F15";
        KeyCode[KeyCode["F16"] = 74] = "F16";
        KeyCode[KeyCode["F17"] = 75] = "F17";
        KeyCode[KeyCode["F18"] = 76] = "F18";
        KeyCode[KeyCode["F19"] = 77] = "F19";
        KeyCode[KeyCode["NumLock"] = 78] = "NumLock";
        KeyCode[KeyCode["ScrollLock"] = 79] = "ScrollLock";
        /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the ';:' key
         */
        KeyCode[KeyCode["Semicolon"] = 80] = "Semicolon";
        /**
         * For any country/region, the '+' key
         * For the US standard keyboard, the '=+' key
         */
        KeyCode[KeyCode["Equal"] = 81] = "Equal";
        /**
         * For any country/region, the ',' key
         * For the US standard keyboard, the ',<' key
         */
        KeyCode[KeyCode["Comma"] = 82] = "Comma";
        /**
         * For any country/region, the '-' key
         * For the US standard keyboard, the '-_' key
         */
        KeyCode[KeyCode["Minus"] = 83] = "Minus";
        /**
         * For any country/region, the '.' key
         * For the US standard keyboard, the '.>' key
         */
        KeyCode[KeyCode["Period"] = 84] = "Period";
        /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the '/?' key
         */
        KeyCode[KeyCode["Slash"] = 85] = "Slash";
        /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the '`~' key
         */
        KeyCode[KeyCode["Backquote"] = 86] = "Backquote";
        /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the '[{' key
         */
        KeyCode[KeyCode["BracketLeft"] = 87] = "BracketLeft";
        /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the '\|' key
         */
        KeyCode[KeyCode["Backslash"] = 88] = "Backslash";
        /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the ']}' key
         */
        KeyCode[KeyCode["BracketRight"] = 89] = "BracketRight";
        /**
         * Used for miscellaneous characters; it can vary by keyboard.
         * For the US standard keyboard, the ''"' key
         */
        KeyCode[KeyCode["Quote"] = 90] = "Quote";
        /**
         * Used for miscellaneous characters; it can vary by keyboard.
         */
        KeyCode[KeyCode["OEM_8"] = 91] = "OEM_8";
        /**
         * Either the angle bracket key or the backslash key on the RT 102-key keyboard.
         */
        KeyCode[KeyCode["IntlBackslash"] = 92] = "IntlBackslash";
        KeyCode[KeyCode["Numpad0"] = 93] = "Numpad0";
        KeyCode[KeyCode["Numpad1"] = 94] = "Numpad1";
        KeyCode[KeyCode["Numpad2"] = 95] = "Numpad2";
        KeyCode[KeyCode["Numpad3"] = 96] = "Numpad3";
        KeyCode[KeyCode["Numpad4"] = 97] = "Numpad4";
        KeyCode[KeyCode["Numpad5"] = 98] = "Numpad5";
        KeyCode[KeyCode["Numpad6"] = 99] = "Numpad6";
        KeyCode[KeyCode["Numpad7"] = 100] = "Numpad7";
        KeyCode[KeyCode["Numpad8"] = 101] = "Numpad8";
        KeyCode[KeyCode["Numpad9"] = 102] = "Numpad9";
        KeyCode[KeyCode["NumpadMultiply"] = 103] = "NumpadMultiply";
        KeyCode[KeyCode["NumpadAdd"] = 104] = "NumpadAdd";
        KeyCode[KeyCode["NUMPAD_SEPARATOR"] = 105] = "NUMPAD_SEPARATOR";
        KeyCode[KeyCode["NumpadSubtract"] = 106] = "NumpadSubtract";
        KeyCode[KeyCode["NumpadDecimal"] = 107] = "NumpadDecimal";
        KeyCode[KeyCode["NumpadDivide"] = 108] = "NumpadDivide";
        /**
         * Cover all key codes when IME is processing input.
         */
        KeyCode[KeyCode["KEY_IN_COMPOSITION"] = 109] = "KEY_IN_COMPOSITION";
        KeyCode[KeyCode["ABNT_C1"] = 110] = "ABNT_C1";
        KeyCode[KeyCode["ABNT_C2"] = 111] = "ABNT_C2";
        KeyCode[KeyCode["AudioVolumeMute"] = 112] = "AudioVolumeMute";
        KeyCode[KeyCode["AudioVolumeUp"] = 113] = "AudioVolumeUp";
        KeyCode[KeyCode["AudioVolumeDown"] = 114] = "AudioVolumeDown";
        KeyCode[KeyCode["BrowserSearch"] = 115] = "BrowserSearch";
        KeyCode[KeyCode["BrowserHome"] = 116] = "BrowserHome";
        KeyCode[KeyCode["BrowserBack"] = 117] = "BrowserBack";
        KeyCode[KeyCode["BrowserForward"] = 118] = "BrowserForward";
        KeyCode[KeyCode["MediaTrackNext"] = 119] = "MediaTrackNext";
        KeyCode[KeyCode["MediaTrackPrevious"] = 120] = "MediaTrackPrevious";
        KeyCode[KeyCode["MediaStop"] = 121] = "MediaStop";
        KeyCode[KeyCode["MediaPlayPause"] = 122] = "MediaPlayPause";
        KeyCode[KeyCode["LaunchMediaPlayer"] = 123] = "LaunchMediaPlayer";
        KeyCode[KeyCode["LaunchMail"] = 124] = "LaunchMail";
        KeyCode[KeyCode["LaunchApp2"] = 125] = "LaunchApp2";
        /**
         * Placed last to cover the length of the enum.
         * Please do not depend on this value!
         */
        KeyCode[KeyCode["MAX_VALUE"] = 126] = "MAX_VALUE";
    })(KeyCode || (KeyCode = {}));
    var MarkerSeverity;
    (function (MarkerSeverity) {
        MarkerSeverity[MarkerSeverity["Hint"] = 1] = "Hint";
        MarkerSeverity[MarkerSeverity["Info"] = 2] = "Info";
        MarkerSeverity[MarkerSeverity["Warning"] = 4] = "Warning";
        MarkerSeverity[MarkerSeverity["Error"] = 8] = "Error";
    })(MarkerSeverity || (MarkerSeverity = {}));
    var MarkerTag;
    (function (MarkerTag) {
        MarkerTag[MarkerTag["Unnecessary"] = 1] = "Unnecessary";
        MarkerTag[MarkerTag["Deprecated"] = 2] = "Deprecated";
    })(MarkerTag || (MarkerTag = {}));
    /**
     * Position in the minimap to render the decoration.
     */
    var MinimapPosition;
    (function (MinimapPosition) {
        MinimapPosition[MinimapPosition["Inline"] = 1] = "Inline";
        MinimapPosition[MinimapPosition["Gutter"] = 2] = "Gutter";
    })(MinimapPosition || (MinimapPosition = {}));
    /**
     * Type of hit element with the mouse in the editor.
     */
    var MouseTargetType;
    (function (MouseTargetType) {
        /**
         * Mouse is on top of an unknown element.
         */
        MouseTargetType[MouseTargetType["UNKNOWN"] = 0] = "UNKNOWN";
        /**
         * Mouse is on top of the textarea used for input.
         */
        MouseTargetType[MouseTargetType["TEXTAREA"] = 1] = "TEXTAREA";
        /**
         * Mouse is on top of the glyph margin
         */
        MouseTargetType[MouseTargetType["GUTTER_GLYPH_MARGIN"] = 2] = "GUTTER_GLYPH_MARGIN";
        /**
         * Mouse is on top of the line numbers
         */
        MouseTargetType[MouseTargetType["GUTTER_LINE_NUMBERS"] = 3] = "GUTTER_LINE_NUMBERS";
        /**
         * Mouse is on top of the line decorations
         */
        MouseTargetType[MouseTargetType["GUTTER_LINE_DECORATIONS"] = 4] = "GUTTER_LINE_DECORATIONS";
        /**
         * Mouse is on top of the whitespace left in the gutter by a view zone.
         */
        MouseTargetType[MouseTargetType["GUTTER_VIEW_ZONE"] = 5] = "GUTTER_VIEW_ZONE";
        /**
         * Mouse is on top of text in the content.
         */
        MouseTargetType[MouseTargetType["CONTENT_TEXT"] = 6] = "CONTENT_TEXT";
        /**
         * Mouse is on top of empty space in the content (e.g. after line text or below last line)
         */
        MouseTargetType[MouseTargetType["CONTENT_EMPTY"] = 7] = "CONTENT_EMPTY";
        /**
         * Mouse is on top of a view zone in the content.
         */
        MouseTargetType[MouseTargetType["CONTENT_VIEW_ZONE"] = 8] = "CONTENT_VIEW_ZONE";
        /**
         * Mouse is on top of a content widget.
         */
        MouseTargetType[MouseTargetType["CONTENT_WIDGET"] = 9] = "CONTENT_WIDGET";
        /**
         * Mouse is on top of the decorations overview ruler.
         */
        MouseTargetType[MouseTargetType["OVERVIEW_RULER"] = 10] = "OVERVIEW_RULER";
        /**
         * Mouse is on top of a scrollbar.
         */
        MouseTargetType[MouseTargetType["SCROLLBAR"] = 11] = "SCROLLBAR";
        /**
         * Mouse is on top of an overlay widget.
         */
        MouseTargetType[MouseTargetType["OVERLAY_WIDGET"] = 12] = "OVERLAY_WIDGET";
        /**
         * Mouse is outside of the editor.
         */
        MouseTargetType[MouseTargetType["OUTSIDE_EDITOR"] = 13] = "OUTSIDE_EDITOR";
    })(MouseTargetType || (MouseTargetType = {}));
    /**
     * A positioning preference for rendering overlay widgets.
     */
    var OverlayWidgetPositionPreference;
    (function (OverlayWidgetPositionPreference) {
        /**
         * Position the overlay widget in the top right corner
         */
        OverlayWidgetPositionPreference[OverlayWidgetPositionPreference["TOP_RIGHT_CORNER"] = 0] = "TOP_RIGHT_CORNER";
        /**
         * Position the overlay widget in the bottom right corner
         */
        OverlayWidgetPositionPreference[OverlayWidgetPositionPreference["BOTTOM_RIGHT_CORNER"] = 1] = "BOTTOM_RIGHT_CORNER";
        /**
         * Position the overlay widget in the top center
         */
        OverlayWidgetPositionPreference[OverlayWidgetPositionPreference["TOP_CENTER"] = 2] = "TOP_CENTER";
    })(OverlayWidgetPositionPreference || (OverlayWidgetPositionPreference = {}));
    /**
     * Vertical Lane in the overview ruler of the editor.
     */
    var OverviewRulerLane;
    (function (OverviewRulerLane) {
        OverviewRulerLane[OverviewRulerLane["Left"] = 1] = "Left";
        OverviewRulerLane[OverviewRulerLane["Center"] = 2] = "Center";
        OverviewRulerLane[OverviewRulerLane["Right"] = 4] = "Right";
        OverviewRulerLane[OverviewRulerLane["Full"] = 7] = "Full";
    })(OverviewRulerLane || (OverviewRulerLane = {}));
    var PositionAffinity;
    (function (PositionAffinity) {
        /**
         * Prefers the left most position.
        */
        PositionAffinity[PositionAffinity["Left"] = 0] = "Left";
        /**
         * Prefers the right most position.
        */
        PositionAffinity[PositionAffinity["Right"] = 1] = "Right";
        /**
         * No preference.
        */
        PositionAffinity[PositionAffinity["None"] = 2] = "None";
    })(PositionAffinity || (PositionAffinity = {}));
    var RenderLineNumbersType;
    (function (RenderLineNumbersType) {
        RenderLineNumbersType[RenderLineNumbersType["Off"] = 0] = "Off";
        RenderLineNumbersType[RenderLineNumbersType["On"] = 1] = "On";
        RenderLineNumbersType[RenderLineNumbersType["Relative"] = 2] = "Relative";
        RenderLineNumbersType[RenderLineNumbersType["Interval"] = 3] = "Interval";
        RenderLineNumbersType[RenderLineNumbersType["Custom"] = 4] = "Custom";
    })(RenderLineNumbersType || (RenderLineNumbersType = {}));
    var RenderMinimap;
    (function (RenderMinimap) {
        RenderMinimap[RenderMinimap["None"] = 0] = "None";
        RenderMinimap[RenderMinimap["Text"] = 1] = "Text";
        RenderMinimap[RenderMinimap["Blocks"] = 2] = "Blocks";
    })(RenderMinimap || (RenderMinimap = {}));
    var ScrollType;
    (function (ScrollType) {
        ScrollType[ScrollType["Smooth"] = 0] = "Smooth";
        ScrollType[ScrollType["Immediate"] = 1] = "Immediate";
    })(ScrollType || (ScrollType = {}));
    var ScrollbarVisibility;
    (function (ScrollbarVisibility) {
        ScrollbarVisibility[ScrollbarVisibility["Auto"] = 1] = "Auto";
        ScrollbarVisibility[ScrollbarVisibility["Hidden"] = 2] = "Hidden";
        ScrollbarVisibility[ScrollbarVisibility["Visible"] = 3] = "Visible";
    })(ScrollbarVisibility || (ScrollbarVisibility = {}));
    /**
     * The direction of a selection.
     */
    var SelectionDirection;
    (function (SelectionDirection) {
        /**
         * The selection starts above where it ends.
         */
        SelectionDirection[SelectionDirection["LTR"] = 0] = "LTR";
        /**
         * The selection starts below where it ends.
         */
        SelectionDirection[SelectionDirection["RTL"] = 1] = "RTL";
    })(SelectionDirection || (SelectionDirection = {}));
    var SignatureHelpTriggerKind;
    (function (SignatureHelpTriggerKind) {
        SignatureHelpTriggerKind[SignatureHelpTriggerKind["Invoke"] = 1] = "Invoke";
        SignatureHelpTriggerKind[SignatureHelpTriggerKind["TriggerCharacter"] = 2] = "TriggerCharacter";
        SignatureHelpTriggerKind[SignatureHelpTriggerKind["ContentChange"] = 3] = "ContentChange";
    })(SignatureHelpTriggerKind || (SignatureHelpTriggerKind = {}));
    /**
     * A symbol kind.
     */
    var SymbolKind;
    (function (SymbolKind) {
        SymbolKind[SymbolKind["File"] = 0] = "File";
        SymbolKind[SymbolKind["Module"] = 1] = "Module";
        SymbolKind[SymbolKind["Namespace"] = 2] = "Namespace";
        SymbolKind[SymbolKind["Package"] = 3] = "Package";
        SymbolKind[SymbolKind["Class"] = 4] = "Class";
        SymbolKind[SymbolKind["Method"] = 5] = "Method";
        SymbolKind[SymbolKind["Property"] = 6] = "Property";
        SymbolKind[SymbolKind["Field"] = 7] = "Field";
        SymbolKind[SymbolKind["Constructor"] = 8] = "Constructor";
        SymbolKind[SymbolKind["Enum"] = 9] = "Enum";
        SymbolKind[SymbolKind["Interface"] = 10] = "Interface";
        SymbolKind[SymbolKind["Function"] = 11] = "Function";
        SymbolKind[SymbolKind["Variable"] = 12] = "Variable";
        SymbolKind[SymbolKind["Constant"] = 13] = "Constant";
        SymbolKind[SymbolKind["String"] = 14] = "String";
        SymbolKind[SymbolKind["Number"] = 15] = "Number";
        SymbolKind[SymbolKind["Boolean"] = 16] = "Boolean";
        SymbolKind[SymbolKind["Array"] = 17] = "Array";
        SymbolKind[SymbolKind["Object"] = 18] = "Object";
        SymbolKind[SymbolKind["Key"] = 19] = "Key";
        SymbolKind[SymbolKind["Null"] = 20] = "Null";
        SymbolKind[SymbolKind["EnumMember"] = 21] = "EnumMember";
        SymbolKind[SymbolKind["Struct"] = 22] = "Struct";
        SymbolKind[SymbolKind["Event"] = 23] = "Event";
        SymbolKind[SymbolKind["Operator"] = 24] = "Operator";
        SymbolKind[SymbolKind["TypeParameter"] = 25] = "TypeParameter";
    })(SymbolKind || (SymbolKind = {}));
    var SymbolTag;
    (function (SymbolTag) {
        SymbolTag[SymbolTag["Deprecated"] = 1] = "Deprecated";
    })(SymbolTag || (SymbolTag = {}));
    /**
     * The kind of animation in which the editor's cursor should be rendered.
     */
    var TextEditorCursorBlinkingStyle;
    (function (TextEditorCursorBlinkingStyle) {
        /**
         * Hidden
         */
        TextEditorCursorBlinkingStyle[TextEditorCursorBlinkingStyle["Hidden"] = 0] = "Hidden";
        /**
         * Blinking
         */
        TextEditorCursorBlinkingStyle[TextEditorCursorBlinkingStyle["Blink"] = 1] = "Blink";
        /**
         * Blinking with smooth fading
         */
        TextEditorCursorBlinkingStyle[TextEditorCursorBlinkingStyle["Smooth"] = 2] = "Smooth";
        /**
         * Blinking with prolonged filled state and smooth fading
         */
        TextEditorCursorBlinkingStyle[TextEditorCursorBlinkingStyle["Phase"] = 3] = "Phase";
        /**
         * Expand collapse animation on the y axis
         */
        TextEditorCursorBlinkingStyle[TextEditorCursorBlinkingStyle["Expand"] = 4] = "Expand";
        /**
         * No-Blinking
         */
        TextEditorCursorBlinkingStyle[TextEditorCursorBlinkingStyle["Solid"] = 5] = "Solid";
    })(TextEditorCursorBlinkingStyle || (TextEditorCursorBlinkingStyle = {}));
    /**
     * The style in which the editor's cursor should be rendered.
     */
    var TextEditorCursorStyle;
    (function (TextEditorCursorStyle) {
        /**
         * As a vertical line (sitting between two characters).
         */
        TextEditorCursorStyle[TextEditorCursorStyle["Line"] = 1] = "Line";
        /**
         * As a block (sitting on top of a character).
         */
        TextEditorCursorStyle[TextEditorCursorStyle["Block"] = 2] = "Block";
        /**
         * As a horizontal line (sitting under a character).
         */
        TextEditorCursorStyle[TextEditorCursorStyle["Underline"] = 3] = "Underline";
        /**
         * As a thin vertical line (sitting between two characters).
         */
        TextEditorCursorStyle[TextEditorCursorStyle["LineThin"] = 4] = "LineThin";
        /**
         * As an outlined block (sitting on top of a character).
         */
        TextEditorCursorStyle[TextEditorCursorStyle["BlockOutline"] = 5] = "BlockOutline";
        /**
         * As a thin horizontal line (sitting under a character).
         */
        TextEditorCursorStyle[TextEditorCursorStyle["UnderlineThin"] = 6] = "UnderlineThin";
    })(TextEditorCursorStyle || (TextEditorCursorStyle = {}));
    /**
     * Describes the behavior of decorations when typing/editing near their edges.
     * Note: Please do not edit the values, as they very carefully match `DecorationRangeBehavior`
     */
    var TrackedRangeStickiness;
    (function (TrackedRangeStickiness) {
        TrackedRangeStickiness[TrackedRangeStickiness["AlwaysGrowsWhenTypingAtEdges"] = 0] = "AlwaysGrowsWhenTypingAtEdges";
        TrackedRangeStickiness[TrackedRangeStickiness["NeverGrowsWhenTypingAtEdges"] = 1] = "NeverGrowsWhenTypingAtEdges";
        TrackedRangeStickiness[TrackedRangeStickiness["GrowsOnlyWhenTypingBefore"] = 2] = "GrowsOnlyWhenTypingBefore";
        TrackedRangeStickiness[TrackedRangeStickiness["GrowsOnlyWhenTypingAfter"] = 3] = "GrowsOnlyWhenTypingAfter";
    })(TrackedRangeStickiness || (TrackedRangeStickiness = {}));
    /**
     * Describes how to indent wrapped lines.
     */
    var WrappingIndent;
    (function (WrappingIndent) {
        /**
         * No indentation => wrapped lines begin at column 1.
         */
        WrappingIndent[WrappingIndent["None"] = 0] = "None";
        /**
         * Same => wrapped lines get the same indentation as the parent.
         */
        WrappingIndent[WrappingIndent["Same"] = 1] = "Same";
        /**
         * Indent => wrapped lines get +1 indentation toward the parent.
         */
        WrappingIndent[WrappingIndent["Indent"] = 2] = "Indent";
        /**
         * DeepIndent => wrapped lines get +2 indentation toward the parent.
         */
        WrappingIndent[WrappingIndent["DeepIndent"] = 3] = "DeepIndent";
    })(WrappingIndent || (WrappingIndent = {}));

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    class KeyMod {
        static chord(firstPart, secondPart) {
            return KeyChord(firstPart, secondPart);
        }
    }
    KeyMod.CtrlCmd = 2048 /* CtrlCmd */;
    KeyMod.Shift = 1024 /* Shift */;
    KeyMod.Alt = 512 /* Alt */;
    KeyMod.WinCtrl = 256 /* WinCtrl */;
    function createMonacoBaseAPI() {
        return {
            editor: undefined,
            languages: undefined,
            CancellationTokenSource: CancellationTokenSource,
            Emitter: Emitter,
            KeyCode: KeyCode,
            KeyMod: KeyMod,
            Position: Position,
            Range: Range,
            Selection: Selection,
            SelectionDirection: SelectionDirection,
            MarkerSeverity: MarkerSeverity,
            MarkerTag: MarkerTag,
            Uri: URI,
            Token: Token
        };
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    function leftIsWordBounday(wordSeparators, text, textLength, matchStartIndex, matchLength) {
        if (matchStartIndex === 0) {
            // Match starts at start of string
            return true;
        }
        const charBefore = text.charCodeAt(matchStartIndex - 1);
        if (wordSeparators.get(charBefore) !== 0 /* Regular */) {
            // The character before the match is a word separator
            return true;
        }
        if (charBefore === 13 /* CarriageReturn */ || charBefore === 10 /* LineFeed */) {
            // The character before the match is line break or carriage return.
            return true;
        }
        if (matchLength > 0) {
            const firstCharInMatch = text.charCodeAt(matchStartIndex);
            if (wordSeparators.get(firstCharInMatch) !== 0 /* Regular */) {
                // The first character inside the match is a word separator
                return true;
            }
        }
        return false;
    }
    function rightIsWordBounday(wordSeparators, text, textLength, matchStartIndex, matchLength) {
        if (matchStartIndex + matchLength === textLength) {
            // Match ends at end of string
            return true;
        }
        const charAfter = text.charCodeAt(matchStartIndex + matchLength);
        if (wordSeparators.get(charAfter) !== 0 /* Regular */) {
            // The character after the match is a word separator
            return true;
        }
        if (charAfter === 13 /* CarriageReturn */ || charAfter === 10 /* LineFeed */) {
            // The character after the match is line break or carriage return.
            return true;
        }
        if (matchLength > 0) {
            const lastCharInMatch = text.charCodeAt(matchStartIndex + matchLength - 1);
            if (wordSeparators.get(lastCharInMatch) !== 0 /* Regular */) {
                // The last character in the match is a word separator
                return true;
            }
        }
        return false;
    }
    function isValidMatch(wordSeparators, text, textLength, matchStartIndex, matchLength) {
        return (leftIsWordBounday(wordSeparators, text, textLength, matchStartIndex, matchLength)
            && rightIsWordBounday(wordSeparators, text, textLength, matchStartIndex, matchLength));
    }
    class Searcher {
        constructor(wordSeparators, searchRegex) {
            this._wordSeparators = wordSeparators;
            this._searchRegex = searchRegex;
            this._prevMatchStartIndex = -1;
            this._prevMatchLength = 0;
        }
        reset(lastIndex) {
            this._searchRegex.lastIndex = lastIndex;
            this._prevMatchStartIndex = -1;
            this._prevMatchLength = 0;
        }
        next(text) {
            const textLength = text.length;
            let m;
            do {
                if (this._prevMatchStartIndex + this._prevMatchLength === textLength) {
                    // Reached the end of the line
                    return null;
                }
                m = this._searchRegex.exec(text);
                if (!m) {
                    return null;
                }
                const matchStartIndex = m.index;
                const matchLength = m[0].length;
                if (matchStartIndex === this._prevMatchStartIndex && matchLength === this._prevMatchLength) {
                    if (matchLength === 0) {
                        // the search result is an empty string and won't advance `regex.lastIndex`, so `regex.exec` will stuck here
                        // we attempt to recover from that by advancing by two if surrogate pair found and by one otherwise
                        if (getNextCodePoint(text, textLength, this._searchRegex.lastIndex) > 0xFFFF) {
                            this._searchRegex.lastIndex += 2;
                        }
                        else {
                            this._searchRegex.lastIndex += 1;
                        }
                        continue;
                    }
                    // Exit early if the regex matches the same range twice
                    return null;
                }
                this._prevMatchStartIndex = matchStartIndex;
                this._prevMatchLength = matchLength;
                if (!this._wordSeparators || isValidMatch(this._wordSeparators, text, textLength, matchStartIndex, matchLength)) {
                    return m;
                }
            } while (m);
            return null;
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    class UnicodeTextModelHighlighter {
        static computeUnicodeHighlights(model, options, range) {
            const startLine = range ? range.startLineNumber : 1;
            const endLine = range ? range.endLineNumber : model.getLineCount();
            const codePointHighlighter = new CodePointHighlighter(options);
            const candidates = codePointHighlighter.getCandidateCodePoints();
            let regex;
            if (candidates === 'allNonBasicAscii') {
                regex = new RegExp('[^\\t\\n\\r\\x20-\\x7E]', 'g');
            }
            else {
                regex = new RegExp(`${buildRegExpCharClassExpr(Array.from(candidates))}`, 'g');
            }
            const searcher = new Searcher(null, regex);
            const ranges = [];
            let hasMore = false;
            let m;
            let ambiguousCharacterCount = 0;
            let invisibleCharacterCount = 0;
            let nonBasicAsciiCharacterCount = 0;
            forLoop: for (let lineNumber = startLine, lineCount = endLine; lineNumber <= lineCount; lineNumber++) {
                const lineContent = model.getLineContent(lineNumber);
                const lineLength = lineContent.length;
                // Reset regex to search from the beginning
                searcher.reset(0);
                do {
                    m = searcher.next(lineContent);
                    if (m) {
                        let startIndex = m.index;
                        let endIndex = m.index + m[0].length;
                        // Extend range to entire code point
                        if (startIndex > 0) {
                            const charCodeBefore = lineContent.charCodeAt(startIndex - 1);
                            if (isHighSurrogate(charCodeBefore)) {
                                startIndex--;
                            }
                        }
                        if (endIndex + 1 < lineLength) {
                            const charCodeBefore = lineContent.charCodeAt(endIndex - 1);
                            if (isHighSurrogate(charCodeBefore)) {
                                endIndex++;
                            }
                        }
                        const str = lineContent.substring(startIndex, endIndex);
                        const highlightReason = codePointHighlighter.shouldHighlightNonBasicASCII(str);
                        if (highlightReason !== 0 /* None */) {
                            if (highlightReason === 3 /* Ambiguous */) {
                                ambiguousCharacterCount++;
                            }
                            else if (highlightReason === 2 /* Invisible */) {
                                invisibleCharacterCount++;
                            }
                            else if (highlightReason === 1 /* NonBasicASCII */) {
                                nonBasicAsciiCharacterCount++;
                            }
                            else {
                                assertNever();
                            }
                            const MAX_RESULT_LENGTH = 1000;
                            if (ranges.length >= MAX_RESULT_LENGTH) {
                                hasMore = true;
                                break forLoop;
                            }
                            ranges.push(new Range(lineNumber, startIndex + 1, lineNumber, endIndex + 1));
                        }
                    }
                } while (m);
            }
            return {
                ranges,
                hasMore,
                ambiguousCharacterCount,
                invisibleCharacterCount,
                nonBasicAsciiCharacterCount
            };
        }
        static computeUnicodeHighlightReason(char, options) {
            const codePointHighlighter = new CodePointHighlighter(options);
            const reason = codePointHighlighter.shouldHighlightNonBasicASCII(char);
            switch (reason) {
                case 0 /* None */:
                    return null;
                case 2 /* Invisible */:
                    return { kind: 1 /* Invisible */ };
                case 3 /* Ambiguous */: {
                    const codePoint = char.codePointAt(0);
                    const primaryConfusable = codePointHighlighter.ambiguousCharacters.getPrimaryConfusable(codePoint);
                    const notAmbiguousInLocales = AmbiguousCharacters.getLocales().filter((l) => !AmbiguousCharacters.getInstance(new Set([...options.allowedLocales, l])).isAmbiguous(codePoint));
                    return { kind: 0 /* Ambiguous */, confusableWith: String.fromCodePoint(primaryConfusable), notAmbiguousInLocales };
                }
                case 1 /* NonBasicASCII */:
                    return { kind: 2 /* NonBasicAscii */ };
            }
        }
    }
    function buildRegExpCharClassExpr(codePoints, flags) {
        const src = `[${escapeRegExpCharacters(codePoints.map((i) => String.fromCodePoint(i)).join(''))}]`;
        return src;
    }
    class CodePointHighlighter {
        constructor(options) {
            this.options = options;
            this.allowedCodePoints = new Set(options.allowedCodePoints);
            this.ambiguousCharacters = AmbiguousCharacters.getInstance(new Set(options.allowedLocales));
        }
        getCandidateCodePoints() {
            if (this.options.nonBasicASCII) {
                return 'allNonBasicAscii';
            }
            const set = new Set();
            if (this.options.invisibleCharacters) {
                for (const cp of InvisibleCharacters.codePoints) {
                    set.add(cp);
                }
            }
            if (this.options.ambiguousCharacters) {
                for (const cp of this.ambiguousCharacters.getConfusableCodePoints()) {
                    set.add(cp);
                }
            }
            for (const cp of this.allowedCodePoints) {
                set.delete(cp);
            }
            return set;
        }
        shouldHighlightNonBasicASCII(character) {
            const codePoint = character.codePointAt(0);
            if (this.allowedCodePoints.has(codePoint)) {
                return 0 /* None */;
            }
            if (this.options.nonBasicASCII) {
                return 1 /* NonBasicASCII */;
            }
            if (this.options.invisibleCharacters) {
                const isAllowedInvisibleCharacter = character === ' ' || character === '\n' || character === '\t';
                // TODO check for emojis
                if (!isAllowedInvisibleCharacter && InvisibleCharacters.isInvisibleCharacter(codePoint)) {
                    return 2 /* Invisible */;
                }
            }
            if (this.options.ambiguousCharacters) {
                if (this.ambiguousCharacters.isAmbiguous(codePoint)) {
                    return 3 /* Ambiguous */;
                }
            }
            return 0 /* None */;
        }
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    var __awaiter = (undefined && undefined.__awaiter) || function (thisArg, _arguments, P, generator) {
        function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
        return new (P || (P = Promise))(function (resolve, reject) {
            function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
            function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
            function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
            step((generator = generator.apply(thisArg, _arguments || [])).next());
        });
    };
    /**
     * @internal
     */
    class MirrorModel extends MirrorTextModel {
        get uri() {
            return this._uri;
        }
        get eol() {
            return this._eol;
        }
        getValue() {
            return this.getText();
        }
        getLinesContent() {
            return this._lines.slice(0);
        }
        getLineCount() {
            return this._lines.length;
        }
        getLineContent(lineNumber) {
            return this._lines[lineNumber - 1];
        }
        getWordAtPosition(position, wordDefinition) {
            const wordAtText = getWordAtText(position.column, ensureValidWordDefinition(wordDefinition), this._lines[position.lineNumber - 1], 0);
            if (wordAtText) {
                return new Range(position.lineNumber, wordAtText.startColumn, position.lineNumber, wordAtText.endColumn);
            }
            return null;
        }
        words(wordDefinition) {
            const lines = this._lines;
            const wordenize = this._wordenize.bind(this);
            let lineNumber = 0;
            let lineText = '';
            let wordRangesIdx = 0;
            let wordRanges = [];
            return {
                *[Symbol.iterator]() {
                    while (true) {
                        if (wordRangesIdx < wordRanges.length) {
                            const value = lineText.substring(wordRanges[wordRangesIdx].start, wordRanges[wordRangesIdx].end);
                            wordRangesIdx += 1;
                            yield value;
                        }
                        else {
                            if (lineNumber < lines.length) {
                                lineText = lines[lineNumber];
                                wordRanges = wordenize(lineText, wordDefinition);
                                wordRangesIdx = 0;
                                lineNumber += 1;
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            };
        }
        getLineWords(lineNumber, wordDefinition) {
            const content = this._lines[lineNumber - 1];
            const ranges = this._wordenize(content, wordDefinition);
            const words = [];
            for (const range of ranges) {
                words.push({
                    word: content.substring(range.start, range.end),
                    startColumn: range.start + 1,
                    endColumn: range.end + 1
                });
            }
            return words;
        }
        _wordenize(content, wordDefinition) {
            const result = [];
            let match;
            wordDefinition.lastIndex = 0; // reset lastIndex just to be sure
            while (match = wordDefinition.exec(content)) {
                if (match[0].length === 0) {
                    // it did match the empty string
                    break;
                }
                result.push({ start: match.index, end: match.index + match[0].length });
            }
            return result;
        }
        getValueInRange(range) {
            range = this._validateRange(range);
            if (range.startLineNumber === range.endLineNumber) {
                return this._lines[range.startLineNumber - 1].substring(range.startColumn - 1, range.endColumn - 1);
            }
            const lineEnding = this._eol;
            const startLineIndex = range.startLineNumber - 1;
            const endLineIndex = range.endLineNumber - 1;
            const resultLines = [];
            resultLines.push(this._lines[startLineIndex].substring(range.startColumn - 1));
            for (let i = startLineIndex + 1; i < endLineIndex; i++) {
                resultLines.push(this._lines[i]);
            }
            resultLines.push(this._lines[endLineIndex].substring(0, range.endColumn - 1));
            return resultLines.join(lineEnding);
        }
        offsetAt(position) {
            position = this._validatePosition(position);
            this._ensureLineStarts();
            return this._lineStarts.getPrefixSum(position.lineNumber - 2) + (position.column - 1);
        }
        positionAt(offset) {
            offset = Math.floor(offset);
            offset = Math.max(0, offset);
            this._ensureLineStarts();
            const out = this._lineStarts.getIndexOf(offset);
            const lineLength = this._lines[out.index].length;
            // Ensure we return a valid position
            return {
                lineNumber: 1 + out.index,
                column: 1 + Math.min(out.remainder, lineLength)
            };
        }
        _validateRange(range) {
            const start = this._validatePosition({ lineNumber: range.startLineNumber, column: range.startColumn });
            const end = this._validatePosition({ lineNumber: range.endLineNumber, column: range.endColumn });
            if (start.lineNumber !== range.startLineNumber
                || start.column !== range.startColumn
                || end.lineNumber !== range.endLineNumber
                || end.column !== range.endColumn) {
                return {
                    startLineNumber: start.lineNumber,
                    startColumn: start.column,
                    endLineNumber: end.lineNumber,
                    endColumn: end.column
                };
            }
            return range;
        }
        _validatePosition(position) {
            if (!Position.isIPosition(position)) {
                throw new Error('bad position');
            }
            let { lineNumber, column } = position;
            let hasChanged = false;
            if (lineNumber < 1) {
                lineNumber = 1;
                column = 1;
                hasChanged = true;
            }
            else if (lineNumber > this._lines.length) {
                lineNumber = this._lines.length;
                column = this._lines[lineNumber - 1].length + 1;
                hasChanged = true;
            }
            else {
                const maxCharacter = this._lines[lineNumber - 1].length + 1;
                if (column < 1) {
                    column = 1;
                    hasChanged = true;
                }
                else if (column > maxCharacter) {
                    column = maxCharacter;
                    hasChanged = true;
                }
            }
            if (!hasChanged) {
                return position;
            }
            else {
                return { lineNumber, column };
            }
        }
    }
    /**
     * @internal
     */
    class EditorSimpleWorker {
        constructor(host, foreignModuleFactory) {
            this._host = host;
            this._models = Object.create(null);
            this._foreignModuleFactory = foreignModuleFactory;
            this._foreignModule = null;
        }
        dispose() {
            this._models = Object.create(null);
        }
        _getModel(uri) {
            return this._models[uri];
        }
        _getModels() {
            const all = [];
            Object.keys(this._models).forEach((key) => all.push(this._models[key]));
            return all;
        }
        acceptNewModel(data) {
            this._models[data.url] = new MirrorModel(URI.parse(data.url), data.lines, data.EOL, data.versionId);
        }
        acceptModelChanged(strURL, e) {
            if (!this._models[strURL]) {
                return;
            }
            const model = this._models[strURL];
            model.onEvents(e);
        }
        acceptRemovedModel(strURL) {
            if (!this._models[strURL]) {
                return;
            }
            delete this._models[strURL];
        }
        computeUnicodeHighlights(url, options, range) {
            return __awaiter(this, void 0, void 0, function* () {
                const model = this._getModel(url);
                if (!model) {
                    return { ranges: [], hasMore: false, ambiguousCharacterCount: 0, invisibleCharacterCount: 0, nonBasicAsciiCharacterCount: 0 };
                }
                return UnicodeTextModelHighlighter.computeUnicodeHighlights(model, options, range);
            });
        }
        // ---- BEGIN diff --------------------------------------------------------------------------
        computeDiff(originalUrl, modifiedUrl, ignoreTrimWhitespace, maxComputationTime) {
            return __awaiter(this, void 0, void 0, function* () {
                const original = this._getModel(originalUrl);
                const modified = this._getModel(modifiedUrl);
                if (!original || !modified) {
                    return null;
                }
                const originalLines = original.getLinesContent();
                const modifiedLines = modified.getLinesContent();
                const diffComputer = new DiffComputer(originalLines, modifiedLines, {
                    shouldComputeCharChanges: true,
                    shouldPostProcessCharChanges: true,
                    shouldIgnoreTrimWhitespace: ignoreTrimWhitespace,
                    shouldMakePrettyDiff: true,
                    maxComputationTime: maxComputationTime
                });
                const diffResult = diffComputer.computeDiff();
                const identical = (diffResult.changes.length > 0 ? false : this._modelsAreIdentical(original, modified));
                return {
                    quitEarly: diffResult.quitEarly,
                    identical: identical,
                    changes: diffResult.changes
                };
            });
        }
        _modelsAreIdentical(original, modified) {
            const originalLineCount = original.getLineCount();
            const modifiedLineCount = modified.getLineCount();
            if (originalLineCount !== modifiedLineCount) {
                return false;
            }
            for (let line = 1; line <= originalLineCount; line++) {
                const originalLine = original.getLineContent(line);
                const modifiedLine = modified.getLineContent(line);
                if (originalLine !== modifiedLine) {
                    return false;
                }
            }
            return true;
        }
        computeMoreMinimalEdits(modelUrl, edits) {
            return __awaiter(this, void 0, void 0, function* () {
                const model = this._getModel(modelUrl);
                if (!model) {
                    return edits;
                }
                const result = [];
                let lastEol = undefined;
                edits = edits.slice(0).sort((a, b) => {
                    if (a.range && b.range) {
                        return Range.compareRangesUsingStarts(a.range, b.range);
                    }
                    // eol only changes should go to the end
                    const aRng = a.range ? 0 : 1;
                    const bRng = b.range ? 0 : 1;
                    return aRng - bRng;
                });
                for (let { range, text, eol } of edits) {
                    if (typeof eol === 'number') {
                        lastEol = eol;
                    }
                    if (Range.isEmpty(range) && !text) {
                        // empty change
                        continue;
                    }
                    const original = model.getValueInRange(range);
                    text = text.replace(/\r\n|\n|\r/g, model.eol);
                    if (original === text) {
                        // noop
                        continue;
                    }
                    // make sure diff won't take too long
                    if (Math.max(text.length, original.length) > EditorSimpleWorker._diffLimit) {
                        result.push({ range, text });
                        continue;
                    }
                    // compute diff between original and edit.text
                    const changes = stringDiff(original, text, false);
                    const editOffset = model.offsetAt(Range.lift(range).getStartPosition());
                    for (const change of changes) {
                        const start = model.positionAt(editOffset + change.originalStart);
                        const end = model.positionAt(editOffset + change.originalStart + change.originalLength);
                        const newEdit = {
                            text: text.substr(change.modifiedStart, change.modifiedLength),
                            range: { startLineNumber: start.lineNumber, startColumn: start.column, endLineNumber: end.lineNumber, endColumn: end.column }
                        };
                        if (model.getValueInRange(newEdit.range) !== newEdit.text) {
                            result.push(newEdit);
                        }
                    }
                }
                if (typeof lastEol === 'number') {
                    result.push({ eol: lastEol, text: '', range: { startLineNumber: 0, startColumn: 0, endLineNumber: 0, endColumn: 0 } });
                }
                return result;
            });
        }
        // ---- END minimal edits ---------------------------------------------------------------
        computeLinks(modelUrl) {
            return __awaiter(this, void 0, void 0, function* () {
                const model = this._getModel(modelUrl);
                if (!model) {
                    return null;
                }
                return computeLinks(model);
            });
        }
        textualSuggest(modelUrls, leadingWord, wordDef, wordDefFlags) {
            return __awaiter(this, void 0, void 0, function* () {
                const sw = new StopWatch(true);
                const wordDefRegExp = new RegExp(wordDef, wordDefFlags);
                const seen = new Set();
                outer: for (let url of modelUrls) {
                    const model = this._getModel(url);
                    if (!model) {
                        continue;
                    }
                    for (let word of model.words(wordDefRegExp)) {
                        if (word === leadingWord || !isNaN(Number(word))) {
                            continue;
                        }
                        seen.add(word);
                        if (seen.size > EditorSimpleWorker._suggestionsLimit) {
                            break outer;
                        }
                    }
                }
                return { words: Array.from(seen), duration: sw.elapsed() };
            });
        }
        // ---- END suggest --------------------------------------------------------------------------
        //#region -- word ranges --
        computeWordRanges(modelUrl, range, wordDef, wordDefFlags) {
            return __awaiter(this, void 0, void 0, function* () {
                const model = this._getModel(modelUrl);
                if (!model) {
                    return Object.create(null);
                }
                const wordDefRegExp = new RegExp(wordDef, wordDefFlags);
                const result = Object.create(null);
                for (let line = range.startLineNumber; line < range.endLineNumber; line++) {
                    const words = model.getLineWords(line, wordDefRegExp);
                    for (const word of words) {
                        if (!isNaN(Number(word.word))) {
                            continue;
                        }
                        let array = result[word.word];
                        if (!array) {
                            array = [];
                            result[word.word] = array;
                        }
                        array.push({
                            startLineNumber: line,
                            startColumn: word.startColumn,
                            endLineNumber: line,
                            endColumn: word.endColumn
                        });
                    }
                }
                return result;
            });
        }
        //#endregion
        navigateValueSet(modelUrl, range, up, wordDef, wordDefFlags) {
            return __awaiter(this, void 0, void 0, function* () {
                const model = this._getModel(modelUrl);
                if (!model) {
                    return null;
                }
                const wordDefRegExp = new RegExp(wordDef, wordDefFlags);
                if (range.startColumn === range.endColumn) {
                    range = {
                        startLineNumber: range.startLineNumber,
                        startColumn: range.startColumn,
                        endLineNumber: range.endLineNumber,
                        endColumn: range.endColumn + 1
                    };
                }
                const selectionText = model.getValueInRange(range);
                const wordRange = model.getWordAtPosition({ lineNumber: range.startLineNumber, column: range.startColumn }, wordDefRegExp);
                if (!wordRange) {
                    return null;
                }
                const word = model.getValueInRange(wordRange);
                const result = BasicInplaceReplace.INSTANCE.navigateValueSet(range, selectionText, wordRange, word, up);
                return result;
            });
        }
        // ---- BEGIN foreign module support --------------------------------------------------------------------------
        loadForeignModule(moduleId, createData, foreignHostMethods) {
            const proxyMethodRequest = (method, args) => {
                return this._host.fhr(method, args);
            };
            const foreignHost = createProxyObject$1(foreignHostMethods, proxyMethodRequest);
            const ctx = {
                host: foreignHost,
                getMirrorModels: () => {
                    return this._getModels();
                }
            };
            if (this._foreignModuleFactory) {
                this._foreignModule = this._foreignModuleFactory(ctx, createData);
                // static foreing module
                return Promise.resolve(getAllMethodNames(this._foreignModule));
            }
            // ESM-comment-begin
            // 		return new Promise<any>((resolve, reject) => {
            // 			require([moduleId], (foreignModule: { create: IForeignModuleFactory }) => {
            // 				this._foreignModule = foreignModule.create(ctx, createData);
            // 
            // 				resolve(types.getAllMethodNames(this._foreignModule));
            // 
            // 			}, reject);
            // 		});
            // ESM-comment-end
            // ESM-uncomment-begin
            return Promise.reject(new Error(`Unexpected usage`));
            // ESM-uncomment-end
        }
        // foreign method request
        fmr(method, args) {
            if (!this._foreignModule || typeof this._foreignModule[method] !== 'function') {
                return Promise.reject(new Error('Missing requestHandler or method: ' + method));
            }
            try {
                return Promise.resolve(this._foreignModule[method].apply(this._foreignModule, args));
            }
            catch (e) {
                return Promise.reject(e);
            }
        }
    }
    // ---- END diff --------------------------------------------------------------------------
    // ---- BEGIN minimal edits ---------------------------------------------------------------
    EditorSimpleWorker._diffLimit = 100000;
    // ---- BEGIN suggest --------------------------------------------------------------------------
    EditorSimpleWorker._suggestionsLimit = 10000;
    if (typeof importScripts === 'function') {
        // Running in a web worker
        globals.monaco = createMonacoBaseAPI();
    }

    /*---------------------------------------------------------------------------------------------
     *  Copyright (c) Microsoft Corporation. All rights reserved.
     *  Licensed under the MIT License. See License.txt in the project root for license information.
     *--------------------------------------------------------------------------------------------*/
    let initialized = false;
    function initialize(foreignModule) {
        if (initialized) {
            return;
        }
        initialized = true;
        const simpleWorker = new SimpleWorkerServer((msg) => {
            self.postMessage(msg);
        }, (host) => new EditorSimpleWorker(host, foreignModule));
        self.onmessage = (e) => {
            simpleWorker.onmessage(e.data);
        };
    }
    self.onmessage = (e) => {
        // Ignore first message in this case and initialize if not yet initialized
        if (!initialized) {
            initialize(null);
        }
    };

    exports.initialize = initialize;

    Object.defineProperty(exports, '__esModule', { value: true });

    return exports;

})({});
