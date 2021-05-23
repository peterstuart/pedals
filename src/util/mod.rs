pub fn zero_slice(data: &mut [f32]) {
    for value in data {
        *value = 0.0;
    }
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
