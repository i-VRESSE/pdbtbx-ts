import test from 'node:test';
import assert from 'node:assert';

test('open_pdb()', async (t) => {
    const outdir = 'dist'
    const pdbtbx = await import(`../${outdir}/index.js`)
    await pdbtbx.init()

    await t.test('single atom', async () => {
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

    await t.test('master err', async () => {
        const content = `ATOM      1  N  BALA B   1      72.883  57.697  56.410  0.50 83.80           N
MASTER        0    0    0    0    0    0    0    0    2    0    0    0
END`;
        try {
            pdbtbx.open_pdb(content)
            assert.fail('Should have thrown error')
        } catch (error) {
            assert.match(error[0], /is different then posed in the MASTER Record/)
        }
    })

    await t.test('dup_resnum', async () => {
        const content = `ATOM      1  N   THR A  19      51.382  49.015  12.266  1.00  0.15      A
ATOM   2587  N   ALA B  19      27.171  30.284  19.911  1.00  0.14      B
END`;

        const result = pdbtbx.open_pdb(content)

        const expected = {
            identifier: undefined,
            chains: ['A', 'B'],
            residue_sequence_numbers: [19],
            residues_per_chain: new Map([[
                'A',
                [{
                    insertion_code: '-',
                    number: 19
                }]
            ], [
                'B',
                [{
                    insertion_code: '-',
                    number: 19
                }]
            ]]),
            warnings: []
        }
        assert.deepStrictEqual(result, expected)
    })

    await t.test('sorted_resnum', async () => {
        const content = `ATOM    588 1HG  GLU A  18     -13.363  -4.163  -2.372  1.00  0.00           H
ATOM      5  CB BALA B   1      74.804  56.369  55.453  0.50 84.29           C
ATOM      6  N  BASP B   2      75.872  56.703  57.905  0.50 85.59           N
END`;

        const result = pdbtbx.open_pdb(content)

        const expected = {
            identifier: undefined,
            chains: ['A', 'B'],
            residue_sequence_numbers: [1, 2 ,18],
            residues_per_chain: new Map([[
                'A',
                [{
                    insertion_code: '-',
                    number: 18
                }]
            ], [
                'B',
                [{
                    insertion_code: '-',
                    number: 1
                }, {
                    insertion_code: '-',
                    number: 2
                }]
            ]]),
            warnings: []
        }
        assert.deepStrictEqual(result, expected)
    })
})
