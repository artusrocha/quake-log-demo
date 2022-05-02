use wasm_bindgen::prelude::*;

use analysis::Analysis;

mod analysis;

#[wasm_bindgen]
pub fn analyze(log: &str) -> String {
    let mut analysis = Analysis::new();
    analysis.process_log(&log.to_string());
    serde_json::to_string(&analysis.reports).unwrap()
}
