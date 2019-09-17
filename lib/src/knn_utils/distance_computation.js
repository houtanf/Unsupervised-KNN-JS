function compute_distance(algo, target, neighbor) {
	return {
		label: neighbor.label,
		distance: algo(target.vector, neighbor.vector)
	};
}

module.exports = { compute_distance };
