const { euclideanKnn } = require('../lib/knn')


euclideanKnnTests = [
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
test.each(euclideanKnnTests)('Test euclideanKnn on correct data', 
  function(k, neighbors, target, expected) {
    const output = euclideanKnn(k, neighbors, target)

    expect(output).toEqual(expected)
  }
 )