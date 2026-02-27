use std::sync::{Mutex, OnceLock};

use carreltex_core::{
    append_event_v0, artifact_bytes_within_cap_v0, report_json_has_status_token_v0,
    report_json_missing_components_is_empty_v0, validate_compile_report_json, validate_main_tex,
    validate_tex_stats_json_v0, CompileRequestV0, CompileStatus, Mount,
    DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0, EVENT_KIND_LOG_BYTES_V0, EVENT_KIND_TEX_STATS_JSON_V0,
    MAX_LOG_BYTES_V0, MAX_TEX_STATS_JSON_BYTES_V0, MAX_WASM_ALLOC_BYTES_V0,
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

fn last_log_state() -> &'static Mutex<Vec<u8>> {
    static STATE: OnceLock<Mutex<Vec<u8>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(Vec::new()))
}

fn last_xdv_state() -> &'static Mutex<Vec<u8>> {
    static STATE: OnceLock<Mutex<Vec<u8>>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(Vec::new()))
}

fn last_events_state() -> &'static Mutex<Vec<u8>> {
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

fn resolve_artifact_name<'a>(name_ptr: *const u8, name_len: usize) -> Option<&'a str> {
    let name_bytes = read_input_bytes(name_ptr, name_len)?;
    core::str::from_utf8(name_bytes).ok()
}

fn copy_bytes_to_out(bytes: &[u8], out_ptr: *mut u8, out_len: usize) -> usize {
    if out_ptr.is_null() || out_len == 0 {
        return 0;
    }
    if out_len < bytes.len() {
        return 0;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), out_ptr, bytes.len());
    }
    bytes.len()
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_alloc(size: usize) -> *mut u8 {
    if size == 0 || size > MAX_WASM_ALLOC_BYTES_V0 {
        return core::ptr::null_mut();
    }

    let mut buffer = Vec::<u8>::with_capacity(size);
    let ptr = buffer.as_mut_ptr();
    core::mem::forget(buffer);
    ptr
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_dealloc(ptr: *mut u8, size: usize) {
    if ptr.is_null() || size == 0 || size > MAX_WASM_ALLOC_BYTES_V0 {
        return;
    }

    unsafe {
        drop(Vec::<u8>::from_raw_parts(ptr, 0, size));
    }
}

#[cfg(test)]
mod tests {
    use super::{carreltex_wasm_alloc, carreltex_wasm_dealloc};
    use carreltex_core::MAX_WASM_ALLOC_BYTES_V0;

    #[test]
    fn wasm_alloc_accepts_small_size() {
        let ptr = carreltex_wasm_alloc(1);
        assert!(!ptr.is_null());
        carreltex_wasm_dealloc(ptr, 1);
    }

    #[test]
    fn wasm_alloc_accepts_max_size() {
        let ptr = carreltex_wasm_alloc(MAX_WASM_ALLOC_BYTES_V0);
        assert!(!ptr.is_null());
        carreltex_wasm_dealloc(ptr, MAX_WASM_ALLOC_BYTES_V0);
    }

    #[test]
    fn wasm_alloc_rejects_size_above_max() {
        let ptr = carreltex_wasm_alloc(MAX_WASM_ALLOC_BYTES_V0 + 1);
        assert!(ptr.is_null());
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

#[no_mangle]
pub extern "C" fn carreltex_wasm_mount_read_file_len_v0(
    path_ptr: *const u8,
    path_len: usize,
) -> usize {
    let path_bytes = match read_input_bytes(path_ptr, path_len) {
        Some(bytes) => bytes,
        None => return 0,
    };
    let mount = match mount_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };
    match mount.read_file_by_bytes_v0(path_bytes) {
        Ok(Some(bytes)) => bytes.len(),
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_mount_read_file_copy_v0(
    path_ptr: *const u8,
    path_len: usize,
    out_ptr: *mut u8,
    out_len: usize,
) -> usize {
    if out_ptr.is_null() || out_len == 0 {
        return 0;
    }
    let path_bytes = match read_input_bytes(path_ptr, path_len) {
        Some(bytes) => bytes,
        None => return 0,
    };
    let mount = match mount_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };
    let bytes = match mount.read_file_by_bytes_v0(path_bytes) {
        Ok(Some(bytes)) => bytes,
        _ => return 0,
    };
    if out_len < bytes.len() {
        return 0;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), out_ptr, bytes.len());
    }
    bytes.len()
}

fn set_last_report_bytes(report_json: &str) {
    let mut last = match last_report_state().lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };
    last.clear();
    last.extend_from_slice(report_json.as_bytes());
}

