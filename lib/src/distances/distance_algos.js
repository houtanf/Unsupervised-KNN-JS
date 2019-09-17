const _ = require('lodash');

function get_algo(name) {
	switch (name) {
		case 'euclidean':
			return { algo: euclidean };
		default:
			throw Error(`Algorithm ${name} not found`);
	}
}

function euclidean(target, neighbor) {
	return Math.sqrt(
		_(target)
			.zip(neighbor)
			.map(([t, n]) => Math.pow(t - n, 2.0))
			.sum()
	);
}

module.exports = { get_algo };
