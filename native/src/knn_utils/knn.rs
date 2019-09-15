use rayon::prelude::*;
use super::distance_computation::compute_distance;


pub fn knn(algo: fn(&Vec<f64>, &Vec<f64>) -> f64, target: &Vec<f64>, k: f64, neighbors: &Vec<(String, Vec<f64>)>) 
  -> Vec<(String, f64)> {
    let distances = get_distances(algo, target, neighbors);
    get_k(distances, k)
}


fn get_distances(algo: fn(&Vec<f64>, &Vec<f64>) -> f64,target: &Vec<f64>, neighbors: &Vec<(String, Vec<f64>)>)
  -> Vec<(String, f64)> {
    neighbors.par_iter()
             .map(|neigh| compute_distance(algo, target, neigh) )
             .collect()
}


fn get_k(mut dists: Vec<(String, f64)>, k: f64) -> Vec<(String, f64)> {
  dists.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
  dists.iter()
       .take(k as usize)
       .cloned()
       .collect()
}