use std::cmp::PartialOrd;


pub fn compute_distance<V, F: PartialOrd>
  (algo: impl Fn(&V, &V) -> F, target: &V, (label, vector): &(String, V)) 
  -> (String, F) {
    (label.clone(), algo(target, &vector))
}