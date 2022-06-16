import pdbtbx_ts from '../pkg/pdbtbx_ts'
import * as wasm from '../pkg/pdbtbx_ts_bg.wasm'
export * from '../pkg/pdbtbx_ts'

/**
 * Initialize WebAssembly module
 */
export async function init () {
    const w: any = wasm
    await pdbtbx_ts(w.default)
    return
}
