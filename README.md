<div align="center">

  <h1><code>wasm-pack-template</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
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
The `web` target was picked because it compatible in more environments, but requires the suer to supply the wasm file.
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
content = await readFile('./e2aP_1F3G.pdb', encoding='ascii')
const outdir = 'pkg'
const pdbtbx = await import(`./${outdir}/pdbtbx_ts.js`)
const wasmBuffer = await readFile(`${outdir}/pdbtbx_ts_bg.wasm`)
await pdbtbx.default(wasmBuffer)
info = pdbtbx.open_pdb(content)
info.chains
info.residues_per_chain.get('A')
```

With NodeJS >=16 from app with pdbtbx-ts installed

```js
const { readFile } = await import('fs/promises')
content = await readFile('./e2aP_1F3G.pdb', encoding='ascii')
const pdbtbx = await import('pdbtbx-ts')
const wasmBuffer = await readFile('node_modules/pdbtbx-ts/pdbtbx_ts_bg.wasm')
await pdbtbx.default(wasmBuffer)
info = pdbtbx.open_pdb(content)
info.chains
```
