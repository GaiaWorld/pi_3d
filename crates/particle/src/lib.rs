// #![feature(allocator_api)]

use nalgebra::UnitQuaternion;
use pi_scene_math::{Vector3, Quaternion};

#[macro_use]
extern crate lazy_static;

pub mod emitter;
pub mod extend;
pub mod interpolation;
pub mod iparticle_system_config;
pub mod math;
pub mod mesh_particle_system;
pub mod modifier;
pub mod particle;
pub mod particle_system_tool;
pub mod pool;

// 适配xyz都为0的情况
pub fn normalize(v: &Vector3) -> Vector3 {
    if v[0] != 0.0 || v[1] != 0.0 || v[2] != 0.0 {
        return v.normalize();
    }
    Vector3::new(0.0, 0.0, 0.0)
}

pub fn multiply(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3::new(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z)
}

pub fn rotateByQuaternionAroundPointToRef(v: &Vector3, center: &Vector3, q: Quaternion) -> Vector3{
    q.transform_vector(&(v - center)) + center
}