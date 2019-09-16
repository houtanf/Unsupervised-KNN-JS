use neon::prelude::*;


pub fn convert_target(target: Vec<Handle<JsValue>>) -> Vec<f64> {
  target.iter()
        .map(|val| val.downcast::<JsNumber>().unwrap() )
        .map(|val| val.value() )
        .collect()
}


pub fn convert_neighbors(cx: &mut FunctionContext, neighbors: Vec<Handle<JsValue>>) 
  -> Vec<(String, Vec<f64>)> {
    neighbors.iter()
             .map(|val| val.downcast::<JsObject>().unwrap() )
             .map(|val| convert_object(cx, val) )
             .collect()
}


pub fn convert_object(cx: &mut FunctionContext, obj: Handle<JsObject>) -> (String, Vec<f64>) {
  let label = obj.get(cx, "label").unwrap()
                  .downcast::<JsString>().unwrap()
                  .value();
  let vector = obj.get(cx, "vector").unwrap()
                  .downcast::<JsArray>().unwrap()
                  .to_vec(cx).unwrap();
  (label, convert_target(vector))
}