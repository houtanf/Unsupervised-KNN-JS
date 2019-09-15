#[macro_use]
extern crate neon;
extern crate num;
extern crate rayon;

use neon::prelude::*;

mod knn_utils;
 use knn_utils::knn::knn;
mod distances;
 use distances::distance_algos::get_algo;


fn default_knn(mut cx: FunctionContext) -> JsResult<JsArray> {
    let name: String = cx.argument::<JsString>(3)?.value();
    let algo = get_algo(name).unwrap();
    run_knn(cx, algo)
}


fn run_knn(mut cx: FunctionContext, algo: fn(&Vec<f64>, &Vec<f64>) -> f64) -> JsResult<JsArray> {
    let k: f64 = cx.argument::<JsNumber>(0)?.value();
    let neighbors_js = cx.argument::<JsArray>(1)?.to_vec(&mut cx)?;
    let target_js = cx.argument::<JsArray>(2)?.to_vec(&mut cx)?;

    let target = convert_target(target_js);

    let mut neighbors = Vec::new();
    for val in neighbors_js.iter() {
        let casted = val.downcast::<JsObject>().unwrap();
        let label = casted.get(&mut cx, "label").unwrap()
                          .downcast::<JsString>().unwrap()
                          .value();
        let vector = casted.get(&mut cx, "vector").unwrap()
                           .downcast::<JsArray>().unwrap()
                           .to_vec(&mut cx).unwrap();
        neighbors.push( (label, convert_target(vector)) )
    }

    let nearest_neighbors = knn(algo, &target, k, &neighbors);

    let array = JsArray::new(&mut cx, nearest_neighbors.len() as u32);
    for (i, (label, dist)) in nearest_neighbors.into_iter().enumerate() {
        let obj = JsObject::new(&mut cx);
        let js_label = cx.string(label);
        let js_dist = cx.number(dist);
        obj.set(&mut cx, "label", js_label).unwrap();
        obj.set(&mut cx, "distance", js_dist).unwrap();
        array.set(&mut cx, i as u32, obj).unwrap();
    }
    Ok( array )
}


fn convert_target(target: Vec<Handle<JsValue>>) -> Vec<f64> {
    target.iter()
          .map(|val| val.downcast::<JsNumber>().unwrap() )
          .map(|val| val.value() )
          .collect()
}


register_module!(mut cx, {
    cx.export_function("defaultKnn", default_knn)
});
