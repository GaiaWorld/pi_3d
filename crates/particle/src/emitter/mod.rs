use pi_scene_math::Vector3;

pub mod box_shape_emitter;
pub mod ishape_emitter_type;
pub mod circle_shape_emitter;
pub mod cone_shape_emitter;
pub mod edge_shape_emitter;
pub mod hemisphere_shape_emitter;
pub mod point_shape_emitter;
pub mod rectangle_shape_emitter;
pub mod sphere_shape_emitter;

pub struct SerializationObject {
    pub _type: Option<String> ,
    pub radius: Option<f32>,
    pub angle: Option<f32>,
    pub direction_randomizer: Option<f32>,
    pub radius_range: Option<f32>,
    pub height_range: Option<f32>,
    pub emit_from_spawn_point_only: Option<bool>,
    pub size: Option<f32>,
    pub direction1: Option<Vector3>,
    pub direction2: Option<Vector3>
}