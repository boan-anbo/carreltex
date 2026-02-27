use std::sync::{Mutex, OnceLock};

use carreltex_core::{
    compile_main_v0, validate_compile_report_json, validate_main_tex, CompileStatus, Mount,
    MAIN_TEX_MAX_BYTES,
};

#[no_mangle]
pub extern "C" fn carreltex_wasm_smoke_add(left: i32, right: i32) -> i32 {
    left + right
}

fn mount_state() -> &'static Mutex<Mount> {
    static STATE: OnceLock<Mutex<Mount>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(Mount::default()))
}

fn last_report_state() -> &'static Mutex<Vec<u8>> {
    static STATE: OnceLock<Mutex<Vec<u8>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(Vec::new()))
}

fn read_input_bytes<'a>(ptr: *const u8, len: usize) -> Option<&'a [u8]> {
    if ptr.is_null() || len == 0 {
        return None;
    }
    Some(unsafe { core::slice::from_raw_parts(ptr, len) })
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_alloc(size: usize) -> *mut u8 {
    if size == 0 || size > MAIN_TEX_MAX_BYTES {
        return core::ptr::null_mut();
    }

    let mut buffer = Vec::<u8>::with_capacity(size);
    let ptr = buffer.as_mut_ptr();
    core::mem::forget(buffer);
    ptr
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_dealloc(ptr: *mut u8, size: usize) {
    if ptr.is_null() || size == 0 || size > MAIN_TEX_MAX_BYTES {
        return;
    }

    unsafe {
        drop(Vec::<u8>::from_raw_parts(ptr, 0, size));
    }
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_validate_main_tex(ptr: *const u8, len: usize) -> i32 {
    let bytes = match read_input_bytes(ptr, len) {
        Some(bytes) => bytes,
        None => return 1,
    };

    if validate_main_tex(bytes).is_ok() {
        0
    } else {
        1
    }
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_mount_reset() -> i32 {
    let mut mount = match mount_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 1,
    };
    mount.reset();
    0
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_mount_add_file(
    path_ptr: *const u8,
    path_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) -> i32 {
    let path_bytes = match read_input_bytes(path_ptr, path_len) {
        Some(bytes) => bytes,
        None => return 1,
    };
    let data_bytes = match read_input_bytes(data_ptr, data_len) {
        Some(bytes) => bytes,
        None => return 1,
    };

    let mut mount = match mount_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 1,
    };

    if mount.add_file(path_bytes, data_bytes).is_ok() {
        0
    } else {
        1
    }
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_mount_finalize() -> i32 {
    let mut mount = match mount_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 1,
    };
    if mount.finalize().is_ok() {
        0
    } else {
        1
    }
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_mount_has_file(path_ptr: *const u8, path_len: usize) -> i32 {
    let path_bytes = match read_input_bytes(path_ptr, path_len) {
        Some(bytes) => bytes,
        None => return 1,
    };
    let mount = match mount_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 1,
    };

    match mount.has_file(path_bytes) {
        Ok(true) => 0,
        _ => 1,
    }
}

fn set_last_report_bytes(report_json: &str) {
    let mut last = match last_report_state().lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };
    last.clear();
    last.extend_from_slice(report_json.as_bytes());
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_compile_main_v0() -> i32 {
    let mut mount = match mount_state().lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_last_report_bytes("{\"missing_components\":[],\"status\":\"INVALID_INPUT\"}");
            return CompileStatus::InvalidInput as i32;
        }
    };

    let (status, report) = compile_main_v0(&mut mount);
    let json = report.to_canonical_json();
    // Fail-closed: if the report is malformed, degrade to INVALID_INPUT.
    if validate_compile_report_json(&json).is_err() {
        set_last_report_bytes("{\"missing_components\":[],\"status\":\"INVALID_INPUT\"}");
        return CompileStatus::InvalidInput as i32;
    }

    set_last_report_bytes(&json);
    status as i32
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_compile_report_len_v0() -> usize {
    let last = match last_report_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };
    last.len()
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_compile_report_copy_v0(out_ptr: *mut u8, out_len: usize) -> usize {
    if out_ptr.is_null() || out_len == 0 {
        return 0;
    }
    let last = match last_report_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };
    if out_len < last.len() {
        return 0;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(last.as_ptr(), out_ptr, last.len());
    }
    last.len()
}

