import pdbtbx_ts from '../pkg/pdbtbx_ts'
import * as wasm from '../pkg/pdbtbx_ts_bg.wasm'
export * from '../pkg/pdbtbx_ts'

function toArray(base64encoded: string) {
    if (typeof Buffer === 'function') {
        return Buffer.from(base64encoded, 'base64')
    }
    const binary_string = atob(base64encoded);
    const len = binary_string.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
        bytes[i] = binary_string.charCodeAt(i);
    }
    return bytes.buffer;
}

/**
 * Initialize WebAssembly module
 */
export async function init () {
    const w: any = wasm
    await pdbtbx_ts(toArray(w.default))
    return
}
