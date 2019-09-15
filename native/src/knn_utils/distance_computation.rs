use num::Float;


pub fn compute_distance<F: Float>
  (algo: impl Fn(&Vec<F>, &Vec<F>) -> F, target: &Vec<F>, neighbor: &(String, Vec<F>)) -> (String, F) {
  (neighbor.0.clone(), algo(target, &neighbor.1))
}