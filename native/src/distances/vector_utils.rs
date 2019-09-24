

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