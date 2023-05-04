use warp_core::run;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
fn main() {
    pollster::block_on(run());
}
