use super::vector_utils::{ dot_product, norm };


pub fn get_algo(name: String) -> fn(&Vec<f64>, &Vec<f64>) -> f64 {
  match name.to_lowercase().as_ref() {
    "euclidean" => euclidean,
    "cosine" => cosine_distance,
    "mse" => mean_square_error,
    "manhattan" => manhattan,
    "chebyshev" => chebyshev,
    "canberra" => canberra,
    "hamming" => hamming,
    "L3" => |target, neighbor| minkowski(3.0, target, neighbor),
    "L4" => |target, neighbor| minkowski(4.0, target, neighbor),
    "L5" => |target, neighbor| minkowski(5.0, target, neighbor),
    _ => panic!( "Algorithm {} not found", name),
  }
}


fn euclidean(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  target.iter()
        .zip(neighbor)
        .map( |(t, n)| (t - n).powf(2.0) )
        .sum::<f64>()
        .sqrt()
}


fn cosine_distance(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  let dot = dot_product(target, neighbor);
  let target_norm = norm(target);
  let neighbor_norm = norm(neighbor);
  1.0 - (dot / (target_norm * neighbor_norm))
}


fn mean_square_error(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  target.iter()
        .zip(neighbor)
        .map( |(t, n)| (t - n).powf(2.0) )
        .sum::<f64>() 
        / target.len() as f64
}


fn manhattan(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  target.iter()
        .zip(neighbor)
        .map( |(t, n)| (t - n).abs() )
        .sum()
}


fn chebyshev(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  target.iter()
        .zip(neighbor)
        .map( |(t, n)| (t - n).abs() )
        .max_by( |x, y| x.partial_cmp(y).unwrap() )
        .unwrap()
}


fn canberra(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  target.iter()
        .zip(neighbor)
        .map( |(t, n)| (t - n).abs() / (t.abs() + n.abs()) )
        .sum()
}


fn hamming(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  target.iter()
        .zip(neighbor)
        .map( |(t, n)| if t != n { 1.0 } else { 0.0 } )
        .sum()
}


fn minkowski(p: f64, target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  target.iter()
        .zip(neighbor)
        .map( |(t, n)| (t - n).powf(p) )
        .sum::<f64>()
        .powf( 1.0 / p )
}