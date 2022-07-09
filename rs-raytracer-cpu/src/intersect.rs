use std::cmp::max;
use crate::aabb::AABB;
use crate::math::{max_v, max_vec, min_v, min_vec};
use crate::ray::Ray;

pub fn intersect_with_aabb(bound: &AABB, ray: &Ray) -> f32 {
    let vec_min = (bound.min - ray.o)/ray.d;
    let vec_max = (bound.max - ray.o)/ray.d;
    let t_min = max_v(&min_vec(&vec_min, &vec_max)).max(0f32);
    let t_max = min_v(&max_vec(&vec_min, &vec_max));
    if t_max < t_min {
        return crate::math::INF;
    }
    return t_min;
}