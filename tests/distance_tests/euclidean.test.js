const { knn } = require('../../lib/knn')


euclideanTests = [
  [1, [{label: 'name', vector: [1, 2]}], [1, 2, 3, 4, 5], [{label: 'name', distance: 0}]],
  [1, [{label: 'name', vector: [1, 2, 3, 4, 5]}], [1, 2], [{label: 'name', distance: 0}]],
  [1, [{label: 'name', vector: [1, 2]}], [2], [{label: 'name', distance: 1}]],
  [1, [{label: 'name', vector: [2]}], [1, 2], [{label: 'name', distance: 1}]],
  [2, [{label: 'name', vector: [1, 2]}, {label: 'other', vector: [3, 2]}], [1, 2],
     [{label: 'name', distance: 0}, {label: 'other', distance: 2}]],
  [2, [{label: 'name', vector: [1, 4]}, {label: 'other', vector: [2, 2]}], [1, 2],
     [{label: 'other', distance: 1}, {label: 'name', distance: 2}]],
]

test.each(euclideanTests)('Test euclidean distance on correct data with input: %d %o %o %o', 
  function(k, neighbors, target, expected) {
    const output = knn('euclidean', k, neighbors, target)

    expect(output).toEqual(expected)
  }
 )