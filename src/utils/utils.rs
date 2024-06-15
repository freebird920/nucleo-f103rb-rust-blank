pub fn hz_into_mhz(hz: u32) -> f32 {
    let hz_as_float = hz as f32;
    let mhz = hz_as_float / 1_000_000.0;
    let mhz_times_100 = mhz * 100.0;
    let rounded_down = (mhz_times_100 as i32) as f32; // floor
    let remainder = mhz_times_100 - rounded_down;
    if remainder >= 0.5 {
        (rounded_down + 1.0) / 100.0
    } else {
        rounded_down / 100.0
    }
}