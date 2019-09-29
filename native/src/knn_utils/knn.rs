use rayon::prelude::*;
use std::cmp::PartialOrd;
use std::marker::{ Send, Sync };

use super::distance_computation::compute_distance;


pub fn knn<V: Send + Sync>
  (algo: fn(&V, &V) -> f64, target: &V, k: f64, neighbors: &Vec<(String, V)>) -> Vec<(String, f64)> {
    if k < 0.0 {
      panic!( "K value cannot be negative\n" )
    }
    let distances = get_distances(algo, target, neighbors);
    get_k(distances, k)
}


fn get_distances<V: Send + Sync>
  (algo: fn(&V, &V) -> f64,target: &V, neighbors: &Vec<(String, V)>) -> Vec<(String, f64)> {
    neighbors.par_iter()
             .map(|neigh| compute_distance(algo, target, neigh) )
             .collect()
}


fn get_k<O: PartialOrd + Clone>(mut dists: Vec<(String, O)>, k: f64) -> Vec<(String, O)> {
  dists.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
  dists.iter()
       .take(k as usize)
       .cloned()
       .collect()
}