use num::Float;


pub fn compute_distance<F: Float>
  (algo: impl Fn(&Vec<F>, &Vec<F>) -> F, target: &Vec<F>, (label, vector): &(String, Vec<F>)) 
  -> (String, F) {
    (label.clone(), algo(target, &vector))
}