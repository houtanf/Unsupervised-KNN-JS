const knn = require('../lib/knn')

knnAlgos = ['euclidean']

knnTestCases = [
  [1, [{label: 'name', vector: [1, 2]}], [1, 2], [{label: 'name', distance: 0}]],
  [2, [{label: 'name', vector: [1, 2]}], [1, 2], [{label: 'name', distance: 0}]],
  [0, [{label: 'name', vector: [1, 2]}], [1, 2], []],
  [1, [{label: 'name', vector: [1, 2]}], [1, 2, 3, 4, 5], [{label: 'name', distance: 0}]],
  [1, [{label: 'name', vector: [1, 2, 3, 4, 5]}], [1, 2], [{label: 'name', distance: 0}]],
  [1, [{label: 'name', vector: [1, 2]}], [2], [{label: 'name', distance: 1}]],
  [1, [{label: 'name', vector: [2]}], [1, 2], [{label: 'name', distance: 1}]],
  [2, [{label: 'name', vector: [1, 2]}, {label: 'other', vector: [3, 2]}], [1, 2],
     [{label: 'name', distance: 0}, {label: 'other', distance: 2}]],
  [1, [{label: 'name', vector: [1, 2]}, {label: 'other', vector: [3, 2]}], [1, 2],
     [{label: 'name', distance: 0}]],
  [2, [{label: 'name', vector: [1, 4]}, {label: 'other', vector: [2, 2]}], [1, 2],
     [{label: 'other', distance: 1}, {label: 'name', distance: 2}]],
]

knnTests = knnAlgos.flatMap(algo => 
   knnTestCases.map( test => [algo, ...test])
)

test.each(knnTests)('Test knn on correct data with %s algo', 
  function(algo, k, neighbors, target, expected) {
    const output = knn(algo, k, neighbors, target)

    expect(output).toEqual(expected)
  }
 )