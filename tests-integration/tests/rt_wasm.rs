#![cfg(all(target_family = "wasm", not(target_os = "wasi")))]

use wasm_bindgen_test::wasm_bindgen_test;

/// Regression test for https://github.com/tokio-rs/tokio/issues/7319
#[wasm_bindgen_test]
fn entry_point() {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
}