fn set_last_log_bytes(log_bytes: &[u8]) {
    let mut last = match last_log_state().lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };
    last.clear();
    last.extend_from_slice(log_bytes);
}

fn set_last_xdv_bytes(xdv_bytes: &[u8]) {
    let mut last = match last_xdv_state().lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };
    last.clear();
    last.extend_from_slice(xdv_bytes);
}

fn read_last_xdv_bytes() -> Option<Vec<u8>> {
    let last = last_xdv_state().lock().ok()?;
    Some(last.clone())
}

fn set_last_events_bytes(events_bytes: &[u8]) {
    let mut last = match last_events_state().lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };
    last.clear();
    last.extend_from_slice(events_bytes);
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
    set_last_log_bytes(&[]);
    set_last_xdv_bytes(&[]);
    set_last_events_bytes(&[]);
}

fn store_compile_result_or_fail_closed(
    report_json: &str,
    log_bytes: &[u8],
    xdv_bytes: &[u8],
    tex_stats_json: &str,
    status: CompileStatus,
    expected_log_max_bytes: usize,
) -> i32 {
    if validate_compile_report_json(report_json).is_err() {
        write_report_for_status(CompileStatus::InvalidInput);
        return CompileStatus::InvalidInput as i32;
    }
    if !report_json_has_status_token_v0(status, report_json) {
        write_report_for_status(CompileStatus::InvalidInput);
        return CompileStatus::InvalidInput as i32;
    }
    let missing_components_empty = match report_json_missing_components_is_empty_v0(report_json) {
        Some(value) => value,
        None => {
            write_report_for_status(CompileStatus::InvalidInput);
            return CompileStatus::InvalidInput as i32;
        }
    };
    match status {
        CompileStatus::NotImplemented if missing_components_empty => {
            write_report_for_status(CompileStatus::InvalidInput);
            return CompileStatus::InvalidInput as i32;
        }
        CompileStatus::Ok | CompileStatus::InvalidInput if !missing_components_empty => {
            write_report_for_status(CompileStatus::InvalidInput);
            return CompileStatus::InvalidInput as i32;
        }
        _ => {}
    }
    if log_bytes.len() > MAX_LOG_BYTES_V0 as usize || log_bytes.len() > expected_log_max_bytes {
        write_report_for_status(CompileStatus::InvalidInput);
        return CompileStatus::InvalidInput as i32;
    }
    match status {
        CompileStatus::InvalidInput if !tex_stats_json.is_empty() => {
            write_report_for_status(CompileStatus::InvalidInput);
            return CompileStatus::InvalidInput as i32;
        }
        CompileStatus::NotImplemented
            if tex_stats_json.is_empty() || tex_stats_json.len() > MAX_TEX_STATS_JSON_BYTES_V0 =>
        {
            write_report_for_status(CompileStatus::InvalidInput);
            return CompileStatus::InvalidInput as i32;
        }
        CompileStatus::NotImplemented if validate_tex_stats_json_v0(tex_stats_json).is_err() => {
            write_report_for_status(CompileStatus::InvalidInput);
            return CompileStatus::InvalidInput as i32;
        }
        CompileStatus::Ok => {
            write_report_for_status(CompileStatus::InvalidInput);
            return CompileStatus::InvalidInput as i32;
        }
        _ => {}
    }
    if !artifact_bytes_within_cap_v0(xdv_bytes) {
        write_report_for_status(CompileStatus::InvalidInput);
        return CompileStatus::InvalidInput as i32;
    }
    match status {
        CompileStatus::Ok if xdv_bytes.is_empty() => {
            write_report_for_status(CompileStatus::InvalidInput);
            return CompileStatus::InvalidInput as i32;
        }
        CompileStatus::InvalidInput | CompileStatus::NotImplemented if !xdv_bytes.is_empty() => {
            write_report_for_status(CompileStatus::InvalidInput);
            return CompileStatus::InvalidInput as i32;
        }
        _ => {}
    }

    set_last_report_bytes(report_json);
    set_last_log_bytes(log_bytes);
    set_last_xdv_bytes(xdv_bytes);
    status as i32
}

