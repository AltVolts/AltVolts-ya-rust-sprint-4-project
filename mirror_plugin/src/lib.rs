use std::ffi::{c_char, CStr};
use serde::Deserialize;

#[derive(Deserialize)]
struct PluginParams {
    #[serde(default)]
    horizontal: bool,
    #[serde(default)]
    vertical: bool,
}

#[unsafe(no_mangle)]
extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) {
    let params_str = if params.is_null() {
        "{}"
    } else {
        unsafe { CStr::from_ptr(params) }.to_str().unwrap_or("{}")
    };

    let params: PluginParams = serde_json::from_str(params_str).unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON: {}, using defaults", e);
        PluginParams { horizontal: false, vertical: false }
    });

    if rgba_data.is_null() {
        eprintln!("Null pointer for rgba_data!");
        return;
    }

    let width = width as usize;
    let height = height as usize;
    let row_size = width * 4;
    let total_bytes = row_size * height;
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, total_bytes) };
}