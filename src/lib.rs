use groan_rs::select::Select;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn validate_gsl_query(query: &str) -> String {
    match Select::parse_query(query) {
        Ok(_) => String::from("Valid query."),
        Err(e) => format!("Invalid query. {}", e.to_string()),
    }
}
