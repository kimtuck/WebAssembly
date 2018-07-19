/* tslint:disable */
import * as wasm from './app_wasm_bg';

let slab = [];

let slab_next = 0;

function addHeapObject(obj) {
    if (slab_next === slab.length)
        slab.push(slab.length + 1);
    const idx = slab_next;
    const next = slab[idx];

    slab_next = next;

    slab[idx] = { obj, cnt: 1 };
    return idx << 1;
}

export function __wbg_static_accessor_document_document() {
    return addHeapObject(document);
}

const __wbg_f_createElement_createElement_HTMLDocument_target = HTMLDocument.prototype.createElement;

let stack = [];

function getObject(idx) {
    if ((idx & 1) === 1) {
        return stack[idx >> 1];
    } else {
        const val = slab[idx >> 1];

    return val.obj;

    }
}

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null ||
        cachegetUint8Memory.buffer !== wasm.memory.buffer)
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_f_createElement_createElement_HTMLDocument(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    return addHeapObject(__wbg_f_createElement_createElement_HTMLDocument_target.call(getObject(arg0), varg1));
}

const __wbg_f_getElementById_get_element_by_id_HTMLDocument_target = HTMLDocument.prototype.getElementById;

export function __wbg_f_getElementById_get_element_by_id_HTMLDocument(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    return addHeapObject(__wbg_f_getElementById_get_element_by_id_HTMLDocument_target.call(getObject(arg0), varg1));
}

function GetOwnOrInheritedPropertyDescriptor(obj, id) {
  while (obj) {
    let desc = Object.getOwnPropertyDescriptor(obj, id);
    if (desc) return desc;
    obj = Object.getPrototypeOf(obj);
  }
  throw "descriptor not found";
}

const __wbg_f_set_inner_html_set_inner_html_Element_target = GetOwnOrInheritedPropertyDescriptor(Element.prototype, 'innerHTML').set;;

export function __wbg_f_set_inner_html_set_inner_html_Element(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __wbg_f_set_inner_html_set_inner_html_Element_target.call(getObject(arg0), varg1);
}

const __wbg_f_appendChild_append_child_Element_target = Element.prototype.appendChild;

function dropRef(idx) {

    let obj = slab[idx >> 1];

    obj.cnt -= 1;
    if (obj.cnt > 0)
        return;

    // If we hit 0 then free up our space in the slab
    slab[idx >> 1] = slab_next;
    slab_next = idx >> 1;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropRef(idx);
    return ret;
}

export function __wbg_f_appendChild_append_child_Element(arg0, arg1) {
    __wbg_f_appendChild_append_child_Element_target.call(getObject(arg0), takeObject(arg1));
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

export function letter_count_webassembly(arg0) {
    const [ptr0, len0] = passStringToWasm(arg0);
    try {
        return wasm.letter_count_webassembly(ptr0, len0);
    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);
    }
}

export function __wbindgen_object_drop_ref(i) { dropRef(i); }

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

