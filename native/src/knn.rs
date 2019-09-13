use rayon::prelude::*;
use distance_computation::{compute_distance, euclidean};


pub fn knn(target: &Vec<f64>, k: u32, neighbors: &Vec<(String, Vec<f64>)>) 
  -> Vec<(String, f64)> {
    let distances = get_distances(target, neighbors);
    get_k(distances, k)
}


fn get_distances(target: &Vec<f64>, neighbors: &Vec<(String, Vec<f64>)>)
  -> Vec<(String, f64)> {
    neighbors.par_iter()
             .map(|neigh| compute_distance(euclidean, target, neigh) )
             .collect()
}


fn get_k(mut dists: Vec<(String, f64)>, k: u32) -> Vec<(String, f64)> {
  dists.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
  dists.iter()
       .take(k as usize)
       .cloned()
       .collect()
}