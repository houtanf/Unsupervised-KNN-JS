use neon::prelude::*;


pub fn to_jsarray(mut cx: FunctionContext, nearest_neighbors: Vec<(String, f64)>) -> Handle<JsArray> {
    let array = JsArray::new(&mut cx, nearest_neighbors.len() as u32);
    for (i, (label, dist)) in nearest_neighbors.into_iter().enumerate() {
        let obj = JsObject::new(&mut cx);
        let js_label = cx.string(label);
        let js_dist = cx.number(dist);
        obj.set(&mut cx, "label", js_label).unwrap();
        obj.set(&mut cx, "distance", js_dist).unwrap();
        array.set(&mut cx, i as u32, obj).unwrap();
    }
    array
}