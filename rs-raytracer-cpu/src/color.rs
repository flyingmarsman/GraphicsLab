pub fn rgb_to_uint(r: f32, g: f32, b: f32) -> u32 {
    let r = (r * 255.0) as u32;
    let g = (g * 255.0) as u32;
    let b = (b * 255.0) as u32;
    (r << 16) | (g << 8) | b | 0xFF000000
}