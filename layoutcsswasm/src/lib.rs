use config::LayoutStyleConfig;
use wasm_bindgen::prelude::*;
use layoutcss_parser::*;

#[wasm_bindgen]
pub fn add(text: String) -> String {
    let config = LayoutStyleConfig{harmonic_ratio:1.618, dev:false,min_screen:String::from("800px"), max_screen:String::from("1200px"), base_value:String::from("16px"), resizing_ratio:1.1};
    let result = get_css_from_string(&text, None, None, &config);
    result
}