fn store_events_for_v0_or_fail_closed(log_bytes: &[u8], tex_stats_json: &str) -> Result<(), ()> {
    let mut events = Vec::new();
    if append_event_v0(&mut events, EVENT_KIND_LOG_BYTES_V0, log_bytes).is_err() {
        return Err(());
    }
    if append_event_v0(
        &mut events,
        EVENT_KIND_TEX_STATS_JSON_V0,
        tex_stats_json.as_bytes(),
    )
    .is_err()
    {
        return Err(());
    }
    set_last_events_bytes(&events);
    Ok(())
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
    let status = store_compile_result_or_fail_closed(
        &result.report_json,
        &result.log_bytes,
        &result.main_xdv_bytes,
        &result.tex_stats_json,
        result.status,
        DEFAULT_COMPILE_MAIN_MAX_LOG_BYTES_V0 as usize,
    );
    if status == CompileStatus::InvalidInput as i32 {
        return status;
    }
    if store_events_for_v0_or_fail_closed(&result.log_bytes, &result.tex_stats_json).is_err() {
        write_report_for_status(CompileStatus::InvalidInput);
        return CompileStatus::InvalidInput as i32;
    }
    status
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
    let status = store_compile_result_or_fail_closed(
        &result.report_json,
        &result.log_bytes,
        &result.main_xdv_bytes,
        &result.tex_stats_json,
        result.status,
        request.max_log_bytes as usize,
    );
    if status == CompileStatus::InvalidInput as i32 {
        return status;
    }
    if store_events_for_v0_or_fail_closed(&result.log_bytes, &result.tex_stats_json).is_err() {
        write_report_for_status(CompileStatus::InvalidInput);
        return CompileStatus::InvalidInput as i32;
    }
    status
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

#[no_mangle]
pub extern "C" fn carreltex_wasm_compile_log_len_v0() -> usize {
    let last = match last_log_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };
    last.len()
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_compile_log_copy_v0(out_ptr: *mut u8, out_len: usize) -> usize {
    if out_ptr.is_null() || out_len == 0 {
        return 0;
    }
    let last = match last_log_state().lock() {
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

#[no_mangle]
pub extern "C" fn carreltex_wasm_events_len_v0() -> usize {
    let last = match last_events_state().lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };
    last.len()
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_events_copy_v0(out_ptr: *mut u8, out_len: usize) -> usize {
    if out_ptr.is_null() || out_len == 0 {
        return 0;
    }
    let last = match last_events_state().lock() {
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

#[no_mangle]
pub extern "C" fn carreltex_wasm_artifact_main_xdv_len_v0() -> usize {
    let name = b"main.xdv";
    carreltex_wasm_artifact_len_v0(name.as_ptr(), name.len())
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_artifact_main_xdv_copy_v0(
    out_ptr: *mut u8,
    out_len: usize,
) -> usize {
    let name = b"main.xdv";
    carreltex_wasm_artifact_copy_v0(name.as_ptr(), name.len(), out_ptr, out_len)
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_artifact_len_v0(name_ptr: *const u8, name_len: usize) -> usize {
    let name = match resolve_artifact_name(name_ptr, name_len) {
        Some(name) => name,
        None => return 0,
    };
    if name != "main.xdv" {
        return 0;
    }

    let bytes = match read_last_xdv_bytes() {
        Some(bytes) => bytes,
        None => return 0,
    };
    if !artifact_bytes_within_cap_v0(&bytes) {
        return 0;
    }
    bytes.len()
}

#[no_mangle]
pub extern "C" fn carreltex_wasm_artifact_copy_v0(
    name_ptr: *const u8,
    name_len: usize,
    out_ptr: *mut u8,
    out_len: usize,
) -> usize {
    let name = match resolve_artifact_name(name_ptr, name_len) {
        Some(name) => name,
        None => return 0,
    };
    if name != "main.xdv" {
        return 0;
    }

    let bytes = match read_last_xdv_bytes() {
        Some(bytes) => bytes,
        None => return 0,
    };
    if !artifact_bytes_within_cap_v0(&bytes) {
        return 0;
    }
    copy_bytes_to_out(&bytes, out_ptr, out_len)
}
