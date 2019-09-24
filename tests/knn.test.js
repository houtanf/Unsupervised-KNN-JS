const { toMatchCloseTo } = require('jest-matcher-deep-close-to')
expect.extend({ toMatchCloseTo })

const { knn } = require('../lib/knn')


knnAlgos = [
  'euclidean',
  'cosine',
  'mse',
  'manhattan',
  'chebyshev',
  'canberra',
  'hamming',
  'L3',
  'L4',
  'L5',
  'pearson',
]

knnTestCases = [
  [1, [{label: 'name', vector: [1, 2]}], [1, 2], [{label: 'name', distance: 0}]],
  [2, [{label: 'name', vector: [1, 2]}], [1, 2], [{label: 'name', distance: 0}]],
  [0, [{label: 'name', vector: [1, 2]}], [1, 2], []],
  [2, [{label: 'name', vector: [1, 2]}, {label: 'other', vector: [1, 2]}], [1, 2],
     [{label: 'name', distance: 0}, {label: 'other', distance: 0}]],
  [1, [{label: 'name', vector: [1, 2]}, {label: 'other', vector: [3, 2]}], [1, 2],
     [{label: 'name', distance: 0}]],
  [1, [{label: 'name', vector: [5, 4]}, {label: 'other', vector: [1, 2]}], [1, 2],
     [{label: 'other', distance: 0}]],
]

knnTests = knnAlgos.map(algo => 
   knnTestCases.map( test => [algo, ...test])
).reduce((x, y) => x.concat(y), [])

test.each(knnTests)('Test knn on correct data with %s algo, %d %o %o %o input', 
  function(algo, k, neighbors, target, expected) {
    const output = knn(algo, k, neighbors, target)

    expect(output).toMatchCloseTo(expected, 5)
  }
 )


knnFailCases = [
  ['euclidean', -1, [{label: 'name', vector: [1, 2]}], [1, 2]],
  ['doesnt exist', 1, [{label: 'hi', vector: [1]}], [1]],

  [null, null, null, null],
  ['euclidean', null, null, null],
  ['euclidean', 1, null, null],
  ['euclidean', 1, [null], null],
  ['euclidean', 1, [{incorrect: '?'}], null],
  ['euclidean', 1, [{label: 'no vector'}], null],
  ['euclidean', 1, [{vector: 'no label'}], null],
  ['euclidean', 1, [{vector: [1]}], null],
  ['euclidean', 1, [{lable: 'hi', vector: ['not num']}], null],
  ['euclidean', 1, [{label: null, vector: [1]}], null],
  ['euclidean', 1, [{label: 'hi', vector: [1]}], null],
  ['euclidean', 1, [{label: 'hi', vector: [1]}], 'not list'],
  ['euclidean', 1, [{label: 'hi', vector: [1]}], ['not num']],
]

test.each(knnFailCases)('Test knn with incorrect inputs', 
  function(algo, k, neighbors, target) {
    const caller = knn.bind(null, algo, k, neighbors, target)

    expect(caller).toThrow()
  }
 )