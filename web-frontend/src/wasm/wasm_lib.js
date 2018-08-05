/* tslint:disable */
import * as wasm from './wasm_lib_bg';
import { random } from './platform';
import { log } from './platform';
import { start_frame } from './platform';
import { end_frame } from './platform';
import { draw_rectangle } from './platform';

export function __wbg_random_677791a27e7cfa8f() {
    return random();
}

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_log_85a925654c9c374f(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    
    varg0 = varg0.slice();
    wasm.__wbindgen_free(arg0, arg1 * 1);
    
    log(varg0);
}

export function __wbg_start_frame_0f2f1c60193b9b75() {
    start_frame();
}

export function __wbg_end_frame_a56b016ede2f678d() {
    end_frame();
}

export function __wbg_draw_rectangle_b1d01c6b7a266db4(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    draw_rectangle(arg0, arg1, arg2, arg3, arg4, arg5, arg6);
}
/**
* @returns {void}
*/
export function init() {
    return wasm.init();
}

const TextEncoder = typeof self === 'object' && self.TextEncoder
    ? self.TextEncoder
    : require('util').TextEncoder;

let cachedEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {
    
    const buf = cachedEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}
/**
* @param {string} arg0
* @param {number} arg1
* @param {number} arg2
* @returns {void}
*/
export function update(arg0, arg1, arg2) {
    const [ptr0, len0] = passStringToWasm(arg0);
    return wasm.update(ptr0, len0, arg1, arg2);
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

