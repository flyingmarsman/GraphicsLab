use glam::*;

pub const ERR: f32 = 1e-6;
pub const INF: f32 = 1e20;

pub fn max_v(v:&Vec3) -> f32 {
    v.x.max(v.y.max(v.z))
}

pub fn min_v(v:&Vec3) -> f32 {
    v.x.min(v.y.min(v.z))
}

pub fn min_vec(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(v1.x.min(v2.x), v1.y.min(v2.y), v1.z.min(v2.z))
}

pub fn max_vec(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(v1.x.max(v2.x), v1.y.max(v2.y), v1.z.max(v2.z))
}