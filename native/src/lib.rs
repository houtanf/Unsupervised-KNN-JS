#[macro_use]
extern crate neon;
extern crate num;
extern crate rayon;

use neon::prelude::*;

mod distance_computation;
mod knn;
 use knn::knn;


fn euclidean_knn(mut cx: FunctionContext) -> JsResult<JsArray> {
    let k: u32 = cx.argument::<JsNumber>(0)?.value();
    let neighbors_js = cx.argument::<JsArray>(1)?.to_vec(&mut cx)?;
    let target_js = cx.argument::<JsArray>(2)?.to_vec(&mut cx)?;

    let target = convert_target(target_js);
    let neighbors = convert_neighbors(cx, neighbors_js);

    let nearest_neighbors = knn(&target, &neighbors, k);
    to_jsarray(cx, &nearest_neighbors)
}


fn convert_target(target: Vec<Handle<JsValue>>) -> Vec<f64> {
    target.iter()
          .map(|val| val.downcast::<JsNumber>().unwrap() )
          .map(|val| val.value() )
          .collect()
}


fn convert_neighbors(mut cx: FunctionContext, neighbors: Vec<Handle<JsValue>>) -> Vec<(String, Vec<f64>)> {
    neighbors.iter()
             .map(|val| val.downcast::<JsObject>().unwrap() )
             .map(|val| convert_object(cx, val) )
             .collect()
}


fn convert_object(mut cx: FunctionContext, obj: Handle<JsObject>) -> (String, Vec<f64>) {
    let label = obj.get(&mut cx, "label").unwrap()
                   .downcast::<JsString>().unwrap()
                   .value();
    let vector = obj.get(&mut cx, "vector").unwrap()
                    .downcast::<JsArray>().unwrap()
                    .to_vec(&mut cx).unwrap();
    (label, convert_target(vector))
}


fn to_jsarray(mut cx: FunctionContext, results: &Vec<(String, f64)>) -> JsResult<JsArray> {
    //todo
    Ok(cx.empty_array())
}


register_module!(mut cx, {
    cx.export_function("euclideanKnn", euclidean_knn)
});
