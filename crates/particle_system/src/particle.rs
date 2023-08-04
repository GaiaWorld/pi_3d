use pi_scene_math::{Color4, Matrix, Vector3};
use pi_wy_rng::WyRng;

use crate::tools::Random;

#[derive(Clone, Debug)]
pub struct Particle {
    pub start_world_matrix: Matrix,
    pub start_world_matrix_invert: Matrix,
    pub emit_world_matrix: Matrix,
    pub start_scaling: Vector3,
    pub start_color: Color4,
    pub emit_rotation: Vector3,
    pub position: Vector3,
    pub rotation: Vector3,
    pub scaling: Vector3,
    pub velocity: Vector3,
    pub direction: Vector3,
    pub readldirection: Vector3,
    pub direction_length: f32,
    pub color: Color4,
    pub texture_start_frame: f32,
    pub texture_row: f32,
    pub age: f32,
    pub lifetime: f32,
    pub trial_lifetime: f32,
    pub trial_width: f32,
    pub uv: [f32; 4],
    pub global_record_list: Vec<[f32; 4]>,
    pub local_record_list: Vec<[f32; 4]>,
    pub color_over_lifetime_amount: f32,
    pub base_random: f32,
}

impl Default for Particle {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            start_world_matrix: Matrix::identity(),
            start_world_matrix_invert: Matrix::identity(),
            emit_world_matrix: Matrix::identity(),
            start_scaling: Vector3::new(1., 1., 1.),
            start_color: Color4::new(1., 1., 1., 1.),
            emit_rotation: Vector3::zeros(),
            position: Vector3::zeros(),
            rotation: Vector3::zeros(),
            scaling: Vector3::new(1., 1., 1.),
            velocity: Vector3::zeros(),
            direction: Vector3::zeros(),
            readldirection: Vector3::zeros(),
            direction_length: 0.,
            color: Color4::new(1., 1., 1., 1.),
            texture_start_frame: 0.,
            texture_row: 0.,
            age: f32::MAX,
            lifetime: 1000.,
            trial_lifetime: 0.,
            trial_width: 1.,
            uv: [1., 1., 0., 0.],
            global_record_list: vec![],
            local_record_list: vec![],
            color_over_lifetime_amount: 1.,
            base_random: 0.,
        }
    }
}

impl Particle {
    pub fn reset(&mut self, random: &mut Random) {
        let mut rng = rand::thread_rng();
        self.age = f32::MAX;
        self.lifetime = 1000.;
        self.trial_lifetime = 0.;
        self.start_world_matrix = Matrix::identity();
        self.start_world_matrix_invert = Matrix::identity();
        self.emit_world_matrix = Matrix::identity();
        self.start_scaling = Vector3::new(1., 1., 1.);
        self.start_color = Color4::new(1., 1., 1., 1.);
        self.emit_rotation = Vector3::zeros();
        self.position = Vector3::zeros();
        self.rotation = Vector3::zeros();
        self.scaling = Vector3::new(1., 1., 1.);
        self.velocity = Vector3::zeros();
        self.direction = Vector3::zeros();
        self.readldirection = Vector3::zeros();
        self.direction_length = 0.;
        self.color = Color4::new(1., 1., 1., 1.);
        self.base_random = random.random();
        self.uv[0] = 1.;
        self.uv[1] = 1.;
        self.uv[2] = 0.;
        self.uv[3] = 0.;
    }
}
