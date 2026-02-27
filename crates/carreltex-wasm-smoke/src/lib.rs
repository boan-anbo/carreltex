#[no_mangle]
pub extern "C" fn carreltex_wasm_smoke_add(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

