use std::sync::{Mutex, OnceLock};

use carreltex_core::{
    validate_compile_report_json, validate_main_tex, CompileRequestV0, CompileStatus, Mount,
    MAIN_TEX_MAX_BYTES, MAX_LOG_BYTES_V0,
};
use carreltex_engine::{compile_main_v0, compile_request_v0};

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

#[derive(Default)]
struct CompileRequestState {
    entrypoint: Option<String>,
    source_date_epoch: Option<u64>,
    max_log_bytes: Option<u32>,
}

fn compile_request_state() -> &'static Mutex<CompileRequestState> {
    static STATE: OnceLock<Mutex<CompileRequestState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(CompileRequestState::default()))
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

fn write_report_for_status(status: CompileStatus) {
    let fallback = match status {
        CompileStatus::Ok => "{\"status\":\"OK\",\"missing_components\":[]}",
        CompileStatus::InvalidInput => "{\"status\":\"INVALID_INPUT\",\"missing_components\":[]}",
        CompileStatus::NotImplemented => {
            "{\"status\":\"NOT_IMPLEMENTED\",\"missing_components\":[\"tex-engine\"]}"
        }
    };
    set_last_report_bytes(fallback);
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_compile_main_v0() -> i32 {
    let mut mount = match mount_state().lock() {
        Ok(guard) => guard,
        Err(_) => {
            write_report_for_status(CompileStatus::InvalidInput);
            return CompileStatus::InvalidInput as i32;
        }
    };

    let result = compile_main_v0(&mut mount);
    let json = result.report_json;
    // Fail-closed: if the report is malformed, degrade to INVALID_INPUT.
    if validate_compile_report_json(&json).is_err() {
        write_report_for_status(CompileStatus::InvalidInput);
        return CompileStatus::InvalidInput as i32;
    }

    set_last_report_bytes(&json);
    result.status as i32
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_compile_request_reset_v0() -> i32 {
    let mut state = match compile_request_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 1,
    };
    state.entrypoint = None;
    state.source_date_epoch = None;
    state.max_log_bytes = None;
    0
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_compile_request_set_entrypoint_v0(
    ptr: *const u8,
    len: usize,
) -> i32 {
    let bytes = match read_input_bytes(ptr, len) {
        Some(bytes) => bytes,
        None => return 1,
    };
    let text = match core::str::from_utf8(bytes) {
        Ok(text) => text,
        Err(_) => return 1,
    };
    if text != "main.tex" {
        return 1;
    }

    let mut state = match compile_request_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 1,
    };
    state.entrypoint = Some(text.to_owned());
    0
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_compile_request_set_source_date_epoch_v0(epoch: u64) -> i32 {
    if epoch == 0 {
        return 1;
    }
    let mut state = match compile_request_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 1,
    };
    state.source_date_epoch = Some(epoch);
    0
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_compile_request_set_max_log_bytes_v0(value: u32) -> i32 {
    if value == 0 || value > MAX_LOG_BYTES_V0 {
        return 1;
    }
    let mut state = match compile_request_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 1,
    };
    state.max_log_bytes = Some(value);
    0
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_compile_run_v0() -> i32 {
    let request = {
        let state = match compile_request_state().lock() {
            Ok(guard) => guard,
            Err(_) => {
                write_report_for_status(CompileStatus::InvalidInput);
                return CompileStatus::InvalidInput as i32;
            }
        };
        CompileRequestV0 {
            entrypoint: state.entrypoint.clone().unwrap_or_default(),
            source_date_epoch: state.source_date_epoch.unwrap_or(0),
            max_log_bytes: state.max_log_bytes.unwrap_or(0),
        }
    };

    let mut mount = match mount_state().lock() {
        Ok(guard) => guard,
        Err(_) => {
            write_report_for_status(CompileStatus::InvalidInput);
            return CompileStatus::InvalidInput as i32;
        }
    };

    let result = compile_request_v0(&mut mount, &request);
    if validate_compile_report_json(&result.report_json).is_err() {
        write_report_for_status(CompileStatus::InvalidInput);
        return CompileStatus::InvalidInput as i32;
    }

    set_last_report_bytes(&result.report_json);
    result.status as i32
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
