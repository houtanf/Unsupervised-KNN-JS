

pub fn dot_product(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
  a.iter()
   .zip(b)
   .map(|(x, y)| x * y)
   .sum()
}


pub fn norm(vector: &Vec<f64>) -> f64 {
  vector.iter()
        .map( |n| n.powi(2) )
        .sum::<f64>()
        .sqrt()
}


pub fn mean(vector: &Vec<f64>) -> f64 {
  vector.iter()
        .sum::<f64>()
        / vector.len() as f64
}


pub fn std(mean: f64, vector: &Vec<f64>) -> f64 {
  (vector.iter()
        .map( |x| (x - mean).powi(2) )
        .sum::<f64>()
        / vector.len() as f64)
        .sqrt()
}


pub fn covariance(mean_x: f64, mean_y: f64, x: &Vec<f64>, y: &Vec<f64>)  -> f64 {
  x.iter()
   .zip(y)
   .map( |(xi, yi)| (xi - mean_x) * (yi - mean_y) )
   .sum::<f64>()
   / x.len() as f64
}