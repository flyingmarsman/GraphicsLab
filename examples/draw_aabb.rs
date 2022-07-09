use std::time::SystemTime;
use minifb::*;
use rs_raytracer_cpu::aabb::AABB;
use rs_raytracer_cpu::camera::Camera;
use rs_raytracer_cpu::color::rgb_to_uint;
use rs_raytracer_cpu::intersect::intersect_with_aabb;
use rs_raytracer_cpu::math::INF;
use rs_raytracer_cpu::screen::ScreenPos;

fn main() {
    let w = 1024;
    let h = 1024;
    let mut buffer: Vec<u32> = vec![0; w * h];
    let mut window = Window::new("Test", w, h, WindowOptions::default()).unwrap();
    let camera = Camera::new();
    let aabb = AABB::new(
        glam::Vec3::new(-0.5, -0.5, -0.5),
        glam::Vec3::new(0.5, 0.5, 0.5),
    );
    let t = SystemTime::now();
    for i in 0..w {
        for j in 0..h {
            let screen_pos = ScreenPos::new(i as i32, j as i32, w as i32, h as i32);
            let ray = camera.generate_ray(&screen_pos);
            let t = intersect_with_aabb( &aabb, &ray);
            if t < INF {
                buffer[i + j * w] = rgb_to_uint(1.0, 1.0, 1.0);
            }

        }
    }
    println!("cost: {}", t.elapsed().unwrap().as_millis());
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, w, h).unwrap();
    }
}