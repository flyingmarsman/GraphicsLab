use glam::*;
use crate::ray::Ray;
use crate::screen::ScreenPos;

pub struct Camera {
    pub pos: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec3::new(1.0, 1.0, 1.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 60.0,
            aspect_ratio: 1.0,
            near: 0.0,
            far: 100.0,
        }
    }

    pub fn rot_look_at(&self) -> glam::Mat3 {
        let forward = (self.pos - self.target).normalize();
        let right = forward.cross(self.up).normalize();
        let up = right.cross(forward);
        Mat3::from_cols(
            right,
            up,
            forward,
        )
    }

    pub fn generate_ray(&self, pos: &ScreenPos) -> Ray {
        let x_pos = pos.x_pos();
        let y_pos = pos.y_pos();
        let h = -1f32 / (self.fov / 2.0 * std::f32::consts::PI / 180f32).tan();
        let dir = self.rot_look_at() * glam::vec3(x_pos, y_pos, h).normalize();
        let origin = self.pos;
        Ray { o: origin, d: dir }
    }
}