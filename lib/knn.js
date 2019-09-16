const { defaultKnn } = require('../native');


const knn = function(algo, k, neighbors, target) {
  return defaultKnn(k, neighbors, target, algo)
}


module.exports = { knn }