#![warn(rust_2018_idioms)]
#![cfg(feature = "rt")]

#[cfg(all(feature = "rt", target_family = "wasm", not(target_os = "wasi")))]
use wasm_bindgen_test::wasm_bindgen_test as test;

#[test]
fn num_workers_wasm() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    assert_eq!(1, rt.metrics().num_workers());
}
