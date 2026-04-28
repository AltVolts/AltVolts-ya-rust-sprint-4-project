use std::ffi::{c_char, CStr};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PluginParams {
    radius: u32,
    iterations: u32,
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
        PluginParams { radius: 3, iterations: 3 }
    });

    println!("Mirror plugin process image with - width: {}, height: {}, params: {:#?}", width, height, params);
}