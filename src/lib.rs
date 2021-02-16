mod lox;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run(source: &str) -> JsValue {
    let mut result = String::new();
    lox::run(
        String::from(source),
        &mut Box::new(lox::environment::Environment::new()),
        &mut result,
    );
    return JsValue::from_str(&format!("{}", result));
}
