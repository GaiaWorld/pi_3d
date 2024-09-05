use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_scene_math::Vector3;

use super::base::*;


pub trait TLight {
    fn create_light(&self, scene: ObjectID, name: Atom) -> ObjectID;
    fn light_direction(&self, entity: ObjectID, value: Vector3) -> &Self;
    fn lighting_mode(&self, entity: ObjectID, value: LightingMode) -> &Self;
    fn shadow_enable(&self, entity: ObjectID, value: bool) -> &Self;
    fn shadow_mixz(&self, entity: ObjectID, value: f32) -> &Self;
    fn shadow_maxz(&self, entity: ObjectID, value: f32) -> &Self;
    fn shadow_frustumsize(&self, entity: ObjectID, value: f32) -> &Self;
    fn shadow_bias(&self, entity: ObjectID, value: f32) -> &Self;
    fn shadow_normal_bias(&self, entity: ObjectID, value: f32) -> &Self;
    fn shadow_depth_scale(&self, entity: ObjectID, value: f32) -> &Self;
}