use num::Float;


pub fn compute_distance<F: Float>
  (algo: impl Fn(&Vec<F>, &Vec<F>) -> F, target: &Vec<F>, neighbor: &(String, Vec<F>)) -> (String, F) {
  (neighbor.0.clone(), algo(target, &neighbor.1))
}


pub fn euclidean(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  target.iter()
        .zip(neighbor)
        .map(|(t, n)| (t - n).powf(2.0) )
        .sum::<f64>()
        .sqrt()
}