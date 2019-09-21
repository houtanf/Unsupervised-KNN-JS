#[macro_use]
extern crate neon;
extern crate rayon;

use neon::prelude::*;

mod conversions;
 use conversions::to_rust::{convert_target, convert_neighbors};
 use conversions::to_js::to_jsarray;
mod knn_utils;
 use knn_utils::knn::knn;
mod distances;
 use distances::distance_algos::get_algo;


fn default_knn(mut cx: FunctionContext) -> JsResult<JsArray> {
  let name: String = cx.argument::<JsString>(3)?.value();
  let algo = get_algo(name);
  run_knn(cx, algo)
}


fn run_knn(mut cx: FunctionContext, algo: fn(&Vec<f64>, &Vec<f64>) -> f64) -> JsResult<JsArray> {
  let k: f64 = cx.argument::<JsNumber>(0)?.value();
  let neighbors_js = cx.argument::<JsArray>(1)?.to_vec(&mut cx)?;
  let target_js = cx.argument::<JsArray>(2)?.to_vec(&mut cx)?;

  let target = convert_target(target_js);
  let neighbors = convert_neighbors(&mut cx, neighbors_js);


  let nearest_neighbors = knn(algo, &target, k, &neighbors);

  Ok( to_jsarray(cx, nearest_neighbors) )
}


register_module!(mut cx, {
  cx.export_function("defaultKnn", default_knn)
});
