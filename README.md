# NPM Package for pdbtbx

Wrapper around [pdbtbx rust library](https://crates.io/crates/pdbtbx) for reading (crystallographic) Protein Data Bank (PDB) and mmCIF files.

[![Build](https://github.com/i-VRESSE/pdbtbx-ts/actions/workflows/build.yml/badge.svg)](https://github.com/i-VRESSE/pdbtbx-ts/actions/workflows/build.yml)

## 🚴 Usage

Add to your app with

```shell
npm install pdbtbx-ts
```

In NodeJS use

```js
const { readFile } = await import('fs/promises')
const pdbtbx = await import('pdbtbx-ts')
const wasmBuffer = await readFile('node_modules/pdbtbx-ts/pdbtbx_ts_bg.wasm')
await pdbtbx.default(wasmBuffer)
# A PDB file downloaded from https://github.com/haddocking/haddock3/tree/main/examples/docking-protein-protein/data
const content = await readFile('./e2aP_1F3G.pdb', encoding='ascii')
info = pdbtbx.open_pdb(content)
info.chains
```

## Development

### 🛠️ Build with `wasm-pack build`

```shell
wasm-pack build --target web
# Make generated package compatible with vite, vitest and NodeJS>=16
perl -pi -e 's/\"module\"/"type": "module", "main"/' pkg/package.json
```

### 🔬 Test

```shell
wasm-pack test --node
```

Run JS tests that consume wasm. Requires NodeJS>=18.

```shell
node --test tests
```

### 🎁 Publish to NPM with `wasm-pack publish`

```shell
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

## Using it locally

First build npm package

```shell
wasm-pack build --target web
# Make generated package compatible with vite, vitest and NodeJS>=16
perl -pi -e 's/\"module\"/"type": "module", "main"/' pkg/package.json
```

`wasm-pack build` has several [targets](https://rustwasm.github.io/wasm-pack/book/commands/build.html#target).
The `web` target was picked because it compatible in more environments, but requires the user to supply the wasm file.
The `nodejs` and `bundler` targets are incompatible with vite and vitest.

Inside app dir

```shell
yarn add ../pdbtbx-ts/pkg
```

In src/parse.ts

```js
import pdbtbx, { open_pdb } from 'pdbtbx-ts'

function parse(content) {
  if (process.env.NODE_ENV === 'test') {
    // vitest is run in NodeJS so needs wasm file read from disk instead of fetch using url
    const { readFile } = await import('fs/promises')
    const wasmBuffer = await readFile('node_modules/pdbtbx-ts/pdbtbx_ts_bg.wasm')
    await pdbtbx(wasmBuffer)
  } else {
    // To make vite aware of wasm file, it needs to passed to pdbtbx-ts default method.
    // TODO make prettier URL
    const mod = new URL('../../node_modules/pdbtbx-ts/pdbtbx_ts_bg.wasm', import.meta.url)
    await pdbtbx(mod)
  }
  await pdbtbx(mod)
  const info = open_pdb(content)
  return info
}
```

With NodeJS >=16 from this dir

```js
const { readFile } = await import('fs/promises')
const outdir = 'pkg'
const pdbtbx = await import(`./${outdir}/pdbtbx_ts.js`)
const wasmBuffer = await readFile(`${outdir}/pdbtbx_ts_bg.wasm`)
await pdbtbx.default(wasmBuffer)
const content = await readFile('./e2aP_1F3G.pdb', encoding='ascii')
const info = pdbtbx.open_pdb(content)
info.chains
info.residues_per_chain.get('A')
```

With NodeJS >=16 from app with pdbtbx-ts installed

```js
const { readFile } = await import('fs/promises')
const pdbtbx = await import('pdbtbx-ts')
const wasmBuffer = await readFile('node_modules/pdbtbx-ts/pdbtbx_ts_bg.wasm')
await pdbtbx.default(wasmBuffer)
const content = await readFile('./e2aP_1F3G.pdb', encoding='ascii')
const info = pdbtbx.open_pdb(content)
info.chains
```
