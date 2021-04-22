pub fn zero_slice(data: &mut [f32]) {
    for value in data {
        *value = 0.0;
    }
}
