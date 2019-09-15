#[macro_use]
extern crate neon;
extern crate num;
extern crate rayon;

use neon::prelude::*;

mod distance_computation;
mod knn;
 use knn::knn;


fn euclidean_knn(mut cx: FunctionContext) -> JsResult<JsArray> {
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

    let nearest_neighbors = knn(&target, k, &neighbors);

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


//fn convert_neighbors(cx: FunctionContext, neighbors: Vec<Handle<JsValue>>) -> Vec<(String, Vec<f64>)> {
//    neighbors.iter()
//             .map(|val| val.downcast::<JsObject>().unwrap() )
//             .map(|val| convert_object(cx, val) )
//             .collect()
//}


//fn convert_object(mut cx: FunctionContext, obj: Handle<JsObject>) -> (String, Vec<f64>) {
//    let label = obj.get(&mut cx, "label").unwrap()
//                   .downcast::<JsString>().unwrap()
//                   .value();
//    let vector = obj.get(&mut cx, "vector").unwrap()
//                    .downcast::<JsArray>().unwrap()
//                    .to_vec(&mut cx).unwrap();
//    (label, convert_target(vector))
//}


//fn to_jsarray(cx: FunctionContext, results: Vec<(String, f64)>) -> Handle<JsArray> {
//    let objects = results.into_iter()
//                         .map(|tup| to_jsobject(cx, tup).upcast())
//                         .collect();
//    make_jsarray(cx, objects)
//}
//
//fn make_jsarray(mut cx: FunctionContext, objects: Vec<Handle<JsValue>>) -> Handle<JsArray> {
//    let array = JsArray::new(&mut cx, objects.len() as u32);
//    for (i, obj) in objects.into_iter().enumerate() {
//        array.set(&mut cx, i as u32, obj).unwrap();
//    }
//    array
//}

//fn to_jsobject(&mut cx: FunctionContext, (label, dist): (String, f64)) -> Handle<JsObject> {
//    let obj = JsObject::new(&mut cx);
//    let js_label = cx.string(label);
//    let js_dist = cx.number(dist);
//    obj.set(&mut cx, "label", js_label).unwrap();
//    obj.set(&mut cx, "distance", js_dist).unwrap();
//    obj
//}


register_module!(mut cx, {
    cx.export_function("euclideanKnn", euclidean_knn)
});
