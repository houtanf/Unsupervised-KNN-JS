

pub fn get_algo(name: String) -> Result<fn(&Vec<f64>, &Vec<f64>) -> f64, String> {
  match name.to_lowercase().as_ref() {
    "euclidean" => Ok( euclidean ),
    _ => Err( format!("Algorithm {} not found\n", name) ),
  }
}


fn euclidean(target: &Vec<f64>, neighbor: &Vec<f64>) -> f64 {
  target.iter()
        .zip(neighbor)
        .map(|(t, n)| (t - n).powf(2.0) )
        .sum::<f64>()
        .sqrt()
}