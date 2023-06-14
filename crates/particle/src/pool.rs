use pi_scene_math::Vector3;

use crate::particle::Particle;

pub struct Pool {
    particle_list: Vec<Particle>,
    vector3_list: Vec<Vector3>,
}

impl Pool {
    pub fn create_particle(&mut self) -> Particle {
        if let Some(value) = self.particle_list.pop() {
            return value;
        }

        return Particle::default();
    }

    pub fn recycle_particle(&mut self, mut value: Particle) {
        value.reset();
        self.particle_list.push(value);
    }

    pub fn create_vector3(&mut self) -> Vector3 {
        if let Some(value) = self.vector3_list.pop() {
            return value;
        }

        return Vector3::zeros();
    }

    pub fn recycle_vector3(&mut self, value: Vector3) {
        self.vector3_list.push(value);
    }

    pub fn new() -> Self {
        Self {
            particle_list: vec![],
            vector3_list: vec![],
        }
    }
}
