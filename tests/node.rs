#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use contextual_identities::*;

#[wasm_bindgen_test]
fn color_serialization() {
    assert_eq!(serde_json::to_string_pretty(&Color::Red).expect("Failed to serialize"), r#""red""#)
}

#[wasm_bindgen_test]
fn icon_serialization() {
    assert_eq!(serde_json::to_string_pretty(&Icon::Dollar).expect("Failed to serialize"), r#""dollar""#)
}
