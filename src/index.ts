import pdbtbx_ts from '../pkg/pdbtbx_ts'
import * as wasm from '../pkg/pdbtbx_ts_bg.wasm'
export { open_pdb } from '../pkg/pdbtbx_ts'

/**
 *
 * @returns
 */
export async function init () {
    await pdbtbx_ts(wasm)
    return
}
