use wasm_bindgen::prelude::*;

use unit_splitter_web::Model;

#[wasm_bindgen]
pub fn start() {
    draco::start(
        Model::default(),
        draco::select("#unit-splitter-root")
            .expect("root element for unit splitter was not found")
            .into(),
    );
}

fn main() {}
