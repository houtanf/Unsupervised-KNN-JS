use super::vector_utils::{dot_product, norm};


pub fn get_algo(name: String) -> fn(&Vec<f64>, &Vec<f64>) -> f64 {
  match name.to_lowercase().as_ref() {
    "euclidean" => euclidean,
    "cosine" => cosine_distance,
    _ => panic!( "Algorithm {} not found", name),
  }
}


fn euclidean(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  target.iter()
        .zip(neighbor)
        .map(|(t, n)| (t - n).powf(2.0) )
        .sum::<f64>()
        .sqrt()
}


fn cosine_distance(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  let dot = dot_product(target, neighbor);
  let target_norm = norm(target);
  let neighbor_norm = norm(neighbor);
  1.0 - (dot / (target_norm * neighbor_norm))
}