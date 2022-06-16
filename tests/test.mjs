import test from 'node:test';
import assert from 'node:assert';

test('open_pdb', async () => {
    const outdir = 'dist'
    const pdbtbx = await import(`../${outdir}/index.js`)
    await pdbtbx.init()

    const content = `ATOM      1  N  BALA A   1      72.883  57.697  56.410  0.50 83.80           N
END`;
    const result = pdbtbx.open_pdb(content)

    const expected = {
        identifier: undefined,
        chains: ['A'],
        residue_sequence_numbers: [1],
        residues_per_chain: new Map([[
            'A',
            [{
                insertion_code: '-',
                number: 1
            }]
        ]]),
        warnings: []
    }
    assert.deepStrictEqual(result, expected)
})