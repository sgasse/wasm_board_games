let wasm_bindgen;
(function() {
    const __exports = {};
    let script_src;
    if (typeof document === 'undefined') {
        script_src = location.href;
    } else {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }
    let wasm;

    const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachedUint8Memory0 = null;

    function getUint8Memory0() {
        if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
            cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    const heap = new Array(128).fill(undefined);

    heap.push(undefined, null, true, false);

    let heap_next = heap.length;

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
*/
__exports.ExpandResult = Object.freeze({ Done:0,"0":"Done",NotDone:1,"1":"NotDone", });
/**
*/
__exports.Cell = Object.freeze({ Empty:0,"0":"Empty",X:1,"1":"X",O:2,"2":"O", });
/**
*/
class Board {

    static __wrap(ptr) {
        const obj = Object.create(Board.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_board_free(ptr);
    }
    /**
    * @param {number} height
    * @param {number} width
    * @returns {Board}
    */
    static new(height, width) {
        const ret = wasm.board_new(height, width);
        return Board.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    width() {
        const ret = wasm.board_width(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    height() {
        const ret = wasm.board_height(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    cells_ptr() {
        const ret = wasm.board_cells_ptr(this.ptr);
        return ret;
    }
    /**
    * @param {number} row
    * @param {number} col
    * @param {number} mark
    * @returns {boolean}
    */
    set_cell(row, col, mark) {
        const ret = wasm.board_set_cell(this.ptr, row, col, mark);
        return ret !== 0;
    }
    /**
    * @param {number} row
    * @param {number} col
    * @returns {number}
    */
    get_index(row, col) {
        const ret = wasm.board_get_index(this.ptr, row, col);
        return ret >>> 0;
    }
    /**
    * @param {number} row
    * @param {number} col
    * @returns {boolean}
    */
    in_bounds(row, col) {
        const ret = wasm.board_in_bounds(this.ptr, row, col);
        return ret !== 0;
    }
    /**
    * @param {number} idx
    * @returns {Coords}
    */
    get_coords(idx) {
        const ret = wasm.board_get_coords(this.ptr, idx);
        return Coords.__wrap(ret);
    }
    /**
    */
    reset() {
        wasm.board_reset(this.ptr);
    }
    /**
    * Determine the winner on the lines through `self.last_move`.
    *
    * This assumes that there is no winning pattern on any other line which
    * does not go through `self.last_move`. This is a reasonable assumption
    * if every game state is evaluated directly, thus a previously completed
    * pattern on another line would have been detected before.
    * @param {Coords} last_move
    * @param {number} num_winner
    * @returns {number}
    */
    line_winner(last_move, num_winner) {
        _assertClass(last_move, Coords);
        const ret = wasm.board_line_winner(this.ptr, last_move.ptr, num_winner);
        return ret >>> 0;
    }
    /**
    * @param {number} col
    * @returns {Coords}
    */
    first_empty_in_column(col) {
        const ret = wasm.board_first_empty_in_column(this.ptr, col);
        return Coords.__wrap(ret);
    }
}
__exports.Board = Board;
/**
*/
class BoardMove {

    static __wrap(ptr) {
        const obj = Object.create(BoardMove.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_boardmove_free(ptr);
    }
    /**
    * @returns {Coords}
    */
    get coords() {
        const ret = wasm.__wbg_get_boardmove_coords(this.ptr);
        return Coords.__wrap(ret);
    }
    /**
    * @param {Coords} arg0
    */
    set coords(arg0) {
        _assertClass(arg0, Coords);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_boardmove_coords(this.ptr, ptr0);
    }
    /**
    * @returns {number}
    */
    get side() {
        const ret = wasm.__wbg_get_boardmove_side(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set side(arg0) {
        wasm.__wbg_set_boardmove_side(this.ptr, arg0);
    }
    /**
    * @param {number} row
    * @param {number} col
    * @param {number} side
    * @returns {BoardMove}
    */
    static new(row, col, side) {
        const ret = wasm.boardmove_new(row, col, side);
        return BoardMove.__wrap(ret);
    }
    /**
    * @param {any} js_value
    * @returns {BoardMove}
    */
    static from_js_value(js_value) {
        const ret = wasm.boardmove_from_js_value(addHeapObject(js_value));
        return BoardMove.__wrap(ret);
    }
    /**
    * @returns {any}
    */
    to_js_value() {
        const ret = wasm.boardmove_to_js_value(this.ptr);
        return takeObject(ret);
    }
}
__exports.BoardMove = BoardMove;
/**
*/
class Coords {

    static __wrap(ptr) {
        const obj = Object.create(Coords.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_coords_free(ptr);
    }
    /**
    * @returns {number}
    */
    get row() {
        const ret = wasm.__wbg_get_coords_row(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set row(arg0) {
        wasm.__wbg_set_coords_row(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get col() {
        const ret = wasm.__wbg_get_coords_col(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set col(arg0) {
        wasm.__wbg_set_coords_col(this.ptr, arg0);
    }
    /**
    * @param {number} row
    * @param {number} col
    * @returns {Coords}
    */
    static new(row, col) {
        const ret = wasm.coords_new(row, col);
        return Coords.__wrap(ret);
    }
}
__exports.Coords = Coords;
/**
*/
class FiarGameInterface {

    static __wrap(ptr) {
        const obj = Object.create(FiarGameInterface.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_fiargameinterface_free(ptr);
    }
    /**
    * @returns {FiarGameInterface}
    */
    static new() {
        const ret = wasm.fiargameinterface_new();
        return FiarGameInterface.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    expand_one_level() {
        const ret = wasm.fiargameinterface_expand_one_level(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {BoardMove} game_move
    * @returns {boolean}
    */
    track_move(game_move) {
        _assertClass(game_move, BoardMove);
        var ptr0 = game_move.__destroy_into_raw();
        const ret = wasm.fiargameinterface_track_move(this.ptr, ptr0);
        return ret !== 0;
    }
    /**
    * @returns {BoardMove}
    */
    get_best_move() {
        const ret = wasm.fiargameinterface_get_best_move(this.ptr);
        return BoardMove.__wrap(ret);
    }
    /**
    */
    reset() {
        wasm.fiargameinterface_reset(this.ptr);
    }
}
__exports.FiarGameInterface = FiarGameInterface;
/**
*/
class FiarGameState {

    static __wrap(ptr) {
        const obj = Object.create(FiarGameState.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_fiargamestate_free(ptr);
    }
    /**
    * @param {Board} board
    * @param {BoardMove} last_move
    * @returns {FiarGameState}
    */
    static new(board, last_move) {
        _assertClass(board, Board);
        var ptr0 = board.__destroy_into_raw();
        _assertClass(last_move, BoardMove);
        var ptr1 = last_move.__destroy_into_raw();
        const ret = wasm.fiargamestate_new(ptr0, ptr1);
        return FiarGameState.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    side() {
        const ret = wasm.fiargamestate_side(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {BoardMove}
    */
    last_move() {
        const ret = wasm.fiargamestate_last_move(this.ptr);
        return BoardMove.__wrap(ret);
    }
}
__exports.FiarGameState = FiarGameState;
/**
*/
class T3GameInterface {

    static __wrap(ptr) {
        const obj = Object.create(T3GameInterface.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_t3gameinterface_free(ptr);
    }
    /**
    * @returns {T3GameInterface}
    */
    static new() {
        const ret = wasm.t3gameinterface_new();
        return T3GameInterface.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    expand_one_level() {
        const ret = wasm.t3gameinterface_expand_one_level(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {BoardMove} game_move
    * @returns {boolean}
    */
    track_move(game_move) {
        _assertClass(game_move, BoardMove);
        var ptr0 = game_move.__destroy_into_raw();
        const ret = wasm.t3gameinterface_track_move(this.ptr, ptr0);
        return ret !== 0;
    }
    /**
    * @returns {BoardMove}
    */
    get_best_move() {
        const ret = wasm.t3gameinterface_get_best_move(this.ptr);
        return BoardMove.__wrap(ret);
    }
    /**
    */
    reset() {
        wasm.t3gameinterface_reset(this.ptr);
    }
}
__exports.T3GameInterface = T3GameInterface;
/**
*/
class T3GameState {

    static __wrap(ptr) {
        const obj = Object.create(T3GameState.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_t3gamestate_free(ptr);
    }
    /**
    * @param {Board} board
    * @param {BoardMove} last_move
    * @returns {T3GameState}
    */
    static new(board, last_move) {
        _assertClass(board, Board);
        var ptr0 = board.__destroy_into_raw();
        _assertClass(last_move, BoardMove);
        var ptr1 = last_move.__destroy_into_raw();
        const ret = wasm.t3gamestate_new(ptr0, ptr1);
        return T3GameState.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    side() {
        const ret = wasm.t3gamestate_side(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {BoardMove}
    */
    last_move() {
        const ret = wasm.t3gamestate_last_move(this.ptr);
        return BoardMove.__wrap(ret);
    }
}
__exports.T3GameState = T3GameState;

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_boardmove_new = function(arg0) {
        const ret = BoardMove.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_in = function(arg0, arg1) {
        const ret = getObject(arg0) in getObject(arg1);
        return ret;
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
        const ret = getObject(arg0) == getObject(arg1);
        return ret;
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbg_getwithrefkey_15c62c2b8546208d = function(arg0, arg1) {
        const ret = getObject(arg0)[getObject(arg1)];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_20cbc34131e76824 = function(arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    };
    imports.wbg.__wbg_log_7bb108d119bafbc1 = function(arg0) {
        console.log(getObject(arg0));
    };
    imports.wbg.__wbg_log_d047cf0648d2678e = function(arg0, arg1) {
        console.log(getObject(arg0), getObject(arg1));
    };
    imports.wbg.__wbg_get_27fe3dac1c4d0224 = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_e498fbc24f9c1d4f = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_new_f9876326328f45ed = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_ArrayBuffer_a69f02ee4c4f5065 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof ArrayBuffer;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_isSafeInteger_8c4789029e885159 = function(arg0) {
        const ret = Number.isSafeInteger(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_entries_4e1315b774245952 = function(arg0) {
        const ret = Object.entries(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_cf65c07de34b9a08 = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_537b7341ce90bb31 = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_17499e8aa4003ebd = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_27a2afe8ab42b09f = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Uint8Array_01cebe79ca606cca = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Uint8Array;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };

    return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedFloat64Memory0 = null;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = script_src.replace(/\.js$/, '_bg.wasm');
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

wasm_bindgen = Object.assign(init, { initSync }, __exports);

})();
