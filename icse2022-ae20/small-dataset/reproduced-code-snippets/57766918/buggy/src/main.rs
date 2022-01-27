use wasm_bindgen::prelude::*;
use serde::Serialize;

type ExtrudeGeometry = Vec<i32>;

#[derive(Serialize)]
struct OutputGeometryCollection {
    collection: Vec<ExtrudeGeometry>,
    r#type: String
}

#[derive(Serialize)]
struct OutputObject {
    data: ExtrudeGeometry,
    r#type: String
}

#[wasm_bindgen]
pub fn toCollection(arr: js_sys::Array, r_type: String) -> JsValue {
    let n_arr: Vec<ExtrudeGeometry> = arr.into_serde().unwrap();
    if r_type == "GeometryCollection" {
        return JsValue::from_serde(&OutputGeometryCollection {
            collection: n_arr,
            r#type: r_type,
        })
        .unwrap();
    } else {
        let ex: ExtrudeGeometry = n_arr[0];
        return JsValue::from_serde(&OutputObject {
            data: ex,
            r#type: r_type,
        })
        .unwrap();
    }
}

fn main() {}