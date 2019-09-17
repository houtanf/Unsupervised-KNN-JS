const { model } = require('./knn_utils/model');
const { get_algo } = require('./distances/distance_algos');

function run_knn(algorithm_name = 'euclidean', target, neighbors, k) {
	const { algo } = get_algo(algorithm_name);
	return model(algo, target, neighbors, k);
}

module.exports = { run_knn };
