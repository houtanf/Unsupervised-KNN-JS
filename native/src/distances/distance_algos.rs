

pub fn euclidean(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  target.iter()
        .zip(neighbor)
        .map(|(t, n)| (t - n).powf(2.0) )
        .sum::<f64>()
        .sqrt()
}