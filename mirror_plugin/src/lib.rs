use serde::Deserialize;
use std::ffi::{CStr, c_char};

const RGB_PIXEL_BYTES: usize = 4;

#[derive(Deserialize)]
struct PluginParams {
    #[serde(default)]
    horizontal: bool,
    #[serde(default)]
    vertical: bool,
}

#[unsafe(no_mangle)]
extern "C" fn process_image(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char) {
    let params_str = if params.is_null() {
        "{}"
    } else {
        unsafe { CStr::from_ptr(params) }.to_str().unwrap_or("{}")
    };

    let params: PluginParams = serde_json::from_str(params_str).unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON: {}, using defaults", e);
        PluginParams {
            horizontal: false,
            vertical: false,
        }
    });

    if rgba_data.is_null() {
        eprintln!("Null pointer for rgba_data!");
        return;
    }

    let width = width as usize;
    let height = height as usize;
    let total_bytes = width * height * RGB_PIXEL_BYTES;
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, total_bytes) };

    if params.horizontal && params.vertical {
        println!("180° rotation (horizontal + vertical)...");
        full_mirror(width, height, data)
    } else if params.horizontal {
        println!("horizontal mirroring...");
        horizontal_mirror(width, data);
    } else if params.vertical {
        println!("vertical mirroring...");
        vertical_mirror(width, height, data);
    }

}

fn horizontal_mirror(width: usize, data: &mut [u8]) {
    let row_size = width * RGB_PIXEL_BYTES;
    for row in data.chunks_exact_mut(row_size) {
        let pixels = unsafe { std::slice::from_raw_parts_mut(row.as_mut_ptr() as *mut u32, width) };
        let mut left = 0;
        let mut right = width - 1;

        while left < right {
            pixels.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

fn vertical_mirror(width: usize, height: usize, data: &mut [u8]) {
    let row_size = width * RGB_PIXEL_BYTES;
    let mut top = 0;
    let mut bottom = height - 1;

    while top < bottom {
        let top_start = top * row_size;
        let bottom_start = bottom * row_size;

        let (head, tail) = data.split_at_mut(bottom_start);
        let top_row = &mut head[top_start..top_start + row_size];
        let bottom_row = &mut tail[..row_size];
        top_row.swap_with_slice(bottom_row);

        top += 1;
        bottom -= 1;
    }
}

fn full_mirror(width: usize, height: usize, data: &mut [u8]) {
    let total_pixels = width * height;
    let pixels = unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u32, total_pixels) };

    let half = total_pixels / 2;
    for i in 0..half {
        let x1 = i % width;
        let y1 = i / width;
        let x2 = width - 1 - x1;
        let y2 = height - 1 - y1;

        let idx1 = y1 * width + x1;
        let idx2 = y2 * width + x2;
        pixels.swap(idx1, idx2);
    }
}