#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate nalgebra as na;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use na::{Matrix, Dynamic, VecStorage, DMatrix};
// #[wasm_bindgen]
// extern {
    
// }

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeatMapGread {
    matrix: Matrix<u32, Dynamic, Dynamic, VecStorage<u32, Dynamic, Dynamic>>,
    matrix_copy: Matrix<u32, Dynamic, Dynamic, VecStorage<u32, Dynamic, Dynamic>>,
}

#[wasm_bindgen]
impl HeatMapGread {
    
    pub fn new(width: usize, height: usize) -> Self {
        let matrix = DMatrix::from_fn(height, width, |_, _| 0_u32);
        let matrix_copy = matrix.clone_owned();
        console_log!("Created {} - {}", &width, &height);
        Self {matrix, matrix_copy}
    }

    pub fn make_grid_copy(&mut self) {
        self.matrix_copy = self.matrix.clone_owned();
    }

    pub fn test_js_call(&self, callback: &js_sys::Function, num: u32) {
        callback.call1(&JsValue::null(), &JsValue::from_f64(num as f64)).unwrap();
    }
}