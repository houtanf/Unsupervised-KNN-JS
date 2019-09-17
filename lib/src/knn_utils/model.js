const _ = require('lodash');
const { compute_distance } = require('./distance_computation');

function get_k(distances, k) {
	return _(distances)
		.sortBy(d => d.distance)
		.take(k)
		.value();
}

function get_distances(algo, target, neighbors) {
	return _(neighbors)
		.map(neighbor => compute_distance(algo, target, neighbor))
		.value();
}

// named knn in rust, model in js
function model(algo, target, neighbors, k) {
	if (k < 0.0) {
		throw Error('K value cannot be negative');
	}
	let distances = get_distances(algo, target, neighbors);
	return get_k(distances, k);
}

module.exports = { model };
