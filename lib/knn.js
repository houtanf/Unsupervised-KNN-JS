const { defaultKnn } = require('../native');
// const { run_knn } = require('./src/index');

const knn = function(algo, k, neighbors, target) {
	// rust implementation
	return defaultKnn(k, neighbors, target, algo);

	// javascript implementation
	// return run_knn(algo, target, neighbors, k);
};

module.exports = { knn };
