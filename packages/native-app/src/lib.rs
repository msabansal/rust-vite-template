#[path = "util/tracing.rs"]
mod tracing_util;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    tracing::info!("Saying hello from the native-app package!");
    alert("Hello, native-app!");
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    #[cfg(web_platform)]
    console_error_panic_hook::set_once();
    tracing_util::init();

    Ok(())
}
