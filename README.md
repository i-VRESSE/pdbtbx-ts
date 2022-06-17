# NPM Package for pdbtbx

Wrapper around [pdbtbx rust library](https://crates.io/crates/pdbtbx) for reading (crystallographic) Protein Data Bank (PDB) and mmCIF files in JavaScript.

[![npmjs.com](https://img.shields.io/npm/v/@i-vresse/pdbtbx-ts.svg?style=flat)](https://www.npmjs.com/package/@i-vresse/pdbtbx-ts)
[![Build](https://github.com/i-VRESSE/pdbtbx-ts/actions/workflows/build.yml/badge.svg)](https://github.com/i-VRESSE/pdbtbx-ts/actions/workflows/build.yml)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.6532349.svg)](https://doi.org/10.5281/zenodo.6532349)
[![fair-software.eu](https://img.shields.io/badge/fair--software.eu-%E2%97%8F%20%20%E2%97%8F%20%20%E2%97%8F%20%20%E2%97%8F%20%20%E2%97%8B-yellow)](https://fair-software.eu)

This NPM package only includes a subset of what the rust library can do.
For now it is meanly focused on getting chain names and residue sequence numbers from PDB files.

This package uses [wasm-pack](https://rustwasm.github.io/) and [tsup](https://tsup.egoist.sh) to compile the pdbtbx rust library and its bindings to a NPM package using WebAssembly.

## 🚴 Usage

Add to your app with

```shell
npm install @i-vresse/pdbtbx-ts
```

In NodeJS use

```js
const pdbtbx = await import('@i-vresse/pdbtbx-ts')
await pdbtbx.init()
const { readFile } = await import('fs/promises')
// A PDB file downloaded from https://github.com/haddocking/haddock3/tree/main/examples/docking-protein-protein/data
const content = await readFile('./e2aP_1F3G.pdb', encoding='ascii')
const info = pdbtbx.open_pdb(content)
{
  identifier: undefined,
  chains: [ 'A' ],
  residue_sequence_numbers: [
     19,  20,  21,  22,  23,  24,  25,  26,  27,  28,  29,  30,
    ...
  ],
  residues_per_chain: Map(1) {
    'A' => [
      { number: 19, insertion_code: '-' },
      { number: 20, insertion_code: '-' },
      ...
    ]
  },
  warnings: []
}
```

In an application using Vite, vitest and TypeScript use

```js
import { init, open_pdb } from 'pdbtbx-ts'

async function parse(content: string) {
  await init()
  const info = open_pdb(content)
  return info
}
```

## Development

Below are instructions how to develop this repository.

Requirements:

* Rust
* [wasm-pack](https://rustwasm.github.io/wasm-pack/)
* NodeJS version 18 or greater

### 🛠️ Build

To build a NPM package in `dist/` directory use

```shell
npm install
npm run build
```

### 🔬 Test

Tests can be found in `tests/` directory.

Run JS tests that consume wasm.

```shell
npm run test
```

### 🎁 Publish to npmjs.com

```shell
npm publish --access public .
```
